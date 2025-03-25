use super::*;

pub struct Cache<'a>(HashMap<&'a str, HashMap<&'a str, Vec<TypeDef<'a>>>>);

impl<'a> Cache<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut map: HashMap<&str, HashMap<&str, Vec<TypeDef>>> = HashMap::new();

        for file in files {
            // TODO: gather nested types

            for def in file.TypeDef() {
                if def.flags().is_nested() {
                    continue;
                }

                map.entry(def.namespace())
                    .or_default()
                    .entry(trim_tick(def.name()))
                    .or_default()
                    .push(def);
            }
        }

        Self(map)
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.0
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .cloned()
    }

    pub fn expect(&self, namespace: &str, name: &str) -> TypeDef {
        let mut iter = self.get(namespace, name);

        if let Some(def) = iter.next() {
            if iter.next().is_none() {
                def
            } else {
                panic!("more than one type found: {namespace}.{name}");
            }
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }
}
