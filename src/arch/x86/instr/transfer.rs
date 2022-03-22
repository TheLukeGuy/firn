use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::{Cpu, GeneralByteReg, SegmentReg};

pub fn mov_r8_imm8(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8) {
    cpu.set_reg_8(reg, imm);
}

pub fn mov_al_moffs8(cpu: &mut Cpu, segment: SegmentReg, offset: u16) {
    let value = cpu.get_mem_8(segment, offset);
    cpu.set_reg_8(Al, value);
}
