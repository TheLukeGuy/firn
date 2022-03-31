use crate::GeneralByteReg::Al;
use crate::SegmentReg::Es;
use crate::{Cpu, GeneralByteReg, GeneralWordReg, RegMem, RmPtr, SegmentReg};

pub fn mov_r8_imm8(cpu: &mut Cpu, reg: GeneralByteReg, imm: u8) {
    cpu.set_reg_8(reg, imm);
}

pub fn mov_al_moffs8(cpu: &mut Cpu, segment: SegmentReg, offset: u16) {
    let value = cpu.mem_8(segment, offset);
    cpu.set_reg_8(Al, value);
}

pub fn mov_rm8_r8(cpu: &mut Cpu, reg: GeneralByteReg, rm: RegMem) {
    let value = cpu.reg_8(reg);
    rm.set_8(cpu, value);
}

pub fn mov_r16_imm16(cpu: &mut Cpu, reg: GeneralWordReg, imm: u16) {
    cpu.set_reg_16(reg.into(), imm);
}

pub fn mov_rm16_r16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let value = cpu.reg_16(reg.into());
    rm.set_16(cpu, value);
}

pub fn mov_sreg_rm16(cpu: &mut Cpu, reg: SegmentReg, rm: RegMem) {
    let value = rm.get_16(cpu);
    cpu.set_reg_16(reg.into(), value);
}

pub fn mov_r16_rm16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let value = rm.get_16(cpu);
    cpu.set_reg_16(reg.into(), value);
}

pub fn les_r16_m16_16(cpu: &mut Cpu, reg: GeneralWordReg, mem: RmPtr) {
    // TODO: Ensure this is correct
    let (instr_segment, offset_addr) = mem.address(cpu);
    let offset = cpu.mem_16(instr_segment, offset_addr);
    let segment = cpu.mem_16(instr_segment, offset_addr + 2);
    cpu.set_reg_16(reg.into(), offset);
    cpu.set_reg_16(Es.into(), segment);
}

pub fn mov_r8_rm8(cpu: &mut Cpu, reg: GeneralByteReg, rm: RegMem) {
    let value = rm.get_8(cpu);
    cpu.set_reg_8(reg, value);
}
