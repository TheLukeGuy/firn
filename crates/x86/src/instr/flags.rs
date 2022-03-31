use crate::Cpu;

pub fn cli(cpu: &mut Cpu) {
    cpu.flags.interrupt = false;
}

pub fn cld(cpu: &mut Cpu) {
    cpu.flags.direction = false;
}
