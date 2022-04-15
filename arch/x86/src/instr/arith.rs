use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Ax;
use crate::{GeneralByteReg, GeneralWordReg, RegMem, System};
use firn_arch_x86_macros::{arith_instr, instr};

// TODO: MUL
// TODO: IMUL
// TODO: DIV
// TODO: IDIV

arith_instr!(ADD);
arith_instr!(ADC);
arith_instr!(SUB);
arith_instr!(SBB);

#[instr("CMP AL, imm8")]
pub fn cmp_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    sub_8(sys, old, imm);
}

#[instr("CMP AX, imm16")]
pub fn cmp_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    sub_16(sys, old, imm);
}

#[instr("CMP r/m8, imm8")]
pub fn cmp_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    sub_8(sys, old, imm);
}

#[instr("CMP r/m16, imm16")]
pub fn cmp_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    let old = rm.get_16(sys);
    sub_16(sys, old, imm);
}

#[instr("CMP r/m16, imm8")]
pub fn cmp_rm16_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_16(sys);
    sub_16(sys, old, imm as u16);
}

#[instr("CMP r/m8, r8")]
pub fn cmp_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let reg = sys.cpu.reg_8(reg);
    sub_8(sys, old, reg);
}

#[instr("CMP r/m16, r16")]
pub fn cmp_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let reg = sys.cpu.reg_16(reg.into());
    sub_16(sys, old, reg);
}

#[instr("CMP r8, r/m8")]
pub fn cmp_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let old = sys.cpu.reg_8(reg);
    let rm = rm.get_8(sys);
    sub_8(sys, old, rm);
}

#[instr("CMP r16, r/m16")]
pub fn cmp_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let old = sys.cpu.reg_16(reg.into());
    let rm = rm.get_16(sys);
    sub_16(sys, old, rm);
}

arith_instr!(OR);
arith_instr!(AND);
arith_instr!(XOR);

#[instr("NOT r/m8")]
pub fn not_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    rm.set_8(sys, !old);
}

#[instr("NOT r/m16")]
pub fn not_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    rm.set_16(sys, !old);
}

#[instr("NEG r/m8")]
pub fn neg_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    let overflow = old != 0;
    let (value, signed_overflow) = 0i8.overflowing_sub(old as i8);
    let value = value as u8;

    set_all_flags_8(sys, value, overflow, signed_overflow);
    rm.set_8(sys, value);
}

#[instr("NEG r/m16")]
pub fn neg_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let overflow = old != 0;
    let (value, signed_overflow) = 0i16.overflowing_sub(old as i16);
    let value = value as u16;

    set_all_flags_16(sys, value, overflow, signed_overflow);
    rm.set_16(sys, value);
}

#[instr("INC r/m8")]
pub fn inc_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    let value = add_8(sys, old, 1);
    rm.set_8(sys, value);
}

#[instr("INC r/m16")]
pub fn inc_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let value = add_16(sys, old, 1);
    rm.set_16(sys, value);
}

#[instr("INC r16")]
pub fn inc_r16(sys: &mut System, reg: GeneralWordReg) {
    let old = sys.cpu.reg_16(reg.into());
    let value = add_16(sys, old, 1);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr("DEC r/m8")]
pub fn dec_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    let value = sub_8(sys, old, 1);
    rm.set_8(sys, value);
}

#[instr("DEC r/m16")]
pub fn dec_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let value = sub_16(sys, old, 1);
    rm.set_16(sys, value);
}

#[instr("DEC r16")]
pub fn dec_r16(sys: &mut System, reg: GeneralWordReg) {
    let old = sys.cpu.reg_16(reg.into());
    let value = sub_16(sys, old, 1);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr("TEST AL, imm8")]
pub fn test_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    and_8(sys, old, imm);
}

#[instr("TEST AX, imm16")]
pub fn test_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    and_16(sys, old, imm);
}

#[instr("TEST r/m8, imm8")]
pub fn test_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    and_8(sys, old, imm);
}

#[instr("TEST r/m16, imm16")]
pub fn test_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    let old = rm.get_16(sys);
    and_16(sys, old, imm);
}

#[instr("TEST r/m8, r8")]
pub fn test_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let reg = sys.cpu.reg_8(reg);
    and_8(sys, old, reg);
}

#[instr("TEST r/m16, r16")]
pub fn test_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let reg = sys.cpu.reg_16(reg.into());
    and_16(sys, old, reg);
}

fn set_basic_flags_8(sys: &mut System, value: u8) {
    sys.cpu.flags.set_parity_from_u8(value);
    sys.cpu.flags.set_zero_from_u8(value);
    sys.cpu.flags.set_sign_from_u8(value);
}

fn set_basic_flags_16(sys: &mut System, value: u16) {
    sys.cpu.flags.set_parity_from_u16(value);
    sys.cpu.flags.set_zero_from_u16(value);
    sys.cpu.flags.set_sign_from_u16(value);
}

fn set_all_flags_8(sys: &mut System, value: u8, overflow: bool, signed_overflow: bool) {
    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

fn set_all_flags_16(sys: &mut System, value: u16, overflow: bool, signed_overflow: bool) {
    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

fn add_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i8).overflowing_add(right as i8);
    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

fn add_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);
    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

fn adc_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let cf = sys.cpu.flags.carry as u8;

    let (value, first_overflow) = left.overflowing_add(right);
    let (value, second_overflow) = value.overflowing_add(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i8).overflowing_add(right as i8);
    let (_, second_overflow) = signed_value.overflowing_add(cf as i8);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

fn adc_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let cf = sys.cpu.flags.carry as u16;

    let (value, first_overflow) = left.overflowing_add(right);
    let (value, second_overflow) = value.overflowing_add(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i16).overflowing_add(right as i16);
    let (_, second_overflow) = signed_value.overflowing_add(cf as i16);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

fn sub_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i8).overflowing_sub(right as i8);
    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

fn sub_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i16).overflowing_sub(right as i16);
    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

fn sbb_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let cf = sys.cpu.flags.carry as u8;

    let (value, first_overflow) = left.overflowing_sub(right);
    let (value, second_overflow) = value.overflowing_sub(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i8).overflowing_sub(right as i8);
    let (_, second_overflow) = signed_value.overflowing_sub(cf as i8);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_8(sys, value, overflow, signed_overflow);

    value
}

fn sbb_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let cf = sys.cpu.flags.carry as u16;

    let (value, first_overflow) = left.overflowing_sub(right);
    let (value, second_overflow) = value.overflowing_sub(cf);
    let overflow = first_overflow || second_overflow;

    let (signed_value, first_overflow) = (left as i16).overflowing_sub(right as i16);
    let (_, second_overflow) = signed_value.overflowing_sub(cf as i16);
    let signed_overflow = first_overflow || second_overflow;

    set_all_flags_16(sys, value, overflow, signed_overflow);

    value
}

fn or_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left | right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

fn or_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left | right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

fn and_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left & right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

fn and_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left & right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

fn xor_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let value = left ^ right;

    set_basic_flags_8(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}

fn xor_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let value = left ^ right;

    set_basic_flags_16(sys, value);
    sys.cpu.flags.carry = false;
    sys.cpu.flags.overflow = false;

    value
}
