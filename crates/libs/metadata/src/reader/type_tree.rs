use super::*;
pub use std::collections::BTreeMap;

// The TypeTree needs to use a BTreeMap rather than the fast HashMap because it affects code gen and we need
// the code gen to be stable.
pub struct TypeTree {
    pub namespace: &'static str,
    pub types: BTreeMap<&'static str, Vec<Type>>,
    pub namespaces: BTreeMap<&'static str, TypeTree>,
}

impl TypeTree {
    pub fn from_namespace(namespace: &'static str) -> Self {
        Self { namespace, types: BTreeMap::new(), namespaces: BTreeMap::new() }
    }

    pub fn insert_namespace(&mut self, namespace: &'static str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces.entry(&namespace[pos..next]).or_insert_with(|| Self::from_namespace(&namespace[..next])).insert_namespace(namespace, next + 1)
        } else {
            self.namespaces.entry(&namespace[pos..]).or_insert_with(|| Self::from_namespace(namespace))
        }
    }

    pub fn insert_type(&mut self, name: &'static str, def: Type) {
        self.types.entry(name).or_default().push(def);
    }

    pub fn get_type(&self, name: &str) -> Option<&Vec<Type>> {
        self.types.get(name)
    }

    pub fn get_namespace(&self, namespace: &str) -> Option<&Self> {
        if let Some(next) = namespace.find('.') {
            self.namespaces.get(&namespace[..next]).and_then(|child| child.get_namespace(&namespace[next + 1..]))
        } else {
            self.namespaces.get(namespace)
        }
    }
}
