struct TemplateElementsBuilder {
    rpc_name: String,
    args: Option<syn::Item>,
    responses: Option<syn::Item>,
}
impl TemplateElementsBuilder {
    fn check_response_or_args(&mut self, element: syn::Item) {
        let id = unpack_ident_from_element(&element);
        if id.to_string().rfind("Response").is_some() {
            self.responses = Some(element);
        } else if id.to_string().rfind("Arguments").is_some() {
            self.args = Some(element);
        }
    }
    fn populate_method_template(self) -> proc_macro2::TokenStream {
        let rpc_name = self.rpc_name;
        match (self.args, self.responses) {
            //hooray for shadowing!
            (Some(args), Some(responses)) => {
                quote::quote! {
                    fn #rpc_name(self, #args) -> Wrapping(#responses) {

                    }
                }
            }
            (args, responses) => panic!(
                "{}args: {}\n  responses: {}",
                "Something missing: \n  ",
                args.is_none(),
                responses.is_none()
            ),
        }
    }
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
    contents: Vec<syn::Item>,
) -> proc_macro2::TokenStream {
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
    for rpc_element in contents {
        templatebuilder.check_response_or_args(rpc_element);
    }
    templatebuilder.populate_method_template()
}
pub(crate) fn generate_populated_templates() {
    let source = extract_response_idents();
    let syntax = syn::parse_file(&source).expect("Unable to parse file");
    for item in syntax.items {
        if let syn::Item::Mod(module) = item {
            println!();
            println!("rpc: {}", &module.ident.to_string());

            if let Some(c) = module.content {
                format_from_tg_to_rpc_client(module.ident.to_string(), c.1);
            }
        } else {
            panic!("Non module item in toplevel of typegen output.")
        }
    }
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
