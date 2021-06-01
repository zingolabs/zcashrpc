use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
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
        // Add assertions about shapes of tg interpretations
        let id = unpack_ident_from_element(&element);
        if id.to_string().rfind("Response").is_some() {
            self.responses = Some(element);
        } else if id.to_string().rfind("Arguments").is_some() {
            match element {
                syn::Item::Enum(ref argcontents) => {
                    validate_enum_arguments_shape(argcontents)
                }
                syn::Item::Struct(ref argcontents) => {
                    validate_struct_arguments_shape(argcontents)
                }
                _ => {
                    panic!()
                }
            }
            self.args = Some(element);
        }
    }
}
fn validate_struct_arguments_shape(argcontents: &syn::ItemStruct) {
    use quote::ToTokens;
    if let syn::Fields::Unnamed(fields) = &argcontents.fields {
        let unnamed_fields_len = fields.unnamed.len();
        assert!(
            (unnamed_fields_len > 0) && (unnamed_fields_len < 7),
            "Unnexpected number of arg fields in: {}",
            argcontents.to_token_stream().to_string(),
        );
    } else {
        panic!("Expected a tuple-like struct.");
    }
}
fn validate_enum_arguments_shape(argcontents: &syn::ItemEnum) {
    assert!(argcontents.variants.len() == 2);
    for variant in &argcontents.variants {
        if let syn::Fields::Unnamed(fields) = &variant.fields {
            assert!(fields.unnamed.len() == 1, "Unexpected unnamed field.");
        } else {
            panic!(
                "contained unexpected non-unnamed fields: {:#?}",
                variant.fields
            );
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
        interpolate_into_quote(rpc_name, args, responses)
    }
}
fn convert_tg_args_for_rpc_method(
    rpc_name: &Ident,
    args: Option<syn::Item>,
) -> (Option<TokenStream>, TokenStream) {
    if let Some(actualargs) = args {
        let argid = unpack_ident_from_element(&actualargs);
        let mut token_args = quote!(args);
        match actualargs {
            syn::Item::Struct(ref argcontents) => {
                if let syn::Fields::Unnamed(fields) = &argcontents.fields {
                    if fields.unnamed.len() == 1 {
                        token_args = quote!([#token_args]);
                    }
                } else {
                    panic!("Argument struct is not unnamed!");
                }
            }
            syn::Item::Enum(ref argcontents) => {
                validate_enum_arguments_shape(&argcontents);
                token_args = quote!([#token_args]);
            }
            _ => {
                panic!("Neither Struct nor Enum")
            }
        }
        (
            Some(quote!(args: rpc_types::#rpc_name::#argid)),
            quote!(Self::serialize_into_output_format(#token_args)),
        )
    } else {
        (None, quote!(Vec::new()))
    }
}
fn interpolate_into_quote(
    rpc_name: Ident,
    args: Option<syn::Item>,
    responses: syn::Item,
) -> proc_macro2::TokenStream {
    let responseid = unpack_ident_from_element(&responses);
    let rpc_name_string = rpc_name.to_string();
    let (args_quote, serialize_quote) =
        convert_tg_args_for_rpc_method(&rpc_name, args);
    quote!(
        pub fn #rpc_name(
            &mut self,
            #args_quote
        ) -> impl Future<
            Output = ResponseResult<rpc_types::#rpc_name::#responseid>,
        > {
            let args_for_make_request = #serialize_quote;
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
    use super::*;
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
    fn make_z_getnewaddress_mod_contents() -> [syn::Item; 2] {
        [
            syn::parse_quote!(
                #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
                pub struct ZGetnewaddressArguments(Option<String>);
            ),
            syn::parse_quote!(
                pub type ZGetnewaddressResponse = String;
            ),
        ]
    }
    mod convert_tg_args_for_rpc_method {
        use super::*;
        #[test]
        fn z_getnewaddress_some_case() {
            let expected_args_method_param = "args : rpc_types :: z_getnewaddress :: ZGetnewaddressArguments".to_string();
            let expected_serial_af_call =
                "Self :: serialize_into_output_format ([args])".to_string();

            let input_rpc_name_id =
                Ident::new("z_getnewaddress", Span::call_site());
            let input_args = make_z_getnewaddress_mod_contents()[0].clone();
            let (wrapped_observed_args_method_param, observed_serial_af_call) =
                convert_tg_args_for_rpc_method(
                    &input_rpc_name_id,
                    Some(input_args),
                );
            let observed_args_method_param =
                wrapped_observed_args_method_param.unwrap().to_string();
            let observed_serial_af_call = observed_serial_af_call.to_string();
            testutils::Comparator {
                expected: expected_args_method_param,
                observed: observed_args_method_param,
            }
            .compare();
            testutils::Comparator {
                expected: expected_serial_af_call,
                observed: observed_serial_af_call,
            }
            .compare();
        }
        #[test]
        fn getinfo_none_case() {
            let expected_serial_af_call = quote::quote!(Vec::new()).to_string();

            let input_rpc_name_id = Ident::new("getinfo", Span::call_site());
            let (observed_args_method_param, observed_serial_af_call) =
                convert_tg_args_for_rpc_method(&input_rpc_name_id, None);
            assert!(observed_args_method_param.is_none());
            let observed_serial_af_call = observed_serial_af_call.to_string();
            testutils::Comparator {
                expected: expected_serial_af_call,
                observed: observed_serial_af_call,
            }
            .compare();
        }
        fn get_getaddressdeltasarguments_tokens() -> Option<syn::Item> {
            Some(syn::parse_quote!(
                #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
                pub enum GetaddressdeltasArguments {
                    MultiAddress(Arg1),
                    Address(String),
                }
            ))
        }
        #[test]
        fn get_addressdeltas_enumarg_multiaddress_case() {
            let input_args = get_getaddressdeltasarguments_tokens();
            let input_rpc_name_id =
                Ident::new("getaddressdeltas", Span::call_site());
            let (args_params, serialize_argument) =
                convert_tg_args_for_rpc_method(&input_rpc_name_id, input_args);
            dbg!(serialize_argument.to_string());
            /*let expected_serialize_argument = quote::quote!(
                Self::serialize_into_output_format(
                    match
                    #token_args))
            );*/
        }
        #[ignore]
        #[test]
        fn get_addressdeltas_enumarg_address_case() {
            use rust_decimal::Decimal;
            #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
            pub struct Arg1 {
                pub chain_info: Option<bool>,
                pub end: Option<Decimal>,
                pub start: Option<Decimal>,
                pub addresses: Vec<String>,
            }
            todo!("BORKEN!");
            #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
            pub enum GetaddressdeltasArguments {
                MultiAddress(Arg1),
                Address(String),
            }
        }
        #[should_panic(expected = "Argument struct is not unnamed!")]
        #[test]
        fn named_field_argument_structure() {
            let input_args = syn::parse_quote!(
                struct BadField {
                    naughty: String,
                }
            );
            let input_rpc_name_id =
                Ident::new("NO_REAL_RPC", Span::call_site());

            convert_tg_args_for_rpc_method(
                &input_rpc_name_id,
                Some(input_args),
            );
        }
    }
    mod format_from_tg_to_rpc_client {
        use super::*;
        #[test]
        fn getinfo_happy_path() {
            let input_mod_contents =
                vec![syn::Item::Struct(get_getinfo_response())];
            let observed = format_from_tg_to_rpc_client(
                "getinfo".to_string(),
                input_mod_contents,
            )
            .to_string();
            #[rustfmt::skip]
            let expected = quote!(
                pub fn getinfo(
                    &mut self,
                ) -> impl Future<
                    Output = ResponseResult<
                        rpc_types::getinfo::GetinfoResponse
                    >,
                > {
                    let args_for_make_request = Vec::new();
                    self.make_request("getinfo", args_for_make_request)
                }
            )
            .to_string();
            testutils::Comparator { expected, observed }.compare();
        }
        #[test]
        fn z_getnewaddress() {
            //Create expected
            #[rustfmt::skip]
            let expected = quote!(
                pub fn z_getnewaddress(
                    &mut self,
                    args: rpc_types::z_getnewaddress::ZGetnewaddressArguments
                ) -> impl Future<
                    Output = ResponseResult<
                        rpc_types::z_getnewaddress::ZGetnewaddressResponse
                    >,
                > {
                    let args_for_make_request = Self::serialize_into_output_format([args]);
                    self.make_request("z_getnewaddress", args_for_make_request)
                }
            )
            .to_string();

            //Make observation
            let input_mod_contents = make_z_getnewaddress_mod_contents();
            let observed = format_from_tg_to_rpc_client(
                "z_getnewaddress".to_string(),
                input_mod_contents.to_vec(),
            )
            .to_string();
            testutils::Comparator { expected, observed }.compare();
        }
        #[ignore]
        #[test]
        fn z_mergetoaddress() {
            todo!("Exercise Option<arg> args.");
        }
    }
    mod interpolate_into_quote {
        use super::*;
        #[test]
        fn getinfo_happy_path() {
            //! Inputs to parse_quote are copied from earlier typegen outputs.
            let args_tokens = None;
            let response_tokens = syn::Item::Struct(get_getinfo_response());
            let rpc_name = Ident::new("getinfo", Span::call_site());
            let args = args_tokens;
            let responses = response_tokens;

            let observed =
                interpolate_into_quote(rpc_name, args, responses).to_string();
            #[rustfmt::skip]
            let expected = quote!(
                pub fn getinfo(
                    &mut self,
                ) -> impl Future<
                    Output = ResponseResult<
                        rpc_types::getinfo::GetinfoResponse
                    >,
                > {
                    let args_for_make_request = Vec::new();
                    self.make_request("getinfo", args_for_make_request)
                }
            )
            .to_string();
            testutils::Comparator { expected, observed }.compare();
        }
        #[test]
        fn z_getnewaddress() {
            let rpc_name = Ident::new("z_getnewaddress", Span::call_site());
            let [args, responses] = make_z_getnewaddress_mod_contents();
            let observed =
                interpolate_into_quote(rpc_name, Some(args), responses)
                    .to_string();
            #[rustfmt::skip]
            let expected =  quote!(
                pub fn z_getnewaddress(
                    &mut self,
                    args: rpc_types::z_getnewaddress::ZGetnewaddressArguments
                ) -> impl Future<
                    Output = ResponseResult<
                        rpc_types::z_getnewaddress::ZGetnewaddressResponse
                    >,
                > {
                    let args_for_make_request = Self::serialize_into_output_format([args]);
                    self.make_request("z_getnewaddress", args_for_make_request)
                }
            )
           .to_string();
            testutils::Comparator { expected, observed }.compare();
        }
        #[ignore]
        #[test]
        fn z_mergetoaddress() {
            todo!("Exercise Option<arg> args.");
        }
    }
}
