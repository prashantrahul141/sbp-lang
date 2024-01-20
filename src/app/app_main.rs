use crate::cli::{self, Cli};
use clap::Parser;

/// Top level app.
/// this holds the implementation for calling all requied top level methods when called from cli.
pub struct App {
    pub has_error: bool,
}

impl App {
    // create new instance
    pub fn new() -> Self {
        Self { has_error: false }
    }

    pub fn run(&mut self) {
        // setup logging.

        // parsing cli args.
        spdlog::debug!("parsing cli");
        let cli_args = Cli::parse();

        // calling App's required method according to option used in cli.
        match cli_args.command {
            // repl
            cli::Commands::Repl => {
                spdlog::debug!("repl command was invoked.");
                self.repl();
            }

            // compile.
            cli::Commands::Compile { filepath } => {
                spdlog::debug!("compile command was invoked");
                self.compile_file(&filepath);
            }

            // docs.
            cli::Commands::Docs => {
                spdlog::debug!("docs command was invoked");
                App::docs();
            }
        }
    }
}
