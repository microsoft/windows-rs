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

    fn row_usize(&self, key: Row, column: usize) -> usize {
        self.files[key.file as usize].usize(key.row as _, key.table as _, column)
    }
    fn row_str(&self, key: Row, column: usize) -> &str {
        self.files[key.file as usize].str(key.row as _, key.table as _, column)
    }
    fn row_blob(&self, key: Row, column: usize) -> Blob {
        let file = key.file as usize;
        Blob::new(file, self.files[file].blob(key.row as _, key.table as _, column))
    }
    fn row_equal_range(&self, key: Row, table: usize, column: usize, value: usize) -> impl Iterator<Item = Row> {
        let (first, last) = self.files[key.file as usize].equal_range(table, column, value);
        (first..last).map(move |row| Row::new(row, table, key.file as _))
    }
    fn row_attributes(&self, key: Row, source: HasAttribute) -> impl Iterator<Item = Attribute> {
        self.row_equal_range(key, TABLE_CUSTOMATTRIBUTE, 0, source.encode()).map(Attribute)
    }
    fn row_list(&self, key: Row, table: usize, column: usize) -> impl Iterator<Item = Row> {
        let file = key.file as usize;
        let first = self.row_usize(key, column) - 1;
        let last = if key.row + 1 < self.files[file].tables[key.table as usize].len as _ { self.row_usize(key.next(), column) - 1 } else { self.files[file].tables[table].len };
        (first..last).map(move |row| Row::new(row, table, file))
    }
    fn row_decode<T: Decode>(&self, key: Row, column: usize) -> T {
        T::decode(key.file as _, self.row_usize(key, column))
    }

    pub fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.type_def_namespace(row), self.type_def_name(row)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row)),
            _ => unimplemented!(),
        }
    }

    pub fn attribute_name(&self, row: Attribute) -> &str {
        let AttributeType::MemberRef(row) = self.row_decode(row.0, 1);
        let MemberRefParent::TypeRef(row) = self.row_decode(row.0, 0);
        self.type_ref_name(row)
    }

    pub fn class_layout_packing_size(&self, row: ClassLayout) -> usize {
        self.row_usize(row.0, 0)
    }

    pub fn constant_type(&self, row: Constant) -> Type {
        let code = self.row_usize(row.0, 0);
        Type::from_code(code).expect("Type not found")
    }
    pub fn constant_value(&self, row: Constant) -> Value {
        let mut blob = self.row_blob(row.0, 2);
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

    pub fn field_flags(&self, row: Field) -> FieldAttributes {
        FieldAttributes(self.row_usize(row.0, 0))
    }
    pub fn field_name(&self, row: Field) -> &str {
        self.row_str(row.0, 1)
    }
    pub fn field_constant(&self, row: Field) -> Option<Constant> {
        self.row_equal_range(row.0, TABLE_CONSTANT, 1, HasConstant::Field(row).encode()).map(Constant).next()
    }
    pub fn field_attributes(&self, row: Field) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::Field(row))
    }
    //     pub fn field_type(&self, scope: &Reader, enclosing: Option<TypeDef>) -> Type {
    //         let mut blob = scope.blob(self.0, 2);
    //         blob.read_usize();
    //         blob.read_modifiers();
    //         let def = scope.type_from_blob(&mut blob, enclosing, &[]).expect("Type not found");

    //         if self.is_const(scope) {
    //             def.to_const()
    //         } else {
    //             def
    //         }
    //     }
    pub fn field_is_const(&self, row: Field) -> bool {
        self.field_attributes(row).any(|attribute| self.attribute_name(attribute) == "ConstAttribute")
    }

    pub fn generic_param_name(&self, row: GenericParam) -> &str {
        self.row_str(row.0, 3)
    }

    pub fn impl_map_flags(&self, row: ImplMap) -> PInvokeAttributes {
        PInvokeAttributes(self.row_usize(row.0, 0))
    }
    pub fn impl_map_scope(&self, row: ImplMap) -> ModuleRef {
        ModuleRef(Row::new(self.row_usize(row.0, 3) - 1, TABLE_MODULEREF, row.0.file as _))
    }

    pub fn interface_impl_type(&self, row: InterfaceImpl) -> TypeDefOrRef {
        self.row_decode(row.0, 1)
    }
    pub fn interface_impl_attributes(&self, row: InterfaceImpl) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::InterfaceImpl(row))
    }

    pub fn member_ref_parent(&self, row: MemberRef) -> MemberRefParent {
        self.row_decode(row.0, 0)
    }
    pub fn member_ref_signature(&self, row: MemberRef) -> Blob {
        self.row_blob(row.0, 2)
    }

    pub fn method_def_impl_flags(&self, row: MethodDef) -> MethodImplAttributes {
        MethodImplAttributes(self.row_usize(row.0, 0))
    }
    pub fn method_def_flags(&self, row: MethodDef) -> MethodAttributes {
        MethodAttributes(self.row_usize(row.0, 1))
    }
    pub fn method_def_name(&self, row: MethodDef) -> &str {
        self.row_str(row.0, 3)
    }
    pub fn method_def_signature(&self, row: MethodDef) -> Blob {
        self.row_blob(row.0, 4)
    }
    pub fn method_def_params(&self, row: MethodDef) -> impl Iterator<Item = Param> {
        self.row_list(row.0, TABLE_PARAM, 4).map(Param)
    }

    pub fn module_ref_name(&self, row: ModuleRef) -> &str {
        self.row_str(row.0, 0)
    }

    pub fn param_flags(&self, row: Param) -> ParamAttributes {
        ParamAttributes(self.row_usize(row.0, 0))
    }
    pub fn param_sequence(&self, row: Param) -> usize {
        self.row_usize(row.0, 1)
    }
    pub fn param_name(&self, row: Param) -> &str {
        self.row_str(row.0, 2)
    }

    pub fn type_def_flags(&self, row: TypeDef) -> TypeAttributes {
        TypeAttributes(self.row_usize(row.0, 0))
    }
    pub fn type_def_name(&self, row: TypeDef) -> &str {
        self.row_str(row.0, 1)
    }
    pub fn type_def_namespace(&self, row: TypeDef) -> &str {
        self.row_str(row.0, 2)
    }
    pub fn type_def_type_name(&self, row: TypeDef) -> TypeName {
        TypeName::new(self.type_def_namespace(row), self.type_def_namespace(row))
    }
    pub fn type_def_extends(&self, row: TypeDef) -> TypeDefOrRef {
        self.row_decode(row.0, 3)
    }
    pub fn type_def_fields(&self, row: TypeDef) -> impl Iterator<Item = Field> {
        self.row_list(row.0, TABLE_FIELD, 4).map(Field)
    }
    pub fn type_def_methods(&self, row: TypeDef) -> impl Iterator<Item = MethodDef> {
        self.row_list(row.0, TABLE_METHODDEF, 5).map(MethodDef)
    }
    pub fn type_def_attributes(&self, row: TypeDef) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::TypeDef(row))
    }
    pub fn type_def_generic_params(&self, row: TypeDef) -> impl Iterator<Item = GenericParam> {
        self.row_equal_range(row.0, TABLE_GENERICPARAM, 2, TypeOrMethodDef::TypeDef(row).encode()).map(GenericParam)
    }
    pub fn type_def_interface_impls(&self, row: TypeDef) -> impl Iterator<Item = InterfaceImpl> {
        self.row_equal_range(row.0, TABLE_INTERFACEIMPL, 0, (row.0.row + 1) as _).map(InterfaceImpl)
    }
    // pub fn type_def_underlying_type(&self, row:TypeDef) -> Type {
    //     if let Some(field) = self.type_def_fields(row).next() {
    //         if let Some(constant) = field.constant(scope) {
    //             return constant.ty(scope);
    //         } else {
    //             return field.ty(scope, Some(*self));
    //         }
    //     }

    //     unimplemented!();
    // }

    pub fn type_ref_name(&self, row: TypeRef) -> &str {
        self.row_str(row.0, 1)
    }
    pub fn type_ref_namespace(&self, row: TypeRef) -> &str {
        self.row_str(row.0, 2)
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
