use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Token};

struct HandlerNames {
    names: HashSet<Ident>,
}

impl Parse for HandlerNames {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let names = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        Ok(HandlerNames {
            names: names.into_iter().collect(),
        })
    }
}

pub fn io_ports_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as HandlerNames);

    let insert_calls = input.names.iter().map(|name| {
        let meta_name = format_ident!("{}_meta", name);

        quote! {
            let meta = self.#meta_name();
            handlers.insert(meta.port, meta.handler);
        }
    });

    let expanded = quote! {
        {
            let mut handlers = multimap::MultiMap::new();
            #(#insert_calls)*

            handlers
        }
    };

    expanded.into()
}
