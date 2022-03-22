use crate::arch::x86::SegmentReg::{Cs, Ds, Es, Ss};
use crate::arch::x86::{instr, modrm, Cpu, GeneralByteReg, Instr, InstrFunc};

pub fn decode(cpu: &mut Cpu) -> Instr {
    let mut segment = None;

    loop {
        match cpu.read_mem_8() {
            0x26 => segment = Some(Es),
            0x2e => segment = Some(Cs),
            0x36 => segment = Some(Ss),
            0x3e => segment = Some(Ds),
            0xb0 => {
                break Instr::R8Imm8 {
                    func: InstrFunc(instr::transfer::mov_r8_imm8),
                    reg: reg_8(cpu),
                    imm: cpu.read_mem_8(),
                }
            }
            0xea => {
                break Instr::Ptr16_16 {
                    func: InstrFunc(instr::control::jmp_ptr16_16),
                    offset: cpu.read_mem_16(),
                    segment: cpu.read_mem_16(),
                }
            }
            0xfa => break Instr::Basic(InstrFunc(instr::flags::cli)),
            opcode => panic!("invalid or unimplemented instruction: {:#x}", opcode),
        }
    }
}

fn reg_8(cpu: &mut Cpu) -> GeneralByteReg {
    modrm::decode_reg_8(cpu.read_mem_8())
}
