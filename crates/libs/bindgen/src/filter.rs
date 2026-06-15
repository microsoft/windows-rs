use super::*;
use std::collections::{BTreeSet, HashMap};

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
    rules: Vec<(String, bool)>,
    methods: HashMap<(String, String), MethodFilter>,
    /// Types marked with the `?Ns.Type` prefix in `--filter`. Such types are
    /// still emitted (struct, vtable, `_Impl` trait, vtable thunks, IID,
    /// `Interface` impl) so they can be implemented and queried, but the
    /// inherent `impl IFace { fn Method(&self) -> Result<T> { ... } }`
    /// call-side wrapper block is suppressed. Useful for required-but-uncalled
    /// interfaces (e.g. `IPropertyValue` on an `IReference<T>` implementation)
    /// where implementers stub the methods (typically with `E_NOTIMPL`) but
    /// no caller invokes them through this projection.
    trait_only: BTreeSet<(String, String)>,
    /// Types marked with the `??Ns.Type` prefix in `--filter`. Such types
    /// are emitted "skeleton-only": the struct, IID, hierarchy, and
    /// `Interface` impl are kept (so the type can participate in QI and
    /// class hierarchy chains), but every vtable slot is demoted to
    /// `Slot: usize` and the inherent caller-side wrapper block is
    /// suppressed. The `_Impl` trait is omitted via the existing
    /// has-skipped-methods path. Useful for interfaces needed only for
    /// hierarchy / casting (e.g. an abstract base) that the caller never
    /// invokes through this projection.
    full_demote: BTreeSet<(String, String)>,
    /// Warnings collected while parsing `--filter` entries — emitted
    /// alongside the regular bindgen warnings once the `WarningBuilder`
    /// is available. Currently used to flag deny entries that match more
    /// than one `MethodDef` row (i.e. real CLR overloads), so consumers
    /// don't silently lose methods they didn't intend to filter out.
    warnings: Vec<String>,
    /// Enums with specific variants requested (lean+COM mode).
    /// Key: (namespace, type_name), Value: which variants to emit.
    enum_variants: HashMap<(String, String), VariantFilter>,
    /// Types explicitly mentioned in `::Method` filter entries. Used by the
    /// minimal type closure to know which classes/interfaces to start from.
    method_roots: BTreeSet<(String, String)>,
}

/// Which enum variants are requested.
#[derive(Debug, Clone)]
pub enum VariantFilter {
    /// All variants.
    All,
    /// Specific variants by name.
    Names(BTreeSet<String>),
}

impl VariantFilter {
    pub fn includes(&self, name: &str) -> bool {
        match self {
            VariantFilter::All => true,
            VariantFilter::Names(set) => set.contains(name),
        }
    }
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

impl MethodFilter {
    /// Returns true if the given method name should be included.
    /// In minimal closure mode: empty keep set means "all methods", non-empty
    /// means only listed methods.
    pub fn includes_for_closure(&self, method_name: &str) -> bool {
        if self.keep.is_empty() {
            // Empty keep = no method-level filter or ::* = all methods
            return true;
        }
        self.keep.contains(method_name)
    }
}

impl Filter {
    #[track_caller]
    pub fn new(reader: &Reader, include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut trait_only: BTreeSet<(String, String)> = BTreeSet::new();
        let mut full_demote: BTreeSet<(String, String)> = BTreeSet::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut enum_variants: HashMap<(String, String), VariantFilter> = HashMap::new();
        let mut method_roots: BTreeSet<(String, String)> = BTreeSet::new();

        for filter in include {
            push_filter(
                reader,
                &mut rules,
                &mut methods,
                &mut trait_only,
                &mut full_demote,
                &mut warnings,
                &mut enum_variants,
                &mut method_roots,
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
                &mut enum_variants,
                &mut method_roots,
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
            enum_variants,
            method_roots,
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
            return true;
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
    /// Returns `None` if the enum was included as a plain type (all variants).
    pub fn enum_variant_filter(&self, namespace: &str, name: &str) -> Option<&VariantFilter> {
        self.enum_variants
            .get(&(namespace.to_string(), name.to_string()))
    }

    /// Returns the method filter entries for use by the minimal type closure.
    /// Each entry is (namespace, type_name, keep_set) where keep_set is empty
    /// for "all methods" and non-empty for specific methods.
    pub fn method_entries(&self) -> &HashMap<(String, String), MethodFilter> {
        &self.methods
    }

    /// Returns types explicitly mentioned in `::Method` filter entries.
    pub fn method_roots(&self) -> &BTreeSet<(String, String)> {
        &self.method_roots
    }

    /// Returns the include/exclude rules.
    pub fn rules(&self) -> &[(String, bool)] {
        &self.rules
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
    enum_variants: &mut HashMap<(String, String), VariantFilter>,
    method_roots: &mut BTreeSet<(String, String)>,
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
        // Handle ::* and ::{a,b,c} syntax — expand to individual method entries.
        let method_parts: Vec<&str> = if method_part == "*" {
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

        for mp in method_parts {
            push_method_filter(
                reader,
                methods,
                warnings,
                enum_variants,
                method_roots,
                type_part,
                mp,
                include,
                filter,
            );
        }
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
/// expanding property/event sugar against `reader`. `method_part` may be:
/// - `*` — all methods on the type
/// - A single method name (raw or overload-disambiguated)
///
/// For `::*` on classes, all non-static required interface methods are
/// included. For `::*` on enums, all variants are included. The `{a,b}`
/// multi-method syntax is handled by the caller (split into individual
/// calls to this function).
#[track_caller]
#[allow(clippy::too_many_arguments)]
fn push_method_filter(
    reader: &Reader,
    methods: &mut HashMap<(String, String), MethodFilter>,
    warnings: &mut Vec<String>,
    enum_variants: &mut HashMap<(String, String), VariantFilter>,
    method_roots: &mut BTreeSet<(String, String)>,
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

    // Resolve the type.
    let ty = reader
        .with_full_name(namespace, type_name)
        .next()
        .unwrap_or_else(|| panic!("type not found: `{type_part}` (in method filter `{raw}`)"));

    // Record as a method root for minimal closure tracking.
    if include {
        method_roots.insert((namespace.to_string(), type_name.to_string()));
    }

    // Handle ::* (all methods/variants)
    if method_part == "*" {
        match &ty {
            Type::Class(class) => {
                let required = class.required_interfaces(reader);
                for iface in &required {
                    if matches!(iface.kind, InterfaceKind::Static) {
                        continue;
                    }
                    let key = (
                        iface.def.namespace().to_string(),
                        iface.def.name().to_string(),
                    );
                    methods.entry(key).or_default();
                }
            }
            Type::Enum(_) | Type::CppEnum(_) => {
                let key = (namespace.to_string(), type_name.to_string());
                enum_variants.insert(key, VariantFilter::All);
            }
            _ => {
                let key = (namespace.to_string(), type_name.to_string());
                methods.entry(key).or_default();
            }
        }
        return;
    }

    // Handle enum variant entries
    match &ty {
        Type::Enum(e) => {
            assert!(
                e.def.fields().any(
                    |f| f.flags().contains(FieldAttributes::Literal) && f.name() == method_part
                ),
                "variant `{method_part}` not found on enum `{type_part}` \
                 (in filter entry `{raw}`)"
            );
            let key = (namespace.to_string(), type_name.to_string());
            let set = enum_variants
                .entry(key)
                .or_insert_with(|| VariantFilter::Names(BTreeSet::new()));
            if let VariantFilter::Names(names) = set {
                names.insert(method_part.to_string());
            }
            return;
        }
        Type::CppEnum(e) => {
            assert!(
                e.def.fields().any(
                    |f| f.flags().contains(FieldAttributes::Literal) && f.name() == method_part
                ),
                "variant `{method_part}` not found on enum `{type_part}` \
                 (in filter entry `{raw}`)"
            );
            let key = (namespace.to_string(), type_name.to_string());
            let set = enum_variants
                .entry(key)
                .or_insert_with(|| VariantFilter::Names(BTreeSet::new()));
            if let VariantFilter::Names(names) = set {
                names.insert(method_part.to_string());
            }
            return;
        }
        _ => {}
    }

    // Class-level method filter: cascade to required interfaces.
    if let Type::Class(class) = &ty {
        // Special case: `CreateInstance` means "include the class for
        // instantiation". This isn't a real method on any interface — it's
        // a filter-level directive that triggers class emission.
        if method_part == "CreateInstance" {
            // method_roots already recorded above; the class is included
            // in the closure via method_roots. Also include composable
            // factory interfaces so new()/compose() work.
            let required = class.required_interfaces(reader);
            for iface in &required {
                if matches!(iface.kind, InterfaceKind::Composable) {
                    let key = (
                        iface.def.namespace().to_string(),
                        iface.def.name().to_string(),
                    );
                    methods.entry(key).or_default();
                }
            }
            return;
        }

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

        // Also search composable factory interfaces for the method.
        if !any_match {
            for iface in &required {
                if matches!(iface.kind, InterfaceKind::Composable)
                    && iface.def.methods().any(|m| method_matches(m, method_part))
                {
                    any_match = true;
                    break;
                }
            }
        }

        // Include composable factory interfaces so new()/compose() work.
        if any_match {
            for iface in &required {
                if matches!(iface.kind, InterfaceKind::Composable) {
                    let key = (
                        iface.def.namespace().to_string(),
                        iface.def.name().to_string(),
                    );
                    methods.entry(key).or_default();
                }
            }
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
        _ => panic!(
            "type `{type_part}` is not an interface, delegate, \
             enum, or class (in filter entry `{raw}`)"
        ),
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
