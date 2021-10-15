use super::*;
pub use std::collections::BTreeMap;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeInclude {
    Full,
    Minimal,
    None,
}

impl Default for TypeInclude {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct TypeEntry {
    pub def: Vec<ElementType>,
    pub include: TypeInclude,
}

// The TypeTree needs to use a BTreeMap rather than the fast HashMap because it affects code gen and we need
// the code gen to be stable.
pub struct TypeTree {
    pub namespace: &'static str,
    pub types: BTreeMap<&'static str, TypeEntry>,
    pub namespaces: BTreeMap<&'static str, TypeTree>,
    pub include: bool,
}

impl TypeTree {
    pub fn from_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            types: BTreeMap::new(),
            namespaces: BTreeMap::new(),
            include: false,
        }
    }

    pub fn module_features(&self, features: &mut BTreeSet<&'static str>, keys: &mut std::collections::HashSet<Row>) {
        self.types.values().map(|entry|entry.def.iter()).flatten().for_each(|def|def.module_features(features, keys));
    }

    pub fn insert_namespace(&mut self, namespace: &'static str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::from_namespace(&namespace[..next]))
                .insert_namespace(namespace, next + 1)
        } else {
            self.namespaces
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::from_namespace(namespace))
        }
    }

    pub fn insert_type(&mut self, name: &'static str, def: ElementType) {
        self.types.entry(name).or_default().def.push(def);
    }

    // TODO: slow method - remove or make this an iterator somehow?
    pub fn namespaces(&self) -> Vec<&'static str> {
        let mut namespaces = Vec::new();

        for tree in self.namespaces.values() {
            if !tree.types.is_empty() {
                namespaces.push(tree.namespace)
            }

            namespaces.append(&mut tree.namespaces());
        }

        namespaces
    }

    pub fn get_type(&self, name: &str) -> Option<&TypeEntry> {
        self.types.get(name)
    }

    pub fn get_type_mut(&mut self, name: &str) -> Option<&mut TypeEntry> {
        self.types.get_mut(name)
    }

    pub fn get_namespace(&self, namespace: &str) -> Option<&Self> {
        self.get_namespace_pos(namespace, 0)
    }

    fn get_namespace_pos(&self, namespace: &str, pos: usize) -> Option<&Self> {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .get(&namespace[pos..next])
                .and_then(|child| child.get_namespace_pos(namespace, next + 1))
        } else {
            self.namespaces.get(&namespace[pos..])
        }
    }

    pub fn get_namespace_mut(&mut self, namespace: &str) -> Option<&mut Self> {
        self.get_namespace_mut_pos(namespace, 0)
    }

    fn get_namespace_mut_pos(&mut self, namespace: &str, pos: usize) -> Option<&mut Self> {
        self.include = true;
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .get_mut(&namespace[pos..next])
                .and_then(|child| child.get_namespace_mut_pos(namespace, next + 1))
        } else {
            self.namespaces.get_mut(&namespace[pos..]).map(|ns| {
                ns.include = true;
                ns
            })
        }
    }
}
