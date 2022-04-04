use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{instr, Cpu, ExtSystem, GeneralByteReg, GeneralWordReg, Instr, SegmentReg};
use firn_core::System;
use std::io;
use std::io::Write;

fn match_opcode(sys: &mut System<Cpu>, opcode: u8, segment: SegmentReg, rep: bool) -> Instr {
    match opcode {
        0x00 => Instr::new_r8_rm8(instr::arith::add_rm8_r8, &mut sys.cpu),
        0x03 => Instr::new_r16_rm16(instr::arith::add_r16_rm16, &mut sys.cpu),
        0x06 => Instr::new_basic(instr::stack::push_es),
        0x07 => Instr::new_basic(instr::stack::pop_es),
        0x15 => Instr::new_imm16(instr::arith::adc_ax_imm16, &mut sys.cpu),
        0x1e => Instr::new_basic(instr::stack::push_ds),
        0x31 => Instr::new_r16_rm16(instr::arith::xor_rm16_r16, &mut sys.cpu),
        0x3c => Instr::new_imm8(instr::arith::cmp_al_imm8, &mut sys.cpu),
        0x3d => Instr::new_imm16(instr::arith::cmp_ax_imm16, &mut sys.cpu),
        opcode @ 0x50..=0x57 => Instr::new_r16(instr::stack::push_r16, reg_16(opcode)),
        opcode @ 0x58..=0x5f => Instr::new_r16(instr::stack::pop_r16, reg_16(opcode)),
        0x61 => Instr::new_basic(instr::stack::popa),
        0x68 => Instr::new_imm16(instr::stack::push_imm16, &mut sys.cpu),
        0x6a => Instr::new_imm8(instr::stack::push_imm8, &mut sys.cpu),
        0x74 => Instr::new_imm8(instr::control::jz_rel8, &mut sys.cpu),
        0x7c => Instr::new_imm8(instr::control::jl_rel8, &mut sys.cpu),
        opcode @ 0x80 => match extension(sys) {
            7 => Instr::new_rm8_imm8(instr::arith::cmp_rm8_imm8, &mut sys.cpu),

            extension => invalid(sys, opcode, Some(extension)),
        },
        opcode @ 0x83 => match extension(sys) {
            0 => Instr::new_rm16_imm8(instr::arith::add_rm16_imm8, &mut sys.cpu),

            extension => invalid(sys, opcode, Some(extension)),
        },
        0x88 => Instr::new_r8_rm8(instr::transfer::mov_rm8_r8, &mut sys.cpu),
        0x89 => Instr::new_r16_rm16(instr::transfer::mov_rm16_r16, &mut sys.cpu),
        0x8a => Instr::new_r8_rm8(instr::transfer::mov_r8_rm8, &mut sys.cpu),
        0x8b => Instr::new_r16_rm16(instr::transfer::mov_r16_rm16, &mut sys.cpu),
        0x8e => Instr::new_sreg_rm16(instr::transfer::mov_sreg_rm16, &mut sys.cpu),
        0xa0 => Instr::new_moffs8(instr::transfer::mov_al_moffs8, &mut sys.cpu, segment),
        0xaa => Instr::new_basic_rep(instr::strings::stosb, rep),
        0xab => Instr::new_basic_rep(instr::strings::stosw, rep),
        opcode @ 0xb0..=0xb7 => {
            Instr::new_r8_imm8(instr::transfer::mov_r8_imm8, &mut sys.cpu, reg_8(opcode))
        }
        opcode @ 0xb8..=0xbf => {
            Instr::new_r16_imm16(instr::transfer::mov_r16_imm16, &mut sys.cpu, reg_16(opcode))
        }
        0xc3 => Instr::new_basic(instr::control::ret),
        0xc4 => Instr::new_r16_m16(instr::transfer::les_r16_m16_16, &mut sys.cpu),
        0xc8 => Instr::new_imm16_imm8(instr::control::enter_imm16_imm8, &mut sys.cpu),
        0xe3 => Instr::new_imm8(instr::control::jcxz_rel8, &mut sys.cpu),
        0xe4 => Instr::new_imm8(instr::ports::in_al_imm8, &mut sys.cpu),
        0xe6 => Instr::new_imm8(instr::ports::out_imm8_al, &mut sys.cpu),
        0xe8 => Instr::new_imm16(instr::control::call_rel16, &mut sys.cpu),
        0xea => Instr::new_ptr16_16(instr::control::jmp_ptr16_16, &mut sys.cpu),
        0xec => Instr::new_basic(instr::ports::in_al_dx),
        0xfa => Instr::new_basic(instr::flags::cli),
        0xfc => Instr::new_basic(instr::flags::cld),

        opcode => invalid(sys, opcode, None),
    }
}

pub fn decode(sys: &mut System<Cpu>) -> Instr {
    let mut segment = Ds;
    let mut rep = false;

    loop {
        match sys.read_mem_8() {
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

                break match_opcode(sys, opcode, segment, rep);
            }
        }
    }
}

fn extension(sys: &mut System<Cpu>) -> u8 {
    (sys.read_mem_8() / 0o10) % 0o10
}

fn invalid(sys: &mut System<Cpu>, opcode: u8, extension: Option<u8>) -> ! {
    match extension {
        Some(extension) => panic!(
            "invalid or unimplemented instruction: {:#x} /{}",
            opcode, extension
        ),
        None => {
            let extension = self::extension(sys);
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
