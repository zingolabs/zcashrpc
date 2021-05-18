mod calls;
mod cli_commands;
mod responses;
mod utils;

use proc_macro::TokenStream;

fn build_collector(source: &str) -> utils::ClientMethodGenerator {
    let mut syntax = syn::parse_file(&source).expect("Unable to parse file");
    use syn::visit_mut::VisitMut;
    let mut responses = utils::ClientMethodGenerator { modules: vec![] };
    responses.visit_file_mut(&mut syntax);
    responses
}
#[proc_macro]
pub fn declare_all_rpc_methods(_: TokenStream) -> TokenStream {
    let src = utils::extract_response_idents();
    let module_asts = build_collector(&src);
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
pub fn declare_rpc_response_types(_input: TokenStream) -> TokenStream {
    responses::declare_rpc_response_types().into()
}

#[proc_macro]
pub fn declare_rcli_command_types(input: TokenStream) -> TokenStream {
    utils::make_code(input.into(), cli_commands::make_command).into()
}
