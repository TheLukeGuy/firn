use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::{Cpu, GeneralWordReg, RegMem};

pub fn xor_rm16_r16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let old = rm.get_16(cpu);
    let value = old ^ cpu.get_reg_16(reg.into());
    rm.set_16(cpu, value);

    cpu.flags.set_pzs_from_u16(value);
}

pub fn cmp_al_imm8(cpu: &mut Cpu, imm: u8) {
    let old = cpu.get_reg_8(Al);
    cmp_8(cpu, old, imm);
}

pub fn cmp_rm8_imm8(cpu: &mut Cpu, rm: RegMem, imm: u8) {
    let old = rm.get_8(cpu);
    cmp_8(cpu, old, imm);
}

pub fn add_r16_rm16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let old = cpu.get_reg_16(reg.into());
    let addend = rm.get_16(cpu);
    let value = add_16(cpu, old, addend);
    cpu.set_reg_16(reg.into(), value);
}

fn cmp_8(cpu: &mut Cpu, left: u8, right: u8) {
    let (value, unsigned_overflow) = left.overflowing_sub(right);
    let (_, signed_overflow) = (left as i8).overflowing_sub(right as i8);

    cpu.flags.set_pzs_from_u8(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
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
