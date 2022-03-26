use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::{Cpu, GeneralByteReg, GeneralWordReg, RegMem, SegmentReg};

pub fn mov_r8_imm8(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8) {
    cpu.set_reg_8(reg, imm);
}

pub fn mov_al_moffs8(cpu: &mut Cpu, segment: SegmentReg, offset: u16) {
    let value = cpu.get_mem_8(segment, offset);
    cpu.set_reg_8(Al, value);
}

pub fn mov_rm8_r8(cpu: &mut Cpu, rm: RegMem, reg: GeneralByteReg) {
    let value = cpu.get_reg_8(reg);
    rm.set_8(cpu, value);
}

pub fn mov_r16_imm16(cpu: &mut Cpu, reg: GeneralWordReg, imm: u16) {
    cpu.set_reg_16(reg.into(), imm);
}

pub fn mov_rm16_r16(cpu: &mut Cpu, rm: RegMem, reg: GeneralWordReg) {
    let value = cpu.get_reg_16(reg.into());
    rm.set_16(cpu, value);
}

pub fn mov_sreg_rm16(cpu: &mut Cpu, reg: SegmentReg, rm: RegMem) {
    let value = rm.get_16(cpu);
    cpu.set_reg_16(reg.into(), value);
}
