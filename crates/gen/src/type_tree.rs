use crate::*;
use squote::TokenStream;

/// A namespaced tree of types
#[derive(Default)]
pub struct TypeTree {
    pub types: Vec<TypeDefinition>,
    pub namespaces: TypeNamespaces,
    pub include_foundation: bool,
}

impl TypeTree {
    pub fn from_limits(reader: &'static winmd::TypeReader, limits: &TypeLimits) -> Self {
        let mut tree = TypeTree::default();
        let mut set = std::collections::BTreeSet::new();

        for limit in limits.limits() {
            match &limit.limit {
                TypeLimit::All => {
                    for def in reader.namespace_types(&limit.namespace) {
                        tree.insert_if(reader, &mut set, &def);
                    }
                }
                TypeLimit::Some(types) => {
                    for name in types {
                        tree.insert_if(
                            reader,
                            &mut set,
                            &reader.expect_type((&limit.namespace, name)),
                        );
                    }
                }
            }
        }

        tree
    }

    fn insert_if(
        &mut self,
        reader: &winmd::TypeReader,
        set: &mut std::collections::BTreeSet<winmd::TypeDef>,
        def: &winmd::Type,
    ) {
        match def {
            winmd::Type::TypeDef(def) => match def.category() {
                winmd::TypeCategory::Contract | winmd::TypeCategory::Attribute => {}
                _ => {
                    if set.insert(*def) {
                        let t = TypeDefinition::from_type_def(def);

                        for def in t.dependencies() {
                            self.insert_if(reader, set, &winmd::Type::TypeDef(def));
                        }

                        self.insert(t.name().namespace, t);
                    }
                }
            },
            winmd::Type::MethodDef((def, method)) => {
                let t = TypeDefinition::from_method_def(def, method);

                for def in t.dependencies() {
                    self.insert_if(reader, set, &winmd::Type::TypeDef(def));
                }

                self.insert(t.name().namespace, t);
            }
            winmd::Type::Field((def, field)) => {
                let t = TypeDefinition::from_field(def, field);
                self.insert(t.name().namespace, t);
            }
        }
    }

    /// Insert a [`TypeDefinition`] into [`TypeTree`]
    ///
    /// This recursively searchs the tree for an entry corresponding to the namespace
    pub fn insert(&mut self, namespace: &'static str, t: TypeDefinition) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .0
                .entry(&namespace[..pos])
                .or_default()
                .insert(&namespace[pos + 1..], t);
        } else {
            self.namespaces
                .0
                .entry(namespace)
                .or_default()
                .types
                .push(t);
        }
    }

    pub fn remove(&mut self, namespace: &str) {
        if let Some(pos) = namespace.find('.') {
            if let Some(tree) = self.namespaces.0.get_mut(&namespace[..pos]) {
                tree.remove(&namespace[pos + 1..])
            }
        } else {
            self.namespaces.0.remove(namespace);
        }
    }

    pub fn reexport(&mut self) {
        self.namespaces
            .0
            .entry("Windows")
            .or_default()
            .include_foundation = true;
    }

    /// Turn the tree into a token stream for code generation
    pub fn gen<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.types
            .iter()
            .map(|t| t.gen())
            .chain(self.namespaces.gen())
    }
}

#[cfg(test)]
mod tests {
    use crate::{NamespaceTypes, TypeLimit, TypeLimits, TypeTree};

    #[test]
    fn test_dependency_inclusion() {
        let reader = winmd::TypeReader::get();
        let mut limits = TypeLimits::new(reader);
        limits
            .insert(NamespaceTypes {
                namespace: "windows.foundation",
                limit: TypeLimit::All,
            })
            .unwrap();
        limits
            .insert(NamespaceTypes {
                namespace: "windows.ui",
                limit: TypeLimit::All,
            })
            .unwrap();

        // Since Windows.Foundation depends on Windows.Foundation.Collections and
        // Windows.UI doesn't have dependencies, we should only see those namespaces.
        let root = TypeTree::from_limits(reader, &limits);

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
