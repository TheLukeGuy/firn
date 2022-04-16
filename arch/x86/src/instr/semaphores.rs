use crate::GeneralWordReg::Ax;
use crate::System;
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

// TODO: INT
// TODO: IRET
