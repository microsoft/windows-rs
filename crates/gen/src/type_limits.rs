use super::*;
use std::collections::BTreeSet;

pub struct TypeLimits {
    reader: &'static TypeReader,
    inner: BTreeSet<NamespaceTypes>,
}

impl TypeLimits {
    pub fn new(reader: &'static TypeReader) -> Self {
        Self {
            reader,
            inner: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, mut limit: NamespaceTypes) {
        limit.namespace = self.reader.resolve_namespace(&limit.namespace);
        self.inner.insert(limit);
    }

    pub fn limits(&self) -> impl Iterator<Item = &NamespaceTypes> {
        self.inner.iter()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NamespaceTypes {
    pub namespace: &'static str,
    pub limit: TypeLimit,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeLimit {
    All,
    Some(Vec<String>),
}
