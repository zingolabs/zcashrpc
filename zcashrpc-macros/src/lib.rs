mod calls;
mod cli_commands;
mod responses;
mod utils;

use proc_macro::TokenStream;

fn build_collector(source: &str) -> utils::ResponseIdentCollector {
    let mut syntax = syn::parse_file(&source).expect("Unable to parse file");
    use syn::visit_mut::VisitMut;
    let mut responses = utils::ResponseIdentCollector {
        response_idents: vec![],
    };
    responses.visit_file_mut(&mut syntax);
    responses
}
#[proc_macro]
pub fn declare_all_rpc_methods(_: TokenStream) -> TokenStream {
    let src = utils::extract_response_idents();
    let responses = build_collector(&src);
    for response in responses.response_idents {
        let _rpcname = dbg!(response.to_string().trim_end_matches("Response"));
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
