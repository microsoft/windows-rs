use super::*;

pub struct Scope<'a> {
    files: &'a [File],
    types: HashMap<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>,
    nested: HashMap<ScopeKey, BTreeMap<&'a str, ScopeKey>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = HashMap::<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>::new();
        let mut nested = HashMap::<ScopeKey, BTreeMap<&'a str, ScopeKey>>::new();

        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = ScopeKey::new(row, TABLE_TYPEDEF, file_index);
                let namespace = file.str(key.row as _, key.table as _, 2);
                let name = file.str(key.row as _, key.table as _, 1);
                types.entry(namespace).or_default().entry(name).or_default().push(key);
            }
            for row in 0..file.tables[TABLE_NESTEDCLASS].len {
                let key = ScopeKey::new(row, TABLE_NESTEDCLASS, file_index);
                let inner = ScopeKey::new(file.usize(key.row as _, key.table as _, 0) - 1, TABLE_TYPEDEF, file_index);
                let outer = ScopeKey::new(file.usize(key.row as _, key.table as _, 1) - 1, TABLE_TYPEDEF, file_index);
                let name = file.str(inner.row as _, inner.table as _, 1);
                nested.entry(outer).or_default().insert(name, inner);
            }
        }

        Self { files, types, nested }
    }

    pub fn tree(&self) -> Tree {
        let mut tree = Tree::from_namespace("");

        for ns in self.types.keys() {
            if !ns.is_empty() {
                tree.insert_namespace(ns, 0);
            }
        }

        tree
    }

    pub fn namespace_types(&self, namespace: &str) -> impl Iterator<Item = TypeDef> {
        self.types[namespace].values().flatten().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn nested_types(&self, type_def: &TypeDef) -> impl Iterator<Item = TypeDef> {
        self.nested[&type_def.0.key].values().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn resolve(&self, name: &TypeName) -> impl Iterator<Item = TypeDef> {
        let types: &[ScopeKey] = if let Some(types) = self.types.get(name.namespace).and_then(|types| types.get(name.name)) { types } else { &[] };

        types.iter().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn usize(&self, key: &ScopeKey, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }

    pub fn str(&self, key: &ScopeKey, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }

    pub fn blob(&self, key: &ScopeKey, column: usize) -> Blob {
        let file = key.file as usize;
        Blob::new(self, file, self.files[file].blob(key.row as _, key.table as _, column))
    }

    pub fn equal_range(&self, file: usize, table: usize, column: usize, value: usize) -> impl Iterator<Item = Row> {
        let (first, last) = self.files[file].equal_range(table, column, value);
        (first..last).map(move |row| Row::new(self, ScopeKey::new(row, table, file)))
    }

    pub fn list(&self, key: &ScopeKey, table: usize, column: usize) -> impl Iterator<Item = Row> {
        let file = key.file as usize;
        let first = self.usize(key, column) - 1;
        let last = if key.row + 1 < self.files[file].tables[key.table as usize].len as _ { self.usize(&key.next(), column) - 1 } else { self.files[file].tables[table].len };
        (first..last).map(move |row| Row::new(self, ScopeKey::new(row, table, file)))
    }

    pub fn decode<T: Decode<'a>>(&'a self, key: &ScopeKey, column: usize) -> T {
        T::decode(self, key.file as _, self.usize(key, column))
    }
}
