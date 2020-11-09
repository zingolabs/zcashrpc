//! `generate` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use abscissa_core::{Command, Options, Runnable};

/// `generate` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>

type GenResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Command, Debug, Options, Default)]
pub struct GenerateCmd {
    #[options(help = "print this message")]
    help: bool,

    #[options(
        long = "optional",
        meta = "[FIELDNAME]",
        help = "field names here are optional"
    )]
    optional_if_present: Vec<String>,

    #[options(
        long = "missing",
        short = "m",
        meta = "[STRUCTNAME:[]]",
        help = "the named structs have the named fields added, if not present"
    )]
    add_if_missing: String,
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        if self.help {
            println!("Env args: {:?}", std::env::args());
            let usage = abscissa_core::command::Usage::for_command::<Self>();
            println!("Info:");
            usage.print_info();
            println!("Usage:");
            usage.print_usage();
            println!("usage struct: {:#?}", usage);
        } else {
            println!("{:#?}", wrapper_fn_to_enable_question_mark(self));
        }
    }
}

fn parse_struct_and_field(
    input: &str,
) -> Result<Vec<(String, Vec<String>)>, abscissa_core::FrameworkError> {
    let none_to_err = || {
        abscissa_core::error::context::Context::new(
            abscissa_core::FrameworkErrorKind::ParseError,
            Some(Box::<dyn std::error::Error + Send + Sync>::from(
                String::from("invalid add_if_missing syntax"),
            )),
        )
    };

    let mut ret = Vec::new();
    for item in input.split_terminator(';') {
        let mut struct_and_field = item.split(':');
        let (object, fields) = (
            struct_and_field.next().ok_or_else(none_to_err)?,
            struct_and_field.next().ok_or_else(none_to_err)?,
        );
        let fields = fields.split(',').map(|x| x.to_string()).collect();
        ret.push((object.to_string(), fields));
    }
    Ok(ret)
}

impl abscissa_core::config::Override<crate::config::ZcashrpcTypegenConfig>
    for GenerateCmd
{
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: crate::config::ZcashrpcTypegenConfig,
    ) -> Result<
        crate::config::ZcashrpcTypegenConfig,
        abscissa_core::FrameworkError,
    > {
        config
            .optional_if_present
            .append(&mut self.optional_if_present.clone());
        config
            .add_if_missing
            .append(&mut parse_struct_and_field(&self.add_if_missing)?);

        Ok(config)
    }
}

fn wrapper_fn_to_enable_question_mark(
    _cmd: &GenerateCmd,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = crate::prelude::app_config();
    std::fs::File::create(&config.output)?;
    for file in std::fs::read_dir(&config.input).unwrap() {
        let (file_name, file_body) = get_data(file?)?;
        println!("Parsed input: {:#?}, {:#?}", file_name, file_body);
        let name = file_name.strip_suffix(".json").unwrap().to_string();
        match file_body {
            serde_json::Value::Object(obj) => typegen(obj, &name),
            val => alias(val, name),
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
    let data_items = data.into_iter().fold(
        std::collections::BTreeMap::new(),
        |mut ret, (key, value)| {
            ret.insert(key, value);
            ret
        },
    );
    for field in data_items {
        println!("Got field: {}, {}", field.0, field.1);
        let (field_name, val) = field;
        let key = proc_macro2::Ident::new(
            &field_name,
            proc_macro2::Span::call_site(),
        );
        let val = quote_value(Some(&to_camel_case(&field_name)), val)?;
        let mut added_code = quote::quote!(pub #key: #val,);
        if crate::prelude::app_config()
            .optional_if_present
            .contains(&field_name)
        {
            added_code = quote::quote!(Option<#added_code>)
        }
        code.push(added_code);
    }

    let ident = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
    let code = quote::quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #ident {
            #(#code)*
        }
    );

    println!("Going to write: {}", code.to_string());
    let output = &crate::prelude::app_config().output;
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
    if let serde_json::Value::Object(_) = data {
        unimplemented!("We don't want to create struct aliases.")
    }
    let name = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
    let type_body = quote_value(None, data)?;
    let aliased = quote::quote!(
        pub type #name = #type_body;
    );
    println!("Going to write: {}", aliased.to_string());
    let output = &crate::prelude::app_config().output;
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

fn to_camel_case(input: &str) -> String {
    let mut ret = input.to_string();
    let ch = ret.remove(0);
    ret.insert(0, ch.to_ascii_uppercase());
    ret
}
