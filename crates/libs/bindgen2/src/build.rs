use crate::{Filter, Layout, Metadata, Request, format};
use std::path::{Path, PathBuf};
use windows_metadata2::{Database, Image};

/// Creates a build-script facade over the bindgen2 projection engine.
pub fn builder() -> Bindgen {
    Bindgen::new()
}

/// Collects metadata inputs and writes one generated Rust file.
#[derive(Default)]
pub struct Bindgen {
    inputs: Vec<PathBuf>,
    input_default: bool,
    output: PathBuf,
    filters: Vec<String>,
    layout: Layout,
    sys: bool,
    implement_all: bool,
}

impl Bindgen {
    /// Creates an empty generation request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a metadata file or a directory containing metadata files.
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds the default WinRT and Win32 metadata.
    pub fn input_default(&mut self) -> &mut Self {
        self.input_default = true;
        self
    }

    /// Sets the generated Rust file.
    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    /// Adds one metadata filter path.
    pub fn filter(&mut self, filter: impl AsRef<str>) -> &mut Self {
        self.filters.push(filter.as_ref().to_string());
        self
    }

    /// Adds metadata filter paths.
    pub fn filters<I, S>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filters.extend(
            filters
                .into_iter()
                .map(|filter| filter.as_ref().to_string()),
        );
        self
    }

    /// Emits raw ABI-oriented bindings.
    pub fn sys(&mut self) -> &mut Self {
        self.sys = true;
        self
    }

    /// Emits one flat list of items.
    pub fn flat(&mut self) -> &mut Self {
        self.layout = Layout::Flat;
        self
    }

    /// Emits implementation traits for every selected interface.
    pub fn implement_all(&mut self) -> &mut Self {
        self.implement_all = true;
        self
    }

    /// Generates, formats, and writes the requested bindings.
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.output.as_os_str().is_empty() {
            return Err("output is required".into());
        }
        if self.filters.is_empty() {
            return Err("at least one filter is required".into());
        }

        let database = Database::new(self.images()?)?;
        let mut filter = Filter::new();
        for path in &self.filters {
            filter.include_path(&database, path)?;
        }

        let metadata = Metadata::new(database)?;
        let mut request = if self.implement_all {
            Request::filtered(filter).implement_all()
        } else {
            Request::filtered(filter)
        };
        if self.sys {
            request = request.sys();
        }
        let tokens = metadata.generator(request)?.render(self.layout)?;
        let contents = format(&tokens.to_string())?;

        if std::fs::read_to_string(&self.output).is_ok_and(|existing| existing == contents) {
            return Ok(());
        }
        if let Some(parent) = self.output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.output, contents)?;
        Ok(())
    }

    fn images(&self) -> Result<Vec<Image>, Box<dyn std::error::Error>> {
        let mut images = Vec::new();
        for input in &self.inputs {
            if input.is_dir() {
                let mut paths = std::fs::read_dir(input)?
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.is_file()
                            && path
                                .extension()
                                .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                    })
                    .collect::<Vec<_>>();
                paths.sort();
                if paths.is_empty() {
                    return Err(format!(
                        "no .winmd files found in directory `{}`",
                        input.display()
                    )
                    .into());
                }
                for path in paths {
                    images.push(Image::read(path)?);
                }
            } else {
                images.push(Image::read(input)?);
            }
        }

        if self.inputs.is_empty() || self.input_default {
            images.push(Image::new(windows_default::WINRT)?);
            images.push(Image::new(windows_default::WIN32)?);
        }
        Ok(images)
    }
}
