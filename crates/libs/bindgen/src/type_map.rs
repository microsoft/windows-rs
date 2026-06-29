use super::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct TypeMap(HashMap<TypeName, HashSet<Type>>);

pub trait Dependencies {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader);

    fn dependencies(&self, reader: &Reader) -> TypeMap {
        let mut dependencies = TypeMap::new();
        self.combine(&mut dependencies, reader);
        dependencies
    }
}

impl std::ops::Deref for TypeMap {
    type Target = HashMap<TypeName, HashSet<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TypeMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    #[track_caller]
    pub fn filter(reader: &Reader, filter: &Filter, references: &References) -> Self {
        let mut dependencies = Self::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                for (name, types) in &reader[namespace] {
                    if let Some(filter_rule) = filter.includes_type_name(TypeName(namespace, name))
                    {
                        // A longer rule string means a more specific (fully-qualified) path.
                        // Skip types already owned by a reference whose rule is more specific
                        // (longer) than the filter rule that matched this type.
                        if references
                            .matching_rule(TypeName(namespace, name))
                            .is_some_and(|reference_rule| reference_rule.len() > filter_rule.len())
                        {
                            continue;
                        }

                        let mut item_dependencies = Self::new();

                        for ty in types {
                            ty.combine(&mut item_dependencies, reader);
                        }

                        if item_dependencies.excluded(filter, references) {
                            continue;
                        }

                        for ty in types {
                            dependencies.insert(ty.clone());
                        }

                        dependencies.combine_references(&item_dependencies, references);
                    }
                }
            }
        }

        dependencies
    }

    pub fn insert(&mut self, ty: Type) -> bool {
        self.0.entry(ty.type_name()).or_default().insert(ty)
    }

    pub fn combine_references(&mut self, other: &Self, references: &References) {
        for (tn, types) in &other.0 {
            if references.contains(*tn).is_none() {
                let set = self.0.entry(*tn).or_default();
                types.iter().for_each(|ty| {
                    set.insert(ty.clone());
                });
            }
        }
    }

    pub fn combine(&mut self, other: &Self) {
        for (name, types) in &other.0 {
            let set = self.0.entry(*name).or_default();
            types.iter().for_each(|ty| {
                set.insert(ty.clone());
            });
        }
    }

    pub fn included(&self, config: &Config) -> bool {
        // Types reachable only through a reference-provided interface (e.g.
        // `IReference`'s base `IPropertyValue`) are owned by that reference crate.
        // They are never named in the generated code, so they don't need to be
        // independently available here.
        let covered = self.reference_provided_closure(config);

        self.0.iter().all(|(tn, _)| {
            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            if tn.namespace().is_empty() {
                return true;
            }

            if config.types.contains_key(tn) {
                return true;
            }

            if config.references.contains(*tn).is_some() {
                return true;
            }

            // Generic types have a tick suffix (e.g. `IReference`1`) that may not
            // match reference filter rules. Try without the suffix.
            if let Some(pos) = tn.name().find('`') {
                let base_name: &str = &tn.name()[..pos];
                let base_tn = TypeName(tn.namespace(), base_name);
                if config.references.contains(base_tn).is_some() {
                    return true;
                }
            }

            if covered.contains(tn) {
                return true;
            }

            false
        })
    }

    /// Collects every type reachable through a reference-provided interface in this
    /// dependency set. Such types (e.g. the `IPropertyValue` base of `IReference`) are
    /// encapsulated by the reference crate, so a method that only reaches them via a
    /// reference-provided type can still be fully described.
    fn reference_provided_closure(&self, config: &Config) -> HashSet<TypeName> {
        let mut covered = HashSet::new();

        for (tn, types) in &self.0 {
            let is_reference = config.references.contains(*tn).is_some()
                || tn.name().find('`').is_some_and(|pos| {
                    config
                        .references
                        .contains(TypeName(tn.namespace(), &tn.name()[..pos]))
                        .is_some()
                });

            if is_reference {
                for ty in types {
                    for dep_tn in ty.dependencies(config.reader).0.keys() {
                        covered.insert(*dep_tn);
                    }
                }
            }
        }

        covered
    }

    fn excluded(&self, filter: &Filter, references: &References) -> bool {
        self.0
            .iter()
            .any(|(tn, _)| filter.excludes_type_name(*tn) && references.contains(*tn).is_none())
    }
}
