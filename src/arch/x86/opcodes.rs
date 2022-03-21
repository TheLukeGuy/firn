use crate::arch::x86::{instr, Cpu, Instr, InstrFunc};

pub fn decode(cpu: &mut Cpu, opcode: u8) -> Instr {
    match opcode {
        0xea => Instr::Ptr16_16 {
            func: InstrFunc(instr::control::jmp_ptr16_16),
            offset: cpu.read_mem_16(),
            segment: cpu.read_mem_16(),
        },
        _ => panic!("invalid or unimplemented instruction: {:#x}", opcode),
    }
}
