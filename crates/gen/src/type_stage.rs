use crate::tables::TypeDef;
use crate::type_limits::TypeLimits;
use crate::type_tree::TypeTree;
use crate::types::Type;
use crate::TypeReader;

use std::collections::*;

/// A map between type def and the fully resolved types
#[derive(Default)]
pub struct TypeStage { key: BTreeSet<TypeDef>, tree: TypeTree }

impl TypeStage {
    /// Resolve types from the relevant types in a [`TypeLimit`]
    pub fn from_limits(reader: &TypeReader, limits: &TypeLimits) -> Self {
        let mut stage = Self::default();

        // tODO: perhaps the slowness is because the reader is enumerated for every limit 
        // perhaps flip it around so that the namespaces are only enumerated once.
        for limit in limits.limits() {
            //println!("{}", limit.namespace);

            match &limit.limit {
                crate::type_limits::TypeLimit::All => {
                    for def in reader.types[&limit.namespace].values() {
                        stage.insert(reader, *def);
                    }
                }
                crate::type_limits::TypeLimit::Some(types) => {
                    let namespace = &reader.types[&limit.namespace];
                    for name in types {
                        stage.insert(reader,namespace[name]);
                    }
                }
            }

            // for (type_name, def) in reader.namespace_types(&limit.namespace) {
            //     if limit.contains_type(type_name) {
            //         stage.insert(reader, *def);
            //     }
            // }
        }

        stage
    }

    // TODO: collapse TypeStage and TypeTree so we don't need the temporary BTreeMap in TypeStage 
    // and instead just build the TypeTree directly while inserting here. Then into_tree can just return
    // the finished product.

    pub fn insert(&mut self, reader: &TypeReader, def: TypeDef) {
        if self.key.insert(def) {
            let t = def.into_type(reader);
            t.insert_dependencies(reader, self);
            self.tree.insert(t.name().namespace.clone(), t);
        }

        // if let btree_map::Entry::Vacant(entry) = self.key.entry(def) {
        //     let info = entry.insert(def.into_type(reader));

        //     for def in info.dependencies() {
        //         self.insert(reader, def);
        //     }
        // }
    }

    // pub fn insert(&mut self, reader: &TypeReader, def: TypeDef) {
    //     if !self.0.contains_key(&def) {
    //         self.0.insert(def, def.into_type(reader));
    //         self.0[&def].insert_dependencies(reader, self);
    //     }


    //     // if let btree_map::Entry::Vacant(entry) = self.0.entry(def) {
    //     //     entry.insert(def.into_type(reader)).insert_dependencies(reader, self);
    //     // }
    // }

    /// Resolve the types into a type tree for code generation
    pub fn into_tree(self) -> TypeTree {
        self.tree
        // let mut tree = TypeTree::default();
        // self.key
        //     .into_iter()
        //     .for_each(|(_, t)| tree.insert(t.name().namespace.clone(), t));
        // tree
    }
}
