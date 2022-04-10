use crate::GeneralWordReg::{Ax, Bp, Bx, Cx, Di, Dx, Si, Sp};
use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{ExtSystem, GeneralWordReg, RmPtr, System};
use firn_arch_x86_macros::instr;

#[instr(PUSH m16)]
pub fn push_m16(sys: &mut System, ptr: RmPtr) {
    let value = ptr.get_16(sys);
    sys.push_16(value);
}

#[instr(PUSH r16)]
pub fn push_r16(sys: &mut System, reg: GeneralWordReg) {
    sys.push_reg_16(reg.into());
}

#[instr(PUSH imm8)]
pub fn push_imm8(sys: &mut System, imm: u8) {
    sys.push_8(imm);
}

#[instr(PUSH imm16)]
pub fn push_imm16(sys: &mut System, imm: u16) {
    sys.push_16(imm);
}

#[instr(PUSH CS)]
pub fn push_cs(sys: &mut System) {
    sys.push_reg_16(Cs.into());
}

#[instr(PUSH SS)]
pub fn push_ss(sys: &mut System) {
    sys.push_reg_16(Ss.into());
}

#[instr(PUSH DS)]
pub fn push_ds(sys: &mut System) {
    sys.push_reg_16(Ds.into());
}

#[instr(PUSH ES)]
pub fn push_es(sys: &mut System) {
    sys.push_reg_16(Es.into());
}

#[instr(PUSHA)]
pub fn pusha(sys: &mut System) {
    let sp = sys.cpu.reg_16(Sp.into());
    sys.push_reg_16(Ax.into());
    sys.push_reg_16(Cx.into());
    sys.push_reg_16(Dx.into());
    sys.push_reg_16(Bx.into());
    sys.push_16(sp);
    sys.push_reg_16(Bp.into());
    sys.push_reg_16(Si.into());
    sys.push_reg_16(Di.into());
}

#[instr(POP m16)]
pub fn pop_m16(sys: &mut System, ptr: RmPtr) {
    let value = sys.pop_16();
    ptr.set_16(sys, value);
}

#[instr(POP r16)]
pub fn pop_r16(sys: &mut System, reg: GeneralWordReg) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(reg.into(), value);
}

#[instr(POP DS)]
pub fn pop_ds(sys: &mut System) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(Ds.into(), value);
}

#[instr(POP ES)]
pub fn pop_es(sys: &mut System) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(Es.into(), value);
}

#[instr(POP SS)]
pub fn pop_ss(sys: &mut System) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(Ss.into(), value);
}

#[instr(POPA)]
pub fn popa(sys: &mut System) {
    sys.pop_reg_16(Di.into());
    sys.pop_reg_16(Si.into());
    sys.pop_reg_16(Bp.into());
    sys.cpu.inc_reg_16(Sp.into(), 2);
    sys.pop_reg_16(Bx.into());
    sys.pop_reg_16(Dx.into());
    sys.pop_reg_16(Cx.into());
    sys.pop_reg_16(Ax.into());
}
