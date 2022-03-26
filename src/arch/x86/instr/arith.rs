use crate::arch::x86::GeneralByteReg::Al;
use crate::arch::x86::{Cpu, GeneralWordReg, RegMem};

pub fn xor_rm16_r16(cpu: &mut Cpu, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(cpu);
    let value = old ^ cpu.get_reg_16(reg.into());
    rm.set_16(cpu, value);

    cpu.flags.set_pzs_from_u16(value);
}

pub fn cmp_al_imm8(cpu: &mut Cpu, imm: u8) {
    let old = cpu.get_reg_8(Al);
    let (value, unsigned_overflow) = old.overflowing_sub(imm);
    let (_, signed_overflow) = (old as i8).overflowing_sub(imm as i8);

    cpu.flags.set_pzs_from_u8(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

pub fn cmp_rm8_imm8(cpu: &mut Cpu, rm: RegMem, imm: u8) {
    let old = rm.get_8(cpu);
    let (value, unsigned_overflow) = old.overflowing_sub(imm);
    let (_, signed_overflow) = (old as i8).overflowing_sub(imm as i8);

    cpu.flags.set_pzs_from_u8(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}

pub fn add_r16_rm16(cpu: &mut Cpu, reg: GeneralWordReg, rm: RegMem) {
    let left = rm.get_16(cpu);
    let right = cpu.get_reg_16(reg.into());
    let (value, unsigned_overflow) = left.overflowing_add(right);
    let (_, signed_overflow) = (left as i16).overflowing_add(right as i16);

    cpu.flags.set_pzs_from_u16(value);
    cpu.flags.carry = unsigned_overflow;
    cpu.flags.overflow = signed_overflow;
    // TODO: Set AF
}
