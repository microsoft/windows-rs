use super::*;

type Map<'a> = BTreeMap<String, BTreeMap<String, (&'a str, syntax::Item)>>;

#[derive(Default)]
pub struct Index<'a>(Map<'a>);

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, source_file: &'a str, namespace: &str, item:  syntax::Item) {
        if let syntax::Item::Module(module) = item {
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
            self.0
                .entry(namespace.to_string())
                .or_default()
                .insert(item.to_string(), (source_file, item));
        }
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.0
            .get(namespace)
            .and_then(|types| types.get(name))
            .is_some()
    }

    pub fn items(&self) -> impl Iterator<Item = (&str, &str, &str, &syntax::Item)> + '_ {
        self.0
            .iter()
            .flat_map(|(namespace, items)| {
                items
                    .iter()
                    .map(move |(name, items)| (namespace, name, items))
            })
            .map(|(namespace, name, (source_file, item))| {
                 (namespace.as_str(), name.as_str(), *source_file, item)
            })
    }
}
