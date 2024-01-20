use super::app::App;
use spdlog::{LevelFilter, Logger};
use std::sync::Arc;

impl App {
    pub fn set_logging_level(level: LevelFilter) {
        let default_logger: Arc<Logger> = spdlog::default_logger();
        default_logger.set_level_filter(level);
    }

    pub fn error(&self, line: u32, message: String) {
        self.report(line, "".to_string(), message)
    }

    pub fn report(&self, line: u32, where_is: String, message: String) {
        print!("[line {}] Error {} : {}", line, where_is, message);
    }
}
