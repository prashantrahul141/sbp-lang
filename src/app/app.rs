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
        let cli_args = Cli::parse();

        // calling App's required method according to option used in cli.
        match cli_args.command {
            // compile.
            cli::Commands::Compile { filepath } => App::compile(&filepath),

            // docs.
            cli::Commands::Docs { query } => App::docs(&query),
        }
    }
}
