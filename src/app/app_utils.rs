use super::app::App;
use spdlog::{LevelFilter, Logger};
use std::sync::Arc;

impl App {
    pub fn set_logging_level(level: LevelFilter) {
        let default_logger: Arc<Logger> = spdlog::default_logger();
        default_logger.set_level_filter(level);
    }
}
