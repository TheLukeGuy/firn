use crate::arch::x86::SegmentReg::{Cs, Ds, Es, Ss};
use crate::arch::x86::{
    instr, modrm, Cpu, GeneralByteReg, Instr, InstrFunc, Modrm, ModrmRegType, Size,
};

pub fn decode(cpu: &mut Cpu) -> Instr {
    let mut segment = None;

    loop {
        match cpu.read_mem_8() {
            0x26 => segment = Some(Es),
            0x2e => segment = Some(Cs),
            0x31 => {
                let modrm = modrm_all_16(cpu);
                break Instr::Rm16R16 {
                    func: InstrFunc(instr::arith::xor_rm16_r16),
                    rm: modrm.reg_mem,
                    reg: modrm.word_reg(),
                };
            }
            0x36 => segment = Some(Ss),
            0x3e => segment = Some(Ds),
            0xb0 => {
                break Instr::R8Imm8 {
                    func: InstrFunc(instr::transfer::mov_r8_imm8),
                    reg: reg_8(cpu),
                    imm: cpu.read_mem_8(),
                }
            }
            0xa0 => {
                break Instr::AlMoffs8 {
                    func: InstrFunc(instr::transfer::mov_al_moffs8),
                    segment: segment.unwrap_or(Ds),
                    offset: cpu.read_mem_16(),
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

fn modrm_all_16(cpu: &mut Cpu) -> Modrm {
    let modrm = cpu.read_mem_8();
    Modrm::decode(cpu, modrm, Some(ModrmRegType::WordSized), Size::Word)
}

fn reg_8(cpu: &mut Cpu) -> GeneralByteReg {
    modrm::decode_byte_reg(cpu.read_mem_8())
}
