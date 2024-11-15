use super::*;

#[derive(Debug)]
pub struct NameTree (
    HashSet<TypeName<'static>>
);

impl std::ops::Deref for NameTree {
    type Target = HashSet<TypeName<'static>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: can't just use filter onces name tree is built since the filter won't include dependencies

impl NameTree {
    pub fn new(reader: &'static Reader, filter: &Filter) -> Self {
        let mut set = HashSet::new();
        let mut dependencies = Dependencies::new();

        for namespace in reader.keys() {
            if filter.includes_namespace(namespace) {
                for name in reader[namespace].keys() {
                    if filter.includes_type_name(namespace, name) {
                        let mut item_dependencies = Dependencies::new();

                        for item in &reader[namespace][name] {
                            item.dependencies(&mut item_dependencies);
                        }

                        if item_dependencies.excluded(filter) {
                            continue;
                        }

                        set.insert(TypeName(namespace, name));
                        dependencies.combine(&item_dependencies);
                    }
                }
            }
        }

        for (namespace, name) in dependencies.iter() {
            set.insert(TypeName(namespace, name));
        }

        Self(set)
    }

    pub fn includes_type_name(&self, namespace: &str, name: &str) -> bool {
        self.0.contains(&TypeName(namespace, name))
    }

}
