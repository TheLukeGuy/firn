use crate::GeneralWordReg::Cx;
use crate::SegmentReg::Ds;
use crate::{opcodes, SegmentReg, System};
use std::fmt::{Debug, Formatter};
use std::ops::Range;

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

    pub rep: bool,
    pub rep_ne: bool,

    pub segment: SegmentReg,
}

impl Prefixes {
    pub fn new() -> Self {
        Self {
            lock: false,

            rep: false,
            rep_ne: false,

            segment: Ds,
        }
    }

    pub fn rep_range(&self, sys: &System) -> Range<u16> {
        if self.rep {
            0..sys.cpu.reg_16(Cx.into())
        } else {
            0..1
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
