#![doc = include_str!("../readme.md")]

mod cli;
mod config;
mod derive;
mod derive_writer;
mod filter;
mod format;
mod guid;
mod implements;
mod io;
mod package_writer;
mod param;
mod paths;
mod references;
mod signature;
mod tables;
mod tokens;
mod type_map;
mod type_name;
mod type_tree;
mod types;
mod value;
mod winmd;

pub use cli::bindgen;
use config::*;
use derive::*;
use derive_writer::*;
use filter::*;
use guid::*;
use implements::*;
use io::*;
use package_writer::*;
use param::*;
use references::*;
use signature::*;
use std::cmp::Ordering;
use std::collections::*;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use tables::*;
use tokens::*;
use type_map::*;
use type_name::*;
use type_tree::*;
use types::*;
use value::*;
use winmd::*;
mod filter_parser;
mod method_names;
mod type_closure;
use method_names::*;
use type_closure::*;

fn report_timing(output: &Path, phase: &str, elapsed: std::time::Duration) {
    if std::env::var_os("WINDOWS_BINDGEN_TIMINGS").is_some() {
        eprintln!(
            "windows-bindgen timing `{}` {phase}: {:.3} ms",
            output.display(),
            elapsed.as_secs_f64() * 1_000.0
        );
    }
}

/// Creates a new [`Bindgen`] builder for generating Windows API bindings.
pub fn builder() -> Bindgen {
    Bindgen::new()
}

/// Builder for generating Windows API bindings.
///
/// This is the fluent alternative to [`bindgen`].
///
/// # Example
///
/// ```rust,no_run
/// windows_bindgen::Bindgen::new()
///     .output("src/bindings.rs")
///     .filter("GetTickCount")
///     .write();
/// ```
#[derive(Default)]
pub struct Bindgen {
    input: Vec<Input>,
    input_default: bool,
    filter: Vec<String>,
    output: PathBuf,
    derive: Vec<String>,
    implement: Option<Vec<String>>,
    rustfmt: Option<String>,
    layout: Layout,
    style: Style,
    dead_code: bool,
}

enum Input {
    Path(PathBuf),
    Bytes(Vec<u8>),
}

/// Output layout for the generated bindings.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// One Rust module per metadata namespace (the default).
    #[default]
    Modules,
    /// A single flat list of items (no namespace modules).
    Flat,
    /// One file per namespace + `Cargo.toml` features.
    Package,
}

impl Layout {
    fn is_flat(self) -> bool {
        matches!(self, Self::Flat)
    }
    fn is_package(self) -> bool {
        matches!(self, Self::Package)
    }
}

/// Code-style mode for the generated bindings.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Style {
    /// Full-fidelity bindings (the default).
    #[default]
    Default,
    /// Raw / sys-style bindings.
    Sys {
        /// When `true`, emit `extern { fn ... }` instead of `link!` macros.
        extern_fns: bool,
    },
    /// Minimal-mode bindings (drop class wrappers, inherited forwarders,
    /// handle ergonomics; auto-revoke events).
    Minimal,
}

impl Style {
    fn is_sys(self) -> bool {
        matches!(self, Self::Sys { .. })
    }
    fn is_minimal(self) -> bool {
        matches!(self, Self::Minimal)
    }
    fn sys_fn_extern(self) -> bool {
        matches!(self, Self::Sys { extern_fns: true })
    }

    /// Minimal bindings use the class's default interface directly.
    fn emit_class_methods(self) -> bool {
        !self.is_minimal()
    }

    /// Minimal bindings require casting to the interface that owns an inherited method.
    fn emit_inherited_forwarders(self) -> bool {
        !self.is_minimal()
    }

    /// Minimal bindings require casting inherited iterables to `IIterable<T>`.
    fn emit_iterable_into_iterator(self) -> bool {
        !self.is_minimal()
    }

    /// Minimal bindings expose input strings as `&str`.
    fn minimal_string_input(self, param: &Param) -> bool {
        self.is_minimal() && param.is_input_only() && matches!(param.ty, Type::String)
    }

    /// Minimal bindings return strings as `String`.
    fn minimal_string_return(self, ty: &Type) -> bool {
        self.is_minimal() && matches!(ty, Type::String)
    }

    /// Sys bindings omit standard derives beyond `Copy` and `Clone`.
    fn derive_std_traits(self) -> bool {
        !self.is_sys()
    }

    /// Sys bindings omit traits that require `windows-core`.
    fn emit_core_traits(self) -> bool {
        !self.is_sys()
    }

    /// Whether handle structs are emitted as bare aliases rather than newtypes.
    fn emit_bare_typedef(self) -> bool {
        self.is_sys() || self.is_minimal()
    }
}

impl Bindgen {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a `.winmd` file or directory.
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.input.push(Input::Path(input.as_ref().to_path_buf()));
        self
    }

    /// Adds the default Windows metadata.
    pub fn input_default(&mut self) -> &mut Self {
        self.input_default = true;
        self
    }

    /// Adds a `.winmd` file from memory.
    pub fn input_bytes(&mut self, input: &[u8]) -> &mut Self {
        self.input.push(Input::Bytes(input.to_vec()));
        self
    }

    /// Adds `.winmd` files from memory.
    pub fn input_byte_sets<I, B>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        for input in inputs {
            self.input_bytes(input.as_ref());
        }
        self
    }

    /// Adds `.winmd` files or directories.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Sets the generated Rust file.
    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    /// Add a filter rule to include or exclude APIs.
    ///
    /// Filter rules may be a function or type name, a namespace prefix, a fully-qualified name,
    /// or a method-level entry of the form `Namespace.Type::Method` (with optional `Property` /
    /// `Event` sugar). Prefix with `!` to exclude rather than include. See the crate-level
    /// docs for the full grammar.
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filters(std::iter::once(filter))
    }

    /// Adds filter rules from a text file.
    #[track_caller]
    pub fn filter_file(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.filters(cli::read_tokens(input))
    }

    /// Adds filter rules from text files.
    #[track_caller]
    pub fn filter_files<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.filter_file(input);
        }
        self
    }

    /// Add multiple filter rules to include or exclude APIs.
    ///
    /// Filter rules may be a function or type name, a namespace prefix, a fully-qualified name,
    /// or a method-level entry of the form `Namespace.Type::Method` (with optional `Property` /
    /// `Event` sugar). Prefix with `!` to exclude rather than include. See the crate-level
    /// docs for the full grammar.
    pub fn filters<I, S>(&mut self, filters: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for filter in filters {
            self.filter.push(filter.as_ref().to_string());
        }
        self
    }

    /// Add an extra trait for types to derive.
    pub fn derive(&mut self, derive: &str) -> &mut Self {
        self.derives(std::iter::once(derive))
    }

    /// Add multiple extra traits for types to derive.
    pub fn derives<I, S>(&mut self, derives: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for derive in derives {
            self.derive.push(derive.as_ref().to_string());
        }
        self
    }

    /// Override the default Rust formatter path.
    pub fn rustfmt(&mut self, rustfmt: &str) -> &mut Self {
        self.rustfmt = Some(rustfmt.to_string());
        self
    }

    /// Avoid the default namespace-to-module conversion.
    #[track_caller]
    pub fn flat(&mut self) -> &mut Self {
        if matches!(self.layout, Layout::Package) {
            panic!("cannot combine `--package` and `--flat`");
        }
        self.layout = Layout::Flat;
        self
    }

    fn uses_inline_core_types(&self) -> bool {
        self.style.is_sys() && !self.layout.is_package()
    }

    /// Generate bindings as a package with one file per namespace.
    #[track_caller]
    pub fn package(&mut self) -> &mut Self {
        if matches!(self.layout, Layout::Flat) {
            panic!("cannot combine `--package` and `--flat`");
        }
        self.layout = Layout::Package;
        self
    }

    /// Includes implementation traits for every WinRT interface in scope.
    #[track_caller]
    pub fn implement_all(&mut self) -> &mut Self {
        match &self.implement {
            None => self.implement = Some(vec![]),
            Some(names) if names.is_empty() => {}
            Some(_) => panic!("cannot combine `implement_all` with selected implementations"),
        }
        self
    }

    /// Includes implementation traits for a WinRT interface or namespace prefix.
    ///
    /// The name may be a fully-qualified type name (`Namespace.Name`) or a namespace prefix that
    /// matches every type defined under it.
    #[track_caller]
    pub fn implement(&mut self, name: &str) -> &mut Self {
        assert!(
            !self.implement.as_ref().is_some_and(Vec::is_empty),
            "cannot combine selected implementations with `implement_all`"
        );
        self.implement
            .get_or_insert_with(Vec::new)
            .push(name.to_string());
        self
    }

    /// Includes implementation traits for multiple WinRT interfaces or namespace prefixes.
    pub fn implements<I, S>(&mut self, names: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for name in names {
            self.implement(name.as_ref());
        }
        self
    }

    /// Generate raw or sys-style Rust bindings.
    ///
    /// Mutually exclusive with [`Bindgen::minimal`]; panics if `minimal` was
    /// already selected.
    #[track_caller]
    pub fn sys(&mut self) -> &mut Self {
        let extern_fns = matches!(self.style, Style::Sys { extern_fns: true });
        if matches!(self.style, Style::Minimal) {
            panic!("cannot combine `--sys` and `--minimal`");
        }
        self.style = Style::Sys { extern_fns };
        self
    }

    /// Generate minimal-mode Rust bindings.
    ///
    /// Drops per-class wrapper methods, inherited interface forwarders, handle
    /// ergonomics, and free-function wrappers.
    ///
    /// Mutually exclusive with `--sys`.
    #[track_caller]
    pub fn minimal(&mut self) -> &mut Self {
        if matches!(self.style, Style::Sys { .. }) {
            panic!("cannot combine `--sys` and `--minimal`");
        }
        self.style = Style::Minimal;
        self
    }

    /// Generate `extern` declarations rather than `link!` macros for sys-style Rust bindings.
    ///
    /// Only valid in combination with [`Bindgen::sys`]; panics otherwise.
    #[track_caller]
    pub fn extern_fns(&mut self) -> &mut Self {
        match &mut self.style {
            Style::Sys { extern_fns } => *extern_fns = true,
            _ => panic!("`--extern` requires `--sys`"),
        }
        self
    }

    /// Emit `pub(crate)` instead of `pub` on generated items to surface unused
    /// bindings as dead-code warnings.
    pub fn dead_code(&mut self) -> &mut Self {
        self.dead_code = true;
        self
    }

    /// Generate the bindings.
    #[track_caller]
    pub fn write(&self) {
        let total = std::time::Instant::now();

        // Validate before setting up reader and reference state.
        assert!(
            !self.output.as_os_str().is_empty(),
            "output is required (call `.output()` or pass `--out`)"
        );

        let mut include: Vec<&str> = vec![];
        let mut exclude: Vec<&str> = vec![];

        for f in &self.filter {
            if let Some(rest) = f.strip_prefix('!') {
                exclude.push(rest);
            } else {
                include.push(f.as_str());
            }
        }

        assert!(!include.is_empty(), "at least one `--filter` required");

        let sys = self.style.is_sys();
        let link = if sys { "windows_link" } else { "windows_core" };

        let phase = std::time::Instant::now();
        let reader_storage;
        let reader = if self.input.is_empty() {
            default_reader()
        } else {
            reader_storage = Reader::new(expand_input(&self.input, self.input_default));
            &reader_storage
        };
        report_timing(&self.output, "metadata", phase.elapsed());

        let phase = std::time::Instant::now();
        let mut references: Vec<ReferenceStage> = Vec::new();

        if !sys {
            // Register implicit references to sibling windows-* crates present in metadata.
            for (probe_namespace, crate_name, paths) in [
                (
                    "Windows.Foundation",
                    "windows_future",
                    &[
                        "Windows.Foundation.AsyncActionCompletedHandler",
                        "Windows.Foundation.AsyncActionProgressHandler",
                        "Windows.Foundation.AsyncActionWithProgressCompletedHandler",
                        "Windows.Foundation.AsyncOperationCompletedHandler",
                        "Windows.Foundation.AsyncOperationProgressHandler",
                        "Windows.Foundation.AsyncOperationWithProgressCompletedHandler",
                        "Windows.Foundation.AsyncStatus",
                        "Windows.Foundation.IAsyncAction",
                        "Windows.Foundation.IAsyncActionWithProgress",
                        "Windows.Foundation.IAsyncInfo",
                        "Windows.Foundation.IAsyncOperation",
                        "Windows.Foundation.IAsyncOperationWithProgress",
                    ][..],
                ),
                (
                    "Windows.Foundation.Collections",
                    "windows_collections",
                    &[
                        "Windows.Foundation.Collections.CollectionChange",
                        "Windows.Foundation.Collections.IIterable",
                        "Windows.Foundation.Collections.IIterator",
                        "Windows.Foundation.Collections.IKeyValuePair",
                        "Windows.Foundation.Collections.IMap",
                        "Windows.Foundation.Collections.IMapChangedEventArgs",
                        "Windows.Foundation.Collections.IMapView",
                        "Windows.Foundation.Collections.IObservableMap",
                        "Windows.Foundation.Collections.IObservableVector",
                        "Windows.Foundation.Collections.IVector",
                        "Windows.Foundation.Collections.IVectorChangedEventArgs",
                        "Windows.Foundation.Collections.IVectorView",
                        "Windows.Foundation.Collections.MapChangedEventHandler",
                        "Windows.Foundation.Collections.VectorChangedEventHandler",
                    ][..],
                ),
                (
                    "Windows.Foundation",
                    "windows_reference",
                    &["Windows.Foundation.IReference"][..],
                ),
                (
                    "Windows.Foundation",
                    "windows_time",
                    &["Windows.Foundation.DateTime", "Windows.Foundation.TimeSpan"][..],
                ),
                (
                    "Windows.Foundation.Numerics",
                    "windows_numerics",
                    &[
                        "Windows.Foundation.Numerics.Matrix3x2",
                        "Windows.Foundation.Numerics.Matrix4x4",
                        "Windows.Foundation.Numerics.Vector2",
                        "Windows.Foundation.Numerics.Vector3",
                        "Windows.Foundation.Numerics.Vector4",
                    ][..],
                ),
            ] {
                if reader.contains_key(probe_namespace) {
                    let filtered: Vec<&str> = paths
                        .iter()
                        .copied()
                        .filter(|path| {
                            if let Some((namespace, name)) = path.rsplit_once('.')
                                && let Some(ns_map) = reader.get(namespace)
                            {
                                return ns_map.contains_key(name);
                            }
                            false
                        })
                        .collect();
                    if !filtered.is_empty() {
                        prepend_default_refs(&mut references, crate_name, &filtered);
                    }
                }
            }
        }

        let derive_str: Vec<&str> = self.derive.iter().map(|s| s.as_str()).collect();
        let implements = self.implement.as_ref().map(|names| {
            let names_str: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
            Implements::new(&names_str)
        });

        let references = References::new(reader, references);
        report_timing(&self.output, "references", phase.elapsed());

        let phase = std::time::Instant::now();
        let (filter, types) = {
            let mut all_parsed = Vec::new();
            for entry in &include {
                all_parsed.extend(filter_parser::parse_filter_entry(entry));
            }
            for entry in &exclude {
                let mut entries = filter_parser::parse_filter_entry(entry);
                for e in &mut entries {
                    e.exclude = true;
                }
                all_parsed.extend(entries);
            }
            let resolved = filter_parser::resolve_entries(reader, &all_parsed);

            let mut filter = Filter::from_resolved(reader, &resolved);

            // Precise filters use bottom-up closure; broad filters and packages scan top-down.
            let types = if !filter.has_broad_filter && !self.layout.is_package() {
                filter.uses_closure = true;
                TypeClosure::build(reader, &mut filter, &references)
            } else {
                TypeMap::filter(reader, &filter, &references, self.style.is_sys())
            };

            (filter, types)
        };
        report_timing(&self.output, "selection", phase.elapsed());

        let phase = std::time::Instant::now();
        let derive = Derive::new(reader, &types, &derive_str);
        if let Some(implements) = &implements {
            filter.validate_implements(implements);
        }

        let event_only_delegates = compute_event_only_delegates(&types, reader);

        let config = Config {
            bindgen: self,
            reader,
            types: &types,
            references: &references,
            filter: &filter,
            derive: &derive,
            implement: implements.as_ref(),
            link,
            namespace: "",
            event_only_delegates: &event_only_delegates,
            self_ty: None,
            self_generics: Vec::new(),
            prunable: std::sync::Arc::new(BTreeSet::new()),
        };

        let tree = TypeTree::new(&types);
        report_timing(&self.output, "planning", phase.elapsed());
        config.write(tree);
        report_timing(&self.output, "total", total.elapsed());
    }
}

fn default_reader() -> &'static Reader {
    static READER: std::sync::OnceLock<Reader> = std::sync::OnceLock::new();
    READER.get_or_init(|| Reader::new(default_input()))
}

#[track_caller]
fn default_input() -> Vec<File> {
    [windows_default::WINRT, windows_default::WIN32]
        .into_iter()
        .map(|bytes| File::new(bytes.to_vec()).unwrap())
        .collect()
}

fn expand_input(input: &[Input], input_default: bool) -> Vec<File> {
    #[track_caller]
    fn expand_path(result: &mut Vec<File>, path: &Path) {
        if path.is_dir() {
            let mut paths = vec![];

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("failed to read directory `{}`", path.display()))
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
                    paths.push(path);
                }
            }

            assert!(
                !paths.is_empty(),
                "failed to find .winmd files in directory `{}`",
                path.display()
            );

            for path in paths {
                let bytes = std::fs::read(&path)
                    .unwrap_or_else(|_| panic!("failed to read binary file `{}`", path.display()));
                let file = File::new(bytes)
                    .unwrap_or_else(|| panic!("failed to read .winmd format `{}`", path.display()));
                result.push(file);
            }
        } else {
            let Ok(bytes) = std::fs::read(path) else {
                panic!("failed to read binary file `{}`", path.display());
            };
            let Some(file) = File::new(bytes) else {
                panic!("failed to read .winmd format `{}`", path.display());
            };
            result.push(file);
        }
    }

    let mut result = if input_default {
        default_input()
    } else {
        vec![]
    };

    for input in input {
        match input {
            Input::Path(path) => expand_path(&mut result, path),
            Input::Bytes(bytes) => result.push(
                File::new(bytes.clone())
                    .unwrap_or_else(|| panic!("failed to read .winmd format from memory")),
            ),
        }
    }

    result
}

/// Finds delegates used only as event-handler parameters.
fn compute_event_only_delegates(types: &TypeMap, reader: &Reader) -> HashSet<TypeName> {
    let mut event_delegates: HashSet<TypeName> = HashSet::new();
    let mut non_event_delegates: HashSet<TypeName> = HashSet::new();

    for type_set in types.values() {
        for ty in type_set {
            let (methods, generics): (Box<dyn Iterator<Item = MethodDef>>, &[Type]) = match ty {
                Type::Interface(i) => (Box::new(i.def.methods()), &i.generics),
                _ => continue,
            };

            for method in methods {
                let is_event_add = method.flags().contains(MethodAttributes::SpecialName)
                    && method.name().starts_with("add_");

                let sig = method.method_signature(generics, reader);
                for param in &sig.params {
                    if let Type::Delegate(d) = &param.ty {
                        if is_event_add {
                            event_delegates.insert(d.type_name());
                        } else {
                            non_event_delegates.insert(d.type_name());
                        }
                    }
                }
            }
        }
    }

    event_delegates
        .difference(&non_event_delegates)
        .copied()
        .collect()
}

fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
    namespace.starts_with(starts_with)
        && (namespace.len() == starts_with.len()
            || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
}

/// Collapses private per-header Win32 package namespaces to the public umbrella.
fn flat_module_namespace(namespace: &str) -> &str {
    const UMBRELLA: &str = "Windows.Win32";
    if namespace.len() > UMBRELLA.len()
        && namespace.starts_with(UMBRELLA)
        && namespace.as_bytes()[UMBRELLA.len()] == b'.'
    {
        return UMBRELLA;
    }
    namespace
}

/// Derives the cargo-feature name for a `--package` namespace.
fn namespace_feature(namespace: &str) -> String {
    if let Some(stem) = namespace.strip_prefix("Windows.Win32.") {
        stem.replace('.', "_")
    } else if let Some((_, rest)) = namespace.split_once('.') {
        rest.replace('.', "_")
    } else {
        namespace.to_string()
    }
}

/// Prepend reference entries so they take precedence.
fn prepend_default_refs(refs: &mut Vec<ReferenceStage>, crate_name: &str, paths: &[&str]) {
    refs.splice(
        0..0,
        paths
            .iter()
            .rev()
            .map(|path| ReferenceStage::new(crate_name, path)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        assert!(namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics.Direct3D11on12"
        ));
        assert!(namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics"
        ));
        assert!(!namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics.Direct3D11"
        ));
        assert!(!namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D",
            "Windows.Win32.Graphics.Direct3D11"
        ));
    }

    #[test]
    fn default_metadata_reader_is_reused() {
        assert!(std::ptr::eq(default_reader(), default_reader()));
    }

    #[test]
    fn implementation_selection() {
        let mut builder = Bindgen::new();
        builder
            .implement("Test.IFirst")
            .implements(["Test.ISecond", "Other"]);
        assert_eq!(
            builder.implement,
            Some(vec![
                "Test.IFirst".to_string(),
                "Test.ISecond".to_string(),
                "Other".to_string()
            ])
        );

        let mut builder = Bindgen::new();
        builder.implement_all().implement_all();
        assert_eq!(builder.implement, Some(vec![]));

        let mut builder = Bindgen::new();
        builder.implements(std::iter::empty::<&str>());
        assert_eq!(builder.implement, None);
    }

    #[test]
    #[should_panic(expected = "cannot combine selected implementations with `implement_all`")]
    fn implementation_after_all_panics() {
        Bindgen::new().implement_all().implement("Test.IFirst");
    }

    #[test]
    #[should_panic(expected = "cannot combine `implement_all` with selected implementations")]
    fn implementation_all_after_selection_panics() {
        Bindgen::new().implement("Test.IFirst").implement_all();
    }
}
