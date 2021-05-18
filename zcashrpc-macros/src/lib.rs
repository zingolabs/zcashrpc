mod calls;
mod cli_commands;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn declare_all_rpc_methods(_: TokenStream) -> TokenStream {
    let module_asts = utils::create_methodgenerator();
    for modast in module_asts.modules {
        let _rpcname =
            dbg!(modast.ident.to_string().trim_end_matches("Response"));
    }
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
