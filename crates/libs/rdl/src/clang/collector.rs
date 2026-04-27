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

    /// Mark the named enum as a flags enum (sets `Enum::flags = true`).
    ///
    /// Called after a `DEFINE_ENUM_FLAG_OPERATORS(Name)` macro expansion is
    /// detected so that the enum's RDL output will include `#[flags]`.
    pub fn mark_flags(&mut self, name: &str) {
        if let Some(Item::Enum(e)) = self.0.get_mut(name) {
            e.flags = true;
        }
    }
}
