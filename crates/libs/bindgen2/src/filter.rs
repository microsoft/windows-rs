use std::collections::{BTreeMap, BTreeSet};

/// Exact item and namespace inclusions for one generation request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Filter {
    names: BTreeSet<String>,
    items: BTreeMap<String, BTreeSet<String>>,
    methods: BTreeMap<String, BTreeMap<String, BTreeSet<String>>>,
    namespaces: BTreeSet<String>,
}

impl Filter {
    /// Creates an empty filter that selects no items.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a filter that includes each item name in any namespace.
    pub fn names<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut filter = Self::new();
        for name in names {
            filter.include_name(name);
        }
        filter
    }

    /// Includes every item with this name in any namespace.
    pub fn include_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.names.insert(name.into());
        self
    }

    /// Includes one item by exact metadata namespace and name.
    pub fn include_item(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> &mut Self {
        self.items
            .entry(namespace.into())
            .or_default()
            .insert(name.into());
        self
    }

    /// Includes all items in this metadata namespace and its child namespaces.
    pub fn include_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespaces.insert(namespace.into());
        self
    }

    /// Includes one method by exact metadata namespace, type, and method name.
    pub fn include_method(
        &mut self,
        namespace: impl Into<String>,
        ty: impl Into<String>,
        method: impl Into<String>,
    ) -> &mut Self {
        self.methods
            .entry(namespace.into())
            .or_default()
            .entry(ty.into())
            .or_default()
            .insert(method.into());
        self
    }

    pub(crate) fn methods(&self, namespace: &str, name: &str) -> Option<&BTreeSet<String>> {
        self.methods
            .get(namespace)
            .and_then(|types| types.get(name))
    }

    pub(crate) fn includes(&self, namespace: &str, name: &str) -> bool {
        self.names.contains(name)
            || self
                .items
                .get(namespace)
                .is_some_and(|names| names.contains(name))
            || self.namespaces.iter().any(|included| {
                namespace == included
                    || namespace
                        .strip_prefix(included)
                        .is_some_and(|suffix| suffix.starts_with('.'))
            })
    }
}
