fn main() {
    std::fs::File::create(output_path()).unwrap();
    for filenode in std::fs::read_dir(&std::path::Path::new(
        &std::env::args()
            .nth(1)
            .unwrap_or("./example_dir".to_string()),
    ))
    .unwrap()
    {
        process_response(filenode.expect("Problem getting direntry!"));
    }
}

fn process_response(file: std::fs::DirEntry) -> () {
    let file_body = get_data(&file).expect("Couldn't unpack file!");
    let name = file
        .file_name()
        .to_string_lossy()
        .strip_suffix(".json")
        .unwrap()
        .to_string();
    match file_body {
        serde_json::Value::Object(obj) => typegen(obj, &name),
        val => alias(val, &name),
    }
    .expect("file_body failed to match");
}

fn output_path() -> Box<std::path::Path> {
    Box::from(std::path::Path::new(
        &std::env::args().nth(2).unwrap_or("./output.rs".to_string()),
    ))
}

fn get_data(
    file: &std::fs::DirEntry,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut file = std::fs::File::open(file.path())?;
    let mut file_body = String::new();
    use std::io::Read as _;
    file.read_to_string(&mut file_body)?;
    let file_body = serde_json::de::from_str(&file_body)?;
    Ok(file_body)
}

fn typegen(
    data: serde_json::Map<String, serde_json::Value>,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut code = Vec::new();
    // The default collection behind a serde_json_map is a BTreeMap
    // and being the predicate of "in" causes into_iter to be called.
    // See: https://docs.serde.rs/src/serde_json/map.rs.html#3
    for (field_name, val) in data {
        //println!("Got field: {}, {}", field_name, val);
        let key = proc_macro2::Ident::new(
            &field_name,
            proc_macro2::Span::call_site(),
        );
        let val = quote_value(Some(&to_camel_case(&field_name)), val)?;
        let added_code = quote::quote!(pub #key: #val,);
        code.push(added_code);
    }

    let ident = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
    let code = quote::quote!(
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct #ident {
            #(#code)*
        }
    );

    //println!("Going to write: {}", code.to_string());
    let mut output = std::fs::OpenOptions::new()
        .append(true)
        .open(output_path())?;
    //println!("Writing to file: {:#?}", output);
    use std::io::Write as _;
    write!(output, "{}", code.to_string())?;
    //println!("Written!");
    Ok(())
}

fn alias(
    data: serde_json::Value,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let serde_json::Value::Object(_) = data {
        unimplemented!("We don't want to create struct aliases.")
    }
    let name = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
    let type_body = quote_value(None, data)?;
    let aliased = quote::quote!(
        pub type #name = #type_body;
    );
    //println!("Going to write: {}", aliased.to_string());
    let mut output = std::fs::OpenOptions::new()
        .append(true)
        .open(output_path())?;
    //println!("Writing to file: {:#?}", output);
    use std::io::Write as _;
    write!(output, "{}", aliased.to_string())?;
    //println!("Written!");
    Ok(())
}

fn quote_value(
    name: Option<&str>,
    val: serde_json::Value,
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    Ok(match val {
        serde_json::Value::String(kind) => match kind.as_str() {
            "Decimal" => quote::quote!(rust_decimal::Decimal),
            "bool" => quote::quote!(bool),
            "String" => quote::quote!(String),
            otherwise => {
                return Err(format!(
                    "Unexpected type descriptor: \n {}",
                    otherwise
                )
                .into())
            }
        },

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
        otherwise => {
            return Err(
                format!("Did not expect to recieve: \n {}", otherwise).into()
            )
        }
    })
}

fn to_camel_case(input: &str) -> String {
    let mut ret = input.to_string();
    let ch = ret.remove(0);
    ret.insert(0, ch.to_ascii_uppercase());
    ret
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn quote_value_string() {
        let quoted_string = quote_value(None, serde_json::json!("String"));
        assert_eq!(
            quote::quote!(String).to_string(),
            quoted_string.unwrap().to_string(),
        );
    }
    #[test]
    fn quote_value_number() {
        let quoted_number = quote_value(None, serde_json::json!("Decimal"));
        assert_eq!(
            quote::quote!(rust_decimal::Decimal).to_string(),
            quoted_number.unwrap().to_string(),
        );
    }
    #[test]
    fn quote_value_bool() {
        let quoted_bool = quote_value(None, serde_json::json!("bool"));
        assert_eq!(
            quote::quote!(bool).to_string(),
            quoted_bool.unwrap().to_string(),
        );
    }
}