use crate::lexer::lexer_main::Lexer;

use super::app_main::App;

impl App {
    /// Top level compile function, this function
    /// takes source as input string and calls all
    ///  the steps required to compile and run it.
    pub fn compile(&self, source: &String) {
        // lexical analysis.
        spdlog::debug!("compiling : \n{}", source);
        let lexer = Lexer::new();
    }
}
