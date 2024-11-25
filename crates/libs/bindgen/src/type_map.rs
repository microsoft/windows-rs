use super::*;

// TODO: call this TypeMap and reuse in Reader?

type Set = HashMap<TypeName, HashSet<Type>>;

// TODO: do we even need a wrapper type at this point?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeMap(Set);

impl std::ops::Deref for TypeMap {
    type Target = Set;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl core::ops::DerefMut for TypeMap {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl TypeMap {
    pub fn new() -> Self {
        Self(Set::new())
    }

    pub fn filter(reader: &'static Reader, filter: &Filter) -> Self {
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

                        dependencies.combine(&item_dependencies);
                    }
                }
            }
        }

        dependencies
    }

    pub fn insert(&mut self, ty: Type) -> bool {
        self.0.entry(ty.type_name()).or_default().insert(ty)
    }

    pub fn combine(&mut self, other: &Self) {
        for (name, types) in &other.0 {
            let set = self.0.entry(*name).or_default();
            types.iter().for_each(|ty| {
                set.insert(ty.clone());
            });
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|(tn, _)| !other.0.contains_key(tn))
                .map(|(tn, ty)| (*tn, ty.clone()))
                .collect(),
        )
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.0.keys().map(|tn| tn.0)
    }

    pub fn included(&self, config: &Config) -> bool {
        self.0.iter().all(|(tn, _)| {
            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            if tn.namespace().is_empty() {
                return true;
            }

            // TODO: would it be faster/simpler to search by `ty` instead
            if config.includes.contains(*tn) {
                return true;
            }

            if config.references.contains(*tn).is_some() {
                return true;
            }

            // TODO: maybe have Reference type that includes map for crate association and a Filter for quick type inclusion detection here
            //if config.reference.values().find(|namespace|namespace)

            false
        })
    }

    fn excluded(&self, filter: &Filter) -> bool {
        self.0.iter().any(|(tn, _)| filter.excludes_type_name(*tn))
    }

    pub fn contains(&self, name: TypeName) -> bool {
        self.0.contains_key(&name)
    }
}
