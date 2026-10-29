// SPDX-License-Identifier: GPL-2.0

mod boot;
mod hal;

use kernel::{
    debugfs,
    device,
    dma::{
        Coherent,
        CoherentBox,
        CoherentView,
        DmaAddress, //
    },
    io::{
        io_project,
        io_write,
        Io, //
    },
    pci,
    prelude::*, //
};

pub(crate) mod cmdq;
pub(crate) mod commands;
mod fw;
mod nvkv;
mod regs;

pub(crate) use fw::{
    GspFmcBootParams,
    GspFwWprMeta,
    LibosMemoryRegionInitArgument,
    LibosParams, //
};
pub(crate) use hal::boot_firmware_files;

use crate::{
    driver::Bar0,
    falcon::{
        gsp::Gsp as GspFalcon,
        sec2::Sec2 as Sec2Falcon,
        Falcon, //
    },
    fsp::Fsp,
    gpu::Chipset,
    gsp::{
        cmdq::Cmdq,
        fw::GspArgumentsPadded, //
    },
    num,
    vgpu::VgpuManager, //
};

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;

/// Common context for the GSP boot process.
///
/// It carries two distinct lifetimes:
///
/// - `'gpu` is the lifetime of the bound GPU device, as captured by the GPU subdevices.
/// - `'ctx` is a shorter lifetime during which this context borrows those subdevices.
pub(crate) struct GspBootContext<'ctx, 'gpu> {
    pub(crate) pdev: &'gpu pci::Device<device::Bound>,
    pub(crate) bar: Bar0<'gpu>,
    pub(crate) chipset: Chipset,
    pub(crate) gsp_falcon: &'ctx Falcon<'gpu, GspFalcon>,
    pub(crate) sec2_falcon: &'ctx Falcon<'gpu, Sec2Falcon>,
    pub(crate) fsp: Option<&'ctx mut Fsp<'gpu>>,
    pub(crate) vgpu: &'ctx VgpuManager,
}

impl<'ctx, 'gpu> GspBootContext<'ctx, 'gpu> {
    pub(crate) fn dev(&self) -> &'gpu device::Device<device::Bound> {
        self.pdev.as_ref()
    }
}

/// Number of GSP pages to use in a RM log buffer.
const RM_LOG_BUFFER_NUM_PAGES: usize = 0x10;

/// Array of page table entries, as understood by the GSP bootloader.
#[repr(C)]
#[derive(FromBytes, IntoBytes)]
struct PteArray<const NUM_ENTRIES: usize>([u64; NUM_ENTRIES]);

impl<const NUM_PAGES: usize> PteArray<NUM_PAGES> {
    /// Initialize a new page table array mapping `NUM_PAGES` GSP pages starting at address `start`.
    fn init(view: CoherentView<'_, Self>, start: DmaAddress) -> Result<()> {
        for i in 0..NUM_PAGES {
            io_write!(view, .0[build: i],
                start
                    .checked_add(num::usize_as_u64(i) << GSP_PAGE_SHIFT)
                    .ok_or(EOVERFLOW)?
            );
        }

        Ok(())
    }
}

/// The logging buffers are byte queues that contain encoded printf-like
/// messages from GSP-RM.  They need to be decoded by a special application
/// that can parse the buffers.
///
/// The 'loginit' buffer contains logs from early GSP-RM init and
/// exception dumps.  The 'logrm' buffer contains the subsequent logs. Both are
/// written to directly by GSP-RM and can be any multiple of GSP_PAGE_SIZE.
///
/// The physical address map for the log buffer is stored in the buffer
/// itself, starting with offset 1. Offset 0 contains the "put" pointer (pp).
/// Initially, pp is equal to 0. If the buffer has valid logging data in it,
/// then pp points to index into the buffer where the next logging entry will
/// be written. Therefore, the logging data is valid if:
///   1 <= pp < sizeof(buffer)/sizeof(u64)
struct LogBuffer<'a, const NUM_PAGES: usize>(Coherent<'a, [[u8; GSP_PAGE_SIZE]; NUM_PAGES]>);

/// A log buffer at the default size, [`RM_LOG_BUFFER_NUM_PAGES`] pages.
///
/// Matches the registry defaults for the init, interrupt, RM and MNOC tasks
/// (`NV_REG_STR_RM_GSP_LOG_BUFFER_SIZE_TASK_*_DEFAULT`).
type TaskLogBuffer<'a> = LogBuffer<'a, RM_LOG_BUFFER_NUM_PAGES>;

/// A single-page log buffer, the size of the logs of the root task and of the RM state monitor
/// task.
type SmallLogBuffer<'a> = LogBuffer<'a, 1>;

impl<'a, const NUM_PAGES: usize> LogBuffer<'a, NUM_PAGES> {
    /// Creates a new `LogBuffer` mapped on `dev`.
    fn new(dev: &'a device::Device<device::Bound>) -> Result<Self> {
        let obj = Self(Coherent::zeroed(dev, GFP_KERNEL)?);

        let start_addr = obj.0.dma_address();

        let pte_view = io_project!(
            obj.0,
            [build: 0][build: size_of::<u64>()..][build: ..NUM_PAGES * size_of::<u64>()]
        )
        .try_cast::<PteArray<NUM_PAGES>>()?;
        PteArray::init(pte_view, start_addr)?;

        Ok(obj)
    }
}

/// The log buffers to which GSP-RM writes its debug output, one per LIBOS3 task.
///
/// The r000 firmware runs LIBOS3 on GA102 and later, and LIBOS2 on Turing and GA100. LIBOS2 maps
/// only the init and RM log buffers, and skips the names that it does not have.
struct LogBuffers<'a> {
    /// Init task.
    loginit: TaskLogBuffer<'a>,
    /// Interrupt task.
    logintr: TaskLogBuffer<'a>,
    /// RM task.
    logrm: TaskLogBuffer<'a>,
    /// MNOC task.
    logmnoc: TaskLogBuffer<'a>,
    /// Root task.
    logroot: SmallLogBuffer<'a>,
    /// RM state monitor task.
    logrmon: SmallLogBuffer<'a>,
}

impl<'a> LogBuffers<'a> {
    /// Number of log buffers.
    const COUNT: usize = 6;

    /// Allocates the six log buffers, mapped on `dev`.
    fn new(dev: &'a device::Device<device::Bound>) -> Result<Self> {
        Ok(Self {
            loginit: TaskLogBuffer::new(dev)?,
            logintr: TaskLogBuffer::new(dev)?,
            logrm: TaskLogBuffer::new(dev)?,
            logmnoc: TaskLogBuffer::new(dev)?,
            logroot: SmallLogBuffer::new(dev)?,
            logrmon: SmallLogBuffer::new(dev)?,
        })
    }

    /// Fills the first [`Self::COUNT`] entries of `libos` with the log buffers, under the names
    /// that GSP-RM looks them up by.
    fn init_arguments(
        &self,
        libos: &mut CoherentBox<'_, [LibosMemoryRegionInitArgument]>,
    ) -> Result {
        libos.init_at(
            0,
            LibosMemoryRegionInitArgument::new("LOGINIT", &self.loginit.0),
        )?;
        libos.init_at(
            1,
            LibosMemoryRegionInitArgument::new("LOGINTR", &self.logintr.0),
        )?;
        libos.init_at(
            2,
            LibosMemoryRegionInitArgument::new("LOGRM", &self.logrm.0),
        )?;
        libos.init_at(
            3,
            LibosMemoryRegionInitArgument::new("LOGMNOC", &self.logmnoc.0),
        )?;
        libos.init_at(
            4,
            LibosMemoryRegionInitArgument::new("LOGROOT", &self.logroot.0),
        )?;
        libos.init_at(
            5,
            LibosMemoryRegionInitArgument::new("LOGRMON", &self.logrmon.0),
        )?;

        Ok(())
    }

    /// Exposes each log buffer as a binary file in `dir`, under the lowercase form of its name.
    fn register_debugfs<'data>(&'data self, dir: &debugfs::ScopedDir<'data, '_>) {
        dir.read_binary_file(c"loginit", &self.loginit.0);
        dir.read_binary_file(c"logintr", &self.logintr.0);
        dir.read_binary_file(c"logrm", &self.logrm.0);
        dir.read_binary_file(c"logmnoc", &self.logmnoc.0);
        dir.read_binary_file(c"logroot", &self.logroot.0);
        dir.read_binary_file(c"logrmon", &self.logrmon.0);
    }
}

/// GSP runtime data.
#[pin_data]
pub(crate) struct Gsp<'gsp> {
    /// Libos arguments.
    pub(crate) libos: Coherent<'gsp, [LibosMemoryRegionInitArgument]>,
    /// Log buffers, optionally exposed via debugfs.
    #[pin]
    logs: debugfs::Scope<LogBuffers<'gsp>>,
    /// Command queue.
    #[pin]
    pub(crate) cmdq: Cmdq<'gsp>,
    /// RM arguments.
    rmargs: Coherent<'gsp, GspArgumentsPadded>,
    /// Buffer in which GSP-RM reports its own state.
    rm_state_monitor: Coherent<'gsp, [u8; GSP_PAGE_SIZE]>,
}

impl<'gsp> Gsp<'gsp> {
    // Creates an in-place initializer for a `Gsp` manager for `pdev`.
    pub(crate) fn new(
        pdev: &'gsp pci::Device<device::Bound>,
        bar: Bar0<'gsp>,
    ) -> impl PinInit<Self, Error> + 'gsp {
        pin_init::pin_init_scope(move || {
            let dev = pdev.as_ref();
            let log_buffers = LogBuffers::new(dev)?;

            Ok(try_pin_init!(Self {
                cmdq <- Cmdq::new(dev, bar),
                rm_state_monitor: Coherent::zeroed(dev, GFP_KERNEL)?,
                rmargs: Coherent::init(
                    dev,
                    GFP_KERNEL,
                    GspArgumentsPadded::new(&cmdq, rm_state_monitor),
                )?,
                libos: {
                    let mut libos = CoherentBox::zeroed_slice(
                        dev,
                        GSP_PAGE_SIZE / size_of::<LibosMemoryRegionInitArgument>(),
                        GFP_KERNEL,
                    )?;

                    log_buffers.init_arguments(&mut libos)?;
                    libos.init_at(
                        LogBuffers::COUNT,
                        LibosMemoryRegionInitArgument::new("RMARGS", rmargs),
                    )?;

                    libos.into()
                },
                logs <- {
                    #[allow(static_mut_refs)]
                    // SAFETY: `DEBUGFS_ROOT` is created before driver registration and cleared
                    // after driver unregistration, so no probe() can race with its modification.
                    //
                    // PANIC: `DEBUGFS_ROOT` cannot be `None` here.  It is set before driver
                    // registration and cleared after driver unregistration, so it is always
                    // `Some` for the entire lifetime that probe() can be called.
                    let log_parent: &debugfs::Dir = unsafe { crate::DEBUGFS_ROOT.as_ref() }
                        .expect("DEBUGFS_ROOT not initialized");

                    log_parent.scope(log_buffers, dev.name(), |logs, dir| {
                        logs.register_debugfs(dir)
                    })
                },
            }))
        })
    }
}

/// Opaque bundle required to unload the GSP. Created by [`Gsp::boot`], consumed by [`Gsp::unload`].
pub(crate) struct UnloadBundle<'a>(KBox<dyn hal::UnloadBundle + 'a>);

/// The results of [`Gsp::boot`]: the static GPU configuration and the unload bundle.
pub(crate) struct BootResult<'a> {
    /// The unload bundle for [`Gsp::unload`], if one could be built.
    pub(crate) unload_bundle: Option<UnloadBundle<'a>>,
    /// The static GPU configuration, as decoded from the `GSP_INIT` reply.
    pub(crate) static_info: commands::GspStaticInfo,
}
