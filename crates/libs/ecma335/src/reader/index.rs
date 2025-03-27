use super::*;

pub struct Index<'a> {
    types: HashMap<&'a str, HashMap<&'a str, Vec<TypeDef<'a>>>>,
    nested: HashMap<TypeDef<'a>, Vec<TypeDef<'a>>>,
}

impl<'a> Index<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types: HashMap<&str, HashMap<&str, Vec<TypeDef>>> = HashMap::new();
        let mut nested: HashMap<TypeDef, Vec<TypeDef>> = HashMap::new();

        for file in files {
            for def in file.TypeDef() {
                let namespace = def.namespace();

                if namespace.is_empty() {
                    // Skips `<Module>` as well as nested types.
                    continue;
                }

                types
                    .entry(namespace)
                    .or_default()
                    .entry(def.name())
                    .or_default()
                    .push(def);
            }

            for map in file.NestedClass() {
                let outer = map.outer();
                let inner = map.inner();
                nested.entry(outer).or_default().push(inner);
            }
        }

        Self { types, nested }
    }

    pub fn all(&self) -> impl Iterator<Item = TypeDef> + '_ {
        self.types
            .values()
            .flat_map(|types| types.values())
            .flatten()
            .cloned()
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.types
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

    pub fn nested(&self, ty: TypeDef<'a>) -> impl Iterator<Item = TypeDef<'a>> + '_ {
        self.nested
            .get(&ty)
            .map(|nested| nested.iter())
            .unwrap_or_else(|| [].iter())
            .cloned()
    }
}
