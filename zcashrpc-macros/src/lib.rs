mod calls;
mod cli_commands;
mod typegen_interpreter;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn define_rpc_methods(_: TokenStream) -> TokenStream {
    typegen_interpreter::generate_rpc_interface(
        typegen_interpreter::TemplateElements::procedurecall_trait_method,
    )
    .into()
}

#[proc_macro]
pub fn define_rpc_unittests(_: TokenStream) -> TokenStream {
    let tests = typegen_interpreter::generate_rpc_interface(
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
