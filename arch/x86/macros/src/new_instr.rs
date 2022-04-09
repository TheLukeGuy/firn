use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Path, PathSegment, Token};

struct NewInstrArgs {
    opcode: Ident,
    prefixes: Ident,
    func: Path,
}

impl Parse for NewInstrArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let opcode = input.parse()?;
        input.parse::<Token![,]>()?;
        let prefixes = input.parse()?;
        input.parse::<Token![,]>()?;
        let func = input.parse()?;

        Ok(Self {
            opcode,
            prefixes,
            func,
        })
    }
}

pub fn new_instr_impl(input: TokenStream) -> TokenStream {
    let NewInstrArgs {
        opcode,
        prefixes,
        func,
    } = parse_macro_input!(input as NewInstrArgs);

    let mut meta_func = func.clone();
    let last_segment = meta_func.segments.last_mut().unwrap();
    let meta_name = format_ident!("{}_meta", last_segment.ident);
    *last_segment = PathSegment::from(meta_name);

    let expanded = quote! {
        crate::Instr::new(#opcode, #prefixes, #func, #meta_func)
    };

    expanded.into()
}
