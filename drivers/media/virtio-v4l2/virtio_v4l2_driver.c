// SPDX-License-Identifier: GPL-2.0+

#include "linux/delay.h"
#include "linux/device.h"
#include "linux/mm.h"
#include "linux/mutex.h"
#include "linux/scatterlist.h"
#include "linux/types.h"
#include "linux/videodev2.h"
#include "linux/wait.h"
#include "linux/workqueue.h"
#include "media/v4l2-dev.h"
#include "media/v4l2-event.h"
#include <linux/module.h>
#include <linux/version.h>
#include <linux/virtio.h>
#include <linux/virtio_config.h>
#include <linux/virtio_ids.h>

#include <media/v4l2-device.h>
#include <media/v4l2-ioctl.h>

#define DRIVER_NAME "virtio_v4l2"

/**
 * A buffer ready to be dequeued by user-space.
 */
struct virtio_v4l2_pending_dqbuf {
	struct v4l2_buffer buffer;
	struct v4l2_plane planes[VIDEO_MAX_PLANES];

	/* Link into the list of pending buffers */
	struct list_head list;
};

/**
 * Virtio-v4l2 device.
 */
struct virtio_v4l2 {
	struct v4l2_device v4l2_dev;
	struct video_device video_dev;

	struct virtio_device *virtio_dev;
	struct virtqueue *commandq;
	struct virtqueue *eventq;
	struct work_struct eventq_work;

	/* Our one and only event buffer */
	/* 
	 * TODO this is probably a bottleneck as the host have to wait for the guest
	 * to acknowledge a decoded frame before delivering the next.
	 */
	void *event_buffer;

	/* List of active decoding sessions */
	struct list_head sessions;
	/* Protects `sessions` */
	struct mutex sessions_lock;

	/*
	 * Command and response buffers for commands without an active session, e.g.
	 * MMAP buffers unmapping.
	 */
	void *cmd_buf, *resp_buf;
	/* Protects `cmd_buf` and `resp_buf` */
	struct mutex bufs_lock;

	/* Waitqueue for host responses on the command queue */
	wait_queue_head_t wq;
};

static inline struct virtio_v4l2 *to_virtio_v4l2(struct video_device *video_dev)
{
	return container_of(video_dev, struct virtio_v4l2, video_dev);
}

#define VIRTIO_V4L2_LAST_QUEUE (V4L2_BUF_TYPE_META_OUTPUT)

struct virtio_v4l2_queue_state {
	/* Whether the queue is currently streaming */
	bool streaming;
	/* How many buffers are currently allocated */
	size_t allocated_bufs;
	/* How many buffers are currently queued to the host */
	size_t queued_bufs;
	/* Buffers that can be dequeued */
	struct list_head pending_dqbufs;
};

/*
 * Convert planar buffer types to non-planar. Used to index the "queues" field
 * of virtio_v4l2_session and harmonize all code around non-planar queue types.
 */
static enum v4l2_buf_type buf_nonplanar(enum v4l2_buf_type buf)
{
	switch (buf) {
	case V4L2_BUF_TYPE_VIDEO_CAPTURE_MPLANE:
		return V4L2_BUF_TYPE_VIDEO_CAPTURE;
	case V4L2_BUF_TYPE_VIDEO_OUTPUT_MPLANE:
		return V4L2_BUF_TYPE_VIDEO_OUTPUT;
	default:
		return buf;
	}
}

/**
 * A session on a virtio_v4l2 device, created whenever the device is opened.
 */
struct virtio_v4l2_session {
	struct v4l2_fh fh;

	/* Session ID used to communicate with the host */
	u32 id;

	/*
	 * Whether dequeue should block or not (nonblocking if file opened with
	 * O_NONBLOCK).
	 */
	bool nonblocking_dequeue;

	/*
	 * Command and response buffers. Since communication with the host is
	 * synchronous, we only need one pair per session.
	 */
	void *cmd_buf, *resp_buf;

	/* State of all the queues */
	struct virtio_v4l2_queue_state queues[VIRTIO_V4L2_LAST_QUEUE + 1];
	/* Protects pending_dqbufs */
	struct mutex dqbufs_lock;
	/* Waitqueue for dequeued buffers, if VIDIOC_DQBUF needs to block or when polling. */
	wait_queue_head_t dqbufs_wait;

	/* Link into the list of sessions */
	struct list_head list;
};

static inline struct virtio_v4l2_session *fh_to_session(struct v4l2_fh *fh)
{
	return container_of(fh, struct virtio_v4l2_session, fh);
}

/**
 * Size of our virtio buffers. 16K will definitely be enough to contain anything we need.
 */
#define VIRTIO_BUF_SIZE 0x4000

/**
 * Allocate a new session. The id and list fields must still be set by the caller.
 */
static struct virtio_v4l2_session *
virtio_v4l2_session_alloc(struct virtio_v4l2 *vv, u32 id,
			  bool nonblocking_dequeue)
{
	struct virtio_v4l2_session *session;
	int i;

	session = devm_kzalloc(&vv->virtio_dev->dev, sizeof(*session),
			       GFP_KERNEL);
	if (!session)
		goto err_session;

	session->cmd_buf =
		devm_kzalloc(&vv->virtio_dev->dev, VIRTIO_BUF_SIZE, GFP_KERNEL);
	if (!session->cmd_buf)
		goto err_cmd_buf;

	session->resp_buf =
		devm_kzalloc(&vv->virtio_dev->dev, VIRTIO_BUF_SIZE, GFP_KERNEL);
	if (!session->resp_buf)
		goto err_resp_buf;

	session->id = id;
	session->nonblocking_dequeue = nonblocking_dequeue;

	INIT_LIST_HEAD(&session->list);
	v4l2_fh_init(&session->fh, &vv->video_dev);
	v4l2_fh_add(&session->fh);

	for (i = 0; i <= VIRTIO_V4L2_LAST_QUEUE; i++)
		INIT_LIST_HEAD(&session->queues[i].pending_dqbufs);
	mutex_init(&session->dqbufs_lock);

	init_waitqueue_head(&session->dqbufs_wait);

	mutex_lock(&vv->sessions_lock);
	list_add_tail(&session->list, &vv->sessions);
	mutex_unlock(&vv->sessions_lock);

	return session;

err_resp_buf:
	devm_kfree(&vv->virtio_dev->dev, session->cmd_buf);
err_cmd_buf:
	devm_kfree(&vv->virtio_dev->dev, session);
err_session:
	return ERR_PTR(-ENOMEM);
}

/**
 * Close and destroy `session`.
 */
static void virtio_v4l2_session_close(struct virtio_v4l2 *vv,
				      struct virtio_v4l2_session *session)
{
	v4l2_fh_del(&session->fh);
	v4l2_fh_exit(&session->fh);

	mutex_lock(&vv->sessions_lock);
	list_del(&session->list);
	mutex_unlock(&vv->sessions_lock);

	devm_kfree(&vv->virtio_dev->dev, session->resp_buf);
	devm_kfree(&vv->virtio_dev->dev, session->cmd_buf);
	devm_kfree(&vv->virtio_dev->dev, session);
}

/**
 * Lookup the session with `id`.
 */
static struct virtio_v4l2_session *
virtio_v4l2_find_session(struct virtio_v4l2 *vv, u32 id)
{
	struct list_head *p;
	struct virtio_v4l2_session *session = NULL;

	mutex_lock(&vv->sessions_lock);
	list_for_each(p, &vv->sessions) {
		struct virtio_v4l2_session *s =
			list_entry(p, struct virtio_v4l2_session, list);
		if (s->id == id) {
			session = s;
			break;
		}
	}
	mutex_unlock(&vv->sessions_lock);

	return session;
}

/*
 * Virtio protocol definition.
 */

#define VIRTIO_V4L2_CMD_OPEN 1
#define VIRTIO_V4L2_CMD_CLOSE 2
#define VIRTIO_V4L2_CMD_IOCTL 3
#define VIRTIO_V4L2_CMD_MMAP 4
#define VIRTIO_V4L2_CMD_MUNMAP 5

struct virtio_v4l2_cmd_header {
	u32 cmd;
	u32 __padding;
};

struct virtio_v4l2_resp_header {
	u32 status;
	u32 __padding;
};

struct virtio_v4l2_cmd_open {
	struct virtio_v4l2_cmd_header hdr;
};

struct virtio_v4l2_resp_open {
	struct virtio_v4l2_resp_header hdr;
	u32 session_id;
	u32 __padding;
};

struct virtio_v4l2_cmd_close {
	struct virtio_v4l2_cmd_header hdr;
	u32 session_id;
	u32 __padding;
};

struct virtio_v4l2_resp_close {
	struct virtio_v4l2_resp_header hdr;
};

struct virtio_v4l2_cmd_ioctl {
	struct virtio_v4l2_cmd_header hdr;
	u32 session_id;
	u32 code;
};

struct virtio_v4l2_resp_ioctl {
	struct virtio_v4l2_resp_header hdr;
};

#define VIRTIO_V4L2_MMAP_FLAG_RW (1 << 0)

struct virtio_v4l2_cmd_mmap {
	struct virtio_v4l2_cmd_header hdr;
	u32 session_id;
	u32 flags;
	u64 offset;
};

struct virtio_v4l2_resp_mmap {
	struct virtio_v4l2_resp_header hdr;
	u64 addr;
	u64 len;
};

struct virtio_v4l2_cmd_munmap {
	struct virtio_v4l2_cmd_header hdr;
	u64 offset;
};

struct virtio_v4l2_resp_munmap {
	struct virtio_v4l2_resp_header hdr;
};

#define VIRTIO_V4L2_EVT_ERROR 0
#define VIRTIO_V4L2_EVT_DQBUF 1
#define VIRTIO_V4L2_EVT_EVENT 2

struct virtio_v4l2_event_header {
	u32 event;
	u32 session_id;
};

/**
 * Host-side error.
 */
struct virtio_v4l2_event_error {
	struct virtio_v4l2_event_header hdr;
	u32 errno;
	u32 __padding;
};

/**
 * Signals that a buffer is not being used anymore on the host and can be
 * dequeued.
 */
struct virtio_v4l2_event_dqbuf {
	struct virtio_v4l2_event_header hdr;
	struct v4l2_buffer buffer;
	struct v4l2_plane planes[VIDEO_MAX_PLANES];
};

/**
 * Signals that a V4L2 event has been emitted for a stream.
 */
struct virtio_v4l2_event_event {
	struct virtio_v4l2_event_header hdr;
	struct v4l2_event event;
};

/*
 * End of virtio protocol definition.
 */

/**
 * Callback parameters to the virtio command queue.
 */
struct virtio_v4l2_cmd_callback_param {
	struct virtio_v4l2 *vv;
	/* Flag to switch once the command is completed */
	bool done_flag;
	/* Size of the received response */
	size_t resp_len;
};

/**
 * Callback for the command queue. This just wakes up the thread that was
 * waiting on the command to complete.
 */
static void commandq_callback(struct virtqueue *queue)
{
	unsigned int len;
	struct virtio_v4l2_cmd_callback_param *param;

	while ((param = virtqueue_get_buf(queue, &len))) {
		param->done_flag = true;
		param->resp_len = len;
		wake_up(&param->vv->wq);
	}

	virtqueue_enable_cb(queue);
}

/**
 * Send a command to the host and wait for its response.
 * @vv: the virtio_v4l2 device to communicate with.
 * @cmd: buffer containing the command to send.
 * @cmd_size: size of the command to send to the host.
 * @resp: buffer that will receive the response from the host.
 * @resp_size: must be initialized to the size that can be written in resp. The
 * actual size of the response from the host will also be written here.
 * @minimum_resp_size: the minimum size of the response expected by the caller
 * in case the command succeeded. Anything shorter than that will result in an
 * error.
 *
 * Returns 0 in case of success or an error code. If an error is returned,
 * resp_size and resp might not have been updated.
 */
static int virtio_v4l2_send_command(struct virtio_v4l2 *vv, void *cmd,
				    size_t cmd_size, void *resp,
				    size_t *resp_size, size_t minimum_resp_size)
{
	struct scatterlist *sgs[2], vcmd, vresp;
	struct virtio_v4l2_cmd_callback_param cb_param = {
		.vv = vv,
		.done_flag = false,
		.resp_len = 0,
	};
	struct virtio_v4l2_resp_header *resp_header;
	int ret;

	sg_init_one(&vcmd, cmd, cmd_size);
	sgs[0] = &vcmd;

	sg_init_one(&vresp, resp, *resp_size);
	sgs[1] = &vresp;

	ret = virtqueue_add_sgs(vv->commandq, sgs, 1, 1, &cb_param, GFP_ATOMIC);
	if (ret) {
		v4l2_err(&vv->v4l2_dev,
			 "failed to add sgs to command virtqueue\n");
		return ret;
	}

	if (!virtqueue_kick(vv->commandq)) {
		v4l2_err(&vv->v4l2_dev, "failed to kick command virtqueue\n");
		return -EINVAL;
	}

	/* Wait for the response. */
	ret = wait_event_timeout(vv->wq, cb_param.done_flag, 5 * HZ);
	if (ret == 0) {
		v4l2_err(&vv->v4l2_dev,
			 "timed out waiting for response to open command\n");
		return -EBUSY;
	}

	/* Make sure we have at least a response header - anything shorter is
	 * invalid. */
	if (cb_param.resp_len < sizeof(*resp_header)) {
		v4l2_err(&vv->v4l2_dev,
			 "received response header is too short\n");
		return -EINVAL;
	}

	resp_header = resp;

	/* Has the host succeeded in serving the command? */
	if (resp_header->status != 0)
		/* Host returns a positive error code. */
		return -resp_header->status;

	*resp_size = cb_param.resp_len;

	/* Make sure the host wrote a complete reply. */
	if (*resp_size < minimum_resp_size) {
		v4l2_err(
			&vv->v4l2_dev,
			"received response is too short: received %d, expected at least %d\n",
			*resp_size, minimum_resp_size);
		return -EINVAL;
	}

	return 0;
}

/**
 * Send the event buffer to the host so it can return it back to us filled with
 * the next event that occurred.
 */
static int virtio_v4l2_send_event_buffer(struct virtio_v4l2 *vv)
{
	struct scatterlist *sgs[1], vresp;
	int ret;

	sg_init_one(&vresp, vv->event_buffer, VIRTIO_BUF_SIZE);
	sgs[0] = &vresp;

	ret = virtqueue_add_sgs(vv->eventq, sgs, 0, 1, vv, GFP_ATOMIC);
	if (ret) {
		v4l2_err(&vv->v4l2_dev,
			 "failed to add sgs to event virtqueue\n");
		return ret;
	}

	if (!virtqueue_kick(vv->eventq)) {
		v4l2_err(&vv->v4l2_dev, "failed to kick event virtqueue\n");
		return -EINVAL;
	}

	return 0;
}

static void eventq_callback(struct virtqueue *queue)
{
	struct virtio_v4l2 *vv = queue->vdev->priv;

	schedule_work(&vv->eventq_work);
}

/**
 * Event callback. This processes the returned event buffer and immediately
 * sends it again to the host so it can send us the next event without ever
 * starving.
 */
void virtio_v4l2_event_work(struct work_struct *work)
{
	struct virtio_v4l2 *vv =
		container_of(work, struct virtio_v4l2, eventq_work);
	struct virtqueue *queue = vv->eventq;
	struct virtio_v4l2_event_error *error_evt;
	struct virtio_v4l2_event_dqbuf *dqbuf_evt;
	struct virtio_v4l2_event_event *event_evt;
	struct virtio_v4l2_session *session;
	struct virtio_v4l2_pending_dqbuf *dqbuf;
	unsigned int len;

	while ((vv = virtqueue_get_buf(queue, &len))) {
		struct virtio_v4l2_event_header *evt = vv->event_buffer;

		/* Make sure we received enough data */
		if (len < sizeof(*evt)) {
			v4l2_err(
				&vv->v4l2_dev,
				"event is too short: got %d, expected at least %d\n",
				len, sizeof(*evt));
			continue;
		}

		session = virtio_v4l2_find_session(vv, evt->session_id);
		if (session == NULL) {
			v4l2_err(&vv->v4l2_dev, "cannot find session %d\n",
				 evt->session_id);
			continue;
		}

		switch (evt->event) {
		case VIRTIO_V4L2_EVT_ERROR:
			if (len < sizeof(*error_evt)) {
				v4l2_err(
					&vv->v4l2_dev,
					"error event is too short: got %d, expected %d\n",
					len, sizeof(*error_evt));
				break;
			}
			error_evt = vv->event_buffer;
			v4l2_err(&vv->v4l2_dev,
				 "received error %d for session %d",
				 error_evt->errno, error_evt->hdr.session_id);
			break;

		/*
		 * Dequeued buffer: put it into the right queue so user-space can dequeue
		 * it.
		 */
		case VIRTIO_V4L2_EVT_DQBUF:
			if (len < sizeof(*dqbuf_evt)) {
				v4l2_err(
					&vv->v4l2_dev,
					"dqbuf event is too short: got %d, expected %d\n",
					len, sizeof(*dqbuf_evt));
				break;
			}
			dqbuf_evt = vv->event_buffer;
			if (dqbuf_evt->buffer.type > VIRTIO_V4L2_LAST_QUEUE) {
				v4l2_err(
					&vv->v4l2_dev,
					"unmanaged queue %d passed to dqbuf event",
					dqbuf_evt->buffer.type);
				break;
			}
			dqbuf = devm_kzalloc(&vv->virtio_dev->dev,
					     sizeof(*dqbuf), GFP_KERNEL);
			if (!dqbuf) {
				v4l2_err(
					&vv->v4l2_dev,
					"failed to allocate memory for pending dequeued buffer");
				break;
			}
			memcpy(&dqbuf->buffer, &dqbuf_evt->buffer,
			       sizeof(dqbuf->buffer));
			memcpy(&dqbuf->planes, &dqbuf_evt->planes,
			       sizeof(dqbuf->planes));

			mutex_lock(&session->dqbufs_lock);
			list_add_tail(&dqbuf->list,
				      &session
					       ->queues[buf_nonplanar(
						       dqbuf->buffer.type)]
					       .pending_dqbufs);
			mutex_unlock(&session->dqbufs_lock);

			session->queues[buf_nonplanar(dqbuf->buffer.type)]
				.queued_bufs -= 1;

			wake_up(&session->dqbufs_wait);
			break;

		case VIRTIO_V4L2_EVT_EVENT:
			if (len < sizeof(*event_evt)) {
				v4l2_err(
					&vv->v4l2_dev,
					"stream event is too short: got %d, expected %d\n",
					len, sizeof(*event_evt));
				break;
			}

			event_evt = vv->event_buffer;
			v4l2_event_queue_fh(&session->fh, &event_evt->event);
			break;

		default:
			v4l2_err(&vv->v4l2_dev, "unknown event type %d\n",
				 evt->event);
			break;
		}

		virtio_v4l2_send_event_buffer(vv);
	}

	virtqueue_enable_cb(queue);
}

/**
 * Opens the device and create a new session.
 */
static int virtio_v4l2_device_open(struct file *file)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_cmd_open *cmd_open = vv->cmd_buf;
	struct virtio_v4l2_resp_open *resp_open = vv->resp_buf;
	struct virtio_v4l2_session *session;
	size_t resp_len;
	u32 session_id;
	int ret;

	mutex_lock(&vv->bufs_lock);
	cmd_open->hdr.cmd = VIRTIO_V4L2_CMD_OPEN;
	resp_len = sizeof(*resp_open);
	ret = virtio_v4l2_send_command(vv, cmd_open, sizeof(*cmd_open),
				       resp_open, &resp_len,
				       sizeof(*resp_open));
	session_id = resp_open->session_id;
	mutex_unlock(&vv->bufs_lock);
	if (ret != 0)
		return ret;

	session = virtio_v4l2_session_alloc(vv, session_id,
					    (file->f_flags & O_NONBLOCK));
	if (IS_ERR(session))
		return PTR_ERR(session);

	file->private_data = &session->fh;

	return 0;
}

/**
 * Close a previously opened session.
 */
static int virtio_v4l2_device_close(struct file *file)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(file->private_data);
	struct virtio_v4l2_cmd_close *cmd_close = session->cmd_buf;
	struct virtio_v4l2_resp_close *resp_close = session->resp_buf;
	size_t resp_len;
	int ret;

	cmd_close->hdr.cmd = VIRTIO_V4L2_CMD_CLOSE;
	cmd_close->session_id = session->id;
	resp_len = sizeof(*resp_close);
	ret = virtio_v4l2_send_command(vv, cmd_close, sizeof(*cmd_close),
				       resp_close, &resp_len,
				       sizeof(*resp_close));
	if (ret != 0)
		return ret;

	virtio_v4l2_session_close(vv, session);

	return 0;
}

/**
 * Implements poll logic for a virtio-v4l2 device.
 */
static __poll_t virtio_v4l2_device_poll(struct file *file, poll_table *wait)
{
	struct virtio_v4l2_session *session = fh_to_session(file->private_data);
	struct virtio_v4l2_queue_state *input_queue =
		&session->queues[V4L2_BUF_TYPE_VIDEO_CAPTURE];
	struct virtio_v4l2_queue_state *output_queue =
		&session->queues[V4L2_BUF_TYPE_VIDEO_OUTPUT];
	__poll_t req_events = poll_requested_events(wait);
	__poll_t rc = 0;

	poll_wait(file, &session->dqbufs_wait, wait);
	poll_wait(file, &session->fh.wait, wait);

	/*
	 * This function is adequate for m2m devices, however we may need to detect
	 * the device type and provide variants if this doesn't work with other kinds
	 * of devices.
	 */

	mutex_lock(&session->dqbufs_lock);
	if (req_events & (EPOLLIN | EPOLLRDNORM | EPOLLOUT | EPOLLWRNORM)) {
		if ((!input_queue->streaming ||
		     input_queue->queued_bufs == 0) &&
		    (!output_queue->streaming ||
		     output_queue->queued_bufs == 0)) {
			rc |= EPOLLERR;
		} else {
			if (!list_empty(&input_queue->pending_dqbufs))
				rc |= EPOLLIN | EPOLLRDNORM;

			if (!list_empty(&output_queue->pending_dqbufs))
				rc |= EPOLLOUT | EPOLLWRNORM;
		}
	}
	mutex_unlock(&session->dqbufs_lock);

	if (v4l2_event_pending(&session->fh)) {
		rc |= EPOLLPRI;
	}

	return rc;
}

/**
 * Inform the host that a previously created MMAP mapping is no longer needed
 * and can be removed.
 */
static void virtio_v4l2_vma_close(struct vm_area_struct *vma)
{
	struct virtio_v4l2 *vv = vma->vm_private_data;
	struct virtio_v4l2_cmd_munmap *cmd_munmap = vv->cmd_buf;
	struct virtio_v4l2_resp_munmap *resp_munmap = vv->resp_buf;
	size_t resp_len;
	int ret;

	mutex_lock(&vv->bufs_lock);
	cmd_munmap->hdr.cmd = VIRTIO_V4L2_CMD_MUNMAP;
	cmd_munmap->offset = vma->vm_pgoff << PAGE_SHIFT;
	resp_len = sizeof(*resp_munmap);
	ret = virtio_v4l2_send_command(vv, cmd_munmap, sizeof(*cmd_munmap),
				       resp_munmap, &resp_len,
				       sizeof(*resp_munmap));
	mutex_unlock(&vv->bufs_lock);
	if (ret) {
		v4l2_err(&vv->v4l2_dev, "host failed to unmap buffer: %d\n",
			 ret);
	}
}

static struct vm_operations_struct virtio_v4l2_vm_ops = {
	.close = virtio_v4l2_vma_close,
};

/**
 * Perform a mmap request from the guest.
 *
 * This requests the host to map a MMAP buffer for us, so we can make that
 * mapping visible into the user-space address space.
 */
static int virtio_v4l2_device_mmap(struct file *file,
				   struct vm_area_struct *vma)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(file->private_data);
	struct virtio_v4l2_cmd_mmap *cmd_mmap = session->cmd_buf;
	struct virtio_v4l2_resp_mmap *resp_mmap = session->resp_buf;
	size_t resp_len;
	int ret;

	if (!(vma->vm_flags & VM_SHARED))
		return -EINVAL;
	if (!(vma->vm_flags & (VM_READ | VM_WRITE)))
		return -EINVAL;

	cmd_mmap->hdr.cmd = VIRTIO_V4L2_CMD_MMAP;
	cmd_mmap->session_id = session->id;
	cmd_mmap->flags =
		(vma->vm_flags & VM_WRITE) ? VIRTIO_V4L2_MMAP_FLAG_RW : 0;
	cmd_mmap->offset = vma->vm_pgoff << PAGE_SHIFT;
	resp_len = sizeof(*resp_mmap);

	/*
	 * The host performs reference counting and is smart enough to return the
	 * same guest physical address if this is called several times on the same
	 * buffer.
	 * */
	ret = virtio_v4l2_send_command(vv, cmd_mmap, sizeof(*cmd_mmap),
				       resp_mmap, &resp_len,
				       sizeof(*resp_mmap));
	if (ret)
		return ret;

	vma->vm_private_data = vv;

	if (vma->vm_end - vma->vm_start > PAGE_ALIGN(resp_mmap->len)) {
		virtio_v4l2_vma_close(vma);
		return -EINVAL;
	}

	ret = io_remap_pfn_range(vma, vma->vm_start,
				 resp_mmap->addr >> PAGE_SHIFT,
				 vma->vm_end - vma->vm_start,
				 vma->vm_page_prot);
	if (ret)
		return ret;

	vma->vm_ops = &virtio_v4l2_vm_ops;

	return 0;
}

static const struct v4l2_file_operations virtio_v4l2_fops = {
	.owner = THIS_MODULE,
	.open = virtio_v4l2_device_open,
	.release = virtio_v4l2_device_close,
	.poll = virtio_v4l2_device_poll,
	.unlocked_ioctl = video_ioctl2,
	.mmap = virtio_v4l2_device_mmap,
};

/* Convert a V4L2 IOCTL into the IOCTL code we can give to the host */
#define VIRTIO_V4L2_IOCTL_CODE(IOCTL) ((IOCTL >> _IOC_NRSHIFT) & _IOC_NRMASK)

/**
 * Send an ioctl that does not expect a reply beyond an error status (i.e. an
 * ioctl specified with _IOW) to the host.
 */
static int virtio_v4l2_send_w_ioctl(struct v4l2_fh *fh, u32 ioctl_code,
				    const void *ioctl_data,
				    size_t ioctl_data_len)
{
	struct video_device *video_dev = fh->vdev;
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	struct virtio_v4l2_cmd_ioctl *cmd_ioctl = session->cmd_buf;
	struct virtio_v4l2_resp_ioctl *resp_ioctl = session->resp_buf;
	size_t resp_len = VIRTIO_BUF_SIZE;

	cmd_ioctl->hdr.cmd = VIRTIO_V4L2_CMD_IOCTL;
	cmd_ioctl->session_id = session->id;
	cmd_ioctl->code = ioctl_code;
	memcpy(cmd_ioctl + 1, ioctl_data, ioctl_data_len);

	return virtio_v4l2_send_command(vv, cmd_ioctl,
					sizeof(*cmd_ioctl) + ioctl_data_len,
					resp_ioctl, &resp_len,
					sizeof(*resp_ioctl));
}

/**
 * Sends an ioctl that expects a response of exactly the same size as the
 * input (i.e. an ioctl specified with _IOWR) to the host.
 *
 * This corresponds to what most V4L2 ioctls do. For instance VIDIOC_ENUM_FMT
 * takes a partially-initialized struct v4l2_fmtdesc and returns its filled
 * version.
 *
 * Ioctls specified with _IOR can also use this, since the host will simply
 * ignore the extra input data provided.
 */
static int virtio_v4l2_send_wr_ioctl(struct v4l2_fh *fh, u32 ioctl_code,
				     void *ioctl_data, size_t ioctl_data_len)
{
	struct video_device *video_dev = fh->vdev;
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	struct virtio_v4l2_cmd_ioctl *cmd_ioctl = session->cmd_buf;
	struct virtio_v4l2_resp_ioctl *resp_ioctl = session->resp_buf;
	size_t resp_len = VIRTIO_BUF_SIZE;
	int ret;

	cmd_ioctl->hdr.cmd = VIRTIO_V4L2_CMD_IOCTL;
	cmd_ioctl->session_id = session->id;
	cmd_ioctl->code = ioctl_code;
	memcpy(cmd_ioctl + 1, ioctl_data, ioctl_data_len);

	ret = virtio_v4l2_send_command(vv, cmd_ioctl,
				       sizeof(*cmd_ioctl) + ioctl_data_len,
				       resp_ioctl, &resp_len,
				       sizeof(*resp_ioctl) + ioctl_data_len);
	if (ret)
		return ret;

	resp_len -= sizeof(*resp_ioctl);

	/* Make sure that the reply's length is the same as the input */
	if (resp_len != ioctl_data_len)
		return -EINVAL;

	memcpy(ioctl_data, resp_ioctl + 1, resp_len);

	return 0;
}

/**
 * Sends an ioctl send sends and receive a v4l2_buffer to the host.
 *
 * v4l2_buffer has potentially another user-space pointer that we need to copy
 * from/to, hence the dedicated function.
 *
 */
static int virtio_v4l2_send_buffer_ioctl(struct v4l2_fh *fh, u32 ioctl_code,
					 struct v4l2_buffer *b)
{
	/* Staging area for the v4l2_buffer we will send to the virtqueue */
	struct {
		struct v4l2_buffer buf;
		struct v4l2_plane planes[VIDEO_MAX_PLANES];
	} staging_area;
	struct v4l2_plane *user_planes = b->m.planes;
	int ret;

	memset(&staging_area, 0, sizeof(staging_area));

	/* TODO We should also convert single planar formats to multi-planar here? */
	memcpy(&staging_area.buf, b, sizeof(*b));
	if (V4L2_TYPE_IS_MULTIPLANAR(b->type)) {
		const size_t planes_size =
			sizeof(struct v4l2_plane) * b->length;

		memcpy(&staging_area.planes[0], b->m.planes, planes_size);
		staging_area.buf.m.planes = NULL;
	}

	ret = virtio_v4l2_send_wr_ioctl(fh, ioctl_code, &staging_area,
					sizeof(staging_area));
	if (ret)
		return ret;

	memcpy(b, &staging_area.buf, sizeof(*b));
	if (V4L2_TYPE_IS_MULTIPLANAR(b->type)) {
		b->m.planes = user_planes;
		memcpy(b->m.planes, &staging_area.planes[0],
		       sizeof(struct v4l2_plane) * b->length);
	}

	return 0;
}

/**
 * Helper function to clear the list of buffers waiting to be dequeued on a
 * queue that has just been streamed off.
 */
static void
virtio_v4l2_clear_pending_dqbufs(struct virtio_v4l2 *vv,
				 struct virtio_v4l2_session *session,
				 enum v4l2_buf_type queue)
{
	struct list_head *p, *n;

	if (queue > VIRTIO_V4L2_LAST_QUEUE)
		return;

	mutex_lock(&session->dqbufs_lock);

	list_for_each_safe(
		p, n, &session->queues[buf_nonplanar(queue)].pending_dqbufs) {
		struct virtio_v4l2_pending_dqbuf *dqbuf =
			list_entry(p, struct virtio_v4l2_pending_dqbuf, list);

		list_del(&dqbuf->list);
		devm_kfree(&vv->virtio_dev->dev, dqbuf);
	}

	mutex_unlock(&session->dqbufs_lock);
}

/*
 * V4L2 ioctl handlers.
 *
 * Most of these functions just forward the ioctl to the host, with some
 * exceptions. Most notably, DQBUF is not forwarded since the host notifies us
 * of dequeued buffers using an event.
 */

static int virtio_v4l2_querycap(struct file *file, void *fh,
				struct v4l2_capability *cap)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);

	strncpy(cap->driver, DRIVER_NAME, sizeof(cap->driver));
	virtio_cread_bytes(vv->virtio_dev, 8, cap->card, sizeof(cap->card));
	snprintf(cap->bus_info, sizeof(cap->bus_info), "virtio:%s",
		 video_dev->name);

	cap->capabilities = video_dev->device_caps | V4L2_CAP_DEVICE_CAPS;
	cap->device_caps = video_dev->device_caps;

	return 0;
}

static int virtio_v4l2_enum_fmt(struct file *file, void *fh,
				struct v4l2_fmtdesc *fmt_desc)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_ENUM_FMT),
					fmt_desc, sizeof(*fmt_desc));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_g_fmt(struct file *file, void *fh,
			     struct v4l2_format *format)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_G_FMT),
					format, sizeof(*format));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_try_fmt(struct file *file, void *fh,
			       struct v4l2_format *format)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_TRY_FMT),
					format, sizeof(*format));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_s_fmt(struct file *file, void *fh,
			     struct v4l2_format *format)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_S_FMT),
					format, sizeof(*format));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_queryctrl(struct file *file, void *fh,
				 struct v4l2_queryctrl *ctrl)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_QUERYCTRL), ctrl,
		sizeof(*ctrl));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_query_ext_ctrl(struct file *file, void *fh,
				      struct v4l2_query_ext_ctrl *ctrl)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_QUERY_EXT_CTRL), ctrl,
		sizeof(*ctrl));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_g_selection(struct file *file, void *fh,
				   struct v4l2_selection *s)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_G_SELECTION), s, sizeof(*s));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_s_selection(struct file *file, void *fh,
				   struct v4l2_selection *s)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_S_SELECTION), s, sizeof(*s));
	if (ret)
		return ret;

	return 0;
}

static int
virtio_v4l2_subscribe_event(struct v4l2_fh *fh,
			    const struct v4l2_event_subscription *sub)
{
	int ret;

	ret = virtio_v4l2_send_w_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_SUBSCRIBE_EVENT), sub,
		sizeof(*sub));
	if (ret)
		return ret;

	switch (sub->type) {
	case V4L2_EVENT_EOS:
		ret = v4l2_event_subscribe(fh, sub, 1, NULL);
		break;
	case V4L2_EVENT_SOURCE_CHANGE:
		ret = v4l2_src_change_event_subscribe(fh, sub);
		break;
	}
	if (ret)
		return ret;

	return 0;
}

static int
virtio_v4l2_unsubscribe_event(struct v4l2_fh *fh,
			      const struct v4l2_event_subscription *sub)
{
	int ret;

	ret = virtio_v4l2_send_w_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_UNSUBSCRIBE_EVENT), sub,
		sizeof(*sub));
	if (ret)
		return ret;

	ret = v4l2_event_unsubscribe(fh, sub);
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_streamon(struct file *file, void *fh,
				enum v4l2_buf_type i)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	int ret;

	if (i > VIRTIO_V4L2_LAST_QUEUE) {
		v4l2_err(&vv->v4l2_dev, "unsupported queue: %d\n", i);
		return -EINVAL;
	}

	ret = virtio_v4l2_send_w_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_STREAMON), &i, sizeof(i));
	if (ret)
		return ret;

	session->queues[buf_nonplanar(i)].streaming = true;

	return 0;
}

static int virtio_v4l2_streamoff(struct file *file, void *fh,
				 enum v4l2_buf_type i)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	struct virtio_v4l2_queue_state *queue;
	int ret;

	if (i > VIRTIO_V4L2_LAST_QUEUE) {
		v4l2_err(&vv->v4l2_dev, "unsupported queue: %d\n", i);
		return -EINVAL;
	}

	ret = virtio_v4l2_send_w_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_STREAMOFF), &i, sizeof(i));
	if (ret)
		return ret;

	queue = &session->queues[buf_nonplanar(i)];

	queue->streaming = false;
	queue->queued_bufs = 0;

	virtio_v4l2_clear_pending_dqbufs(vv, session, i);

	return 0;
}

static int virtio_v4l2_reqbufs(struct file *file, void *fh,
			       struct v4l2_requestbuffers *b)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	struct virtio_v4l2_queue_state *queue;
	int ret;

	if (b->type > VIRTIO_V4L2_LAST_QUEUE) {
		v4l2_err(&vv->v4l2_dev, "unsupported queue: %d\n", b->type);
		return -EINVAL;
	}

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_REQBUFS), b, sizeof(*b));
	if (ret)
		return ret;

	queue = &session->queues[buf_nonplanar(b->type)];

	/* REQBUFS(0) is an implicit STREAMOFF. */
	if (b->count == 0) {
		virtio_v4l2_clear_pending_dqbufs(vv, session, b->type);
		queue->queued_bufs = 0;
		queue->streaming = false;
	}

	queue->allocated_bufs = b->count;

	return 0;
}

static int virtio_v4l2_querybuf(struct file *file, void *fh,
				struct v4l2_buffer *b)
{
	int ret;

	ret = virtio_v4l2_send_buffer_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_QUERYBUF), b);
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_qbuf(struct file *file, void *fh, struct v4l2_buffer *b)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(fh);
	int ret;

	if (b->type > VIRTIO_V4L2_LAST_QUEUE) {
		v4l2_err(&vv->v4l2_dev, "unsupported queue: %d\n", b->type);
		return -EINVAL;
	}

	ret = virtio_v4l2_send_buffer_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_QBUF), b);
	if (ret)
		return ret;

	session->queues[buf_nonplanar(b->type)].queued_bufs += 1;

	return 0;
}

static int virtio_v4l2_dqbuf(struct file *file, void *fh, struct v4l2_buffer *b)
{
	struct video_device *video_dev = video_devdata(file);
	struct virtio_v4l2 *vv = to_virtio_v4l2(video_dev);
	struct virtio_v4l2_session *session = fh_to_session(file->private_data);
	struct virtio_v4l2_pending_dqbuf *dqbuf;
	struct list_head *buffer_queue;
	struct v4l2_plane *planes = NULL;
	const bool is_multiplanar = V4L2_TYPE_IS_MULTIPLANAR(b->type);
	int ret;

	if (b->type > VIRTIO_V4L2_LAST_QUEUE) {
		v4l2_err(&vv->v4l2_dev, "unsupported queue for dqbuf: %d\n",
			 b->type);
		return -EINVAL;
	}

	buffer_queue = &session->queues[buf_nonplanar(b->type)].pending_dqbufs;

	/* Only block for a buffer if the file has been opened with O_NONBLOCK. */
	if (session->nonblocking_dequeue) {
		if (list_empty(buffer_queue))
			return -EAGAIN;
	} else {
		ret = wait_event_interruptible(session->dqbufs_wait,
					       !list_empty(buffer_queue));
		if (ret)
			return -EINTR;
	}

	mutex_lock(&session->dqbufs_lock);
	dqbuf = list_first_entry(buffer_queue, struct virtio_v4l2_pending_dqbuf,
				 list);
	list_del(&dqbuf->list);
	mutex_unlock(&session->dqbufs_lock);

	if (is_multiplanar) {
		size_t nb_planes = min(b->length, (u32)VIDEO_MAX_PLANES);
		memcpy(b->m.planes, dqbuf->planes,
		       nb_planes * sizeof(struct v4l2_plane));
		planes = b->m.planes;
	}

	memcpy(b, &dqbuf->buffer, sizeof(*b));

	if (is_multiplanar) {
		b->m.planes = planes;
	}

	devm_kfree(&vv->virtio_dev->dev, dqbuf);

	return 0;
}

static int virtio_v4l2_enum_input(struct file *file, void *fh,
				  struct v4l2_input *input)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_ENUMINPUT), input,
		sizeof(*input));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_g_input(struct file *file, void *fh, unsigned int *i)
{
	u32 input;
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_G_INPUT),
					&input, sizeof(input));
	if (ret)
		return ret;

	*i = input;

	return 0;
}

static int virtio_v4l2_s_input(struct file *file, void *fh, unsigned int i)
{
	u32 input = i;
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_S_INPUT),
					&input, sizeof(input));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_enum_output(struct file *file, void *fh,
				   struct v4l2_output *output)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_ENUMOUTPUT), output,
		sizeof(*output));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_g_output(struct file *file, void *fh, unsigned int *o)
{
	u32 output;
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_G_OUTPUT),
					&output, sizeof(output));
	if (ret)
		return ret;

	*o = output;

	return 0;
}

static int virtio_v4l2_s_output(struct file *file, void *fh, unsigned int o)
{
	u32 output = o;
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(fh,
					VIRTIO_V4L2_IOCTL_CODE(VIDIOC_S_OUTPUT),
					&output, sizeof(output));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_encoder_cmd(struct file *file, void *fh,
				   struct v4l2_encoder_cmd *cmd)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_ENCODER_CMD), cmd,
		sizeof(*cmd));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_try_encoder_cmd(struct file *file, void *fh,
				       struct v4l2_encoder_cmd *cmd)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_TRY_ENCODER_CMD), cmd,
		sizeof(*cmd));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_decoder_cmd(struct file *file, void *fh,
				   struct v4l2_decoder_cmd *cmd)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_DECODER_CMD), cmd,
		sizeof(*cmd));
	if (ret)
		return ret;

	return 0;
}

static int virtio_v4l2_try_decoder_cmd(struct file *file, void *fh,
				       struct v4l2_decoder_cmd *cmd)
{
	int ret;

	ret = virtio_v4l2_send_wr_ioctl(
		fh, VIRTIO_V4L2_IOCTL_CODE(VIDIOC_TRY_DECODER_CMD), cmd,
		sizeof(*cmd));
	if (ret)
		return ret;

	return 0;
}

static const struct v4l2_ioctl_ops virtio_v4l2_ioctl_ops = {
	.vidioc_querycap = virtio_v4l2_querycap,
	.vidioc_enum_fmt_vid_cap = virtio_v4l2_enum_fmt,
	.vidioc_enum_fmt_vid_out = virtio_v4l2_enum_fmt,
	.vidioc_g_fmt_vid_cap = virtio_v4l2_g_fmt,
	.vidioc_g_fmt_vid_cap_mplane = virtio_v4l2_g_fmt,
	.vidioc_g_fmt_vid_out = virtio_v4l2_g_fmt,
	.vidioc_g_fmt_vid_out_mplane = virtio_v4l2_g_fmt,
	.vidioc_try_fmt_vid_cap = virtio_v4l2_try_fmt,
	.vidioc_try_fmt_vid_cap_mplane = virtio_v4l2_try_fmt,
	.vidioc_try_fmt_vid_out = virtio_v4l2_try_fmt,
	.vidioc_try_fmt_vid_out_mplane = virtio_v4l2_try_fmt,
	.vidioc_s_fmt_vid_cap = virtio_v4l2_s_fmt,
	.vidioc_s_fmt_vid_cap_mplane = virtio_v4l2_s_fmt,
	.vidioc_s_fmt_vid_out = virtio_v4l2_s_fmt,
	.vidioc_s_fmt_vid_out_mplane = virtio_v4l2_s_fmt,
	.vidioc_queryctrl = virtio_v4l2_queryctrl,
	.vidioc_query_ext_ctrl = virtio_v4l2_query_ext_ctrl,
	.vidioc_g_selection = virtio_v4l2_g_selection,
	.vidioc_s_selection = virtio_v4l2_s_selection,
	.vidioc_subscribe_event = virtio_v4l2_subscribe_event,
	.vidioc_unsubscribe_event = virtio_v4l2_unsubscribe_event,
	.vidioc_streamon = virtio_v4l2_streamon,
	.vidioc_streamoff = virtio_v4l2_streamoff,
	.vidioc_reqbufs = virtio_v4l2_reqbufs,
	.vidioc_querybuf = virtio_v4l2_querybuf,
	.vidioc_qbuf = virtio_v4l2_qbuf,
	.vidioc_dqbuf = virtio_v4l2_dqbuf,
	.vidioc_enum_input = virtio_v4l2_enum_input,
	.vidioc_g_input = virtio_v4l2_g_input,
	.vidioc_s_input = virtio_v4l2_s_input,
	.vidioc_enum_output = virtio_v4l2_enum_output,
	.vidioc_g_output = virtio_v4l2_g_output,
	.vidioc_s_output = virtio_v4l2_s_output,
	.vidioc_encoder_cmd = virtio_v4l2_encoder_cmd,
	.vidioc_try_encoder_cmd = virtio_v4l2_try_encoder_cmd,
	.vidioc_decoder_cmd = virtio_v4l2_decoder_cmd,
	.vidioc_try_decoder_cmd = virtio_v4l2_try_decoder_cmd,
};

static int virtio_v4l2_probe(struct virtio_device *virtio_dev)
{
	struct device *dev = &virtio_dev->dev;
	struct virtqueue *vqs[2];
	static vq_callback_t *vq_callbacks[] = {
		commandq_callback,
		eventq_callback,
	};
	static const char *const vq_names[] = { "command", "event" };
	struct virtio_v4l2 *vv;
	struct video_device *vd;
	static int ret;

	vv = devm_kzalloc(dev, sizeof(*vv), GFP_KERNEL);
	if (!vv)
		return -ENOMEM;

	vv->event_buffer = devm_kzalloc(dev, VIRTIO_BUF_SIZE, GFP_KERNEL);
	if (!vv->event_buffer) {
		return -ENOMEM;
	}

	vv->cmd_buf = devm_kzalloc(dev, VIRTIO_BUF_SIZE, GFP_KERNEL);
	if (!vv->cmd_buf)
		return -ENOMEM;

	vv->resp_buf = devm_kzalloc(dev, VIRTIO_BUF_SIZE, GFP_KERNEL);
	if (!vv->resp_buf)
		return -ENOMEM;

	mutex_init(&vv->bufs_lock);

	INIT_LIST_HEAD(&vv->sessions);
	mutex_init(&vv->sessions_lock);

	vv->virtio_dev = virtio_dev;
	virtio_dev->priv = vv;

	init_waitqueue_head(&vv->wq);

	/* TODO proper index. */
	dev_set_name(dev, "%s.%i", DRIVER_NAME, 0);

	ret = v4l2_device_register(dev, &vv->v4l2_dev);
	if (ret)
		return ret;

	ret = virtio_find_vqs(virtio_dev, 2, vqs, vq_callbacks, vq_names, NULL);
	if (ret)
		goto err_find_vqs;

	vv->commandq = vqs[0];
	vv->eventq = vqs[1];
	INIT_WORK(&vv->eventq_work, virtio_v4l2_event_work);

	virtio_device_ready(virtio_dev);

	vd = &vv->video_dev;

	vd->v4l2_dev = &vv->v4l2_dev;
	vd->vfl_type = VFL_TYPE_VIDEO;
	vd->ioctl_ops = &virtio_v4l2_ioctl_ops;
	vd->fops = &virtio_v4l2_fops;
	vd->device_caps = virtio_cread32(virtio_dev, 0);
	if (vd->device_caps & (V4L2_CAP_VIDEO_M2M | V4L2_CAP_VIDEO_M2M_MPLANE))
		vd->vfl_dir |= VFL_DIR_M2M;
	else if (vd->device_caps &
		 (V4L2_CAP_VIDEO_OUTPUT | V4L2_CAP_VIDEO_OUTPUT_MPLANE))
		vd->vfl_dir |= VFL_DIR_TX;
	else
		vd->vfl_dir = VFL_DIR_RX;
	vd->release = video_device_release_empty;
	strscpy(vd->name, "virtio-v4l2", sizeof(vd->name));

	video_set_drvdata(vd, vv);

	ret = video_register_device(vd, virtio_cread32(virtio_dev, 4), 0);
	if (ret)
		return ret;

	ret = virtio_v4l2_send_event_buffer(vv);
	if (ret) {
		goto send_event_buffer;
	}

	return 0;

send_event_buffer:
	video_unregister_device(&vv->video_dev);
	virtio_dev->config->del_vqs(virtio_dev);
err_find_vqs:
	v4l2_device_unregister(&vv->v4l2_dev);

	return ret;
}

static void virtio_v4l2_remove(struct virtio_device *virtio_dev)
{
	struct virtio_v4l2 *vv = virtio_dev->priv;
	struct list_head *p, *n;

	v4l2_device_unregister(&vv->v4l2_dev);
	virtio_dev->config->del_vqs(virtio_dev);
	video_unregister_device(&vv->video_dev);

	list_for_each_safe(p, n, &vv->sessions) {
		struct virtio_v4l2_session *s =
			list_entry(p, struct virtio_v4l2_session, list);

		virtio_v4l2_session_close(vv, s);
	}
}

static struct virtio_device_id id_table[] = {
	{ VIRTIO_ID_V4L2, VIRTIO_DEV_ANY_ID },
	{ 0 },
};

static unsigned int features[] = {};

static struct virtio_driver virtio_v4l2_driver = {
	.feature_table = features,
	.feature_table_size = ARRAY_SIZE(features),
	.driver.name = DRIVER_NAME,
	.driver.owner = THIS_MODULE,
	.id_table = id_table,
	.probe = virtio_v4l2_probe,
	.remove = virtio_v4l2_remove,
};

module_virtio_driver(virtio_v4l2_driver);

MODULE_DEVICE_TABLE(virtio, id_table);
MODULE_DESCRIPTION("virtio v4l2 driver");
MODULE_AUTHOR("Alexandre Courbot <acourbot@chromium.org>");
MODULE_LICENSE("GPL");
