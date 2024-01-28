use crate::token;

use super::{environment::Environment, interpreter_main::Interpreter};

impl Interpreter {
    pub fn get_globals() -> Box<Environment> {
        let mut globals = Box::new(Environment::new(None));

        // language version.
        globals.define(
            "__VERSION__".to_string(),
            super::environment::SplaxDeclarations::Literals(Box::new(
                token::token_main::TokenLiterals::String(
                    std::env!("CARGO_PKG_VERSION").to_string(),
                ),
            )),
        );

        // url to homepage.
        globals.define(
            "__HOMEPAGE__".to_string(),
            super::environment::SplaxDeclarations::Literals(Box::new(
                token::token_main::TokenLiterals::String(
                    std::env!("CARGO_PKG_HOMEPAGE").to_string(),
                ),
            )),
        );

        globals
    }
}
