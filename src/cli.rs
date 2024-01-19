use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name="sbp-lang", about="A learning programming language.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    // repl
    #[command()]
    #[command(about = "Interactive repl")]
    Repl,

    // file path to compile
    #[command()]
    #[command(about = "Compile a file")]
    Compile {
        #[arg(required = true)]
        filepath: PathBuf,
    },

    // see docs for a query.
    #[command()]
    #[command(about = "See docs")]
    Docs,
}
