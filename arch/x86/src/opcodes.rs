use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{instr, ExtSystem, Instr, SegmentReg, System};
use firn_arch_x86_macros::new_instr;
use std::io;
use std::io::Write;

fn match_opcode(sys: &mut System, opcode: u8, segment: SegmentReg, rep: bool) -> Instr {
    match opcode {
        0x00 => new_instr!(opcode, instr::arith::add_rm8_r8),
        0x03 => new_instr!(opcode, instr::arith::add_r16_rm16),
        0x06 => new_instr!(opcode, instr::stack::push_es),
        0x07 => new_instr!(opcode, instr::stack::pop_es),
        0x15 => new_instr!(opcode, instr::arith::adc_ax_imm16),
        0x1e => new_instr!(opcode, instr::stack::push_ds),
        0x31 => new_instr!(opcode, instr::arith::xor_rm16_r16),
        0x3c => new_instr!(opcode, instr::arith::cmp_al_imm8),
        0x3d => new_instr!(opcode, instr::arith::cmp_ax_imm16),
        opcode @ 0x50..=0x57 => new_instr!(opcode, instr::stack::push_r16),
        opcode @ 0x58..=0x5f => new_instr!(opcode, instr::stack::pop_r16),
        0x61 => new_instr!(opcode, instr::stack::popa),
        0x68 => new_instr!(opcode, instr::stack::push_imm16),
        0x6a => new_instr!(opcode, instr::stack::push_imm8),
        0x74 => new_instr!(opcode, instr::control::jz_rel8),
        0x7c => new_instr!(opcode, instr::control::jl_rel8),
        opcode @ 0x80 => match extension(sys) {
            7 => new_instr!(opcode, instr::arith::cmp_rm8_imm8),

            extension => invalid(sys, opcode, Some(extension)),
        },
        opcode @ 0x83 => match extension(sys) {
            0 => new_instr!(opcode, instr::arith::add_rm16_imm8),

            extension => invalid(sys, opcode, Some(extension)),
        },
        0x88 => new_instr!(opcode, instr::transfer::mov_rm8_r8),
        0x89 => new_instr!(opcode, instr::transfer::mov_rm16_r16),
        0x8a => new_instr!(opcode, instr::transfer::mov_r8_rm8),
        0x8b => new_instr!(opcode, instr::transfer::mov_r16_rm16),
        0x8e => new_instr!(opcode, instr::transfer::mov_sreg_rm16),
        0xa0 => new_instr!(opcode, instr::transfer::mov_al_moffs8),
        0xaa => new_instr!(opcode, instr::strings::stosb),
        0xab => new_instr!(opcode, instr::strings::stosw),
        opcode @ 0xb0..=0xb7 => {
            new_instr!(opcode, instr::transfer::mov_r8_imm8)
        }
        opcode @ 0xb8..=0xbf => {
            new_instr!(opcode, instr::transfer::mov_r16_imm16)
        }
        0xc3 => new_instr!(opcode, instr::control::ret),
        0xc4 => new_instr!(opcode, instr::transfer::les_r16_m16_16),
        0xc8 => new_instr!(opcode, instr::control::enter_imm16_imm8),
        0xe3 => new_instr!(opcode, instr::control::jcxz_rel8),
        0xe4 => new_instr!(opcode, instr::ports::in_al_imm8),
        0xe6 => new_instr!(opcode, instr::ports::out_imm8_al),
        0xe8 => new_instr!(opcode, instr::control::call_rel16),
        0xea => new_instr!(opcode, instr::control::jmp_ptr16_16),
        0xec => new_instr!(opcode, instr::ports::in_al_dx),
        0xfa => new_instr!(opcode, instr::flags::cli),
        0xfc => new_instr!(opcode, instr::flags::cld),

        opcode => invalid(sys, opcode, None),
    }
}

pub fn decode(sys: &mut System) -> Instr {
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

fn extension(sys: &mut System) -> u8 {
    (sys.peek_mem_8() / 0o10) % 0o10
}

fn invalid(sys: &mut System, opcode: u8, extension: Option<u8>) -> ! {
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
