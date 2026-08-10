use super::*;
use std::path::{Path, PathBuf};

/// A simple flat sys request compatible with the current `tool_bindings` request files.
pub struct SysRequest {
    output: PathBuf,
    filter: Filter,
}

impl SysRequest {
    /// Reads a request containing `--out`, `--flat`, `--sys`, and bare `--filter` names.
    pub fn read(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path).map_err(|source| Error::RequestIo {
            path: path.to_path_buf(),
            source,
        })?;
        let mut output = None;
        let mut flat = false;
        let mut sys = false;
        let mut reading_filters = false;
        let mut filters = Vec::new();

        for (index, source_line) in source.lines().enumerate() {
            let line_number = index + 1;
            let line = source_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if reading_filters && !line.starts_with("--") {
                if !line
                    .bytes()
                    .all(|byte| byte == b'_' || byte.is_ascii_alphanumeric())
                {
                    return Err(Self::invalid(
                        path,
                        line_number,
                        format!("unsupported filter `{line}`"),
                    ));
                }
                filters.push(line.to_string());
                continue;
            }

            reading_filters = false;
            let mut arguments = line.split_whitespace();
            while let Some(argument) = arguments.next() {
                match argument {
                    "--out" => {
                        let value = arguments.next().ok_or_else(|| {
                            Self::invalid(path, line_number, "`--out` requires a path")
                        })?;
                        if output
                            .replace((PathBuf::from(value), line_number))
                            .is_some()
                        {
                            return Err(Self::invalid(
                                path,
                                line_number,
                                "`--out` may appear only once",
                            ));
                        }
                    }
                    "--flat" => flat = true,
                    "--sys" => sys = true,
                    "--filter" => {
                        if arguments.next().is_some() {
                            return Err(Self::invalid(
                                path,
                                line_number,
                                "`--filter` names must appear on following lines",
                            ));
                        }
                        reading_filters = true;
                    }
                    unsupported => {
                        return Err(Self::invalid(
                            path,
                            line_number,
                            format!("unsupported option `{unsupported}`"),
                        ));
                    }
                }
            }
        }

        let (output, output_line) =
            output.ok_or_else(|| Self::invalid(path, 0, "missing `--out`"))?;
        if output.as_os_str().is_empty()
            || output
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(Self::invalid(
                path,
                output_line,
                "`--out` must be a relative path without parent traversal",
            ));
        }
        if !flat {
            return Err(Self::invalid(path, 0, "missing `--flat`"));
        }
        if !sys {
            return Err(Self::invalid(path, 0, "missing `--sys`"));
        }
        Ok(Self {
            output,
            filter: Filter::names(filters),
        })
    }

    /// Returns the request's output path.
    pub fn output(&self) -> &Path {
        &self.output
    }

    /// Creates a flat generator for this request.
    pub fn generator(&self, metadata: &Metadata) -> Result<Generator, Error> {
        metadata.generator_with_filter(
            Options {
                layout: Layout::Flat,
            },
            self.filter.clone(),
        )
    }

    /// Generates this request under the supplied output root.
    pub fn write(&self, metadata: &Metadata, root: impl AsRef<Path>) -> Result<bool, Error> {
        self.generator(metadata)?
            .write_file(root.as_ref().join(&self.output))
    }

    fn invalid(path: &Path, line: usize, message: impl Into<String>) -> Error {
        Error::InvalidRequest {
            path: path.to_path_buf(),
            line,
            message: message.into(),
        }
    }
}
