use crate::tables::TypeDef;
use crate::type_limits::TypeLimits;
use crate::types::Type;
use crate::types::TypeTree;
use crate::TypeReader;

use std::collections::*;

/// A map between type def and the fully resolved types
#[derive(Default, Debug)]
pub struct TypeStage(pub BTreeMap<TypeDef, Type>);

impl TypeStage {
    /// Resolve types from the relevant types in a [`TypeLimit`]
    pub fn from_limits(reader: &TypeReader, limits: &TypeLimits) -> Self {
        let mut stage = Self::default();

        for namespace in &limits.0 {
            for def in reader.namespace_types(&namespace) {
                stage.insert(reader, *def);
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
