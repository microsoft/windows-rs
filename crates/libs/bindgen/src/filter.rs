use super::*;
use std::collections::{BTreeSet, HashMap};

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
    /// Warnings collected while parsing `--filter` entries — emitted
    /// alongside the regular bindgen warnings once the `WarningBuilder`
    /// is available. Currently used to flag deny entries that match more
    /// than one `MethodDef` row (i.e. real CLR overloads), so consumers
    /// don't silently lose methods they didn't intend to filter out.
    warnings: Vec<String>,
}

/// Per-type method filter. Entries may be either:
/// * an exact `MethodDef` name (after property/event sugar expansion, e.g.
///   `Property` -> `get_Property` + `put_Property`), or
/// * an overload-disambiguated Rust name produced by `[overload("…")]`
///   (e.g. `InsertKeyFrameWithEasingFunction` on the `InsertKeyFrame`
///   MethodDef row).
///
/// At lookup time both the raw `MethodDef` name and any overload name are
/// checked against the set, so a single entry can address either form.
#[derive(Debug)]
pub enum MethodFilter {
    /// Allowlist: only the listed methods are kept; others demote to `Slot: usize`.
    Keep(BTreeSet<String>),
    /// Denylist: the listed methods demote to `Slot: usize`; others kept.
    Exclude(BTreeSet<String>),
}

impl Filter {
    #[track_caller]
    pub fn new(reader: &Reader, include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];
        let mut methods: HashMap<(String, String), MethodFilter> = HashMap::new();
        let mut trait_only: BTreeSet<(String, String)> = BTreeSet::new();
        let mut warnings: Vec<String> = Vec::new();

        for filter in include {
            push_filter(
                reader,
                &mut rules,
                &mut methods,
                &mut trait_only,
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
            warnings,
        }
    }

    /// Warnings collected while parsing `--filter` entries. Surface these
    /// through the regular `WarningBuilder` so the user sees them in the
    /// final report.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Validate that no method-level filter entry targets a type matched by
    /// `--implements`. Methods on implemented types must always be emitted.
    #[track_caller]
    pub fn validate_implements(&self, implements: &Implements) {
        if implements.is_empty() {
            return;
        }
        for (namespace, name) in self.methods.keys() {
            if implements.matches_str(namespace, name) {
                panic!(
                    "method-level filter on `{namespace}.{name}` conflicts with `--implements`: \
                     methods on implemented interfaces are always emitted"
                );
            }
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
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
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

        match filter {
            MethodFilter::Keep(set) => in_set(set),
            MethodFilter::Exclude(set) => !in_set(set),
        }
    }

    /// Returns `true` if `name` was marked with the `?Ns.Type` prefix in
    /// `--filter`, indicating that its inherent method-wrapper block should
    /// be suppressed. The type is still emitted (struct, vtable, `_Impl`
    /// trait, vtable thunks, IID, `Interface` impl) so implementers can
    /// stub its methods and the ABI is preserved; only the caller-side
    /// `impl IFace { fn X(&self) -> Result<T> { ... } }` block is skipped.
    pub fn is_trait_only(&self, name: TypeName) -> bool {
        self.trait_only
            .contains(&(name.namespace().to_string(), name.name().to_string()))
    }
}

#[track_caller]
fn push_filter(
    reader: &Reader,
    rules: &mut Vec<(String, bool)>,
    methods: &mut HashMap<(String, String), MethodFilter>,
    trait_only: &mut BTreeSet<(String, String)>,
    warnings: &mut Vec<String>,
    filter: &str,
    include: bool,
) {
    // `?Ns.Type` marks a type as "trait-only": included in the closure with
    // its `_Impl` trait and vtable but without the caller-side method wrapper
    // block. Only valid on include entries — exclusion is orthogonal.
    let (filter, mark_trait_only) = if let Some(rest) = filter.strip_prefix('?') {
        if !include {
            panic!("cannot combine `?` (trait-only) with `!` (exclude) on the same filter entry: `{filter}`");
        }
        if rest.contains("::") {
            panic!("`?` (trait-only) cannot be combined with a method-level filter: `{filter}`");
        }
        (rest, true)
    } else {
        (filter, false)
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

    let record_trait_only = |trait_only: &mut BTreeSet<(String, String)>, full: &str| {
        if !mark_trait_only {
            return;
        }
        if let Some((ns, name)) = full.rsplit_once('.') {
            trait_only.insert((ns.to_string(), name.to_string()));
        }
    };

    if reader.contains_key(filter) {
        if mark_trait_only {
            panic!("`?` (trait-only) requires a fully-qualified `Namespace.Type` entry, not a namespace: `{filter}`");
        }
        rules.push((filter.to_string(), include));
        return;
    }

    if let Some((namespace, name)) = filter.rsplit_once('.') {
        if reader.with_full_name(namespace, name).next().is_some() {
            rules.push((filter.to_string(), include));
            record_trait_only(trait_only, filter);
            return;
        }

        if let Some(starts_with) = name.strip_suffix('*') {
            if mark_trait_only {
                panic!("`?` (trait-only) cannot be combined with a wildcard filter: `{filter}`");
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
            panic!("`?` (trait-only) requires a fully-qualified `Namespace.Type` entry, not a namespace: `?{filter}`");
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
    if method_part.is_empty() || method_part.contains("::") {
        panic!("invalid method filter `{raw}`: expected `Namespace.Type::Method`");
    }
    if type_part.contains("::") {
        panic!("invalid method filter `{raw}`: expected `Namespace.Type::Method`");
    }

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
                type_part,
                raw,
            );
        }

        if !any_match {
            panic!(
                "method `{method_part}` not found on `{type_part}` or any of its \
                 required interfaces (in method filter `{raw}`); searched: [{}]",
                searched.join(", ")
            );
        }
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
    if expanded.is_empty() {
        panic!(
            "method `{method_part}` not found on `{type_part}` \
             (in method filter `{raw}`)"
        );
    }

    maybe_warn_ambiguous_overload(
        warnings,
        method_part,
        namespace,
        type_name,
        &defs,
        include,
        raw,
    );
    register_method_filter(
        methods, namespace, type_name, expanded, include, type_part, raw,
    );
}

/// Resolve `method_part` against `defs`. Returns the names to register on
/// this type's filter, after sugar / overload-name expansion. Empty when
/// no match is found.
///
/// Resolution order:
/// 1. Exact match against a raw `MethodDef` name (e.g. `IFoo::get_Value`
///    or `IFoo::Bar`).
/// 2. Property/event sugar: `Bar` -> `get_Bar` + `put_Bar`, falling back
///    to `add_Bar` + `remove_Bar`.
/// 3. Overload-disambiguated Rust name produced by `[overload("…")]`
///    (e.g. `InsertKeyFrameWithEasingFunction`). This addresses a single
///    `MethodDef` row even when several rows share the same metadata name.
fn expand_method_part(method_part: &str, defs: &[MethodDef]) -> Vec<String> {
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

#[track_caller]
fn register_method_filter(
    methods: &mut HashMap<(String, String), MethodFilter>,
    namespace: &str,
    type_name: &str,
    expanded: Vec<String>,
    include: bool,
    type_part: &str,
    raw: &str,
) {
    let key = (namespace.to_string(), type_name.to_string());
    let entry = methods.entry(key).or_insert_with(|| {
        if include {
            MethodFilter::Keep(BTreeSet::new())
        } else {
            MethodFilter::Exclude(BTreeSet::new())
        }
    });

    match (entry, include) {
        (MethodFilter::Keep(set), true) => set.extend(expanded),
        (MethodFilter::Exclude(set), false) => set.extend(expanded),
        _ => panic!(
            "cannot mix allow (`Ns.Type::Method`) and deny (`!Ns.Type::Method`) \
             method filter entries on the same type `{type_part}` (in `{raw}`)"
        ),
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
