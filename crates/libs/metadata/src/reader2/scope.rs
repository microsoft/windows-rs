use super::*;

// TODO: Scope should be as simple as possible and only serve one function which
// is to index the TypeDefs - not worry about other indexes like constants and functions
// or do any other wrapping or help.

pub struct Scope<'a> {
    files: &'a [File],
    // TODO: when inserting, need to ensure that all parent namespaces exist
    types: BTreeMap<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>,
    // Nested type values must be sorted by name.
    nested: BTreeMap<ScopeKey, Vec<ScopeKey>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = BTreeMap::<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>::new();
        let mut nested = BTreeMap::new();

        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = ScopeKey { row: row as _, table: TABLE_TYPEDEF as _, file: file_index as _ };

                let name = file.str(key.row as _, key.table as _, 1);
                let namespace = file.str(key.row as _, key.table as _, 2);
                types.entry(namespace).or_default().entry(name).or_default().push(key);
            }
        }

        Self { files, types, nested }
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &&str> {
        self.types.keys()
    }

    pub fn nested_namespaces(&self, parent: &'a str) -> impl Iterator<Item = &&str> {
        self.types.range(parent..).take_while(move |(namespace, _)| namespace.starts_with(parent)).filter_map(|(namespace, _)| namespace.as_bytes().get(parent.len()).map(|_| namespace))
    }

    pub fn namespace_types(&self, namespace: &str) -> impl Iterator<Item = TypeDef> {
        self.types[namespace].values().flatten().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn nested_types(&self, type_def: &'a TypeDef) -> impl Iterator<Item = TypeDef> {
        self.nested[&type_def.0.key].iter().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn get_type(&self, name: &TypeName) -> impl Iterator<Item = TypeDef> {
        let types: &[ScopeKey] = if let Some(types) = self.types.get(name.namespace).and_then(|types| types.get(name.name)) { types } else { &[] };

        types.iter().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn raw_types(&self) -> impl Iterator<Item = TypeDef> {
        self.files.iter().enumerate().flat_map(move |(file_id, file)| (0..file.tables[TABLE_TYPEDEF].len).map(move |row| TypeDef(Row::new(self, ScopeKey { row: row as _, table: TABLE_TYPEDEF as _, file: file_id as _ }), Vec::new())))
    }

    pub fn usize(&self, key: &ScopeKey, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }

    pub fn str(&self, key: &ScopeKey, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }
}
