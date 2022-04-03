use proc_macro::TokenStream;

mod io_port;
mod io_ports;

#[proc_macro_attribute]
pub fn io_port(args: TokenStream, input: TokenStream) -> TokenStream {
    io_port::io_port_impl(args, input)
}

#[proc_macro]
pub fn io_ports(input: TokenStream) -> TokenStream {
    io_ports::io_ports_impl(input)
}
