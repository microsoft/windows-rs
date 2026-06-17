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
            MethodSet::All => true,
            MethodSet::Names(set) => set.contains(name),
        }
    }
}

/// Returns true if `method_name` matches either the raw metadata name or the
/// overload-disambiguated name of `m`.
fn method_matches(m: MethodDef, method_name: &str) -> bool {
    if m.name() == method_name {
        return true;
    }
    method_overload_name(m).as_deref() == Some(method_name)
}

#[derive(Debug, Default)]
pub struct Filter {
    pub rules: Vec<(String, bool)>,
    methods: HashMap<(String, String), MethodFilter>,
    warnings: Vec<String>,
    /// Enums with specific variants requested.
    enum_variants: HashMap<(String, String), MethodSet>,
    /// Classes that explicitly requested `CreateInstance`.
    activatable: HashSet<(String, String)>,
    /// When `true`, methods on types with no explicit MethodFilter entry are
    /// demoted by default (minimal/opt-in mode).
    pub default_demote: bool,
    /// Interfaces with specific methods requested (for type closure).
    /// Key: (namespace, type_name), Value: requested method names (or All).
    pub requested_interfaces: HashMap<(String, String), MethodSet>,
    /// Types directly included without `::` (for type closure).
    pub direct_types: Vec<(String, String)>,
    /// `true` if the filter includes broad entries (namespaces or name globs)
    /// that are not compatible with bottom-up type closure.
    pub has_broad_filter: bool,
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
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

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

    /// Returns `true` if `method` on `type_name` should be emitted as a real
    /// vtable slot (rather than demoted to an opaque `Slot: usize`).
    /// In the absence of a method filter for this type, all methods are kept.
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
    /// only listed deny entries are demoted (deny-only mode).
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        let key = (
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        );

        let Some(filter) = self.methods.get(&key) else {
            // No explicit method filter for this type. In minimal mode,
            // check requested_interfaces — types registered with MethodSet::All
            // (e.g. composable factory interfaces) should include all methods.
            if self.default_demote {
                if let Some(MethodSet::All) = self.requested_interfaces.get(&key) {
                    return true;
                }
            }
            return !self.default_demote;
        };

        let raw = method.name();
        let overload = method_overload_name(method);

        // In minimal mode (default_demote), match by overload-disambiguated name
        // when one exists — the raw metadata name is shared with other overloads
        // and would include them all indiscriminately. In non-minimal mode,
        // match either raw or overload for broader compatibility.
        let in_set = |set: &BTreeSet<String>| -> bool {
            if self.default_demote {
                if let Some(ref name) = overload {
                    set.contains(name.as_str())
                } else {
                    set.contains(raw)
                }
            } else {
                set.contains(raw)
                    || overload
                        .as_ref()
                        .is_some_and(|name| set.contains(name.as_str()))
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

    /// Create a filter for minimal/opt-in mode. Parses the same `::` syntax
    /// as the standard filter but with `default_demote = true` — only
    /// explicitly-requested methods are emitted.
    ///
    /// Supported entry syntax:
    /// - `Namespace.Type` — include a type (function, struct, enum, class)
    /// - `Namespace.Type::method` — include a specific method
    /// - `Namespace.Type::*` — include all methods on a type
    /// - `Namespace.Type::{a, b}` — include multiple methods
    /// - `Namespace.Class::CreateInstance` — mark class as activatable
    #[track_caller]
    pub fn from_resolved(
        reader: &Reader,
        entries: &[filter_parser::ResolvedFilter],
        default_demote: bool,
    ) -> Self {
        use filter_parser::ResolvedKind;

        let mut rules: Vec<(String, bool)> = Vec::new();
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut enum_variants: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut activatable: HashSet<(String, String)> = HashSet::new();
        let mut requested_interfaces: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut direct_types: Vec<(String, String)> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();
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
                ResolvedKind::NameGlob { namespace, prefix } => {
                    if include {
                        has_broad_filter = true;
                    }
                    // Expand glob to concrete types
                    if let Some(ns_map) = reader.get(namespace.as_str()) {
                        for name in ns_map.keys() {
                            if name.starts_with(prefix.as_str()) {
                                let full = format!("{namespace}.{name}");
                                rules.push((full, include));
                            }
                        }
                    }
                }
                ResolvedKind::Type { namespace, name } => {
                    let full = format!("{namespace}.{name}");
                    rules.push((full, include));

                    if include {
                        // Record as a direct type for MinimalTypeMap's closure.
                        // Populated regardless of mode — the type-map decision
                        // happens after filter construction.
                        let key = (namespace.clone(), name.clone());
                        if !direct_types.contains(&key) {
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

                    // Register each member
                    if members.len() == 1 && members[0] == "*" {
                        // ::* — expand all methods/members on the type
                        if include {
                            Self::register_type_for_minimal(
                                reader,
                                namespace,
                                name,
                                &mut requested_interfaces,
                                &mut direct_types,
                                &mut activatable,
                                &mut enum_variants,
                            );
                        }
                    } else {
                        for member in members {
                            Self::register_member(
                                reader,
                                &mut methods,
                                &mut warnings,
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
            warnings,
            enum_variants,
            activatable,
            default_demote,
            requested_interfaces,
            direct_types,
            has_broad_filter,
        }
    }

    /// Register a type for minimal mode's type closure.
    fn register_type_for_minimal(
        reader: &Reader,
        namespace: &str,
        name: &str,
        requested_interfaces: &mut HashMap<(String, String), MethodSet>,
        direct_types: &mut Vec<(String, String)>,
        activatable: &mut HashSet<(String, String)>,
        enum_variants: &mut HashMap<(String, String), MethodSet>,
    ) {
        let key = (namespace.to_string(), name.to_string());

        if let Some(ty) = reader.with_full_name(namespace, name).next() {
            match &ty {
                Type::Class(c) => {
                    if !direct_types.contains(&key) {
                        direct_types.push(key.clone());
                    }
                    activatable.insert(key);
                    for iface in c.required_interfaces(reader) {
                        let iface_ns = iface.def.namespace().to_string();
                        let iface_name = iface.def.name().to_string();
                        requested_interfaces
                            .entry((iface_ns, iface_name))
                            .or_insert(MethodSet::All);
                    }
                }
                Type::Enum(_) | Type::CppEnum(_) => {
                    enum_variants.insert(key.clone(), MethodSet::All);
                    if !direct_types.contains(&key) {
                        direct_types.push(key);
                    }
                }
                Type::Interface(_) | Type::CppInterface(_) => {
                    requested_interfaces.insert(key, MethodSet::All);
                }
                _ => {
                    if !direct_types.contains(&key) {
                        direct_types.push(key);
                    }
                }
            }
        }
    }

    /// Register a specific member (method/variant) on a type.
    #[allow(clippy::too_many_arguments, clippy::redundant_clone)]
    fn register_member(
        reader: &Reader,
        methods: &mut HashMap<(String, String), MethodFilter>,
        warnings: &mut Vec<String>,
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
                            if iface.def.methods().any(|m| method_matches(m, member)) {
                                let iface_key = (
                                    iface.def.namespace().to_string(),
                                    iface.def.name().to_string(),
                                );
                                // Register on the interface
                                let set = requested_interfaces
                                    .entry(iface_key.clone())
                                    .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                                if let MethodSet::Names(names) = set {
                                    names.insert(member.to_string());
                                    if let Some(event) = member.strip_prefix("add_") {
                                        names.insert(format!("remove_{event}"));
                                    }
                                }
                                let filter_entry = methods.entry(iface_key).or_default();
                                if include {
                                    filter_entry.keep.insert(member.to_string());
                                    if let Some(event) = member.strip_prefix("add_") {
                                        filter_entry.keep.insert(format!("remove_{event}"));
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
                        }
                        // Check composable interfaces too
                        if !found {
                            for iface in &required {
                                if matches!(iface.kind, InterfaceKind::Composable)
                                    && iface.def.methods().any(|m| method_matches(m, member))
                                {
                                    found = true;
                                    break;
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
                    // Register in requested_interfaces for type closure
                    // (used by MinimalTypeMap to walk only requested method
                    // signatures). Populated regardless of mode — the decision
                    // of which type-map algorithm to use happens later.
                    let set = requested_interfaces
                        .entry(key.clone())
                        .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                    if let MethodSet::Names(names) = set {
                        names.insert(member.to_string());
                        // Auto-include remove_X when add_X is requested
                        if let Some(event) = member.strip_prefix("add_") {
                            names.insert(format!("remove_{event}"));
                        }
                    }
                    // Also register in method filter for emission control
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
                    maybe_warn_ambiguous_overload(
                        warnings, member, namespace, name, &defs, include, member,
                    );
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
#[allow(clippy::too_many_arguments)]
fn expand_method_part(method_part: &str, defs: &[MethodDef]) -> Vec<String> {
    // Accessor-only sugar: `get:Prop` / `set:Prop` / `add:Evt` / `remove:Evt`
    // expand to a single accessor name, letting callers opt in to just the
    // setter (or just the getter) without listing the raw `put_Prop` /
    // `get_Prop` name. This is especially useful in reactive UI code where
    // state flows one direction and most properties only need a setter.
    if let Some((prefix, name)) = method_part.split_once(':') {
        let accessor = match prefix {
            "get" => format!("get_{name}"),
            "set" => format!("put_{name}"),
            "add" => format!("add_{name}"),
            "remove" => format!("remove_{name}"),
            _ => return Vec::new(),
        };
        if defs.iter().any(|m| m.name() == accessor) {
            return vec![accessor];
        }
        return Vec::new();
    }

    if defs.iter().any(|m| m.name() == method_part) {
        // Exact match against a metadata method name. No sugar expansion
        // needed. Even if the same metadata name covers several overload
        // rows, the entry intentionally applies to all of them — this
        // preserves the historical behavior of `!Iface::Method`.
        return vec![method_part.to_string()];
    }

    // Sugar expansion. Try property accessors (`get_X` / `put_X`) first;
    // if that produces nothing, fall back to event accessors (`add_X` /
    // `remove_X`). WinRT interfaces cannot define a property and an event
    // under the same name (compile-time metadata restriction), so the
    // property-then-event ordering is unambiguous in practice.
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

    // Overload-disambiguated name match. The set entry is the overload
    // name itself — `Filter::includes_method` checks the overload name of
    // each `MethodDef` alongside its raw name, so this addresses exactly
    // the row whose `[overload("…")]` attribute carries this value.
    if defs
        .iter()
        .any(|m| method_overload_name(*m).as_deref() == Some(method_part))
    {
        return vec![method_part.to_string()];
    }

    Vec::new()
}

/// If a deny filter entry collapses onto a raw `MethodDef` name that is
/// shared by multiple rows (real CLR overloads), warn the user listing the
/// disambiguated Rust names so they can use the precise form to keep the
/// overloads they actually consume.
fn maybe_warn_ambiguous_overload(
    warnings: &mut Vec<String>,
    method_part: &str,
    namespace: &str,
    type_name: &str,
    defs: &[MethodDef],
    include: bool,
    raw: &str,
) {
    if include {
        return;
    }
    // Only flag exact raw-name matches. Sugar and overload-name resolution
    // already address specific rows (or all property/event accessors), so
    // their multi-row behavior is intentional.
    let matching: Vec<MethodDef> = defs
        .iter()
        .copied()
        .filter(|m| m.name() == method_part)
        .collect();
    if matching.len() < 2 {
        return;
    }
    let names: Vec<String> = matching
        .iter()
        .map(|m| method_overload_name(*m).unwrap_or_else(|| m.name().to_string()))
        .collect();
    warnings.push(format!(
        "filter `{raw}` denies {n} overloads of `{namespace}.{type_name}::{method_part}`: \
         [{names}]; use the overload-disambiguated name (e.g. \
         `!{namespace}.{type_name}::{first}`) to address a single overload\n",
        n = matching.len(),
        names = names.join(", "),
        first = names
            .iter()
            .find(|n| n.as_str() != method_part)
            .map_or(method_part, String::as_str),
    ));
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
