use crate::arch::x86::Cpu;
use crate::arch::x86::GeneralWordReg::{Bp, Cx, Sp};
use crate::arch::x86::SegmentReg::Cs;

pub fn jmp_ptr16_16(cpu: &mut Cpu, offset: u16, segment: u16) {
    cpu.set_reg_16(Cs.into(), segment);
    cpu.ip = offset;
}

pub fn jz_rel8(cpu: &mut Cpu, rel: u8) {
    if cpu.flags.zero {
        cpu.ip += rel as u16;
    }
}

pub fn enter_imm16_imm8(cpu: &mut Cpu, first: u16, second: u8) {
    cpu.push_reg_16(Bp.into());

    let frame_temp = cpu.reg_16(Sp.into());
    if second > 0 {
        for _ in 1..second {
            cpu.dec_reg_16(Bp.into(), 2);
            cpu.push_reg_16(Bp.into());
        }
        cpu.push_16(frame_temp);
    }

    cpu.set_reg_16(Bp.into(), frame_temp);
    // TODO: Ensure this is correct
    cpu.dec_reg_16(Sp.into(), first);
}

pub fn call_rel16(cpu: &mut Cpu, imm: u16) {
    cpu.push_16(cpu.ip);
    cpu.ip = cpu.ip.wrapping_add(imm);
}

pub fn ret(cpu: &mut Cpu) {
    cpu.ip = cpu.pop_16();
}

pub fn jcxz_rel8(cpu: &mut Cpu, rel: u8) {
    if cpu.reg_16(Cx.into()) == 0 {
        cpu.ip += rel as u16;
    }
}

pub fn jl_rel8(cpu: &mut Cpu, rel: u8) {
    if cpu.flags.sign != cpu.flags.overflow {
        cpu.ip += rel as u16;
    }
}
