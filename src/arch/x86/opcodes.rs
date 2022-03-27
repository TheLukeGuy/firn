use crate::arch::x86::SegmentReg::{Cs, Ds, Es, Ss};
use crate::arch::x86::{instr, Cpu, GeneralByteReg, GeneralWordReg, Instr, SegmentReg};
use std::io;
use std::io::Write;

fn match_opcode(cpu: &mut Cpu, opcode: u8, segment: SegmentReg, rep: bool) -> Instr {
    match opcode {
        0x03 => Instr::new_r16_rm16(instr::arith::add_r16_rm16, cpu),
        0x31 => Instr::new_r16_rm16(instr::arith::xor_rm16_r16, cpu),
        0x3c => Instr::new_imm8(instr::arith::cmp_al_imm8, cpu),
        0x74 => Instr::new_imm8(instr::control::jz_rel8, cpu),
        opcode @ 0x80 => match extension(cpu) {
            7 => Instr::new_rm8_imm8(instr::arith::cmp_rm8_imm8, cpu),

            extension => invalid(cpu, opcode, Some(extension)),
        },
        0x88 => Instr::new_r8_rm8(instr::transfer::mov_rm8_r8, cpu),
        0x89 => Instr::new_r16_rm16(instr::transfer::mov_rm16_r16, cpu),
        0x8e => Instr::new_sreg_rm16(instr::transfer::mov_sreg_rm16, cpu),
        0xa0 => Instr::new_moffs8(instr::transfer::mov_al_moffs8, cpu, segment),
        0xab => Instr::new_basic_rep(instr::strings::stosw, rep),
        opcode @ 0xb0..=0xb7 => {
            Instr::new_r8_imm8(instr::transfer::mov_r8_imm8, cpu, reg_8(opcode))
        }
        opcode @ 0xb8..=0xbf => {
            Instr::new_r16_imm16(instr::transfer::mov_r16_imm16, cpu, reg_16(opcode))
        }
        0xc8 => Instr::new_imm16_imm8(instr::control::enter_imm16_imm8, cpu),
        0xe4 => Instr::new_imm8(instr::ports::in_al_imm8, cpu),
        0xe6 => Instr::new_imm8(instr::ports::out_imm8_al, cpu),
        0xea => Instr::new_ptr16_16(instr::control::jmp_ptr16_16, cpu),
        0xfa => Instr::new_basic(instr::flags::cli),
        0xfc => Instr::new_basic(instr::flags::cld),

        opcode => invalid(cpu, opcode, None),
    }
}

pub fn decode(cpu: &mut Cpu) -> Instr {
    let mut segment = Ds;
    let mut rep = false;

    loop {
        match cpu.read_mem_8() {
            0x26 => segment = Es,
            0x2e => segment = Cs,
            0x36 => segment = Ss,
            0x3e => segment = Ds,
            0xf3 => {
                print!("[REP] ");
                io::stdout().flush().unwrap();

                rep = true;
            }
            opcode => {
                print!("[{:#04x}] ", opcode);
                io::stdout().flush().unwrap();

                break match_opcode(cpu, opcode, segment, rep);
            }
        }
    }
}

fn extension(cpu: &mut Cpu) -> u8 {
    (cpu.read_mem_8() / 0o10) % 0o10
}

fn invalid(cpu: &mut Cpu, opcode: u8, extension: Option<u8>) -> ! {
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

fn reg_8(opcode: u8) -> GeneralByteReg {
    let reg = opcode % 0o10;
    GeneralByteReg::from_u8(reg).expect("invalid byte-sized register in opcode")
}

fn reg_16(opcode: u8) -> GeneralWordReg {
    let reg = opcode % 0o10;
    GeneralWordReg::from_u8(reg).expect("invalid word-sized register in opcode")
}
