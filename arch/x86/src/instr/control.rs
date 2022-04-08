use crate::GeneralWordReg::{Bp, Cx, Sp};
use crate::SegmentReg::Cs;
use crate::{ExtSystem, System};

pub fn jmp_ptr16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Cs.into(), segment);
    sys.cpu.ip = offset;
}

pub fn jz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

pub fn enter_imm16_imm8(sys: &mut System, first: u16, second: u8) {
    sys.push_reg_16(Bp.into());

    let frame_temp = sys.cpu.reg_16(Sp.into());
    if second > 0 {
        for _ in 1..second {
            sys.cpu.dec_reg_16(Bp.into(), 2);
            sys.push_reg_16(Bp.into());
        }
        sys.push_16(frame_temp);
    }

    sys.cpu.set_reg_16(Bp.into(), frame_temp);
    // TODO: Ensure this is correct
    sys.cpu.dec_reg_16(Sp.into(), first);
}

pub fn call_rel16(sys: &mut System, imm: u16) {
    sys.push_16(sys.cpu.ip);
    sys.cpu.ip = sys.cpu.ip.wrapping_add(imm);
}

pub fn ret(sys: &mut System) {
    sys.cpu.ip = sys.pop_16();
}

pub fn jcxz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.reg_16(Cx.into()) == 0 {
        sys.cpu.ip += rel as u16;
    }
}

pub fn jl_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign != sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}
