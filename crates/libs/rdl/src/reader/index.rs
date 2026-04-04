use super::*;

#[derive(Default)]
pub struct Index<'a> {
    pub namespaces: BTreeMap<String, Namespace<'a>>,
}

#[derive(Default)]
pub struct Namespace<'a> {
    pub types: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
    pub functions: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
    pub constants: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
}

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, file: &'a File, namespace: &str, item: &'a Item) {
        if let Item::Module(module) = item {
            let name = module.to_string();
            for item in &module.items {
                let namespace = if namespace.is_empty() {
                    name.clone()
                } else {
                    format!("{namespace}.{}", &name)
                };
                self.insert(file, &namespace, item);
            }
        } else {
            let namespace = self.namespaces.entry(namespace.to_string()).or_default();

            match item {
                Item::Fn(..) => {
                    namespace
                        .functions
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
                Item::Const(..) => {
                    namespace
                        .constants
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
                _ => {
                    namespace
                        .types
                        .entry(item.to_string())
                        .or_default()
                        .push((file, item));
                }
            }
        }
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .is_some_and(|v| !v.is_empty())
    }

    pub fn is_value_type(&self, namespace: &str, name: &str) -> bool {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .and_then(|v| v.first())
            .map(|(_, item)| matches!(item, Item::Struct(_) | Item::Enum(_) | Item::Union(_)))
            .unwrap_or(false)
    }

    pub fn get(&self, namespace: &str, name: &str) -> Option<&Item> {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .and_then(|v| v.first()) // TODO: should probably return an iterator instead
            .map(|(_, item)| *item)
    }
}
