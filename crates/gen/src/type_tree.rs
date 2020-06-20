use crate::type_namespaces::TypeNamespaces;
use crate::types::Type;
use proc_macro2::TokenStream;

/// A namespaced tree of types
#[derive(Default)]
pub struct TypeTree {
    types: Vec<Type>,
    namespaces: TypeNamespaces,
}

impl TypeTree {
    /// Insert a [`Type`] into [`TypeTree`]
    ///
    /// This recursively searchs the tree for an entry corresponding to the namespace
    pub fn insert(&mut self, namespace: String, t: Type) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .0
                .entry(namespace[..pos].to_string())
                .or_default()
                .insert(namespace[pos + 1..].to_string(), t);
        } else {
            self.namespaces
                .0
                .entry(namespace)
                .or_default()
                .types
                .push(t);
        }
    }

    /// Turn the tree into a token stream for code generation
    pub fn to_tokens<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.types
            .iter()
            .map(|t| t.to_tokens())
            .chain(self.namespaces.to_tokens())
    }
}

#[cfg(test)]
mod tests {
    use crate::TypeReader;
    use crate::TypeStage;
    use crate::{NamespaceTypes, TypeLimit, TypeLimits};

    #[test]
    fn test_dependency_inclusion() {
        let reader = &TypeReader::from_os();
        let mut limits = TypeLimits::new(reader);
        limits
            .insert(NamespaceTypes {
                namespace: "windows.foundation".to_owned(),
                limit: TypeLimit::All,
            })
            .unwrap();
        limits
            .insert(NamespaceTypes {
                namespace: "windows.ui".to_owned(),
                limit: TypeLimit::All,
            })
            .unwrap();
        let stage = TypeStage::from_limits(reader, &limits);

        // Since Windows.Foundation depends on Windows.Foundation.Collections and
        // Windows.UI doesn't have dependencies, we should only see those namespaces.
        let root = stage.into_tree();

        // There is one root namespace.
        assert!(root.namespaces.0.len() == 1);
        let windows = &root.namespaces.0["Windows"];

        // The Windows namespace will only contain Foundation and UI.
        assert!(windows.namespaces.0.len() == 2);
        let foundation = &windows.namespaces.0["Foundation"];
        let ui = &windows.namespaces.0["UI"];

        // The UI namespace will not contain any further namespaces.
        assert!(ui.namespaces.0.is_empty());

        // The Foundation namespace will contain the Collections namespace.
        assert!(foundation.namespaces.0.len() == 1);
        let collections = &foundation.namespaces.0["Collections"];

        // The Collections namespace will not contain any further namespaces.
        assert!(collections.namespaces.0.is_empty());

        // The root never has any types.
        assert!(root.types.is_empty());

        // The Windows namespace has no types.
        assert!(windows.types.is_empty());

        // The UI namespace has all of its types.
        assert!(ui.types.iter().any(|t| t.name().name == "Colors"));
        assert!(ui.types.iter().any(|t| t.name().name == "IColorsStatics"));

        // The Foundation namespace has all of its types.
        assert!(foundation.types.iter().any(|t| t.name().name == "Uri"));
        assert!(foundation
            .types
            .iter()
            .any(|t| t.name().name == "IStringable"));

        // The Collections namespace only has the needed types.
        assert!(collections
            .types
            .iter()
            .any(|t| t.name().name == "IVectorView`1"));
        assert!(
            collections
                .types
                .iter()
                .any(|t| t.name().name == "PropertySet")
                == false
        );
    }
}
