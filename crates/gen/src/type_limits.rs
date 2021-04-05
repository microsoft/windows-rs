use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub struct TypeLimits {
    reader: &'static TypeReader,
    inner: BTreeMap<NamespaceTypes, TypeLimitMeta>,
}

impl TypeLimits {
    pub fn new(reader: &'static TypeReader) -> Self {
        Self {
            reader,
            inner: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, mut limit: NamespaceTypes, namespace: TypeLimitMeta) {
        limit.namespace = self.reader.resolve_namespace(&limit.namespace);
        self.inner.insert(limit, namespace);
    }

    pub fn limits(&self) -> impl Iterator<Item = (&NamespaceTypes, &TypeLimitMeta)> {
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

/// Additional metadata associated with a namespace.
#[derive(Clone)]
pub struct TypeLimitMeta {
    /// Constraints in place for compiling the type. Like feature flags.
    pub constraints: BTreeSet<TypeConstraint>,
}
