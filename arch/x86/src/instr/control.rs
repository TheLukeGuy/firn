use crate::GeneralWordReg::{Bp, Cx, Sp};
use crate::SegmentReg::Cs;
use crate::{ExtSystem, RegMem, System};
use firn_arch_x86_macros::instr;

#[instr(JCXZ rel8)]
pub fn jcxz_rel8(sys: &mut System, rel: u8) {
    let value = sys.cpu.reg_16(Cx.into());
    if value == 0 {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(LOOP rel8)]
pub fn loop_rel8(sys: &mut System, rel: u8) {
    let value = sys.cpu.reg_16(Cx.into()).wrapping_sub(1);
    sys.cpu.set_reg_16(Cx.into(), value);

    if value != 0 {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(LOOPE rel8)]
pub fn loope_rel8(sys: &mut System, rel: u8) {
    let value = sys.cpu.reg_16(Cx.into()).wrapping_sub(1);
    sys.cpu.set_reg_16(Cx.into(), value);

    if value != 0 && sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(LOOPNE rel8)]
pub fn loopne_rel8(sys: &mut System, rel: u8) {
    let value = sys.cpu.reg_16(Cx.into()).wrapping_sub(1);
    sys.cpu.set_reg_16(Cx.into(), value);

    if value != 0 && !sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JMP rel8)]
pub fn jmp_rel8(sys: &mut System, rel: u8) {
    sys.cpu.inc_ip_8(rel);
}

#[instr(JMP rel16)]
pub fn jmp_rel16(sys: &mut System, rel: u16) {
    sys.cpu.inc_ip_16(rel);
}

#[instr(JMP r/m16)]
pub fn jmp_rm16(sys: &mut System, rm: RegMem) {
    let value = rm.get_16(sys);
    sys.cpu.ip = value;
}

#[instr(JMP ptr16:16)]
pub fn jmp_ptr16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Cs.into(), segment);
    sys.cpu.ip = offset;
}

#[instr(JMP m16:16)]
pub fn jmp_m16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.cpu.set_reg_16(Cs.into(), segment);
    sys.cpu.ip = offset;
}

#[instr(CALL rel16)]
pub fn call_rel16(sys: &mut System, imm: u16) {
    sys.push_16(sys.cpu.ip);
    sys.cpu.inc_ip_16(imm);
}

#[instr(CALL r/m16)]
pub fn call_rm16(sys: &mut System, rm: RegMem) {
    sys.push_16(sys.cpu.ip);
    sys.cpu.ip = rm.get_16(sys);
}

#[instr(CALL ptr16:16)]
pub fn call_ptr16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.push_16(sys.cpu.ip);
    sys.push_reg_16(Cs.into());

    sys.cpu.ip = offset;
    sys.cpu.set_reg_16(Cs.into(), segment);
}

#[instr(CALL m16:16)]
pub fn call_m16_16(sys: &mut System, offset: u16, segment: u16) {
    sys.push_16(sys.cpu.ip);
    sys.push_reg_16(Cs.into());

    sys.cpu.ip = offset;
    sys.cpu.set_reg_16(Cs.into(), segment);
}

#[instr(RET)]
pub fn ret_near(sys: &mut System) {
    sys.cpu.ip = sys.pop_16();
}

#[instr(RET)]
pub fn ret_far(sys: &mut System) {
    sys.cpu.ip = sys.pop_16();
    let cs = sys.pop_16();
    sys.cpu.set_reg_16(Cs.into(), cs);
}

#[instr(RET imm16)]
pub fn ret_imm16_near(sys: &mut System, imm: u16) {
    sys.cpu.ip = sys.pop_16();
    sys.cpu.inc_reg_16(Sp.into(), imm);
}

#[instr(RET imm16)]
pub fn ret_imm16_far(sys: &mut System, imm: u16) {
    sys.cpu.ip = sys.pop_16();
    let cs = sys.pop_16();
    sys.cpu.set_reg_16(Cs.into(), cs);
    sys.cpu.inc_reg_16(Sp.into(), imm);
}

#[instr(ENTER imm16, imm8)]
pub fn enter_imm16_imm8(sys: &mut System, first: u16, second: u8) {
    let level = second % 32;
    sys.push_reg_16(Bp.into());

    let frame_ptr = sys.cpu.reg_16(Sp.into());
    if level > 0 {
        for _ in 1..level {
            sys.cpu.dec_reg_16(Bp.into(), 2);
            sys.push_reg_16(Bp.into());
        }
        sys.push_16(frame_ptr);
    }

    sys.cpu.set_reg_16(Bp.into(), frame_ptr);
    sys.cpu.dec_reg_16(Sp.into(), first);
}

#[instr(LEAVE)]
pub fn leave(sys: &mut System) {
    let new_sp = sys.cpu.reg_16(Bp.into());
    sys.cpu.set_reg_16(Sp.into(), new_sp);
    let new_bp = sys.pop_16();
    sys.cpu.set_reg_16(Bp.into(), new_bp);
}
