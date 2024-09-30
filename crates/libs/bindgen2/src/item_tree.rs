use super::*;

#[derive(Debug)]
pub struct ItemTree {
    // TODO: need this namespace?
    pub namespace: &'static str,
    pub nested: BTreeMap<&'static str, Self>,
    pub items: BTreeSet<&'static Item>,
}

impl ItemTree {
    pub fn new(reader: &'static Reader, tree: &Tree) -> Self {
        let mut new = Self::with_namespace(tree.namespace);

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

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: BTreeMap::new(),
            items: BTreeSet::new(),
        }
    }
}
