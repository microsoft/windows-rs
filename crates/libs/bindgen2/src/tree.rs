use super::*;

#[derive(Debug)]
pub struct Tree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Self>,
    pub items: HashSet<&'static str>,
}

impl Tree {
    pub fn new(reader: &winmd::Reader, filter: &Filter, include_dependencies: bool) -> Self {
        let mut tree = Self::with_namespace("");
        let mut dependencies = HashMap::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                let tree = tree.insert_namespace(namespace);

                for name in reader[namespace].keys() {
                    if filter.includes_type_name(namespace, name) {
                        tree.items.insert(name);

                        if include_dependencies {
                            reader.dependencies(namespace, name, &mut dependencies);
                        }
                    }
                }
            }
        }

        // if let Some(dependencies) = dependencies {
        //     for (namespace, names) in dependencies.iter() {
        //         for name in names {
        //             tree.insert_dependency(namespace, name);
        //         }
        //     }
        // }

        tree
    }

    fn with_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            nested: HashMap::new(),
            items: HashSet::new(),
        }
    }

    fn insert_namespace(&mut self, namespace: &'static str) -> &mut Self {
        fn insert_namespace<'a>(
            parent: &'a mut Tree,
            namespace: &'static str,
            pos: usize,
        ) -> &'a mut Tree {
            if let Some(next) = namespace[pos..].find('.') {
                let next = pos + next;

                let parent = parent
                    .nested
                    .entry(&namespace[pos..next])
                    .or_insert_with(|| Tree::with_namespace(&namespace[..next]));

                insert_namespace(parent, namespace, next + 1)
            } else {
                parent
                    .nested
                    .entry(&namespace[pos..])
                    .or_insert_with(|| Tree::with_namespace(namespace))
            }
        }

        insert_namespace(self, namespace, 0)
    }
}
