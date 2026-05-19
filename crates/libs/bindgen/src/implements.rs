use super::*;

/// Describes how the `_Impl` scaffolding and caller-side method wrappers should
/// be emitted for a given interface type. Computed once by
/// [`Config::implement_mode`] from the `--implement`/`--implements` flags,
/// the `?Ns.Type` trait-only filter, and the caller-provided default
/// (typically based on interface exclusivity).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImplementMode {
    /// No `_Impl` trait is emitted. Caller-side method wrappers are still
    /// emitted as usual.
    None,
    /// The `_Impl` trait (and vtable thunks) IS emitted so the interface can
    /// be implemented, but the inherent `impl IFace { fn Method… }` caller-side
    /// wrapper block is suppressed. Corresponds to the `?Ns.Type` prefix in
    /// `--filter`.
    TraitOnly,
    /// Both the `_Impl` trait and the caller-side method wrappers are emitted.
    Full,
}

impl ImplementMode {
    /// `true` when the `_Impl` trait should be emitted.
    pub fn has_impl_trait(self) -> bool {
        matches!(self, Self::TraitOnly | Self::Full)
    }

    /// `true` when the inherent caller-side `impl IFace { fn Method… }` block
    /// should be emitted (i.e. not suppressed by a `?` trait-only filter).
    pub fn has_caller_wrappers(self) -> bool {
        matches!(self, Self::None | Self::Full)
    }
}

/// Matcher used by the `--implements` option to determine whether the `_Impl`
/// scaffolding for a given type should be emitted.
///
/// Patterns may be a fully-qualified type name (`Namespace.Name`) or a
/// namespace prefix (which matches every type under that namespace).
#[derive(Debug, Default)]
pub struct Implements(Vec<String>);

impl Implements {
    pub fn new(patterns: &[&str]) -> Self {
        Self(patterns.iter().map(|s| s.to_string()).collect())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn matches(&self, name: &TypeName) -> bool {
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

/// Returns `true` if `rule` selects the type `namespace.name`. A `rule` whose
/// length is less than or equal to `namespace.len()` is treated as a namespace
/// prefix and matches every type whose namespace starts with `rule`. Otherwise
/// `rule` must be a fully-qualified `Namespace.Name` whose namespace component
/// equals `namespace` exactly and whose name component equals `name` exactly.
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
