use spdlog::{debug, error, trace};

use super::app::App;
use std::{fs, path::PathBuf, process::exit};

impl App {
    /// top level compile function.
    /// this handles calling all steps required for compilation.
    pub fn compile(filepath: &PathBuf) {
        // checking for existence of file.
        if !filepath.exists() {
            error!(
                "The file \"{}\" does not exist.",
                filepath.to_str().unwrap_or(""),
            );
            exit(1);
        }

        // reading file contents.
        debug!("Reading file contents : {}", filepath.to_str().unwrap());
        let mut file_contents: String;
        match fs::read_to_string(filepath) {
            Ok(file) => {
                debug!("successfully read file contents.");
                trace!("contents of the file : {}", file);
                file_contents = file;
            }
            Err(err) => {
                error!("failed to read file because {}", err.to_string());
                exit(1);
            }
        }

        // lexical analysis.
    }
}
