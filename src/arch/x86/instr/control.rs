use crate::arch::x86::Cpu;
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
