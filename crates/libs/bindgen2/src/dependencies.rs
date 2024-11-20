use super::*;

// TODO: get rid of this in favor of Includes

// TODO: store `HashSet<TypeName<'static>>` instead
type Set = HashSet<(&'static str, &'static str)>;

// TODO: do we even need a wrapper type at this point?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependencies {
    set: Set,
}

impl std::ops::Deref for Dependencies {
    type Target = HashSet<(&'static str, &'static str)>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}

impl Dependencies {
    pub fn new() -> Self {
        Self { set: Set::new() }
    }

    pub fn filter(reader: &'static Reader, filter: &Filter) -> Self {
        let mut set = HashSet::new();
        let mut dependencies = Dependencies::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                for name in reader[namespace].keys() {
                    if filter.includes_type_name(namespace, name) {
                        let mut item_dependencies = Dependencies::new();

                        for item in &reader[namespace][name] {
                            item.dependencies(&mut item_dependencies);
                        }

                        if item_dependencies.excluded(filter) {
                            continue;
                        }

                        set.insert((*namespace, *name));
                        dependencies.combine(&item_dependencies);
                    }
                }
            }
        }

        for (namespace, name) in dependencies.iter() {
            set.insert((namespace, name));
        }

        Self {set}
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
        // TODO: maybe move dependency recursion direclty into here?
        // Have the Dependencies store a Reader to look up deps directly
        // This would work more like the old standalone one?
        self.set.insert((namespace, name))
    }

    pub fn combine(&mut self, other: &Self) {
        for (namespace, name) in other.set.iter() {
            self.insert(namespace, name);
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            set: self.set.difference(&other.set).copied().collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + '_ {
        self.set.iter().copied()
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.set.iter().map(|(namespace, _)| namespace).copied()
    }

    pub fn included(&self, config: &Config) -> bool {
        self.set.iter().all(|(namespace, name)| {
            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            if namespace.is_empty() {
                return true;
            }

            if config.includes.contains(&(namespace, name)) {
                return true;
            }

            if config
                .references
                .includes_type_name(namespace, name)
                .is_some()
            {
                return true;
            }

            // TODO: maybe have Reference type that includes map for crate association and a Filter for quick type inclusion detection here
            //if config.reference.values().find(|namespace|namespace)

            false
        })
    }

    pub fn excluded(&self, filter: &Filter) -> bool {
        self.set
            .iter()
            .any(|(namespace, name)| filter.excludes_type_name(namespace, name))
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
