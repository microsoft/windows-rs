use super::*;

// This uses BTree rather than Hash as we're getting close to writing the tokens in sorted order.
#[derive(Debug)]
pub struct ItemTree {
    pub namespace: &'static str,
    pub nested: BTreeMap<&'static str, Self>,
    pub items: BTreeSet<Item>,
}

impl ItemTree {
    pub fn new(reader: &'static Reader, config: &Config) -> Self {
        fn expand(reader: &'static Reader, tree: &NameTree, config: &Config) -> ItemTree {
            let mut new = ItemTree {
                namespace: tree.namespace,
                nested: BTreeMap::new(),
                items: BTreeSet::new(),
            };

            for name in &tree.items {
                for mut item in reader.with_full_name(tree.namespace, name) {
                    item.expand(config);
                    new.items.insert(item);
                }
            }

            for (name, tree) in &tree.nested {
                new.nested.insert(name, expand(reader, tree, config));
            }

            // TODO: load methods here or above

            new
        }

        expand(reader, &config.tree, config)
    }

    // This is used to provide a flat iterator of trees so that they can be processed in parallel.
    pub fn flatten_trees(&self) -> Vec<&Self> {
        let mut flatten = if self.namespace.is_empty() {
            vec![]
        } else {
            vec![self]
        };
        flatten.extend(self.nested.values().flat_map(|tree| tree.flatten_trees()));
        flatten
    }

    // This is used to actually remove the tree structure and just have a flat iterator or items
    // that just happen to be sorted, thanks to BTreeSet, so that a flat list of items can be
    // generated in sort order.
    pub fn flatten_items(mut self) -> BTreeSet<Item> {
        fn flatten(set: &mut BTreeSet<Item>, tree: &mut ItemTree) {
            set.append(&mut tree.items);

            for tree in tree.nested.values_mut() {
                flatten(set, tree);
            }
        }

        let mut items = BTreeSet::new();
        flatten(&mut items, &mut self);
        items
    }

    pub fn feature(&self) -> String {
        self.namespace.split_once('.').unwrap().1.replace('.', "_")
    }
}
