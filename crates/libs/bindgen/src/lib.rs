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
mod libraries;
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
pub use libraries::*;
use package_writer::*;
use param::*;
use references::*;
use signature::*;
use std::cmp::Ordering;
use std::collections::*;
use std::fmt::Write;
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
mod minimal_type_map;
use method_names::*;
use minimal_type_map::*;

/// Creates a new [`Bindgen`] builder for generating Windows API bindings.
pub fn builder() -> Bindgen {
    Bindgen::new()
}

/// Builder for generating Windows API bindings.
///
/// This provides a fluent builder API as an alternative to the command-line-like [`bindgen`] function.
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
    input: Vec<String>,
    filter: Vec<String>,
    output: String,
    derive: Vec<String>,
    implement: Option<Vec<String>>,
    rustfmt: Option<String>,
    layout: Layout,
    style: Style,
    dead_code: bool,
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
        /// When `true`, emit `extern { fn … }` instead of `link!` macros.
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

    // The predicates below name the individual code-generation policies that
    // distinguish the styles, so call sites read by intent rather than
    // re-deriving the same `is_minimal()` checks. See the "Output-mode
    // consolidation" tracking note in `docs/crates/windows-bindgen.md`.

    /// Whether to emit per-class wrapper methods. Minimal bindings omit them;
    /// callers reach the methods through the class's default interface instead.
    fn emit_class_methods(self) -> bool {
        !self.is_minimal()
    }

    /// Whether to emit caller-side wrappers that forward to a type's
    /// **inherited** interface methods. Minimal bindings omit these; callers
    /// `cast` to the owning interface instead.
    fn emit_inherited_forwarders(self) -> bool {
        !self.is_minimal()
    }

    /// Whether to emit the `IntoIterator` bridge that forwards to an inherited
    /// `IIterable<T>`. Minimal bindings omit it (an iterable's own
    /// `BufferedIterator` impl is still emitted); callers `cast` to `IIterable`.
    fn emit_iterable_into_iterator(self) -> bool {
        !self.is_minimal()
    }

    /// Whether an HSTRING **input** parameter is exposed as `&str` (converted to
    /// `HSTRING` inside the method body). Minimal bindings do this; other styles
    /// route strings through the `Param`/`IntoParam` machinery instead.
    fn minimal_string_input(self, param: &Param) -> bool {
        self.is_minimal() && param.is_input() && matches!(param.ty, Type::String)
    }

    /// Whether an HSTRING **return** value is exposed as an owned `String`.
    /// Minimal bindings do this; other styles return `HSTRING`.
    fn minimal_string_return(self, ty: &Type) -> bool {
        self.is_minimal() && matches!(ty, Type::String)
    }

    /// Whether plain value types (structs/enums) derive the standard
    /// `Default`/`Debug`/`PartialEq` traits (on top of the always-emitted
    /// `Copy`/`Clone`). Sys bindings emit bare value types with no extra derives.
    fn derive_std_traits(self) -> bool {
        !self.is_sys()
    }

    /// Whether to emit the `windows-core` trait block (type-kind, runtime
    /// signature, and `NAME`) for a value type. Sys bindings have no
    /// `windows-core` dependency, so they omit it.
    fn emit_core_traits(self) -> bool {
        !self.is_sys()
    }
}

impl Bindgen {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `.winmd` file or directory containing `.winmd` files.
    /// Use `"default"` to include the metadata bundled with `windows-bindgen`.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.inputs(std::iter::once(input))
    }

    /// Add multiple `.winmd` files or directories containing `.winmd` files.
    /// Use `"default"` to include the metadata bundled with `windows-bindgen`.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input.push(input.as_ref().to_string());
        }
        self
    }

    /// Set the output file where generated bindings will be written.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
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

    /// Include implementation traits for WinRT interfaces.
    ///
    /// Each entry may be a fully-qualified type name (`Namespace.Name`) or a
    /// namespace prefix that matches every type defined under it. When called
    /// with no patterns (an empty iterator), `_Impl` scaffolding is emitted for
    /// every WinRT interface in scope. When called with one or more patterns,
    /// `_Impl` scaffolding is emitted only for types matching the patterns,
    /// rather than for every interface/class in the filter set. The latter is
    /// a finer-grained alternative to the broad form and can significantly
    /// reduce build time when only a handful of interfaces need to be
    /// implemented.
    pub fn implement<I, S>(&mut self, names: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let list = self.implement.get_or_insert_with(Vec::new);
        for name in names {
            list.push(name.as_ref().to_string());
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
        // Validate up front so we fail fast before any expensive plumbing
        // (link string, input vec, references, reader, …) runs.
        assert!(
            !self.output.is_empty(),
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

        let default_input = ["default"];
        let input: Vec<&str> = if self.input.is_empty() {
            default_input.to_vec()
        } else {
            self.input.iter().map(|s| s.as_str()).collect()
        };

        let reader = Reader::new(expand_input(&input));

        let mut references: Vec<ReferenceStage> = Vec::new();

        if !sys {
            // Register implicit references to sibling windows-* crates for
            // common WinRT / Win32 types present in the input metadata.
            for (probe_namespace, crate_name, paths) in [
                (
                    "Windows.Foundation",
                    "windows_future",
                    &["Windows.Foundation.Async*", "Windows.Foundation.IAsync*"][..],
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
                (
                    "Windows.Win32.Foundation",
                    "windows_core",
                    &[
                        "Windows.Win32.Foundation.WIN32_ERROR",
                        "Windows.Win32.Foundation.NTSTATUS",
                        "Windows.Win32.System.Rpc.RPC_STATUS",
                    ][..],
                ),
            ] {
                if reader.contains_key(probe_namespace) {
                    let filtered: Vec<&str> = paths
                        .iter()
                        .copied()
                        .filter(|path| {
                            if let Some((namespace, name)) = path.rsplit_once('.') {
                                if let Some(ns_map) = reader.get(namespace) {
                                    if let Some(prefix) = name.strip_suffix('*') {
                                        return ns_map.keys().any(|k| k.starts_with(prefix));
                                    }
                                    return ns_map.contains_key(name);
                                }
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

        let references = References::new(&reader, references);

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
            let resolved = filter_parser::resolve_entries(&reader, &all_parsed);

            let mut filter = Filter::from_resolved(&reader, &resolved);

            // Use bottom-up type closure (MinimalTypeMap) when `--minimal` is
            // set and the filter has precise entries without broad patterns.
            // This walks only the signatures of requested methods to discover
            // the minimal set of required types.
            let types =
                if self.style.is_minimal() && !filter.has_broad_filter && !self.layout.is_package()
                {
                    MinimalTypeMap::build(&reader, &mut filter, &references)
                } else {
                    TypeMap::filter(&reader, &filter, &references)
                };

            (filter, types)
        };

        let derive = Derive::new(&reader, &types, &derive_str);
        if let Some(implements) = &implements {
            filter.validate_implements(implements);
        }

        let event_only_delegates = compute_event_only_delegates(&types, &reader);

        let config = Config {
            bindgen: self,
            reader: &reader,
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
        };

        let tree = TypeTree::new(&types);
        config.write(tree);
    }
}

#[track_caller]
fn expand_input(input: &[&str]) -> Vec<File> {
    #[track_caller]
    fn expand_input(result: &mut Vec<String>, input: &str) {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("failed to read directory `{input}`"))
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
                    result.push(path.to_string_lossy().to_string());
                }
            }

            assert!(
                result.len() != prev_len,
                "failed to find .winmd files in directory `{input}`"
            );
        } else {
            result.push(input.to_string());
        }
    }

    let mut paths = vec![];
    let mut use_default = false;

    for input in input {
        if *input == "default" {
            use_default = true;
        } else {
            expand_input(&mut paths, input);
        }
    }

    let mut input = vec![];

    if use_default {
        input = [
            std::include_bytes!("../default/Windows.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Win32.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Wdk.winmd").to_vec(),
        ]
        .into_iter()
        .map(|bytes| File::new(bytes).unwrap())
        .collect();
    }

    for path in &paths {
        let Ok(bytes) = std::fs::read(path) else {
            panic!("failed to read binary file `{path}`");
        };

        let Some(file) = File::new(bytes) else {
            panic!("failed to read .winmd format `{path}`");
        };

        input.push(file);
    }

    input
}

/// Computes the set of delegate TypeNames that are exclusively used as
/// parameters in `add_*` SpecialName methods (i.e., event handlers). These
/// delegates never need a public `new()` or `Invoke()` because the event-add
/// wrapper inlines the DelegateBox construction directly.
fn compute_event_only_delegates(types: &TypeMap, reader: &Reader) -> HashSet<TypeName> {
    let mut event_delegates: HashSet<TypeName> = HashSet::new();
    let mut non_event_delegates: HashSet<TypeName> = HashSet::new();

    for type_set in types.values() {
        for ty in type_set {
            let (methods, generics): (Box<dyn Iterator<Item = MethodDef>>, &[Type]) = match ty {
                Type::Interface(i) => (Box::new(i.def.methods()), &i.generics),
                Type::Class(_) => continue,
                _ => continue,
            };

            for method in methods {
                let is_event_add = method.flags().contains(MethodAttributes::SpecialName)
                    && method.name().starts_with("add_");

                let sig = method.method_signature("", generics, reader);
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
}
