use crate::System;
use firn_arch_x86_macros::instr;

#[instr(CLI)]
pub fn cli(sys: &mut System) {
    sys.cpu.flags.interrupt = false;
}

#[instr(CLD)]
pub fn cld(sys: &mut System) {
    sys.cpu.flags.direction = false;
}
