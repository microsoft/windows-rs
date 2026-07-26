use super::*;

/// Matcher for `--implement` type names and namespace prefixes.
#[derive(Debug, Default)]
pub struct Implements(Vec<String>);

impl Implements {
    pub fn new(patterns: &[&str]) -> Self {
        Self(patterns.iter().map(|s| s.to_string()).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn matches(&self, name: TypeName) -> bool {
        self.0
            .iter()
            .any(|rule| match_type_name(rule, name.namespace(), name.name()))
    }

    /// Like `matches`, but operating on plain string slices. Used during
    /// filter validation where interned `TypeName`s aren't available.
    pub fn matches_str(&self, namespace: &str, name: &str) -> bool {
        self.0
            .iter()
            .any(|rule| match_type_name(rule, namespace, name))
    }
}

/// Returns true when `rule` selects `namespace.name`.
fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace_starts_with(namespace, rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}
