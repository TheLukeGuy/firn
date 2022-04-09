use crate::{opcodes, System};
use std::fmt::{Debug, Formatter};

pub mod arith;
pub mod control;
pub mod flags;
pub mod ports;
pub mod stack;
pub mod strings;
pub mod transfer;

#[derive(Debug)]
pub struct InstrMeta {
    pub mnemonic: String,
}

pub struct InstrFunc(fn(sys: &mut System, opcode: u8));

impl Debug for InstrFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

#[derive(Debug)]
pub struct Instr {
    pub opcode: u8,
    pub meta: InstrMeta,
    func: InstrFunc,
}

impl Instr {
    pub fn decode(sys: &mut System) -> Instr {
        opcodes::decode(sys)
    }

    pub fn new(
        opcode: u8,
        func: fn(sys: &mut System, opcode: u8),
        meta_func: fn() -> InstrMeta,
    ) -> Self {
        Self {
            opcode,
            meta: meta_func(),
            func: InstrFunc(func),
        }
    }

    pub fn execute(&self, sys: &mut System) {
        self.func.0(sys, self.opcode);
    }
}
