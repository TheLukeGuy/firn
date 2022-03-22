use crate::arch::x86::Cpu;

pub fn cli(cpu: &mut Cpu) {
    cpu.flags.interrupt = false;
}
