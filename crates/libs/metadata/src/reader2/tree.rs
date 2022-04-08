use super::*;

#[derive(Debug)]
pub struct Tree<'a> {
    namespace: &'a str,
    nested: BTreeMap<&'a str, Tree<'a>>,
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
}
