use crate::cli::{self, Cli};
use clap::Parser;
use spdlog::LevelFilter;

/// Top level app.
/// this holds the implementation for calling all requied top level methods when called from cli.
pub struct App;

impl App {
    pub fn run() {
        // setup logging.
        App::set_logging_level(LevelFilter::All);

        // parsing cli args.
        spdlog::debug!("parsing cli");
        let cli_args = Cli::parse();

        // calling App's required method according to option used in cli.
        match cli_args.command {
            // repl
            cli::Commands::Repl => {
                spdlog::debug!("repl command was invoked.");
            }

            // compile.
            cli::Commands::Compile { filepath } => {
                spdlog::debug!("compile command was invoked");
                App::compile_file(&filepath);
            }

            // docs.
            cli::Commands::Docs => {
                spdlog::debug!("docs command was invoked");
                App::docs();
            }
        }
    }
}
