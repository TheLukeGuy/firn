use crate::SegmentReg::Ds;
use crate::{opcodes, SegmentReg, System};
use std::fmt::{Debug, Formatter};

pub mod arith;
pub mod conditionals;
pub mod control;
pub mod flags;
pub mod ports;
pub mod stack;
pub mod strings;
pub mod transfer;

#[derive(Debug)]
pub struct Prefixes {
    pub lock: bool,

    pub rep_or_rep_e: bool,
    pub repne: bool,

    pub segment: SegmentReg,
}

impl Prefixes {
    pub fn new() -> Self {
        Self {
            lock: false,

            rep_or_rep_e: false,
            repne: false,

            segment: Ds,
        }
    }
}

impl Default for Prefixes {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct InstrMeta {
    pub mnemonic: String,
}

pub struct InstrFunc(fn(sys: &mut System, opcode: u8, prefixes: &Prefixes));

impl Debug for InstrFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("InstrFunc")
    }
}

#[derive(Debug)]
pub struct Instr {
    pub opcode: u8,
    pub prefixes: Prefixes,

    pub meta: InstrMeta,
    func: InstrFunc,
}

impl Instr {
    pub fn decode(sys: &mut System) -> Instr {
        opcodes::decode(sys)
    }

    pub fn new(
        opcode: u8,
        prefixes: Prefixes,
        func: fn(sys: &mut System, opcode: u8, prefixes: &Prefixes),
        meta_func: fn() -> InstrMeta,
    ) -> Self {
        Self {
            opcode,
            prefixes,

            meta: meta_func(),
            func: InstrFunc(func),
        }
    }

    pub fn execute(&self, sys: &mut System) {
        self.func.0(sys, self.opcode, &self.prefixes);
    }
}
