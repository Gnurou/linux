// SPDX-License-Identifier: GPL-2.0

//! TLB (Translation Lookaside Buffer) flush support for GPU MMU.
//!
//! After modifying page table entries, the GPU's TLB must be flushed to
//! ensure the new mappings take effect. This module provides TLB flush
//! functionality for virtual memory managers.
//!
//! # Example
//!
//! ```ignore
//! use crate::mm::tlb::Tlb;
//!
//! fn page_table_update(tlb: &Tlb, pdb_addr: VramAddress) -> Result<()> {
//!     // ... modify page tables ...
//!
//!     // Flush TLB to make changes visible (polls for completion).
//!     tlb.flush(pdb_addr)?;
//!
//!     Ok(())
//! }
//! ```

#![allow(dead_code)]

use kernel::{
    devres::Devres,
    io::poll::read_poll_timeout,
    new_mutex,
    prelude::*,
    sync::{Arc, Mutex},
    time::Delta, //
};

use crate::{
    driver::Bar0,
    mm::VramAddress,
    regs, //
};

/// TLB manager for GPU translation buffer operations.
#[pin_data]
pub(crate) struct Tlb {
    bar: Arc<Devres<Bar0>>,
    /// TLB flush serialization lock:  This lock is acquired during the
    /// DMA fence signalling critical path. It must NEVER be held across any
    /// reclaimable CPU memory allocations because the memory reclaim path can
    /// call `dma_fence_wait()`, which would deadlock with this lock held.
    #[pin]
    lock: Mutex<()>,
}

impl Tlb {
    /// Create a new TLB manager.
    pub(super) fn new(bar: Arc<Devres<Bar0>>) -> impl PinInit<Self> {
        pin_init!(Self {
            bar,
            lock <- new_mutex!((), "tlb_flush"),
        })
    }

    /// Flush the GPU TLB for a specific page directory base.
    ///
    /// This invalidates all TLB entries associated with the given PDB address.
    /// Must be called after modifying page table entries to ensure the GPU sees
    /// the updated mappings.
    pub(crate) fn flush(&self, pdb_addr: VramAddress) -> Result {
        let _guard = self.lock.lock();

        let bar = self.bar.try_access().ok_or(ENODEV)?;

        // Write PDB address.
        regs::NV_TLB_FLUSH_PDB_LO::from_pdb_addr(pdb_addr.raw_u64()).write(&*bar);
        regs::NV_TLB_FLUSH_PDB_HI::from_pdb_addr(pdb_addr.raw_u64()).write(&*bar);

        // Trigger flush: invalidate all pages and enable.
        regs::NV_TLB_FLUSH_CTRL::default()
            .set_page_all(true)
            .set_enable(true)
            .write(&*bar);

        // Poll for completion - enable bit clears when flush is done.
        read_poll_timeout(
            || Ok(regs::NV_TLB_FLUSH_CTRL::read(&*bar)),
            |ctrl| !ctrl.enable(),
            Delta::ZERO,
            Delta::from_secs(2),
        )?;

        Ok(())
    }
}
