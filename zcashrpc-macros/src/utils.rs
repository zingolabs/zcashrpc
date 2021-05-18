pub fn make_code(
    input: proc_macro2::TokenStream,
    map_fn: fn(
        (proc_macro2::Ident, proc_macro2::Group),
    ) -> proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let code_vec: Vec<proc_macro2::TokenStream> =
        conjoin_calls_with_args(input.into_iter(), Vec::new())
            .into_iter()
            .map(map_fn)
            .collect();
    quote::quote!(#(#code_vec)*)
}

type CallsWithArgs = Vec<(proc_macro2::Ident, proc_macro2::Group)>;

fn conjoin_calls_with_args(
    mut iter: proc_macro2::token_stream::IntoIter,
    mut vec: CallsWithArgs,
) -> CallsWithArgs {
    match iter.next() {
        Some(proc_macro2::TokenTree::Ident(i)) => {
            vec.push((
                i,
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    proc_macro2::TokenStream::new(),
                ),
            ));
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Group(n)) => {
            let ident = vec.last().unwrap().0.clone();
            vec.pop();
            vec.push((ident, n));
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Punct(_)) => {
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Literal(l)) => {
            panic!("Unexpected literal '{}' in macro input.", l)
        }
        None => vec,
    }
}

enum NextIdent {
    Arg,
    Type,
}

fn strip_types(
    mut iter: proc_macro2::token_stream::IntoIter,
    mut vec: Vec<proc_macro2::TokenTree>,
    next_ident: NextIdent,
) -> proc_macro2::TokenStream {
    match iter.next() {
        Some(proc_macro2::TokenTree::Ident(i)) => match next_ident {
            NextIdent::Arg => {
                vec.push(i.into());
                strip_types(iter, vec, NextIdent::Type)
            }
            NextIdent::Type => strip_types(iter, vec, NextIdent::Arg),
        },
        Some(_) => strip_types(iter, vec, next_ident),
        None => vec.into_iter().collect(),
    }
}

type SnakeCase = proc_macro2::Ident;
type CamelCaseStruct = proc_macro2::Ident;
type TypedArgs = proc_macro2::TokenStream;
type UntypedArgs = proc_macro2::TokenStream;

pub fn format_input(
    suffix: &str,
    input: (proc_macro2::Ident, proc_macro2::Group),
) -> (SnakeCase, CamelCaseStruct, TypedArgs, UntypedArgs) {
    let (ident, params) = input;
    let mut call_ident_string = ident.to_string().to_lowercase();
    if call_ident_string.starts_with('z') {
        call_ident_string.insert(1, '_');
    }
    let call_ident = proc_macro2::Ident::new(&call_ident_string, ident.span());
    let response_ident = proc_macro2::Ident::new(
        &format!("{}{}", ident.to_string(), suffix),
        ident.span(),
    );
    let param_stream = params.stream();
    let arg_id_stream: proc_macro2::TokenStream =
        strip_types(params.stream().into_iter(), Vec::new(), NextIdent::Arg);
    (call_ident, response_ident, param_stream, arg_id_stream)
}

enum ArgumentTokens {
    Struct,
    Enum,
}
enum ResponseTokens {
    Struct,
    Enum,
    Alias,
}

fn f(rpc_element: syn::Item) -> (ArgumentTokens, ResponseTokens) {
    if let syn::Item::Struct(structitem) = rpc_element {
        println!("struct is: {}", &structitem.ident.to_string());
    } else if let syn::Item::Type(typeitem) = rpc_element {
        println!("typeitem is: {}", &typeitem.ident.to_string());
    } else if let syn::Item::Enum(enumitem) = rpc_element {
        println!("enumitem is: {}", &enumitem.ident.to_string());
    }
    (ArgumentTokens::Struct, ResponseTokens::Struct)
}
pub(crate) fn create_methodgenerator() -> ClientMethodGenerator {
    let source = extract_response_idents();
    let syntax = syn::parse_file(&source).expect("Unable to parse file");
    for i in syntax.items {
        if let syn::Item::Mod(m) = i {
            println!();
            println!("rpc: {}", &m.ident.to_string());
            if let Some(c) = m.content {
                for item in c.1 {
                    let rpc_tokens = f(item);
                }
            }
        }
    }
    //let mut reqresponse_types = ClientMethodGenerator { rpc: syntax };
    //reqresponse_types.visit_file_mut(&mut syntax);
    //reqresponse_types
    ClientMethodGenerator {}
}

use syn::visit::Visit;
#[derive(Debug)]
pub(crate) struct ClientMethodGenerator {}
impl Visit<'_> for ClientMethodGenerator {
    fn visit_item_mod(&mut self, module: &syn::ItemMod) {
        syn::visit::visit_item_mod(self, module);
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
