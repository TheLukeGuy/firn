use crate::GeneralWordReg::{Ax, Bp, Bx, Cx, Di, Dx, Si, Sp};
use crate::SegmentReg::{Ds, Es};
use crate::{ExtSystem, GeneralWordReg, System};

pub fn push_r16(sys: &mut System, reg: GeneralWordReg) {
    sys.push_reg_16(reg.into());
}

pub fn push_ds(sys: &mut System) {
    sys.push_reg_16(Ds.into());
}

pub fn pop_r16(sys: &mut System, reg: GeneralWordReg) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(reg.into(), value);
}

pub fn push_es(sys: &mut System) {
    sys.push_reg_16(Es.into());
}

pub fn pop_es(sys: &mut System) {
    let value = sys.pop_16();
    sys.cpu.set_reg_16(Es.into(), value);
}

pub fn push_imm16(sys: &mut System, imm: u16) {
    sys.push_16(imm);
}

pub fn push_imm8(sys: &mut System, imm: u8) {
    sys.push_8(imm);
}

pub fn popa(sys: &mut System) {
    let di = sys.pop_16();
    let si = sys.pop_16();
    let bp = sys.pop_16();
    sys.cpu.inc_reg_16(Sp.into(), 2);
    let bx = sys.pop_16();
    let dx = sys.pop_16();
    let cx = sys.pop_16();
    let ax = sys.pop_16();

    sys.cpu.set_reg_16(Di.into(), di);
    sys.cpu.set_reg_16(Si.into(), si);
    sys.cpu.set_reg_16(Bp.into(), bp);
    sys.cpu.set_reg_16(Bx.into(), bx);
    sys.cpu.set_reg_16(Dx.into(), dx);
    sys.cpu.set_reg_16(Cx.into(), cx);
    sys.cpu.set_reg_16(Ax.into(), ax);
}
