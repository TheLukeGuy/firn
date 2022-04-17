use crate::GeneralByteReg::{Ah, Al};
use crate::GeneralWordReg::{Ax, Dx};
use crate::{arith, GeneralByteReg, GeneralWordReg, RegMem, System};
use firn_arch_x86_macros::{arith_instr, instr};

arith_instr!(ADD);
arith_instr!(ADC);
arith_instr!(SUB);
arith_instr!(SBB);

#[instr("CMP AL, imm8")]
pub fn cmp_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    arith::sub_8(sys, old, imm);
}

#[instr("CMP AX, imm16")]
pub fn cmp_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    arith::sub_16(sys, old, imm);
}

#[instr("CMP r/m8, imm8")]
pub fn cmp_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    arith::sub_8(sys, old, imm);
}

#[instr("CMP r/m16, imm16")]
pub fn cmp_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    let old = rm.get_16(sys);
    arith::sub_16(sys, old, imm);
}

#[instr("CMP r/m16, imm8")]
pub fn cmp_rm16_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_16(sys);
    arith::sub_16(sys, old, imm as u16);
}

#[instr("CMP r/m8, r8")]
pub fn cmp_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let reg = sys.cpu.reg_8(reg);
    arith::sub_8(sys, old, reg);
}

#[instr("CMP r/m16, r16")]
pub fn cmp_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let reg = sys.cpu.reg_16(reg.into());
    arith::sub_16(sys, old, reg);
}

#[instr("CMP r8, r/m8")]
pub fn cmp_r8_rm8(sys: &mut System, reg: GeneralByteReg, rm: RegMem) {
    let old = sys.cpu.reg_8(reg);
    let rm = rm.get_8(sys);
    arith::sub_8(sys, old, rm);
}

#[instr("CMP r16, r/m16")]
pub fn cmp_r16_rm16(sys: &mut System, reg: GeneralWordReg, rm: RegMem) {
    let old = sys.cpu.reg_16(reg.into());
    let rm = rm.get_16(sys);
    arith::sub_16(sys, old, rm);
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

    arith::set_all_flags_8(sys, value, overflow, signed_overflow);
    rm.set_8(sys, value);
}

#[instr("NEG r/m16")]
pub fn neg_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let overflow = old != 0;
    let (value, signed_overflow) = 0i16.overflowing_sub(old as i16);
    let value = value as u16;

    arith::set_all_flags_16(sys, value, overflow, signed_overflow);
    rm.set_16(sys, value);
}

#[instr("INC r/m8")]
pub fn inc_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    let value = arith::add_8(sys, old, 1);
    rm.set_8(sys, value);
}

#[instr("INC r/m16")]
pub fn inc_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let value = arith::add_16(sys, old, 1);
    rm.set_16(sys, value);
}

#[instr("INC r16")]
pub fn inc_r16(sys: &mut System, reg: GeneralWordReg) {
    let old = sys.cpu.reg_16(reg.into());
    let value = arith::add_16(sys, old, 1);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr("DEC r/m8")]
pub fn dec_rm8(sys: &mut System, rm: RegMem) {
    let old = rm.get_8(sys);
    let value = arith::sub_8(sys, old, 1);
    rm.set_8(sys, value);
}

#[instr("DEC r/m16")]
pub fn dec_rm16(sys: &mut System, rm: RegMem) {
    let old = rm.get_16(sys);
    let value = arith::sub_16(sys, old, 1);
    rm.set_16(sys, value);
}

#[instr("DEC r16")]
pub fn dec_r16(sys: &mut System, reg: GeneralWordReg) {
    let old = sys.cpu.reg_16(reg.into());
    let value = arith::sub_16(sys, old, 1);
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr("TEST AL, imm8")]
pub fn test_al_imm8(sys: &mut System, imm: u8) {
    let old = sys.cpu.reg_8(Al);
    arith::and_8(sys, old, imm);
}

#[instr("TEST AX, imm16")]
pub fn test_ax_imm16(sys: &mut System, imm: u16) {
    let old = sys.cpu.reg_16(Ax.into());
    arith::and_16(sys, old, imm);
}

#[instr("TEST r/m8, imm8")]
pub fn test_rm8_imm8(sys: &mut System, rm: RegMem, imm: u8) {
    let old = rm.get_8(sys);
    arith::and_8(sys, old, imm);
}

#[instr("TEST r/m16, imm16")]
pub fn test_rm16_imm16(sys: &mut System, rm: RegMem, imm: u16) {
    let old = rm.get_16(sys);
    arith::and_16(sys, old, imm);
}

#[instr("TEST r/m8, r8")]
pub fn test_rm8_r8(sys: &mut System, rm: RegMem, reg: GeneralByteReg) {
    let old = rm.get_8(sys);
    let reg = sys.cpu.reg_8(reg);
    arith::and_8(sys, old, reg);
}

#[instr("TEST r/m16, r16")]
pub fn test_rm16_r16(sys: &mut System, rm: RegMem, reg: GeneralWordReg) {
    let old = rm.get_16(sys);
    let reg = sys.cpu.reg_16(reg.into());
    arith::and_16(sys, old, reg);
}

#[instr("MUL r/m8")]
pub fn mul_rm8(sys: &mut System, rm: RegMem) {
    let multiplicand = rm.get_8(sys);
    let multiplier = sys.cpu.reg_8(Al);
    let value = multiplicand as u16 * multiplier as u16;
    sys.cpu.set_reg_16(Ax.into(), value);

    let extended = value > u8::MAX as u16;
    sys.cpu.flags.carry = extended;
    sys.cpu.flags.overflow = extended;
}

#[instr("MUL r/m16")]
pub fn mul_rm16(sys: &mut System, rm: RegMem) {
    let multiplicand = rm.get_16(sys);
    let multiplier = sys.cpu.reg_16(Ax.into());
    let value = multiplicand as u32 * multiplier as u32;

    let low = (value & 0xff) as u16;
    let high = (value >> 8) as u16;
    sys.cpu.set_reg_16(Ax.into(), low);
    sys.cpu.set_reg_16(Dx.into(), high);

    let extended = high != 0;
    sys.cpu.flags.carry = extended;
    sys.cpu.flags.overflow = extended;
}

// TODO: IMUL

macro_rules! check_div_8 {
    ($sys:ident, $value:ident, $dividend:ident, $divisor:ident) => {
        if let Ok($value) = u8::try_from($value) {
            let remainder = $dividend % $divisor;
            $sys.cpu.set_reg_8(Ah, remainder as u8);
            $sys.cpu.set_reg_8(Al, $value);
        } else {
            crate::ExtSystem::interrupt($sys, 0);
        }
    };
}

macro_rules! check_div_16 {
    ($sys:ident, $value:ident, $dividend:ident, $divisor:ident) => {
        if let Ok($value) = u16::try_from($value) {
            let remainder = $dividend % $divisor;
            $sys.cpu.set_reg_16(Dx.into(), remainder as u16);
            $sys.cpu.set_reg_16(Ax.into(), $value);
        } else {
            crate::ExtSystem::interrupt($sys, 0);
        }
    };
}

#[instr("DIV r/m8")]
pub fn div_rm8(sys: &mut System, rm: RegMem) {
    let dividend = sys.cpu.reg_16(Ax.into());
    let divisor = rm.get_8(sys) as u16;
    let value = dividend / divisor;

    check_div_8!(sys, value, dividend, divisor);
}

#[instr("DIV r/m16")]
pub fn div_rm16(sys: &mut System, rm: RegMem) {
    let dx = sys.cpu.reg_16(Dx.into());
    let ax = sys.cpu.reg_16(Ax.into());
    let dividend = ((dx as u32) << 16) | ax as u32;
    let divisor = rm.get_16(sys) as u32;
    let value = dividend / divisor;

    check_div_16!(sys, value, dividend, divisor);
}

#[instr("IDIV r/m8")]
pub fn idiv_rm8(sys: &mut System, rm: RegMem) {
    let dividend = sys.cpu.reg_16(Ax.into()) as i16;
    let divisor = rm.get_8(sys) as i16;
    let value = dividend / divisor;

    check_div_8!(sys, value, dividend, divisor);
}

#[instr("IDIV r/m16")]
pub fn idiv_rm16(sys: &mut System, rm: RegMem) {
    let dx = sys.cpu.reg_16(Dx.into());
    let ax = sys.cpu.reg_16(Ax.into());
    let dividend = ((dx as i32) << 16) | ax as i32;
    let divisor = rm.get_16(sys) as i32;
    let value = dividend / divisor;

    check_div_16!(sys, value, dividend, divisor);
}
