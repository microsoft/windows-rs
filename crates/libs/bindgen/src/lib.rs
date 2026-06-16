#![doc = include_str!("../readme.md")]

mod cli;
mod config;
mod derive;
mod derive_writer;
mod filter;
mod format;
mod guid;
mod implements;
mod index;
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
mod warnings;
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
pub use warnings::*;
use winmd::*;
mod filter_parser;
mod method_names;
mod minimal_type_map;
use method_names::*;
use minimal_type_map::*;

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
///     .write()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct Bindgen {
    input: Vec<String>,
    filter: Vec<String>,
    output: String,
    references: Vec<String>,
    derive: Vec<String>,
    implement: Option<Vec<String>>,
    rustfmt: Option<String>,
    link: Option<String>,
    layout: Layout,
    style: Style,
    deps: Option<DepMode>,
    index: bool,
}

/// Output layout for the generated bindings. Mutually exclusive variants
/// replace the legacy `flat: bool` + `package: bool` + `no_toml: bool`
/// triple, making invalid combinations unrepresentable.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum Layout {
    /// One Rust module per metadata namespace (the default).
    #[default]
    Modules,
    /// A single flat list of items (no namespace modules).
    Flat,
    /// One file per namespace + `Cargo.toml` features.
    Package {
        /// When `true`, skip rewriting `Cargo.toml`.
        no_toml: bool,
    },
}

impl Layout {
    fn is_flat(self) -> bool {
        matches!(self, Layout::Flat)
    }
    fn is_package(self) -> bool {
        matches!(self, Layout::Package { .. })
    }
    fn no_toml(self) -> bool {
        matches!(self, Layout::Package { no_toml: true })
    }
}

/// Code-style mode for the generated bindings. Mutually exclusive variants
/// replace the legacy `sys: bool` + `minimal: bool` + `sys_fn_extern: bool`
/// triple, making invalid combinations unrepresentable.
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
        matches!(self, Style::Sys { .. })
    }
    fn is_minimal(self) -> bool {
        matches!(self, Style::Minimal)
    }
    fn sys_fn_extern(self) -> bool {
        matches!(self, Style::Sys { extern_fns: true })
    }
}

impl Bindgen {
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

    /// Add a reference dependency.
    pub fn reference(&mut self, reference: &str) -> &mut Self {
        self.references(std::iter::once(reference))
    }

    /// Add multiple reference dependencies.
    pub fn references<I, S>(&mut self, references: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for reference in references {
            self.references.push(reference.as_ref().to_string());
        }
        self
    }

    /// Override the default Rust formatter path.
    pub fn rustfmt(&mut self, rustfmt: &str) -> &mut Self {
        self.rustfmt = Some(rustfmt.to_string());
        self
    }

    /// Override the default `windows-link` implementation for system calls.
    pub fn link(&mut self, link: &str) -> &mut Self {
        self.link = Some(link.to_string());
        self
    }

    /// Avoid the default namespace-to-module conversion.
    #[track_caller]
    pub fn flat(&mut self) -> &mut Self {
        if matches!(self.layout, Layout::Package { .. }) {
            panic!("cannot combine `--package` and `--flat`");
        }
        self.layout = Layout::Flat;
        self
    }

    /// Select how generated bindings depend on the `windows-*` crates.
    ///
    /// - [`DepMode::Core`]: depend on `windows-core` (default).
    /// - [`DepMode::Specific`]: depend on `windows-result`, `windows-strings`,
    ///   and `windows-link` directly.
    /// - [`DepMode::None`]: no `windows-*` dependencies (default for `--sys`).
    pub fn deps(&mut self, mode: DepMode) -> &mut Self {
        self.deps = Some(mode);
        self
    }

    /// Returns the effective dependency mode, resolving the default based on
    /// the current style when `--deps` was not explicitly set.
    pub(crate) fn resolved_deps(&self) -> DepMode {
        self.deps
            .unwrap_or(if self.style.is_sys() && !self.layout.is_package() {
                DepMode::None
            } else {
                DepMode::Core
            })
    }

    /// Avoid generating the Cargo.toml features when using `package` mode.
    ///
    /// Only valid in combination with [`Bindgen::package`]; panics otherwise.
    #[track_caller]
    pub fn no_toml(&mut self) -> &mut Self {
        match &mut self.layout {
            Layout::Package { no_toml } => *no_toml = true,
            _ => panic!("`--no-toml` requires `--package`"),
        }
        self
    }

    /// Generate bindings as a package with one file per namespace.
    #[track_caller]
    pub fn package(&mut self) -> &mut Self {
        let no_toml = matches!(self.layout, Layout::Package { no_toml: true });
        if matches!(self.layout, Layout::Flat) {
            panic!("cannot combine `--package` and `--flat`");
        }
        self.layout = Layout::Package { no_toml };
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
    /// In `minimal` mode, the per-class wrapper methods on WinRT runtime
    /// classes are omitted (only static/composable factory helpers and the
    /// `new()` activation helper are kept), and inherited / forwarding wrapper
    /// methods on interfaces are omitted (only methods that dispatch on the
    /// interface's own vtable are kept). Callers must explicitly
    /// `cast::<IFoo>()?` to the interface that owns a slot before invoking it.
    ///
    /// Handle types are emitted as bare `pub type` aliases over their
    /// underlying primitive (matching `--sys`), without the
    /// `is_invalid`, `Free`, or `AlsoUsableFor` machinery. Free functions are
    /// emitted as their `link!` (or extern) declaration only, without the
    /// `Result<T>` / `from_thread` / `from_abi` ergonomic wrappers (also
    /// matching `--sys`).
    ///
    /// Event accessor pairs (`add_*`/`remove_*`) are replaced by a single
    /// auto-revoking wrapper that returns a `windows_core::EventRevoker`.
    /// The `Remove*` wrapper is suppressed. Callers can call
    /// `windows_core::EventRevoker::into_token` to recover the raw token when interoperating
    /// with code that manages registration tokens directly.
    ///
    /// This is a build-time / disk / memory optimization: the generated source
    /// is dramatically smaller and rustc does much less type-checking and
    /// codegen work, at the cost of API ergonomics. Vtable layout, ABI, GUIDs,
    /// `RuntimeType` signatures, and `interface_hierarchy!` invocations are
    /// preserved bit-for-bit, so existing `windows-implement` consumers and
    /// raw-vtable callers are unaffected.
    ///
    /// `--minimal` is mutually exclusive with `--sys`.
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

    /// Generate a `features.json` index alongside the output file.
    pub fn index(&mut self) -> &mut Self {
        self.index = true;
        self
    }

    fn is_sys(&self) -> bool {
        self.style.is_sys()
    }

    /// Generate the bindings.
    #[track_caller]
    #[must_use]
    pub fn write(&self) -> Warnings {
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

        let sys = self.is_sys();

        let link = if let Some(link) = self.link.as_deref() {
            link
        } else if sys || self.resolved_deps() == DepMode::Specific {
            "windows_link"
        } else {
            "windows_core"
        };

        let default_input = ["default"];
        let input: Vec<&str> = if self.input.is_empty() {
            default_input.to_vec()
        } else {
            self.input.iter().map(|s| s.as_str()).collect()
        };

        let reader = Reader::new(expand_input(&input));

        let mut references: Vec<ReferenceStage> = self
            .references
            .iter()
            .map(|s| ReferenceStage::parse(s))
            .collect();

        if !sys && self.resolved_deps() != DepMode::None {
            // Implicit references onto sibling `windows-*` crates that
            // re-export common WinRT / Win32 types. Each group is registered
            // only when its source namespace is actually present in the
            // current metadata input. Stages are *prepended* via
            // `prepend_default_refs` so they take precedence over any
            // user-supplied `--reference` entries (matching the historical
            // `references.insert(0, …)` ordering).
            let win32_foundation_crate = if self.resolved_deps() == DepMode::Specific {
                "windows_result"
            } else {
                "windows_core"
            };
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
                    win32_foundation_crate,
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

        // Detect new Rust-style filter syntax: entries that use `::` as the
        // namespace separator (not just for methods). Old syntax uses dots for
        // namespaces: `Windows.Foundation.DateTime` or `Ns.Type::{Method}`.
        // New syntax uses `::` throughout: `Windows::Foundation::DateTime`.
        let use_new_parser = include.iter().chain(exclude.iter()).any(|e| {
            // New syntax: first `::` is NOT preceded by a dot-separated path.
            // e.g. `Windows::Foundation` has `::` at position 7 with no dots before it.
            // Old syntax: `Windows.Foundation.IFoo::{Method}` has dots before `::`.
            if let Some(pos) = e.find("::") {
                !e[..pos].contains('.')
            } else {
                false
            }
        });

        let (filter, types) = if use_new_parser {
            // New unified filter path: parse + resolve all entries, then build
            // the filter and type closure uniformly.
            let mut parsed = Vec::new();
            for entry in include.iter().chain(exclude.iter()) {
                parsed.extend(filter_parser::parse_filter_entry(entry));
            }
            // Mark exclusion entries (from the `exclude` list, unless they
            // already have `!` prefix from the new syntax).
            // Actually, with new syntax all entries go through include and
            // `!` prefix handles exclusions inline. The old --exclude entries
            // will have been converted to `!` prefixed in the filter list.
            // For now, handle the case where exclude entries come separately:
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
            let default_demote = self.style.is_minimal();
            let mut filter = Filter::from_resolved(&reader, &resolved, default_demote);

            let types = if self.style.is_minimal() {
                MinimalTypeMap::build(&reader, &mut filter, &references)
            } else {
                TypeMap::filter(&reader, &filter, &references)
            };
            (filter, types)
        } else if self.style.is_minimal() {
            // Legacy minimal path
            let mut filter = Filter::new_minimal(&reader, &include);
            let types = MinimalTypeMap::build(&reader, &mut filter, &references);
            (filter, types)
        } else {
            // Legacy non-minimal path
            let filter = Filter::new(&reader, &include, &exclude);
            let types = TypeMap::filter(&reader, &filter, &references);
            (filter, types)
        };

        let derive = Derive::new(&reader, &types, &derive_str);
        if let Some(implements) = &implements {
            filter.validate_implements(implements);
        }
        let warnings = WarningBuilder::default();
        for message in filter.warnings() {
            warnings.add(message.clone());
        }

        let event_only_delegates = if self.style.is_minimal() {
            compute_event_only_delegates(&types, &reader)
        } else {
            HashSet::new()
        };

        let config = Config {
            bindgen: self,
            reader: &reader,
            types: &types,
            references: &references,
            filter: &filter,
            derive: &derive,
            implement: implements.as_ref(),
            link,
            warnings: &warnings,
            namespace: "",
            event_only_delegates: &event_only_delegates,
        };

        let tree = TypeTree::new(&types);
        config.write(tree);

        if self.index {
            index::write(&types, &format!("{}/features.json", self.output), &reader);
        }

        warnings.build()
    }
}

/// Selects how generated bindings depend on the `windows-*` crates.
///
/// Used with [`Bindgen::deps`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepMode {
    /// Depend on `windows-core` (default).
    Core,
    /// Depend on `windows-result`, `windows-strings`, and `windows-link` directly.
    Specific,
    /// No `windows-*` dependencies (default for `--sys`).
    None,
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
    // Collect all delegates in the type map.
    let mut all_delegates: HashSet<TypeName> = HashSet::new();
    for type_set in types.values() {
        for ty in type_set {
            if let Type::Delegate(d) = ty {
                all_delegates.insert(d.type_name());
            }
        }
    }

    // Track delegates that appear in non-event contexts.
    let mut non_event_delegates: HashSet<TypeName> = HashSet::new();

    for type_set in types.values() {
        for ty in type_set {
            let (methods, generics): (Box<dyn Iterator<Item = MethodDef>>, &[Type]) = match ty {
                Type::Interface(i) => (Box::new(i.def.methods()), &i.generics),
                Type::Class(c) => {
                    // Classes reference interfaces; the interfaces' own methods
                    // will be visited when we process the Interface variant.
                    // However, some classes have factory/static methods on
                    // exclusive interfaces that may take delegate params.
                    // Those interfaces are in the type map separately, so skip.
                    let _ = c;
                    continue;
                }
                _ => continue,
            };

            for method in methods {
                let is_event_add = method.flags().contains(MethodAttributes::SpecialName)
                    && method.name().starts_with("add_");

                if is_event_add {
                    continue;
                }

                // For non-event methods, any delegate param means that delegate
                // is used outside of events.
                let sig = method.method_signature("", generics, reader);
                for param in &sig.params {
                    if let Type::Delegate(d) = &param.ty {
                        non_event_delegates.insert(d.type_name());
                    }
                }
            }
        }
    }

    // Event-only = all delegates minus those used in non-event contexts.
    all_delegates
        .difference(&non_event_delegates)
        .copied()
        .collect()
}

fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
    namespace.starts_with(starts_with)
        && (namespace.len() == starts_with.len()
            || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
}

/// Prepend a group of default `ReferenceStage` entries onto `refs`, in the
/// same order historically produced by repeated `refs.insert(0, …)` calls
/// over each path in source order. Reads as: "register this list of paths
/// as a `Flat`-style reference onto `crate_name`, taking precedence over
/// any user-supplied `--reference` entries already in `refs`".
fn prepend_default_refs(refs: &mut Vec<ReferenceStage>, crate_name: &str, paths: &[&str]) {
    // `paths` is iterated in reverse so the resulting prefix matches the
    // historical order produced by repeated `refs.insert(0, p)` calls (which
    // reverses `paths` as a side effect). `splice(0..0, …)` does a single
    // O(n) shift instead of one shift per element.
    refs.splice(
        0..0,
        paths
            .iter()
            .rev()
            .map(|path| ReferenceStage::new(crate_name, ReferenceStyle::Flat, path)),
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
