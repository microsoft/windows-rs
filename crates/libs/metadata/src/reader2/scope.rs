use super::*;

pub struct Scope<'a> {
    files: &'a [File],
    types: HashMap<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>,
    nested: HashMap<TypeDef, BTreeMap<&'a str, TypeDef>>,
}

impl<'a> Scope<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = HashMap::<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>::new();
        let mut nested = HashMap::<TypeDef, BTreeMap<&'a str, TypeDef>>::new();
        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = ScopeKey::new(row, TABLE_TYPEDEF, file_index);
                let namespace = file.str(key.row as _, key.table as _, 2);
                let name = file.str(key.row as _, key.table as _, 1);
                types.entry(namespace).or_default().entry(name).or_default().push(TypeDef(key));
            }
            for row in 0..file.tables[TABLE_NESTEDCLASS].len {
                let key = ScopeKey::new(row, TABLE_NESTEDCLASS, file_index);
                let inner = ScopeKey::new(file.usize(key.row as _, key.table as _, 0) - 1, TABLE_TYPEDEF, file_index);
                let outer = ScopeKey::new(file.usize(key.row as _, key.table as _, 1) - 1, TABLE_TYPEDEF, file_index);
                let name = file.str(inner.row as _, inner.table as _, 1);
                nested.entry(TypeDef(outer)).or_default().insert(name, TypeDef(inner));
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

    pub fn namespace_types(&self, namespace: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.types[namespace].values().flatten().map(|ty| *ty)
    }

    pub fn nested_types(&self, type_def: &TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested[type_def].values().map(|ty| *ty)
    }

    pub fn get(&self, type_name: &TypeName) -> impl Iterator<Item = TypeDef> + '_ {
        self.types[type_name.namespace][type_name.name].iter().map(|ty| *ty)
    }

    pub fn get_nested(&self, outer: &TypeDef, name: &str) -> TypeDef {
        self.nested[&outer][name]
    }

    pub fn usize(&self, key: ScopeKey, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }

    pub fn str(&self, key: ScopeKey, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }

    pub fn blob(&'a self, key: ScopeKey, column: usize) -> Blob<'a> {
        let file = key.file as usize;
        Blob::new(self, file, self.files[file].blob(key.row as _, key.table as _, column))
    }

    pub fn equal_range(&self, key: ScopeKey, table: usize, column: usize, value: usize) -> impl Iterator<Item = ScopeKey> {
        let (first, last) = self.files[key.file as usize].equal_range(table, column, value);
        (first..last).map(move |row| ScopeKey::new(row, table, key.file as _))
    }

    pub fn attributes(&self, key: ScopeKey, source: HasAttribute) -> impl Iterator<Item = Attribute> {
        self.equal_range(key, TABLE_CUSTOMATTRIBUTE, 0, source.encode()).map(Attribute)
    }

    pub fn list(&self, key: ScopeKey, table: usize, column: usize) -> impl Iterator<Item = ScopeKey> {
        let file = key.file as usize;
        let first = self.usize(key, column) - 1;
        let last = if key.row + 1 < self.files[file].tables[key.table as usize].len as _ { self.usize(key.next(), column) - 1 } else { self.files[file].tables[table].len };
        (first..last).map(move |row| ScopeKey::new(row, table, file))
    }

    pub fn decode<T: Decode>(&self, key: ScopeKey, column: usize) -> T {
        T::decode(key.file as _, self.usize(key, column))
    }
}
