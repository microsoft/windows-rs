use super::*;

#[derive(Default)]
pub struct Index<'a> {
    pub namespaces: BTreeMap<String, Namespace<'a>>,
}

#[derive(Default)]
pub struct Namespace<'a> {
    /// Multiple items may share the same name when they carry different
    /// `SupportedArchitecture` attributes (e.g. three arch-specific variants
    /// of `SLIST_HEADER`).  All variants are kept so that every one of them
    /// is encoded into the output winmd.
    pub types: BTreeMap<String, Vec<(&'a File, &'a Item)>>,
    pub functions: BTreeMap<String, (&'a File, &'a Item)>,
    pub constants: BTreeMap<String, (&'a File, &'a Item)>,
}

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
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
                    namespace.functions.insert(item.to_string(), (file, item));
                }
                Item::Const(..) => {
                    namespace.constants.insert(item.to_string(), (file, item));
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

    /// Returns the first definition of a type with the given name.
    ///
    /// When multiple arch-specific variants exist they all share the same
    /// underlying structure (e.g. the same field layout for enum underlying
    /// types), so the first entry is sufficient for type-resolution purposes.
    pub fn get(&self, namespace: &str, name: &str) -> Option<&Item> {
        self.namespaces
            .get(namespace)
            .and_then(|namespace| namespace.types.get(name))
            .and_then(|v| v.first())
            .map(|(_, item)| *item)
    }
}
