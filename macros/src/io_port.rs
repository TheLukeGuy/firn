use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, Expr, FnArg, ItemFn, ReturnType, Signature, Type};

fn io_port_variant(sig: &Signature) -> syn::Result<Ident> {
    fn err(span: Span, message: &str) -> syn::Result<Ident> {
        Err(Error::new(span, message))
    }

    enum Direction {
        In,
        Out,
    }

    let params = &sig.inputs;
    let (span, value_type, direction) = match params.len() {
        1 => match &sig.output {
            ReturnType::Type(_, output) => (output.span(), Some(&**output), Direction::In),
            output => (output.span(), None, Direction::In),
        },
        2 => match &params[1] {
            FnArg::Typed(param) => (param.span(), Some(&*param.ty), Direction::Out),
            param => (param.span(), None, Direction::Out),
        },
        _ => return err(params.span(), "expected 1 or 2 parameters"),
    };

    let ident = match value_type {
        Some(Type::Path(path)) => {
            let segments = &path.path.segments;
            let ident = &segments[0].ident;

            ident.to_string()
        }
        _ => return err(span, "expected u8 or u16"),
    };

    let variant = match (direction, ident.as_str()) {
        (Direction::In, "u8") => "In8",
        (Direction::In, "u16") => "In16",
        (Direction::Out, "u8") => "Out8",
        (Direction::Out, "u16") => "Out16",
        _ => return err(span, "expected u8 or u16"),
    };

    Ok(format_ident!("{}", variant))
}

pub fn io_port_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let port: Expr = match syn::parse(args) {
        Ok(port) => port,
        Err(err) => return err.into_compile_error().into(),
    };

    let input = parse_macro_input!(input as ItemFn);
    let vis = &input.vis;

    let variant = match io_port_variant(&input.sig) {
        Ok(handler) => handler,
        Err(err) => return err.into_compile_error().into(),
    };

    let name = &input.sig.ident;
    let meta_name = format_ident!("{}_meta", name);

    let expanded = quote! {
        #input

        #vis fn #meta_name(&self) -> firn_core::device::IoPortWrapper<Self> {
            firn_core::device::IoPortWrapper {
                port: #port,
                handler: firn_core::device::IoPortHandler::#variant(Self::#name),
            }
        }
    };

    expanded.into()
}
