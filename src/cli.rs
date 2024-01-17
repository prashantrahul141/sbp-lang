use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name="ll-lang", about="A learning programming language.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    // file path to compile
    #[command(arg_required_else_help = true)]
    #[command(about = "Compile a file")]
    Compile {
        #[arg(required = true)]
        filepath: PathBuf,
    },
}
