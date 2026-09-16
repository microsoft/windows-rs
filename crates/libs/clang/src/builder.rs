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
    reference_bytes: Vec<Vec<u8>>,
    reference_default: bool,
    output: PathBuf,
    namespace: String,
    args: Vec<String>,
    target: Option<String>,
    library: String,
    filters: BTreeSet<String>,
    functions: BTreeSet<String>,
}

impl Clang {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.inputs.push(input.as_ref().to_path_buf());
        self
    }

    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        self.inputs
            .extend(inputs.into_iter().map(|input| input.as_ref().to_path_buf()));
        self
    }

    pub fn input_text(&mut self, input: impl Into<String>) -> &mut Self {
        self.input_text.push(input.into());
        self
    }

    pub fn input_texts<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.input_text.extend(inputs.into_iter().map(Into::into));
        self
    }

    pub fn reference(&mut self, reference: impl AsRef<Path>) -> &mut Self {
        self.references.push(reference.as_ref().to_path_buf());
        self
    }

    pub fn references<I, S>(&mut self, references: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        self.references.extend(
            references
                .into_iter()
                .map(|reference| reference.as_ref().to_path_buf()),
        );
        self
    }

    pub fn reference_bytes(&mut self, reference: impl AsRef<[u8]>) -> &mut Self {
        self.reference_bytes.push(reference.as_ref().to_vec());
        self
    }

    pub fn reference_byte_sets<I, B>(&mut self, references: I) -> &mut Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        self.reference_bytes.extend(
            references
                .into_iter()
                .map(|reference| reference.as_ref().to_vec()),
        );
        self
    }

    pub fn reference_default(&mut self) -> &mut Self {
        self.reference_default = true;
        self
    }

    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    pub fn namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into();
        self
    }

    pub fn library(&mut self, library: impl Into<String>) -> &mut Self {
        self.library = library.into();
        self
    }

    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn target(&mut self, target: impl Into<String>) -> &mut Self {
        self.target = Some(target.into());
        self
    }

    pub fn filter(&mut self, filter: impl Into<String>) -> &mut Self {
        self.filters.insert(normalize_name(&filter.into()));
        self
    }

    pub fn filters<I, S>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for filter in filters {
            self.filter(filter);
        }
        self
    }

    pub fn symbol(&mut self, symbol: impl Into<String>) -> &mut Self {
        self.functions.insert(symbol.into());
        self
    }

    pub fn symbols<I, S>(&mut self, symbols: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.functions.extend(symbols.into_iter().map(Into::into));
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        let snapshot = self.snapshot()?;
        let references = self.load_references()?;
        let excluded_types = references.keys().cloned().collect();
        let mut options = EmitOptions::new(&self.namespace, &references);
        options.library = (!self.library.is_empty()).then_some(self.library.as_str());
        options.excluded_types = Some(&excluded_types);
        options.functions = (!self.functions.is_empty()).then_some(&self.functions);
        let rdl = snapshot.emit_with_options(&options)?;
        write_file(&self.output, rdl)
    }

    pub fn write_by_header(&self) -> Result<(), Error> {
        let snapshot = self.snapshot()?;
        let references = self.load_references()?;
        let excluded_types = references.keys().cloned().collect();
        let mut options = EmitOptions::new(&self.namespace, &references);
        options.library = (!self.library.is_empty()).then_some(self.library.as_str());
        options.excluded_types = Some(&excluded_types);
        options.functions = (!self.functions.is_empty()).then_some(&self.functions);
        for (header, rdl) in snapshot.emit_by_header_with_options(&options)? {
            let stem = Path::new(&header)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| Error(format!("header has no file stem: {header}")))?
                .to_lowercase();
            write_file(&self.output.join(stem).with_extension("rdl"), rdl)?;
        }
        Ok(())
    }

    fn snapshot(&self) -> Result<Snapshot, Error> {
        if self.namespace.is_empty() {
            return Err(Error("namespace is required".to_string()));
        }
        if self.output.as_os_str().is_empty() {
            return Err(Error("output is required".to_string()));
        }

        let mut paths = vec![];
        for input in &self.inputs {
            collect_files(input, "header", is_header, &mut paths)?;
        }
        paths.sort();
        paths.dedup();

        let mut inputs = Vec::with_capacity(paths.len() + self.input_text.len());
        for path in paths {
            let source = std::fs::read_to_string(&path)
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

        let mut args = Vec::with_capacity(self.args.len() + usize::from(self.target.is_some()));
        if let Some(target) = &self.target {
            args.push(format!("--target={target}"));
        }
        args.extend(self.args.iter().cloned());
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        extract(inputs, &args)
    }

    fn load_references(&self) -> Result<BTreeMap<String, TypeReference>, Error> {
        let mut bytes = self.reference_bytes.clone();
        if self.reference_default {
            bytes.push(windows_default::WINRT.to_vec());
            bytes.push(windows_default::WIN32.to_vec());
        }

        let mut paths = vec![];
        for reference in &self.references {
            collect_files(reference, "WinMD", is_winmd, &mut paths)?;
        }
        paths.sort();
        paths.dedup();
        for path in paths {
            bytes.push(
                std::fs::read(&path).map_err(|error| {
                    Error(format!("failed to read {}: {error}", path.display()))
                })?,
            );
        }

        let files = bytes
            .into_iter()
            .map(|bytes| {
                windows_metadata::reader::File::new(bytes)
                    .ok_or_else(|| Error("failed to read WinMD".to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(type_references(files))
    }
}

fn type_references(files: Vec<windows_metadata::reader::File>) -> BTreeMap<String, TypeReference> {
    let index = windows_metadata::reader::Index::new(files);
    let mut references = BTreeMap::new();
    let mut ambiguous = BTreeSet::new();
    for (namespace, name, ty) in index.iter() {
        let kind = match ty.category() {
            windows_metadata::reader::TypeCategory::Enum => TypeReferenceKind::Enum,
            windows_metadata::reader::TypeCategory::Interface => TypeReferenceKind::Interface,
            _ => TypeReferenceKind::Type,
        };
        let mut reference = TypeReference::new(namespace, name, kind);
        if kind == TypeReferenceKind::Enum {
            reference = reference.with_enum_members(ty.fields().map(|field| field.name()));
        }
        if references
            .insert(name.to_string(), reference.clone())
            .is_some_and(|existing| existing != reference)
        {
            ambiguous.insert(name.to_string());
        }
    }
    references.retain(|name, _| !ambiguous.contains(name));
    references
}

fn collect_files(
    path: &Path,
    kind: &str,
    include: fn(&Path) -> bool,
    result: &mut Vec<PathBuf>,
) -> Result<(), Error> {
    if path.is_file() {
        if include(path) {
            result.push(path.to_path_buf());
            return Ok(());
        }
        return Err(Error(format!("{} is not a {kind} file", path.display())));
    }
    if path.is_dir() {
        let initial_len = result.len();
        for entry in std::fs::read_dir(path)
            .map_err(|error| Error(format!("failed to read {}: {error}", path.display())))?
        {
            let entry =
                entry.map_err(|error| Error(format!("failed to read directory entry: {error}")))?;
            let path = entry.path();
            if path.is_file() && include(&path) {
                result.push(path);
            }
        }
        if result.len() == initial_len {
            return Err(Error(format!(
                "failed to find {kind} files in {}",
                path.display()
            )));
        }
        return Ok(());
    }
    Err(Error(format!("{} does not exist", path.display())))
}

fn is_header(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "h" | "hh" | "hpp" | "hxx"
            )
        })
}

fn is_winmd(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
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
