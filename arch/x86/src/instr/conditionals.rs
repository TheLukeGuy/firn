use crate::System;
use firn_arch_x86_macros::instr;

#[instr(JA rel8)]
pub fn ja(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry && !sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JBE rel8)]
pub fn jbe(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry || sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JG rel8)]
pub fn jg(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero && (sys.cpu.flags.sign == sys.cpu.flags.overflow) {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JGE rel8)]
pub fn jge(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign == sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JL rel8)]
pub fn jl(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign != sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JLE rel8)]
pub fn jle(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero || (sys.cpu.flags.sign != sys.cpu.flags.overflow) {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JZ rel8)]
pub fn jz(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNZ rel8)]
pub fn jnz(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JO rel8)]
pub fn jo(sys: &mut System, rel: u8) {
    if sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNO rel8)]
pub fn jno(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.overflow {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JC rel8)]
pub fn jc(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNC rel8)]
pub fn jnc(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JS rel8)]
pub fn js(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNS rel8)]
pub fn jns(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.sign {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JP rel8)]
pub fn jp(sys: &mut System, rel: u8) {
    if sys.cpu.flags.parity {
        sys.cpu.ip += rel as u16;
    }
}

#[instr(JNP rel8)]
pub fn jnp(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.parity {
        sys.cpu.ip += rel as u16;
    }
}
