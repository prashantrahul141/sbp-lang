use super::app_main::App;
use crate::{
    interpreter::interpreter_main::Interpreter, lexer::lexer_main::Lexer,
    parser::parser_main::Parser,
};

impl App {
    /// Top level compile function, this function
    /// takes source as input string and calls all
    /// the steps required to compile and run it.
    pub fn compile(&self, source: String) {
        spdlog::info!("Compiling : \n{}", source);

        // lexical analysis.
        spdlog::info!("Running Lexer on source.");
        let mut lexer = Lexer::new(source, App::get_reserved_keywords());
        let tokens = lexer.scan_tokens().to_owned();

        spdlog::info!("Parsing recieved tokens.");
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        if parser.has_error {
            spdlog::error!("Found parsing errrors, terminating execution.");
            return;
        }

        let mut interpreter = Interpreter::new();
        interpreter.interpret(statements)
    }
}
