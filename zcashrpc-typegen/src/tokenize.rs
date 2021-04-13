use crate::error;
use crate::special_cases;
use error::TypegenResult;
use proc_macro2::TokenStream;
use quote::quote;

fn handle_terminal_enum(label: &str, name: &str) -> TokenStream {
    let variants = label
        .strip_prefix("ENUM:")
        .unwrap()
        .split(',')
        .map(|x| x.trim());
    let variant_idents = variants
        .clone()
        .map(|x| {
            proc_macro2::TokenTree::Ident(crate::callsite_ident(
                &x.split('-')
                    .map(crate::capitalize_first_char)
                    .collect::<String>(),
            ))
            .into()
        })
        .collect::<Vec<TokenStream>>();
    let variant_idents_renames = variants
        .map(|x| format!("#[serde(rename = \"{}\")]", x).parse().unwrap())
        .collect::<Vec<TokenStream>>();
    let name_tokens = crate::callsite_ident(&format!("{}Response", name));
    quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub enum #name_tokens {
            #(#variant_idents_renames #variant_idents,)*
        }
    )
}

pub(crate) fn value(
    name: &str,
    val: serde_json::Value,
) -> TypegenResult<(TokenStream, Vec<TokenStream>, bool)> {
    match val {
        serde_json::Value::String(label) => terminal(name, label.as_str()),
        serde_json::Value::Array(vec) => {
            array(name, vec).map(|x| (x.0, x.1, false))
        }
        serde_json::Value::Object(obj) => {
            object(name, obj).map(|x| (x.0, x.1, false))
        }
        otherwise => Err(error::QuizfaceAnnotationError {
            kind: error::InvalidAnnotationKind::from(otherwise),
            location: name.to_string(),
        })?,
    }
}

fn terminal(
    name: &str,
    label: &str,
) -> TypegenResult<(TokenStream, Vec<TokenStream>, bool)> {
    Ok((
        match label {
            "Decimal" => quote!(rust_decimal::Decimal),
            "bool" => quote!(bool),
            "String" => quote!(String),
            "hexadecimal" => quote!(String),
            "INSUFFICIENT" => {
                return Err(error::TypegenError::from(
                    error::QuizfaceAnnotationError {
                        kind: error::InvalidAnnotationKind::Insufficient,
                        location: name.to_string(),
                    },
                ))
            }
            enumeration if enumeration.starts_with("ENUM:") => {
                let ident = crate::callsite_ident(name);
                let enum_tokens = handle_terminal_enum(enumeration, name);
                return Ok((quote!(#ident), vec![enum_tokens], true));
            }
            otherwise => {
                return Err(error::QuizfaceAnnotationError {
                    kind: error::InvalidAnnotationKind::from(
                        serde_json::Value::String(otherwise.to_string()),
                    ),
                    location: name.to_string(),
                }
                .into())
            }
        },
        Vec::new(),
        false,
    ))
}

fn array(
    name: &str,
    mut array_of: Vec<serde_json::Value>,
) -> TypegenResult<(TokenStream, Vec<TokenStream>)> {
    let (val, inner_structs, _terminal_enum) = value(
        name,
        array_of.pop().ok_or(error::QuizfaceAnnotationError {
            kind: error::InvalidAnnotationKind::EmptyArray,
            location: name.to_string(),
        })?,
    )?;
    Ok((quote!(Vec<#val>), inner_structs))
}

fn object(
    name: &str,
    val: serde_json::Map<String, serde_json::Value>,
) -> TypegenResult<(TokenStream, Vec<TokenStream>)> {
    let ident = crate::callsite_ident(name);
    let (case, inner_structs) = crate::structgen(val, name)?;
    match case {
        special_cases::Case::Regular => Ok((quote!(#ident), inner_structs)),
        special_cases::Case::FourXs => Ok((
            quote!(std::collections::HashMap<String, #ident>),
            inner_structs,
        )),
    }
}
