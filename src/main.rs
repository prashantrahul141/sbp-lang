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

mod parser {
    pub mod error;
    pub mod parser_main;
    mod utils;
}

mod interpreter {
    pub mod interpreter_main;
    pub mod utils;
}

fn main() {
    // creating app.
    let mut app = app::app_main::App::new();

    // setup logging.
    app::app_main::App::setup_logging();
    // setup custom panic!() message.
    app::app_main::App::setup_custom_panic();

    // running.
    app.run();
}
