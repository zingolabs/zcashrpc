//! In order to leverage all of Rust's type safety, this crate produces
//! a set of concrete Rust types for responses from the zcashd-RPC interface.

mod error;
mod special_cases;
mod tokenize;
use error::TypegenResult;
use proc_macro2::TokenStream;
use quote::quote;

/// Process quizface-formatted response specifications from files, producing
/// Rust types, in the `rpc_response_types.rs` file.
fn main() {
    let initial_comment = r#"//procedurally generated response types, note that zcashrpc-typegen
           //is in early alpha, and output is subject to change at any time.
"#;
    use std::io::Write as _;
    std::fs::write(output_path(), initial_comment).unwrap();
    let mut iter = std::fs::read_dir(&std::path::Path::new(
        &std::env::args()
            .nth(1)
            .unwrap_or("./example_dir".to_string()),
    ))
    .unwrap()
    .map(Result::unwrap)
    .collect::<Vec<std::fs::DirEntry>>();
    iter.sort_unstable_by(|file_node1, file_node2| {
        file_node1.path().cmp(&file_node2.path())
    });
    for filenode in iter {
        match process_response(&filenode.path()) {
            Ok(code) => {
                let mut outfile = std::fs::OpenOptions::new()
                    .append(true)
                    .open(output_path())
                    .unwrap();
                outfile.write_all(code.to_string().as_bytes()).unwrap();
                assert!(std::process::Command::new("rustfmt")
                    .arg(output_path())
                    .output()
                    .unwrap()
                    .status
                    .success());
            }
            Err(error::TypegenError::Annotation(err))
                if err.kind == error::InvalidAnnotationKind::Insufficient =>
            {
                ()
            }
            _ => todo!("Holy moly something is messed up!"),
        }
    }
}

fn under_to_camel(name: &str) -> String {
    name.split('_').map(|x| capitalize_first_char(x)).collect()
}

fn camel_to_under(name: &str) -> String {
    name.chars()
        .fold(vec![String::new()], |mut v, c| {
            if c.is_ascii_uppercase() {
                v.push(c.to_ascii_lowercase().to_string());
                v
            } else {
                let end = v.len() - 1;
                v[end].push(c);
                v
            }
        })
        .into_iter()
        .skip_while(String::is_empty)
        .collect::<Vec<String>>()
        .join("_")
}

fn process_response(file: &std::path::Path) -> TypegenResult<TokenStream> {
    let (file_name, file_body) = get_data(file);
    let mod_name = callsite_ident(&if special_cases::RESERVED_KEYWORDS
        .contains(&file_name.as_ref())
    {
        format!("{}_mod", &file_name)
    } else {
        file_name.clone()
    });
    let type_name =
        [under_to_camel(&file_name), "Response".to_string()].concat();
    let mut output = match file_body {
        serde_json::Value::Array(mut vec) => match vec.len() {
            0 => emptygen(&type_name),
            1 => match vec.pop().unwrap() {
                serde_json::Value::Object(obj) => {
                    structgen(obj, &type_name).map(|x| x.1)?
                }
                val => alias(val, &type_name)?,
            },
            _ => enumgen(vec, &type_name)?,
        },
        non_array => {
            panic!("Received {}, expected array", non_array.to_string())
        }
    };

    output.sort_by(|ts1, ts2| ts1.to_string().cmp(&ts2.to_string()));
    output.dedup_by(|ts1, ts2| ts1.to_string() == ts2.to_string());
    Ok(quote::quote!(pub mod #mod_name { #(#output)* }))
}

const VARIANT_NAMES: &[&str] = &["Regular", "Verbose", "VeryVerbose"];

fn get_data(file: &std::path::Path) -> (String, serde_json::Value) {
    let file_body =
        from_file_deserialize(&file).expect("Couldn't unpack file!");
    (
        file.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".json")
            .unwrap()
            .to_string(),
        file_body,
    )
}

/// This function provides input for the OS interface that we access via
/// std::process, and std::fs.
const TYPEGEN_VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn output_path() -> std::ffi::OsString {
    use std::ffi::OsString;
    let in_version = std::fs::read_dir("../../quizface/output/")
        .expect("Missing interpretations.")
        .map(|x| x.unwrap().file_name())
        .collect::<Vec<OsString>>()
        .pop()
        .expect("Can't retrieve input dir name.");
    let outstring = format!(
        "./output/{}_{}/rpc_response_types.rs",
        in_version
            .into_string()
            .expect("Couldn't get String from OsString."),
        TYPEGEN_VERSION
    );
    let outpath = std::path::Path::new(&outstring);
    std::fs::create_dir_all(outpath.parent().expect("Couldn't create parent."))
        .expect("Couldn't create outdir.");
    std::ffi::OsString::from(
        std::env::args()
            .nth(2)
            .unwrap_or(outpath.to_str().unwrap().into()),
    )
}

/// Handles data access from fs location through deserialization
fn from_file_deserialize(
    file_path: &std::path::Path,
) -> TypegenResult<serde_json::Value> {
    let from_io_to_fs = error::FSError::from_io_error(file_path);
    let mut file = std::fs::File::open(file_path).map_err(&from_io_to_fs)?;
    let mut file_body = String::new();
    use std::io::Read as _;
    file.read_to_string(&mut file_body)
        .map_err(&from_io_to_fs)?;
    let file_body_json =
        serde_json::de::from_str(&file_body).map_err(|err| {
            error::JsonError::from_serde_json_error(err, file_body)
        })?;
    Ok(file_body_json)
}

/// Simple wrapper that always generates Idents with "call_site" spans.
fn callsite_ident(name: &str) -> proc_macro2::Ident {
    proc_macro2::Ident::new(name, proc_macro2::Span::call_site())
}

fn handle_options_and_keywords(
    serde_rename: &mut Option<TokenStream>,
    field_name: &mut String,
    option: &mut bool,
) -> () {
    if special_cases::RESERVED_KEYWORDS.contains(&field_name.as_str()) {
        *serde_rename = Some(
            format!("#[serde(rename = \"{}\")]", &field_name)
                .parse()
                .unwrap(),
        );
        field_name.push_str("_field");
    }

    if field_name.starts_with("Option<") {
        *field_name = field_name
            .trim_end_matches(">")
            .trim_start_matches("Option<")
            .to_string();
        *option = true;
    }
}

fn enumgen(
    inner_nodes: Vec<serde_json::Value>,
    //This one layer out from the map that's passed to structgen!
    //Don't let the identical type signatures fool you.
    enum_name: &str,
) -> TypegenResult<Vec<TokenStream>> {
    assert!(inner_nodes.len() <= VARIANT_NAMES.len());
    let mut inner_structs = Vec::new();
    let ident = callsite_ident(enum_name);
    let enum_code: Vec<TokenStream> = inner_nodes
        .into_iter()
        .zip(VARIANT_NAMES.iter())
        .map(|(value, variant_name)| {
            let variant_name = capitalize_first_char(&variant_name);
            let variant_name_tokens = callsite_ident(&variant_name);
            match value {
                serde_json::Value::Object(obj) => {
                    let field_data = handle_fields(enum_name, obj)?;
                    inner_structs.extend(field_data.inner_structs);
                    match field_data.case {
                        special_cases::Case::Regular => {
                            let variant_body_tokens =
                                field_data.ident_val_tokens;
                            Ok(quote!(
                            #variant_name_tokens {
                                #(#variant_body_tokens)*
                            },))
                        }
                        other_case => unimplemented!(
                            "Hit special case {:?} in enumgen",
                            other_case
                        ),
                    }
                }
                non_object => {
                    let (variant_body_tokens, new_structs, _terminal_enum) =
                        tokenize::value(&variant_name, non_object)?;
                    inner_structs.extend(new_structs);
                    Ok(quote!(#variant_name_tokens(#variant_body_tokens),))
                }
            }
        })
        .collect::<TypegenResult<Vec<TokenStream>>>()?;
    inner_structs.push(quote!(
            #[derive(Debug, serde::Deserialize, serde::Serialize)]
            pub enum #ident {
                #(#enum_code)*
            }
    ));
    Ok(inner_structs)
}

fn structgen(
    inner_nodes: serde_json::Map<String, serde_json::Value>,
    struct_name: &str,
) -> TypegenResult<(special_cases::Case, Vec<TokenStream>)> {
    let ident = callsite_ident(struct_name);
    let field_data = handle_fields(struct_name, inner_nodes)?;
    let mut ident_val_tokens = field_data.ident_val_tokens;
    let body = match field_data.case {
        special_cases::Case::Regular => {
            add_pub_keywords(&mut ident_val_tokens);
            quote!(
                pub struct #ident {
                    #(#ident_val_tokens)*
                }
            )
        }
        special_cases::Case::FourXs => {
            return Ok((special_cases::Case::FourXs, field_data.inner_structs));
        }
    };

    let mut generated_code = vec![quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        #body
    )];
    generated_code.extend(field_data.inner_structs);
    Ok((special_cases::Case::Regular, generated_code))
}

fn emptygen(struct_name: &str) -> Vec<TokenStream> {
    let ident = callsite_ident(struct_name);
    vec![quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #ident;
    )]
}

fn add_pub_keywords(tokens: &mut Vec<TokenStream>) {
    *tokens = tokens
        .into_iter()
        .map(|ts| match ts.clone().into_iter().next() {
            None | Some(proc_macro2::TokenTree::Punct(_)) => ts.clone(),
            _ => quote!(pub #ts),
        })
        .collect();
}

struct FieldsInfo {
    case: special_cases::Case,
    ident_val_tokens: Vec<TokenStream>,
    inner_structs: Vec<TokenStream>,
}
fn handle_fields(
    struct_name: &str,
    inner_nodes: serde_json::Map<String, serde_json::Value>,
) -> TypegenResult<FieldsInfo> {
    let mut ident_val_tokens: Vec<TokenStream> = Vec::new();
    let mut inner_structs = Vec::new();
    let mut case = special_cases::Case::Regular;
    for (mut field_name, val) in inner_nodes {
        //special case handling
        if &field_name == "xxxx" {
            inner_structs = tokenize::value(struct_name, val)?.1; // .0 unused
            case = special_cases::Case::FourXs;
            break;
        }

        let mut serde_rename = None;
        let mut option = false;
        handle_options_and_keywords(
            &mut serde_rename,
            &mut field_name,
            &mut option,
        );
        field_name = camel_to_under(&field_name);

        //temp_acc needed because destructuring assignments are unstable
        //see https://github.com/rust-lang/rust/issues/71126 for more info
        let (mut tokenized_val, new_struct, _terminal_enum) =
            tokenize::value(&under_to_camel(&field_name), val)?;
        inner_structs.extend(new_struct);
        if option {
            use std::str::FromStr as _;
            tokenized_val =
                TokenStream::from_str(&format!("Option<{}>", tokenized_val))
                    .unwrap();
        }

        let token_ident = callsite_ident(&field_name);
        ident_val_tokens.push(quote!(#serde_rename));
        ident_val_tokens.push(quote!(#token_ident: #tokenized_val,));
    }
    Ok(FieldsInfo {
        case,
        inner_structs,
        ident_val_tokens,
    })
}

fn alias(
    data: serde_json::Value,
    name: &str,
) -> TypegenResult<Vec<TokenStream>> {
    let ident = callsite_ident(&name);
    let (type_body, mut inner_structs, terminal_enum) = tokenize::value(
        &capitalize_first_char(name.trim_end_matches("Response")),
        data,
    )?;
    if !terminal_enum {
        let aliased = quote!(
            pub type #ident = #type_body;
        );
        inner_structs.push(aliased);
    }
    Ok(inner_structs)
}

fn capitalize_first_char(input: &str) -> String {
    let mut ret = input.to_string();
    let ch = ret.remove(0);
    ret.insert(0, ch.to_ascii_uppercase());
    ret
}

#[cfg(test)]
mod unit {
    mod atomic {
        use crate::*;
        #[test]
        fn tokenize_value_string() {
            let quoted_string = tokenize::value(
                "some_field",
                serde_json::json!("String"),
                Vec::new(),
            );
            assert_eq!(
                quote!(String).to_string(),
                quoted_string.unwrap().0.to_string(),
            );
        }
        #[test]
        fn tokenize_value_number() {
            let quoted_number = tokenize::value(
                "some_field",
                serde_json::json!("Decimal"),
                Vec::new(),
            );
            assert_eq!(
                quote!(rust_decimal::Decimal).to_string(),
                quoted_number.unwrap().0.to_string(),
            );
        }
        #[test]
        fn tokenize_value_bool() {
            let quoted_bool = tokenize::value(
                "some_field",
                serde_json::json!("bool"),
                Vec::new(),
            );
            assert_eq!(
                quote!(bool).to_string(),
                quoted_bool.unwrap().0.to_string(),
            );
        }
    }
    mod intermediate {
        use crate::*;
        #[test]
        fn process_response_getinfo() {
            let getinfo_path = std::path::Path::new(
                "./test_data/quizface_output/getinfo.json",
            );
            let output = process_response(getinfo_path);
            assert_eq!(
                output.unwrap().to_string(),
                test_consts::GETINFO_RESPONSE
            );
        }
        #[test]
        fn tokenize_object_simple_unnested() {
            let quoted_object = tokenize::value(
                "somefield",
                serde_json::json!(
                    {
                        "inner_a": "String",
                        "inner_b": "bool",
                        "inner_c": "Decimal",
                    }
                ),
                Vec::new(),
            )
            .unwrap();
            assert_eq!(
                quote!(somefield).to_string(),
                quoted_object.0.to_string(),
            );
            assert_eq!(
                quoted_object.1[0].to_string(),
                test_consts::SIMPLE_UNNESTED_RESPONSE,
            );
        }
    }
}

#[cfg(test)]
mod test_consts {
    pub(super) const GETINFO_RESPONSE: &str = "pub mod getinfo { # [derive \
    (Debug , serde :: Deserialize , serde :: Serialize)] pub struct \
    GetinfoResponse { pub proxy : Option < String > , pub balance : \
    rust_decimal :: Decimal , pub blocks : rust_decimal :: Decimal , pub \
    connections : rust_decimal :: Decimal , pub difficulty : rust_decimal :: \
    Decimal , pub errors : String , pub keypoololdest : rust_decimal :: \
    Decimal , pub keypoolsize : rust_decimal :: Decimal , pub paytxfee : \
    rust_decimal :: Decimal , pub protocolversion : rust_decimal :: Decimal , \
    pub relayfee : rust_decimal :: Decimal , pub testnet : bool , pub \
    timeoffset : rust_decimal :: Decimal , pub unlocked_until : rust_decimal \
    :: Decimal , pub version : rust_decimal :: Decimal , pub walletversion : \
    rust_decimal :: Decimal , } }";
    pub(super) const SIMPLE_UNNESTED_RESPONSE: &str = "# [derive (Debug , \
    serde :: Deserialize , serde :: Serialize)] pub struct somefield { pub \
    inner_a : String , pub inner_b : bool , pub inner_c : rust_decimal :: \
    Decimal , }";
}
