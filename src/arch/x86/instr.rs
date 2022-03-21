use crate::arch::x86::{opcodes, Cpu};
use std::fmt::{Debug, Formatter};

pub mod arith;
pub mod control;

pub struct InstrFunc<F>(pub F);

impl<F> Debug for InstrFunc<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

#[derive(Debug)]
pub enum Instr {
    Ptr16_16 {
        func: InstrFunc<fn(cpu: &mut Cpu, offset: u16, segment: u16)>,
        offset: u16,
        segment: u16,
    },
}

impl Instr {
    pub fn decode(cpu: &mut Cpu) -> Self {
        let opcode = cpu.read_mem_8();

        opcodes::decode(cpu, opcode)
    }

    pub fn execute(self, cpu: &mut Cpu) {
        match self {
            Instr::Ptr16_16 {
                func,
                offset,
                segment,
            } => func.0(cpu, offset, segment),
        }
    }
}
