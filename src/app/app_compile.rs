use spdlog::{debug, error, trace};

use super::app::App;
use std::{path::PathBuf, process::exit};

impl App {
    pub fn compile(filepath: &PathBuf) {
        if !filepath.exists() {
            error!(
                "The file \"{}\" does not exist.",
                filepath.to_str().unwrap_or(""),
            );
            exit(1);
        }
    }
}
