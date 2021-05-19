mod calls;
mod cli_commands;
mod typegen_interpreter;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn declare_all_rpc_methods(_: TokenStream) -> TokenStream {
    typegen_interpreter::generate_populated_templates();
    quote::quote!("a").into()
}

#[proc_macro]
pub fn declare_rpc_client_methods(input: TokenStream) -> TokenStream {
    utils::make_code(input.into(), calls::make_call).into()
}

#[proc_macro]
pub fn declare_rcli_command_types(input: TokenStream) -> TokenStream {
    utils::make_code(input.into(), cli_commands::make_command).into()
}
