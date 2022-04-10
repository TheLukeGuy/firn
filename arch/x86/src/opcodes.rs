use crate::SegmentReg::{Cs, Ds, Es, Ss};
use crate::{instr, ExtSystem, Instr, Prefixes, System};
use firn_arch_x86_macros::new_instr;
use std::io;
use std::io::Write;

fn match_opcode(sys: &mut System, opcode: u8, prefixes: Prefixes) -> Instr {
    match opcode {
        0x00 => new_instr!(opcode, prefixes, instr::arith::add_rm8_r8),
        0x03 => new_instr!(opcode, prefixes, instr::arith::add_r16_rm16),
        0x06 => new_instr!(opcode, prefixes, instr::stack::push_es),
        0x07 => new_instr!(opcode, prefixes, instr::stack::pop_es),
        0x0e => new_instr!(opcode, prefixes, instr::stack::push_cs),
        0x15 => new_instr!(opcode, prefixes, instr::arith::adc_ax_imm16),
        0x16 => new_instr!(opcode, prefixes, instr::stack::push_ss),
        0x17 => new_instr!(opcode, prefixes, instr::stack::pop_ss),
        0x1e => new_instr!(opcode, prefixes, instr::stack::push_ds),
        0x1f => new_instr!(opcode, prefixes, instr::stack::pop_ds),
        0x31 => new_instr!(opcode, prefixes, instr::arith::xor_rm16_r16),
        0x3c => new_instr!(opcode, prefixes, instr::arith::cmp_al_imm8),
        0x3d => new_instr!(opcode, prefixes, instr::arith::cmp_ax_imm16),
        opcode @ 0x50..=0x57 => new_instr!(opcode, prefixes, instr::stack::push_r16),
        opcode @ 0x58..=0x5f => new_instr!(opcode, prefixes, instr::stack::pop_r16),
        0x60 => new_instr!(opcode, prefixes, instr::stack::pusha),
        0x61 => new_instr!(opcode, prefixes, instr::stack::popa),
        0x68 => new_instr!(opcode, prefixes, instr::stack::push_imm16),
        0x6a => new_instr!(opcode, prefixes, instr::stack::push_imm8),
        0x70 => new_instr!(opcode, prefixes, instr::conditionals::jo_rel8),
        0x71 => new_instr!(opcode, prefixes, instr::conditionals::jno_rel8),
        0x72 => new_instr!(opcode, prefixes, instr::conditionals::jc_rel8),
        0x73 => new_instr!(opcode, prefixes, instr::conditionals::jnc_rel8),
        0x74 => new_instr!(opcode, prefixes, instr::conditionals::jz_rel8),
        0x75 => new_instr!(opcode, prefixes, instr::conditionals::jnz_rel8),
        0x76 => new_instr!(opcode, prefixes, instr::conditionals::jbe_rel8),
        0x77 => new_instr!(opcode, prefixes, instr::conditionals::ja_rel8),
        0x78 => new_instr!(opcode, prefixes, instr::conditionals::js_rel8),
        0x79 => new_instr!(opcode, prefixes, instr::conditionals::jns_rel8),
        0x7a => new_instr!(opcode, prefixes, instr::conditionals::jp_rel8),
        0x7b => new_instr!(opcode, prefixes, instr::conditionals::jnp_rel8),
        0x7c => new_instr!(opcode, prefixes, instr::conditionals::jl_rel8),
        0x7d => new_instr!(opcode, prefixes, instr::conditionals::jge_rel8),
        0x7e => new_instr!(opcode, prefixes, instr::conditionals::jle_rel8),
        0x7f => new_instr!(opcode, prefixes, instr::conditionals::jg_rel8),
        opcode @ 0x80 => match extension(sys) {
            7 => new_instr!(opcode, prefixes, instr::arith::cmp_rm8_imm8),

            extension => invalid(sys, opcode, Some(extension)),
        },
        opcode @ 0x83 => match extension(sys) {
            0 => new_instr!(opcode, prefixes, instr::arith::add_rm16_imm8),

            extension => invalid(sys, opcode, Some(extension)),
        },
        0x88 => new_instr!(opcode, prefixes, instr::transfer::mov_rm8_r8),
        0x89 => new_instr!(opcode, prefixes, instr::transfer::mov_rm16_r16),
        0x8a => new_instr!(opcode, prefixes, instr::transfer::mov_r8_rm8),
        0x8b => new_instr!(opcode, prefixes, instr::transfer::mov_r16_rm16),
        0x8c => new_instr!(opcode, prefixes, instr::transfer::mov_rm16_sreg),
        0x8e => new_instr!(opcode, prefixes, instr::transfer::mov_sreg_rm16),
        opcode @ 0x8f => match extension(sys) {
            0 => new_instr!(opcode, prefixes, instr::stack::pop_m16),

            extension => invalid(sys, opcode, Some(extension)),
        },
        0x9a => new_instr!(opcode, prefixes, instr::control::call_ptr16_16),
        0x9c => new_instr!(opcode, prefixes, instr::flags::pushf),
        0x9d => new_instr!(opcode, prefixes, instr::flags::popf),
        0x9e => new_instr!(opcode, prefixes, instr::flags::sahf),
        0x9f => new_instr!(opcode, prefixes, instr::flags::lahf),
        0xa0 => new_instr!(opcode, prefixes, instr::transfer::mov_al_moffs8),
        0xaa => new_instr!(opcode, prefixes, instr::strings::stosb),
        0xab => new_instr!(opcode, prefixes, instr::strings::stosw),
        opcode @ 0xb0..=0xb7 => new_instr!(opcode, prefixes, instr::transfer::mov_r8_imm8),
        opcode @ 0xb8..=0xbf => new_instr!(opcode, prefixes, instr::transfer::mov_r16_imm16),
        0xc2 => new_instr!(opcode, prefixes, instr::control::ret_imm16_near),
        0xc3 => new_instr!(opcode, prefixes, instr::control::ret_near),
        0xc4 => new_instr!(opcode, prefixes, instr::transfer::les_r16_m16_16),
        0xc8 => new_instr!(opcode, prefixes, instr::control::enter_imm16_imm8),
        0xc9 => new_instr!(opcode, prefixes, instr::control::leave),
        0xca => new_instr!(opcode, prefixes, instr::control::ret_imm16_far),
        0xcb => new_instr!(opcode, prefixes, instr::control::ret_far),
        0xe0 => new_instr!(opcode, prefixes, instr::control::loopne_rel8),
        0xe1 => new_instr!(opcode, prefixes, instr::control::loope_rel8),
        0xe2 => new_instr!(opcode, prefixes, instr::control::loop_rel8),
        0xe3 => new_instr!(opcode, prefixes, instr::control::jcxz_rel8),
        0xe4 => new_instr!(opcode, prefixes, instr::ports::in_al_imm8),
        0xe5 => new_instr!(opcode, prefixes, instr::ports::in_ax_imm8),
        0xe6 => new_instr!(opcode, prefixes, instr::ports::out_imm8_al),
        0xe7 => new_instr!(opcode, prefixes, instr::ports::out_imm8_ax),
        0xe8 => new_instr!(opcode, prefixes, instr::control::call_rel16),
        0xe9 => new_instr!(opcode, prefixes, instr::control::jmp_rel16),
        0xea => new_instr!(opcode, prefixes, instr::control::jmp_ptr16_16),
        0xeb => new_instr!(opcode, prefixes, instr::control::jmp_rel8),
        0xec => new_instr!(opcode, prefixes, instr::ports::in_al_dx),
        0xed => new_instr!(opcode, prefixes, instr::ports::in_ax_dx),
        0xee => new_instr!(opcode, prefixes, instr::ports::out_dx_al),
        0xef => new_instr!(opcode, prefixes, instr::ports::out_dx_ax),
        0xf5 => new_instr!(opcode, prefixes, instr::flags::cmc),
        0xf8 => new_instr!(opcode, prefixes, instr::flags::clc),
        0xf9 => new_instr!(opcode, prefixes, instr::flags::stc),
        0xfa => new_instr!(opcode, prefixes, instr::flags::cli),
        0xfb => new_instr!(opcode, prefixes, instr::flags::sti),
        0xfc => new_instr!(opcode, prefixes, instr::flags::cld),
        0xfd => new_instr!(opcode, prefixes, instr::flags::std),
        opcode @ 0xff => match extension(sys) {
            2 => new_instr!(opcode, prefixes, instr::control::call_rm16),
            3 => new_instr!(opcode, prefixes, instr::control::call_m16_16),
            4 => new_instr!(opcode, prefixes, instr::control::jmp_rm16),
            5 => new_instr!(opcode, prefixes, instr::control::jmp_m16_16),
            6 => new_instr!(opcode, prefixes, instr::stack::push_m16),

            extension => invalid(sys, opcode, Some(extension)),
        },

        opcode => invalid(sys, opcode, None),
    }
}

pub fn decode(sys: &mut System) -> Instr {
    let mut prefixes = Prefixes::new();
    loop {
        match sys.read_mem_8() {
            0x26 => prefixes.segment = Es,
            0x2e => prefixes.segment = Cs,
            0x36 => prefixes.segment = Ss,
            0x3e => prefixes.segment = Ds,
            0xf3 => prefixes.rep = true,
            opcode => {
                print!("[{:#04x}] ", opcode);
                io::stdout().flush().unwrap();

                break match_opcode(sys, opcode, prefixes);
            }
        }
    }
}

fn extension(sys: &mut System) -> u8 {
    // TODO: Does every instruction with an extension use ModRM?
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
