use super::*;

#[derive(Debug)]
pub struct Tree<'a> {
    pub namespace: &'a str,
    pub nested: BTreeMap<&'a str, Tree<'a>>,
}

impl<'a> Tree<'a> {
    pub(crate) fn from_namespace(namespace: &'a str) -> Self {
        Self { namespace, nested: BTreeMap::new() }
    }
    pub(crate) fn insert_namespace(&mut self, namespace: &'a str, pos: usize) -> &mut Self {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.nested.entry(&namespace[pos..next]).or_insert_with(|| Self::from_namespace(&namespace[..next])).insert_namespace(namespace, next + 1)
        } else {
            self.nested.entry(&namespace[pos..]).or_insert_with(|| Self::from_namespace(namespace))
        }
    }
    pub fn flatten(&self) -> Vec<&Self> {
        std::iter::once(self).chain(self.nested.values().flat_map(|tree| tree.flatten())).collect()
    }
    pub fn seek(mut self, namespace: &'a str) -> Option<Self> {
        if let Some(next) = namespace.find('.') {
            self.nested.remove(&namespace[..next]).and_then(|tree| tree.seek(&namespace[next + 1..]))
        } else {
            self.nested.remove(namespace)
        }
    }
}
