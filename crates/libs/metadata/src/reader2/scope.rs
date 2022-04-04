use super::*;

// TODO: Scope should be as simple as possible and only serve one function which
// is to index the TypeDefs - not worry about other indexes like constants and functions
// or do any other wrapping or help.

pub struct Scope<'a> {
    files: &'a [File],
    types: HashMap<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>,
    nested: HashMap<ScopeKey, BTreeMap<&'a str, ScopeKey>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = HashMap::<&'a str, BTreeMap<&'a str, Vec<ScopeKey>>>::new();
        let mut nested = HashMap::new();

        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = ScopeKey { row: row as _, table: TABLE_TYPEDEF as _, file: file_index as _ };
                let namespace = file.str(key.row as _, key.table as _, 2);
                let name = file.str(key.row as _, key.table as _, 1);
                types.entry(namespace).or_default().entry(name).or_default().push(key);
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

    pub fn nested_types(&self, type_def: &'a TypeDef) -> impl Iterator<Item = TypeDef> {
        self.nested[&type_def.0.key].values().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn get_type(&self, name: &TypeName) -> impl Iterator<Item = TypeDef> {
        let types: &[ScopeKey] = if let Some(types) = self.types.get(name.namespace).and_then(|types| types.get(name.name)) { types } else { &[] };

        types.iter().map(|key| TypeDef(Row::new(self, *key), Vec::new()))
    }

    pub fn usize(&self, key: &ScopeKey, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }

    pub fn str(&self, key: &ScopeKey, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }
}
