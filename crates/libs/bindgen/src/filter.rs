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

/// Whether a type was requested directly or only pulled in as a dependency shell.
#[derive(Debug, Clone)]
pub enum TypeRole {
    /// Explicitly requested - project the given method set.
    Named(MethodSet),
    /// Reachable only as a dependency - project name-only.
    Shell,
}

/// Resolved `--filter` rules: which namespaces, types, methods, and enum variants to
/// include or exclude, plus the seeds used to build the bottom-up type closure.
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
    /// Types whose full metadata hierarchy was explicitly requested.
    hierarchy_types: HashSet<(String, String)>,
    /// Individual class-to-interface hierarchy edges selected through class members.
    hierarchy_interfaces: HashSet<(String, String, String, String)>,
    /// `true` if the filter includes broad entries (a whole namespace)
    /// that are not compatible with bottom-up type closure.
    pub has_broad_filter: bool,
    /// `true` when bottom-up closure is used; non-seed types become shells.
    pub uses_closure: bool,
}

/// Per-type method filter. Both raw metadata names and overload names are recorded;
/// deny entries win over allow entries, and a non-empty allow set makes unlisted methods opt out.
#[derive(Debug, Default)]
pub struct MethodFilter {
    /// Methods explicitly allow-listed.
    keep: BTreeSet<String>,
    /// Methods explicitly deny-listed. Wins over `keep` on overlap.
    drop: BTreeSet<String>,
}

impl Filter {
    pub fn includes_full_hierarchy(&self, namespace: &str, name: &str) -> bool {
        self.hierarchy_types
            .contains(&(namespace.to_string(), name.to_string()))
    }

    pub fn includes_hierarchy(&self, namespace: &str, name: &str, interface: &Interface) -> bool {
        self.includes_full_hierarchy(namespace, name)
            || self.hierarchy_interfaces.contains(&(
                namespace.to_string(),
                name.to_string(),
                interface.def.namespace().to_string(),
                interface.def.name().to_string(),
            ))
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

    /// Returns whether this type projects requested methods or only a name-only shell.
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

    /// Decides whether a method is emitted or demoted to an opaque vtable slot.
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        let key = (
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        );

        let Some(filter) = self.methods.get(&key) else {
            // No method filter: fall back to the type role.
            return match self.type_role(type_name) {
                TypeRole::Named(set) => set.includes(method.name()),
                TypeRole::Shell => false,
            };
        };

        let raw = method.name();
        let overload = method_overload_name(method);

        // Prefer overload names; raw names would match every row in the overload set.
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
        // Empty allow set means deny-only mode; otherwise unlisted methods are opt out.
        filter.keep.is_empty()
    }

    /// Returns the variant filter for a given enum, if one was specified.
    /// Returns `None` if the enum was included as a plain type (all variants kept).
    pub fn enum_variant_filter(&self, namespace: &str, name: &str) -> Option<&MethodSet> {
        self.enum_variants
            .get(&(namespace.to_string(), name.to_string()))
    }

    /// Returns `true` if the class was explicitly marked as activatable
    /// (i.e. `CreateInstance` was in the filter).
    pub fn is_activatable(&self, namespace: &str, name: &str) -> bool {
        self.activatable
            .contains(&(namespace.to_string(), name.to_string()))
    }

    /// Builds the inclusion rules and closure seeds from resolved filter entries.
    #[track_caller]
    pub fn from_resolved(reader: &Reader, entries: &[filter_parser::ResolvedFilter]) -> Self {
        use filter_parser::ResolvedKind;

        let mut rules: Vec<(String, bool)> = Vec::new();
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut enum_variants: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut activatable: HashSet<(String, String)> = HashSet::new();
        let mut requested_interfaces: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut direct_types: Vec<(String, String)> = Vec::new();
        let mut hierarchy_types = HashSet::new();
        let mut hierarchy_interfaces = HashSet::new();
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
                        Self::register_hierarchy_type(
                            reader,
                            namespace,
                            name,
                            &mut hierarchy_types,
                        );
                        // Bare interface mentions seed all methods; other types use `direct_types`.
                        let key = (namespace.clone(), name.clone());
                        if Self::is_interface(reader, namespace, name) {
                            requested_interfaces.entry(key).or_insert(MethodSet::All);
                        } else {
                            // Unscoped enum variants are standalone constants; pull them in explicitly.
                            if Self::is_unscoped_enum(reader, namespace, name) {
                                enum_variants.entry(key.clone()).or_insert(MethodSet::All);
                            }
                            if !direct_types.contains(&key) {
                                direct_types.push(key);
                            }
                        }
                    }
                }
                ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } => {
                    let full = format!("{namespace}.{name}");
                    // Member-level entries always include the type - exclusion
                    // applies to the method/variant, not the type itself.
                    if !rules.iter().any(|(r, _)| r == &full) {
                        rules.push((full, true));
                    }

                    if members.is_empty() {
                        // `Ns.Type::{}` - an explicit name-only shell.
                        if include {
                            let key = (namespace.clone(), name.clone());
                            // An empty variant set makes an enum shell project no
                            // variants, rather than falling through to "all".
                            if Self::is_enum(reader, namespace, name) {
                                enum_variants
                                    .entry(key.clone())
                                    .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                            }
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
                                &mut hierarchy_interfaces,
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
            hierarchy_types,
            hierarchy_interfaces,
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

    /// Whether the named type resolves to an unscoped (C-style) enum, whose
    /// variants are surfaced as standalone constants rather than associated
    /// consts and so must be pulled into the closure explicitly.
    fn is_unscoped_enum(reader: &Reader, namespace: &str, name: &str) -> bool {
        matches!(
            reader.with_full_name(namespace, name).next(),
            Some(Type::CppEnum(e)) if !e.def.has_attribute("ScopedEnumAttribute")
        )
    }

    /// Whether the named type resolves to an enum of either flavor (WinRT/scoped
    /// or unscoped Win32).
    fn is_enum(reader: &Reader, namespace: &str, name: &str) -> bool {
        matches!(
            reader.with_full_name(namespace, name).next(),
            Some(Type::Enum(_) | Type::CppEnum(_))
        )
    }

    fn register_hierarchy_type(
        reader: &Reader,
        namespace: &str,
        name: &str,
        hierarchy_types: &mut HashSet<(String, String)>,
    ) {
        let mut current = reader.with_full_name(namespace, name).next();
        while let Some(ty) = current {
            let Type::Class(class) = ty else {
                let name = ty.type_name();
                hierarchy_types.insert((name.namespace().to_string(), name.name().to_string()));
                return;
            };
            hierarchy_types.insert((
                class.def.namespace().to_string(),
                class.def.name().to_string(),
            ));
            let Some(extends) = class.def.extends() else {
                return;
            };
            if extends == (TypeName::Object.0, TypeName::Object.1) {
                return;
            }
            current = reader
                .with_full_name(extends.namespace(), extends.name())
                .next();
        }
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
        hierarchy_interfaces: &mut HashSet<(String, String, String, String)>,
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

                        // A composable class uses its parameterless factory method as `new()`.
                        // Select that constructor only; other overloads are independent APIs.
                        for iface in required
                            .iter()
                            .filter(|iface| iface.kind == InterfaceKind::Composable)
                        {
                            let defs: Vec<MethodDef> = iface.def.methods().collect();
                            let expanded = expand_method_part(member, &defs);
                            if expanded.is_empty() {
                                continue;
                            }
                            let iface_key = (
                                iface.def.namespace().to_string(),
                                iface.def.name().to_string(),
                            );
                            let set = requested_interfaces
                                .entry(iface_key.clone())
                                .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                            if let MethodSet::Names(names) = set {
                                names.extend(expanded.iter().cloned());
                            }
                            methods.entry(iface_key).or_default().keep.extend(expanded);
                        }
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
                            if include {
                                hierarchy_interfaces.insert((
                                    namespace.to_string(),
                                    name.to_string(),
                                    iface_key.0.clone(),
                                    iface_key.1.clone(),
                                ));
                            }
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
                }
                Type::Interface(_) | Type::CppInterface(_) | Type::Delegate(_) => {
                    // Register property/event sugar under the real metadata method names.
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
                    // The closure walk only follows these requested method signatures.
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

    // Bare property/event names expand to their accessor pairs; property wins if both match.
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

    // Overload names address the row carrying that `[overload("...")]` value.
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
