struct TemplateElementsBuilder {
    rpc_name: String,
    args: Option<syn::Item>,
    responses: Option<syn::Item>,
}
impl TemplateElementsBuilder {
    fn build(self) -> TemplateElements {
        TemplateElements {
            rpc_name: self.rpc_name,
            args: self.args.expect("Unininitialized??"),
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
    args: syn::Item,
    responses: syn::Item,
}
use proc_macro2::{Ident, Span};
impl TemplateElements {
    fn populate_method_template(self) -> proc_macro2::TokenStream {
        let rpc_name = Ident::new(&self.rpc_name, Span::call_site());
        let args = self.args;
        let responses = self.responses;
        let temp = interpolate_into_quote(rpc_name, args, responses);
        if &self.rpc_name == "getinfo" {
            println!("{}", &temp.to_string());
        }
        temp
    }
}
fn interpolate_into_quote(
    rpc_name: Ident,
    args: syn::Item,
    responses: syn::Item,
) -> proc_macro2::TokenStream {
    let argid = unpack_ident_from_element(&args);
    let responseid = unpack_ident_from_element(&responses);
    quote::quote! [
        fn #rpc_name(self, args: #argid)
            -> impl Future<Output = ResponseResult<#responseid>> {

        }
    ]
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
    for rpc_element in mod_contents {
        templatebuilder.update_if_response_or_args(rpc_element);
    }
    templatebuilder.build().populate_method_template()
}
pub(crate) fn generate_populated_templates() {
    let source = extract_response_idents();
    let syntax = syn::parse_file(&source).expect("Unable to parse file");
    for item in syntax.items {
        if let syn::Item::Mod(module) = item {
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

#[cfg(test)]
mod test {
    use super::*;
    mod format_from_tg_to_rpc_client {
        use super::*;
        #[ignore]
        #[test]
        fn getinfo_happy() {}
    }
}
