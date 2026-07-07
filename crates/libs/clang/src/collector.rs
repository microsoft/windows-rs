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

    /// Keeps only the entries whose name satisfies `keep`, dropping the rest. Used by the
    /// reachability sweep to remove unreferenced out-of-scope declarations.
    pub fn retain(&mut self, mut keep: impl FnMut(&str) -> bool) {
        self.0.retain(|name, _| keep(name));
    }

    /// Like [`retain`](Self::retain) but the predicate also sees the [`Item`], so the caller
    /// can decide by entity category (type vs value). Used by the additive exclusion to keep
    /// type/value duplicates matched to their own category.
    pub fn retain_items(&mut self, mut keep: impl FnMut(&str, &Item) -> bool) {
        self.0.retain(|name, item| keep(name, item));
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

    /// Fill in missing interface GUIDs from `IID_<Name>` variable declarations.
    ///
    /// For each interface with `guid: None`, looks up the interface name in
    /// `iid_vars` and applies the UUID if found.
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
