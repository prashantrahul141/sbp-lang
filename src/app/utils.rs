use super::app_main::App;
use crate::token::{token_main::Token, token_types::TokenType};
use core::panic;
use spdlog::Logger;
use std::sync::Arc;

impl App {
    /// Struct method to setup global logging.
    pub fn setup_logging() {
        if cfg!(debug_assertions) {
            let default_logger: Arc<Logger> = spdlog::default_logger();
            default_logger.set_level_filter(spdlog::LevelFilter::All);
            spdlog::info!("in debug mode, defaulting log level to : All");
        } else {
            match spdlog::init_env_level_from("SPX_LOG") {
                Ok(applied) => {
                    let default_logger: Arc<Logger> = spdlog::default_logger();
                    if applied {
                        spdlog::info!(
                            "Applied level from env variable to : {:?}",
                            default_logger.level_filter()
                        );
                    } else {
                        default_logger.set_level_filter(spdlog::LevelFilter::Off);
                    }
                }
                Err(_) => {
                    let default_logger: Arc<Logger> = spdlog::default_logger();
                    default_logger.set_level_filter(spdlog::LevelFilter::All);
                    spdlog::info!(
                        "Failed to apply log level from env variable defaulting to : All"
                    );
                }
            }
        };
    }

    /// Global struct method to display error, its a wrapper for App::repot()
    /// # Arguments
    /// * `line` - the line number in the file error was found.
    /// * `message` - message to display.
    pub fn error(line: usize, message: String) {
        App::report(line, "".to_string(), message.to_string());
    }

    /// Global struct method to display error, its a wrapper for App::repot()
    /// similar to error, but it takes tokens.
    /// # Arguments
    /// * `token` - the token where error occured.
    /// * `message` - message to display.
    pub fn error_token(token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            App::report(token.line, " at end {}".to_string(), message);
        } else {
            App::report(token.line, format!(" at '{}' ", token.lexeme), message);
        }
    }

    /// Global struct method to display error.
    /// # Arguments
    /// * `line` - the line number in the file error was found.S
    /// * `where_is` - in which phase the error was found.
    /// * `message` - message to display.
    pub fn report(line: usize, where_is: String, message: String) {
        spdlog::error!("App::report called for line : {line} with where_is : {where_is} and message : {message}");
        println!("[line {}] Error '{}' : {}", line, where_is, message);
    }

    /// Function to show runtime errors and set state.
    /// # Argument
    /// * `line` - line number where error occured.
    /// * `message` - message for the error.
    pub fn runtime_error(line: usize, message: String) {
        spdlog::error!("App::runtime_error called for line : {line} wht message : {message}");
        App::report(line, "".to_string(), message.to_string());
        panic!("[line {}] : {}", line, message);
    }

    /// Sets up hook for global panic!().
    pub fn setup_custom_panic() {
        use std::panic;
        panic::set_hook(Box::new(|panic_info| {
            if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
                println!("Runtime panic occured: {message:?}");
            } else {
                println!("Runtime panic occured.");
            }
        }));
    }
}
