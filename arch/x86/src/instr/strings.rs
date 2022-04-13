use crate::GeneralByteReg::Al;
use crate::GeneralWordReg::{Ax, Di, Dx, Si};
use crate::SegmentReg::{Ds, Es};
use crate::{ExtSystem, GeneralWordReg, Prefixes, System};
use firn_arch_x86_macros::instr;

#[instr("INSB", REP)]
pub fn insb(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.port_in_8(port).unwrap_or(0);
    sys.set_mem_reg_8(Es, Di, value);

    increment(sys, Di, 1);
}

#[instr("INSW", REP)]
pub fn insw(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.port_in_16(port).unwrap_or(0);
    sys.set_mem_reg_16(Es, Di, value);

    increment(sys, Di, 2);
}

#[instr("OUTSB", REP)]
pub fn outsb(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.mem_reg_8(Ds, Si);
    sys.port_out_8(port, value);

    increment(sys, Si, 1);
}

#[instr("OUTSW", REP)]
pub fn outsw(sys: &mut System) {
    let port = sys.cpu.reg_16(Dx.into());
    let value = sys.mem_reg_16(Ds, Si);
    sys.port_out_16(port, value);

    increment(sys, Si, 2);
}

#[instr("MOVSB", REP)]
pub fn movsb(sys: &mut System, prefixes: &Prefixes) {
    let value = sys.mem_reg_8(prefixes.segment, Si);
    sys.set_mem_reg_8(Es, Di, value);

    increment(sys, Di, 1);
    increment(sys, Si, 1);
}

#[instr("MOVSW", REP)]
pub fn movsw(sys: &mut System, prefixes: &Prefixes) {
    let value = sys.mem_reg_16(prefixes.segment, Si);
    sys.set_mem_reg_16(Es, Di, value);

    increment(sys, Di, 2);
    increment(sys, Si, 2);
}

// These will be done after arith instructions
// TODO: CMPSB
// TODO: CMPSW

#[instr("STOSB", REP)]
pub fn stosb(sys: &mut System) {
    let value = sys.cpu.reg_8(Al);
    sys.set_mem_reg_8(Es, Di, value);

    increment(sys, Di, 1);
}

#[instr("STOSW", REP)]
pub fn stosw(sys: &mut System) {
    let value = sys.cpu.reg_16(Ax.into());
    sys.set_mem_reg_16(Es, Di, value);

    increment(sys, Di, 2);
}

#[instr("LODSB", REP)]
pub fn lodsb(sys: &mut System, prefixes: &Prefixes) {
    let value = sys.mem_reg_8(prefixes.segment, Si);
    sys.cpu.set_reg_8(Al, value);

    increment(sys, Si, 1);
}

#[instr("LODSW", REP)]
pub fn lodsw(sys: &mut System, prefixes: &Prefixes) {
    let value = sys.mem_reg_16(prefixes.segment, Si);
    sys.cpu.set_reg_16(Ax.into(), value);

    increment(sys, Si, 2);
}

// These will be done after arith instructions
// TODO: SCASB
// TODO: SCASW

fn increment(sys: &mut System, reg: GeneralWordReg, amount: u16) {
    if !sys.cpu.flags.direction {
        sys.cpu.inc_reg_16(reg.into(), amount);
    } else {
        sys.cpu.dec_reg_16(reg.into(), amount);
    }
}
