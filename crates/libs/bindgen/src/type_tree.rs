use super::*;

// BTree collections preserve output order.
#[derive(Debug)]
pub struct TypeTree {
    pub namespace: &'static str,
    pub nested: BTreeMap<&'static str, Self>,
    pub types: BTreeSet<Type>,
}

impl TypeTree {
    pub fn new(dependencies: &TypeMap, flat: bool) -> Self {
        validate_nested_names(dependencies, flat);
        let mut tree = Self::with_namespace("");

        for (tn, types) in dependencies.iter() {
            let tree = tree.insert_namespace(tn.namespace());
            // Insert in a deterministic total order so that when two distinct
            // types share a `sort_key` (same-name collisions in `--flat` layouts),
            // the `BTreeSet` keeps a stable winner (first insert wins on ties).
            let mut sorted: Vec<&Type> = types.iter().collect();
            sorted.sort_by(|a, b| a.dedup_cmp(b));
            sorted.into_iter().for_each(|ty| {
                tree.types.insert(ty.clone());
            });
        }

        fn validate_nested_names(dependencies: &TypeMap, flat: bool) {
            let mut pending: Vec<_> = dependencies
                .values()
                .flatten()
                .map(|ty| (ty.clone(), arch_mask(ty.arches())))
                .collect();
            let mut seen = HashSet::new();
            let mut names: BTreeMap<_, Vec<(windows_metadata::TypeName, i32, bool)>> =
                BTreeMap::new();
            while let Some((ty, emitted_arches)) = pending.pop() {
                if !seen.insert((ty.clone(), emitted_arches)) {
                    continue;
                }
                let name = ty.type_name();
                let (identity, nested) = match &ty {
                    Type::CppStruct(record) => {
                        pending.extend(
                            record
                                .nested
                                .values()
                                .cloned()
                                .map(|child| (Type::CppStruct(child), emitted_arches)),
                        );
                        (record.def.full_name(), record.enclosing.is_some())
                    }
                    Type::CppFn(_) | Type::CppConst(_) => continue,
                    _ if name.name().is_empty() => continue,
                    _ => (
                        windows_metadata::TypeName::named(name.namespace(), name.name()),
                        false,
                    ),
                };
                let key = (if flat { "" } else { name.namespace() }, name.name());
                let entries = names.entry(key).or_default();
                for (other, other_arches, other_nested) in entries.iter() {
                    assert!(
                        !(nested || *other_nested)
                            || emitted_arches & other_arches == 0
                            || identity == *other,
                        "nested type name collision for `{}` between `{}.{}` and `{}.{}`",
                        name.name(),
                        identity.namespace,
                        identity.name,
                        other.namespace,
                        other.name,
                    );
                }
                entries.push((identity, emitted_arches, nested));
            }
        }

        tree
    }

    /// Returns namespace trees in output order.
    pub fn flatten_trees(&self) -> Vec<&Self> {
        let mut flatten = if self.namespace.is_empty() {
            vec![]
        } else {
            vec![self]
        };
        flatten.extend(self.nested.values().flat_map(|tree| tree.flatten_trees()));
        flatten
    }

    /// Flattens all namespace types into output order.
    pub fn flatten_types(mut self) -> BTreeSet<Type> {
        fn flatten(set: &mut BTreeSet<Type>, tree: &mut TypeTree) {
            set.append(&mut tree.types);

            for tree in tree.nested.values_mut() {
                flatten(set, tree);
            }
        }

        let mut types = BTreeSet::new();
        flatten(&mut types, &mut self);
        types
    }

    pub fn feature(&self) -> String {
        namespace_feature(self.namespace)
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: BTreeMap::new(),
            types: BTreeSet::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str) -> &mut Self {
        fn insert_namespace<'a>(
            parent: &'a mut TypeTree,
            namespace: &'static str,
            pos: usize,
        ) -> &'a mut TypeTree {
            if let Some(next) = namespace[pos..].find('.') {
                let next = pos + next;

                let parent = parent
                    .nested
                    .entry(&namespace[pos..next])
                    .or_insert_with(|| TypeTree::with_namespace(&namespace[..next]));

                insert_namespace(parent, namespace, next + 1)
            } else {
                parent
                    .nested
                    .entry(&namespace[pos..])
                    .or_insert_with(|| TypeTree::with_namespace(namespace))
            }
        }

        if namespace.is_empty() {
            self
        } else {
            insert_namespace(self, namespace, 0)
        }
    }
}
