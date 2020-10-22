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
        println!("{:#?}, {:#?}", file_name, file_body);
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
