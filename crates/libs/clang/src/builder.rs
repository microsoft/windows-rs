use super::*;
use std::path::{Path, PathBuf};

/// Creates a high-level RDL generator backed by [`extract`].
pub fn clang() -> Clang {
    Clang::new()
}

/// High-level adapter for common header-to-RDL generation.
///
/// Use [`extract`] directly when the caller needs to inspect or combine snapshots before emission.
#[derive(Default)]
pub struct Clang {
    inputs: Vec<PathBuf>,
    input_text: Vec<String>,
    references: Vec<PathBuf>,
    reference_default: bool,
    output: PathBuf,
    namespace: String,
    args: Vec<String>,
    library: String,
    filters: BTreeSet<String>,
    functions: BTreeSet<String>,
}

impl Clang {
    /// Creates an empty generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a header file as a translation unit.
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds header files as translation units.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        self.inputs
            .extend(inputs.into_iter().map(|input| input.as_ref().to_path_buf()));
        self
    }

    /// Adds an in-memory translation unit.
    pub fn input_text(&mut self, input: impl Into<String>) -> &mut Self {
        self.input_text.push(input.into());
        self
    }

    /// Adds a WinMD file used to resolve and exclude existing declarations.
    pub fn reference(&mut self, reference: impl AsRef<Path>) -> &mut Self {
        self.references.push(reference.as_ref().to_path_buf());
        self
    }

    /// Adds the default Windows Runtime and Win32 metadata references.
    pub fn reference_default(&mut self) -> &mut Self {
        self.reference_default = true;
        self
    }

    /// Sets the output RDL file.
    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    /// Sets the namespace containing the emitted declarations.
    pub fn namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into();
        self
    }

    /// Sets the import library attached to emitted functions.
    pub fn library(&mut self, library: impl Into<String>) -> &mut Self {
        self.library = library.into();
        self
    }

    /// Adds compiler arguments passed to libclang.
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Includes declarations from headers matching this path suffix.
    pub fn filter(&mut self, filter: impl Into<String>) -> &mut Self {
        self.filters.insert(normalize_name(&filter.into()));
        self
    }

    /// Restricts emitted free functions to the supplied names.
    pub fn symbols<I, S>(&mut self, symbols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.functions.extend(symbols.into_iter().map(Into::into));
        self
    }

    /// Extracts the configured inputs and writes one RDL file.
    pub fn write(&self) -> Result<(), Error> {
        let snapshot = self.snapshot()?;
        let references = self.load_references()?;
        let mut options = EmitOptions::new(&self.namespace, references.types());
        options.library = (!self.library.is_empty()).then_some(self.library.as_str());
        references.apply_reference_exclusions(&mut options);
        options.functions = (!self.functions.is_empty()).then_some(&self.functions);
        let rdl = snapshot.emit_with_options(&options)?;
        write_file(&self.output, rdl)
    }

    fn snapshot(&self) -> Result<Snapshot, Error> {
        if self.namespace.is_empty() {
            return Err(Error("namespace is required".to_string()));
        }
        if self.output.as_os_str().is_empty() {
            return Err(Error("output is required".to_string()));
        }

        let mut inputs = Vec::with_capacity(self.inputs.len() + self.input_text.len());
        for path in &self.inputs {
            let source = std::fs::read_to_string(path)
                .map_err(|error| Error(format!("failed to read {}: {error}", path.display())))?;
            inputs.push(
                Input::new(path.to_string_lossy(), source)
                    .with_root_suffixes(self.filters.iter().cloned()),
            );
        }
        for (index, source) in self.input_text.iter().enumerate() {
            inputs.push(
                Input::new(format!("windows-clang-{index}.h"), source)
                    .with_root_suffixes(self.filters.iter().cloned()),
            );
        }
        if inputs.is_empty() {
            return Err(Error("input is required".to_string()));
        }

        let args: Vec<&str> = self.args.iter().map(String::as_str).collect();
        extract(inputs, &args)
    }

    fn load_references(&self) -> Result<MetadataReferences, Error> {
        let mut files = Vec::with_capacity(self.references.len() + 2);
        if self.reference_default {
            files.push(
                windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
            );
            files.push(
                windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
            );
        }

        for reference in &self.references {
            let bytes = std::fs::read(reference).map_err(|error| {
                Error(format!("failed to read {}: {error}", reference.display()))
            })?;
            files.push(
                windows_metadata::reader::File::new(bytes).ok_or_else(|| {
                    Error(format!("failed to read WinMD {}", reference.display()))
                })?,
            );
        }
        Ok(MetadataReferences::new(files))
    }
}

fn write_file(path: &Path, contents: String) -> Result<(), Error> {
    if path.as_os_str().is_empty() {
        return Err(Error("output is required".to_string()));
    }
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| Error(format!("failed to create {}: {error}", parent.display())))?;
    }
    std::fs::write(path, contents)
        .map_err(|error| Error(format!("failed to write {}: {error}", path.display())))
}
