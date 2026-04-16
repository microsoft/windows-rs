use super::*;

#[derive(Default)]
pub struct Collector(BTreeMap<String, Item>);

impl std::ops::Deref for Collector {
    type Target = BTreeMap<String, Item>;

    fn deref(&self) -> &BTreeMap<String, Item> {
        &self.0
    }
}

impl Collector {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, item: Item) {
        self.0.insert(item.to_string(), item);
    }
}
