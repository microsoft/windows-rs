use super::*;

#[derive(Default)]
pub struct References<'a> {
    map: BTreeMap<(&'a str, &'a str), u32>
}

impl<'a> References<'a> {
    pub fn insert(&mut self, namespace: &'a str, name: &'a str) {
        self.map.entry((namespace, name)).or_default();
    }

    pub fn stage(&mut self) {
        for (index, value) in self.map.values_mut().enumerate() {
            *value = index as _;
        }
    }

    pub fn get(&self, namespace: &str, name: &str) -> u32 {
        self.map[&(namespace, name)]
    }
}
