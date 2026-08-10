use super::*;
use std::io::Write;
use std::path::Path;

impl Generator {
    /// Renders and formats this request as Rust source.
    pub fn format(&self) -> Result<String, Error> {
        let source = self.write()?.to_string();
        let mut command = std::process::Command::new("rustfmt");
        command
            .args(["--edition", "2024", "--config", "newline_style=Unix"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = command.spawn().map_err(Error::FormatterIo)?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Error::FormatterFailed("rustfmt stdin was not available".to_string()))?;
        if let Err(error) = stdin.write_all(source.as_bytes()) {
            drop(stdin);
            let _ = child.wait();
            return Err(Error::FormatterIo(error));
        }
        drop(stdin);
        let output = child.wait_with_output().map_err(Error::FormatterIo)?;
        if !output.status.success() {
            return Err(Error::FormatterFailed(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        String::from_utf8(output.stdout).map_err(|error| {
            Error::FormatterFailed(format!("rustfmt returned invalid UTF-8: {error}"))
        })
    }

    /// Formats and writes this request, returning whether the file changed.
    pub fn write_file(&self, path: impl AsRef<Path>) -> Result<bool, Error> {
        let path = path.as_ref();
        let contents = self.format()?;
        match std::fs::read(path) {
            Ok(existing) if existing == contents.as_bytes() => return Ok(false),
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(source) => {
                return Err(Error::FileIo {
                    path: path.to_path_buf(),
                    source,
                });
            }
        }
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|source| Error::FileIo {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        std::fs::write(path, contents).map_err(|source| Error::FileIo {
            path: path.to_path_buf(),
            source,
        })?;
        Ok(true)
    }
}
