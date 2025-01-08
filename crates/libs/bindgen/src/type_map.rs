use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeMap {
    types: HashMap<TypeName, HashSet<Type>>,
    deprecated: bool,
    // TODO: add arches
}

impl std::ops::Deref for TypeMap {
    type Target = HashMap<TypeName, HashSet<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.types
    }
}

impl TypeMap {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            deprecated: false,
        }
    }

    #[track_caller]
    pub fn filter(reader: &'static Reader, filter: &Filter, references: &References) -> Self {
        let mut dependencies = Self::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                for (name, types) in &reader[namespace] {
                    if filter.includes_type_name(TypeName(namespace, name)) {
                        let mut item_dependencies = Self::new();

                        for ty in types {
                            ty.dependencies(&mut item_dependencies);
                        }

                        if item_dependencies.excluded(filter) {
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
        self.types.entry(ty.type_name()).or_default().insert(ty)
    }

    fn combine_references(&mut self, other: &Self, references: &References) {
        for (tn, types) in &other.types {
            if references.contains(*tn).is_none() {
                let set = self.types.entry(*tn).or_default();
                types.iter().for_each(|ty| {
                    set.insert(ty.clone());
                });
            }
        }
    }

    pub fn combine(&mut self, other: &Self) {
        for (name, types) in &other.types {
            let set = self.types.entry(*name).or_default();
            types.iter().for_each(|ty| {
                set.insert(ty.clone());
            });
        }
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecated
    }

    pub fn deprecated<R: HasAttributes>(&mut self, row: R) {
        if row.has_attribute("DeprecatedAttribute") {
            self.deprecated = true;
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            types: self
                .types
                .iter()
                .filter(|(tn, _)| !other.types.contains_key(tn))
                .map(|(tn, ty)| (*tn, ty.clone()))
                .collect(),
            deprecated: self.deprecated && !other.deprecated,
        }
    }

    pub fn included(&self, config: &Config) -> bool {
        self.types.iter().all(|(tn, _)| {
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

            false
        })
    }

    fn excluded(&self, filter: &Filter) -> bool {
        self.types
            .iter()
            .any(|(tn, _)| filter.excludes_type_name(*tn))
    }
}
