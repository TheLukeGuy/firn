use crate::arch::x86::GeneralWordReg::{Ax, Bp, Bx, Cx, Di, Dx, Si, Sp};
use crate::arch::x86::SegmentReg::{Ds, Es};
use crate::arch::x86::{Cpu, GeneralWordReg};

pub fn push_r16(cpu: &mut Cpu, reg: GeneralWordReg) {
    cpu.push_reg_16(reg.into());
}

pub fn push_ds(cpu: &mut Cpu) {
    cpu.push_reg_16(Ds.into());
}

pub fn pop_r16(cpu: &mut Cpu, reg: GeneralWordReg) {
    let value = cpu.pop_16();
    cpu.set_reg_16(reg.into(), value);
}

pub fn push_es(cpu: &mut Cpu) {
    cpu.push_reg_16(Es.into());
}

pub fn pop_es(cpu: &mut Cpu) {
    let value = cpu.pop_16();
    cpu.set_reg_16(Es.into(), value);
}

pub fn push_imm16(cpu: &mut Cpu, imm: u16) {
    cpu.push_16(imm);
}

pub fn push_imm8(cpu: &mut Cpu, imm: u8) {
    cpu.push_8(imm);
}

pub fn popa(cpu: &mut Cpu) {
    let di = cpu.pop_16();
    let si = cpu.pop_16();
    let bp = cpu.pop_16();
    cpu.inc_reg_16(Sp.into(), 2);
    let bx = cpu.pop_16();
    let dx = cpu.pop_16();
    let cx = cpu.pop_16();
    let ax = cpu.pop_16();

    cpu.set_reg_16(Di.into(), di);
    cpu.set_reg_16(Si.into(), si);
    cpu.set_reg_16(Bp.into(), bp);
    cpu.set_reg_16(Bx.into(), bx);
    cpu.set_reg_16(Dx.into(), dx);
    cpu.set_reg_16(Cx.into(), cx);
    cpu.set_reg_16(Ax.into(), ax);
}
