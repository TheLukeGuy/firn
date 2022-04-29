use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub fn shift_instr_impl(input: TokenStream) -> TokenStream {
    let instr = parse_macro_input!(input as Ident);

    let instr_lower = Ident::new(&instr.to_string().to_lowercase(), instr.span());
    let operation_8 = format_ident!("{}_8", instr_lower);
    let operation_16 = format_ident!("{}_16", instr_lower);

    let rm8_1_attr = format!("{} r/m8, 1", instr);
    let rm8_cl_attr = format!("{} r/m8, CL", instr);
    let rm8_imm8_attr = format!("{} r/m8, imm8", instr);
    let rm16_1_attr = format!("{} r/m16, 1", instr);
    let rm16_cl_attr = format!("{} r/m16, CL", instr);
    let rm16_imm8_attr = format!("{} r/m16, imm8", instr);

    let rm8_1 = format_ident!("{}_rm8_1", instr_lower);
    let rm8_cl = format_ident!("{}_rm8_cl", instr_lower);
    let rm8_imm8 = format_ident!("{}_rm8_imm8", instr_lower);
    let rm16_1 = format_ident!("{}_rm16_1", instr_lower);
    let rm16_cl = format_ident!("{}_rm16_cl", instr_lower);
    let rm16_imm8 = format_ident!("{}_rm16_imm8", instr_lower);

    let expanded = quote! {
        #[firn_arch_x86_macros::instr(#rm8_1_attr)]
        pub fn #rm8_1(sys: &mut crate::System, rm: crate::RegMem) {
            let old = rm.get_8(sys);
            let value = crate::arith::#operation_8(sys, old, 1);
            rm.set_8(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm8_cl_attr)]
        pub fn #rm8_cl(sys: &mut crate::System, rm: crate::RegMem) {
            let old = rm.get_8(sys);
            let reg = sys.cpu.reg_8(crate::GeneralByteReg::Cl);
            let value = crate::arith::#operation_8(sys, old, reg);
            rm.set_8(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm8_imm8_attr)]
        pub fn #rm8_imm8(sys: &mut crate::System, rm: crate::RegMem, imm: u8) {
            let old = rm.get_8(sys);
            let value = crate::arith::#operation_8(sys, old, imm);
            rm.set_8(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_1_attr)]
        pub fn #rm16_1(sys: &mut crate::System, rm: crate::RegMem) {
            let old = rm.get_16(sys);
            let value = crate::arith::#operation_16(sys, old, 1);
            rm.set_16(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_cl_attr)]
        pub fn #rm16_cl(sys: &mut crate::System, rm: crate::RegMem) {
            let old = rm.get_16(sys);
            let reg = sys.cpu.reg_8(crate::GeneralByteReg::Cl);
            let value = crate::arith::#operation_16(sys, old, reg);
            rm.set_16(sys, value);
        }

        #[firn_arch_x86_macros::instr(#rm16_imm8_attr)]
        pub fn #rm16_imm8(sys: &mut crate::System, rm: crate::RegMem, imm: u8) {
            let old = rm.get_16(sys);
            let value = crate::arith::#operation_16(sys, old, imm);
            rm.set_16(sys, value);
        }
    };

    expanded.into()
}
