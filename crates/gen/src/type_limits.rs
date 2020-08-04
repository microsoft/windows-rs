use crate::TypeReader;

use std::collections::BTreeSet;

/// The set of relevant namespaces and types
pub struct TypeLimits<'a> {
    reader: &'a TypeReader,
    inner: BTreeSet<NamespaceTypes>,
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
    pub fn insert(&mut self, mut limit: NamespaceTypes) -> Result<(), String> {
        let namespace = match self
            .reader
            .types
            .iter()
            .find(|(name, _)| name.to_lowercase() == limit.namespace.to_lowercase())
        {
            Some((n, _)) => n,
            None => return Err(limit.namespace),
        };

        limit.namespace = namespace.clone();
        self.inner.insert(limit);
        Ok(())
    }

    pub fn limits(&self) -> impl Iterator<Item = &NamespaceTypes> {
        self.inner.iter()
    }
}

/// A namespace's relevant types
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NamespaceTypes {
    pub namespace: String,
    pub limit: TypeLimit,
}

impl NamespaceTypes {
    pub fn contains_type(&self, typ: &str) -> bool {
        match &self.limit {
            TypeLimit::All => true,
            TypeLimit::Some(types) => types.iter().any(|t| t == typ),
        }
    }
}

/// A limit on the types in a namespace.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeLimit {
    /// All the types in a namespace
    All,
    /// Some types in the namespace
    Some(Vec<String>),
}
