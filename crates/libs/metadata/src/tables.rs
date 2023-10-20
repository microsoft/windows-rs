use super::*;

macro_rules! tables {
    ($(($name:ident, $table:literal))+) => {
        $(
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub struct $name(pub Row);
        impl AsRow for $name {
            const TABLE: usize = $table;
            fn to_row(&self) -> Row {
                self.0
            }
            fn from_row(row: Row) -> Self {
                $name(row)
            }
        }
    )*
    };
}

tables! {
    (AssemblyRef, 15)
    (Attribute, 1)
    (ClassLayout, 16)
    (Constant, 0)
    (Field, 2)
    (GenericParam, 3)
    (ImplMap, 11)
    (InterfaceImpl, 4)
    (MemberRef, 5)
    (MethodDef, 6)
    (Module, 14)
    (ModuleRef, 12)
    (NestedClass, 13)
    (Param, 7)
    (TypeDef, 8)
    (TypeRef, 9)
    (TypeSpec, 10)
}

impl Attribute {
    pub fn parent(&self) -> HasAttribute {
        self.decode(0)
    }

    pub fn ty(&self) -> AttributeType {
        self.decode(1)
    }

    pub fn name(&self) -> &'static str {
        let AttributeType::MemberRef(ctor) = self.ty();
        let MemberRefParent::TypeRef(ty) = ctor.parent();
        ty.name()
    }

    pub fn type_name(&self) -> TypeName {
        let AttributeType::MemberRef(ctor) = self.ty();
        let MemberRefParent::TypeRef(ty) = ctor.parent();
        ty.type_name()
    }

    pub fn args(&self) -> Vec<(&'static str, Value)> {
        let AttributeType::MemberRef(member) = self.ty();
        let mut sig = member.blob(2);
        let mut values = self.blob(2);
        let _prolog = values.read_u16();
        let _this_and_gen_param_count = sig.read_usize();
        let fixed_arg_count = sig.read_usize();
        let _ret_type = sig.read_usize();
        let mut args = Vec::with_capacity(fixed_arg_count);
        let reader = self.reader();

        for _ in 0..fixed_arg_count {
            let arg = match reader.type_from_blob(&mut sig, None, &[]) {
                Type::Bool => Value::Bool(values.read_bool()),
                Type::I8 => Value::I8(values.read_i8()),
                Type::U8 => Value::U8(values.read_u8()),
                Type::I16 => Value::I16(values.read_i16()),
                Type::U16 => Value::U16(values.read_u16()),
                Type::I32 => Value::I32(values.read_i32()),
                Type::U32 => Value::U32(values.read_u32()),
                Type::I64 => Value::I64(values.read_i64()),
                Type::U64 => Value::U64(values.read_u64()),
                Type::String => Value::String(values.read_str().to_string()),
                Type::Type => Value::TypeName(TypeName::parse(values.read_str())),
                Type::TypeDef(def, _) => Value::EnumDef(def, Box::new(values.read_integer(def.underlying_type()))),
                rest => unimplemented!("{rest:?}"),
            };

            args.push(("", arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let mut name = values.read_str();
            let arg = match arg_type {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(values.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(values.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(values.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(values.read_u32()),
                ELEMENT_TYPE_STRING => Value::String(values.read_str().to_string()),
                0x50 => Value::TypeName(TypeName::parse(values.read_str())),
                0x55 => {
                    let type_name = TypeName::parse(name);
                    let def = reader.get_type_def(type_name.namespace, type_name.name).next().expect("Type not found");
                    name = values.read_str();
                    Value::EnumDef(def, Box::new(values.read_integer(def.underlying_type())))
                }
                rest => unimplemented!("{rest:?}"),
            };
            args.push((name, arg));
        }

        debug_assert_eq!(sig.slice.len(), 0);
        debug_assert_eq!(values.slice.len(), 0);

        args
    }
}

impl ClassLayout {
    pub fn packing_size(&self) -> usize {
        self.usize(0)
    }
}

impl Constant {
    pub fn ty(&self) -> Type {
        Type::from_code(self.usize(0)).expect("Constant type not found")
    }

    pub fn value(&self) -> Value {
        let mut blob = self.blob(2);

        match self.ty() {
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
}

impl Field {
    pub fn flags(&self) -> FieldAttributes {
        FieldAttributes(self.usize(0) as u16)
    }

    pub fn name(&self) -> &'static str {
        self.str(1)
    }

    pub fn constant(&self) -> Option<Constant> {
        self.equal_range(1, HasConstant::Field(*self).encode()).next()
    }

    // TODO: enclosing craziness is only needed for nested structs - get rid of those in riddle and this goes away.
    pub fn ty(&self, enclosing: Option<TypeDef>) -> Type {
        let mut blob = self.blob(2);
        blob.read_usize();
        blob.read_modifiers();
        let def = self.reader().type_from_blob(&mut blob, enclosing, &[]);

        if self.has_attribute("ConstAttribute") {
            def.to_const_type().to_const_ptr()
        } else {
            def
        }
    }
}

impl GenericParam {
    pub fn number(&self) -> u16 {
        self.usize(0) as u16
    }

    pub fn name(&self) -> &'static str {
        self.str(3)
    }
}

impl ImplMap {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0))
    }

    pub fn scope(&self) -> ModuleRef {
        ModuleRef(self.row(3))
    }

    pub fn import_name(&self) -> &'static str {
        self.str(2)
    }
}

impl InterfaceImpl {
    pub fn ty(&self, generics: &[Type]) -> Type {
        self.reader().type_from_ref(self.decode(1), None, generics)
    }
}

impl MemberRef {
    pub fn parent(&self) -> MemberRefParent {
        self.decode(0)
    }

    pub fn name(&self) -> &'static str {
        self.str(1)
    }
}

impl MethodDef {
    pub fn impl_flags(&self) -> MethodImplAttributes {
        MethodImplAttributes(self.usize(1) as u16)
    }

    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.usize(2) as u16)
    }

    pub fn name(&self) -> &'static str {
        self.str(3)
    }

    pub fn params(&self) -> RowIterator<Param> {
        self.list(5)
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.equal_range(1, MemberForwarded::MethodDef(*self).encode()).next()
    }

    pub fn module_name(&self) -> String {
        // TODO: riddle should always lower case the module name to avoid allocating here
        let Some(impl_map) = self.impl_map() else {
            return String::new();
        };

        impl_map.scope().name().to_lowercase()
    }

    pub fn signature(&self, generics: &[Type]) -> MethodDefSig {
        let reader = self.reader();
        let mut blob = self.blob(4);
        let call_flags = MethodCallAttributes(blob.read_usize() as u8);
        let params = blob.read_usize();
        let return_type = reader.type_from_blob(&mut blob, None, generics);

        MethodDefSig { call_flags, return_type, params: (0..params).map(|_| reader.type_from_blob(&mut blob, None, generics)).collect() }
    }
}

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.str(0)
    }
}

impl NestedClass {
    pub fn inner(&self) -> TypeDef {
        TypeDef(self.row(0))
    }

    pub fn outer(&self) -> TypeDef {
        TypeDef(self.row(1))
    }
}

impl Param {
    pub fn flags(&self) -> ParamAttributes {
        ParamAttributes(self.usize(0) as u16)
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1) as u16
    }

    pub fn name(&self) -> &'static str {
        self.str(2)
    }
}

impl TypeDef {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.usize(0) as u32)
    }

    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }

    pub fn extends(&self) -> Option<TypeName> {
        let extends = self.usize(3);

        if extends == 0 {
            return None;
        }

        Some(TypeDefOrRef::decode(self.file(), extends).type_name())
    }

    pub fn methods(&self) -> RowIterator<MethodDef> {
        self.list(5)
    }

    pub fn fields(&self) -> RowIterator<Field> {
        self.list(4)
    }

    pub fn generics(&self) -> RowIterator<GenericParam> {
        self.equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interface_impls(&self) -> RowIterator<InterfaceImpl> {
        self.equal_range(0, self.index() + 1)
    }

    pub fn enclosing_type(&self) -> Option<TypeDef> {
        self.equal_range::<NestedClass>(0, self.index() + 1).next().map(|row| TypeDef(row.row(1)))
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.equal_range(2, self.index() + 1).next()
    }

    pub fn underlying_type(&self) -> Type {
        let field = self.fields().next().expect("Field not found");
        if let Some(constant) = field.constant() {
            constant.ty()
        } else {
            field.ty(Some(*self))
        }
    }

    pub fn kind(&self) -> TypeKind {
        match self.extends() {
            None => TypeKind::Interface,
            Some(TypeName::Enum) => TypeKind::Enum,
            Some(TypeName::Delegate) => TypeKind::Delegate,
            Some(TypeName::Struct) => TypeKind::Struct,
            Some(_) => TypeKind::Class,
        }
    }

    pub fn size(&self) -> usize {
        match self.kind() {
            TypeKind::Struct => {
                if self.flags().contains(TypeAttributes::ExplicitLayout) {
                    self.fields().map(|field| field.ty(Some(*self)).size()).max().unwrap_or(1)
                } else {
                    let mut sum = 0;
                    for field in self.fields() {
                        let ty = field.ty(Some(*self));
                        let size = ty.size();
                        let align = ty.align();
                        sum = (sum + (align - 1)) & !(align - 1);
                        sum += size;
                    }
                    sum
                }
            }
            TypeKind::Enum => self.underlying_type().size(),
            _ => 4,
        }
    }

    pub fn align(&self) -> usize {
        match self.kind() {
            TypeKind::Struct => self.fields().map(|field| field.ty(Some(*self)).align()).max().unwrap_or(1),
            TypeKind::Enum => self.underlying_type().align(),
            _ => 4,
        }
    }
}

impl TypeRef {
    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }

    pub fn type_name(&self) -> TypeName {
        TypeName::new(self.namespace(), self.name())
    }

    pub fn resolution_scope(&self) -> ResolutionScope {
        self.decode(0)
    }
}

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}
