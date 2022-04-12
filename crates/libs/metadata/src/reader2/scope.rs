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
        Blob::new( file, self.files[file].blob(key.row as _, key.table as _, column))
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

    pub fn type_from_code(&self, code: TypeDefOrRef, enclosing: Option<&TypeDef>, generics: &'a [Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.signature(self);
            return self.type_from_blob_impl(&mut blob, None, generics);
        }

        let mut full_name = code.type_name(self);

        for (known_name, kind) in WELL_KNOWN_TYPES {
            if full_name == known_name {
                return kind;
            }
        }

        for (from, to) in REMAP_TYPES {
            if full_name == from {
                full_name = to;
                break;
            }
        }

        if let Some(outer) = enclosing {
            Type::TypeDef((self.get_nested(outer, full_name.name), Vec::new()))
        } else {
            Type::TypeDef((self.get(&full_name).next().expect("Type not found"), Vec::new()))
        }
    }

    pub fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<&TypeDef>, generics: &'a [Type]) -> Option<Type> {
        let is_winrt_const_ref = blob.read_modifiers().iter().any(|def| def.type_name(self) == TypeName::IsConst);

        let is_winrt_array_ref = blob.read_expected(0x10);

        if blob.read_expected(0x01) {
            return None;
        }

        let is_winrt_array = blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        let mut kind = self.type_from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            kind = Type::MutPtr((Box::new(kind), pointers));
        }

        Some(if is_winrt_array {
            if is_winrt_array_ref {
                Type::WinrtArrayRef(Box::new(kind))
            } else {
                Type::WinrtArray(Box::new(kind))
            }
        } else if is_winrt_const_ref {
            Type::WinrtConstRef(Box::new(kind))
        } else {
            kind
        })
    }

    fn type_from_blob_impl(&self, blob: &mut Blob, enclosing: Option<&TypeDef>, generics: &'a [Type]) -> Type {
        let code = blob.read_usize();

        if let Some(code) = Type::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => self.type_from_code(TypeDefOrRef::decode(blob.file, blob.read_usize()), enclosing, generics),
            0x13 => generics.get(blob.read_usize() as usize).unwrap_or(&Type::Void).clone(),
            0x14 => {
                let kind = self.type_from_blob(blob, enclosing, generics).unwrap();
                let _rank = blob.read_usize();
                let _bounds_count = blob.read_usize();
                let bounds = blob.read_usize();
                Type::Win32Array((Box::new(kind), bounds))
            }
            0x15 => {
                blob.read_usize();

                let mut def = self.get(&TypeDefOrRef::decode( blob.file, blob.read_usize()).type_name(self)).next().expect("Type not found");
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef((def, args))
            }
            _ => unimplemented!(),
        }
    }
}

const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

const WELL_KNOWN_TYPES: [(TypeName, Type); 10] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::IInspectable, Type::IInspectable), (TypeName::LARGE_INTEGER, Type::I64), (TypeName::ULARGE_INTEGER, Type::U64), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR)];
