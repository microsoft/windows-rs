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
    /// Types marked with the `?Ns.Type` prefix in `--filter`.
    trait_only: BTreeSet<(String, String)>,
    /// Types marked with the `??Ns.Type` prefix in `--filter`.
    full_demote: BTreeSet<(String, String)>,
    warnings: Vec<String>,
    /// Enums with specific variants requested.
    enum_variants: HashMap<(String, String), MethodSet>,
    /// Classes that explicitly requested `CreateInstance`.
    activatable: HashSet<(String, String)>,
    /// When `true`, methods on types with no explicit MethodFilter entry are
    /// demoted by default (minimal/opt-in mode).
    default_demote: bool,
    /// Interfaces with specific methods requested (for type closure in minimal mode).
    /// Key: (namespace, type_name), Value: requested method names (or All).
    pub requested_interfaces: HashMap<(String, String), MethodSet>,
    /// Types directly included without `::` (for type closure in minimal mode).
    pub direct_types: Vec<(String, String)>,
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
    #[track_caller]
    pub fn new(reader: &Reader, include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut trait_only: BTreeSet<(String, String)> = BTreeSet::new();
        let mut full_demote: BTreeSet<(String, String)> = BTreeSet::new();
        let mut warnings: Vec<String> = Vec::new();

        for filter in include {
            push_filter(
                reader,
                &mut rules,
                &mut methods,
                &mut trait_only,
                &mut full_demote,
                &mut warnings,
                filter,
                true,
            );
        }

        for filter in exclude {
            push_filter(
                reader,
                &mut rules,
                &mut methods,
                &mut trait_only,
                &mut full_demote,
                &mut warnings,
                filter,
                false,
            );
        }

        debug_assert!(!rules.is_empty() || !methods.is_empty());

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self {
            rules,
            methods,
            trait_only,
            full_demote,
            warnings,
            enum_variants: HashMap::new(),
            activatable: HashSet::new(),
            default_demote: false,
            requested_interfaces: HashMap::new(),
            direct_types: Vec::new(),
        }
    }

    /// Warnings collected while parsing `--filter` entries. Surface these
    /// through the regular `WarningBuilder` so the user sees them in the
    /// final report.
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
        // `??Ns.Type` (full vtable demotion) overrides any per-method
        // filter: the entire vtable is opaque for this type.
        if self.full_demote.contains(&(
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        )) {
            return false;
        }

        let Some(filter) = self.methods.get(&(
            type_name.namespace().to_string(),
            type_name.name().to_string(),
        )) else {
            return !self.default_demote;
        };

        let raw = method.name();
        let overload = method_overload_name(method);

        let in_set = |set: &BTreeSet<String>| -> bool {
            set.contains(raw)
                || overload
                    .as_ref()
                    .is_some_and(|name| set.contains(name.as_str()))
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

    /// Returns `true` if `name` was marked with the `?Ns.Type` or
    /// `??Ns.Type` prefix in `--filter`, indicating that its inherent
    /// method-wrapper block should be suppressed. The type is still
    /// emitted (struct, vtable, `_Impl` trait when fully-typed, vtable
    /// thunks, IID, `Interface` impl) so implementers can stub its
    /// methods and the ABI is preserved; only the caller-side
    /// `impl IFace { fn X(&self) -> Result<T> { ... } }` block is skipped.
    pub fn is_trait_only(&self, name: TypeName) -> bool {
        let key = (name.namespace().to_string(), name.name().to_string());
        self.trait_only.contains(&key) || self.full_demote.contains(&key)
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
    pub fn new_minimal(reader: &Reader, include: &[&str]) -> Self {
        let mut filter = Self {
            default_demote: true,
            ..Default::default()
        };

        for entry in include {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }

            if let Some((type_part, method_part)) = entry.split_once("::") {
                filter.parse_minimal_method_entry(reader, entry, type_part, method_part);
            } else {
                filter.parse_minimal_type_entry(reader, entry);
            }
        }

        assert!(
            !(filter.requested_interfaces.is_empty() && filter.direct_types.is_empty()),
            "at least one `--filter` entry required for minimal mode"
        );

        // Build type-level rules for every type we've collected (needed by
        // the codegen pipeline's includes_type_name / includes_namespace).
        for (namespace, name) in &filter.direct_types {
            let rule = format!("{namespace}.{name}");
            if !filter.rules.iter().any(|(r, _)| r == &rule) {
                filter.rules.push((rule, true));
            }
        }
        for (namespace, name) in filter.requested_interfaces.keys() {
            let rule = format!("{namespace}.{name}");
            if !filter.rules.iter().any(|(r, _)| r == &rule) {
                filter.rules.push((rule, true));
            }
        }

        filter
    }

    #[track_caller]
    fn parse_minimal_method_entry(
        &mut self,
        reader: &Reader,
        entry: &str,
        type_part: &str,
        method_part: &str,
    ) {
        let (namespace, type_name) = type_part.rsplit_once('.').unwrap_or_else(|| {
            panic!(
                "invalid filter entry `{entry}`: \
                 type must be fully qualified (e.g. `Namespace.IFoo::Method`)"
            )
        });

        let resolved_ns = reader
            .keys()
            .find(|ns| *ns == &namespace)
            .unwrap_or_else(|| {
                panic!("namespace not found: `{namespace}` (in filter entry `{entry}`)")
            });
        let resolved_name = reader[resolved_ns]
            .keys()
            .find(|n| *n == &type_name)
            .unwrap_or_else(|| panic!("type not found: `{type_part}` (in filter entry `{entry}`)"));

        // Parse method specifier: *, {a, b}, or single name.
        let method_names: Vec<&str> = if method_part == "*" {
            vec!["*"]
        } else if method_part.starts_with('{') && method_part.ends_with('}') {
            method_part[1..method_part.len() - 1]
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            vec![method_part]
        };

        for method_name in method_names {
            if method_name == "*" {
                self.expand_star(reader, entry, resolved_ns, resolved_name);
            } else {
                self.expand_method(
                    reader,
                    entry,
                    type_part,
                    method_name,
                    resolved_ns,
                    resolved_name,
                );
            }
        }
    }

    #[track_caller]
    fn expand_star(&mut self, reader: &Reader, entry: &str, namespace: &str, name: &str) {
        let ty = reader.unwrap_full_name(namespace, name);
        let ns_key = namespace.to_string();
        let name_key = name.to_string();

        match &ty {
            Type::Class(c) => {
                if !self
                    .direct_types
                    .contains(&(ns_key.clone(), name_key.clone()))
                {
                    self.direct_types.push((ns_key.clone(), name_key.clone()));
                }
                self.activatable.insert((ns_key, name_key));
                for iface in c.required_interfaces(reader) {
                    let iface_ns = iface.def.namespace().to_string();
                    let iface_name = iface.def.name().to_string();
                    let ikey = (iface_ns, iface_name);
                    self.requested_interfaces
                        .entry(ikey)
                        .or_insert(MethodSet::All);
                }
            }
            Type::Enum(_) | Type::CppEnum(_) => {
                self.enum_variants
                    .insert((ns_key.clone(), name_key.clone()), MethodSet::All);
                if !self
                    .direct_types
                    .contains(&(ns_key.clone(), name_key.clone()))
                {
                    self.direct_types.push((ns_key, name_key));
                }
            }
            _ => {
                let _ = entry;
                self.requested_interfaces
                    .insert((ns_key, name_key), MethodSet::All);
            }
        }
    }

    #[track_caller]
    #[allow(clippy::too_many_arguments)]
    fn expand_method(
        &mut self,
        reader: &Reader,
        entry: &str,
        type_part: &str,
        method_name: &str,
        namespace: &str,
        name: &str,
    ) {
        let ty = reader.unwrap_full_name(namespace, name);
        let ns_key = namespace.to_string();
        let name_key = name.to_string();

        match &ty {
            Type::Interface(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` (in filter entry `{entry}`)"
                );
                self.insert_requested_method(&ns_key, &name_key, method_name);
            }
            Type::CppInterface(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` (in filter entry `{entry}`)"
                );
                self.insert_requested_method(&ns_key, &name_key, method_name);
            }
            Type::Delegate(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` (in filter entry `{entry}`)"
                );
                self.insert_requested_method(&ns_key, &name_key, method_name);
            }
            Type::Class(c) => {
                self.expand_class_method(
                    reader,
                    entry,
                    type_part,
                    method_name,
                    &ns_key,
                    &name_key,
                    c,
                );
            }
            Type::Enum(e) => {
                assert!(
                    e.def
                        .fields()
                        .any(|f| f.flags().contains(FieldAttributes::Literal)
                            && f.name() == method_name),
                    "variant `{method_name}` not found on enum `{type_part}` (in filter entry `{entry}`)"
                );
                self.insert_enum_variant(&ns_key, &name_key, method_name);
            }
            Type::CppEnum(e) => {
                assert!(
                    e.def
                        .fields()
                        .any(|f| f.flags().contains(FieldAttributes::Literal)
                            && f.name() == method_name),
                    "variant `{method_name}` not found on enum `{type_part}` (in filter entry `{entry}`)"
                );
                self.insert_enum_variant(&ns_key, &name_key, method_name);
            }
            _ => panic!(
                "type `{type_part}` is not an interface, delegate, enum, or class (in filter entry `{entry}`)"
            ),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn expand_class_method(
        &mut self,
        reader: &Reader,
        entry: &str,
        type_part: &str,
        method_name: &str,
        ns_key: &str,
        name_key: &str,
        class: &Class,
    ) {
        let required = class.required_interfaces(reader);
        let mut found = false;

        if method_name == "CreateInstance" {
            if !self
                .direct_types
                .contains(&(ns_key.to_string(), name_key.to_string()))
            {
                self.direct_types
                    .push((ns_key.to_string(), name_key.to_string()));
            }
            self.activatable
                .insert((ns_key.to_string(), name_key.to_string()));
            found = true;
        }

        if !found {
            for iface in &required {
                if iface.def.methods().any(|m| method_matches(m, method_name)) {
                    let iface_ns = iface.def.namespace().to_string();
                    let iface_name = iface.def.name().to_string();
                    self.insert_requested_method(&iface_ns, &iface_name, method_name);
                    if !self
                        .direct_types
                        .contains(&(ns_key.to_string(), name_key.to_string()))
                    {
                        self.direct_types
                            .push((ns_key.to_string(), name_key.to_string()));
                    }
                    found = true;
                    break;
                }
            }
        }

        if !found {
            for iface in &required {
                if matches!(iface.kind, InterfaceKind::Composable)
                    && iface.def.methods().any(|m| method_matches(m, method_name))
                {
                    found = true;
                    break;
                }
            }
        }

        assert!(
            found,
            "method `{method_name}` not found on class `{type_part}` (in filter entry `{entry}`)"
        );

        // Include composable factory interfaces so new() works.
        for iface in &required {
            if matches!(iface.kind, InterfaceKind::Composable) {
                let iface_ns = iface.def.namespace().to_string();
                let iface_name = iface.def.name().to_string();
                self.requested_interfaces
                    .entry((iface_ns, iface_name))
                    .or_insert(MethodSet::All);
            }
        }
    }

    #[track_caller]
    fn parse_minimal_type_entry(&mut self, reader: &Reader, entry: &str) {
        let (namespace, type_name) = entry.rsplit_once('.').unwrap_or_else(|| {
            panic!(
                "invalid filter entry `{entry}`: \
                 must be fully qualified (e.g. `Namespace.TypeName`)"
            )
        });

        let resolved_ns = reader
            .keys()
            .find(|ns| *ns == &namespace)
            .unwrap_or_else(|| {
                panic!("namespace not found: `{namespace}` (in filter entry `{entry}`)")
            });
        let resolved_name = reader[resolved_ns]
            .keys()
            .find(|n| *n == &type_name)
            .unwrap_or_else(|| panic!("type not found: `{entry}`"));

        self.direct_types
            .push((resolved_ns.to_string(), resolved_name.to_string()));
    }

    fn insert_requested_method(&mut self, namespace: &str, name: &str, method_name: &str) {
        let key = (namespace.to_string(), name.to_string());

        // Update requested_interfaces for type closure.
        let set = self
            .requested_interfaces
            .entry(key.clone())
            .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
        match set {
            MethodSet::All => {
                let mut names = BTreeSet::new();
                names.insert(method_name.to_string());
                *set = MethodSet::Names(names);
            }
            MethodSet::Names(names) => {
                names.insert(method_name.to_string());
            }
        }

        // Update methods for emission filtering.
        let entry = self.methods.entry(key).or_default();
        entry.keep.insert(method_name.to_string());
        // Also keep `remove_X` when `add_X` is requested.
        if let Some(event) = method_name.strip_prefix("add_") {
            let remove_name = format!("remove_{event}");
            entry.keep.insert(remove_name);
        }
    }

    fn insert_enum_variant(&mut self, namespace: &str, name: &str, variant: &str) {
        let key = (namespace.to_string(), name.to_string());
        let set = self
            .enum_variants
            .entry(key.clone())
            .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
        match set {
            MethodSet::All => {}
            MethodSet::Names(names) => {
                names.insert(variant.to_string());
            }
        }
        if !self.direct_types.contains(&key) {
            self.direct_types.push(key);
        }
    }

    /// Build a `Filter` from resolved filter entries (new unified syntax).
    ///
    /// This constructor handles both minimal and non-minimal modes uniformly.
    /// The `default_demote` flag is set based on whether `--minimal` is active,
    /// which controls whether methods on types without explicit `::` member
    /// specs are emitted or suppressed.
    #[track_caller]
    #[allow(clippy::redundant_clone)]
    pub fn from_resolved(
        reader: &Reader,
        entries: &[filter_parser::ResolvedFilter],
        default_demote: bool,
    ) -> Self {
        use filter_parser::ResolvedKind;

        let mut rules: Vec<(String, bool)> = Vec::new();
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut trait_only: BTreeSet<(String, String)> = BTreeSet::new();
        let mut full_demote: BTreeSet<(String, String)> = BTreeSet::new();
        let mut enum_variants: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut activatable: HashSet<(String, String)> = HashSet::new();
        let mut requested_interfaces: HashMap<(String, String), MethodSet> = HashMap::new();
        let mut direct_types: Vec<(String, String)> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();

        for entry in entries {
            let include = !entry.exclude;

            match &entry.kind {
                ResolvedKind::Namespace(ns) => {
                    rules.push((ns.clone(), include));
                }
                ResolvedKind::NameGlob { namespace, prefix } => {
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

                    if entry.trait_only {
                        trait_only.insert((namespace.clone(), name.clone()));
                        if entry.skeleton_only {
                            full_demote.insert((namespace.clone(), name.clone()));
                        }
                    }

                    if include && default_demote {
                        // In minimal mode, a bare type entry means "all methods"
                        // — register in requested_interfaces for type closure
                        // and expand class methods.
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
                }
                ResolvedKind::Members {
                    namespace,
                    name,
                    members,
                } => {
                    let full = format!("{namespace}.{name}");
                    if !rules.iter().any(|(r, _)| r == &full) {
                        rules.push((full, include));
                    }

                    if entry.trait_only {
                        trait_only.insert((namespace.clone(), name.clone()));
                        if entry.skeleton_only {
                            full_demote.insert((namespace.clone(), name.clone()));
                        }
                    }

                    // Register each member
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
                            default_demote,
                        );
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
            trait_only,
            full_demote,
            warnings,
            enum_variants,
            activatable,
            default_demote,
            requested_interfaces,
            direct_types,
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
        default_demote: bool,
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
                    // Route to the class's required interfaces
                    if member == "CreateInstance" {
                        if !direct_types.contains(&key) {
                            direct_types.push(key.clone());
                        }
                        activatable.insert(key);
                    } else {
                        // Find which interface carries this method
                        let required = class.required_interfaces(reader);
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
                        // Include composable factory interfaces
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
                }
                Type::Interface(_) | Type::CppInterface(_) | Type::Delegate(_) => {
                    // Use existing method filter expansion (property/event sugar)
                    if default_demote {
                        // Minimal mode: register in requested_interfaces
                        let set = requested_interfaces
                            .entry(key.clone())
                            .or_insert_with(|| MethodSet::Names(BTreeSet::new()));
                        if let MethodSet::Names(names) = set {
                            names.insert(member.to_string());
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
                    let expanded = expand_method_part(member, &defs);
                    assert!(
                        !expanded.is_empty(),
                        "method `{member}` not found on `{namespace}.{name}`"
                    );
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
fn push_filter(
    reader: &Reader,
    rules: &mut Vec<(String, bool)>,
    methods: &mut HashMap<(String, String), MethodFilter>,
    trait_only: &mut BTreeSet<(String, String)>,
    full_demote: &mut BTreeSet<(String, String)>,
    warnings: &mut Vec<String>,
    filter: &str,
    include: bool,
) {
    // `??Ns.Type` marks a type as "skeleton-only": the struct, IID,
    // hierarchy, and `Interface` impl are emitted (so QI / class-hierarchy
    // chains still work) but every vtable slot is demoted to `Slot: usize`
    // and the inherent caller-side wrapper block is suppressed. Useful for
    // base interfaces needed only for hierarchy / casting whose methods
    // the caller never invokes through this projection.
    //
    // `?Ns.Type` marks a type as "trait-only": included in the closure with
    // its `_Impl` trait and vtable but without the caller-side method wrapper
    // block. Only valid on include entries — exclusion is orthogonal.
    let (filter, mark_trait_only, mark_full_demote) = if let Some(rest) = filter.strip_prefix("??")
    {
        assert!(
            include,
            "cannot combine `??` (skeleton-only) with `!` (exclude) on the same filter entry: `{filter}`"
        );
        assert!(
            !rest.contains("::"),
            "`??` (skeleton-only) cannot be combined with a method-level filter: `{filter}`"
        );
        (rest, true, true)
    } else if let Some(rest) = filter.strip_prefix('?') {
        assert!(
            include,
            "cannot combine `?` (trait-only) with `!` (exclude) on the same filter entry: `{filter}`"
        );
        assert!(
            !rest.contains("::"),
            "`?` (trait-only) cannot be combined with a method-level filter: `{filter}`"
        );
        (rest, true, false)
    } else {
        (filter, false, false)
    };

    if let Some((type_part, method_part)) = filter.split_once("::") {
        push_method_filter(
            reader,
            methods,
            warnings,
            type_part,
            method_part,
            include,
            filter,
        );
        return;
    }

    let record_markers = |trait_only: &mut BTreeSet<(String, String)>,
                          full_demote: &mut BTreeSet<(String, String)>,
                          full: &str| {
        if !mark_trait_only {
            return;
        }
        if let Some((ns, name)) = full.rsplit_once('.') {
            trait_only.insert((ns.to_string(), name.to_string()));
            if mark_full_demote {
                full_demote.insert((ns.to_string(), name.to_string()));
            }
        }
    };

    if reader.contains_key(filter) {
        if mark_trait_only {
            let prefix = if mark_full_demote { "??" } else { "?" };
            let kind = if mark_full_demote {
                "skeleton-only"
            } else {
                "trait-only"
            };
            panic!(
                "`{prefix}` ({kind}) requires a fully-qualified `Namespace.Type` entry, not a namespace: `{filter}`"
            );
        }
        rules.push((filter.to_string(), include));
        return;
    }

    if let Some((namespace, name)) = filter.rsplit_once('.') {
        if reader.with_full_name(namespace, name).next().is_some() {
            rules.push((filter.to_string(), include));
            record_markers(trait_only, full_demote, filter);
            return;
        }

        if let Some(starts_with) = name.strip_suffix('*') {
            if mark_trait_only {
                let prefix = if mark_full_demote { "??" } else { "?" };
                let kind = if mark_full_demote {
                    "skeleton-only"
                } else {
                    "trait-only"
                };
                panic!("`{prefix}` ({kind}) cannot be combined with a wildcard filter: `{filter}`");
            }
            if let Some(types) = reader.get(namespace) {
                let prev_len = rules.len();

                for name in types.keys() {
                    if name.starts_with(starts_with) {
                        rules.push((format!("{namespace}.{name}"), include));
                    }
                }

                if prev_len != rules.len() {
                    return;
                }
            }
        }
    }

    let mut pushed = false;

    for (namespace, types) in reader.iter() {
        if types.get(filter).is_some() {
            rules.push((format!("{namespace}.{filter}"), include));
            if mark_trait_only {
                trait_only.insert((namespace.to_string(), filter.to_string()));
                if mark_full_demote {
                    full_demote.insert((namespace.to_string(), filter.to_string()));
                }
            }
            pushed = true;
        }
    }

    if pushed {
        return;
    }

    if reader
        .keys()
        .any(|namespace| namespace_starts_with(namespace, filter))
    {
        if mark_trait_only {
            let prefix = if mark_full_demote { "??" } else { "?" };
            let kind = if mark_full_demote {
                "skeleton-only"
            } else {
                "trait-only"
            };
            panic!(
                "`{prefix}` ({kind}) requires a fully-qualified `Namespace.Type` entry, not a namespace: `{prefix}{filter}`"
            );
        }
        rules.push((filter.to_string(), include));
        return;
    }

    panic!("type not found: `{filter}`");
}

/// Parse and record a `Ns.Type::Method` (or `!Ns.Type::Method`) entry,
/// expanding property/event sugar against `reader`. `method_part` may also
/// be an overload-disambiguated Rust name produced by `[overload("…")]`
/// (e.g. `InsertKeyFrameWithEasingFunction`), in which case it resolves to
/// the single `MethodDef` row carrying that attribute value rather than
/// every row that happens to share the metadata name.
#[track_caller]
fn push_method_filter(
    reader: &Reader,
    methods: &mut HashMap<(String, String), MethodFilter>,
    warnings: &mut Vec<String>,
    type_part: &str,
    method_part: &str,
    include: bool,
    raw: &str,
) {
    assert!(
        !(method_part.is_empty() || method_part.contains("::")),
        "invalid method filter `{raw}`: expected `Namespace.Type::Method`"
    );
    assert!(
        !type_part.contains("::"),
        "invalid method filter `{raw}`: expected `Namespace.Type::Method`"
    );

    let (namespace, type_name) = type_part
        .rsplit_once('.')
        .unwrap_or_else(|| panic!("invalid method filter `{raw}`: expected `Namespace.Type::Method` (the type part must be fully qualified, e.g. `Windows.Foundation.IStringable::ToString`)"));

    // Resolve the type. We need access to its `MethodDef` rows so we can
    // validate the entry and expand property/event sugar.
    let ty = reader
        .with_full_name(namespace, type_name)
        .next()
        .unwrap_or_else(|| panic!("type not found: `{type_part}` (in method filter `{raw}`)"));

    // Class-level method filter: WinRT runtime classes don't carry their own
    // `MethodDef` rows — the methods exposed via `impl Class { … }` are
    // forwarders generated from each required interface (instance default,
    // static factory, activation/composable factory, base interfaces).
    // Cascade the entry to every required interface that actually exposes
    // the named method so the existing per-interface demotion + class
    // forwarder drop in `class.rs` kicks in for free.
    if let Type::Class(class) = &ty {
        let required = class.required_interfaces(reader);
        let mut any_match = false;
        let mut searched: Vec<String> = Vec::new();

        for iface in &required {
            let defs: Vec<MethodDef> = iface.def.methods().collect();
            searched.push(format!("{}.{}", iface.def.namespace(), iface.def.name()));

            let expanded = expand_method_part(method_part, &defs);
            if expanded.is_empty() {
                continue;
            }
            any_match = true;
            maybe_warn_ambiguous_overload(
                warnings,
                method_part,
                iface.def.namespace(),
                iface.def.name(),
                &defs,
                include,
                raw,
            );
            register_method_filter(
                methods,
                iface.def.namespace(),
                iface.def.name(),
                expanded,
                include,
            );
        }

        assert!(
            any_match,
            "method `{method_part}` not found on `{type_part}` or any of its \
             required interfaces (in method filter `{raw}`); searched: [{}]",
            searched.join(", ")
        );
        return;
    }

    let def = match &ty {
        Type::Interface(t) => t.def,
        Type::CppInterface(t) => t.def,
        Type::Delegate(t) => t.def,
        _ => panic!("type not found: `{type_part}` (in method filter `{raw}`)"),
    };
    let defs: Vec<MethodDef> = def.methods().collect();

    let expanded = expand_method_part(method_part, &defs);
    assert!(
        !expanded.is_empty(),
        "method `{method_part}` not found on `{type_part}` \
         (in method filter `{raw}`)"
    );

    maybe_warn_ambiguous_overload(
        warnings,
        method_part,
        namespace,
        type_name,
        &defs,
        include,
        raw,
    );
    register_method_filter(methods, namespace, type_name, expanded, include);
}

/// Resolve `method_part` against `defs`. Returns the names to register on
/// this type's filter, after sugar / overload-name expansion. Empty when
/// no match is found.
///
/// Resolution order:
/// 1. Accessor-only sugar: `get:X` / `set:X` / `add:X` / `remove:X`
///    addresses a single property/event accessor (`get_X`, `put_X`,
///    `add_X`, `remove_X` respectively).
/// 2. Exact match against a raw `MethodDef` name (e.g. `IFoo::get_Value`
///    or `IFoo::Bar`).
/// 3. Property/event sugar: `Bar` -> `get_Bar` + `put_Bar`, falling back
///    to `add_Bar` + `remove_Bar`.
/// 4. Overload-disambiguated Rust name produced by `[overload("…")]`
///    (e.g. `InsertKeyFrameWithEasingFunction`). This addresses a single
///    `MethodDef` row even when several rows share the same metadata name.
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
