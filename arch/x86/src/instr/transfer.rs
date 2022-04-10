use crate::GeneralByteReg::Al;
use crate::{ExtSystem, GeneralByteReg, GeneralWordReg, Prefixes, RegMem, SegmentReg, System};
use firn_arch_x86_macros::instr;

#[instr(MOV r8, imm8)]
pub fn mov_r8_imm8(sys: &mut System, reg: GeneralByteReg, imm: u8) {
    sys.cpu.set_reg_8(reg, imm);
}

#[instr(MOV AL, moffs8)]
pub fn mov_al_moffs8(sys: &mut System, offset: u16, prefixes: &Prefixes) {
    let value = sys.mem_8(prefixes.segment, offset);
    sys.cpu.set_reg_8(Al, value);
}

#[instr(MOV r/m8, r8)]
pub fn mov_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let value = sys.cpu.reg_8(reg);
    rm.set_8(sys, value);
}

#[instr(MOV r16, imm16)]
pub fn mov_r16_imm16(sys: &mut System, reg: GeneralWordReg, imm: u16) {
    sys.cpu.set_reg_16(reg.into(), imm);
}

#[instr(MOV r/m16, r16)]
pub fn mov_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let value = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);
}

#[instr(MOV Sreg, r/m16)]
pub fn mov_sreg_rm16(sys: &mut System, reg: SegmentReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr(MOV r16, r/m16)]
pub fn mov_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr(MOV r8, r/m8)]
pub fn mov_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let value = rm.get_8(sys);
    sys.cpu.set_reg_8(reg, value);
}

#[instr(MOV r/m16, Sreg)]
pub fn mov_rm16_sreg(sys: &mut System, rm: RegMem, reg: SegmentReg) {
    let value = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);
}
