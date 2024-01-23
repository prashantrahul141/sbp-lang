use crate::{
    ast::{ast_printer::AstPrinter, ast_tree::walk_expr},
    lexer::lexer_main::Lexer,
    parser::parser_main::Parser,
};

use super::app_main::App;

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
            Some(exp) => {
                let mut ast = AstPrinter::new();
                walk_expr(&mut ast, &exp);
            }
            None => panic!("failed to parse"),
        }

        if self.has_error {
            return;
        }
    }
}
