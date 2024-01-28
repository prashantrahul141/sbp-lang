use super::environment::Environment;

/// Top level interpreter struct.
pub struct Interpreter {
    pub globals: Box<Environment>,
    pub environment: Box<Environment>,
}
