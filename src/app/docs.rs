use spdlog;

use super::app::App;

impl App {
    pub fn docs() {
        spdlog::debug!("redirecting to docs.");
    }
}
