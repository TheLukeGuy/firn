use crate::arch::x86::{Cpu, GeneralByteReg};

pub fn mov_r8_imm8(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8) {
    cpu.set_reg_8(reg, imm);
}
