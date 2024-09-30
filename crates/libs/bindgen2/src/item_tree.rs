use super::*;

// This uses BTree rather than Hash as we're getting close to writing the tokens in sorted order.
#[derive(Debug)]
pub struct ItemTree {
    pub nested: BTreeMap<&'static str, Self>,
    pub items: BTreeSet<&'static Item>,
}

impl ItemTree {
    pub fn new(reader: &'static Reader, tree: &NameTree) -> Self {
        let mut new = Self {
            nested: BTreeMap::new(),
            items: BTreeSet::new(),
        };

        for name in &tree.items {
            for item in reader.with_full_name(tree.namespace, name) {
                new.items.insert(item);
            }
        }

        for (name, tree) in &tree.nested {
            new.nested.insert(name, Self::new(reader, tree));
        }

        new
    }
}
