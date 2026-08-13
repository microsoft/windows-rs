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
    rules: Vec<(String, bool)>,
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
        let name = name.into();
        self.rules.push((name.clone(), true));
        self.names.insert(name);
        self
    }

    /// Includes one item by exact metadata namespace and name.
    pub fn include_item(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> &mut Self {
        let namespace = namespace.into();
        let name = name.into();
        self.rules.push((format!("{namespace}.{name}"), true));
        self.items.entry(namespace).or_default().insert(name);
        self
    }

    /// Includes all items in this metadata namespace and its child namespaces.
    pub fn include_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        let namespace = namespace.into();
        self.rules.push((namespace.clone(), true));
        self.namespaces.insert(namespace);
        self
    }

    /// Includes one method by exact metadata namespace, type, and method name.
    pub fn include_method(
        &mut self,
        namespace: impl Into<String>,
        ty: impl Into<String>,
        method: impl Into<String>,
    ) -> &mut Self {
        let namespace = namespace.into();
        let ty = ty.into();
        self.rules.push((format!("{namespace}.{ty}"), true));
        self.methods
            .entry(namespace)
            .or_default()
            .entry(ty)
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
        if let Some(path) = path.strip_prefix('!') {
            let mut resolved = Self::new();
            resolved.include_path(database, path)?;
            for (rule, _) in resolved.rules {
                self.rules.push((rule, false));
            }
            return Ok(self);
        }

        if let Some((prefix, names)) = path
            .strip_suffix('}')
            .and_then(|path| path.rsplit_once("::{"))
        {
            let parts = path_parts(prefix);
            let (parent, ty) = parts.split_at(parts.len() - 1);
            if names.trim().is_empty() {
                if parent.is_empty() {
                    let namespaces = type_namespaces(database, ty[0]);
                    if namespaces.is_empty() {
                        return Err(unresolved(path));
                    }
                    for namespace in namespaces {
                        self.methods
                            .entry(namespace.to_string())
                            .or_default()
                            .entry(ty[0].to_string())
                            .or_default();
                    }
                } else if type_exists(database, &parent.join("."), ty[0]) {
                    self.methods
                        .entry(parent.join("."))
                        .or_default()
                        .entry(ty[0].to_string())
                        .or_default();
                } else {
                    return Err(unresolved(path));
                }
                return Ok(self);
            }
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
                let namespace = parent.join(".");
                for member in names.split(',').map(str::trim) {
                    if !namespace.starts_with("Windows.Win32")
                        || type_has_method(database, &namespace, ty[0], member)
                    {
                        self.include_method(&namespace, ty[0], member);
                    } else {
                        self.include_item(&namespace, member);
                    }
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
        if type_exists(database, &namespace.join("."), name[0]) {
            self.include_item(namespace.join("."), name[0]);
            return Ok(self);
        }
        if namespace_exists(database, &parts.join(".")) {
            self.include_namespace(parts.join("."));
            return Ok(self);
        }
        if namespace_exists(database, &namespace.join(".")) {
            self.include_item(namespace.join("."), name[0]);
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

    pub(crate) fn includes_exact_item(&self, namespace: &str, name: &str) -> bool {
        self.is_included(namespace, name)
            && self
                .items
                .get(namespace)
                .is_some_and(|names| names.contains(name))
    }

    pub(crate) fn includes_exact_type(&self, namespace: &str, name: &str) -> bool {
        self.is_included(namespace, name)
            && (self.includes_exact_item(namespace, name)
                || self
                    .methods
                    .get(namespace)
                    .is_some_and(|types| types.contains_key(name)))
    }

    pub(crate) fn includes(&self, namespace: &str, name: &str) -> bool {
        self.is_included(namespace, name)
            && (self.names.contains(name)
                || self
                    .items
                    .get(namespace)
                    .is_some_and(|names| names.contains(name))
                || self.namespaces.iter().any(|included| {
                    namespace == included
                        || namespace
                            .strip_prefix(included)
                            .is_some_and(|suffix| suffix.starts_with('.'))
                }))
    }

    pub(crate) fn excludes(&self, namespace: &str, name: &str) -> bool {
        self.rule_decision(namespace, name) == Some(false)
    }

    fn is_included(&self, namespace: &str, name: &str) -> bool {
        self.rule_decision(namespace, name) == Some(true)
    }

    fn rule_decision(&self, namespace: &str, name: &str) -> Option<bool> {
        let full_name = format!("{namespace}.{name}");
        self.rules
            .iter()
            .filter(|(rule, _)| {
                (!rule.contains('.') && name == rule)
                    || full_name == *rule
                    || full_name
                        .strip_prefix(rule)
                        .is_some_and(|suffix| suffix.starts_with('.'))
            })
            .max_by_key(|(rule, include)| (rule.len(), !*include))
            .map(|(_, include)| *include)
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

fn type_has_method(database: &Database, namespace: &str, name: &str, method: &str) -> bool {
    database
        .type_definitions(namespace, name)
        .iter()
        .filter_map(|entity| database.definition(*entity))
        .filter_map(|definition| definition.methods().ok())
        .flatten()
        .any(|candidate| candidate.name().ok() == Some(method))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_filter_rule_wins() {
        let mut filter = Filter::new();
        filter.rules.push(("Windows".to_string(), false));
        filter.rules.push(("Windows.Win32".to_string(), true));
        filter
            .rules
            .push(("Windows.Win32.Metadata".to_string(), false));

        assert!(filter.is_included("Windows.Win32.System", "Apis"));
        assert!(!filter.is_included("Windows.Foundation", "Uri"));
        assert!(!filter.is_included("Windows.Win32.Metadata", "ApiDetails"));
    }
}
