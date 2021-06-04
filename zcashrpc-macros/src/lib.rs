mod calls;
mod cli_commands;
mod typegen_interpreter;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn implement_rpc_call_methods(_: TokenStream) -> TokenStream {
    typegen_interpreter::generate_rpc_from_typegen_output(
        typegen_interpreter::TemplateElements::procedurecall_trait_method,
    )
    .into()
}

#[proc_macro]
pub fn implement_rpc_call_unittests(_: TokenStream) -> TokenStream {
    let tests = typegen_interpreter::generate_rpc_from_typegen_output(
        typegen_interpreter::TemplateElements::procedurecall_unittest,
    );
    quote::quote!(
        mod __procgen_unittests {
            use super::*;
            #tests
        }
    )
    .into()
}

#[proc_macro]
pub fn declare_rpc_client_methods(input: TokenStream) -> TokenStream {
    utils::make_code(input.into(), calls::make_call).into()
}

#[proc_macro]
pub fn declare_rcli_command_types(input: TokenStream) -> TokenStream {
    utils::make_code(input.into(), cli_commands::make_command).into()
}
