use super::app::App;
use spdlog::{debug, error, trace};
use std::{fs, path::PathBuf, process::exit};

impl App {
    /// compile function for files.
    /// this is a wrapper for compile function.
    pub fn compile_file(&self, filepath: &PathBuf) {
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
        let file_contents: String;
        match fs::read_to_string(filepath) {
            Ok(file) => {
                debug!("successfully read file contents.");
                trace!("contents of the file : \n{}", file);
                file_contents = file;
            }
            Err(err) => {
                error!("failed to read file because {}", err.to_string());
                exit(1);
            }
        }

        self.compile(&file_contents);

        if self.has_error {
            panic!("Has some error idk");
        }
    }
}
