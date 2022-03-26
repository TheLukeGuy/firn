use crate::arch::x86::SegmentReg::{Cs, Ds, Es, Ss};
use crate::arch::x86::{
    instr, Cpu, GeneralByteReg, GeneralWordReg, Instr, InstrFunc, Modrm, ModrmRegType, Size,
};

pub fn decode(cpu: &mut Cpu) -> Instr {
    let mut segment = None;

    loop {
        match cpu.read_mem_8() {
            0x03 => {
                let modrm = modrm_all_16(cpu);
                break Instr::R16Rm16 {
                    func: InstrFunc(instr::arith::add_r16_rm16),
                    reg: modrm.word_reg(),
                    rm: modrm.reg_mem,
                };
            }
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
            0x3c => {
                break Instr::Imm8 {
                    func: InstrFunc(instr::arith::cmp_al_imm8),
                    imm: cpu.read_mem_8(),
                };
            }
            0x3e => segment = Some(Ds),
            0x74 => {
                break Instr::Imm8 {
                    func: InstrFunc(instr::control::jz_rel8),
                    imm: cpu.read_mem_8(),
                };
            }
            opcode @ 0x80 => match extension(cpu) {
                7 => {
                    let modrm = modrm_all_8(cpu);
                    break Instr::Rm8Imm8 {
                        func: InstrFunc(instr::arith::cmp_rm8_imm8),
                        rm: modrm.reg_mem,
                        imm: cpu.read_mem_8(),
                    };
                }

                extension => invalid(cpu, opcode, Some(extension)),
            },
            0x88 => {
                let modrm = modrm_all_8(cpu);
                break Instr::Rm8R8 {
                    func: InstrFunc(instr::transfer::mov_rm8_r8),
                    rm: modrm.reg_mem,
                    reg: modrm.byte_reg(),
                };
            }
            0x89 => {
                let modrm = modrm_all_16(cpu);
                break Instr::Rm16R16 {
                    func: InstrFunc(instr::transfer::mov_rm16_r16),
                    rm: modrm.reg_mem,
                    reg: modrm.word_reg(),
                };
            }
            0x8e => {
                let modrm = modrm_segment_16(cpu);
                break Instr::SregRm16 {
                    func: InstrFunc(instr::transfer::mov_sreg_rm16),
                    reg: modrm.segment_reg(),
                    rm: modrm.reg_mem,
                };
            }
            0xa0 => {
                break Instr::Moffs8 {
                    func: InstrFunc(instr::transfer::mov_al_moffs8),
                    segment: segment.unwrap_or(Ds),
                    offset: cpu.read_mem_16(),
                };
            }
            opcode @ 0xb0..=0xb7 => {
                break Instr::R8Imm8 {
                    func: InstrFunc(instr::transfer::mov_r8_imm8),
                    reg: reg_8(opcode),
                    imm: cpu.read_mem_8(),
                };
            }
            opcode @ 0xb8..=0xbf => {
                break Instr::R16Imm16 {
                    func: InstrFunc(instr::transfer::mov_r16_imm16),
                    reg: reg_16(opcode),
                    imm: cpu.read_mem_16(),
                };
            }
            0xe4 => {
                break Instr::Imm8 {
                    func: InstrFunc(instr::ports::in_al_imm8),
                    imm: cpu.read_mem_8(),
                };
            }
            0xe6 => {
                break Instr::Imm8 {
                    func: InstrFunc(instr::ports::out_imm8_al),
                    imm: cpu.read_mem_8(),
                };
            }
            0xea => {
                break Instr::Ptr16_16 {
                    func: InstrFunc(instr::control::jmp_ptr16_16),
                    offset: cpu.read_mem_16(),
                    segment: cpu.read_mem_16(),
                };
            }
            0xfa => break Instr::Basic(InstrFunc(instr::flags::cli)),

            opcode => invalid(cpu, opcode, None),
        }
    }
}

fn extension(cpu: &mut Cpu) -> u8 {
    (cpu.read_mem_8() / 0o10) % 0o10
}

fn invalid(cpu: &mut Cpu, opcode: u8, extension: Option<u8>) {
    match extension {
        Some(extension) => panic!(
            "invalid or unimplemented instruction: {:#x} /{}",
            opcode, extension
        ),
        None => {
            let extension = self::extension(cpu);
            panic!(
                "invalid or unimplemented instruction: {:#x} (potentially /{})",
                opcode, extension
            )
        }
    }
}

fn modrm_all_8(cpu: &mut Cpu) -> Modrm {
    let modrm = cpu.read_mem_8();
    Modrm::decode(cpu, modrm, Some(ModrmRegType::ByteSized), Size::Byte)
}

fn modrm_all_16(cpu: &mut Cpu) -> Modrm {
    let modrm = cpu.read_mem_8();
    Modrm::decode(cpu, modrm, Some(ModrmRegType::WordSized), Size::Word)
}

fn modrm_segment_16(cpu: &mut Cpu) -> Modrm {
    let modrm = cpu.read_mem_8();
    Modrm::decode(cpu, modrm, Some(ModrmRegType::Segment), Size::Word)
}

fn reg_8(opcode: u8) -> GeneralByteReg {
    let reg = opcode % 0o10;
    GeneralByteReg::from_u8(reg).expect("invalid byte-sized register in opcode")
}

fn reg_16(opcode: u8) -> GeneralWordReg {
    let reg = opcode % 0o10;
    GeneralWordReg::from_u8(reg).expect("invalid word-sized register in opcode")
}
