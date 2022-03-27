use crate::arch::x86::{
    opcodes, Cpu, GeneralByteReg, GeneralWordReg, Modrm, ModrmRegType, RegMem, SegmentReg, Size,
};
use std::fmt::{Debug, Formatter};
use std::ops::Range;

pub mod arith;
pub mod control;
pub mod flags;
pub mod ports;
pub mod strings;
pub mod transfer;

pub fn rep_16(rep: bool, times: impl Fn() -> u16) -> Range<u16> {
    if rep {
        0..times()
    } else {
        0..1
    }
}

pub struct InstrFunc<F>(pub F);

impl<F> Debug for InstrFunc<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

#[derive(Debug)]
pub enum Instr {
    Basic(InstrFunc<fn(cpu: &mut Cpu)>),
    BasicRep {
        func: InstrFunc<fn(cpu: &mut Cpu, rep: bool)>,
        rep: bool,
    },
    Ptr16_16 {
        func: InstrFunc<fn(cpu: &mut Cpu, offset: u16, segment: u16)>,
        offset: u16,
        segment: u16,
    },
    R8Imm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8)>,
        reg: GeneralByteReg,
        imm: u8,
    },
    Moffs8 {
        func: InstrFunc<fn(cpu: &mut Cpu, segment: SegmentReg, offset: u16)>,
        segment: SegmentReg,
        offset: u16,
    },
    Imm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, imm: u8)>,
        imm: u8,
    },
    R8Rm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, reg: GeneralByteReg, rm: RegMem)>,
        reg: GeneralByteReg,
        rm: RegMem,
    },
    R16Imm16 {
        func: InstrFunc<fn(cpu: &mut Cpu, reg: GeneralWordReg, imm: u16)>,
        reg: GeneralWordReg,
        imm: u16,
    },
    SregRm16 {
        func: InstrFunc<fn(cpu: &mut Cpu, reg: SegmentReg, rm: RegMem)>,
        reg: SegmentReg,
        rm: RegMem,
    },
    Rm8Imm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, rm: RegMem, imm: u8)>,
        rm: RegMem,
        imm: u8,
    },
    R16Rm16 {
        func: InstrFunc<fn(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem)>,
        reg: GeneralWordReg,
        rm: RegMem,
    },
    Imm16Imm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, first: u16, second: u8)>,
        first: u16,
        second: u8,
    },
}

impl Instr {
    pub fn decode(cpu: &mut Cpu) -> Self {
        opcodes::decode(cpu)
    }

    pub fn new_basic(func: fn(cpu: &mut Cpu)) -> Self {
        Instr::Basic(InstrFunc(func))
    }

    pub fn new_basic_rep(func: fn(cpu: &mut Cpu, rep: bool), rep: bool) -> Self {
        Instr::BasicRep {
            func: InstrFunc(func),
            rep,
        }
    }

    pub fn new_ptr16_16(func: fn(cpu: &mut Cpu, offset: u16, segment: u16), cpu: &mut Cpu) -> Self {
        Instr::Ptr16_16 {
            func: InstrFunc(func),
            offset: cpu.read_mem_16(),
            segment: cpu.read_mem_16(),
        }
    }

    pub fn new_r8_imm8(
        func: fn(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8),
        cpu: &mut Cpu,
        reg: GeneralByteReg,
    ) -> Self {
        Instr::R8Imm8 {
            func: InstrFunc(func),
            reg,
            imm: cpu.read_mem_8(),
        }
    }

    pub fn new_moffs8(
        func: fn(cpu: &mut Cpu, segment: SegmentReg, offset: u16),
        cpu: &mut Cpu,
        segment: SegmentReg,
    ) -> Self {
        Instr::Moffs8 {
            func: InstrFunc(func),
            segment,
            offset: cpu.read_mem_16(),
        }
    }

    pub fn new_imm8(func: fn(cpu: &mut Cpu, imm: u8), cpu: &mut Cpu) -> Self {
        Instr::Imm8 {
            func: InstrFunc(func),
            imm: cpu.read_mem_8(),
        }
    }

    pub fn new_r8_rm8(
        func: fn(cpu: &mut Cpu, reg: GeneralByteReg, rm: RegMem),
        cpu: &mut Cpu,
    ) -> Self {
        let modrm = Self::modrm_all_8(cpu);
        Instr::R8Rm8 {
            func: InstrFunc(func),
            reg: modrm.byte_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_r16_imm16(
        func: fn(cpu: &mut Cpu, reg: GeneralWordReg, imm: u16),
        cpu: &mut Cpu,
        reg: GeneralWordReg,
    ) -> Self {
        Instr::R16Imm16 {
            func: InstrFunc(func),
            reg,
            imm: cpu.read_mem_16(),
        }
    }

    pub fn new_sreg_rm16(
        func: fn(cpu: &mut Cpu, reg: SegmentReg, rm: RegMem),
        cpu: &mut Cpu,
    ) -> Self {
        let modrm = Self::modrm_segment_16(cpu);
        Instr::SregRm16 {
            func: InstrFunc(func),
            reg: modrm.segment_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_rm8_imm8(func: fn(cpu: &mut Cpu, rm: RegMem, imm: u8), cpu: &mut Cpu) -> Self {
        let modrm = Self::modrm_all_8(cpu);
        Instr::Rm8Imm8 {
            func: InstrFunc(func),
            rm: modrm.reg_mem,
            imm: cpu.read_mem_8(),
        }
    }

    pub fn new_r16_rm16(
        func: fn(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem),
        cpu: &mut Cpu,
    ) -> Self {
        let modrm = Self::modrm_all_16(cpu);
        Instr::R16Rm16 {
            func: InstrFunc(func),
            reg: modrm.word_reg(),
            rm: modrm.reg_mem,
        }
    }

    pub fn new_imm16_imm8(func: fn(cpu: &mut Cpu, first: u16, second: u8), cpu: &mut Cpu) -> Self {
        Instr::Imm16Imm8 {
            func: InstrFunc(func),
            first: cpu.read_mem_16(),
            second: cpu.read_mem_8(),
        }
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            Instr::Basic(func) => func.0(cpu),
            Instr::BasicRep { func, rep } => func.0(cpu, rep),
            Instr::Ptr16_16 {
                func,
                offset,
                segment,
            } => func.0(cpu, offset, segment),
            Instr::R8Imm8 { func, reg, imm } => func.0(cpu, reg, imm),
            Instr::Moffs8 {
                func,
                segment,
                offset,
            } => func.0(cpu, segment, offset),
            Instr::Imm8 { func, imm } => func.0(cpu, imm),
            Instr::R8Rm8 { func, reg, rm } => func.0(cpu, reg, rm),
            Instr::R16Imm16 { func, reg, imm } => func.0(cpu, reg, imm),
            Instr::SregRm16 { func, reg, rm } => func.0(cpu, reg, rm),
            Instr::Rm8Imm8 { func, rm, imm } => func.0(cpu, rm, imm),
            Instr::R16Rm16 { func, reg, rm } => func.0(cpu, reg, rm),
            Instr::Imm16Imm8 {
                func,
                first,
                second,
            } => func.0(cpu, first, second),
        }
    }

    fn modrm_all_8(cpu: &mut Cpu) -> Modrm {
        let modrm = cpu.read_mem_8();
        Modrm::decode(cpu, modrm, Some(ModrmRegType::ByteSized), Size::Byte)
    }

    fn modrm_all_16(cpu: &mut Cpu) -> Modrm {
        let modrm = cpu.read_mem_8();
        Modrm::decode(cpu, modrm, Some(ModrmRegType::WordSized), Size::Word)
    }

    fn modrm_segment_16(cpu: &mut Cpu) -> Modrm {
        let modrm = cpu.read_mem_8();
        Modrm::decode(cpu, modrm, Some(ModrmRegType::Segment), Size::Word)
    }
}
