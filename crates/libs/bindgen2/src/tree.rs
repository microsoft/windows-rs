use super::*;

pub struct Tree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Tree>,
    pub items: HashMap<&'static str, Vec<winmd::Item>>,
}

impl Tree {
    pub fn new(reader: &winmd::Reader, filter: &Filter, flatten: bool) -> Self {
        let mut tree = Tree::with_namespace("");

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                tree.insert_namespace(namespace, 0);
            }
        }

        if flatten {
            todo!("move everything from nested tree into items")
        }

        tree
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: HashMap::new(),
            items: HashMap::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.nested
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::with_namespace(&namespace[..next]))
                .insert_namespace(namespace, next + 1)
        } else {
            self.nested
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::with_namespace(namespace))
        }
    }
}
