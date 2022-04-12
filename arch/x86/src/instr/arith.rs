use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::Ax;
use crate::{GeneralByteReg, GeneralWordReg, RegMem, System};
use firn_arch_x86_macros::instr;

#[instr("XOR r/m16, r16")]
pub fn xor_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let value = old ^ sys.cpu.reg_16(reg.into());
    rm.set_16(sys, value);

    sys.cpu.flags.set_pzs_from_u16(value);
}

#[instr("CMP AL, imm8")]
pub fn cmp_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    cmp_8(sys, old, imm);
}

#[instr("CMP r/m8, imm8")]
pub fn cmp_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    cmp_8(sys, old, imm);
}

#[instr("ADD r16, r/m16")]
pub fn add_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let old = sys.cpu.reg_16(reg.into());
    let addend = rm.get_16(sys);
    let value = add_16(sys, old, addend);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr("ADD r/m16, imm8")]
pub fn add_rm16_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_16(sys);
    let addend = (imm as i8) as u16;
    let value = add_16(sys, old, addend);
    rm.set_16(sys, value);
}

#[instr("ADC AX, imm16")]
pub fn adc_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    let value = adc_16(sys, old, imm);
    sys.cpu.set_reg_16(Ax.into(), value);
}

#[instr("ADD r/m8, r8")]
pub fn add_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let addend = sys.cpu.reg_8(reg);
    let value = add_8(sys, old, addend);
    rm.set_8(sys, value);
}

#[instr("CMP AX, imm16")]
pub fn cmp_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    cmp_16(sys, old, imm);
}

fn cmp_8(sys: &mut System, left: u8, right: u8) {
    let (value, unsigned_overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i8).overflowing_sub(right as i8);

    sys.cpu.flags.set_pzs_from_u8(value);
    sys.cpu.flags.carry = unsigned_overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

fn cmp_16(sys: &mut System, left: u16, right: u16) {
    let (value, unsigned_overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i16).overflowing_sub(right as i16);

    sys.cpu.flags.set_pzs_from_u16(value);
    sys.cpu.flags.carry = unsigned_overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

fn add_8(sys: &mut System, left: u8, right: u8) -> u8 {
    let (value, unsigned_overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i8).overflowing_add(right as i8);

    sys.cpu.flags.set_pzs_from_u8(value);
    sys.cpu.flags.carry = unsigned_overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}

fn add_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let (value, unsigned_overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);

    sys.cpu.flags.set_pzs_from_u16(value);
    sys.cpu.flags.carry = unsigned_overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}

fn adc_16(sys: &mut System, left: u16, right: u16) -> u16 {
    let carry = sys.cpu.flags.carry as u16;

    let (value, unsigned_overflow) = match left.overflowing_add(right) {
        (value, true) => (value.wrapping_add(carry), true),
        (value, false) => value.overflowing_add(carry),
    };
    let (_, signed_overflow) = match (left as i16).overflowing_add(right as i16) {
        (value, true) => (value.wrapping_add(carry as i16), true),
        (value, false) => value.overflowing_add(carry as i16),
    };

    sys.cpu.flags.set_pzs_from_u16(value);
    sys.cpu.flags.carry = unsigned_overflow;
    sys.cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}
