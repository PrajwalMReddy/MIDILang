use std::process::exit;

pub struct ErrorHandler {
    errors: Vec<Error>,
    file: String,
}

struct Error {
    etype: String,
    message: String,
    line: u32,
}

pub fn init_errors(file: &String) -> ErrorHandler {
    ErrorHandler {
        errors: Vec::<Error>::new(),
        file: file.clone(),
    }
}

impl ErrorHandler {
    fn make_error(etype: String, message: String, line: u32) -> Error {
        return Error {
            etype,
            message,
            line,
        }
    }

    pub fn add_error(&mut self, etype: String, message: String, line: u32) {
        self.errors.push(ErrorHandler::make_error(etype, message, line));
    }

    pub fn display_if_has_errors(&self) {
        if self.has_errors() {
            self.display_errors();
        }
    }

    fn display_errors(&self) {
        println!("\n-- Errors --\n");

        for error in &self.errors {
            let lines: Vec<&str> = self.file.split('\n').collect();
            let line = lines[error.line as usize - 1];

            println!("{} | {}", error.etype, error.message);
            println!("Line {}: {}\n", error.line, line.trim());
        }

        exit(1);
    }

    pub fn has_errors(&self) -> bool {
        self.errors.len() != 0
    }
}
