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

    /// Keep only entries whose name satisfies `keep`.
    pub fn retain(&mut self, mut keep: impl FnMut(&str) -> bool) {
        self.0.retain(|name, _| keep(name));
    }

    /// Keep entries by name and item category.
    pub fn retain_items(&mut self, mut keep: impl FnMut(&str, &Item) -> bool) {
        self.0.retain(|name, item| keep(name, item));
    }

    /// Mark an enum named by `DEFINE_ENUM_FLAG_OPERATORS` as flags.
    pub fn mark_flags(&mut self, name: &str) {
        if let Some(Item::Enum(e)) = self.0.get_mut(name) {
            e.flags = true;
        }
    }

    /// Fill missing interface GUIDs from `IID_<Name>` variable declarations.
    pub fn apply_iid_vars(&mut self, iid_vars: &HashMap<String, String>) {
        for (name, item) in &mut self.0 {
            if let Item::Interface(iface) = item
                && iface.guid.is_none()
                && let Some(uuid) = iid_vars.get(name)
            {
                iface.guid = Some(uuid.clone());
            }
        }
    }
}
