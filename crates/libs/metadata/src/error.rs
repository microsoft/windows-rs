pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default, Debug)]
pub struct Error {
    message: String,
    path: String,
    span: Option<(usize, usize)>,
}

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
    pub fn new(message: &str) -> Self {
        Self { message: message.to_string(), ..Self::default() }
    }

    pub fn with_path(self, path: &str) -> Self {
        Self { path: path.to_string(), ..self }
    }

    pub fn with_span(self, span: proc_macro2::Span) -> Self {
        let start = span.start();
        Self { span: Some((start.line, start.column)), ..self }
    }
}

pub fn read_file(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|_| Error::new("failed to read file").with_path(path))
}

pub fn read_to_string(path: &str) -> Result<String> {
    std::fs::read_to_string(path).map_err(|_| Error::new("failed to read file").with_path(path))
}

pub fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) -> Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).map_err(|_| Error::new("failed to create directory").with_path(path))?;
    }

    std::fs::write(path, contents).map_err(|_| Error::new("failed to write file").with_path(path))
}

pub fn canonicalize(path: &str) -> Result<String> {
    let path = std::fs::canonicalize(path).map_err(|_| Error::new("failed to find path").with_path(path))?;
    let path = path.to_string_lossy().trim_start_matches(r#"\\?\"#).to_string();

    match extension(&path) {
        (_, "") => Ok(path),
        (file, extension) => Ok(format!("{file}.{}", extension.to_lowercase())),
    }
}

pub fn extension(path: &str) -> (&str, &str) {
    if let Some((file, extension)) = path.rsplit_once('.') {
        (file, extension)
    } else {
        ("", "")
    }
}
