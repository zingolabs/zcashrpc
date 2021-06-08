mod typegen_interpreter;

use proc_macro::TokenStream;
use typegen_interpreter::TemplateElements;

#[proc_macro]
pub fn define_rpc_methods(_: TokenStream) -> TokenStream {
    typegen_interpreter::generate_rpc_interface(
        TemplateElements::interpolate_procedurecall_method,
    )
    .into()
}

#[proc_macro]
pub fn define_rpc_unittests(_: TokenStream) -> TokenStream {
    let tests = typegen_interpreter::generate_rpc_interface(
        TemplateElements::interpolate_procedurecall_unittest,
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
pub fn match_rpc(input: TokenStream) -> TokenStream {
    let match_arms = typegen_interpreter::generate_rpc_interface(
        TemplateElements::interpolate_zcashrcli_matcharms,
    );
    let name: syn::Ident = syn::parse(input).unwrap();
    quote::quote!(
        {
            use zcashrpc::client::ProcedureCall as _;
            match #name.as_str() {
                #match_arms
                _ => panic!("invalid rpc!"),
            }
        }
    )
    .into()
}
