use super::*;

#[derive(Debug)]
pub struct Tree {
    pub namespace: &'static str,
    pub nested: HashMap<&'static str, Tree>,
    pub items: HashSet<&'static str>,
}

impl Tree {
    pub fn new(reader: &winmd::Reader, filter: &Filter, include_dependencies: bool) -> Self {
        let mut tree = Tree::with_namespace("");
        let mut dependencies = HashMap::new();
        let dependencies = &include_dependencies.then(|| &mut dependencies);

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                tree.insert_namespace(reader, filter, dependencies, namespace, 0);
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

    fn insert_namespace(
        &mut self,
        reader: &winmd::Reader,
        filter: &Filter,
        dependencies: &Option<&mut HashMap<&'static str, HashSet<&'static str>>>,
        namespace: &'static str,
        pos: usize,
    ) {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.nested
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::with_namespace(&namespace[..next]))
                .insert_namespace(reader, filter, dependencies, namespace, next + 1);
        } else {
            let tree = self
                .nested
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::with_namespace(namespace));

            for name in reader[namespace].keys() {
                if filter.includes_type_name(namespace, name) {
                    tree.items.insert(name);

                    // if let Some(dependencies) = dependencies {
                    //     // TODO: ask reader for dependencies of item
                    // }
                }
            }
        }
    }

    // fn insert_dependency(&mut self, namespace: &'static str, pos: usize, name: &'static str) {
    //     let tree = if let Some(next) = namespace[pos..].find('.') {
    //         let next = pos + next;
    //         self.nested
    //             .entry(&namespace[pos..next])
    //             .or_insert_with(|| Self::with_namespace(&namespace[..next]))
    //             .insert_dependency(, namespace, next + 1, name)
    //     } else {
    //         self.nested
    //             .entry(&namespace[pos..])
    //             .or_insert_with(|| Self::with_namespace(namespace))
    //     };

    //     tree.items.insert(name);
    //     tree
    // }
}
