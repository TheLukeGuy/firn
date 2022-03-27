pub mod cpu;
pub mod device;
pub mod flags;
pub mod instr;
pub mod modrm;
pub mod opcodes;
pub mod regs;

pub use cpu::Cpu;
pub use device::{Device, IoInstr, PortMatchResult};
pub use flags::Flags;
pub use instr::{Instr, InstrFunc};
pub use modrm::{Displacement, Modrm, ModrmRegType, RegMem, RmPtr};
pub use regs::{GeneralByteReg, GeneralReg, GeneralWordReg, Reg, SegmentReg, WordReg};

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Byte,
    Word,
}
