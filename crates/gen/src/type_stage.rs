use crate::tables::TypeDef;
use crate::type_limits::TypeLimits;
use crate::type_tree::TypeTree;
use crate::types::Type;
use crate::TypeReader;

use std::collections::*;

/// A map between type def and the fully resolved types
#[derive(Default, Debug)]
pub struct TypeStage(pub BTreeMap<TypeDef, Type>);

impl TypeStage {
    /// Resolve types from the relevant types in a [`TypeLimit`]
    pub fn from_limits(reader: &TypeReader, limits: &TypeLimits) -> Self {
        let mut stage = Self::default();

        for limit in limits.limits() {
            for (type_name, def) in reader.namespace_types(&limit.namespace) {
                if limit.contains_type(type_name) {
                    stage.insert(reader, *def);
                }
            }
        }

        stage
    }

    fn insert(&mut self, reader: &TypeReader, def: TypeDef) {
        if !self.0.contains_key(&def) {
            let info = def.into_type(reader);
            let depends = info.dependencies();
            self.0.insert(def, info);
            for def in depends {
                self.insert(reader, def);
            }
        }
    }

    /// Resolve the types into a type tree for code generation
    pub fn into_tree(self) -> TypeTree {
        let mut tree = TypeTree::default();
        self.0
            .into_iter()
            .for_each(|(_, t)| tree.insert(t.name().namespace.clone(), t));
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NamespaceTypes, TypeLimit};

    #[test]
    fn test_dependency_inclusion() {
        let reader = &TypeReader::from_os();

        // Windows.Foundation depends on types in Windows.Foundation.Collections
        // Since Windows.Foundation.Collections is not added to the type limits,
        // only the types that are actually needed will be included.
        let mut limits = TypeLimits::new(reader);
        limits
            .insert(NamespaceTypes {
                namespace: "windows.foundation".to_owned(),
                limit: TypeLimit::All,
            })
            .unwrap();
        let stage = TypeStage::from_limits(reader, &limits);

        // Windows.Foundation.WwwFormUrlDecoder depends on Windows.Foundation.Collections.IVectorView`1
        // so that's included.
        assert!(stage.0.values().any(|t| t.name().name == "IVectorView`1"));

        // Windows.Foundation does not however depend on Windows.Foundation.Collections.PropertySet
        // so that's not included.
        assert!(stage.0.values().any(|t| t.name().name == "PropertySet") == false);
    }
}
