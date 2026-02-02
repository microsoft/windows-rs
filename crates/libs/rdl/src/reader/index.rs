use super::*;

type Map<'a> = BTreeMap<String, BTreeMap<String, (&'a str, &'a syntax::Item)>>;

pub struct Item<'a> {
    pub name: &'a str,
    pub item: &'a syntax::Item,
    pub winrt: bool,
}

#[derive(Default)]
pub struct Index<'a>(Map<'a>);

impl<'a> Index<'a> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, source_file: &'a str, namespace: &str, item: &'a syntax::Item) {
        if let syntax::Item::Module(module) = item {
            for item in &module.items {
                let namespace = if namespace.is_empty() {
                    module.to_string()
                } else {
                    format!("{namespace}.{}", module)
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
}

impl<'a> core::ops::Deref for Index<'a> {
    type Target = Map<'a>;

    fn deref(&self) -> &Map<'a> {
        &self.0
    }
}

impl<'a> core::ops::DerefMut for Index<'a> {
    fn deref_mut(&mut self) -> &mut Map<'a> {
        &mut self.0
    }
}
