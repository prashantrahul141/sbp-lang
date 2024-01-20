use super::app_main::App;
use spdlog::Logger;
use std::sync::Arc;

impl App {
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

    pub fn error(line: usize, message: String) {
        App::report(line, "".to_string(), message)
    }

    pub fn report(line: usize, where_is: String, message: String) {
        println!("[line {}] Error '{}' : {}", line, where_is, message);
    }
}
