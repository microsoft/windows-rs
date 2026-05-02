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
                &self.message,
                hyperlink_path(&self.file_name),
                self.line,
                self.column + 1
            )
        } else if self.file_name.is_empty() {
            write!(f, "\nerror: {}", &self.message)
        } else {
            write!(
                f,
                "\nerror: {}\n --> {}",
                &self.message,
                hyperlink_path(&self.file_name)
            )
        }
    }
}

/// Converts a file-system path to a `file://` URI, normalising Windows
/// backslashes and percent-encoding characters that are not valid in a URI.
fn to_file_uri(path: &str) -> String {
    let normalized = path.replace('\\', "/");
    // Windows drive paths look like "C:/..." – prepend an extra '/' so the
    // authority component is empty and the path is absolute.
    let prefix = if normalized.starts_with('/') {
        "file://"
    } else {
        "file:///"
    };
    let mut uri = String::from(prefix);
    for c in normalized.chars() {
        match c {
            // Encode characters that would otherwise break URI parsing.
            ' ' => uri.push_str("%20"),
            '%' => uri.push_str("%25"),
            '#' => uri.push_str("%23"),
            '?' => uri.push_str("%3F"),
            '[' => uri.push_str("%5B"),
            ']' => uri.push_str("%5D"),
            _ => uri.push(c),
        }
    }
    uri
}

/// Wraps `path` in an OSC 8 hyperlink escape sequence when stderr is an
/// interactive terminal, making the path clickable in supporting terminals.
/// Falls back to returning the path unchanged for non-TTY output so that
/// redirected output (log files, CI runners, etc.) is not polluted with
/// escape codes.
fn hyperlink_path(path: &str) -> String {
    use std::io::IsTerminal as _;
    if path.is_empty() || !std::io::stderr().is_terminal() {
        return path.to_string();
    }
    let uri = to_file_uri(path);
    // OSC 8 format: ESC ] 8 ; params ; uri ST  link-text  ESC ] 8 ; ; ST
    // where ST (String Terminator) is ESC \.
    format!("\x1b]8;;{uri}\x1b\\{path}\x1b]8;;\x1b\\")
}
