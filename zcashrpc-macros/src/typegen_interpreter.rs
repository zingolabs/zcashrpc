use proc_macro2::{Ident, Span, TokenStream};
struct TemplateElementsBuilder {
    rpc_name: String,
    args: Option<syn::Item>,
    responses: Option<syn::Item>,
}
impl TemplateElementsBuilder {
    fn build(self) -> TemplateElements {
        TemplateElements {
            rpc_name: self.rpc_name,
            args: self.args,
            responses: self.responses.expect("Unininitialized??"),
        }
    }
    fn update_if_response_or_args(&mut self, element: syn::Item) {
        let id = unpack_ident_from_element(&element);
        if id.to_string().rfind("Response").is_some() {
            self.responses = Some(element);
        } else if id.to_string().rfind("Arguments").is_some() {
            self.args = Some(element);
        }
    }
}
struct TemplateElements {
    rpc_name: String,
    args: Option<syn::Item>,
    responses: syn::Item,
}
impl TemplateElements {
    fn populate_rpcmethod_template(self) -> TokenStream {
        let rpc_name = Ident::new(&self.rpc_name, Span::call_site());
        let args = self.args;
        let responses = self.responses;
        let temp = interpolate_into_quote(rpc_name, args, responses);
        temp
    }
}
#[allow(unused)]
fn interpolate_into_quote(
    rpc_name: Ident,
    args: syn::Item,
    responses: syn::Item,
) -> proc_macro2::TokenStream {
    let argid = unpack_ident_from_element(&args);
    let responseid = unpack_ident_from_element(&responses);
    let rpc_name_string = rpc_name.to_string();
    let args_quote = if argid {
        Some(quote::quote!(args: rpc_types::#rpc_name::#argid))
    } else {
        None
    };
    quote::quote!(
        pub fn #rpc_name(
            &mut self,
            #args_quote,
        ) -> impl Future<
            Output = ResponseResult<rpc_types::#rpc_name::#responseid>,
        > {
            let args_for_make_request =
                Self::serialize_into_output_format(args);
            self.make_request(#rpc_name_string, args_for_make_request)
        }
    )
}
fn unpack_ident_from_element(item: &syn::Item) -> &syn::Ident {
    use syn::Item;
    match item {
        Item::Struct(ref x) => &x.ident,
        Item::Enum(ref x) => &x.ident,
        Item::Type(ref x) => &x.ident,
        otherwise => {
            panic!("Expected Struct, Enum, or Type, found {:?}", otherwise)
        }
    }
}
fn format_from_tg_to_rpc_client(
    rpc_name: String,
    mod_contents: Vec<syn::Item>,
) -> TokenStream {
    //! Takes a typegen generated rpc definition, extracts elements:
    //!   rpc_name: Note the name is converted to a string, because the
    //!   originating span metadata isn't useful, and is potentially
    //!   problematic.
    //!            
    //!   arguments
    //!   responses
    let mut templatebuilder = TemplateElementsBuilder {
        rpc_name,
        args: None,
        responses: None,
    };
    for rpc_element in mod_contents {
        templatebuilder.update_if_response_or_args(rpc_element);
    }
    templatebuilder.build().populate_rpcmethod_template()
}
pub(crate) fn generate_populated_templates() -> TokenStream {
    let source = extract_response_idents();
    let syntax = syn::parse_file(&source).expect("Unable to parse file");
    let mut client_method_definitions = TokenStream::new();
    for item in syntax.items {
        if let syn::Item::Mod(module) = item {
            if let Some(c) = module.content {
                client_method_definitions.extend(format_from_tg_to_rpc_client(
                    module.ident.to_string(),
                    c.1,
                ));
            }
        } else {
            panic!("Non module item in toplevel of typegen output.")
        }
    }
    client_method_definitions
}
pub fn extract_response_idents() -> String {
    let pathstr =
        format!("{}/../src/client/rpc_types.rs", env!("CARGO_MANIFEST_DIR"));
    let raw_rs = std::path::Path::new(&pathstr);
    let mut src = String::new();
    let mut file = std::fs::File::open(&raw_rs).expect("Unable to open file");
    use std::io::Read as _;
    file.read_to_string(&mut src).expect("Unable to read file");
    src
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use std::fmt::Display;
    struct Comparator<T: PartialEq + Display> {
        expected: T,
        observed: T,
    }
    impl<T: PartialEq + Display> Comparator<T> {
        fn compare(self) {
            //! Our convention is that "expected" is "first"
            if self.expected != self.observed {
                panic!(
                    "\n===\nExpected:\n{}\nObserved:\n{}\n===\n",
                    self.expected, self.observed
                );
            }
        }
    }
    fn get_getinfo_response() -> syn::ItemStruct {
        syn::parse_quote!(
            #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
            pub struct GetinfoResponse {
                pub proxy: Option<String>,
                pub balance: rust_decimal::Decimal,
                pub blocks: rust_decimal::Decimal,
                pub connections: rust_decimal::Decimal,
                pub difficulty: rust_decimal::Decimal,
                pub errors: String,
                pub keypoololdest: rust_decimal::Decimal,
                pub keypoolsize: rust_decimal::Decimal,
                pub paytxfee: rust_decimal::Decimal,
                pub protocolversion: rust_decimal::Decimal,
                pub relayfee: rust_decimal::Decimal,
                pub testnet: bool,
                pub timeoffset: rust_decimal::Decimal,
                pub unlocked_until: rust_decimal::Decimal,
                pub version: rust_decimal::Decimal,
                pub walletversion: rust_decimal::Decimal,
            }
        )
    }
    fn get_getinfo_arguments() -> syn::ItemStruct {
        syn::parse_quote!(
            #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
            pub struct GetinfoArguments;
        )
    }
    use super::*;
    mod format_from_tg_to_rpc_client {
        //use super::*;
        #[ignore]
        #[test]
        fn getinfo_happy_path() {}
    }
    mod interpolate_into_quote {
        use super::*;
        #[test]
        fn getinfo_happy_path() {
            //! Inputs to parse_quote are copied from earlier typegen outputs.
            let args_tokens = syn::Item::Struct(get_getinfo_arguments());
            let response_tokens = syn::Item::Struct(get_getinfo_response());
            let rpc_name = Ident::new("getinfo", Span::call_site());
            let args = args_tokens;
            let responses = response_tokens;

            let observed =
                interpolate_into_quote(rpc_name, args, responses).to_string();
            #[rustfmt::skip]
            let expected = quote::quote!(
                fn getinfo(
                    self,
                    args: GetinfoArguments
                ) -> impl Future<Output = ResponseResult<GetinfoResponse>>
                {
                    this body in not really implemented haha!
                }
            ).to_string();
            Comparator { expected, observed }.compare();
        }
    }
}
