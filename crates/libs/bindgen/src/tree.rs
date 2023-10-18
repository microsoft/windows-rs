use super::*;

#[derive(Debug)]
pub struct Tree {
    pub namespace: &'static str,
    pub nested: std::collections::BTreeMap<&'static str, Tree>,
}

impl Tree {
    pub fn new(reader: &'static metadata::Reader) -> Self {
        let mut tree = Tree::from_namespace("");
        for ns in reader.namespaces() {
            if reader.includes_namespace(ns) {
                tree.insert_namespace(ns, 0);
            }
        }
        tree
    }

    fn from_namespace(namespace: &'static str) -> Self {
        Self { namespace, nested: std::collections::BTreeMap::new() }
    }
    fn insert_namespace(&mut self, namespace: &'static str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.nested.entry(&namespace[pos..next]).or_insert_with(|| Self::from_namespace(&namespace[..next])).insert_namespace(namespace, next + 1)
        } else {
            self.nested.entry(&namespace[pos..]).or_insert_with(|| Self::from_namespace(namespace))
        }
    }
    pub fn flatten(&self) -> Vec<&Self> {
        let mut flatten = if self.namespace.is_empty() { vec![] } else { vec![self] };
        flatten.extend(self.nested.values().flat_map(|tree| tree.flatten()));
        flatten
    }
}
