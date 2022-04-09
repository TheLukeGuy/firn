use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::str::FromStr;
use strum_macros::EnumString;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Error, ItemFn, Token};

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
enum Operand {
    Al,
    Ax,
    Dx,

    Imm8,
    Imm16,
}

struct Instr {
    operands: Vec<Operand>,
}

impl Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>()?;

        let parsed = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let mut operands = Vec::new();
        for operand in parsed {
            let operand = match Operand::from_str(&operand.to_string()) {
                Ok(operand) => operand,
                Err(_) => return Err(Error::new(operand.span(), "invalid operand")),
            };

            operands.push(operand);
        }

        Ok(Instr { operands })
    }
}

pub fn instr_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let Instr { operands } = parse_macro_input!(args as Instr);
    let input = parse_macro_input!(input as ItemFn);

    let vis = &input.vis;
    let name = &input.sig.ident;

    let expanded = quote! {
        #vis fn #name(sys: &mut firn_core::System) {
            #input

            #name(sys);
        }
    };

    expanded.into()
}
