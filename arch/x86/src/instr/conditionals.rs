use crate::System;
use firn_arch_x86_macros::instr;

#[instr(JA rel8)]
pub fn ja_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry && !sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JBE rel8)]
pub fn jbe_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry || sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JG rel8)]
pub fn jg_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero && (sys.cpu.flags.sign == sys.cpu.flags.overflow) {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JGE rel8)]
pub fn jge_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign == sys.cpu.flags.overflow {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JL rel8)]
pub fn jl_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign != sys.cpu.flags.overflow {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JLE rel8)]
pub fn jle_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero || (sys.cpu.flags.sign != sys.cpu.flags.overflow) {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JZ rel8)]
pub fn jz_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JNZ rel8)]
pub fn jnz_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.zero {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JO rel8)]
pub fn jo_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.overflow {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JNO rel8)]
pub fn jno_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.overflow {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JC rel8)]
pub fn jc_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.carry {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JNC rel8)]
pub fn jnc_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.carry {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JS rel8)]
pub fn js_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.sign {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JNS rel8)]
pub fn jns_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.sign {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JP rel8)]
pub fn jp_rel8(sys: &mut System, rel: u8) {
    if sys.cpu.flags.parity {
        sys.cpu.inc_ip_8(rel);
    }
}

#[instr(JNP rel8)]
pub fn jnp_rel8(sys: &mut System, rel: u8) {
    if !sys.cpu.flags.parity {
        sys.cpu.inc_ip_8(rel);
    }
}
