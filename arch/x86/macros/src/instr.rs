use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use std::str::FromStr;
use strum_macros::EnumString;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Error, ItemFn, LitStr, Token};

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
    M16,

    Rm8,
    Rm16,

    Ptr16_16,
    M16_16,
}

struct Mnemonic {
    operands: Vec<Operand>,
}

impl Mnemonic {
    fn is_single_reg(operand: &str) -> bool {
        let last = operand.to_lowercase().chars().last().unwrap();

        operand.len() == 2
            && operand.chars().all(char::is_alphabetic)
            && ['l', 'x', 's'].contains(&last)
    }
}

impl Parse for Mnemonic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>()?;

        let mut operands = Vec::new();
        while !input.is_empty() {
            let mut tokens = TokenStream2::new();
            while !input.is_empty() {
                let tt = input.parse()?;
                match tt {
                    TokenTree::Punct(punct) if punct.as_char() == ',' => break,
                    tt => tokens.extend(Some(tt)),
                }
            }

            let operand_str = tokens
                .to_string()
                .replace(' ', "")
                .replace(':', "_")
                .replace('/', "");
            let operand = match Operand::from_str(&operand_str) {
                Ok(operand) => operand,
                Err(_) if Self::is_single_reg(&operand_str) => Operand::SingleReg,
                Err(_) => return Err(Error::new_spanned(tokens, "invalid operand")),
            };

            operands.push(operand);
        }

        Ok(Self { operands })
    }
}

struct Instr {
    mnemonic_str: LitStr,
    mnemonic: Mnemonic,

    rep: bool,
    rep_e: bool,
    rep_ne: bool,
}

impl Parse for Instr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mnemonic_str = input.parse::<LitStr>()?;
        let mnemonic = mnemonic_str.parse()?;

        let _ = input.parse::<Token![,]>();

        let mut rep = false;
        let mut rep_e = false;
        let mut rep_ne = false;
        for prefix in Punctuated::<Ident, Token![,]>::parse_terminated(input)? {
            let prefix_str = prefix.to_string().to_lowercase();
            match prefix_str.as_str() {
                "rep" => rep = true,
                "repe" => rep_e = true,
                "repne" => rep_ne = true,

                _ => return Err(Error::new_spanned(prefix, "invalid prefix")),
            }
        }

        Ok(Self {
            mnemonic_str,
            mnemonic,

            rep,
            rep_e,
            rep_ne,
        })
    }
}

enum ModrmReg {
    ByteSized,
    WordSized,
    Segment,
}

impl ModrmReg {
    fn to_token_stream(&self) -> TokenStream2 {
        match self {
            ModrmReg::ByteSized => quote! {
                ByteSized
            },
            ModrmReg::WordSized => quote! {
                WordSized
            },
            ModrmReg::Segment => quote! {
                Segment
            },
        }
    }
}

#[derive(Eq, PartialEq)]
enum ModrmRm {
    Byte,
    Word,
}

impl ModrmRm {
    fn to_token_stream(&self) -> TokenStream2 {
        match self {
            ModrmRm::Byte => quote! {
                Byte
            },
            ModrmRm::Word => quote! {
                Word
            },
        }
    }
}

fn match_operand(operand: &Operand) -> (Vec<TokenStream2>, Option<ModrmRm>) {
    let mut token_streams = Vec::new();

    let modrm = match operand {
        Operand::M8 | Operand::Rm8 => Some(ModrmRm::Byte),
        Operand::M16 | Operand::Rm16 | Operand::M16_16 => Some(ModrmRm::Word),

        _ => None,
    };

    match operand {
        Operand::SingleReg => (),
        Operand::Imm8 => token_streams.push(quote! {
            crate::ExtSystem::read_mem_8(sys)
        }),
        Operand::Imm16 => token_streams.push(quote! {
            crate::ExtSystem::read_mem_16(sys)
        }),
        Operand::R8 if matches!(modrm, None) => token_streams.push(quote! {
            crate::GeneralByteReg::from_u8(opcode % 0o10)
                .expect("invalid byte-sized register in opcode")
        }),
        Operand::R8 => token_streams.push(quote! {
            modrm.byte_reg()
        }),
        Operand::R16 if matches!(modrm, None) => token_streams.push(quote! {
            crate::GeneralWordReg::from_u8(opcode % 0o10)
                .expect("invalid word-sized register in opcode")
        }),
        Operand::R16 => token_streams.push(quote! {
            modrm.word_reg()
        }),
        Operand::Sreg => token_streams.push(quote! {
            modrm.segment_reg()
        }),
        Operand::M8 | Operand::M16 => token_streams.push(quote! {
            match modrm.reg_mem {
                crate::RegMem::Ptr(ptr) => ptr,
                crate::RegMem::Reg(_) => panic!("expected memory pointer in ModRM byte"),
            }
        }),
        Operand::Rm8 | Operand::Rm16 => token_streams.push(quote! {
            modrm.reg_mem
        }),
        Operand::Ptr16_16 => {
            for _ in 0..2 {
                token_streams.push(quote! {
                    crate::ExtSystem::read_mem_16(sys)
                });
            }
        }
        Operand::M16_16 => {
            token_streams.push(quote! {
                double_address.1
            });
            token_streams.push(quote! {
                double_address.0
            });
        }
    };

    (token_streams, modrm)
}

pub fn instr_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let Instr {
        mnemonic_str,
        mnemonic: Mnemonic { operands },
        rep,
        rep_e,
        rep_ne,
    } = parse_macro_input!(args as Instr);
    let input = parse_macro_input!(input as ItemFn);

    let mut operand_decodes = Vec::new();
    let mut modrm_rm = None;
    for operand in &operands {
        let (mut decodes, modrm) = match_operand(operand);
        operand_decodes.append(&mut decodes);

        if let Some(modrm) = modrm {
            match modrm_rm {
                Some(modrm_rm) if modrm_rm == modrm => {
                    return Error::new_spanned(mnemonic_str, "incompatible operands")
                        .into_compile_error()
                        .into()
                }
                _ => modrm_rm = Some(modrm),
            };
        }
    }

    let modrm_reg = match modrm_rm {
        Some(_) if operands.contains(&Operand::R8) => Some(ModrmReg::ByteSized),
        Some(_) if operands.contains(&Operand::R16) => Some(ModrmReg::WordSized),
        Some(_) if operands.contains(&Operand::Sreg) => Some(ModrmReg::Segment),

        _ => None,
    };

    let vis = &input.vis;
    let fn_name = &input.sig.ident;
    let meta_fn_name = format_ident!("{}_meta", fn_name);

    let modrm_decode = modrm_rm.map(|modrm_rm| {
        let rm_size = modrm_rm.to_token_stream();
        let reg_type = if let Some(reg_type) = modrm_reg {
            let reg_type = reg_type.to_token_stream();
            quote! {
                Some(crate::ModrmRegType::#reg_type)
            }
        } else {
            quote! {
                None
            }
        };

        quote! {
            let modrm = crate::ExtSystem::read_mem_8(sys);
            let modrm = crate::Modrm::decode(
                sys,
                modrm,
                #reg_type,
                crate::Size::#rm_size,
            );
        }
    });
    let modrm_decode = modrm_decode.iter();

    let double_address = if operands.contains(&Operand::M16_16) {
        Some(quote! {
            let double_address = match modrm.reg_mem {
                crate::RegMem::Ptr(ptr) => ptr.double_address(sys),
                _ => panic!("expected memory pointer in ModRM byte"),
            };
        })
    } else {
        None
    };
    let double_address = double_address.iter();

    let mut operand_names = Vec::new();
    let operand_decodes: Vec<_> = operand_decodes
        .iter()
        .enumerate()
        .map(|(index, decode)| {
            let name = format_ident!("op{}", index);
            operand_names.push(name.clone());

            quote! {
                let #name = #decode;
            }
        })
        .collect();

    let takes_prefixes = input.sig.inputs.len() > operand_names.len() + 1;
    let fn_call = if takes_prefixes {
        quote! {
            #fn_name(sys, #(#operand_names,)* prefixes);
        }
    } else {
        quote! {
            #fn_name(sys, #(#operand_names),*);
        }
    };

    let expanded = quote! {
        #vis fn #fn_name(sys: &mut crate::System, opcode: u8, prefixes: &crate::Prefixes) {
            #input

            #(#modrm_decode)*
            #(#double_address)*

            #(#operand_decodes)*

            #fn_call
        }

        #vis fn #meta_fn_name() -> crate::InstrMeta {
            crate::InstrMeta {
                mnemonic: String::from(#mnemonic_str),
            }
        }
    };

    expanded.into()
}
