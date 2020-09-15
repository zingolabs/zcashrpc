//! ZcashRcli Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

use crate::config::ZcashRcliConfig;
use abscissa_core::{Command, Configurable, Help, Options, Runnable};
use std::path::PathBuf;

mod version;

/// ZcashRcli Configuration Filename
pub const CONFIG_FILE: &str = "zcash-rcli.toml";

/// ZcashRcli Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum ZcashRcliCmd {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `getinfo` subcommand
    #[options(help = "getinfo rpc call, no arguments.", name = "getinfo")]
    GetInfo(GetInfoCmd),

    /// The 'generate' regtest command
    #[options(
        help = "generate rpc call. generate x (number of blocks to generate"
    )]
    Generate(GenerateCmd),

    /// The getblockchaininfo subcommand
    #[options(
        help = "getblockchaininfo rpc call, no arguments",
        name = "getblockchaininfo"
    )]
    GetBlockchainInfo(GetBlockchainInfoCmd),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(version::VersionCmd),
}

fn make_client(regtest: bool) -> zcashrpc::Client {
    use zcashrpc::client::utils;
    zcashrpc::Client::new(
        utils::get_zcashd_port(),
        utils::get_cookie(regtest).unwrap(),
    )
}

/// A simple Definition and Runnable implementation for commands that make an
/// rpc call
macro_rules! run_impl {
    ( $($command:ident $rpc_call:ident $([$($arg:ident $of:ty)+])? ),+) => {
        $(///The $rpc_call rpc call
        #[derive(Command, Debug, abscissa_core::Options)]
        pub struct $command {
            $($(
                #[options(free)]
                $arg: $of,
            )+)?


            #[options(help = "command-specific help")]
            help: bool,
        }
        impl Runnable for $command {
            fn run(&self) {
                abscissa_tokio::run(&$crate::application::APPLICATION, async {
                    let response =
                        $crate::commands::make_client(true).$rpc_call($($(self.$arg),+)?);
                    println!("Help flag: {:?}", self.help);
                    println!("{:?}", response.await);
                }).unwrap();
            }
        })+
    };
}

run_impl!(
    GetInfoCmd getinfo,
    GetBlockchainInfoCmd getblockchaininfo,
    GenerateCmd generate [how_many u32]
);

/// This trait allows you to define how application configuration is loaded.
impl Configurable<ZcashRcliConfig> for ZcashRcliCmd {
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

    //    /// Apply changes to the config after it's been loaded, e.g. overriding
    //    /// values in a config file using command-line options.
    //    ///
    //    /// This can be safely deleted if you don't want to override config
    //    /// settings from command-line options.
    //    fn process_config(
    //        &self,
    //        config: ZcashRcliConfig,
    //    ) -> Result<ZcashRcliConfig, FrameworkError> {
    //        match self {
    //            ZcashRcliCmd::GetInfo(cmd) => cmd.override_config(config),
    //            _ => Ok(config),
    //        }
    //    }
}
