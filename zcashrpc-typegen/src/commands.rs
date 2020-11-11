//! ZcashrpcTypegen Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `generate`: creates a set of zcashrpc subcomponent "Response" structs
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod generate;
mod version;

use self::{generate::GenerateCmd, version::VersionCmd};
use crate::config::ZcashrpcTypegenConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

/// ZcashrpcTypegen Configuration Filename
pub const CONFIG_FILE: &str = "zcashrpc_typegen.toml";

/// ZcashrpcTypegen Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum ZcashrpcTypegenCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `generate` subcommand
    #[options(help = "generate types")]
    Generate(GenerateCmd),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCmd),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<ZcashrpcTypegenConfig> for ZcashrpcTypegenCmd {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = PathBuf::from(CONFIG_FILE);

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }
    fn process_config(
        &self,
        config: ZcashrpcTypegenConfig,
    ) -> Result<ZcashrpcTypegenConfig, abscissa_core::FrameworkError> {
        use abscissa_core::config::Override as _;
        match self {
            ZcashrpcTypegenCmd::Generate(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
