use crate::TypeReader;

use std::collections::BTreeSet;

/// The set of relevant namespaces and types
pub struct TypeLimits<'a> {
    reader: &'a TypeReader,
    inner: BTreeSet<NamespaceLimit>,
}

impl<'a> TypeLimits<'a> {
    pub fn new(reader: &'a TypeReader) -> Self {
        Self {
            reader,
            inner: BTreeSet::new(),
        }
    }

    /// Insert a namespace into the set of relevant namespaces
    ///
    /// expects the namespace in the form: `parent::namespace::*`s
    pub fn insert(&mut self, mut limit: NamespaceLimit) {
        let namespace = match self
            .reader
            .types
            .iter()
            .find(|(name, _)| name.to_lowercase() == limit.namespace)
        {
            Some((n, _)) => n,
            None => panic!("Namespace `{}` not found in winmd files", limit.namespace),
        };

        limit.namespace = namespace.clone();
        self.inner.insert(limit);
    }

    pub fn limits(&self) -> impl Iterator<Item = &NamespaceLimit> {
        self.inner.iter()
    }
}

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NamespaceLimit {
    pub namespace: String,
    pub limit: TypeLimit,
}

impl NamespaceLimit {
    pub fn contains_type(&self, typ: &str) -> bool {
        match &self.limit {
            TypeLimit::All => true,
            TypeLimit::Some(types) => types.iter().any(|t| t == typ),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeLimit {
    All,
    Some(Vec<String>),
}

impl Default for TypeLimit {
    fn default() -> Self {
        TypeLimit::All
    }
}
