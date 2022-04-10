use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Ax;
use crate::SegmentReg::{Ds, Es};
use crate::{
    ExtSystem, GeneralByteReg, GeneralWordReg, Prefixes, RegMem, RmPtr, SegmentReg, System,
};
use firn_arch_x86_macros::instr;

#[instr(XCHG AX, r16)]
pub fn xchg_ax_r16(sys: &mut System, reg: GeneralWordReg) {
    let first = sys.cpu.reg_16(Ax.into());
    let second = sys.cpu.reg_16(reg.into());
    sys.cpu.set_reg_16(Ax.into(), second);
    sys.cpu.set_reg_16(reg.into(), first);
}

#[instr(XCHG r/m8, r8)]
pub fn xchg_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let first = rm.get_8(sys);
    let second = sys.cpu.reg_8(reg);
    rm.set_8(sys, second);
    sys.cpu.set_reg_8(reg, first);
}

#[instr(XCHG r/m16, r16)]
pub fn xchg_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let first = rm.get_16(sys);
    let second = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, second);
    sys.cpu.set_reg_16(reg.into(), first);
}

#[instr(MOV r/m8, r8)]
pub fn mov_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let value = sys.cpu.reg_8(reg);
    rm.set_8(sys, value);
}

#[instr(MOV r/m16, r16)]
pub fn mov_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let value = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);
}

#[instr(MOV r8, r/m8)]
pub fn mov_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let value = rm.get_8(sys);
    sys.cpu.set_reg_8(reg, value);
}

#[instr(MOV r16, r/m16)]
pub fn mov_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr(MOV r/m16, Sreg)]
pub fn mov_rm16_sreg(sys: &mut System, rm: RegMem, reg: SegmentReg) {
    let value = sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);
}

#[instr(MOV Sreg, r/m16)]
pub fn mov_sreg_rm16(sys: &mut System, reg: SegmentReg, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr(MOV AL, moffs8)]
pub fn mov_al_moffs8(sys: &mut System, offset: u16, prefixes: &Prefixes) {
    let value = sys.mem_8(prefixes.segment, offset);
    sys.cpu.set_reg_8(Al, value);
}

#[instr(MOV AX, moffs16)]
pub fn mov_ax_moffs16(sys: &mut System, offset: u16, prefixes: &Prefixes) {
    let value = sys.mem_16(prefixes.segment, offset);
    sys.cpu.set_reg_16(Ax.into(), value);
}

#[instr(MOV moffs8, AL)]
pub fn mov_moffs8_al(sys: &mut System, offset: u16, prefixes: &Prefixes) {
    let value = sys.cpu.reg_8(Al);
    sys.set_mem_8(prefixes.segment, offset, value);
}

#[instr(MOV moffs16, AX)]
pub fn mov_moffs16_ax(sys: &mut System, offset: u16, prefixes: &Prefixes) {
    let value = sys.cpu.reg_16(Ax.into());
    sys.set_mem_16(prefixes.segment, offset, value);
}

#[instr(MOV r8, imm8)]
pub fn mov_r8_imm8(sys: &mut System, reg: GeneralByteReg, imm: u8) {
    sys.cpu.set_reg_8(reg, imm);
}

#[instr(MOV r16, imm16)]
pub fn mov_r16_imm16(sys: &mut System, reg: GeneralWordReg, imm: u16) {
    sys.cpu.set_reg_16(reg.into(), imm);
}

#[instr(MOV r/m8, imm8)]
pub fn mov_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    rm.set_8(sys, imm);
}

#[instr(MOV r/m16, imm16)]
pub fn mov_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    rm.set_16(sys, imm);
}

#[instr(LEA r16, m16)]
pub fn lea_r16_m16(sys: &mut System, reg: GeneralWordReg, ptr: RmPtr) {
    let (_, offset) = ptr.address(sys);
    sys.cpu.set_reg_16(reg.into(), offset);
}

#[instr(LDS r16, m16:16)]
pub fn lds_r16_m16_16(sys: &mut System, reg: GeneralWordReg, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Ds.into(), segment);
    sys.cpu.set_reg_16(reg.into(), offset);
}

#[instr(LES r16, m16:16)]
pub fn les_r16_m16_16(sys: &mut System, reg: GeneralWordReg, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Es.into(), segment);
    sys.cpu.set_reg_16(reg.into(), offset);
}
