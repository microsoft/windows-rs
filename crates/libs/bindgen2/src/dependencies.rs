use super::*;

type Map = HashMap<&'static str, HashSet<&'static str>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependencies {
    map: Map,
}

impl Dependencies {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
        self.map.entry(namespace).or_default().insert(name)
    }

    pub fn combine(&mut self, other: Self) {
        for (namespace, name) in other.iter() {
            self.insert(namespace, name);
        }
    }

    // pub fn included(&self, filter: &Filter) -> bool {
    //     for (namespace, name) in self.iter() {
    //         if namespace.is_empty() {
    //             continue;
    //         }

    //         if !filter.includes_type_name(namespace, name) {
    //             return false;
    //         }
    //     }

    //     true
    // }

    pub fn namespaces(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.map.keys().copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + '_ {
        self.map
            .iter()
            .flat_map(|(namespace, names)| names.iter().map(move |name| (*namespace, *name)))
    }

    pub fn included(&self, filter: &Filter) -> bool {
        self.iter().all(|(namespace, name)| filter.includes_type_name(namespace, name))
    }

    pub fn excluded(&self, filter: &Filter) -> bool {
         self
        .iter()
        .any(|(namespace, name)| filter.excludes_type_name(namespace, name))
    }
}

pub trait Dependent {
    fn dependencies(&self, dependencies: &mut Dependencies);
}
