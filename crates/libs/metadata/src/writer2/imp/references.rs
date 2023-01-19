use super::*;

#[derive(Default)]
pub struct References<'a> {
    map: BTreeMap<(&'a str, &'a str), u32>,
}

pub struct StagedReferences<'a>(References<'a>);

impl<'a> References<'a> {
    pub fn insert(&mut self, namespace: &'a str, name: &'a str) {
        self.map.entry((namespace, name)).or_default();
    }

    pub fn stage(mut self) -> StagedReferences<'a> {
        for (index, value) in self.map.values_mut().enumerate() {
            *value = index as _;
        }
        StagedReferences(self)
    }
}

impl<'a> StagedReferences<'a> {
    pub fn index(&self, namespace: &str, name: &str) -> u32 {
        self.0.map[&(namespace, name)]
    }
}
