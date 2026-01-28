#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub file_name: String,
    pub line: usize,
    pub column: usize,
}

impl Error {
    pub fn new(message: &str, file_name: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.to_string(),
            file_name: file_name.to_string(),
            line,
            column,
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.message.fmt(f)
    }
}
