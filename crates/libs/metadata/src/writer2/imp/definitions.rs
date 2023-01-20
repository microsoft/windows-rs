use super::*;

#[derive(Default)]
pub struct Definitions<'a> {
    map: BTreeMap<(&'a str, &'a str), Definition<'a>>,
}

struct Definition<'a> {
    pub item: &'a Item,
    pub index: u32,
}

pub struct StagedDefinitions<'a>(Definitions<'a>);

impl<'a> Definitions<'a> {
    pub fn insert(&mut self, item: &'a Item) {
        if self.map.insert(item_type_name(item), Definition { item, index: 0 }).is_some() {
            panic!("Duplicate type found");
        }
    }

    pub fn stage(mut self) -> StagedDefinitions<'a> {
        for (index, value) in self.map.values_mut().enumerate() {
            value.index = index as _;
        }
        StagedDefinitions(self)
    }
}

impl<'a> StagedDefinitions<'a> {
    pub fn get(&self, namespace: &str, name: &str) -> Option<&'a Item> {
        self.0.map.get(&(namespace, name)).map(|value| value.item)
    }

    pub fn index(&self, namespace: &str, name: &str) -> u32 {
        self.0.map[&(namespace, name)].index
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.0.map.values().map(|value| value.item)
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &Item)> {
        self.0.map.values().map(|value| (value.index, value.item))
    }
}
