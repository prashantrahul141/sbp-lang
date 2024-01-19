mod cli;
mod app {
    pub mod app;
    mod app_compile;
    mod app_docs;
    mod app_utils;
}

fn main() {
    app::app::App::run();
}
