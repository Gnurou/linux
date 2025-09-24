// SPDX-License-Identifier: GPL-2.0

mod boot;
mod fw;

pub(crate) use fw::{GspFwWprMeta, LibosParams};

use kernel::alloc::flags::GFP_KERNEL;
use kernel::device;
use kernel::dma::CoherentAllocation;
use kernel::dma::DmaAddress;
use kernel::dma_write;
use kernel::pci;
use kernel::prelude::*;
use kernel::ptr::Alignment;
use kernel::transmute::AsBytes;

use crate::fb::FbLayout;
use crate::gsp::cmdq::GspCmdq;

use fw::{
    LibosMemoryRegionInitArgument, GSP_ARGUMENTS_CACHED, GSP_SR_INIT_ARGUMENTS,
    MESSAGE_QUEUE_INIT_ARGUMENTS,
};

pub(crate) mod cmdq;
pub(crate) mod commands;

pub(crate) const GSP_PAGE_SHIFT: usize = 12;
pub(crate) const GSP_PAGE_SIZE: usize = 1 << GSP_PAGE_SHIFT;
pub(crate) const GSP_HEAP_ALIGNMENT: Alignment = Alignment::new::<{ 1 << 20 }>();

/// Number of GSP pages to use in a RM log buffer.
const RM_LOG_BUFFER_NUM_PAGES: usize = 0x10;

/// GSP runtime data.
#[pin_data]
pub(crate) struct Gsp {
    pub libos: CoherentAllocation<LibosMemoryRegionInitArgument>,
    pub loginit: CoherentAllocation<u8>,
    pub logintr: CoherentAllocation<u8>,
    pub logrm: CoherentAllocation<u8>,
    pub cmdq: GspCmdq,
    rmargs: CoherentAllocation<GSP_ARGUMENTS_CACHED>,
}

#[repr(C)]
struct PteArray<const NUM_ENTRIES: usize>([u64; NUM_ENTRIES]);

/// SAFETY: arrays of `u64` implement `AsBytes` and we are but a wrapper around it.
unsafe impl<const NUM_ENTRIES: usize> AsBytes for PteArray<NUM_ENTRIES> {}

impl<const NUM_PAGES: usize> PteArray<NUM_PAGES> {
    fn new(handle: DmaAddress) -> Self {
        let mut ptes = [0u64; NUM_PAGES];

        for (i, pte) in ptes.iter_mut().enumerate() {
            *pte = handle + ((i as u64) << GSP_PAGE_SHIFT);
        }

        Self(ptes)
    }
}

/// Creates a new `CoherentAllocation<A>` with `name` of `size` elements, and
/// register it into the `libos` object at argument position `libos_arg_nr`.
fn create_logbuffer_dma_object(
    dev: &device::Device<device::Bound>,
) -> Result<CoherentAllocation<u8>> {
    let mut obj = CoherentAllocation::<u8>::alloc_coherent(
        dev,
        RM_LOG_BUFFER_NUM_PAGES * GSP_PAGE_SIZE,
        GFP_KERNEL | __GFP_ZERO,
    )?;

    let ptes = PteArray::<RM_LOG_BUFFER_NUM_PAGES>::new(obj.dma_handle());

    // SAFETY: `obj` has just been created and we are its sole user.
    unsafe {
        // Copy the self-mapping PTE at the expected location.
        obj.as_slice_mut(size_of::<u64>(), size_of_val(&ptes))?
            .copy_from_slice(ptes.as_bytes())
    };

    Ok(obj)
}

impl Gsp {
    pub(crate) fn new(pdev: &pci::Device<device::Bound>) -> Result<impl PinInit<Self, Error>> {
        let dev = pdev.as_ref();
        let libos = CoherentAllocation::<LibosMemoryRegionInitArgument>::alloc_coherent(
            dev,
            GSP_PAGE_SIZE / size_of::<LibosMemoryRegionInitArgument>(),
            GFP_KERNEL | __GFP_ZERO,
        )?;
        let loginit = create_logbuffer_dma_object(dev)?;
        dma_write!(libos[0] = LibosMemoryRegionInitArgument::new("LOGINIT", &loginit))?;
        let logintr = create_logbuffer_dma_object(dev)?;
        dma_write!(libos[1] = LibosMemoryRegionInitArgument::new("LOGINTR", &logintr))?;
        let logrm = create_logbuffer_dma_object(dev)?;
        dma_write!(libos[2] = LibosMemoryRegionInitArgument::new("LOGRM", &logrm))?;

        // Creates its own PTE array
        let cmdq = GspCmdq::new(dev)?;
        let rmargs = CoherentAllocation::<GSP_ARGUMENTS_CACHED>::alloc_coherent(
            dev,
            1,
            GFP_KERNEL | __GFP_ZERO,
        )?;
        dma_write!(libos[3] = LibosMemoryRegionInitArgument::new("RMARGS", &rmargs))?;

        dma_write!(
            rmargs[0].messageQueueInitArguments = MESSAGE_QUEUE_INIT_ARGUMENTS {
                sharedMemPhysAddr: cmdq.dma_handle(),
                pageTableEntryCount: GspCmdq::NUM_PTES as u32,
                cmdQueueOffset: GspCmdq::CMDQ_OFFSET as u64,
                statQueueOffset: GspCmdq::STATQ_OFFSET as u64,
                ..Default::default()
            }
        )?;
        dma_write!(
            rmargs[0].srInitArguments = GSP_SR_INIT_ARGUMENTS {
                oldLevel: 0,
                flags: 0,
                bInPMTransition: 0,
                ..Default::default()
            }
        )?;
        dma_write!(rmargs[0].bDmemStack = 1)?;

        Ok(try_pin_init!(Self {
            libos,
            loginit,
            logintr,
            logrm,
            rmargs,
            cmdq,
        }))
    }
}
