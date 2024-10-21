use super::*;

type Map = HashMap<&'static str, HashSet<&'static str>>;

#[derive(Debug)]
pub struct Dependencies(Map);

impl Dependencies {
    pub fn new() -> Self {
        Self(Map::new())
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
        self.entry(namespace).or_default().insert(name)
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

    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &'static str)> + '_ {
        self.0
            .iter()
            .flat_map(|(namespace, names)| names.iter().map(move |name| (*namespace, *name)))
    }
}

impl std::ops::Deref for Dependencies {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Dependencies {
    fn deref_mut(&mut self) -> &mut Map {
        &mut self.0
    }
}
