

#[derive(Debug, Clone, Copy)]
pub struct Error {
    pub had_error: bool
}

impl Error {
    pub fn new() -> Error {
        Error {
            had_error: false
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}