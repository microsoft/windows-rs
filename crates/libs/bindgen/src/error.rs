pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default, Debug)]
pub struct Error {
    message: String,
    path: String,
    span: Option<(usize, usize)>,
}

impl std::error::Error for Error {}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error.message.as_str())
    }
}

impl From<syn::Error> for Error {
    fn from(error: syn::Error) -> Self {
        let start = error.span().start();
        Self { message: error.to_string(), span: Some((start.line, start.column)), ..Self::default() }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "error: {}", self.message)?;
        if !self.path.is_empty() {
            if let Some((line, column)) = self.span {
                writeln!(fmt, "  --> {}:{line}:{column}", self.path)?;
            } else {
                writeln!(fmt, "  --> {}", self.path)?;
            }
        }
        Ok(())
    }
}

impl Error {
    pub(crate) fn new(message: &str) -> Self {
        Self { message: message.to_string(), ..Self::default() }
    }

    pub(crate) fn with_path(self, path: &str) -> Self {
        Self { path: path.to_string(), ..self }
    }

    // pub(crate) fn with_span(self, span: proc_macro2::Span) -> Self {
    //     let start = span.start();
    //     Self {
    //         span: Some((start.line, start.column)),
    //         ..self
    //     }
    // }
}
