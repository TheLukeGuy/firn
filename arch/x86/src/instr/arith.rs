use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Ax;
use crate::{GeneralByteReg, GeneralWordReg, RegMem, System};
use firn_arch_x86_macros::instr;

// ADC
// SUB
// SBB
// CMP
// OR
// AND
// XOR
// NOT
// NEG
// INC
// DEC
// TEST
// MUL
// IMUL
// DIV
// IDIV

#[instr("ADD AL, imm8")]
pub fn add_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    let value = add_8(sys, old, imm);
    sys.cpu.set_reg_8(Al, value);
}

#[instr("ADD AX, imm16")]
pub fn add_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    let value = add_16(sys, old, imm);
    sys.cpu.set_reg_16(Ax.into(), value);
}

#[instr("ADD r/m8, imm8")]
pub fn add_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    let value = add_8(sys, old, imm);
    rm.set_8(sys, value);
}

#[instr("ADD r/m16, imm16")]
pub fn add_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    let old = rm.get_16(sys);
    let value = add_16(sys, old, imm);
    rm.set_16(sys, value);
}

#[instr("ADD r/m16, imm8")]
pub fn add_rm16_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_16(sys);
    let value = add_16(sys, old, imm as u16);
    rm.set_16(sys, value);
}

#[instr("ADD r/m8, r8")]
pub fn add_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let reg = sys.cpu.reg_8(reg);
    let value = add_8(sys, old, reg);
    rm.set_8(sys, value);
}

#[instr("ADD r/m16, r16")]
pub fn add_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let reg = sys.cpu.reg_16(reg.into());
    let value = add_16(sys, old, reg);
    rm.set_16(sys, value);
}

#[instr("ADD r8, r/m8")]
pub fn add_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let old = sys.cpu.reg_8(reg);
    let rm = rm.get_8(sys);
    let value = add_8(sys, old, rm);
    sys.cpu.set_reg_8(reg, value);
}

#[instr("ADD r16, r/m16")]
pub fn add_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let old = sys.cpu.reg_16(reg.into());
    let rm = rm.get_16(sys);
    let value = add_16(sys, old, rm);
    sys.cpu.set_reg_16(reg.into(), value);
}

//

fn set_flags_8(sys: &mut System, value: u8, overflow: bool, signed_overflow: bool) {
    sys.cpu.flags.set_parity_from_u8(value);
    sys.cpu.flags.set_zero_from_u8(value);
    sys.cpu.flags.set_sign_from_u8(value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: AF
}

fn set_flags_16(sys: &mut System, value: u16, overflow: bool, signed_overflow: bool) {
    sys.cpu.flags.set_parity_from_u16(value);
    sys.cpu.flags.set_zero_from_u16(value);
    sys.cpu.flags.set_sign_from_u16(value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: AF
}

fn add_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i8).overflowing_add(right as i8);
    set_flags_8(sys, value, overflow, signed_overflow);

    value
}

fn add_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);
    set_flags_16(sys, value, overflow, signed_overflow);

    value
}
