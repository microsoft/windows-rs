/// An error produced while reading, writing, or formatting RDL.
pub struct Error {
    /// Human-readable description of what went wrong.
    pub message: String,
    /// Name of the file in which the error occurred, if known.
    pub file_name: String,
    /// Line number of the error, or `0` if not applicable.
    pub line: usize,
    /// Zero-based column number of the error, or `0` if not applicable.
    pub column: usize,
}

impl Error {
    /// Creates a new error with the given message and source location.
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

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.line != 0 || self.column != 0 {
            write!(
                f,
                "\nerror: {}\n --> {}:{}:{}",
                self.message,
                self.file_name,
                self.line,
                self.column + 1
            )
        } else if self.file_name.is_empty() {
            write!(f, "\nerror: {}", self.message)
        } else {
            write!(f, "\nerror: {}\n --> {}", self.message, self.file_name)
        }
    }
}
