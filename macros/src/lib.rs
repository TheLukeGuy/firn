mod io_port;

#[proc_macro_attribute]
pub fn io_port(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    io_port::io_port_impl(args, input)
}
