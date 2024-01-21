use super::app_main::App;
use std::io::{self, stdout, Write};

impl App {
    /// Instance method to be called when repl command in invoked.
    pub fn repl(&mut self) {
        // input source string.
        let mut source = String::new();

        // looping infinitely
        loop {
            print!(">");
            stdout().flush().unwrap();

            // taking input.
            match io::stdin().read_line(&mut source) {
                Ok(_) => {
                    spdlog::trace!("read from stdin : {}", source);

                    // if Ctrl+D was pressed.
                    if source.is_empty() {
                        break;
                    }

                    // else compile and run the line.
                    self.compile(source);
                }
                // incase of failing to read line.
                Err(err) => {
                    spdlog::error!("failed to read line from stdin because : {}", err);
                }
            };

            source = "".to_string();
            self.has_error = false;
        }

        spdlog::debug!("breaking out of the repl loop.")
    }
}
