use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

struct ArithInstr {
    instr: Ident,
}

impl Parse for ArithInstr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let instr = input.parse()?;

        Ok(ArithInstr { instr })
    }
}

pub fn arith_instr_impl(input: TokenStream) -> TokenStream {
    let ArithInstr { instr } = parse_macro_input!(input as ArithInstr);

    let instr_lower = Ident::new(&instr.to_string().to_lowercase(), instr.span());
    let operation_8 = format_ident!("{}_8", instr_lower);
    let operation_16 = format_ident!("{}_16", instr_lower);

    let al_imm8_attr = format!("{} AL, imm8", instr);
    let ax_imm16_attr = format!("{} AX, imm16", instr);
    let rm8_imm8_attr = format!("{} r/m8, imm8", instr);
    let rm16_imm16_attr = format!("{} r/m16, imm16", instr);
    let rm16_imm8_attr = format!("{} r/m16, imm8", instr);
    let rm8_r8_attr = format!("{} r/m8, r8", instr);
    let rm16_r16_attr = format!("{} r/m8, r16", instr);
    let r8_rm8_attr = format!("{} r8, r/m8", instr);
    let r16_rm16_attr = format!("{} r16, r/m16", instr);

    let al_imm8 = format_ident!("{}_al_imm8", instr_lower);
    let ax_imm16 = format_ident!("{}_ax_imm16", instr_lower);
    let rm8_imm8 = format_ident!("{}_rm8_imm8", instr_lower);
    let rm16_imm16 = format_ident!("{}_rm16_imm16", instr_lower);
    let rm16_imm8 = format_ident!("{}_rm16_imm8", instr_lower);
    let rm8_r8 = format_ident!("{}_rm8_r8", instr_lower);
    let rm16_r16 = format_ident!("{}_rm16_r16", instr_lower);
    let r8_rm8 = format_ident!("{}_r8_rm8", instr_lower);
    let r16_rm16 = format_ident!("{}_r16_rm16", instr_lower);

    let expanded = quote! {
        #[firn_arch_x86_macros::instr(#al_imm8_attr)]
        pub fn #al_imm8(sys: &mut crate::System, imm: u8) {
            let old = sys.cpu.reg_8(crate::GeneralByteReg::Al);
            let value = #operation_8(sys, old, imm);
            sys.cpu.set_reg_8(crate::GeneralByteReg::Al, value);
        }

        #[firn_arch_x86_macros::instr(#ax_imm16_attr)]
        pub fn #ax_imm16(sys: &mut crate::System, imm: u16) {
            let old = sys.cpu.reg_16(crate::GeneralWordReg::Ax.into());
            let value = #operation_16(sys, old, imm);
            sys.cpu.set_reg_16(crate::GeneralWordReg::Ax.into(), value);
        }

        #[firn_arch_x86_macros::instr(#rm8_imm8_attr)]
        pub fn #rm8_imm8(sys: &mut crate::System, rm: crate::RegMem, imm: u8) {
            let old = rm.get_8(sys);
            let value = #operation_8(sys, old, imm);
            rm.set_8(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_imm16_attr)]
        pub fn #rm16_imm16(sys: &mut crate::System, rm: crate::RegMem, imm: u16) {
            let old = rm.get_16(sys);
            let value = #operation_16(sys, old, imm);
            rm.set_16(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_imm8_attr)]
        pub fn #rm16_imm8(sys: &mut crate::System, rm: crate::RegMem, imm: u8) {
            let old = rm.get_16(sys);
            let value = #operation_16(sys, old, imm as u16);
            rm.set_16(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm8_r8_attr)]
        pub fn #rm8_r8(sys: &mut crate::System, rm: crate::RegMem, reg: crate::GeneralByteReg) {
            let old = rm.get_8(sys);
            let reg = sys.cpu.reg_8(reg);
            let value = #operation_8(sys, old, reg);
            rm.set_8(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_r16_attr)]
        pub fn #rm16_r16(sys: &mut crate::System, rm: crate::RegMem, reg: crate::GeneralWordReg) {
            let old = rm.get_16(sys);
            let reg = sys.cpu.reg_16(reg.into());
            let value = #operation_16(sys, old, reg);
            rm.set_16(sys, value);
        }

        #[firn_arch_x86_macros::instr(#r8_rm8_attr)]
        pub fn #r8_rm8(sys: &mut crate::System, reg: crate::GeneralByteReg, rm: crate::RegMem) {
            let old = sys.cpu.reg_8(reg);
            let rm = rm.get_8(sys);
            let value = #operation_8(sys, old, rm);
            sys.cpu.set_reg_8(reg, value);
        }

        #[firn_arch_x86_macros::instr(#r16_rm16_attr)]
        pub fn #r16_rm16(sys: &mut crate::System, reg: crate::GeneralWordReg, rm: crate::RegMem) {
            let old = sys.cpu.reg_16(reg.into());
            let rm = rm.get_16(sys);
            let value = #operation_16(sys, old, rm);
            sys.cpu.set_reg_16(reg.into(), value);
        }
    };

    expanded.into()
}
