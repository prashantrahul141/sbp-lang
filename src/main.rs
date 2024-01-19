mod cli;
mod app {
    pub mod app;
    mod compile;
    mod compile_file;
    mod docs;
    mod utils;
}

fn main() {
    app::app::App::run();
}
