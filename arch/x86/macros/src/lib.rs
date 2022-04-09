use proc_macro::TokenStream;

mod instr;
mod new_instr;

#[proc_macro_attribute]
pub fn instr(args: TokenStream, input: TokenStream) -> TokenStream {
    instr::instr_impl(args, input)
}

#[proc_macro]
pub fn new_instr(input: TokenStream) -> TokenStream {
    new_instr::new_instr_impl(input)
}
