use std::collections::{BTreeMap, BTreeSet};
use windows_metadata2::Database;

/// A filter path that does not resolve to metadata.
#[derive(Debug)]
pub struct FilterError(String);

impl std::fmt::Display for FilterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for FilterError {}

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

    /// Resolves and includes a Rust-use-style metadata path.
    pub fn include_path(
        &mut self,
        database: &Database,
        path: &str,
    ) -> Result<&mut Self, FilterError> {
        if path.starts_with('!') {
            return Err(FilterError(format!(
                "filter exclusions are not supported: `{path}`"
            )));
        }

        if let Some((prefix, names)) = path
            .strip_suffix('}')
            .and_then(|path| path.rsplit_once("::{"))
        {
            let parts = path_parts(prefix);
            let (parent, ty) = parts.split_at(parts.len() - 1);
            if parent.is_empty() {
                let namespaces = type_namespaces(database, ty[0]);
                if namespaces.is_empty() {
                    return Err(unresolved(path));
                }
                for namespace in namespaces {
                    for method in names.split(',').map(str::trim) {
                        self.include_method(namespace, ty[0], method);
                    }
                }
            } else if type_exists(database, &parent.join("."), ty[0]) {
                for method in names.split(',').map(str::trim) {
                    self.include_method(parent.join("."), ty[0], method);
                }
            } else {
                let namespace = parts.join(".");
                if !namespace_exists(database, &namespace) {
                    return Err(unresolved(path));
                }
                for name in names.split(',').map(str::trim) {
                    self.include_item(&namespace, name);
                }
            }
            return Ok(self);
        }

        let parts = path_parts(path);
        if parts.len() == 1 {
            let name = parts[0];
            self.include_name(name);
            if namespace_exists(database, name) {
                self.include_namespace(name);
            }
            return Ok(self);
        }

        let (namespace, name) = parts.split_at(parts.len() - 1);
        if type_exists(database, &namespace.join("."), name[0])
            || namespace_exists(database, &namespace.join("."))
        {
            self.include_item(namespace.join("."), name[0]);
            return Ok(self);
        }
        if namespace_exists(database, &parts.join(".")) {
            self.include_namespace(parts.join("."));
            return Ok(self);
        }

        let (namespace, ty) = namespace.split_at(namespace.len() - 1);
        if namespace.is_empty() {
            let namespaces = type_namespaces(database, ty[0]);
            if namespaces.is_empty() {
                return Err(unresolved(path));
            }
            for namespace in namespaces {
                self.include_method(namespace, ty[0], name[0]);
            }
        } else if type_exists(database, &namespace.join("."), ty[0]) {
            self.include_method(namespace.join("."), ty[0], name[0]);
        } else {
            return Err(unresolved(path));
        }
        Ok(self)
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

fn path_parts(path: &str) -> Vec<&str> {
    path.split([':', '.'])
        .filter(|part| !part.is_empty())
        .collect()
}

fn type_namespaces<'a>(database: &'a Database, ty: &str) -> Vec<&'a str> {
    database
        .type_names()
        .filter_map(|(namespace, name, _)| (name == ty).then_some(namespace))
        .collect()
}

fn type_exists(database: &Database, namespace: &str, name: &str) -> bool {
    !database.type_definitions(namespace, name).is_empty()
}

fn namespace_exists(database: &Database, namespace: &str) -> bool {
    database.type_names().any(|(candidate, _, _)| {
        candidate == namespace
            || candidate
                .strip_prefix(namespace)
                .is_some_and(|suffix| suffix.starts_with('.'))
    })
}

fn unresolved(path: &str) -> FilterError {
    FilterError(format!("unresolved filter path `{path}`"))
}
