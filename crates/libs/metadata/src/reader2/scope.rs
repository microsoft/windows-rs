use super::*;

pub struct Scope<'a> {
    files: &'a [File],
    // TODO: when inserting, need to ensure that all parent namespaces exist
    types: BTreeMap<&'a str, BTreeMap<&'a str, Key>>,
    // Nested type values must be sorted by name.
    nested: BTreeMap<Key, Vec<Key>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        Self { files, types: BTreeMap::new(), nested: BTreeMap::new() }
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &&str> {
        self.types.keys()
    }

    pub fn nested_namespaces(&self, parent: &'a str) -> impl Iterator<Item = &&str> {
        self.types.range(parent..).take_while(move |(namespace, _)| namespace.starts_with(parent)).filter_map(|(namespace, _)| namespace.as_bytes().get(parent.len()).and_then(|_| Some(namespace)))
    }
}

pub(crate) struct Row<'a> {
    scope: &'a Scope<'a>,
    key: Key,
}

struct Key {
    pub row: u32,
    pub table: u32,
    pub file: u32,
}
