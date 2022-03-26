use crate::arch::x86::{opcodes, Cpu, GeneralByteReg, GeneralWordReg, RegMem, SegmentReg};
use std::fmt::{Debug, Formatter};

pub mod arith;
pub mod control;
pub mod flags;
pub mod ports;
pub mod transfer;

pub struct InstrFunc<F>(pub F);

impl<F> Debug for InstrFunc<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

#[derive(Debug)]
pub enum Instr {
    Basic(InstrFunc<fn(cpu: &mut Cpu)>),
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
    Rm16R16 {
        func: InstrFunc<fn(cpu: &mut Cpu, rm: RegMem, reg: GeneralWordReg)>,
        rm: RegMem,
        reg: GeneralWordReg,
    },
    Imm8 {
        func: InstrFunc<fn(cpu: &mut Cpu, imm: u8)>,
        imm: u8,
    },
    Rm8R8 {
        func: InstrFunc<fn(cpu: &mut Cpu, rm: RegMem, reg: GeneralByteReg)>,
        rm: RegMem,
        reg: GeneralByteReg,
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
}

impl Instr {
    pub fn decode(cpu: &mut Cpu) -> Self {
        opcodes::decode(cpu)
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            Instr::Basic(func) => func.0(cpu),
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
            Instr::Rm16R16 { func, rm, reg } => func.0(cpu, rm, reg),
            Instr::Imm8 { func, imm } => func.0(cpu, imm),
            Instr::Rm8R8 { func, rm, reg } => func.0(cpu, rm, reg),
            Instr::R16Imm16 { func, reg, imm } => func.0(cpu, reg, imm),
            Instr::SregRm16 { func, reg, rm } => func.0(cpu, reg, rm),
            Instr::Rm8Imm8 { func, rm, imm } => func.0(cpu, rm, imm),
            Instr::R16Rm16 { func, reg, rm } => func.0(cpu, reg, rm),
        }
    }
}
