use super::app_main::App;

impl App {
    /// Instance method to be called when docs command in invoked.
    pub fn docs() {
        spdlog::debug!("redirecting to docs.");
    }
}
