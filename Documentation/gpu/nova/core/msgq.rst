.. SPDX-License-Identifier: GPL-2.0

==========================================
Nova GPU RPC Message Passing Architecture
==========================================

:Author: Nova driver authors.
:Date: July 2025

Overview
========

The Nova GPU driver communicates with the GSP (GPU System Processor) firmware
using an RPC (Remote Procedure Call) mechanism built on top of circular message
queues in shared memory. This document describes the structure of RPC messages
and the mechanics of the message passing system.

Message Queue Architecture
==========================

The communication between CPU and GSP uses two unidirectional circular queues:

1. **CPU Queue (cpuq)**: CPU writes, GSP reads
2. **GSP Queue (gspq)**: GSP writes, CPU reads

The advantage of this approach is no synchronization is required to access the
queues, if one entity wants to communicate with the other (CPU or GSP), they
simply write into their own queue.

Memory Layout
-------------

The shared memory region (GspMem) where the queues reside has the following
layout::

    +------------------------+ GspMem DMA Handle (base address)
    |    PTE Array (4KB)     |  <- Self-mapping page table
    | PTE[0] = base + 0x0000 |     Points to this page
    | PTE[1] = base + 0x1000 |     Points to CPU queue Header page
    | PTE[2] = base + 0x2000 |     Points to first page of CPU queue data
    | ...                    |     ...
    | ...                    |     ...
    +------------------------+ base + 0x1000
    |    CPU Queue Header    |  MsgqTxHeader + MsgqRxHeader
    |    - TX Header (32B)   |
    |    - RX Header (4B)    | (1 page)
    |    - Padding           |
    +------------------------+ base + 0x2000
    |    CPU Queue Data      | (63 pages)
    |    (63 x 4KB pages)    |  Circular buffer for messages
    | ...                    |     ...
    +------------------------+ base + 0x41000
    |    GSP Queue Header    |  MsgqTxHeader + MsgqRxHeader
    |    - TX Header (32B)   |
    |    - RX Header (4B)    | (1 page)
    |    - Padding           |
    +------------------------+ base + 0x42000
    |    GSP Queue Data      | (63 pages)
    |    (63 x 4KB pages)    |  Circular buffer for messages
    | ...                    |     ...
    +------------------------+ base + 0x81000


Message Passing Mechanics
-------------------------
The split read/write pointer design allows bidirectional communication between the
CPU and GSP without synchronization (if it were a shared queue), for example, the
following diagram illustrates pointer updates, when CPU sends message to GSP::

    +--------------------------------------------------------------------------+
    |                     DMA coherent Shared Memory (GspMem)                  |
    +--------------------------------------------------------------------------+
    |                          (CPU sending message to GSP)                    |
    |  +-------------------+                      +-------------------+        |
    |  |   GSP Queue       |                      |   CPU Queue       |        |
    |  |                   |                      |                   |        |
    |  | +-------------+   |                      | +-------------+   |        |
    |  | |  TX Header  |   |                      | |  TX Header  |   |        |
    |  | | write_ptr   |   |                      | | write_ptr   |---+----,   |
    |  | |             |   |                      | |             |   |    |   |
    |  | +-------------+   |                      | +-------------+   |    |   |
    |  |                   |                      |                   |    |   |
    |  | +-------------+   |                      | +-------------+   |    |   |
    |  | |  RX Header  |   |                      | |  RX Header  |   |    |   |
    |  | |  read_ptr ------+-------,              | |  read_ptr   |   |    |   |
    |  | |             |   |       |              | |             |   |    |   |
    |  | +-------------+   |       |              | +-------------+   |    |   |
    |  |                   |       |              |                   |    |   |
    |  | +-------------+   |       |              | +-------------+   |    |   |
    |  | |   Page 0    |   |       |              | |   Page 0    |   |    |   |
    |  | +-------------+   |       |              | +-------------+   |    |   |
    |  | |   Page 1    |   |       `--------------> |   Page 1    |   |    |   |
    |  | +-------------+   |                      | +-------------+   |    |   |
    |  | |   Page 2    |   |                      | |   Page 2    |<--+----'   | 
    |  | +-------------+   |                      | +-------------+   |        |
    |  | |     ...     |   |                      | |     ...     |   |        |
    |  | +-------------+   |                      | +-------------+   |        |
    |  | |   Page 62   |   |                      | |   Page 62   |   |        |
    |  | +-------------+   |                      | +-------------+   |        |
    |  |   (63 pages)      |                      |   (63 pages)      |        |
    |  +-------------------+                      +-------------------+        |
    |                                                                          |
    +--------------------------------------------------------------------------+

When the CPU sends a message to the GSP, it writes the message to its own
queue (CPU queue) and updates the write pointer in its queue's TX header. The GSP
then reads the read pointer in its own queue's RX header and knows that there are
pending messages from the CPU because its RX header's read pointer is behind the
CPU's TX header's write pointer. After reading the message, the GSP updates its RX
header's read pointer to catch up. The same happens in reverse.

Page-based message passing
--------------------------
The message queue is page-based, which means that the message is stored in a
page-aligned buffer. The page size is 4KB. Each message starts at the beginning of
a page. If the message is shorter than a page, the remaining space in the page is
wasted. The next message starts at the beginning of the next page no matter how
small the previous message was.

Note that messages larger than a page will span multiple pages. This means that
it is possible that the first part of the message lands on the last page, and the
second part of the message lands on the first page, thus requiring out-of-order
memory access. The SBuffer data structure in Nova tackles this use case.

RPC Message Structure:
======================

An RPC message is also called a "Message Element". The entire message has
multiple headers. There is a "message element" header which handles message
queue specific details and integrity, followed by a "RPC" header which handles
the RPC protocol details::

    +----------------------------------+
    |        GspMsgHeader (64B)        | (aka, Message Element Header)
    +----------------------------------+
    | auth_tag_buffer[16]              | --+
    | aad_buffer[16]                   |   |
    | checksum        (u32)            |   +-- Security & Integrity
    | sequence        (u32)            |   |
    | elem_count      (u32)            |   |
    | pad             (u32)            | --+
    +----------------------------------+
    |        GspRpcHeader (32B)        |
    +----------------------------------+
    | header_version  (0x03000000)     | --+
    | signature       (0x43505256)     |   |
    | length          (u32)            |   +-- RPC Protocol
    | function        (u32)            |   |
    | rpc_result      (u32)            |   |
    | rpc_result_private (u32)         |   |
    | sequence        (u32)            |   |
    | cpu_rm_gfid     (u32)            | --+
    +----------------------------------+
    |                                  |
    |        Payload (Variable)        | --- Function-specific data
    |                                  |
    +----------------------------------+
