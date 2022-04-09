use crate::System;
use firn_arch_x86_macros::instr;

#[instr(JA rel8)]
pub fn ja_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry && !sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JBE rel8)]
pub fn jbe_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry || sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JG rel8)]
pub fn jg_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero && (sys.cpu.flags.sign == sys.cpu.flags.overflow) {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JGE rel8)]
pub fn jge_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign == sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JL rel8)]
pub fn jl_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign != sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JLE rel8)]
pub fn jle_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero || (sys.cpu.flags.sign != sys.cpu.flags.overflow) {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JZ rel8)]
pub fn jz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNZ rel8)]
pub fn jnz_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JO rel8)]
pub fn jo_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNO rel8)]
pub fn jno_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JC rel8)]
pub fn jc_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNC rel8)]
pub fn jnc_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JS rel8)]
pub fn js_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNS rel8)]
pub fn jns_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.sign {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JP rel8)]
pub fn jp_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.parity {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNP rel8)]
pub fn jnp_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.parity {
        sys.cpu.ip += rel as u16;
    }
}
