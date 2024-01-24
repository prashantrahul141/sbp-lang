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

        match parser.parse() {
            Some(expr) => {
                if self.has_error {
                    return;
                }

                spdlog::info!("does not have error till parsing.");

                let mut interpreter = Interpreter::new();
                let value = interpreter.interpret(expr);
                println!("{:?}", value);
            }
            None => panic!("failed to parse"),
        }
    }
}
