use super::*;

#[derive(Debug)]
pub struct Tree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Tree>,
    pub items: HashSet<&'static str>,
}

impl Tree {
    pub fn new(reader: &winmd::Reader, filter: &Filter) -> Self {
        let mut tree = Tree::with_namespace("");

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                tree.insert_namespace(reader, filter, namespace, 0);
            }
        }

        tree
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: HashMap::new(),
            items: HashSet::new(),
        }
    }

    fn insert_namespace(
        &mut self,
        reader: &winmd::Reader,
        filter: &Filter,
        namespace: &'static str,
        pos: usize,
    ) -> &mut Self {
        let tree = if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.nested
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::with_namespace(&namespace[..next]))
                .insert_namespace(reader, filter, namespace, next + 1)
        } else {
            self.nested
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::with_namespace(namespace))
        };

        for name in reader[namespace].keys() {
            if filter.includes_type_name(namespace, name) {
                tree.items.insert(name);
            }
        }

        tree
    }

    // TODO: This function takes the tree and expands it to include required dependencies.
    pub fn with_dependencies(self) -> Self {
        self
    }

    // TODO: this should be used when the --flatten option is requested to avoid module nesting of output
    pub fn flatten(self) -> Self {
        self
    }
}
