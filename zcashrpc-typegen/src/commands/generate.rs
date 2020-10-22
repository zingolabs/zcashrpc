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
#[derive(Command, Debug, Options)]
pub struct GenerateCmd {
    /// To whom are we saying hello?
    #[options(free)]
    recipient: Vec<String>,
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        error_handle_run(self);
    }
}

fn error_handle_run(
    cmd: &GenerateCmd,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = app_config();
    for file in std::fs::read_dir(&config.input).unwrap() {
        let (file_name, file_body) = get_data(file?)?;
        println!("Parsed input: {:#?}, {:#?}", file_name, file_body);
        let name = file_name.strip_suffix(".json").unwrap().to_string();
        match file_body {
            serde_json::Value::Object(obj) => typegen(obj, name),
            val @ _ => alias(val, name),
        };
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
    file.read_to_string(&mut file_body);
    let file_body = serde_json::de::from_str(&file_body)?;
    Ok((file_name, file_body))
}

fn typegen(
    data: serde_json::Map<String, serde_json::Value>,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    for field in sort_and_iter(data) {
        println!("Got field: {}, {}", field.0, field.1);
    }
    todo!("Can't generate anything yet")
}

fn alias(
    data: serde_json::Value,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let type_body = match data {
        serde_json::Value::Number(_) => quote::quote!(rust_decimal::Decimal),
        serde_json::Value::Bool(_) => quote::quote!(bool),
        serde_json::Value::Array(vec) => todo!("Can't alias vecs yet"),
        serde_json::Value::Null => {
            return Err(String::from("Unexpected null value").into())
        }
        serde_json::Value::Object(_) => {
            unreachable!("We should never alias an object type")
        }
        serde_json::Value::String(_) => quote::quote!(String),
    };
    let aliased = quote::quote!(
        pub type #name: $type_body;
    );
    println!("{}", aliased.to_string());

    todo!("Can't alias types yet!")
}

fn sort_and_iter(
    obj: serde_json::Map<String, serde_json::Value>,
) -> impl Iterator<Item = (String, serde_json::Value)> {
    let mut obj: Vec<(String, serde_json::Value)> = obj.into_iter().collect();
    obj.sort_unstable_by_key(|(k, v)| k.clone());
    obj.into_iter()
}
