//! `generate` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use abscissa_core::{Options, Runnable};

/// `generate` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>

type GenResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Command, Debug, Options)]
pub struct GenerateCmd {
    /// To whom are we saying hello?
    #[options(free)]
    recipient: Vec<String>,
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        println!("{:#?}", error_handle_run(self));
    }
}

fn error_handle_run(
    cmd: &GenerateCmd,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = app_config();
    std::fs::File::create(&config.output);
    for file in std::fs::read_dir(&config.input).unwrap() {
        let (file_name, file_body) = get_data(file?)?;
        println!("Parsed input: {:#?}, {:#?}", file_name, file_body);
        let name = file_name.strip_suffix(".json").unwrap().to_string();
        match file_body {
            serde_json::Value::Object(obj) => typegen(obj, &name),
            val @ _ => alias(val, name),
        }?;
    }
    Ok(())
}

fn get_data(
    file: std::fs::DirEntry,
) -> Result<(String, serde_json::Value), Box<dyn std::error::Error>> {
    let file_name = file.file_name().to_string_lossy().to_string();
    let mut file = std::fs::File::open(file.path())?;
    let mut file_body = String::new();
    use std::io::Read as _;
    file.read_to_string(&mut file_body)?;
    let file_body = serde_json::de::from_str(&file_body)?;
    Ok((file_name, file_body))
}

fn typegen(
    data: serde_json::Map<String, serde_json::Value>,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut code = Vec::new();
    for field in sort_and_iter(data) {
        println!("Got field: {}, {}", field.0, field.1);
        let (field_name, val) = field;
        let key = proc_macro2::Ident::new(
            &field_name,
            proc_macro2::Span::call_site(),
        );
        let val = quote_value(Some(&to_camel_case(&field_name)), val)?;
        code.push(quote::quote!(pub #key: #val,));
    }

    let ident = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
    let code = quote::quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #ident {
            #(#code)*
        }
    );

    println!("Going to write: {}", code.to_string());
    let output = &app_config().output;
    println!("Opening file: {:#?}", output);
    let mut output = std::fs::OpenOptions::new().append(true).open(output)?;
    println!("Writing to file: {:#?}", output);
    use std::io::Write as _;
    write!(output, "{}", code.to_string())?;
    println!("Written!");
    Ok(())
}

fn alias(
    data: serde_json::Value,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
    let type_body = quote_value(None, data)?;
    let aliased = quote::quote!(
        pub type #name = #type_body;
    );
    println!("Going to write: {}", aliased.to_string());
    let output = &app_config().output;
    println!("Opening file: {:#?}", output);
    let mut output = std::fs::OpenOptions::new().append(true).open(output)?;
    println!("Writing to file: {:#?}", output);
    use std::io::Write as _;
    write!(output, "{}", aliased.to_string())?;
    println!("Written!");
    Ok(())
}

fn quote_value(
    name: Option<&str>,
    val: serde_json::Value,
) -> GenResult<proc_macro2::TokenStream> {
    Ok(match val {
        serde_json::Value::Number(_) => quote::quote!(rust_decimal::Decimal),
        serde_json::Value::Bool(_) => quote::quote!(bool),
        serde_json::Value::Array(mut vec) => {
            let val = quote_value(
                name,
                vec.pop().ok_or(<Box<dyn std::error::Error>>::from(
                    String::from("Cannot determine type of empty array"),
                ))?,
            )?;
            quote::quote!(Vec<#val>)
        }
        serde_json::Value::Null => {
            return Err(String::from("Unexpected null value").into())
        }
        serde_json::Value::Object(obj) => {
            let name = name.ok_or(<Box<dyn std::error::Error>>::from(
                format!("Received struct with no name: {:#?}", obj),
            ))?;
            typegen(obj, name)?;
            let ident =
                proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
            quote::quote!(#ident)
        }
        serde_json::Value::String(_) => quote::quote!(String),
    })
}

fn sort_and_iter(
    obj: serde_json::Map<String, serde_json::Value>,
) -> impl Iterator<Item = (String, serde_json::Value)> {
    let mut obj: Vec<(String, serde_json::Value)> = obj.into_iter().collect();
    obj.sort_unstable_by_key(|(k, v)| k.clone());
    obj.into_iter()
}

fn to_camel_case(input: &str) -> String {
    let mut ret = input.to_string();
    let ch = ret.remove(0);
    ret.insert(0, ch.to_ascii_uppercase());
    ret
}
