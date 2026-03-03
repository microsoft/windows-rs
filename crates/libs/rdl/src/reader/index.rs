use super::*;

#[derive(Default)]
pub struct Index<'a> {
    pub namespaces: BTreeMap<String, Namespace<'a>>,
}

#[derive(Default)]
pub struct Namespace<'a> {
    pub types: BTreeMap<String, (&'a str, Item)>,
    pub functions: BTreeMap<String, (&'a str, Item)>,
    pub constants: BTreeMap<String, (&'a str, Item)>,
}

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, source_file: &'a str, namespace: &str, item: Item) {
        if let Item::Module(module) = item {
            let name = module.to_string();
            for item in module.items {
                let namespace = if namespace.is_empty() {
                    name.clone()
                } else {
                    format!("{namespace}.{}", &name)
                };
                self.insert(source_file, &namespace, item);
            }
        } else {
            let namespace = self.namespaces.entry(namespace.to_string()).or_default();

            match item {
                Item::Fn(..) => {
                    namespace
                        .functions
                        .insert(item.to_string(), (source_file, item));
                }
                Item::Const(..) => {
                    namespace
                        .constants
                        .insert(item.to_string(), (source_file, item));
                }
                _ => {
                    namespace
                        .types
                        .insert(item.to_string(), (source_file, item));
                }
            }
        }
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .is_some()
    }

    pub fn validate(&self, reference: &metadata::reader::TypeIndex) -> Result<(), Error> {
        for namespace in self.namespaces.values() {
            let items = namespace
                .types
                .values()
                .chain(namespace.functions.values())
                .chain(namespace.constants.values());

            for (source_file, item) in items {
                item.validate(source_file, self, reference)?;
            }
        }

        Ok(())
    }
}
