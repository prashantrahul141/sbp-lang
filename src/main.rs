mod cli;

mod app {
    pub mod app_main;
    mod compile;
    mod compile_file;
    mod docs;
    mod global_const;
    mod repl;
    mod utils;
}

mod lexer {
    pub mod lexer_main;
    mod scanners;
    mod utils;
}

mod token {
    pub mod token_main;
    pub mod token_types;
}

mod ast {
    pub mod ast_printer;
    pub mod ast_tree;
}

fn main() {
    // creating app.
    let mut app = app::app_main::App::new();

    // setup logging.
    app::app_main::App::setup_logging();

    // running.
    app.run();
}
