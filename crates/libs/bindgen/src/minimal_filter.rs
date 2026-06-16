use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Returns true if `method_name` matches either the raw metadata name or the
/// overload-disambiguated name of `m`.
fn method_matches(m: MethodDef, method_name: &str) -> bool {
    if m.name() == method_name {
        return true;
    }
    method_overload_name(m).as_deref() == Some(method_name)
}

/// Method-centric filter for `--minimal` mode.
///
/// Instead of the traditional type-level include/exclude filter, this parser
/// treats each entry as either:
///
/// - `Namespace.Interface::method` — request a specific method slot (raw name)
/// - `Namespace.Interface::*` — request all method slots on an interface
/// - `Namespace.Class::method` — resolve through the class's interfaces
/// - `Namespace.Type` — directly include a type (function, struct, enum, etc.)
///
/// Method names must be raw metadata names (`put_Width`, `get_Width`,
/// `add_Tapped`, `remove_Tapped`).
///
/// The type closure is computed automatically from method signatures:
/// only types transitively required by the requested methods are emitted.
#[derive(Debug, Default)]
pub struct MinimalFilter {
    /// Interfaces with specific methods requested.
    /// Key: (namespace, type_name), Value: requested method names (or `All`).
    pub interfaces: HashMap<(&'static str, &'static str), MethodSet>,

    /// Types directly included (no `::` — free functions, structs, enums, etc.).
    pub types: Vec<(&'static str, &'static str)>,

    /// Enums with specific variants requested.
    /// Key: (namespace, type_name), Value: requested variant names (or `All`).
    pub enum_variants: HashMap<(&'static str, &'static str), MethodSet>,

    /// Classes that explicitly requested `CreateInstance` in the filter.
    /// Used to decide whether to emit the `IActivationFactory` default
    /// constructor (`new()`).
    pub activatable: HashSet<(&'static str, &'static str)>,
}

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

impl MinimalFilter {
    /// Parse filter entries using the simplified minimal-mode syntax.
    #[track_caller]
    pub fn new(reader: &Reader, entries: &[&str]) -> Self {
        let mut filter = Self::default();

        for entry in entries {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }

            if let Some((type_part, method_part)) = entry.split_once("::") {
                // Method entry: Namespace.Type::method, ::*, or ::{a, b, c}
                let (namespace, type_name) = type_part.rsplit_once('.').unwrap_or_else(|| {
                    panic!(
                        "invalid minimal filter entry `{entry}`: \
                         type must be fully qualified (e.g. `Namespace.IFoo::Method`)"
                    )
                });

                // Validate the type exists in the reader.
                let resolved_ns = reader
                    .keys()
                    .find(|ns| *ns == &namespace)
                    .unwrap_or_else(|| {
                        panic!("namespace not found: `{namespace}` (in filter entry `{entry}`)")
                    });
                let resolved_name = reader[resolved_ns]
                    .keys()
                    .find(|n| *n == &type_name)
                    .unwrap_or_else(|| {
                        panic!("type not found: `{type_part}` (in filter entry `{entry}`)")
                    });

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
                        let ty = reader.unwrap_full_name(resolved_ns, resolved_name);
                        match &ty {
                            Type::Class(c) => {
                                if !filter.types.contains(&(resolved_ns, resolved_name)) {
                                    filter.types.push((resolved_ns, resolved_name));
                                }
                                filter.activatable.insert((resolved_ns, resolved_name));
                                for iface in c.required_interfaces(reader) {
                                    let iface_ns = iface.def.namespace();
                                    let iface_name = iface.def.name();
                                    let Some(r_ns) = reader.keys().find(|ns| *ns == &iface_ns)
                                    else {
                                        continue;
                                    };
                                    let Some(r_name) =
                                        reader[r_ns].keys().find(|n| *n == &iface_name)
                                    else {
                                        continue;
                                    };
                                    // Don't override an explicit method-level filter entry.
                                    filter
                                        .interfaces
                                        .entry((r_ns, r_name))
                                        .or_insert(MethodSet::All);
                                }
                            }
                            Type::Enum(_) | Type::CppEnum(_) => {
                                filter
                                    .enum_variants
                                    .insert((resolved_ns, resolved_name), MethodSet::All);
                                if !filter.types.contains(&(resolved_ns, resolved_name)) {
                                    filter.types.push((resolved_ns, resolved_name));
                                }
                            }
                            _ => {
                                filter
                                    .interfaces
                                    .insert((resolved_ns, resolved_name), MethodSet::All);
                            }
                        }
                    } else {
                        let ty = reader.unwrap_full_name(resolved_ns, resolved_name);
                        filter.insert_method_for_type(
                            entry,
                            &ty,
                            type_part,
                            method_name,
                            resolved_ns,
                            resolved_name,
                            reader,
                        );
                    }
                }
            } else {
                // Type entry: Namespace.Type
                let (namespace, type_name) = entry.rsplit_once('.').unwrap_or_else(|| {
                    panic!(
                        "invalid minimal filter entry `{entry}`: \
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

                filter.types.push((resolved_ns, resolved_name));
            }
        }

        assert!(
            !(filter.interfaces.is_empty() && filter.types.is_empty()),
            "at least one `--filter` entry required for minimal mode"
        );

        filter
    }

    /// Returns true if the given method on the given type should be emitted as
    /// a live vtable slot (not demoted to `Slot: usize`).
    pub fn includes_method(&self, type_name: TypeName, method_name: &str) -> bool {
        let key = (type_name.namespace(), type_name.name());
        match self.interfaces.get(&key) {
            Some(set) => {
                if set.includes(method_name) {
                    return true;
                }
                // If `remove_X` is requested via the corresponding `add_X`,
                // keep it as a live slot — the auto-revoking event pattern
                // references the remove function pointer in the vtable.
                if let Some(event) = method_name.strip_prefix("remove_") {
                    let add_name = format!("add_{event}");
                    return set.includes(&add_name);
                }
                false
            }
            None => false,
        }
    }

    /// Returns the variant filter for a given enum, if one was specified.
    /// Returns `None` if the enum was included as a plain type (all variants).
    pub fn enum_variant_filter(&self, namespace: &str, name: &str) -> Option<&MethodSet> {
        // The keys are &'static str but we need to look up by &str.
        self.enum_variants
            .iter()
            .find(|((ns, n), _)| *ns == namespace && *n == name)
            .map(|(_, v)| v)
    }

    fn insert_method(&mut self, key: (&'static str, &'static str), name: String) {
        let set = self
            .interfaces
            .entry(key)
            .or_insert_with(|| MethodSet::Names(BTreeSet::new()));

        match set {
            MethodSet::All => {
                // An explicit method-level entry (IFoo::{a,b}) overrides a
                // previous All that came from a class ::* expansion.
                let mut names = BTreeSet::new();
                names.insert(name);
                *set = MethodSet::Names(names);
            }
            MethodSet::Names(names) => {
                names.insert(name);
            }
        }
    }

    fn insert_enum_variant(&mut self, key: (&'static str, &'static str), name: String) {
        let set = self
            .enum_variants
            .entry(key)
            .or_insert_with(|| MethodSet::Names(BTreeSet::new()));

        match set {
            MethodSet::All => {}
            MethodSet::Names(names) => {
                names.insert(name);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_method_for_type(
        &mut self,
        entry: &str,
        ty: &Type,
        type_part: &str,
        method_name: &str,
        resolved_ns: &'static str,
        resolved_name: &'static str,
        reader: &Reader,
    ) {
        match ty {
            Type::Interface(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` \
                     (in filter entry `{entry}`)"
                );
                self.insert_method((resolved_ns, resolved_name), method_name.to_string());
            }
            Type::CppInterface(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` \
                     (in filter entry `{entry}`)"
                );
                self.insert_method((resolved_ns, resolved_name), method_name.to_string());
            }
            Type::Delegate(t) => {
                assert!(
                    t.def.methods().any(|m| method_matches(m, method_name)),
                    "method `{method_name}` not found on `{type_part}` \
                     (in filter entry `{entry}`)"
                );
                self.insert_method((resolved_ns, resolved_name), method_name.to_string());
            }
            Type::Class(c) => {
                let required = c.required_interfaces(reader);
                let mut found = false;
                // Special case: `CreateInstance` means "include for instantiation".
                // Works for both composable classes (with a factory) and
                // default-activatable classes (via IActivationFactory).
                if method_name == "CreateInstance" {
                    if !self.types.contains(&(resolved_ns, resolved_name)) {
                        self.types.push((resolved_ns, resolved_name));
                    }
                    self.activatable.insert((resolved_ns, resolved_name));
                    found = true;
                }
                if !found {
                    for iface in &required {
                        if iface.def.methods().any(|m| method_matches(m, method_name)) {
                            let iface_ns = iface.def.namespace();
                            let iface_name = iface.def.name();
                            let r_ns = reader.keys().find(|ns| *ns == &iface_ns).unwrap();
                            let r_name = reader[r_ns].keys().find(|n| *n == &iface_name).unwrap();
                            self.insert_method((r_ns, r_name), method_name.to_string());
                            if !self.types.contains(&(resolved_ns, resolved_name)) {
                                self.types.push((resolved_ns, resolved_name));
                            }
                            found = true;
                            break;
                        }
                    }
                }
                // Also search composable factory interfaces for the method.
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
                    "method `{method_name}` not found on \
                     class `{type_part}` (in filter entry `{entry}`)"
                );
                // Include composable factory interfaces so new() works.
                // The compose() variant is controlled separately by emit_compose
                // in class.rs based on the --implement option.
                for iface in &required {
                    if matches!(iface.kind, InterfaceKind::Composable) {
                        let iface_ns = iface.def.namespace();
                        let iface_name = iface.def.name();
                        let Some(r_ns) = reader.keys().find(|ns| *ns == &iface_ns) else {
                            continue;
                        };
                        let Some(r_name) = reader[r_ns].keys().find(|n| *n == &iface_name) else {
                            continue;
                        };
                        self.interfaces
                            .entry((r_ns, r_name))
                            .or_insert(MethodSet::All);
                    }
                }
            }
            Type::Enum(e) => {
                assert!(
                    e.def
                        .fields()
                        .any(|f| f.flags().contains(FieldAttributes::Literal)
                            && f.name() == method_name),
                    "variant `{method_name}` not found on enum `{type_part}` \
                     (in filter entry `{entry}`)"
                );
                self.insert_enum_variant((resolved_ns, resolved_name), method_name.to_string());
                if !self.types.contains(&(resolved_ns, resolved_name)) {
                    self.types.push((resolved_ns, resolved_name));
                }
            }
            Type::CppEnum(e) => {
                assert!(
                    e.def
                        .fields()
                        .any(|f| f.flags().contains(FieldAttributes::Literal)
                            && f.name() == method_name),
                    "variant `{method_name}` not found on enum `{type_part}` \
                     (in filter entry `{entry}`)"
                );
                self.insert_enum_variant((resolved_ns, resolved_name), method_name.to_string());
                if !self.types.contains(&(resolved_ns, resolved_name)) {
                    self.types.push((resolved_ns, resolved_name));
                }
            }
            _ => panic!(
                "type `{type_part}` is not an interface, delegate, \
                 enum, or class (in filter entry `{entry}`)"
            ),
        }
    }
}
