use super::*;

// TODO: get rid of this in favor of Includes

// TODO: should store Type rather than TypeName
type Set = HashSet<Type>;

// TODO: do we even need a wrapper type at this point?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependencies(Set);

// impl std::ops::Deref for Dependencies {
//     type Target = HashSet<Type>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl core::ops::DerefMut for Dependencies {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl Dependencies {
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
                        types[0].dependencies(&mut item_dependencies);

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
        self.0.insert(ty)
    }

    pub fn combine(&mut self, other: &Self) {
        self.0.extend(other.0.iter().cloned());
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0.difference(&other.0).cloned().collect())
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.0.iter().map(|ty| ty.namespace())
    }

    // TODO: do we need a consuming version of this?
    pub fn types(&self) -> impl Iterator<Item = Type> + '_ {
        self.0.iter().cloned()
    }

    pub fn included(&self, config: &Config) -> bool {
        self.0.iter().all(|ty| {
            let tn = ty.type_name();

            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            if tn.namespace().is_empty() {
                return true;
            }

            // TODO: would it be faster/simpler to search by `ty` instead
            if config.includes.contains(tn) {
                return true;
            }

            if config.references.contains(tn).is_some() {
                return true;
            }

            // TODO: maybe have Reference type that includes map for crate association and a Filter for quick type inclusion detection here
            //if config.reference.values().find(|namespace|namespace)

            false
        })
    }

    fn excluded(&self, filter: &Filter) -> bool {
        self.0
            .iter()
            .any(|name| filter.excludes_type_name(name.type_name()))
    }

    pub fn contains(&self, name: TypeName<'_>) -> bool {
        self.0.iter().any(|ty| ty.type_name() == name)
    }
}

#[test]
fn test_difference() {
    let mut interface = Dependencies::new();
    interface.insert("Windows.Foundation", "IReference");

    let mut method = Dependencies::new();
    method.insert("Windows.Foundation", "IReference");
    method.insert("Windows.Foundation.Collections", "IVector");

    let mut diff = Dependencies::new();
    diff.insert("Windows.Foundation.Collections", "IVector");

    assert_eq!(diff, method.difference(&interface));
}
