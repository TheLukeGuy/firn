pub mod cpu;
pub mod instr;
pub mod modrm;
pub mod opcodes;
pub mod regs;

pub use cpu::Cpu;
pub use instr::{Instr, InstrFunc};
pub use modrm::{Displacement, Modrm, ModrmRegType, RegMem, RmPtr};
pub use regs::{GeneralByteReg, GeneralReg, GeneralWordReg, Reg, SegmentReg, WordReg};

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Byte,
    Word,
}
