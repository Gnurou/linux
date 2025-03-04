// SPDX-License-Identifier: GPL-2.0

//! Falcon microprocessor base support

// TODO: remove this once this module is actively used.
#![allow(dead_code)]

use core::hint::unreachable_unchecked;
use core::marker::PhantomData;
use core::time::Duration;
use kernel::bindings;
use kernel::devres::Devres;
use kernel::{pci, prelude::*};

use crate::driver::Bar0;
use crate::gpu::Chipset;
use crate::regs;
use crate::timer::Timer;

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FalconCoreRev {
    #[default]
    Rev1 = 1,
    Rev2 = 2,
    Rev3 = 3,
    Rev4 = 4,
    Rev5 = 5,
    Rev6 = 6,
    Rev7 = 7,
}

impl TryFrom<u32> for FalconCoreRev {
    type Error = Error;

    fn try_from(value: u32) -> core::result::Result<Self, Self::Error> {
        use FalconCoreRev::*;

        let rev = match value {
            1 => Rev1,
            2 => Rev2,
            3 => Rev3,
            4 => Rev4,
            5 => Rev5,
            6 => Rev6,
            7 => Rev7,
            _ => return Err(EINVAL),
        };

        Ok(rev)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub(crate) enum FalconSecurityModel {
    #[default]
    None = 0,
    Light = 2,
    Heavy = 3,
}

impl TryFrom<u32> for FalconSecurityModel {
    type Error = Error;

    fn try_from(value: u32) -> core::result::Result<Self, Self::Error> {
        use FalconSecurityModel::*;

        let sec_model = match value {
            0 => None,
            2 => Light,
            3 => Heavy,
            _ => return Err(EINVAL),
        };

        Ok(sec_model)
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FalconCoreRevSubversion {
    #[default]
    Subversion0 = 0,
    Subversion1 = 1,
    Subversion2 = 2,
    Subversion3 = 3,
}

impl From<u32> for FalconCoreRevSubversion {
    fn from(value: u32) -> Self {
        use FalconCoreRevSubversion::*;

        match value & 0b11 {
            0 => Subversion0,
            1 => Subversion1,
            2 => Subversion2,
            3 => Subversion3,
            // SAFETY: the `0b11` mask limits the possible values to `0..=3`.
            4..=u32::MAX => unsafe { unreachable_unchecked() },
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub(crate) enum FalconModSelAlgo {
    #[default]
    Rsa3k = 1,
}

impl TryFrom<u32> for FalconModSelAlgo {
    type Error = Error;

    fn try_from(value: u32) -> core::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(FalconModSelAlgo::Rsa3k),
            _ => Err(EINVAL),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RiscvCoreSelect {
    Falcon = 0,
    Riscv = 1,
}

impl From<bool> for RiscvCoreSelect {
    fn from(value: bool) -> Self {
        match value {
            false => RiscvCoreSelect::Falcon,
            true => RiscvCoreSelect::Riscv,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FalconMem {
    Imem,
    Dmem,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct FalconUCodeDescV3 {
    pub(crate) hdr: u32,
    pub(crate) stored_size: u32,
    pub(crate) pkc_data_offset: u32,
    pub(crate) interface_offset: u32,
    pub(crate) imem_phys_base: u32,
    pub(crate) imem_load_size: u32,
    pub(crate) imem_virt_base: u32,
    pub(crate) dmem_phys_base: u32,
    pub(crate) dmem_load_size: u32,
    pub(crate) engine_id_mask: u16,
    pub(crate) ucode_id: u8,
    pub(crate) signature_count: u8,
    pub(crate) signature_versions: u16,
    _reserved: u16,
}

impl FalconUCodeDescV3 {
    pub(crate) fn ver(&self) -> u8 {
        ((self.hdr & 0xff00) >> 8) as u8
    }

    pub(crate) fn size(&self) -> usize {
        ((self.hdr & 0xffff0000) >> 16) as usize
    }
}

/// Trait defining the parameters of a given Falcon instance.
pub(crate) trait FalconInstance {
    /// Base I/O address for the falcon, relative from which its registers are accessed.
    const BASE: usize;
}

pub(crate) struct Gsp;
impl FalconInstance for Gsp {
    const BASE: usize = 0x00110000;
}
pub(crate) type GspFalcon = Falcon<Gsp>;

pub(crate) struct Sec2;
impl FalconInstance for Sec2 {
    const BASE: usize = 0x00840000;
}
pub(crate) type Sec2Falcon = Falcon<Sec2>;

/// Contains the base parameters common to all Falcon instances.
#[derive(Debug)]
pub(crate) struct Falcon<I: FalconInstance> {
    /// Chipset this falcon belongs to.
    chipset: Chipset,
    /// Whether this falcon is part of a dual falcon/riscv engine.
    has_riscv: bool,
    _instance: PhantomData<I>,
}

impl<I: FalconInstance> Falcon<I> {
    pub(crate) fn new(
        pdev: &pci::Device,
        chipset: Chipset,
        bar: &Devres<Bar0>,
        need_riscv: bool,
    ) -> Result<Self> {
        let hwcfg1 = with_bar!(bar, |b| regs::FalconHwcfg1::read(b, I::BASE))?;
        let rev = hwcfg1.core_rev()?;
        let subver = hwcfg1.core_rev_subversion();
        let sec_model = hwcfg1.security_model()?;

        if need_riscv {
            let hwcfg2 = with_bar!(bar, |b| regs::FalconHwcfg2::read(b, I::BASE))?;
            if !hwcfg2.riscv() {
                dev_err!(
                    pdev.as_ref(),
                    "riscv support requested on falcon that does not support it\n"
                );
                return Err(EINVAL);
            }
        }

        dev_info!(
            pdev.as_ref(),
            "new falcon: {:?} {:?} {:?}",
            rev,
            subver,
            sec_model
        );

        Ok(Self {
            chipset,
            has_riscv: need_riscv,
            _instance: PhantomData,
        })
    }

    fn select_falcon_core(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        if self.has_riscv {
            let bcr_ctrl = with_bar!(bar, |b| regs::RiscvBcrCtrl::read(b, I::BASE))?;
            if bcr_ctrl.core_select() != RiscvCoreSelect::Falcon {
                with_bar!(bar, |b| regs::RiscvBcrCtrl::default()
                    .set_core_select(RiscvCoreSelect::Falcon)
                    .write(b, I::BASE))?;

                timer.wait_on(bar, Duration::from_millis(10), || {
                    bar.try_access_with(|b| regs::RiscvBcrCtrl::read(b, I::BASE))
                        .and_then(|v| if v.valid() { Some(()) } else { None })
                })?;
            }
        }

        Ok(())
    }

    fn reset_wait_mem_scrubbing(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        /* TODO: is this needed? */
        with_bar!(bar, |b| regs::FalconMailbox0::alter(b, I::BASE, |v| v))?;

        timer.wait_on(bar, Duration::from_millis(20), || {
            bar.try_access_with(|b| regs::FalconHwcfg2::read(b, I::BASE))
                .and_then(|r| if r.mem_scrubbing() { Some(()) } else { None })
        })
    }

    fn reset_prep(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        let _ = with_bar!(bar, |b| regs::FalconHwcfg2::read(b, I::BASE))?;

        // Expected to timeout apparently?
        // TODO: check why with OpenRM.
        let _ = timer.wait_on(bar, Duration::from_micros(150), || {
            bar.try_access_with(|b| regs::FalconHwcfg2::read(b, I::BASE))
                .and_then(|r| if r.unk_31() { Some(()) } else { None })
        });

        Ok(())
    }

    fn reset_eng(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        self.reset_prep(bar, timer)?;

        with_bar!(bar, |b| regs::RiscvUnk3c0::alter(b, I::BASE, |v| v
            .set_unk0(true)))?;

        let _: Result<()> = timer.wait_on(bar, Duration::from_micros(10), || None);

        with_bar!(bar, |b| regs::RiscvUnk3c0::alter(b, I::BASE, |v| v
            .set_unk0(false)))?;

        self.reset_wait_mem_scrubbing(bar, timer)?;

        Ok(())
    }

    fn disable(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        self.select_falcon_core(bar, timer)?;

        with_bar!(bar, |b| {
            regs::FalconUnk0048::alter(b, I::BASE, |r| r.set_val0(0));

            regs::FalconIrqmclr::default()
                .set_val(u32::MAX)
                .write(b, I::BASE)
        })?;

        self.reset_eng(bar, timer)
    }

    fn enable(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        self.reset_eng(bar, timer)?;
        self.select_falcon_core(bar, timer)?;
        self.reset_wait_mem_scrubbing(bar, timer)?;

        with_bar!(bar, |b| {
            // We write Boot0 into FalconRm, for some reason...
            regs::FalconRm::default()
                .set_val(regs::Boot0::read(b).into())
                .write(b, I::BASE)
        })
    }

    pub(crate) fn reset(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        self.disable(bar, timer)?;
        self.enable(bar, timer)
    }

    fn dma_init(
        &self,
        bar: &Devres<Bar0>,
        dma_handle: bindings::dma_addr_t,
        mem: FalconMem,
        xfer_len: u32,
        sec: bool,
    ) -> Result<regs::FalconDmaTrfCmd> {
        with_bar!(bar, |b| {
            regs::FalconDmaTrfBase::default()
                .set_base((dma_handle >> 8) as u32)
                .write(b, I::BASE);
            regs::FalconDmaTrfBase1::default()
                .set_base((dma_handle >> 40) as u16)
                .write(b, I::BASE)
        })?;

        Ok(regs::FalconDmaTrfCmd::default()
            .set_size((xfer_len.ilog2() - 2) as u8)
            .set_imem(mem == FalconMem::Imem)
            .set_sec(if sec { 1 } else { 0 }))
    }

    fn dma_xfer(
        &self,
        bar: &Devres<Bar0>,
        mem_base: u32,
        dma_base: u32,
        cmd: regs::FalconDmaTrfCmd,
    ) -> Result<()> {
        with_bar!(bar, |b| {
            regs::FalconDmaTrfMOffs::default()
                .set_offs(mem_base)
                .write(b, I::BASE);
            regs::FalconDmaTrfBOffs::default()
                .set_offs(dma_base)
                .write(b, I::BASE);

            cmd.write(b, I::BASE)
        })
    }

    fn dma_done(&self, bar: &Devres<Bar0>, timer: &Timer) -> Result<()> {
        timer.wait_on(bar, Duration::from_millis(2000), || {
            bar.try_access_with(|b| regs::FalconDmaTrfCmd::read(b, I::BASE))
                .and_then(|v| if v.idle() { Some(()) } else { None })
        })
    }

    fn dma_wr(
        &self,
        bar: &Devres<Bar0>,
        timer: &Timer,
        dma_handle: bindings::dma_addr_t,
        dma_base: u32,
        mem: FalconMem,
        mem_base: u32,
        len: u32,
        sec: bool,
    ) -> Result<()> {
        const DMA_LEN: u32 = 256;

        let (dma_start, dma_addr) = match mem {
            FalconMem::Imem => (0, dma_handle),
            FalconMem::Dmem => (dma_base, dma_handle + dma_base as bindings::dma_addr_t),
        };

        pr_info!(
            "dma write {:?}: dma_handle {:x} dma_start {:x} dma_addr {:x} len {:x}\n",
            mem,
            dma_handle,
            dma_start,
            dma_addr,
            len
        );

        let cmd = self.dma_init(bar, dma_addr, mem, DMA_LEN, sec)?;

        let mut dst = mem_base;
        let mut src = dma_base;
        let mut remain = len;

        while remain >= DMA_LEN {
            self.dma_xfer(bar, dst, src - dma_start, cmd)?;
            self.dma_done(bar, timer)?;

            src += DMA_LEN;
            dst += DMA_LEN;
            remain -= DMA_LEN;
        }

        pr_info!("dma write remain: {} bytes\n", remain);

        Ok(())
    }

    pub(crate) fn dma_load(
        &self,
        bar: &Devres<Bar0>,
        timer: &Timer,
        dma_handle: bindings::dma_addr_t,
        imem_params: (u32, u32, u32),
        dmem_params: (u32, u32, u32),
    ) -> Result<()> {
        pr_info!("dma_load: {:?} {:?}\n", imem_params, dmem_params);

        with_bar!(bar, |b| {
            regs::FalconUnk624::alter(b, I::BASE, |v| v.set_unk7(true));
            regs::FalconDmaCtl::default().write(b, I::BASE);
            regs::FalconUnk600::alter(b, I::BASE, |v| v.set_unk16(false).set_unk2((1 << 2) | 1));
        })?;

        self.dma_wr(
            bar,
            timer,
            dma_handle,
            imem_params.0,
            FalconMem::Imem,
            imem_params.1,
            imem_params.2,
            true,
        )?;
        self.dma_wr(
            bar,
            timer,
            dma_handle,
            dmem_params.0,
            FalconMem::Dmem,
            dmem_params.1,
            dmem_params.2,
            true,
        )?;

        Ok(())
    }

    pub(crate) fn boot(
        &self,
        bar: &Devres<Bar0>,
        timer: &Timer,
        v3_desc: &FalconUCodeDescV3,
        mbox0: Option<u32>,
        mbox1: Option<u32>,
    ) -> Result<(u32, u32)> {
        pr_info!("boot 0\n");

        // Program BROM registers for PKC signature validation.
        if self.chipset >= Chipset::GA102 {
            let pkc_data_offset = v3_desc.pkc_data_offset;
            let engine_id_mask = v3_desc.engine_id_mask;
            let ucode_id = v3_desc.ucode_id;

            pr_info!(
                "dmem_sign: {:#x}, engine_id: {:#x}, ucode_id: {:#x}",
                pkc_data_offset,
                engine_id_mask,
                ucode_id
            );

            with_bar!(bar, |b| {
                regs::FalconBromParaaddr0::default()
                    .set_addr(pkc_data_offset)
                    .write(b, I::BASE);
                regs::FalconBromEngidmask::default()
                    .set_mask(engine_id_mask as u32)
                    .write(b, I::BASE);
                regs::FalconBromCurrUcodeId::default()
                    .set_ucode_id(ucode_id as u32)
                    .write(b, I::BASE);
                regs::FalconModSel::default()
                    .set_algo(FalconModSelAlgo::Rsa3k)
                    .write(b, I::BASE);
            })?;
        }

        pr_info!("boot 1\n");

        with_bar!(bar, |b| {
            if let Some(mbox0) = mbox0 {
                regs::FalconMailbox0::default()
                    .set_mailbox0(mbox0)
                    .write(b, I::BASE);
            }

            if let Some(mbox1) = mbox1 {
                regs::FalconMailbox1::default()
                    .set_mailbox1(mbox1)
                    .write(b, I::BASE);
            }

            // Set `BootVec` to start of non-secure code.
            // TODO: use boot vector variable - apparently this is 0 on v3 hdr?
            regs::FalconBootVec::default()
                .set_boot_vec(0)
                .write(b, I::BASE);

            regs::FalconCpuCtl::default()
                .set_start_cpu(true)
                .write(b, I::BASE);
        })?;

        pr_info!("booted!\n");
        timer.wait_on(bar, Duration::from_secs(2), || {
            bar.try_access()
                .map(|b| regs::FalconCpuCtl::read(&*b, I::BASE))
                .and_then(|v| if v.halted() { Some(()) } else { None })
        })?;

        let (mbox0, mbox1) = with_bar!(bar, |b| {
            let mbox0 = regs::FalconMailbox0::read(b, I::BASE).mailbox0();
            let mbox1 = regs::FalconMailbox1::read(b, I::BASE).mailbox1();

            (mbox0, mbox1)
        })?;

        pr_info!("successfully returned {} {}\n", mbox0, mbox1);

        Ok((mbox0, mbox1))
    }
}

#[repr(C)]
#[derive(Debug)]
struct FalconAppifHdrV1 {
    ver: u8,
    hdr: u8,
    len: u8,
    cnt: u8,
}

#[repr(C)]
#[derive(Debug)]
struct FalconAppifV1 {
    id: u32,
    dmem_base: u32,
}

const NVFW_FALCON_APPIF_ID_DMEMMAPPER: u32 = 0x4;

#[repr(C)]
#[derive(Debug)]
struct FalconAppifDmemmapperV3 {
    signature: u32,
    version: u16,
    size: u16,
    cmd_in_buffer_offset: u32,
    cmd_in_buffer_size: u32,
    cmd_out_buffer_offset: u32,
    cmd_out_buffer_size: u32,
    nvf_img_data_buffer_offset: u32,
    nvf_img_data_buffer_size: u32,
    printf_buffer_hdr: u32,
    ucode_build_time_stamp: u32,
    ucode_signature: u32,
    init_cmd: u32,
    ucode_feature: u32,
    ucode_cmd_mask0: u32,
    ucode_cmd_mask1: u32,
    multi_tgt_tbl: u32,
}

pub(crate) const NVFW_FALCON_APPIF_DMEMMAPPER_CMD_FRTS: u32 = 0x15;

#[derive(Debug)]
#[repr(C)]
struct ReadVbios {
    ver: u32,
    hdr: u32,
    addr: u64,
    size: u32,
    flags: u32,
}

#[derive(Debug)]
#[repr(C)]
struct FrtsRegion {
    ver: u32,
    hdr: u32,
    addr: u32,
    size: u32,
    ftype: u32,
}

const NVFW_FRTS_CMD_REGION_TYPE_FB: u32 = 2;

#[derive(Debug)]
#[repr(C)]
struct FrtsCmd {
    read_vbios: ReadVbios,
    frts_region: FrtsRegion,
}
