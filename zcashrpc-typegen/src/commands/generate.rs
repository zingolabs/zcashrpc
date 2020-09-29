//! `generate` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::ZcashrpcTypegenConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

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
        let config = app_config();
        /*        let output = "src/client/subcomponents";
                let inputs = get_input("json_data");
                for item in inputs {
                    item.parse_type().write_to(output);
                }
        */
        println!("Hello, {}!", &config.hello.recipient);
    }
}

impl config::Override<ZcashrpcTypegenConfig> for GenerateCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: ZcashrpcTypegenConfig,
    ) -> Result<ZcashrpcTypegenConfig, FrameworkError> {
        if !self.recipient.is_empty() {
            config.hello.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
