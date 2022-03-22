use crate::arch::x86::{opcodes, Cpu, GeneralByteReg};
use std::fmt::{Debug, Formatter};

pub mod control;
pub mod flags;
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
        }
    }
}
