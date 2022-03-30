use super::*;

// TODO: now that we just have a large batch code generator, should we just load up all 
// type information in memory and avoid hitting the files repeatedly? Could be faster...

pub struct Scope<'a> {
    files: &'a [File],
    // TODO: when inserting, need to ensure that all parent namespaces exist
    types: BTreeMap<&'a str, BTreeMap<&'a str, Vec<Type<'a>>>>,
    // Nested type values must be sorted by name.
    nested: BTreeMap<TypeDef<'a>, Vec<TypeDef<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = BTreeMap::new();
        let mut nested = BTreeMap::new();

        for file in files {
            
        }

        Self { files, types, nested }
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &&str> {
        self.types.keys()
    }

    pub fn nested_namespaces(&self, parent: &'a str) -> impl Iterator<Item = &&str> {
        self.types.range(parent..).take_while(move |(namespace, _)| namespace.starts_with(parent)).filter_map(|(namespace, _)| namespace.as_bytes().get(parent.len()).and_then(|_| Some(namespace)))
    }

    pub fn namespace_types(&self, namespace: &str) ->  impl Iterator<Item = &Type> {
        self.types[namespace].values().flatten()
    }

    pub fn nested_types(&self, type_def: &'a TypeDef) ->  impl Iterator<Item = &TypeDef> {
        self.nested[type_def].iter()
    }

    pub fn get_type(&self, name: &TypeName) -> impl Iterator<Item = &Type> {
        if let Some(types) = self.types.get(name.namespace).and_then(|types|types.get(name.name)) {
            types.iter()
        } else {
            [].iter()
        }
    }
}
