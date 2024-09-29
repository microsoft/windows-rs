use super::*;

// TODO: for primitive dependencies (e.g. HSTRING, GUID, IUnknown) use the root namespace of ""
// so that standalone code generation can generate them as needed.

type Map = HashMap<&'static str, HashSet<&'static str>>;

pub struct Dependencies(Map);

impl Dependencies {
    pub fn new() -> Self {
        Self(Map::new())
    }

    pub fn insert(&mut self, namespace: &'static str, name: &'static str) -> bool {
        self.entry(namespace).or_default().insert(name)
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
