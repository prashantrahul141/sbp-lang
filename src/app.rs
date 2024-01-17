use crate::cli::{self, Cli};
use clap::Parser;

pub struct App;

impl App {
    pub fn run(&self) {
        let cli_args = Cli::parse();
        match cli_args.command {
            cli::Commands::Compile { filepath: _ } => todo!(),
        }
    }
}
