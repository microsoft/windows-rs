use super::*;

pub trait RowReader<'a> {
    fn row_file<R: AsRow>(&self, row: R) -> &'a File;

    fn row_usize<R: AsRow>(&self, row: R, column: usize) -> usize {
        let file = self.row_file(row);
        let row = row.to_row();
        file.usize(row.row, R::TABLE, column)
    }

    fn row_str<R: AsRow>(&self, row: R, column: usize) -> &'a str {
        let file = self.row_file(row);
        let offset = file.strings + self.row_usize(row, column);
        let bytes = &file.bytes[offset..];
        let nul_pos = bytes.iter().position(|&c| c == 0).expect("expected null-terminated C-string");
        std::str::from_utf8(&bytes[..nul_pos]).expect("expected valid utf-8 C-string")
    }

    fn row_blob<R: AsRow>(&self, row: R, column: usize) -> Blob<'a> {
        let file = self.row_file(row);
        let offset = file.blobs + self.row_usize(row, column);
        let initial_byte = file.bytes[offset];

        let (blob_size, blob_size_bytes) = match initial_byte >> 5 {
            0..=3 => (initial_byte & 0x7f, 1),
            4..=5 => (initial_byte & 0x3f, 2),
            6 => (initial_byte & 0x1f, 4),
            rest => unimplemented!("{rest:?}"),
        };

        let mut blob_size = blob_size as usize;

        for byte in &file.bytes[offset + 1..offset + blob_size_bytes] {
            blob_size = blob_size.checked_shl(8).unwrap_or(0) + (*byte as usize);
        }

        let offset = offset + blob_size_bytes;
        Blob::new(row.file(), &file.bytes[offset..offset + blob_size])
    }

    fn row_list<R: AsRow, L: AsRow>(&self, row: R, column: usize) -> RowIterator<L> {
        let file = self.row_file(row);
        let first = self.row_usize(row, column) - 1;
        let next = row.next();
        let last = if next.index() < file.tables[R::TABLE].len { self.row_usize(next, column) - 1 } else { file.tables[L::TABLE].len };
        RowIterator::new(row.file(), first..last)
    }

    fn row_equal_range<R: AsRow, L: AsRow>(&self, row: R, column: usize, value: usize) -> RowIterator<L> {
        let file = self.row_file(row);
        let mut first = 0;
        let mut last = file.tables[L::TABLE].len;
        let mut count = last;

        loop {
            if count == 0 {
                last = first;
                break;
            }

            let count2 = count / 2;
            let middle = first + count2;
            let middle_value = file.usize(middle, L::TABLE, column);

            match middle_value.cmp(&value) {
                Ordering::Less => {
                    first = middle + 1;
                    count -= count2 + 1;
                }
                Ordering::Greater => count = count2,
                Ordering::Equal => {
                    let first2 = file.lower_bound_of(L::TABLE, first, middle, column, value);
                    first += count;
                    last = file.upper_bound_of(L::TABLE, middle + 1, first, column, value);
                    first = first2;
                    break;
                }
            }
        }

        RowIterator::new(row.file(), first..last)
    }

    fn row_decode<R: AsRow, T: Decode>(&self, row: R, column: usize) -> T {
        T::decode(row.file(), self.row_usize(row, column))
    }

    //
    // Attribute
    //

    fn attribute_name(&self, row: Attribute) -> &'a str {
        let AttributeType::MemberRef(row) = self.row_decode(row, 1);
        let MemberRefParent::TypeRef(row) = self.row_decode(row, 0);
        self.type_ref_name(row)
    }

    fn attributes<R: AsRow + Into<HasAttribute>>(&self, row: R) -> RowIterator<Attribute> {
        self.row_equal_range(row, 0, row.into().encode())
    }

    fn find_attribute<R: AsRow + Into<HasAttribute>>(&self, row: R, name: &str) -> Option<Attribute> {
        self.attributes(row).find(|attribute| self.attribute_name(*attribute) == name)
    }

    fn has_attribute<R: AsRow + Into<HasAttribute>>(&self, row: R, name: &str) -> bool {
        self.find_attribute(row, name).is_some()
    }

    //
    // Other
    //

    fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName<'a> {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.type_def_namespace(row), self.type_def_name(row)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row)),
            rest => unimplemented!("{rest:?}"),
        }
    }

    //
    // ClassLayout
    //

    fn class_layout_packing_size(&self, row: ClassLayout) -> usize {
        self.row_usize(row, 0)
    }

    //
    // Constant
    //

    fn constant_type(&self, row: Constant) -> Type {
        let code = self.row_usize(row, 0);
        Type::from_code(code).expect("Constant type not found")
    }

    fn constant_value(&self, row: Constant) -> Value {
        let mut blob = self.row_blob(row, 2);
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
            Type::String => Value::String(blob.read_string()),
            rest => unimplemented!("{rest:?}"),
        }
    }

    //
    // Field
    //

    fn field_flags(&self, row: Field) -> FieldAttributes {
        FieldAttributes(self.row_usize(row, 0) as u16)
    }

    fn field_name(&self, row: Field) -> &'a str {
        self.row_str(row, 1)
    }

    fn field_constant(&self, row: Field) -> Option<Constant> {
        self.row_equal_range(row, 1, HasConstant::Field(row).encode()).next()
    }

    //
    // GenericParam
    //

    fn generic_param_number(&self, row: GenericParam) -> u16 {
        self.row_usize(row, 0) as u16
    }

    fn generic_param_name(&self, row: GenericParam) -> &'a str {
        self.row_str(row, 3)
    }

    //
    // ImplMap
    //

    fn impl_map_flags(&self, row: ImplMap) -> PInvokeAttributes {
        PInvokeAttributes(self.row_usize(row, 0))
    }

    fn impl_map_scope(&self, row: ImplMap) -> ModuleRef {
        ModuleRef(Row::new(self.row_usize(row, 3) - 1, row.file()))
    }

    fn impl_map_import_name(&self, row: ImplMap) -> &'a str {
        self.row_str(row, 2)
    }

    //
    // MemberRef
    //

    fn member_ref_parent(&self, row: MemberRef) -> MemberRefParent {
        self.row_decode(row, 0)
    }

    fn member_ref_signature(&self, row: MemberRef) -> Blob<'a> {
        self.row_blob(row, 2)
    }

    //
    // MethodDef
    //

    fn method_def_impl_flags(&self, row: MethodDef) -> MethodImplAttributes {
        MethodImplAttributes(self.row_usize(row, 1) as u16)
    }

    fn method_def_flags(&self, row: MethodDef) -> MethodAttributes {
        MethodAttributes(self.row_usize(row, 2) as u16)
    }

    fn method_def_name(&self, row: MethodDef) -> &'a str {
        self.row_str(row, 3)
    }

    fn method_def_params(&self, row: MethodDef) -> RowIterator<Param> {
        self.row_list(row, 5)
    }

    fn method_def_impl_map(&self, row: MethodDef) -> Option<ImplMap> {
        self.row_equal_range(row, 1, MemberForwarded::MethodDef(row).encode()).next()
    }

    fn method_def_module_name(&self, row: MethodDef) -> String {
        // TODO: riddle should always lower case the module name to avoid allocating here
        let Some(impl_map) = self.method_def_impl_map(row) else {
            return String::new();
        };

        self.module_ref_name(self.impl_map_scope(impl_map)).to_lowercase()
    }

    //
    // ModuleRef
    //

    fn module_ref_name(&self, row: ModuleRef) -> &'a str {
        self.row_str(row, 0)
    }

    //
    // NestedClass
    //

    fn nested_class_inner(&self, row: NestedClass) -> TypeDef {
        TypeDef(Row::new(self.row_usize(row, 0) - 1, row.file()))
    }

    fn nested_class_outer(&self, row: NestedClass) -> TypeDef {
        TypeDef(Row::new(self.row_usize(row, 1) - 1, row.file()))
    }

    //
    // Param
    //

    fn param_flags(&self, row: Param) -> ParamAttributes {
        ParamAttributes(self.row_usize(row, 0) as u16)
    }

    fn param_sequence(&self, row: Param) -> u16 {
        self.row_usize(row, 1) as u16
    }

    fn param_name(&self, row: Param) -> &'a str {
        self.row_str(row, 2)
    }

    //
    // TypeDef
    //

    fn type_def_flags(&self, row: TypeDef) -> TypeAttributes {
        TypeAttributes(self.row_usize(row, 0) as u32)
    }

    fn type_def_name(&self, row: TypeDef) -> &'a str {
        self.row_str(row, 1)
    }

    fn type_def_namespace(&self, row: TypeDef) -> &'a str {
        self.row_str(row, 2)
    }

    fn type_def_extends(&self, row: TypeDef) -> Option<TypeName<'a>> {
        match self.row_usize(row, 3) {
            0 => None,
            code => Some(self.type_def_or_ref(TypeDefOrRef::decode(row.file(), code))),
        }
    }

    fn type_def_methods(&self, row: TypeDef) -> RowIterator<MethodDef> {
        self.row_list(row, 5)
    }

    fn type_def_fields(&self, row: TypeDef) -> RowIterator<Field> {
        self.row_list(row, 4)
    }

    fn type_def_generics(&self, row: TypeDef) -> RowIterator<GenericParam> {
        self.row_equal_range(row, 2, TypeOrMethodDef::TypeDef(row).encode())
    }

    fn type_def_interface_impls(&self, row: TypeDef) -> RowIterator<InterfaceImpl> {
        self.row_equal_range(row, 0, row.0.row + 1)
    }

    fn type_def_enclosing_type(&self, row: TypeDef) -> Option<TypeDef> {
        self.row_equal_range::<TypeDef, NestedClass>(row, 0, row.0.row + 1).next().map(|row| TypeDef(Row::new(self.row_usize(row, 1) - 1, row.file())))
    }

    fn type_def_class_layout(&self, row: TypeDef) -> Option<ClassLayout> {
        self.row_equal_range(row, 2, row.0.row + 1).next()
    }

    //
    // TypeRef
    //

    fn type_ref_name(&self, row: TypeRef) -> &'a str {
        self.row_str(row, 1)
    }

    fn type_ref_namespace(&self, row: TypeRef) -> &'a str {
        self.row_str(row, 2)
    }

    fn type_ref_resolution_scope(&self, row: TypeRef) -> ResolutionScope {
        self.row_decode(row, 0)
    }

    //
    // TypeSpec
    //

    fn type_spec_signature(&self, row: TypeSpec) -> Blob<'a> {
        self.row_blob(row, 0)
    }
}

impl<'a> RowReader<'a> for &'a [File] {
    fn row_file<R: AsRow>(&self, row: R) -> &'a File {
        &self[row.to_row().file]
    }
}
