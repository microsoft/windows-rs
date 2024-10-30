use super::*;

type Set = HashSet<(&'static str, &'static str)>;

// TODO: do we even need a wrapper type at this point?
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependencies {
    set: Set,
}

impl Dependencies {
    pub fn new() -> Self {
        Self { set: Set::new() }
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
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

    pub fn included(&self, filter: &NameTree) -> bool {
        self.set.iter().all(|(namespace, name)| {
            // An empty namespace covers core types like `HRESULT`. This way we don't exclude methods
            // that depend on core types that aren't explicitly included in the filter.
            namespace.is_empty() || filter.includes_type_name(namespace, name)
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
