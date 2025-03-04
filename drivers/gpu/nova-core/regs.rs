// SPDX-License-Identifier: GPL-2.0

use core::ops::Deref;
use kernel::io::Io;
use kernel::{register, register_rel};

use crate::falcon::{
    FalconCoreRev, FalconCoreRevSubversion, FalconModSelAlgo, FalconSecurityModel, RiscvCoreSelect,
};
use crate::gpu::Chipset;

register!(Boot0@0x00000000, "Basic revision information about the GPU";
    3:0     minor_rev => as u8, "minor revision of the chip";
    7:4     major_rev => as u8, "major revision of the chip";
    28:20   chipset => try_into Chipset, "chipset model"
);

/* PTIMER */

register!(PtimerTime0@0x00009400;
    31:0    lo => as u32, "low 32-bits of the timer"
);

register!(PtimerTime1@0x00009410;
    31:0    hi => as u32, "high 32 bits of the timer"
);

/* PFALCON */

register_rel!(FalconIrqsclr@0x00000004;
    4:4     halt => as_bit bool;
    6:6     swgen0 => as_bit bool;
);

register_rel!(FalconIrqstat@0x00000008;
    4:4     halt => as_bit bool;
    6:6     swgen0 => as_bit bool;
);

register_rel!(FalconIrqmclr@0x00000014;
    31:0    val => as u32
);

register_rel!(FalconIrqmask@0x00000018;
    31:0    val => as u32
);

register_rel!(FalconRm@0x00000084;
    31:0    val => as u32
);

register_rel!(FalconIrqdest@0x0000001c;
    31:0    val => as u32
);

register_rel!(FalconMailbox0@0x00000040;
    31:0    mailbox0 => as u32
);
register_rel!(FalconMailbox1@0x00000044;
    31:0    mailbox1 => as u32
);

register_rel!(FalconUnk0048@0x00000048;
    1:0     val0 => as u32
);

register_rel!(FalconHwcfg2@0x000000f4;
    10:10   riscv => as_bit bool;
    12:12   mem_scrubbing => as_bit bool;
    31:31   unk_31 => as_bit bool;
);

register_rel!(FalconCpuCtl@0x00000100;
    1:1     start_cpu => as_bit bool;
    4:4     halted => as_bit bool;
    6:6     alias_en => as_bit bool;
);
register_rel!(FalconBootVec@0x00000104;
    31:0    boot_vec => as u32
);

register_rel!(FalconHwCfg@0x00000108;
    8:0     imem_size => as u32;
    17:9    dmem_size => as u32;
);

register_rel!(FalconDmaCtl@0x0000010c;
    0:0     require_ctx => as_bit bool;
    1:1     dmem_scrubbing  => as_bit bool;
    2:2     imem_scrubbing => as_bit bool;
    6:3     dmaq_num => as_bit u8;
    7:7     secure_stat => as_bit bool;
);

register_rel!(FalconDmaTrfBase@0x00000110;
    31:0    base => as u32;
);

register_rel!(FalconDmaTrfMOffs@0x00000114;
    23:0    offs => as u32;
);

register_rel!(FalconDmaTrfCmd@0x00000118;
    0:0     full => as_bit bool;
    1:1     idle => as_bit bool;
    3:2     sec => as_bit u8;
    4:4     imem => as_bit bool;
    5:5     is_write => as_bit bool;
    10:8    size => as u8;
    14:12   ctxdma => as u8;
    16:16   set_dmtag => as u8;
);

register_rel!(FalconDmaTrfBOffs@0x0000011c;
    31:0    offs => as u32;
);

register_rel!(FalconDmaTrfBase1@0x00000128;
    8:0     base => as u16;
);

register_rel!(FalconHwcfg1@0x0000012c;
    3:0     core_rev => try_into FalconCoreRev, "core revision of the falcon";
    5:4     security_model => try_into FalconSecurityModel, "security model of the falcon";
    7:6     core_rev_subversion => into FalconCoreRevSubversion;
    11:8    imem_ports => as u8;
    15:12   dmem_ports => as u8;
);

// TODO: This should be able to take an index, like +0x180[16; 8]? Then the constructor or read
// method take the port we want to address as argument.
register_rel!(FalconImemC@0x00000180;
    7:2     offs => as u8;
    23:8    blk => as u8;
    24:24   aincw => as_bit bool;
    25:25   aincr => as_bit bool;
    28:28   secure => as_bit bool;
    29:29   sec_atomic => as_bit bool;
);

register_rel!(FalconImemD@0x00000184;
    31:0    data => as u32;
);

register_rel!(FalconImemT@0x00000188;
    15:0    data => as u16;
);

register_rel!(FalconDmemC@0x000001c0;
    23:0    addr => as u32;
    7:2     offs => as u8;
    23:8    blk => as u8;
    24:24   aincw => as_bit bool;
    25:25   aincr => as_bit bool;
    26:26   settag => as_bit bool;
    27:27   setlvl => as_bit bool;
    28:28   va => as_bit bool;
    29:29   miss => as_bit bool;
);

register_rel!(FalconDmemD@0x000001c4;
    31:0    data => as u32;
);

register_rel!(FalconModSel@0x00001180;
    7:0     algo => try_into FalconModSelAlgo;
);
register_rel!(FalconBromCurrUcodeId@0x00001198;
    31:0    ucode_id => as u32;
);
register_rel!(FalconBromEngidmask@0x0000119c;
    31:0    mask => as u32;
);
register_rel!(FalconBromParaaddr0@0x00001210;
    31:0    addr => as u32;
);

register_rel!(RiscvCpuCtl@0x00000388;
    0:0     startcpu => as_bit bool;
    4:4     halted => as_bit bool;
    5:5     stopped => as_bit bool;
    7:7     active_stat => as_bit bool;
);

register_rel!(RiscvUnk3c0@0x000003c0;
    0:0     unk0 => as_bit bool;
);

register_rel!(RiscvIrqmask@0x00000528;
    31:0    mask => as u32;
);

register_rel!(RiscvIrqdest@0x0000052c;
    31:0    dest => as u32;
);

register_rel!(FalconUnk600@0x00000600;
    16:16   unk16 => as_bit bool;
    2:0     unk2 => as u8;
);

register_rel!(FalconUnk624@0x00000624;
    7:7     unk7 => as_bit bool;
);

register_rel!(RiscvBcrCtrl@0x00001668;
    0:0     valid => as_bit bool;
    4:4     core_select => as_bit RiscvCoreSelect;
    8:8     br_fetch => as_bit bool;
);
