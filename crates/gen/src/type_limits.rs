use std::collections::BTreeSet;

/// The set of relevant namespaces and types
pub struct TypeLimits {
    reader: &'static winmd::TypeReader,
    pub inner: BTreeSet<NamespaceTypes>,
}

impl TypeLimits {
    pub fn new(reader: &'static winmd::TypeReader) -> Self {
        Self {
            reader,
            inner: BTreeSet::new(),
        }
    }

    /// Insert a namespace into the set of relevant namespaces
    ///
    /// expects the namespace in the form: `parent::namespace::*`s
    pub fn insert(&mut self, mut limit: NamespaceTypes) -> Result<(), &'static str> {
        if let Some(namespace) = self
            .reader
            .find_lowercase_namespace(&limit.namespace.to_lowercase())
        {
            limit.namespace = namespace;
            self.inner.insert(limit);
            Ok(())
        } else {
            Err(limit.namespace)
        }
    }

    pub fn limits(&self) -> impl Iterator<Item = &NamespaceTypes> {
        self.inner.iter()
    }
}

/// A namespace's relevant types
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct NamespaceTypes {
    pub namespace: &'static str,
    pub limit: TypeLimit,
}

/// A limit on the types in a namespace.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeLimit {
    /// All the types in a namespace
    All,
    /// Some types in the namespace
    Some(Vec<String>),
}
