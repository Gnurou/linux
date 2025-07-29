=========================
PRAMIN aperture mechanism
=========================

:Author: Nova driver authors.
:Date: 2025

Introduction
============

PRAMIN is a hardware aperture mechanism that provides CPU access to GPU Video RAM (VRAM) before
the GPU's Memory Management Unit (MMU) and page tables are initialized. This 1MB sliding window,
located at a fixed offset within BAR0, is essential for setting up page tables and other critical
GPU data structures without relying on the GPU's MMU.

Architecture Overview
=====================

Logically, the PRAMIN aperture mechanism is implemented by the GPU's PBUS (PCIe Bus Controller Unit)
and provides a CPU-accessible window into VRAM through the PCIe interface::

    +-----------------+    PCIe     +------------------------------+
    |      CPU        |<----------->|           GPU                |
    +-----------------+             |                              |
                                    |  +----------------------+    |
                                    |  |       PBUS           |    |
                                    |  |  (Bus Controller)    |    |
                                    |  |                      |    |
                                    |  |  +--------------.<------------ (window always starts at BAR0 + 0x700000)
                                    |  |  |   PRAMIN     |    |    |
                                    |  |  |   Window     |    |    |
                                    |  |  |   (1MB)      |    |    |
                                    |  |  +--------------+    |    |
                                    |  |         |            |    |
                                    |  +---------|------------+    |
                                    |            |                 |
                                    |            v                 |
                                    |  .----------------------.<------------ (Program PRAMIN to any
                                    |  |       VRAM           |    |          64KB VRAM physicalboundary)
                                    |  |    (Several GBs)     |    |
                                    |  |                      |    |
                                    |  |  FB[0x000000000000]  |    |
                                    |  |          ...         |    |
                                    |  |  FB[0x7FFFFFFFFFF]   |    |
                                    |  +----------------------+    |
                                    +------------------------------+

PBUS (PCIe Bus Controller) among other things is responsible in the GPU for handling MMIO
accesses to the BAR registers.

PRAMIN Window Operation
=======================

The PRAMIN window provides a 1MB sliding aperture that can be repositioned over
the entire VRAM address space using the NV_PBUS_BAR0_WINDOW register.

Window Control Mechanism
-------------------------

The window position is controlled via the PBUS BAR0_WINDOW register::

    NV_PBUS_BAR0_WINDOW Register
    +-----+-----+--------------------------------------+
    |31-26|25-24|           23-0                       |
    |     |TARG |         BASE_ADDR                    |
    |     | ET  |        (bits 39:16)                  |
    +-----+-----+--------------------------------------+
    
    TARGET field values:
    - 0x0: VID_MEM (Video Memory / VRAM)
    - Other values are used for system memory access or reserved.

64KB Alignment Requirement
---------------------------

The PRAMIN window must be aligned to 64KB boundaries in VRAM. This is enforced
by the BASE_ADDR field representing bits [39:16] of the target address::

    VRAM Address Calculation:
    actual_vram_addr = (BASE_ADDR << 16) + pramin_offset
    
    Where:
    - BASE_ADDR: 24-bit value from NV_PBUS_BAR0_WINDOW[23:0]
    - pramin_offset: 20-bit offset within PRAMIN window [0x00000-0xFFFFF]
    
    Example Window Positioning:
    +---------------------------------------------------------+
    |                    VRAM Space                           |
    |                                                         |
    |  0x000000000  +-----------------+ <-- 64KB aligned      |
    |               | PRAMIN Window   |                       |
    |               |    (1MB)        |                       |
    |  0x0000FFFFF  +-----------------+                       |
    |                                                         |
    |       |              ^                                  |
    |       |              | Window can slide                 |
    |       v              | to any 64KB boundary             |
    |                                                         |
    |  0x123400000  +-----------------+ <-- 64KB aligned      |
    |               | PRAMIN Window   |                       |
    |               |    (1MB)        |                       |
    |  0x1234FFFFF  +-----------------+                       |
    |                                                         |
    |                       ...                               |
    |                                                         |
    |  0x7FFFF0000  +-----------------+ <-- 64KB aligned      |
    |               | PRAMIN Window   |                       |
    |               |    (1MB)        |                       |
    |  0x7FFFFFFFF  +-----------------+                       |
    +---------------------------------------------------------+
