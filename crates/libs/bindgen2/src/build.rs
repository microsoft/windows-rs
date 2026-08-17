use crate::{Filter, Layout, Metadata, Request};
use std::path::{Path, PathBuf};
use windows_metadata2::{Database, Image};

/// Creates a build-script facade over the bindgen2 projection engine.
pub fn builder() -> Bindgen {
    Bindgen::new()
}

/// Creates a build request from a legacy bindgen command file.
pub fn command_file(path: impl AsRef<Path>) -> Result<Bindgen, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut tokens = Vec::<String>::new();
    let mut grouped = String::new();
    for token in contents
        .lines()
        .map(|line| line.split_once("//").map_or(line, |(line, _)| line))
        .flat_map(str::split_whitespace)
    {
        if grouped.is_empty() && !token.contains('{') {
            tokens.push(token.to_string());
            continue;
        }
        if !grouped.is_empty() {
            grouped.push(' ');
        }
        grouped.push_str(token);
        if grouped.matches('{').count() == grouped.matches('}').count() {
            tokens.push(std::mem::take(&mut grouped));
        }
    }
    if !grouped.is_empty() {
        return Err("unterminated filter group".into());
    }
    let mut bindgen = Bindgen::new();
    let mut position = 0;

    while position < tokens.len() {
        let option = tokens[position].as_str();
        position += 1;
        let start = position;
        while position < tokens.len() && !tokens[position].starts_with("--") {
            position += 1;
        }
        let values = &tokens[start..position];

        match option {
            "--in" => {
                for value in values {
                    if value == "default" {
                        bindgen.input_default();
                    } else {
                        bindgen.input(value);
                    }
                }
            }
            "--out" if values.len() == 1 => {
                bindgen.output(&values[0]);
            }
            "--filter" => {
                bindgen.filters(values);
            }
            "--implement" if values.is_empty() => {
                bindgen.implement_all();
            }
            "--implement" => {
                bindgen.implements(values);
            }
            "--derive" => {
                bindgen.derives(values);
            }
            "--flat" if values.is_empty() => {
                bindgen.flat();
            }
            "--package" if values.is_empty() => {
                bindgen.package();
            }
            "--rustfmt" if values.len() == 1 => {
                bindgen.rustfmt(&values[0]);
            }
            "--sys" if values.is_empty() => {
                bindgen.sys();
            }
            "--minimal" if values.is_empty() => {
                bindgen.minimal();
            }
            "--dead-code" if values.is_empty() => {
                bindgen.dead_code();
            }
            "--preserve-field-names" if values.is_empty() => {
                bindgen.preserve_field_names();
            }
            _ => return Err(format!("unsupported bindgen option `{option}`").into()),
        }
    }

    Ok(bindgen)
}

/// Collects metadata inputs and writes one generated Rust file.
#[derive(Default)]
pub struct Bindgen {
    inputs: Vec<PathBuf>,
    input_default: bool,
    output: PathBuf,
    filters: Vec<String>,
    filter_files: Vec<PathBuf>,
    implementations: Vec<String>,
    derives: Vec<String>,
    layout: Layout,
    rustfmt: Option<String>,
    sys: bool,
    minimal: bool,
    dead_code: bool,
    preserve_field_names: bool,
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

    /// Adds metadata files or directories.
    pub fn inputs<I, P>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.inputs
            .extend(inputs.into_iter().map(|input| input.as_ref().to_path_buf()));
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

    /// Adds filters from a text file containing one path per line.
    pub fn filter_file(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.filter_files.push(path.as_ref().to_path_buf());
        self
    }

    /// Adds filters from text files containing one path per line.
    pub fn filter_files<I, P>(&mut self, paths: I) -> &mut Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        self.filter_files
            .extend(paths.into_iter().map(|path| path.as_ref().to_path_buf()));
        self
    }

    /// Selects interfaces that require implementation traits and typed ABI vtables.
    pub fn implements<I, S>(&mut self, implementations: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.implementations.extend(
            implementations
                .into_iter()
                .map(|implementation| implementation.as_ref().to_string()),
        );
        self
    }

    /// Adds native type derives in `Type=Trait` form.
    pub fn derives<I, S>(&mut self, derives: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.derives.extend(
            derives
                .into_iter()
                .map(|derive| derive.as_ref().to_string()),
        );
        self
    }

    /// Emits raw ABI-oriented bindings.
    pub fn sys(&mut self) -> &mut Self {
        self.sys = true;
        self
    }

    /// Emits the focused minimal projection.
    pub fn minimal(&mut self) -> &mut Self {
        self.minimal = true;
        self
    }

    /// Retains compatibility with legacy build scripts that request dead-code output.
    pub fn dead_code(&mut self) -> &mut Self {
        self.dead_code = true;
        self
    }

    /// Preserves WinRT metadata field names instead of converting them to Rust style.
    pub fn preserve_field_names(&mut self) -> &mut Self {
        self.preserve_field_names = true;
        self
    }

    /// Emits one flat list of items.
    pub fn flat(&mut self) -> &mut Self {
        assert_ne!(
            self.layout,
            Layout::Package,
            "cannot combine package and flat"
        );
        self.layout = Layout::Flat;
        self
    }

    /// Emits one generated file per metadata namespace.
    pub fn package(&mut self) -> &mut Self {
        assert_ne!(self.layout, Layout::Flat, "cannot combine package and flat");
        self.layout = Layout::Package;
        self
    }

    /// Overrides the rustfmt configuration used for generated files.
    pub fn rustfmt(&mut self, rustfmt: impl Into<String>) -> &mut Self {
        self.rustfmt = Some(rustfmt.into());
        self
    }

    /// Emits implementation traits for every selected interface.
    pub fn implement_all(&mut self) -> &mut Self {
        self.implement_all = true;
        self
    }

    /// Generates, formats, and writes the requested bindings.
    pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let total = std::time::Instant::now();
        if self.output.as_os_str().is_empty() {
            return Err("output is required".into());
        }
        if self.filters.is_empty() && self.filter_files.is_empty() {
            return Err("at least one filter is required".into());
        }

        let phase = std::time::Instant::now();
        let database = Database::new(self.images()?)?;
        report_timing(&self.output, "metadata database", phase.elapsed());
        let mut filter = Filter::new();
        for path in &self.filters {
            filter.include_path(&database, path)?;
        }
        for file in &self.filter_files {
            for line in std::fs::read_to_string(file)?.lines().map(str::trim) {
                if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                    continue;
                }
                filter.include_path(&database, line)?;
            }
        }
        let implementations = if self.implementations.is_empty() {
            None
        } else {
            let mut implementations = Filter::new();
            for path in &self.implementations {
                implementations.include_path(&database, path)?;
            }
            Some(implementations)
        };

        let phase = std::time::Instant::now();
        let metadata = Metadata::new(database)?;
        report_timing(&self.output, "metadata catalogs", phase.elapsed());
        let mut request = if self.implement_all {
            Request::filtered(filter).implement_all()
        } else if let Some(implementations) = implementations {
            Request::filtered(filter).implementations(implementations)
        } else {
            Request::filtered(filter)
        };
        if self.preserve_field_names || self.layout.is_package() {
            request = request.preserve_field_names();
        }
        if self.sys {
            request = request.sys();
        } else if self.minimal {
            request = if self.dead_code {
                request.minimal()
            } else {
                request.minimal_public()
            };
        }
        for derive in &self.derives {
            let (name, derive) = derive
                .split_once('=')
                .ok_or("derives must use `Type=Trait` form")?;
            request = request.derive(name, derive);
        }
        if self.layout.is_package() {
            request = request.package();
        }
        let phase = std::time::Instant::now();
        let generator = metadata.generator(request)?;
        report_timing(&self.output, "selection", phase.elapsed());
        if self.layout.is_package() {
            let phase = std::time::Instant::now();
            let plan = generator.package_plan(self.sys)?;
            report_timing(&self.output, "package planning", phase.elapsed());
            let phase = std::time::Instant::now();
            self.write_package(plan)?;
            report_timing(&self.output, "package output", phase.elapsed());
            report_timing(&self.output, "total", total.elapsed());
            return Ok(());
        }
        let tokens = generator.render(self.layout)?;
        let contents =
            crate::format::format_with_config(&tokens.to_string(), self.rustfmt.as_deref())?;

        write_if_changed(&self.output, contents)?;
        report_timing(&self.output, "total", total.elapsed());
        Ok(())
    }

    fn write_package(
        &self,
        plan: crate::output::PackagePlan,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for path in plan.removals {
            let _ = std::fs::remove_dir_all(self.output.join(path));
        }
        let modules = plan
            .modules
            .into_iter()
            .map(|module| (module.path, module.tokens.to_string()))
            .collect::<Vec<_>>();
        let output = &self.output;
        let rustfmt = self.rustfmt.as_deref();
        std::thread::scope(|scope| -> Result<(), Box<dyn std::error::Error>> {
            let mut handles = Vec::with_capacity(modules.len());
            for (path, tokens) in modules {
                handles.push(scope.spawn(move || -> Result<(), String> {
                    let contents = crate::format::format_with_config(&tokens, rustfmt)
                        .map_err(|error| error.to_string())?;
                    write_if_changed(&output.join(path), contents)
                        .map_err(|error| error.to_string())
                }));
            }
            for handle in handles {
                handle
                    .join()
                    .map_err(|_| "package formatting worker panicked")?
                    .map_err(|error| -> Box<dyn std::error::Error> { error.into() })?;
            }
            Ok(())
        })?;

        let toml_path = self.output.join("Cargo.toml");
        let existing = std::fs::read_to_string(&toml_path)?;
        let Some((prefix, _)) = existing.split_once("# generated features") else {
            return Err(format!(
                "missing `# generated features` marker in `{}`",
                toml_path.display()
            )
            .into());
        };
        let mut toml = format!("{prefix}# generated features\n");
        for feature in plan.features {
            toml.push_str(&feature);
            toml.push('\n');
        }
        write_if_changed(&toml_path, toml)
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

fn write_if_changed(path: &Path, contents: String) -> Result<(), Box<dyn std::error::Error>> {
    if std::fs::read_to_string(path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, contents)?;
    Ok(())
}

fn report_timing(output: &Path, phase: &str, elapsed: std::time::Duration) {
    crate::report_timing(&format!("`{}` {phase}", output.display()), elapsed);
}
