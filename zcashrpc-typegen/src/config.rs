//! ZcashrpcTypegen Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// ZcashrpcTypegen Configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ZcashrpcTypegenConfig {
    /// Path to the default directory or file to read json from
    pub input: Box<std::path::Path>,
    /// Path to the default location to print our rust code to
    pub output: Box<std::path::Path>,
    ///Fields to be marked as optional, if present
    pub optional_if_present: Vec<String>,
    ///fields to be added if not already present
    pub add_if_missing: MissingTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct MissingTypes {
    pub data: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, String>,
    >,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on ZcashrpcTypegenConfig instead.
impl Default for ZcashrpcTypegenConfig {
    fn default() -> Self {
        Self {
            input: Box::from(std::path::Path::new("../json_data")),
            output: Box::from(std::path::Path::new(
                "../src/client/subcomponents.rs",
            )),
            optional_if_present: Vec::new(),
            add_if_missing: MissingTypes::default(),
        }
    }
}
