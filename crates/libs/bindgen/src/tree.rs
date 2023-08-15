use super::*;

#[derive(Debug)]
pub struct Tree<'a> {
    pub namespace: &'a str,
    pub nested: std::collections::BTreeMap<&'a str, Tree<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new(reader: &'a metadata::Reader, filter: &'a metadata::Filter) -> Self {
        let mut tree = Tree::from_namespace("");
        for ns in reader.namespaces() {
            if filter.includes_namespace(ns) {
                tree.insert_namespace(ns, 0);
            }
        }
        tree
    }

    fn from_namespace(namespace: &'a str) -> Self {
        Self { namespace, nested: std::collections::BTreeMap::new() }
    }
    fn insert_namespace(&mut self, namespace: &'a str, pos: usize) -> &mut Self {
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
