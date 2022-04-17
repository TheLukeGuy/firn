use crate::GeneralWordReg::Ax;
use crate::SegmentReg::Cs;
use crate::{ExtSystem, System};
use firn_arch_x86_macros::instr;
use std::thread;
use std::time::Duration;

#[instr("WAIT")]
pub fn wait(sys: &mut System) {
    // WAIT is unimplemented and performs a NOP
    let first = sys.cpu.reg_16(Ax.into());
    let second = sys.cpu.reg_16(Ax.into());
    sys.cpu.set_reg_16(Ax.into(), second);
    sys.cpu.set_reg_16(Ax.into(), first);
}

#[instr("HLT")]
pub fn hlt(_sys: &mut System) {
    println!("CPU is halted");
    // TODO: Check for interrupts
    loop {
        thread::sleep(Duration::from_millis(1));
    }
}

#[instr("INT 3")]
pub fn int_3(sys: &mut System) {
    sys.interrupt(3);
}

#[instr("INT imm8")]
pub fn int_imm8(sys: &mut System, imm: u8) {
    sys.interrupt(imm);
}

#[instr("INTO")]
pub fn into(sys: &mut System) {
    if sys.cpu.flags.overflow {
        sys.interrupt(4);
    }
}

#[instr("IRET")]
pub fn iret(sys: &mut System) {
    sys.cpu.ip = sys.pop_16();
    let cs = sys.pop_16();
    sys.cpu.set_reg_16(Cs.into(), cs);
    let flags = sys.pop_16();
    sys.cpu.flags.set_16(flags);
}
