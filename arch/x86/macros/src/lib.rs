use proc_macro::TokenStream;

mod instr;

#[proc_macro_attribute]
pub fn instr(args: TokenStream, input: TokenStream) -> TokenStream {
    instr::instr_impl(args, input)
}
