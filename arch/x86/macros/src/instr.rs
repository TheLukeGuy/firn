use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use std::str::FromStr;
use strum_macros::EnumString;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, FnArg, ItemFn, Token, Type};

#[derive(Eq, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
enum Operand {
    SingleReg,

    #[strum(serialize = "imm8", serialize = "rel8")]
    Imm8,
    #[strum(
        serialize = "imm16",
        serialize = "rel16",
        serialize = "moffs8",
        serialize = "moffs16"
    )]
    Imm16,

    R8,
    R16,
    Sreg,

    M8,
    #[strum(serialize = "m16", serialize = "m16_16")]
    M16,

    Rm8,
    Rm16,

    Ptr16_16,
}

struct Instr {
    mnemonic: String,
    operands: Vec<Operand>,
}

impl Instr {
    fn is_single_reg(operand: &str) -> bool {
        let last = operand.to_lowercase().chars().last().unwrap();

        operand.len() == 2
            && operand.chars().all(char::is_alphabetic)
            && ['l', 'x', 's'].contains(&last)
    }
}

impl Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mnemonic = input.parse::<Ident>()?.to_string();
        let _ = input.parse::<Token![,]>();

        let mut operand_names = Vec::new();
        while !input.is_empty() {
            let mut name_tokens = TokenStream2::new();
            while !input.is_empty() {
                let tt = input.parse::<TokenTree>()?;
                match tt {
                    TokenTree::Punct(punct) if punct.as_char() == ',' => break,
                    tt => name_tokens.extend(Some(tt)),
                };
            }
            operand_names.push(name_tokens);
        }

        let mut operands = Vec::new();
        let mut operand_names_str = Vec::new();
        for operand in &operand_names {
            let operand_str = operand.to_string().replace(' ', "");
            operand_names_str.push(operand_str.clone());
            let operand_str = operand_str.replace(':', "_").replace('/', "");

            let operand = if Self::is_single_reg(&operand_str) {
                Operand::SingleReg
            } else {
                match Operand::from_str(&operand_str) {
                    Ok(operand) => operand,
                    Err(_) => return Err(Error::new(operand.span(), "invalid operand")),
                }
            };

            operands.push(operand);
        }

        let mnemonic = format!("{} {}", mnemonic, operand_names_str.join(", "));
        let mnemonic = mnemonic.trim().to_string();

        Ok(Instr { mnemonic, operands })
    }
}

pub fn instr_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let Instr { mnemonic, operands } = parse_macro_input!(args as Instr);
    let input = parse_macro_input!(input as ItemFn);

    let has_modrm_8 = operands.contains(&Operand::Rm8) || operands.contains(&Operand::M8);
    let has_modrm_16 = operands.contains(&Operand::Rm16) || operands.contains(&Operand::M16);
    let has_modrm = has_modrm_8 || has_modrm_16;

    let modrm_decode = if has_modrm_8 {
        quote! {
            let modrm = crate::ExtSystem::read_mem_8(sys);
            let modrm = crate::Modrm::decode(
                sys,
                modrm,
                Some(crate::ModrmRegType::ByteSized),
                crate::Size::Byte
            );
        }
    } else if has_modrm_16 {
        if operands.contains(&Operand::Sreg) {
            quote! {
                let modrm = crate::ExtSystem::read_mem_8(sys);
                let modrm = crate::Modrm::decode(
                    sys,
                    modrm,
                    Some(crate::ModrmRegType::Segment),
                    crate::Size::Word
                );
            }
        } else {
            quote! {
                let modrm = crate::ExtSystem::read_mem_8(sys);
                let modrm = crate::Modrm::decode(
                    sys,
                    modrm,
                    Some(crate::ModrmRegType::WordSized),
                    crate::Size::Word
                );
            }
        }
    } else {
        quote! {}
    };

    let mut operand_names = Vec::new();
    let operand_defs: Vec<_> = operands
        .iter()
        .map(|operand| match operand {
            Operand::SingleReg => vec![],

            Operand::Imm8 => vec![quote! {
                crate::ExtSystem::read_mem_8(sys)
            }],
            Operand::Imm16 => vec![quote! {
                crate::ExtSystem::read_mem_16(sys)
            }],

            Operand::R8 => vec![if has_modrm {
                quote! { modrm.byte_reg() }
            } else {
                quote! {
                    crate::GeneralByteReg::from_u8(opcode % 0o10)
                        .expect("invalid byte-sized register in opcode")
                }
            }],
            Operand::R16 => vec![if has_modrm {
                quote! { modrm.word_reg() }
            } else {
                quote! {
                    crate::GeneralWordReg::from_u8(opcode % 0o10)
                        .expect("invalid word-sized register in opcode")
                }
            }],
            Operand::Sreg => vec![quote! {
                modrm.segment_reg()
            }],

            Operand::M8 | Operand::M16 => vec![quote! {
                match modrm.reg_mem {
                    crate::RegMem::Ptr(ptr) => ptr,
                    crate::RegMem::Reg(_) => panic!("expected memory pointer in ModRM byte"),
                }
            }],

            Operand::Rm8 | Operand::Rm16 => vec![quote! {
                modrm.reg_mem
            }],

            Operand::Ptr16_16 => vec![
                quote! { crate::ExtSystem::read_mem_16(sys) },
                quote! { crate::ExtSystem::read_mem_16(sys) },
            ],
        })
        .filter(|defs| !defs.is_empty())
        .flatten()
        .enumerate()
        .map(|(index, def)| {
            let name = format_ident!("op{}", index);
            operand_names.push(name.clone());

            quote! {
                let #name = #def;
            }
        })
        .collect();

    let vis = &input.vis;

    let name = &input.sig.ident;
    let meta_name = format_ident!("{}_meta", name);

    let last_param = input.sig.inputs.last().unwrap();
    let takes_prefixes = match last_param {
        FnArg::Typed(typed) => match &*typed.ty {
            Type::Reference(reference) => match &*reference.elem {
                Type::Path(path) => path.path.segments.last().unwrap().ident == "Prefixes",
                _ => false,
            },
            _ => false,
        },
        _ => false,
    };

    let instr_call = if takes_prefixes {
        quote! {
            #name(sys, #(#operand_names,)* prefixes);
        }
    } else {
        quote! {
            #name(sys, #(#operand_names),*);
        }
    };

    let expanded = quote! {
        #vis fn #name(sys: &mut crate::System, opcode: u8, prefixes: &crate::Prefixes) {
            #input

            #modrm_decode
            #(#operand_defs)*

            #instr_call
        }

        #vis fn #meta_name() -> crate::InstrMeta {
            crate::InstrMeta {
                mnemonic: String::from(#mnemonic),
            }
        }
    };

    expanded.into()
}
