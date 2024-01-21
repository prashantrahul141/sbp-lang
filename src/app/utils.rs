use super::app_main::App;
use spdlog::Logger;
use std::sync::Arc;

impl App {
    /// Struct method to setup global logging.
    pub fn setup_logging() {
        let default_logger: Arc<Logger> = spdlog::default_logger();

        let log_level = match std::env::var("PROFILE")
            .unwrap_or("release".to_owned())
            .as_str()
        {
            "release" => spdlog::LevelFilter::All,
            "debug" => spdlog::LevelFilter::All,
            _ => spdlog::LevelFilter::All,
        };

        default_logger.set_level_filter(log_level);
    }

    /// Global struct method to display error, its a wrapper for App::repot()
    /// # Arguments
    /// * `line` - the line number in the file error was found.S
    /// * `message` - message to display.
    pub fn error(line: usize, message: String) {
        App::report(line, "".to_string(), message)
    }

    /// Global struct method to display error.
    /// # Arguments
    /// * `line` - the line number in the file error was found.S
    /// * `where_is` - in which phase the error was found.
    /// * `message` - message to display.
    pub fn report(line: usize, where_is: String, message: String) {
        println!("[line {}] Error '{}' : {}", line, where_is, message);
    }
}
