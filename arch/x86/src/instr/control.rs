use crate::GeneralWordReg::{Bp, Cx, Sp};
use crate::SegmentReg::Cs;
use crate::{ExtSystem, System};
use firn_arch_x86_macros::instr;

#[instr(JMP ptr16:16)]
pub fn jmp_ptr16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Cs.into(), segment);
    sys.cpu.ip = offset;
}

#[instr(JZ rel8)]
pub fn jz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(ENTER imm16, imm8)]
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

#[instr(CALL rel16)]
pub fn call_rel16(sys: &mut System, imm: u16) {
    sys.push_16(sys.cpu.ip);
    sys.cpu.ip = sys.cpu.ip.wrapping_add(imm);
}

#[instr(RET)]
pub fn ret(sys: &mut System) {
    sys.cpu.ip = sys.pop_16();
}

#[instr(JCXZ rel8)]
pub fn jcxz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.reg_16(Cx.into()) == 0 {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JL rel8)]
pub fn jl_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign != sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNZ rel8)]
pub fn jnz_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}
