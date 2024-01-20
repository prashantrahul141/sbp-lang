mod cli;
mod app {
    pub mod app_main;
    mod compile;
    mod compile_file;
    mod docs;
    mod repl;
    mod utils;
}

mod lexer {
    pub mod lexer_main;
}

mod token {
    pub mod token_main;
    mod token_types;
}

fn main() {
    let mut app = app::app_main::App::new();
    app.run();
}
