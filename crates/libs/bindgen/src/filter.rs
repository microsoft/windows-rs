use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Which methods are requested on an interface.
#[derive(Debug, Clone)]
pub enum MethodSet {
    /// All methods on the interface.
    All,
    /// Specific methods by their raw MethodDef name.
    Names(BTreeSet<String>),
}

impl MethodSet {
    pub fn includes(&self, name: &str) -> bool {
        match self {
            Self::All => true,
            Self::Names(set) => set.contains(name),
        }
    }
}

/// The inclusion granularity of a type, following the Rust-`use`-style
/// specificity model: a type that was requested by name (namespace, type, or
/// method entry) is [`TypeRole::Named`] and projects its requested surface; a
/// type pulled in only as a dependency of something named is [`TypeRole::Shell`]
/// and projects name-only (opaque vtable slots), so the requested API stays
/// usable without dragging in the dependency's full surface.
#[derive(Debug, Clone)]
pub enum TypeRole {
    /// Explicitly requested — project the given method set.
    Named(MethodSet),
    /// Reachable only as a dependency — project name-only.
    Shell,
}

/// Returns true if `method_name` matches either the raw metadata name or the
/// overload-disambiguated name of `m`.
#[derive(Debug, Default)]
pub struct Filter {
    pub rules: Vec<(String, bool)>,
    methods: HashMap<(String, String), MethodFilter>,
    /// Enums with specific variants requested.
    enum_variants: HashMap<(String, String), MethodSet>,
    /// Classes that explicitly requested `CreateInstance`.
    activatable: HashSet<(String, String)>,
    /// Interfaces with specific methods requested (for type closure).
    /// Key: (namespace, type_name), Value: requested method names (or All).
    pub requested_interfaces: HashMap<(String, String), MethodSet>,
    /// Types directly included without `::` (for type closure).
    pub direct_types: Vec<(String, String)>,
    /// `true` if the filter includes broad entries (a whole namespace)
    /// that are not compatible with bottom-up type closure.
    pub has_broad_filter: bool,
    /// `true` when this filter is resolved via the bottom-up type closure
    /// ([`TypeClosure`]) rather than the namespace scan. A closure build only
    /// names its explicit seeds; every other type it pulls in is a dependency
    /// shell. A scan build (broad filter or `--package`) names everything it
    /// matches.
    pub uses_closure: bool,
}

/// Per-type method filter. Entries are recorded as two parallel sets:
///
/// * `keep` — methods explicitly allow-listed via `Ns.Type::Method`.
/// * `drop` — methods explicitly deny-listed via `!Ns.Type::Method`.
///
/// Entries may be either an exact `MethodDef` name (after property/event
/// sugar expansion, e.g. `Property` -> `get_Property` + `put_Property`),
/// or an overload-disambiguated Rust name produced by `[overload("…")]`
/// (e.g. `InsertKeyFrameWithEasingFunction` on the `InsertKeyFrame`
/// `MethodDef` row).
///
/// At lookup time both the raw `MethodDef` name and any overload name are
/// checked against each set, so a single entry can address either form.
///
/// Allow and deny entries may coexist on the same type. The resolution
/// rules at lookup time are (in order):
///
/// 1. If the method appears in `drop`, the slot is demoted.
/// 2. If the method appears in `keep`, the slot is kept.
/// 3. Otherwise: kept when `keep` is empty (deny-only mode), demoted
///    otherwise (allow-list mode treats unlisted methods as opt-out).
#[derive(Debug, Default)]
pub struct MethodFilter {
    /// Methods explicitly allow-listed.
    keep: BTreeSet<String>,
    /// Methods explicitly deny-listed. Wins over `keep` on overlap.
    drop: BTreeSet<String>,
}

impl Filter {
    /// Validate that no method-level filter entry targets a type matched by
    /// `--implement`. Methods on implemented types must always be emitted.
    #[track_caller]
    pub fn validate_implements(&self, implements: &Implements) {
        if implements.is_empty() {
            return;
        }
        for (namespace, name) in self.methods.keys() {
            assert!(
                !implements.matches_str(namespace, name),
                "method-level filter on `{namespace}.{name}` conflicts with `--implement`: \
                 methods on implemented interfaces are always emitted"
            );
        }
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        for rule in &self.rules {
            if rule.1 {
                // include
                if namespace_starts_with(&rule.0, namespace) {
                    return true;
                }
                if namespace_starts_with(namespace, &rule.0) {
                    return true;
                }
            } else {
                // exclude
                if namespace_starts_with(namespace, &rule.0) {
                    return false;
                }
            }
        }

        false
    }

    pub fn includes_type_name(&self, name: TypeName) -> Option<&str> {
        for rule in &self.rules {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return if rule.1 { Some(&rule.0) } else { None };
            }
        }

        None
    }

    pub fn excludes_type_name(&self, name: TypeName) -> bool {
        for rule in &self.rules {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return !rule.1;
            }
        }

        false
    }

    /// Classifies a type's inclusion granularity under the specificity model.
    ///
    /// Filtering reads like a Rust `use` declaration — a bare mention gives you
    /// the whole thing; braces narrow it:
    ///
    /// * naming a **namespace** (a scan build) takes every type in it, fully;
    /// * naming a **type** (`Ns.Type`) projects it in full — a bare interface
    ///   seeds all its methods, so it resolves to [`TypeRole::Named`]`(All)`;
    /// * naming specific **methods** (`Ns.Type::{a, b}`) resolves to
    ///   [`TypeRole::Named`] with just the listed methods;
    /// * an explicit name-only shell (`Ns.Type::{}`), and any type reached only
    ///   as a dependency of a seed, resolve to [`TypeRole::Shell`].
    ///
    /// A scan build (broad filter / `--package`) has no cherry-pick closure, so
    /// everything it matches is `Named`; the Shell role only arises on a closure
    /// build (a precise filter), for `::{}` shells and dependency types.
    pub fn type_role(&self, type_name: TypeName) -> TypeRole {
        let key = (
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        );

        if let Some(set) = self.requested_interfaces.get(&key) {
            return TypeRole::Named(set.clone());
        }

        if self.uses_closure {
            TypeRole::Shell
        } else {
            TypeRole::Named(MethodSet::All)
        }
    }

    /// Returns `true` if `method` on `type_name` should be emitted as a real
    /// vtable slot (rather than demoted to an opaque `Slot: usize`).
    ///
    /// With no explicit method filter for this type, inclusion follows the
    /// type's [`Filter::type_role`]: a `Named` type keeps the methods in its
    /// set, a `Shell` (a dependency, or an explicit `::{}` mention) keeps none.
    ///
    /// Matching considers both the raw `MethodDef` name and any
    /// overload-disambiguated Rust name produced by `[overload("…")]`, so a
    /// single filter entry can address either form. This lets callers write
    /// e.g. `!IFoo::InsertKeyFrameWithEasingFunction` to deny only the
    /// renamed overload while leaving the bare `InsertKeyFrame` slot intact.
    ///
    /// Allow (`IFoo::Method`) and deny (`!IFoo::Method`) entries may coexist
    /// on the same type. Deny wins on overlap. When at least one allow entry
    /// exists, unlisted methods are demoted (allow-list mode); otherwise
    /// only listed deny entries are demoted (deny-only mode). When `minimal`
    /// is true, overload matching uses the disambiguated name exclusively.
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        let key = (
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        );

        let Some(filter) = self.methods.get(&key) else {
            // No explicit method filter for this type. Its inclusion is decided
            // by its role: a Named type projects the methods in its set, a Shell
            // (a dependency pulled in only via the closure) projects name-only.
            return match self.type_role(type_name) {
                TypeRole::Named(set) => set.includes(method.name()),
                TypeRole::Shell => false,
            };
        };

        let raw = method.name();
        let overload = method_overload_name(method);

        // Match by overload-disambiguated name when one exists — the raw
        // metadata name is shared with other overloads and would include them
        // all indiscriminately.
        let in_set = |set: &BTreeSet<String>| -> bool {
            if let Some(ref name) = overload {
                set.contains(name.as_str())
            } else {
                set.contains(raw)
            }
        };

        // Deny wins on overlap.
        if in_set(&filter.drop) {
            return false;
        }
        if in_set(&filter.keep) {
            return true;
        }
        // Default: keep when there are no allow entries (deny-only mode),
        // drop otherwise (allow-list mode treats unlisted methods as opt-out).
        filter.keep.is_empty()
    }

    /// Returns the variant filter for a given enum, if one was specified.
    /// Returns `None` if the enum was included as a plain type (all variants kept).
    pub fn enum_variant_filter(&self, namespace: &str, name: &str) -> Option<&MethodSet> {
        self.enum_variants
            .get(&(namespace.to_string(), name.to_string()))
    }

    /// Returns `true` if the class was explicitly marked as activatable
    /// (i.e. `CreateInstance` or `::*` was in the filter).
    pub fn is_activatable(&self, namespace: &str, name: &str) -> bool {
        self.activatable
            .contains(&(namespace.to_string(), name.to_string()))
    }

    /// Builds a [`Filter`] from resolved filter entries, recording the seeds
    /// (types, methods, requested interfaces) that drive both the inclusion
    /// rules and the [`TypeClosure`] walk. Method-level specificity is preserved
    /// so that `type_role` / `includes_method` can later decide each type's
    /// projected surface.
    ///
    /// Supported entry syntax (specificity drives granularity, inspired by
    /// Rust's `use` declarations — a bare mention gives you the whole thing,
    /// braces narrow it):
    /// - `Namespace` — include everything in the namespace
    /// - `Namespace.Type` — include a type in full (an interface projects all
    ///   its methods; a struct all its fields; an enum all its variants; a
    ///   class its default interface)
    /// - `Namespace.Type::{}` — include a name-only shell (the type is usable
    ///   in signatures but projects none of its own methods)
    /// - `Namespace.Type::{a, b}` — include only the named methods; the rest
    ///   become name-only
    /// - `Namespace.Class::CreateInstance` — mark class as activatable
    #[track_caller]
    pub fn from_resolved(reader: &Reader, entries: &[filter_parser::ResolvedFilter]) -> Self {
        use filter_parser::ResolvedKind;

        let mut rules: Vec<(String, bool)> = Vec::new();
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut enum_variants: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut activatable: HashSet<(String, String)> = HashSet::new();
        let mut requested_interfaces: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut direct_types: Vec<(String, String)> = Vec::new();
        let mut has_broad_filter = false;

        for entry in entries {
            let include = !entry.exclude;

            match &entry.kind {
                ResolvedKind::Namespace(ns) => {
                    rules.push((ns.clone(), include));
                    if include {
                        has_broad_filter = true;
                    }
                }
                ResolvedKind::Type { namespace, name } => {
                    let full = format!("{namespace}.{name}");
                    rules.push((full, include));

                    if include {
                        // A bare type mention is "full". An interface seeds its
                        // whole method set into the [`TypeClosure`] walk; every
                        // other kind keeps its natural full surface (all struct
                        // fields, all enum variants, a class's default
                        // interface) via `direct_types`. Class activation and an
                        // interface's method subset stay explicit
                        // (`::CreateInstance`, `::{a, b}`); a name-only shell is
                        // the explicit `::{}`.
                        let key = (namespace.clone(), name.clone());
                        if Self::is_interface(reader, namespace, name) {
                            requested_interfaces.entry(key).or_insert(MethodSet::All);
                        } else if !direct_types.contains(&key) {
                            direct_types.push(key);
                        }
                    }
                }
                ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } => {
                    let full = format!("{namespace}.{name}");
                    // Member-level entries always include the type — exclusion
                    // applies to the method/variant, not the type itself.
                    if !rules.iter().any(|(r, _)| r == &full) {
                        rules.push((full, true));
                    }

                    if members.is_empty() {
                        // `Ns.Type::{}` — an explicit name-only shell: make the
                        // type available without projecting any of its methods.
                        // Recorded as a direct type so `type_role` resolves it
                        // to `Shell` on a closure build.
                        if include {
                            let key = (namespace.clone(), name.clone());
                            if !direct_types.contains(&key) {
                                direct_types.push(key);
                            }
                        }
                    } else {
                        for member in members {
                            Self::register_member(
                                reader,
                                &mut methods,
                                &mut requested_interfaces,
                                &mut direct_types,
                                &mut activatable,
                                &mut enum_variants,
                                namespace,
                                name,
                                member,
                                include,
                            );
                        }
                    }
                }
            }
        }

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self {
            rules,
            methods,
            enum_variants,
            activatable,
            requested_interfaces,
            direct_types,
            has_broad_filter,
            uses_closure: false,
        }
    }

    /// Whether the named type resolves to an interface (WinRT or COM), which a
    /// bare mention seeds into the [`TypeClosure`] walk as `All` methods.
    fn is_interface(reader: &Reader, namespace: &str, name: &str) -> bool {
        matches!(
            reader.with_full_name(namespace, name).next(),
            Some(Type::Interface(_) | Type::CppInterface(_))
        )
    }

    /// Register a specific member (method/variant) on a type.
    #[expect(clippy::too_many_arguments, clippy::redundant_clone)]
    fn register_member(
        reader: &Reader,
        methods: &mut HashMap<(String, String), MethodFilter>,
        requested_interfaces: &mut HashMap<(String, String), MethodSet>,
        direct_types: &mut Vec<(String, String)>,
        activatable: &mut HashSet<(String, String)>,
        enum_variants: &mut HashMap<(String, String), MethodSet>,
        namespace: &str,
        name: &str,
        member: &str,
        include: bool,
    ) {
        let key = (namespace.to_string(), name.to_string());

        if let Some(ty) = reader.with_full_name(namespace, name).next() {
            match &ty {
                Type::Enum(e) => {
                    // Check variant exists
                    assert!(
                        e.def.fields().any(|f| {
                            f.flags().contains(FieldAttributes::Literal) && f.name() == member
                        }),
                        "variant `{member}` not found on enum `{namespace}.{name}`"
                    );
                    let set = enum_variants
                        .entry(key.clone())
                        .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                    if let MethodSet::Names(names) = set {
                        names.insert(member.to_string());
                    }
                    if !direct_types.contains(&key) {
                        direct_types.push(key);
                    }
                }
                Type::CppEnum(e) => {
                    assert!(
                        e.def.fields().any(|f| {
                            f.flags().contains(FieldAttributes::Literal) && f.name() == member
                        }),
                        "variant `{member}` not found on enum `{namespace}.{name}`"
                    );
                    let set = enum_variants
                        .entry(key.clone())
                        .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                    if let MethodSet::Names(names) = set {
                        names.insert(member.to_string());
                    }
                    if !direct_types.contains(&key) {
                        direct_types.push(key);
                    }
                }
                Type::Class(class) => {
                    let required = class.required_interfaces(reader);
                    // Route to the class's required interfaces
                    if member == "CreateInstance" {
                        if !direct_types.contains(&key) {
                            direct_types.push(key.clone());
                        }
                        activatable.insert(key.clone());
                    } else {
                        // Find which interface carries this method
                        let mut found = false;
                        for iface in &required {
                            let defs: Vec<MethodDef> = iface.def.methods().collect();
                            let mut expanded = expand_method_part(member, &defs);
                            if expanded.is_empty() {
                                continue;
                            }
                            // Auto-include remove_X when add_X is requested
                            if include {
                                let remove_extras: Vec<String> = expanded
                                    .iter()
                                    .filter_map(|m| {
                                        m.strip_prefix("add_")
                                            .map(|event| format!("remove_{event}"))
                                    })
                                    .filter(|r| defs.iter().any(|d| d.name() == r.as_str()))
                                    .collect();
                                expanded.extend(remove_extras);
                            }
                            let iface_key = (
                                iface.def.namespace().to_string(),
                                iface.def.name().to_string(),
                            );
                            // Register expanded names in requested_interfaces
                            let set = requested_interfaces
                                .entry(iface_key.clone())
                                .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                            if let MethodSet::Names(names) = set {
                                for n in &expanded {
                                    names.insert(n.clone());
                                }
                            }
                            let filter_entry = methods.entry(iface_key).or_default();
                            if include {
                                for n in &expanded {
                                    filter_entry.keep.insert(n.clone());
                                }
                            } else {
                                filter_entry.drop.insert(member.to_string());
                            }
                            if !direct_types.contains(&key) {
                                direct_types.push(key.clone());
                            }
                            found = true;
                            break;
                        }
                        // Check composable interfaces too
                        if !found {
                            for iface in &required {
                                if matches!(iface.kind, InterfaceKind::Composable) {
                                    let defs: Vec<MethodDef> = iface.def.methods().collect();
                                    if !expand_method_part(member, &defs).is_empty() {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        assert!(
                            found,
                            "method `{member}` not found on class `{namespace}.{name}`"
                        );
                    }
                    // Include composable factory interfaces so new() works.
                    for iface in &required {
                        if matches!(iface.kind, InterfaceKind::Composable) {
                            let iface_key = (
                                iface.def.namespace().to_string(),
                                iface.def.name().to_string(),
                            );
                            requested_interfaces
                                .entry(iface_key)
                                .or_insert(MethodSet::All);
                        }
                    }
                }
                Type::Interface(_) | Type::CppInterface(_) | Type::Delegate(_) => {
                    // Expand sugar (e.g. "Click" → "add_Click" + "remove_Click")
                    // before registering so both requested_interfaces and the
                    // method filter use the real metadata method names.
                    let def = match &ty {
                        Type::Interface(t) => t.def,
                        Type::CppInterface(t) => t.def,
                        Type::Delegate(t) => t.def,
                        _ => unreachable!(),
                    };
                    let defs: Vec<MethodDef> = def.methods().collect();
                    let mut expanded = expand_method_part(member, &defs);
                    assert!(
                        !expanded.is_empty(),
                        "method `{member}` not found on `{namespace}.{name}`"
                    );
                    // Auto-include remove_X when add_X is requested
                    if include {
                        let remove_extras: Vec<String> = expanded
                            .iter()
                            .filter_map(|m| {
                                m.strip_prefix("add_")
                                    .map(|event| format!("remove_{event}"))
                            })
                            .filter(|r| defs.iter().any(|d| d.name() == r.as_str()))
                            .collect();
                        expanded.extend(remove_extras);
                    }
                    // Register expanded names in requested_interfaces for the
                    // type closure (used by [`TypeClosure`] to walk only requested
                    // method signatures).
                    let set = requested_interfaces
                        .entry(key.clone())
                        .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                    if let MethodSet::Names(names) = set {
                        for name in &expanded {
                            names.insert(name.clone());
                        }
                    }
                    register_method_filter(methods, namespace, name, expanded, include);
                }
                _ => {
                    panic!("type `{namespace}.{name}` does not support member-level filtering");
                }
            }
        } else {
            panic!("type not found: `{namespace}.{name}`");
        }
    }
}

#[track_caller]
fn expand_method_part(method_part: &str, defs: &[MethodDef]) -> Vec<String> {
    // A member entry names an actual metadata method. A single metadata name
    // may cover several overload rows; the entry applies to all of them, which
    // preserves the behavior of `!Iface::Method`.
    if defs.iter().any(|m| m.name() == method_part) {
        return vec![method_part.to_string()];
    }

    // Bare-name accessor sugar. A property or event is named by its logical
    // name (e.g. `Tick`, `Interval`) and expands to its accessors. Property
    // accessors (`get_X` / `put_X`) are tried first; if none exist, event
    // accessors (`add_X` / `remove_X`) are used. WinRT interfaces cannot define
    // a property and an event under the same name, so the ordering is
    // unambiguous.
    let getter = format!("get_{method_part}");
    let setter = format!("put_{method_part}");
    let adder = format!("add_{method_part}");
    let remover = format!("remove_{method_part}");

    let mut expanded = Vec::new();
    if defs.iter().any(|m| m.name() == getter) {
        expanded.push(getter);
    }
    if defs.iter().any(|m| m.name() == setter) {
        expanded.push(setter);
    }
    if expanded.is_empty() {
        if defs.iter().any(|m| m.name() == adder) {
            expanded.push(adder);
        }
        if defs.iter().any(|m| m.name() == remover) {
            expanded.push(remover);
        }
    }
    if !expanded.is_empty() {
        return expanded;
    }

    // Overload-disambiguated name match. The set entry is the overload name
    // itself — `Filter::includes_method` checks the overload name of each
    // `MethodDef` alongside its raw name, so this addresses exactly the row
    // whose `[overload("…")]` attribute carries this value.
    if defs
        .iter()
        .any(|m| method_overload_name(*m).as_deref() == Some(method_part))
    {
        return vec![method_part.to_string()];
    }

    Vec::new()
}

fn register_method_filter(
    methods: &mut HashMap<(String, String), MethodFilter>,
    namespace: &str,
    type_name: &str,
    expanded: Vec<String>,
    include: bool,
) {
    let key = (namespace.to_string(), type_name.to_string());
    let entry = methods.entry(key).or_default();

    if include {
        entry.keep.extend(expanded);
    } else {
        entry.drop.extend(expanded);
    }
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace_starts_with(namespace, rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}
