use super::*;

pub struct Reader<'a> {
    files: &'a [File],
    types: HashMap<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>,
    nested: HashMap<TypeDef, BTreeMap<&'a str, TypeDef>>,
}

impl<'a> Reader<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = HashMap::<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>::new();
        let mut nested = HashMap::<TypeDef, BTreeMap<&'a str, TypeDef>>::new();
        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = Row::new(row, TABLE_TYPEDEF, file_index);
                let namespace = file.str(key.row as _, key.table as _, 2);
                let name = file.str(key.row as _, key.table as _, 1);
                types.entry(namespace).or_default().entry(name).or_default().push(TypeDef(key));
            }
            for row in 0..file.tables[TABLE_NESTEDCLASS].len {
                let key = Row::new(row, TABLE_NESTEDCLASS, file_index);
                let inner = Row::new(file.usize(key.row as _, key.table as _, 0) - 1, TABLE_TYPEDEF, file_index);
                let outer = Row::new(file.usize(key.row as _, key.table as _, 1) - 1, TABLE_TYPEDEF, file_index);
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
    pub fn nested_types(&self, type_def: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested[&type_def].values().map(|ty| *ty)
    }
    pub fn get(&self, type_name: TypeName) -> impl Iterator<Item = TypeDef> + '_ {
        self.types[type_name.namespace][type_name.name].iter().map(|ty| *ty)
    }
    pub fn get_nested(&self, outer: TypeDef, name: &str) -> TypeDef {
        self.nested[&outer][name]
    }
    pub fn usize(&self, key: Row, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }
    pub fn str(&self, key: Row, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }
    pub fn blob(&self, key: Row, column: usize) -> Blob {
        let file = key.file as usize;
        Blob::new( file, self.files[file].blob(key.row as _, key.table as _, column))
    }
    pub fn equal_range(&self, key: Row, table: usize, column: usize, value: usize) -> impl Iterator<Item = Row> {
        let (first, last) = self.files[key.file as usize].equal_range(table, column, value);
        (first..last).map(move |row| Row::new(row, table, key.file as _))
    }
    pub fn attributes(&self, key: Row, source: HasAttribute) -> impl Iterator<Item = Attribute> {
        self.equal_range(key, TABLE_CUSTOMATTRIBUTE, 0, source.encode()).map(Attribute)
    }
    pub fn list(&self, key: Row, table: usize, column: usize) -> impl Iterator<Item = Row> {
        let file = key.file as usize;
        let first = self.usize(key, column) - 1;
        let last = if key.row + 1 < self.files[file].tables[key.table as usize].len as _ { self.usize(key.next(), column) - 1 } else { self.files[file].tables[table].len };
        (first..last).map(move |row| Row::new(row, table, file))
    }
    pub fn decode<T: Decode>(&self, key: Row, column: usize) -> T {
        T::decode(key.file as _, self.usize(key, column))
    }

    pub fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.str(row.0, 2), self.str(row.0, 1)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.str(row.0, 2), self.str(row.0, 1)),
            _ => unimplemented!(),
        }
    }

    // pub fn attribute_name(&self, row: Attribute) -> &str {
    //     let AttributeType::MemberRef(member) = sel.decode(row, 1);
    //     let MemberRefParent::TypeRef(type_ref) = self.decode(member, 0);
    //     type_ref.name(scope)
    // }
    // pub fn attribute_args(&self, row: Attribute) -> Vec<(&str, Value)> {
    //     let AttributeType::MemberRef(member) = scope.decode(self.0, 1);
    //     let mut sig = member.signature(scope);
    //     let mut values = scope.blob(self.0, 2);
    //     let _prolog = values.read_u16();
    //     let _this_and_gen_param_count = sig.read_usize();
    //     let fixed_arg_count = sig.read_usize();
    //     let _ret_type = sig.read_usize();
    //     let mut args: Vec<(String, Value)> = Vec::with_capacity(fixed_arg_count as usize);

    //     for _ in 0..fixed_arg_count {
    //         let arg = match scope.type_from_blob(&mut sig, None, &[]).expect("Type not found") {
    //             Type::I8 => Value::I8(values.read_i8()),
    //             Type::U8 => Value::U8(values.read_u8()),
    //             Type::I16 => Value::I16(values.read_i16()),
    //             Type::U16 => Value::U16(values.read_u16()),
    //             Type::I32 => Value::I32(values.read_i32()),
    //             Type::U32 => Value::U32(values.read_u32()),
    //             Type::I64 => Value::I64(values.read_i64()),
    //             Type::U64 => Value::U64(values.read_u64()),
    //             Type::String => Value::String(values.read_str().to_string()),
    //             Type::TypeName => Value::TypeDef(scope.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
    //             Type::TypeDef((def, _)) => match def.underlying_type(scope) {
    //                 Type::I8 => Value::I8(values.read_i8()),
    //                 Type::U8 => Value::U8(values.read_u8()),
    //                 Type::I16 => Value::I16(values.read_i16()),
    //                 Type::U16 => Value::U16(values.read_u16()),
    //                 Type::I32 => Value::I32(values.read_i32()),
    //                 Type::U32 => Value::U32(values.read_u32()),
    //                 Type::I64 => Value::I64(values.read_i64()),
    //                 Type::U64 => Value::U64(values.read_u64()),
    //                 _ => unimplemented!(),
    //             },
    //             _ => unimplemented!(),
    //         };

    //         args.push((String::new(), arg));
    //     }
            
    //     let named_arg_count = values.read_u16();
    //     args.reserve(named_arg_count as usize);

    //     for _ in 0..named_arg_count {
    //         let _id = values.read_u8();
    //         let arg_type = values.read_u8();
    //         let name = values.read_str().to_string();
    //         let arg = match arg_type {
    //             0x02 => Value::Bool(values.read_u8() != 0),
    //             0x06 => Value::I16(values.read_i16()),
    //             0x08 => Value::I32(values.read_i32()),
    //             0x09 => Value::U32(values.read_u32()),
    //             0x0E => Value::String(values.read_str().to_string()),
    //             0x50 => Value::TypeDef(scope.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
    //             _ => unimplemented!(),
    //         };
    //         args.push((name, arg));
    //     }
        
    //     args
    // }

    pub fn class_layout_packing_size(&self, row: ClassLayout) -> usize {
        self.usize(row.0, 0)
    }

    pub fn constant_type(&self, row: Constant) -> Type {
        let code = self.usize(row.0, 0);
        Type::from_code(code).expect("Type not found")
    }
    pub fn constant_value(&self, row: Constant) -> Value {
       let mut blob = self.blob(row.0, 2);
        match self.constant_type(row) {
            Type::I8 => Value::I8(blob.read_i8()),
            Type::U8 => Value::U8(blob.read_u8()),
            Type::I16 => Value::I16(blob.read_i16()),
            Type::U16 => Value::U16(blob.read_u16()),
            Type::I32 => Value::I32(blob.read_i32()),
            Type::U32 => Value::U32(blob.read_u32()),
            Type::I64 => Value::I64(blob.read_i64()),
            Type::U64 => Value::U64(blob.read_u64()),
            Type::F32 => Value::F32(blob.read_f32()),
            Type::F64 => Value::F64(blob.read_f64()),
            _ => unimplemented!(),
        }
    }


    pub fn type_ref_name(&self, row: TypeRef) -> &str {
        self.str(row.0, 1)
    }
    pub fn type_ref_namespace(&self, row: TypeRef) -> &str {
        self.str(row.0, 2)
    }
    pub fn type_ref_type_name(&self, row: TypeRef) -> TypeName {
        TypeName::new(self.type_ref_name(row), self.type_ref_namespace(row))
    }

    // pub fn type_from_code(&self, code: TypeDefOrRef, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
    //     if let TypeDefOrRef::TypeSpec(def) = code {
    //         let mut blob = def.signature(self);
    //         return self.type_from_blob_impl(&mut blob, None, generics);
    //     }

    //     let mut full_name = self.type_def_or_ref(code);

    //     for (known_name, kind) in WELL_KNOWN_TYPES {
    //         if full_name == known_name {
    //             return kind;
    //         }
    //     }

    //     for (from, to) in REMAP_TYPES {
    //         if full_name == from {
    //             full_name = to;
    //             break;
    //         }
    //     }

    //     if let Some(outer) = enclosing {
    //         Type::TypeDef((self.get_nested(outer, full_name.name), Vec::new()))
    //     } else {
    //         Type::TypeDef((self.get(full_name).next().expect("Type not found"), Vec::new()))
    //     }
    // }
    // pub fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Option<Type> {
    //     let is_winrt_const_ref = blob.read_modifiers().iter().any(|def| self.type_def_or_ref(def) == TypeName::IsConst);
    //     let is_winrt_array_ref = blob.read_expected(0x10);
    //     if blob.read_expected(0x01) {
    //         return None;
    //     }

    //     let is_winrt_array = blob.read_expected(0x1D);

    //     let mut pointers = 0;

    //     while blob.read_expected(0x0f) {
    //         pointers += 1;
    //     }

    //     let mut kind = self.type_from_blob_impl(blob, enclosing, generics);

    //     if pointers > 0 {
    //         kind = Type::MutPtr((Box::new(kind), pointers));
    //     }

    //     Some(if is_winrt_array {
    //         if is_winrt_array_ref {
    //             Type::WinrtArrayRef(Box::new(kind))
    //         } else {
    //             Type::WinrtArray(Box::new(kind))
    //         }
    //     } else if is_winrt_const_ref {
    //         Type::WinrtConstRef(Box::new(kind))
    //     } else {
    //         kind
    //     })
    // }
    // fn type_from_blob_impl(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
    //     let code = blob.read_usize();

    //     if let Some(code) = Type::from_code(code) {
    //         return code;
    //     }

    //     match code {
    //         0x11 | 0x12 => self.type_from_code(TypeDefOrRef::decode(blob.file, blob.read_usize()), enclosing, generics),
    //         0x13 => generics.get(blob.read_usize() as usize).unwrap_or(&Type::Void).clone(),
    //         0x14 => {
    //             let kind = self.type_from_blob(blob, enclosing, generics).unwrap();
    //             let _rank = blob.read_usize();
    //             let _bounds_count = blob.read_usize();
    //             let bounds = blob.read_usize();
    //             Type::Win32Array((Box::new(kind), bounds))
    //         }
    //         0x15 => {
    //             blob.read_usize();

    //             let def = self.get(self.type_def_or_ref(TypeDefOrRef::decode( blob.file, blob.read_usize())).type_name(self)).next().expect("Type not found");
    //             let mut args = Vec::with_capacity(blob.read_usize());

    //             for _ in 0..args.capacity() {
    //                 args.push(self.type_from_blob_impl(blob, enclosing, generics));
    //             }

    //             Type::TypeDef((def, args))
    //         }
    //         _ => unimplemented!(),
    //     }
    // }
}

const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

const WELL_KNOWN_TYPES: [(TypeName, Type); 10] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::IInspectable, Type::IInspectable), (TypeName::LARGE_INTEGER, Type::I64), (TypeName::ULARGE_INTEGER, Type::U64), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR)];
