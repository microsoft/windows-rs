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

    //
    // Hash functions for fast type lookup
    //

    pub fn namespace_types(&self, namespace: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.types[namespace].values().flatten().copied()
    }
    pub fn nested_types(&self, type_def: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested[&type_def].values().copied()
    }
    pub fn get(&self, type_name: TypeName) -> impl Iterator<Item = TypeDef> + '_ {
        self.types[type_name.namespace][type_name.name].iter().copied()
    }
    pub fn get_nested(&self, outer: TypeDef, name: &str) -> TypeDef {
        self.nested[&outer][name]
    }

    //
    // Row functions providing low-level file access
    //

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

    //
    // Attribute table queries
    //

    pub fn attribute_name(&self, row: Attribute) -> &str {
        let AttributeType::MemberRef(row) = self.row_decode(row.0, 1);
        let MemberRefParent::TypeRef(row) = self.row_decode(row.0, 0);
        self.type_ref_name(row)
    }
    pub fn attribute_args(&self, row: Attribute) -> Vec<(String, Value)> {
        let AttributeType::MemberRef(member) = self.row_decode(row.0, 1);
        let mut sig = self.member_ref_signature(member);
        let mut values = self.row_blob(row.0, 2);
        let _prolog = values.read_u16();
        let _this_and_gen_param_count = sig.read_usize();
        let fixed_arg_count = sig.read_usize();
        let _ret_type = sig.read_usize();
        let mut args: Vec<(String, Value)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
            let arg = match self.type_from_blob(&mut sig, None, &[]).expect("Type not found") {
                Type::I8 => Value::I8(values.read_i8()),
                Type::U8 => Value::U8(values.read_u8()),
                Type::I16 => Value::I16(values.read_i16()),
                Type::U16 => Value::U16(values.read_u16()),
                Type::I32 => Value::I32(values.read_i32()),
                Type::U32 => Value::U32(values.read_u32()),
                Type::I64 => Value::I64(values.read_i64()),
                Type::U64 => Value::U64(values.read_u64()),
                Type::String => Value::String(values.read_str().to_string()),
                Type::TypeName => Value::TypeDef(self.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
                Type::TypeDef((def, _)) => match self.type_def_underlying_type(def) {
                    Type::I8 => Value::I8(values.read_i8()),
                    Type::U8 => Value::U8(values.read_u8()),
                    Type::I16 => Value::I16(values.read_i16()),
                    Type::U16 => Value::U16(values.read_u16()),
                    Type::I32 => Value::I32(values.read_i32()),
                    Type::U32 => Value::U32(values.read_u32()),
                    Type::I64 => Value::I64(values.read_i64()),
                    Type::U64 => Value::U64(values.read_u64()),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            };

            args.push((String::new(), arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let name = values.read_str().to_string();
            let arg = match arg_type {
                0x02 => Value::Bool(values.read_u8() != 0),
                0x06 => Value::I16(values.read_i16()),
                0x08 => Value::I32(values.read_i32()),
                0x09 => Value::U32(values.read_u32()),
                0x0E => Value::String(values.read_str().to_string()),
                0x50 => Value::TypeDef(self.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
                _ => unimplemented!(),
            };
            args.push((name, arg));
        }

        args
    }

    //
    // ClassLayout table queries
    //

    pub fn class_layout_packing_size(&self, row: ClassLayout) -> usize {
        self.row_usize(row.0, 0)
    }

    //
    // Constant table queries
    //

    pub fn constant_type(&self, row: Constant) -> Type {
        let code = self.row_usize(row.0, 0);
        type_from_code(code).expect("Type not found")
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

    //
    // Field table queries
    //

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
    pub fn field_is_const(&self, row: Field) -> bool {
        self.field_attributes(row).any(|attribute| self.attribute_name(attribute) == "ConstAttribute")
    }
    pub fn field_type(&self, row: Field, enclosing: Option<TypeDef>) -> Type {
        let mut blob = self.row_blob(row.0, 2);
        blob.read_usize();
        blob.read_modifiers();
        let def = self.type_from_blob(&mut blob, enclosing, &[]).expect("Type not found");

        if self.field_is_const(row) {
            type_to_const(def)
        } else {
            def
        }
    }
    pub fn field_is_blittable(&self, row: Field, enclosing: TypeDef) -> bool {
        self.type_is_blittable(&self.field_type(row, Some(enclosing)))
    }
    fn field_cfg(&self, row:Field) -> Cfg {
        let mut cfg = Cfg::default();
        self.field_cfg_combine(row, None, &mut cfg);
        self.cfg_add_attributes(&mut cfg, self.field_attributes(row));
        cfg
    }
    fn field_cfg_combine(&self, row:Field, enclosing: Option<TypeDef>, cfg: &mut Cfg) {
        self.type_cfg_combine(&self.field_type(row, enclosing), cfg)
    }

    //
    // GenericParam table queries
    //

    pub fn generic_param_name(&self, row: GenericParam) -> &str {
        self.row_str(row.0, 3)
    }

    //
    // ImplMap table queries
    //

    pub fn impl_map_flags(&self, row: ImplMap) -> PInvokeAttributes {
        PInvokeAttributes(self.row_usize(row.0, 0))
    }
    pub fn impl_map_scope(&self, row: ImplMap) -> ModuleRef {
        ModuleRef(Row::new(self.row_usize(row.0, 3) - 1, TABLE_MODULEREF, row.0.file as _))
    }

    //
    // InterfaceImpl table queries
    //

    pub fn interface_impl_attributes(&self, row: InterfaceImpl) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::InterfaceImpl(row))
    }
    pub fn interface_impl_is_default(&self, row: InterfaceImpl) -> bool {
        self.interface_impl_attributes(row).any(|attribute| self.attribute_name(attribute) == "DefaultAttribute")
    }
    pub fn interface_impl_is_overridable(&self, row: InterfaceImpl) -> bool {
        self.interface_impl_attributes(row).any(|attribute| self.attribute_name(attribute) == "OverridableAttribute")
    }
    pub fn interface_impl_type(&self, row: InterfaceImpl, generics: &[Type]) -> Interface {
        let mut kind = InterfaceKind::None;
        for attribute in self.interface_impl_attributes(row) {
            match self.attribute_name(attribute) {
                "DefaultAttribute" => kind = InterfaceKind::Default,
                "OverridableAttribute" => kind = InterfaceKind::Overridable,
                _ => {}
            }
        }
        Interface { ty: self.type_from_ref(self.row_decode(row.0, 1), None, generics), kind }
    }

    //
    // MemberRef table queries
    //

    pub fn member_ref_parent(&self, row: MemberRef) -> MemberRefParent {
        self.row_decode(row.0, 0)
    }
    pub fn member_ref_signature(&self, row: MemberRef) -> Blob {
        self.row_blob(row.0, 2)
    }

    //
    // MethodDef table queries
    //

    pub fn method_def_impl_flags(&self, row: MethodDef) -> MethodImplAttributes {
        MethodImplAttributes(self.row_usize(row.0, 1))
    }
    pub fn method_def_flags(&self, row: MethodDef) -> MethodAttributes {
        MethodAttributes(self.row_usize(row.0, 2))
    }
    pub fn method_def_name(&self, row: MethodDef) -> &str {
        self.row_str(row.0, 3)
    }
    pub fn method_def_params(&self, row: MethodDef) -> impl Iterator<Item = Param> {
        self.row_list(row.0, TABLE_PARAM, 4).map(Param)
    }
    pub fn method_def_attributes(&self, row: MethodDef) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::MethodDef(row))
    }
    pub fn method_def_is_deprecated(&self, row: MethodDef) -> bool {
        self.method_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "DeprecatedAttribute")
    }
    pub fn method_def_does_not_return(&self, row: MethodDef) -> bool {
        self.method_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "DoesNotReturnAttribute")
    }
    pub fn method_def_special_name(&self, row: MethodDef) -> String {
        let name = self.method_def_name(row);
        if self.method_def_flags(row).special() {
            if name.starts_with("get") {
                name[4..].to_string()
            } else if name.starts_with("put") {
                format!("Set{}", &name[4..])
            } else if name.starts_with("add") {
                name[4..].to_string()
            } else if name.starts_with("remove") {
                format!("Remove{}", &name[7..])
            } else {
                "name".to_string()
            }
        } else {
            for attribute in self.method_def_attributes(row) {
                if self.attribute_name(attribute) == "OverloadAttribute" {
                    for (_, arg) in self.attribute_args(attribute) {
                        if let Value::String(name) = arg {
                            return name;
                        }
                    }
                }
            }
            name.to_string()
        }
    }
    pub fn method_def_static_lib(&self, row: MethodDef) -> Option<String> {
        for attribute in self.method_def_attributes(row) {
            if self.attribute_name(attribute) == "StaticLibraryAttribute" {
                let args = self.attribute_args(attribute);
                if let Value::String(value) = &args[0].1 {
                    return Some(value.clone());
                }
            }
        }
        None
    }
    pub fn method_def_impl_map(&self, row: MethodDef) -> Option<ImplMap> {
        self.row_equal_range(row.0, TABLE_IMPLMAP, 1, MemberForwarded::MethodDef(row).encode()).map(ImplMap).next()
    }
    pub fn method_def_signature(&self, row: MethodDef, generics: &[Type]) -> Signature {
        let mut blob = self.row_blob(row.0, 4);
        blob.read_usize();
        blob.read_usize();

        let return_type = self.type_from_blob(&mut blob, None, generics);
        let mut params: Vec<SignatureParam> = self.method_def_params(row).filter_map(|param| {
            if self.param_sequence(param) == 0 {
                None
            } else {
                let ty = self.type_from_blob(&mut blob, None, generics).expect("Parameter type not found");
                let ty = if self.param_flags(param).output() { type_to_const(ty) } else { ty };
                let array_info = self.param_array_info(param);
                Some(SignatureParam{ def: param, ty, array_info })
            }
        }).collect();

        for position in 0..params.len() {
            // Point len params back to the corresponding ptr params.
            if let Some(ArrayInfo::RelativeLen(relative)) = params[position].array_info {
                // The len params must be input only.
                // TODO: workaround for https://github.com/microsoft/win32metadata/issues/813
                if !self.param_flags(params[relative].def).output() && position != relative {
                    params[relative].array_info = Some(ArrayInfo::RelativePtr(position));
                } else {
                    params[position].array_info = None;
                }
            }
        }

        let mut sets = BTreeMap::<usize, Vec<usize>>::new();

        // Finds sets of ptr params pointing at the same len param.
        for (position, param) in params.iter().enumerate() {
            if let Some(ArrayInfo::RelativeLen(relative)) = param.array_info {
                sets.entry(relative).or_default().push(position);
            }
        }

        // Remove all sets.
        for (len, ptrs) in sets {
            if ptrs.len() > 1 {
                params[len].array_info = None;
                for ptr in ptrs {
                    params[ptr].array_info = None;
                }
            }
        }

        Signature{ def:row, params, return_type }
    }
    fn method_def_cfg(&self, row:MethodDef) -> Cfg {
        let mut cfg = Cfg::default();
        self.method_def_cfg_combine(row, &mut cfg);
        cfg
    }
    // TODO: maybe inline this at the callsite to avoid recalculating the method signature
    fn method_def_cfg_combine(&self, row:MethodDef, cfg: &mut Cfg) {
        self.signature_cfg_combine(&self.method_def_signature(row, &[]), cfg);
    }

    //
    // ModuleRef table queries
    //

    pub fn module_ref_name(&self, row: ModuleRef) -> &str {
        self.row_str(row.0, 0)
    }

    //
    // Param table queries
    //

    pub fn param_flags(&self, row: Param) -> ParamAttributes {
        ParamAttributes(self.row_usize(row.0, 0))
    }
    pub fn param_sequence(&self, row: Param) -> usize {
        self.row_usize(row.0, 1)
    }
    pub fn param_name(&self, row: Param) -> &str {
        self.row_str(row.0, 2)
    }
    pub fn param_attributes(&self, row: Param) -> impl Iterator<Item = Attribute> {
        self.row_attributes(row.0, HasAttribute::Param(row))
    }
    pub fn param_is_com_out_ptr(&self, row: Param) -> bool {
        self.param_attributes(row).any(|attribute| self.attribute_name(attribute) == "ComOutPtrAttribute")
    }
    pub fn param_array_info(&self, row: Param) -> Option<ArrayInfo> {
        for attribute in self.param_attributes(row) {
            if self.attribute_name(attribute) == "NativeArrayInfoAttribute" {
                for (_, value) in self.attribute_args(attribute) {
                    match value {
                        Value::I16(value) => return Some(ArrayInfo::RelativeLen(value as _)),
                        Value::I32(value) => return Some(ArrayInfo::Fixed(value as _)),
                        _ => {}
                    }
                }
            }
        }
        None
    }
    pub fn param_is_retval(&self, row: Param) -> bool {
        self.param_attributes(row).any(|attribute| self.attribute_name(attribute) == "RetValAttribute")
    }
    pub fn param_free_with(&self, row: Param) -> Option<String> {
        for attribute in self.param_attributes(row) {
            if self.attribute_name(attribute) == "FreeWithAttribute" {
                for (_, arg) in self.attribute_args(attribute) {
                    if let Value::String(name) = arg {
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    //
    // TypeDef table queries
    //

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
    pub fn type_def_extends(&self, row: TypeDef) -> TypeName {
        self.type_def_or_ref(self.row_decode(row.0, 3))
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
    pub fn type_def_class_layout(&self, row: TypeDef) -> Option<ClassLayout> {
        self.row_equal_range(row.0, TABLE_CLASSLAYOUT, 2, (row.0.row + 1) as _).map(ClassLayout).next()
    }
    pub fn type_def_underlying_type(&self, row: TypeDef) -> Type {
        let field = self.type_def_fields(row).next().expect("Field not found");
        if let Some(constant) = self.field_constant(field) {
            self.constant_type(constant)
        } else {
            self.field_type(field, Some(row))
        }
    }
    pub fn type_def_kind(&self, row: TypeDef) -> TypeKind {
        if self.type_def_flags(row).interface() {
            TypeKind::Interface
        } else {
            match self.type_def_extends(row) {
                TypeName::Enum => TypeKind::Enum,
                TypeName::Delegate => TypeKind::Delegate,
                TypeName::Struct => TypeKind::Struct,
                _ => TypeKind::Class,
            }
        }
    }
    pub fn type_def_stdcall(&self, row: TypeDef) -> usize {
        if self.type_def_kind(row) == TypeKind::Struct {
            if self.type_def_flags(row).union() {
                self.type_def_fields(row).map(|field| self.type_stdcall(&self.field_type(field, Some(row)))).max().unwrap_or(1)
            } else {
                self.type_def_fields(row).fold(0, |sum, field| sum + self.type_stdcall(&self.field_type(field, Some(row))))
            }
        } else {
            4
        }
    }
    pub fn type_def_is_blittable(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Struct => {
                if self.type_def_type_name(row) == TypeName::BSTR {
                    false
                } else {
                    self.type_def_fields(row).all(|field| self.field_is_blittable(field, row))
                }
            }
            TypeKind::Enum => true,
            TypeKind::Delegate => !self.type_def_flags(row).winrt(),
            _ => false,
        }
    }
    pub fn type_def_is_callback(&self, row: TypeDef) -> bool {
        !self.type_def_flags(row).winrt() && self.type_def_kind(row) == TypeKind::Delegate
    }
    pub fn type_def_has_default_constructor(&self, row: TypeDef) -> bool {
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "ActivatableAttribute" {
                if self.attribute_args(attribute).iter().any(|arg| matches!(arg.1, Value::TypeDef(_))) {
                    continue;
                } else {
                    return true;
                }
            }
        }
        false
    }
    pub fn type_def_invoke_method(&self, row: TypeDef) -> MethodDef {
        self.type_def_methods(row).find(|method| self.method_def_name(*method) == "Invoke").expect("`Invoke` method not found")
    }
    pub fn type_def_interfaces(&'a self, row: TypeDef, generics: &'a [Type]) -> impl Iterator<Item = Interface> + '_ {
        self.type_def_interface_impls(row).map(|row| self.interface_impl_type(row, generics))
    }
    pub fn type_def_is_deprecated(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "DeprecatedAttribute")
    }
    pub fn type_def_is_handle(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "NativeTypedefAttribute")
    }
    pub fn type_def_is_exclusive(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ExclusiveToAttribute")
    }
    pub fn type_def_is_scoped(&self, row: TypeDef) -> bool {
        self.type_def_flags(row).winrt() || self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ScopedEnumAttribute")
    }
    pub fn type_def_is_contract(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ApiContractAttribute")
    }
    pub fn type_def_is_udt(&self, row: TypeDef) -> bool {
        // TODO: should this just check whether the struct has > 1 fields rather than type_def_is_handle?
        self.type_def_kind(row) == TypeKind::Struct && !self.type_def_is_handle(row)
    }
    pub fn type_def_is_convertible(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Interface | TypeKind::Class | TypeKind::Struct => true,
            TypeKind::Delegate => self.type_def_flags(row).winrt(),
            _ => false,
        }
    }
    pub fn type_def_is_primitive(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Enum => true,
            TypeKind::Struct => self.type_def_is_handle(row) && self.type_def_type_name(row) != TypeName::BSTR,
            _ => false,
        }
    }
    pub fn type_def_has_union(&self, row: TypeDef) -> bool {
        if self.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        for row in self.get(self.type_def_type_name(row)) {
            if self.type_def_flags(row).union() {
                return true;
            }
            if self.type_def_fields(row).any(|field| self.type_has_union(&self.field_type(field, Some(row)))) {
                return true;
            }
        }
        false
    }
    pub fn type_def_has_packing(&self, row: TypeDef) -> bool {
        if self.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        for row in self.get(self.type_def_type_name(row)) {
            if self.type_def_class_layout(row).is_some() {
                return true;
            }
            if self.type_def_fields(row).any(|field| self.type_has_packing(&self.field_type(field, Some(row)))) {
                return true;
            }
        }
        false
    }
    pub fn type_def_guid(&self, row: TypeDef) -> Option<GUID> {
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "GuidAttribute" {
                return Some(GUID::from_args(&self.attribute_args(attribute)));
            }
        }
        None
    }
    pub fn type_def_bases(&self, mut row: TypeDef) -> Vec<TypeDef> {
        // TODO: maybe return Vec<Type>
        let mut bases = Vec::new();
        loop {
            let extends = self.type_def_extends(row);
            if extends == TypeName::Object {
                break;
            } else {
                row = self.get(extends).next().expect("Type not found");
                bases.push(row);
            }
        }
        bases
    }
    pub fn type_def_is_flags(&self, row: TypeDef) -> bool {
        // Win32 enums use the Flags attribute. WinRT enums don't have the Flags attribute but are paritioned merely based
        // on whether they are signed.
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "FlagsAttribute") || (self.type_def_flags(row).winrt() && self.type_def_underlying_type(row) == Type::U32)
    }
    pub fn type_def_is_agile(&self, row: TypeDef) -> bool {
        // TODO: add new Agile attribute from Win32 metadata
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "MarshalingBehaviorAttribute" {
                if let Some((_, Value::I32(2))) = self.attribute_args(attribute).get(0) {
                    return true;
                }
            }
        }
        false
    }
    pub fn type_def_invalid_values(&self, row: TypeDef) -> Vec<i64> {
        let mut values = Vec::new();
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "InvalidHandleValueAttribute" {
                if let Some((_, Value::I64(value))) = self.attribute_args(attribute).get(0) {
                    values.push(*value);
                }
            }
        }
        values
    }
    pub fn type_def_usable_for(&self, row: TypeDef) -> Option<TypeDef> {
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "AlsoUsableForAttribute" {
                if let Some((_, Value::String(name))) = self.attribute_args(attribute).get(0) {
                    return self.get(TypeName::new(self.type_def_namespace(row), name.as_str())).next();
                }
            }
        }
        None
    }
    pub fn type_def_is_nullable(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Interface | TypeKind::Class => true,
            // TODO: win32 callbacks should be nullable...
            TypeKind::Delegate => self.type_def_flags(row).winrt(),
            _ => false,
        }
    }
    pub fn type_def_can_implement(&self, row: TypeDef) -> bool {
        for attribute in self.type_def_attributes(row) {
            if self.attribute_name(attribute) == "ExclusiveToAttribute" {
                for (_, arg) in self.attribute_args(attribute) {
                    if let Value::TypeDef(def) = arg {
                        for child in self.type_def_interfaces(def, &[]) {
                            if child.kind == InterfaceKind::Overridable {
                                if let Type::TypeDef((def, _)) = child.ty {
                                    if self.type_def_type_name(def) == self.type_def_type_name(row) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
                return false;
            }
        }
        true
    }
    pub fn type_def_async_kind(&self, row: TypeDef) -> AsyncKind {
        match self.type_def_type_name(row) {
            TypeName::IAsyncAction => AsyncKind::Action,
            TypeName::IAsyncActionWithProgress => AsyncKind::ActionWithProgress,
            TypeName::IAsyncOperation => AsyncKind::Operation,
            TypeName::IAsyncOperationWithProgress => AsyncKind::OperationWithProgress,
            _ => AsyncKind::None,
        }
    }
    pub fn type_def_signature(&self, row: TypeDef, generics: &[Type]) -> String {
        match self.type_def_kind(row) {
            TypeKind::Interface => self.type_def_interface_signature(row, generics),
            TypeKind::Class => {
                if let Type::TypeDef((row, generics)) = self.type_def_interfaces(row, generics).find(|row| row.kind == InterfaceKind::Default).expect("Default interface not found").ty {
                    format!("rc({};{})", self.type_def_type_name(row), self.type_def_interface_signature(row, &generics))
                } else {
                    unimplemented!();
                }
            }
            TypeKind::Enum => format!("enum({};{})", self.type_def_type_name(row), self.type_signature(&self.type_def_underlying_type(row))),
            TypeKind::Struct => {
                let mut result = format!("struct({}", self.type_def_type_name(row));
                for field in self.type_def_fields(row) {
                    result.push(';');
                    result.push_str(&self.type_signature(&self.field_type(field, Some(row))));
                }
                result.push(')');
                result
            }
            TypeKind::Delegate => {
                if generics.is_empty() {
                    format!("delegate({})", self.type_def_interface_signature(row, generics))
                } else {
                    self.type_def_interface_signature(row, generics)
                }
            }
            _ => unimplemented!(),
        }
    }
    fn type_def_interface_signature(&self, row: TypeDef, generics: &[Type]) -> String {
        let guid = self.type_def_guid(row);
        if generics.is_empty() {
            format!("{{{:#?}}}", guid)
        } else {
            let mut result = format!("pinterface({{{:#?}}}", guid);
            for generic in generics {
                result.push(';');
                result.push_str(&self.type_signature(generic));
            }
            result.push(')');
            result
        }
    }
    fn type_def_cfg(&self, row:TypeDef, generics:&[Type]) -> Cfg {
        let mut cfg = Cfg::default();
        self.type_def_cfg_combine(row, generics, &mut cfg);
        self.cfg_add_attributes(&mut cfg, self.type_def_attributes(row));
        cfg
    }
    fn type_def_cfg_impl(&self, row:TypeDef, generics:&[Type]) -> Cfg {
        let mut cfg = Cfg::default();
        cfg
    }
    fn type_def_cfg_combine(&self, row:TypeDef, generics:&[Type], cfg: &mut Cfg) {
        // for generic in &self.generics {
        //     self.type_cfg_combine(generic, cfg);
        // }

        // if cfg.types.entry(self.type_def_namespace(row).or_default().insert(row) {
        //     match self.type_def_kind(row) {
        //         TypeKind::Class => {
        //             if let Some(Type::TypeDef(row, _)) = self.type_def_interfaces(row, generics).find(|row| row.kind == InterfaceKind::Default).filter(|interface|interface.ty) {
        //                 cfg.types.entry(self.type_def_namespace(row).or_default());
        //             }
        //         }
        //         TypeKind::Interface => {
        //             if !self.type_def_flags(row).winrt() {
        //                 for def in self.vtable_types() {
        //                     if let Type::TypeDef(def) = def {
        //                         cfg.add_feature(def.namespace());
        //                     }
        //                 }
        //             }
        //         }
        //         TypeKind::Struct => {
        //             self.fields().for_each(|field| field.combine_cfg(Some(self), cfg));

        //             if let Some(entry) = TypeReader::get().get_type_entry(self.type_name()) {
        //                 for def in entry {
        //                     if let Type::TypeDef(def) = def {
        //                         def.combine_cfg(cfg);
        //                     }
        //                 }
        //             }
        //         }
        //         TypeKind::Delegate => self.invoke_method().combine_cfg(cfg),
        //         _ => {}
        //     }
        // }
    }
    pub fn type_def_vtables(&self, row: TypeDef) -> Vec<Type> {
        let mut result = Vec::new();
        if self.type_def_flags(row).winrt() {
            result.push(Type::IUnknown);
            if self.type_def_kind(row) != TypeKind::Delegate {
                result.push(Type::IInspectable);
            }
        } else {
            let mut next = row;
            while let Some(base) = self.type_def_interfaces(next, &[]).next() {
                match base.ty {
                    Type::TypeDef((row, _)) => {
                        next = row;
                        result.insert(0, base.ty);
                    }
                    Type::IInspectable => {
                        result.insert(0, Type::IUnknown);
                        result.insert(1, Type::IInspectable);
                        break;
                    }
                    Type::IUnknown => {
                        result.insert(0, Type::IUnknown);
                        break;
                    }
                    _ => unimplemented!(),
                }
            }
        }
        result
    }

    //
    // TypeRef table queries
    //

    pub fn type_ref_name(&self, row: TypeRef) -> &str {
        self.row_str(row.0, 1)
    }
    pub fn type_ref_namespace(&self, row: TypeRef) -> &str {
        self.row_str(row.0, 2)
    }
    pub fn type_ref_type_name(&self, row: TypeRef) -> TypeName {
        TypeName::new(self.type_ref_name(row), self.type_ref_namespace(row))
    }

    //
    // TypeSpec table queries
    //

    pub fn type_spec_signature(&self, row: TypeSpec) -> Blob {
        self.row_blob(row.0, 0)
    }

    //
    // Other type queries
    //

    fn cfg_add_attributes(&self, cfg: &mut Cfg, attributes: impl Iterator<Item = Attribute>) {
        for attribute in attributes {
            match self.attribute_name(attribute) {
                "SupportedArchitectureAttribute" => {
                    if let Some((_, Value::I32(value))) = self.attribute_args(attribute).get(0) {
                        if value & 1 == 1 {
                            cfg.arches.insert("x86");
                        }
                        if value & 2 == 2 {
                            cfg.arches.insert("x86_64");
                        }
                        if value & 4 == 4 {
                            cfg.arches.insert("aarch64");
                        }
                    }
                }
                "DeprecatedAttribute" => {
                    cfg.types.entry("deprecated").or_default();
                }
                _ => {}
            }
        }
    }
    fn signature_cfg_combine(&self, signature: &Signature, cfg: &mut Cfg) {
        signature.return_type.iter().for_each(|ty| self.type_cfg_combine(ty, cfg));
        signature.params.iter().for_each(|param| self.type_cfg_combine(&param.ty, cfg));
    }
    fn type_cfg(&self, ty:&Type) -> Cfg {
        let mut cfg = Cfg::default();
        self.type_cfg_combine(ty, &mut cfg);
        cfg
    }
    fn type_cfg_combine(&self, ty:&Type, cfg: &mut Cfg) {
        match ty {
            Type::MethodDef(row) => self.method_def_cfg_combine(*row, cfg),
            Type::Field(row) => self.field_cfg_combine(*row, None,cfg),
            Type::TypeDef((row, generics)) => self.type_def_cfg_combine(*row, generics, cfg),
            Type::Win32Array((ty, _)) => self.type_cfg_combine(ty, cfg),
            Type::ConstPtr((ty, _)) => self.type_cfg_combine(ty, cfg),
            Type::MutPtr((ty, _)) => self.type_cfg_combine(ty, cfg),
            Type::WinrtArray(ty) => self.type_cfg_combine(ty, cfg),
            Type::WinrtArrayRef(ty) => self.type_cfg_combine(ty, cfg),
            _ => {}
        }
    }
    fn type_interfaces(&self, ty: &Type) -> Vec<Interface> {
        fn walk(reader: &Reader, result: &mut Vec<Interface>, parent: &Type, is_base: bool) {
            if let Type::TypeDef((row, generics)) = parent {
                for mut child in reader.type_def_interfaces(*row, generics) {
                    child.kind = if !is_base && child.kind == InterfaceKind::Default {
                        InterfaceKind::Default
                    } else if child.kind == InterfaceKind::Overridable {
                        continue;
                    } else if is_base {
                        InterfaceKind::Base
                    } else {
                        InterfaceKind::None
                    };
                    let mut found = false;
                    for existing in result.iter_mut() {
                        if existing.ty == child.ty {
                            found = true;
                            if child.kind == InterfaceKind::Default {
                                existing.kind = child.kind
                            }
                        }
                    }
                    if !found {
                        walk(reader, result, &child.ty, is_base);
                        result.push(child);
                    }
                }
            }
        }
        let mut result = Vec::new();
        walk(self, &mut result, ty, false);
        if let Type::TypeDef((row, _)) = ty {
            if self.type_def_kind(*row) == TypeKind::Class {
                for base in self.type_def_bases(*row) {
                    walk(self, &mut result, &Type::TypeDef((base, Vec::new())), true);
                }
                for attribute in self.type_def_attributes(*row) {
                    match self.attribute_name(attribute) {
                        "StaticAttribute" | "ActivatableAttribute" => {
                            for (_, arg) in self.attribute_args(attribute) {
                                if let Value::TypeDef(row) = arg {
                                    result.push(Interface { ty: Type::TypeDef((row, Vec::new())), kind: InterfaceKind::Static });
                                    break;
                                }
                            }
                        }
                        "ComposableAttribute" => {
                            let mut public = false;
                            let mut def = None;
                            for (_, arg) in self.attribute_args(attribute) {
                                match arg {
                                    Value::I32(2) => public = true,
                                    Value::TypeDef(row) => def = Some(row),
                                    _ => {}
                                }
                            }
                            if public {
                                if let Some(row) = def {
                                    result.push(Interface { ty: Type::TypeDef((row, Vec::new())), kind: InterfaceKind::Composable });
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        result
    }
    fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.type_def_namespace(row), self.type_def_name(row)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row)),
            _ => unimplemented!(),
        }
    }
    fn type_stdcall(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 16,
            Type::TypeDef((row, _)) => self.type_def_stdcall(*row),
            _ => 4,
        }
    }
    fn type_is_blittable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_blittable(*row),
            Type::String | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
            Type::Win32Array((kind, _)) => self.type_is_blittable(kind),
            _ => true,
        }
    }
    pub fn type_has_union(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_has_union(*row),
            Type::Win32Array((ty, _)) => self.type_has_union(ty),
            _ => false,
        }
    }
    pub fn type_has_packing(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_has_packing(*row),
            Type::Win32Array((ty, _)) => self.type_has_packing(ty),
            _ => false,
        }
    }
    fn type_from_ref(&self, code: TypeDefOrRef, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = self.type_spec_signature(def);
            return self.type_from_blob_impl(&mut blob, None, generics);
        }

        let mut full_name = self.type_def_or_ref(code);

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
            Type::TypeDef((self.get(full_name).next().expect("Type not found"), Vec::new()))
        }
    }
    fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Option<Type> {
        let is_winrt_const_ref = blob.read_modifiers().iter().any(|def| self.type_def_or_ref(*def) == TypeName::IsConst);
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
    fn type_from_blob_impl(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        let code = blob.read_usize();

        if let Some(code) = type_from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => self.type_from_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()), enclosing, generics),
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

                let def = self.get(self.type_def_or_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()))).next().expect("Type not found");
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef((def, args))
            }
            _ => unimplemented!(),
        }
    }
    pub fn type_signature(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "b1".to_string(),
            Type::Char => "c2".to_string(),
            Type::I8 => "i1".to_string(),
            Type::U8 => "u1".to_string(),
            Type::I16 => "i2".to_string(),
            Type::U16 => "u2".to_string(),
            Type::I32 => "i4".to_string(),
            Type::U32 => "u4".to_string(),
            Type::I64 => "i8".to_string(),
            Type::U64 => "u8".to_string(),
            Type::F32 => "f4".to_string(),
            Type::F64 => "f8".to_string(),
            Type::String => "string".to_string(),
            Type::IInspectable => "cinterface(IInspectable)".to_string(),
            Type::GUID => "g16".to_string(),
            Type::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_string(),
            Type::TypeDef((row, generics)) => self.type_def_signature(*row, generics),
            _ => unimplemented!(),
        }
    }
    pub fn type_is_nullable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_nullable(*row),
            Type::IInspectable | Type::IUnknown | Type::MethodDef(_) => true,
            _ => false,
        }
    }
    pub fn type_is_convertible(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_convertible(*row),
            Type::String | Type::IInspectable | Type::GUID | Type::IUnknown | Type::GenericParam(_) | Type::PCSTR | Type::PCWSTR => true,
            Type::WinrtConstRef(ty) => self.type_is_convertible(ty),
            _ => false,
        }
    }
    pub fn type_is_callback(&self, ty: &Type) -> bool {
        match ty {
            // TODO: do we need to know there's a callback behind the pointer?
            Type::ConstPtr((ty, _)) | Type::MutPtr((ty, _)) => self.type_is_callback(ty),
            Type::TypeDef((row, _)) => self.type_def_is_callback(*row),
            _ => false,
        }
    }
    pub fn type_is_callback_array(&self, ty: &Type) -> bool {
        match ty {
            Type::Win32Array((ty, _)) => self.type_is_callback(ty),
            _ => false,
        }
    }
    pub fn type_is_primitive(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_primitive(*row),
            Type::Bool | Type::Char | Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::F32 | Type::F64 | Type::ISize | Type::USize | Type::HRESULT | Type::ConstPtr(_) | Type::MutPtr(_) => true,
            _ => false,
        }
    }
    pub fn type_is_udt(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_udt(*row),
            Type::GUID => true,
            _ => false,
        }
    }
    pub fn type_is_handle(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_is_handle(*row),
            _ => false,
        }
    }
    pub fn type_underlying_type(&self, ty: &Type) -> Type {
        match ty {
            Type::TypeDef((row, _)) => self.type_def_underlying_type(*row),
            Type::HRESULT => Type::I32,
            _ => ty.clone(),
        }
    }
    pub fn type_has_replacement(&self, ty: &Type) -> bool {
        match ty {
            Type::HRESULT | Type::PCSTR | Type::PCWSTR => true,
            Type::TypeDef((row, _)) => self.type_def_is_handle(*row),
            _ => false,
        }
    }
}

pub fn type_is_generic(ty: &Type) -> bool {
    matches!(ty, Type::GenericParam(_))
}
pub fn type_is_pointer(ty: &Type) -> bool {
    matches!(ty, Type::ConstPtr(_) | Type::MutPtr(_))
}
pub fn type_is_unsigned(ty: &Type) -> bool {
    matches!(ty, Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::USize)
}
pub fn type_is_void(ty: &Type) -> bool {
    match ty {
        // TODO: do we care about void behind pointers?
        Type::ConstPtr((kind, _)) | Type::MutPtr((kind, _)) => type_is_void(kind),
        Type::Void => true,
        _ => false,
    }
}
pub fn is_winrt_array(ty: &Type) -> bool {
    matches!(ty, Type::WinrtArray(_))
}

pub fn is_winrt_array_ref(ty: &Type) -> bool {
    matches!(ty, Type::WinrtArrayRef(_))
}

pub fn is_winrt_const_ref(ty: &Type) -> bool {
    matches!(ty, Type::WinrtConstRef(_))
}
fn type_from_code(code: usize) -> Option<Type> {
    match code {
        0x01 => Some(Type::Void),
        0x02 => Some(Type::Bool),
        0x03 => Some(Type::Char),
        0x04 => Some(Type::I8),
        0x05 => Some(Type::U8),
        0x06 => Some(Type::I16),
        0x07 => Some(Type::U16),
        0x08 => Some(Type::I32),
        0x09 => Some(Type::U32),
        0x0a => Some(Type::I64),
        0x0b => Some(Type::U64),
        0x0c => Some(Type::F32),
        0x0d => Some(Type::F64),
        0x18 => Some(Type::ISize),
        0x19 => Some(Type::USize),
        0x0e => Some(Type::String),
        0x1c => Some(Type::IInspectable),
        _ => None,
    }
}
fn type_to_const(ty: Type) -> Type {
    match ty {
        Type::MutPtr((kind, pointers)) => Type::ConstPtr((kind, pointers)),
        Type::PSTR => Type::PCSTR,
        Type::PWSTR => Type::PCWSTR,
        _ => ty,
    }
}
pub fn type_deref(ty: &Type) -> Type {
    match ty {
        Type::ConstPtr((kind, 1)) | Type::MutPtr((kind, 1)) => {
            if **kind == Type::Void {
                Type::U8
            } else {
                *kind.clone()
            }
        }
        Type::ConstPtr((kind, pointers)) => Type::ConstPtr((kind.clone(), pointers - 1)),
        Type::MutPtr((kind, pointers)) => Type::MutPtr((kind.clone(), pointers - 1)),
        Type::PSTR | Type::PCSTR => Type::U8,
        Type::PWSTR | Type::PCWSTR => Type::U16,
        _ => unimplemented!(),
    }
}

const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

const WELL_KNOWN_TYPES: [(TypeName, Type); 10] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::IInspectable, Type::IInspectable), (TypeName::LARGE_INTEGER, Type::I64), (TypeName::ULARGE_INTEGER, Type::U64), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR)];
