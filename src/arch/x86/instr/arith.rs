use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::GeneralWordReg::Ax;
use crate::arch::x86::{Cpu, GeneralByteReg, GeneralWordReg, RegMem};

pub fn xor_rm16_r16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let old = rm.get_16(cpu);
    let value = old ^ cpu.reg_16(reg.into());
    rm.set_16(cpu, value);

    cpu.flags.set_pzs_from_u16(value);
}

pub fn cmp_al_imm8(cpu: &mut Cpu, imm: u8) {
    let old = cpu.reg_8(Al);
    cmp_8(cpu, old, imm);
}

pub fn cmp_rm8_imm8(cpu: &mut Cpu, rm: RegMem, imm: u8) {
    let old = rm.get_8(cpu);
    cmp_8(cpu, old, imm);
}

pub fn add_r16_rm16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let old = cpu.reg_16(reg.into());
    let addend = rm.get_16(cpu);
    let value = add_16(cpu, old, addend);
    cpu.set_reg_16(reg.into(), value);
}

pub fn add_rm16_imm8(cpu: &mut Cpu, rm: RegMem, imm: u8) {
    let old = rm.get_16(cpu);
    let addend = (imm as i8) as u16;
    let value = add_16(cpu, old, addend);
    rm.set_16(cpu, value);
}

pub fn adc_ax_imm16(cpu: &mut Cpu, imm: u16) {
    let old = cpu.reg_16(Ax.into());
    let value = adc_16(cpu, old, imm);
    cpu.set_reg_16(Ax.into(), value);
}

pub fn add_rm8_r8(cpu: &mut Cpu, reg: GeneralByteReg, rm: RegMem) {
    let old = rm.get_8(cpu);
    let addend = cpu.reg_8(reg);
    let value = add_8(cpu, old, addend);
    rm.set_8(cpu, value);
}

fn cmp_8(cpu: &mut Cpu, left: u8, right: u8) {
    let (value, unsigned_overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i8).overflowing_sub(right as i8);

    cpu.flags.set_pzs_from_u8(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

fn add_8(cpu: &mut Cpu, left: u8, right: u8) -> u8 {
    let (value, unsigned_overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i8).overflowing_add(right as i8);

    cpu.flags.set_pzs_from_u8(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}

fn add_16(cpu: &mut Cpu, left: u16, right: u16) -> u16 {
    let (value, unsigned_overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);

    cpu.flags.set_pzs_from_u16(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}

fn adc_16(cpu: &mut Cpu, left: u16, right: u16) -> u16 {
    let carry = cpu.flags.carry as u16;

    let (value, unsigned_overflow) = match left.overflowing_add(right) {
        (value, true) => (value.wrapping_add(carry), true),
        (value, false) => value.overflowing_add(carry),
    };
    let (_, signed_overflow) = match (left as i16).overflowing_add(right as i16) {
        (value, true) => (value.wrapping_add(carry as i16), true),
        (value, false) => value.overflowing_add(carry as i16),
    };

    cpu.flags.set_pzs_from_u16(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF

    value
}
