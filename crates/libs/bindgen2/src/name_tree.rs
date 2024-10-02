use super::*;

#[derive(Debug)]
pub struct NameTree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Self>,
    pub items: HashSet<&'static str>,
}

// TODO: can't just use filter onces name tree is built since the filter won't include dependencies

impl NameTree {
    pub fn new(
        reader: &'static Reader,
        filter: &Filter,
        minimal: bool,
    ) -> &'static Self {
        let mut tree = Self::with_namespace("");
        let mut dependencies = Dependencies::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                let tree = tree.insert_namespace(namespace);

                for name in reader[namespace].keys() {
                    if filter.includes_type_name(namespace, name) {
                        tree.items.insert(name);

                        for item in &reader[namespace][name] {
                            item.dependencies(&mut dependencies, minimal);
                        }
                    }
                }
            }
        }

            for (namespace, names) in dependencies.iter() {
                for name in names {
                    tree.insert_namespace(namespace).items.insert(name);
                }
            }

        Box::leak(Box::new(tree))
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        if let Some(next) = namespace.find('.') {
            self.nested.get(&namespace[..next]).map_or(false, |tree|tree.includes_namespace(&namespace[next + 1..]))
        }
         else {
            self.nested.contains_key(namespace)
         }
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: HashMap::new(),
            items: HashSet::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str) -> &mut Self {
        fn insert_namespace<'a>(
            parent: &'a mut NameTree,
            namespace: &'static str,
            pos: usize,
        ) -> &'a mut NameTree {
            if let Some(next) = namespace[pos..].find('.') {
                let next = pos + next;

                let parent = parent
                    .nested
                    .entry(&namespace[pos..next])
                    .or_insert_with(|| NameTree::with_namespace(&namespace[..next]));

                insert_namespace(parent, namespace, next + 1)
            } else {
                parent
                    .nested
                    .entry(&namespace[pos..])
                    .or_insert_with(|| NameTree::with_namespace(namespace))
            }
        }

        if namespace.is_empty() {
            self
        } else {
            insert_namespace(self, namespace, 0)
        }
    }
}
