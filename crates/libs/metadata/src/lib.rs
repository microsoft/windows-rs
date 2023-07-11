#[doc(hidden)]
pub mod imp;

mod attributes;
mod blob;
mod codes;
mod file;
mod filter;
mod guid;
mod row;
mod r#type;
mod type_name;

pub use attributes::*;
pub use blob::Blob;
pub use codes::*;
pub use file::*;
pub use filter::Filter;
pub use guid::GUID;
use imp::*;
pub use r#type::Type;
use row::Row;
use std::collections::*;
pub use type_name::TypeName;

macro_rules! tables {
    ($($name:ident,)*) => ($(
        #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
        pub struct $name(Row);
    )*)
}

tables! {
    Attribute,
    ClassLayout,
    Constant,
    Field,
    GenericParam,
    ImplMap,
    InterfaceImpl,
    MemberRef,
    MethodDef,
    Module,
    ModuleRef,
    AssemblyRef,
    Param,
    TypeDef,
    TypeRef,
    TypeSpec,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Interface {
    pub ty: Type,
    pub kind: InterfaceKind,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InterfaceKind {
    None,
    Default,
    Overridable,
    Static,
    Base,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct QueryPosition {
    pub object: usize,
    pub guid: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Query(QueryPosition),
    QueryOptional(QueryPosition),
    ResultValue,
    ResultVoid,
    ReturnStruct,
    ReturnValue,
    ReturnVoid,
    PreserveSig,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SignatureParamKind {
    ArrayFixed(usize),
    ArrayRelativeLen(usize),
    ArrayRelativeByteLen(usize),
    ArrayRelativePtr(usize),
    TryInto,
    IntoParam,
    OptionalPointer,
    ValueType,
    Blittable,
    Other,
}

impl SignatureParamKind {
    fn is_array(&self) -> bool {
        matches!(self, Self::ArrayFixed(_) | Self::ArrayRelativeLen(_) | Self::ArrayRelativeByteLen(_) | Self::ArrayRelativePtr(_))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum TypeKind {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
}

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    TypeDef(TypeDef),
    TypeRef(TypeDefOrRef),
    EnumDef(TypeDef, Box<Self>),
    EnumRef(TypeDefOrRef, Box<Self>),
}

pub struct Signature {
    pub def: MethodDef,
    pub params: Vec<SignatureParam>,
    pub return_type: Type,
    pub call_flags: MethodCallAttributes,
}

pub struct SignatureParam {
    pub def: Param,
    pub ty: Type,
    pub kind: SignatureParamKind,
}

#[derive(Default, Clone)]
pub struct Cfg<'a> {
    pub types: BTreeMap<&'a str, BTreeSet<TypeDef>>,
    pub core_types: BTreeSet<Type>,
    pub arches: BTreeSet<&'static str>,
    pub implement: bool,
}

impl<'a> Cfg<'a> {
    pub fn add_feature(&mut self, feature: &'a str) {
        self.types.entry(feature).or_default();
    }
    pub fn union(&self, other: &Self) -> Self {
        let mut union = Self::default();
        self.types.keys().for_each(|feature| {
            union.types.entry(feature).or_default();
        });
        other.types.keys().for_each(|feature| {
            union.types.entry(feature).or_default();
        });
        self.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        other.arches.iter().for_each(|arch| {
            union.arches.insert(arch);
        });
        union
    }
}

pub struct Reader<'a> {
    files: &'a [File],
    types: BTreeMap<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>,
    nested: HashMap<TypeDef, BTreeMap<&'a str, TypeDef>>,
}

impl<'a> Reader<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut types = BTreeMap::<&'a str, BTreeMap<&'a str, Vec<TypeDef>>>::new();
        let mut nested = HashMap::<TypeDef, BTreeMap<&'a str, TypeDef>>::new();
        for (file_index, file) in files.iter().enumerate() {
            for row in 0..file.tables[TABLE_TYPEDEF].len {
                let key = Row::new(row, TABLE_TYPEDEF, file_index);
                let namespace = file.str(key.row as _, key.table as _, 2);
                if namespace.is_empty() {
                    continue;
                }
                let name = trim_tick(file.str(key.row as _, key.table as _, 1));
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

    //
    // Hash functions for fast type lookup
    //

    pub fn namespaces(&self) -> impl Iterator<Item = &str> + '_ {
        self.types.keys().copied()
    }
    pub fn types(&'a self, filter: &'a Filter) -> impl Iterator<Item = TypeDef> + '_ {
        self.types.iter().filter(move |(namespace, _)| filter.includes_namespace(namespace)).flat_map(move |(_, types)| types.values().flatten().copied().filter(move |def| filter.includes_type(self, *def)))
    }
    pub fn namespace_types(&'a self, namespace: &str, filter: &'a Filter) -> impl Iterator<Item = TypeDef> + '_ {
        self.types.get(namespace).map(move |types| types.values().flatten().copied().filter(move |ty| filter.includes_type(self, *ty))).into_iter().flatten()
    }
    pub fn nested_types(&self, type_def: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested.get(&type_def).map(|map| map.values().copied()).into_iter().flatten()
    }
    pub fn get(&self, type_name: TypeName) -> impl Iterator<Item = TypeDef> + '_ {
        if let Some(types) = self.types.get(type_name.namespace) {
            if let Some(definitions) = types.get(type_name.name) {
                return Some(definitions.iter().copied()).into_iter().flatten();
            }
        }
        None.into_iter().flatten()
    }
    pub fn namespace_functions(&self, namespace: &str) -> impl Iterator<Item = MethodDef> + '_ {
        self.get(TypeName::new(namespace, "Apis")).flat_map(move |apis| self.type_def_methods(apis)).filter(move |method| {
            // The ImplMap table contains import information, without which the function cannot be linked.
            let Some(impl_map) = self.method_def_impl_map(*method) else {
                return false;
            };

            // Skip functions exported by ordinal.
            if self.impl_map_import_name(impl_map).starts_with('#') {
                return false;
            }

            // Skip "inline" function constants.
            self.module_ref_name(self.impl_map_scope(impl_map)) != "FORCEINLINE"
        })
    }
    pub fn namespace_constants(&self, namespace: &str) -> impl Iterator<Item = Field> + '_ {
        self.get(TypeName::new(namespace, "Apis")).flat_map(move |apis| self.type_def_fields(apis))
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
    pub fn row_blob(&self, key: Row, column: usize) -> Blob {
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

    pub fn attribute_type_name(&self, row: Attribute) -> TypeName {
        let AttributeType::MemberRef(row) = self.row_decode(row.0, 1);
        let MemberRefParent::TypeRef(row) = self.row_decode(row.0, 0);
        self.type_ref_type_name(row)
    }
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
        let mut args: Vec<(String, Value)> = Vec::with_capacity(fixed_arg_count);

        for _ in 0..fixed_arg_count {
            let arg = match self.type_from_blob(&mut sig, None, &[]) {
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
                Type::TypeName => Value::TypeDef(self.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
                Type::TypeDef(def, _) => Value::EnumDef(def, Box::new(values.read_integer(self.type_def_underlying_type(def)))),
                // It's impossible to know the type of a TypeRef so we just assume 32-bit integer which covers System.* attribute args
                // reasonably well but the solution is to follow the WinRT metadata and define replacements for those attribute types.
                Type::TypeRef(code) => Value::EnumRef(code, Box::new(values.read_integer(Type::I32))),
                rest => unimplemented!("{rest:?}"),
            };

            args.push((String::new(), arg));
        }

        let named_arg_count = values.read_u16();
        args.reserve(named_arg_count as usize);

        for _ in 0..named_arg_count {
            let _id = values.read_u8();
            let arg_type = values.read_u8();
            let mut name = values.read_str().to_string();
            let arg = match arg_type as _ {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(values.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(values.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(values.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(values.read_u32()),
                ELEMENT_TYPE_STRING => Value::String(values.read_str().to_string()),
                0x50 => Value::TypeDef(self.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
                0x55 => {
                    let def = self.get(TypeName::parse(&name)).next().expect("Type not found");
                    name = values.read_str().into();
                    Value::EnumDef(def, Box::new(values.read_integer(self.type_def_underlying_type(def))))
                }
                rest => unimplemented!("{rest:?}"),
            };
            args.push((name, arg));
        }

        assert_eq!(sig.slice.len(), 0);
        assert_eq!(values.slice.len(), 0);

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
            Type::String => Value::String(blob.read_string()),
            rest => unimplemented!("{rest:?}"),
        }
    }

    //
    // Field table queries
    //

    pub fn field_flags(&self, row: Field) -> FieldAttributes {
        FieldAttributes(self.row_usize(row.0, 0) as _)
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
    // TODO: enclosing craziness is only needed for nested structs - get rid of those in metadata vnext and this goes away.
    pub fn field_type(&self, row: Field, enclosing: Option<TypeDef>) -> Type {
        let mut blob = self.row_blob(row.0, 2);
        blob.read_usize();
        blob.read_modifiers();
        let def = self.type_from_blob(&mut blob, enclosing, &[]);

        if self.field_is_const(row) {
            def.to_const_type().to_const_ptr()
        } else {
            def
        }
    }
    pub fn field_is_blittable(&self, row: Field, enclosing: TypeDef) -> bool {
        self.type_is_blittable(&self.field_type(row, Some(enclosing)))
    }
    pub fn field_is_copyable(&self, row: Field, enclosing: TypeDef) -> bool {
        self.type_is_copyable(&self.field_type(row, Some(enclosing)))
    }
    pub fn field_guid(&self, row: Field) -> Option<GUID> {
        for attribute in self.field_attributes(row) {
            if self.attribute_name(attribute) == "GuidAttribute" {
                return Some(GUID::from_args(&self.attribute_args(attribute)));
            }
        }
        None
    }
    pub fn field_cfg(&self, row: Field) -> Cfg {
        let mut cfg = Cfg::default();
        self.field_cfg_combine(row, None, &mut cfg);
        cfg
    }
    fn field_cfg_combine(&'a self, row: Field, enclosing: Option<TypeDef>, cfg: &mut Cfg<'a>) {
        self.type_cfg_combine(&self.field_type(row, enclosing), cfg)
    }
    pub fn field_is_ansi(&self, row: Field) -> bool {
        for attribute in self.field_attributes(row) {
            if self.attribute_name(attribute) == "NativeEncodingAttribute" {
                if let Some((_, Value::String(encoding))) = self.attribute_args(attribute).get(0) {
                    if encoding == "ansi" {
                        return true;
                    }
                }
            }
        }
        false
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
    pub fn impl_map_import_name(&self, row: ImplMap) -> &str {
        self.row_str(row.0, 2)
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
        MethodAttributes(self.row_usize(row.0, 2) as _)
    }
    pub fn method_def_name(&self, row: MethodDef) -> &str {
        self.row_str(row.0, 3)
    }
    pub fn method_def_params(&self, row: MethodDef) -> impl Iterator<Item = Param> {
        self.row_list(row.0, TABLE_PARAM, 5).map(Param)
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
    pub fn method_def_can_return_multiple_success_values(&self, row: MethodDef) -> bool {
        self.method_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "CanReturnMultipleSuccessValuesAttribute")
    }
    pub fn method_def_special_name(&self, row: MethodDef) -> String {
        let name = self.method_def_name(row);
        if self.method_def_flags(row).contains(MethodAttributes::SpecialName) {
            if name.starts_with("get") {
                name[4..].to_string()
            } else if name.starts_with("put") {
                format!("Set{}", &name[4..])
            } else if name.starts_with("add") {
                name[4..].to_string()
            } else if name.starts_with("remove") {
                format!("Remove{}", &name[7..])
            } else {
                name.to_string()
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
    pub fn method_def_module_name(&self, row: MethodDef) -> String {
        let Some(impl_map) = self.method_def_impl_map(row) else {
            return String::new();
        };
        self.module_ref_name(self.impl_map_scope(impl_map)).to_lowercase()
    }
    pub fn method_def_last_error(&self, row: MethodDef) -> bool {
        if let Some(map) = self.method_def_impl_map(row) {
            self.impl_map_flags(map).contains(PInvokeAttributes::SupportsLastError)
        } else {
            false
        }
    }
    pub fn method_def_signature(&self, namespace: &str, row: MethodDef, generics: &[Type]) -> Signature {
        let mut blob = self.row_blob(row.0, 4);
        let call_flags = MethodCallAttributes(blob.read_usize() as _);
        let _param_count = blob.read_usize();
        let mut return_type = self.type_from_blob(&mut blob, None, generics);

        let mut params: Vec<SignatureParam> = self
            .method_def_params(row)
            .filter_map(|param| {
                if self.param_sequence(param) == 0 {
                    if self.param_is_const(param) {
                        return_type = return_type.clone().to_const_type();
                    }
                    None
                } else {
                    let is_output = self.param_flags(param).contains(ParamAttributes::Out);
                    let mut ty = self.type_from_blob(&mut blob, None, generics);

                    if let Some(name) = self.param_or_enum(param) {
                        let alt = self.get(TypeName::new(namespace, &name)).next().expect("Enum not found");
                        ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(Type::TypeDef(alt, Vec::new())));
                    }

                    if self.param_is_const(param) || !is_output {
                        ty = ty.to_const_type();
                    }
                    if !is_output {
                        ty = ty.to_const_ptr();
                    }
                    let kind = self.param_kind(param);
                    Some(SignatureParam { def: param, ty, kind })
                }
            })
            .collect();

        for position in 0..params.len() {
            // Point len params back to the corresponding ptr params.
            match params[position].kind {
                SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                    // The len params must be input only.
                    if !self.param_flags(params[relative].def).contains(ParamAttributes::Out) && position != relative && !params[relative].ty.is_pointer() {
                        params[relative].kind = SignatureParamKind::ArrayRelativePtr(position);
                    } else {
                        params[position].kind = SignatureParamKind::Other;
                    }
                }
                SignatureParamKind::ArrayFixed(_) => {
                    if self.param_free_with(params[position].def).is_some() {
                        params[position].kind = SignatureParamKind::Other;
                    }
                }
                _ => {}
            }
        }

        let mut sets = BTreeMap::<usize, Vec<usize>>::new();

        // Finds sets of ptr params pointing at the same len param.
        for (position, param) in params.iter().enumerate() {
            match param.kind {
                SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                    sets.entry(relative).or_default().push(position);
                }
                _ => {}
            }
        }

        // Remove all sets.
        for (len, ptrs) in sets {
            if ptrs.len() > 1 {
                params[len].kind = SignatureParamKind::Other;
                for ptr in ptrs {
                    params[ptr].kind = SignatureParamKind::Other;
                }
            }
        }

        // Remove any byte arrays that aren't byte-sized types.
        for position in 0..params.len() {
            if let SignatureParamKind::ArrayRelativeByteLen(relative) = params[position].kind {
                if !params[position].ty.is_byte_size() {
                    params[position].kind = SignatureParamKind::Other;
                    params[relative].kind = SignatureParamKind::Other;
                }
            }
        }

        for param in &mut params {
            if param.kind == SignatureParamKind::Other {
                if self.signature_param_is_convertible(param) {
                    if self.signature_param_is_failible_param(param) {
                        param.kind = SignatureParamKind::TryInto;
                    } else {
                        param.kind = SignatureParamKind::IntoParam;
                    }
                } else {
                    let flags = self.param_flags(param.def);
                    if param.ty.is_pointer() && (flags.contains(ParamAttributes::Optional) || self.param_is_reserved(param.def)) {
                        param.kind = SignatureParamKind::OptionalPointer;
                    } else if self.type_is_primitive(&param.ty) && (!param.ty.is_pointer() || self.type_is_blittable(&param.ty.deref())) {
                        param.kind = SignatureParamKind::ValueType;
                    } else if self.type_is_blittable(&param.ty) {
                        param.kind = SignatureParamKind::Blittable;
                    }
                }
            }
        }

        Signature { def: row, params, return_type, call_flags }
    }
    pub fn method_def_extern_abi(&self, def: MethodDef) -> &'static str {
        let impl_map = self.method_def_impl_map(def).expect("ImplMap not found");
        let flags = self.impl_map_flags(impl_map);

        if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
            "system"
        } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
            "cdecl"
        } else {
            unimplemented!()
        }
    }
    pub fn method_def_size(&self, namespace: &str, method: MethodDef) -> usize {
        let signature = self.method_def_signature(namespace, method, &[]);
        signature.params.iter().fold(0, |sum, param| sum + std::cmp::max(4, self.type_size(&param.ty)))
    }
    pub fn type_def_size(&self, def: TypeDef) -> usize {
        match self.type_def_kind(def) {
            TypeKind::Struct => {
                if self.type_def_flags(def).contains(TypeAttributes::ExplicitLayout) {
                    self.type_def_fields(def).map(|field| self.type_size(&self.field_type(field, Some(def)))).max().unwrap_or(1)
                } else {
                    let mut sum = 0;
                    for field in self.type_def_fields(def) {
                        let size = self.type_size(&self.field_type(field, Some(def)));
                        let align = self.type_align(&self.field_type(field, Some(def)));
                        sum = (sum + (align - 1)) & !(align - 1);
                        sum += size;
                    }
                    sum
                }
            }
            TypeKind::Enum => self.type_size(&self.type_def_underlying_type(def)),
            _ => 4,
        }
    }
    fn type_size(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 16,
            Type::TypeDef(def, _) => self.type_def_size(*def),
            Type::Win32Array(ty, len) => self.type_size(ty) * len,
            Type::PrimitiveOrEnum(ty, _) => self.type_size(ty),
            _ => 4,
        }
    }
    fn type_def_align(&self, def: TypeDef) -> usize {
        match self.type_def_kind(def) {
            TypeKind::Struct => self.type_def_fields(def).map(|field| self.type_align(&self.field_type(field, Some(def)))).max().unwrap_or(1),
            TypeKind::Enum => self.type_align(&self.type_def_underlying_type(def)),
            _ => 4,
        }
    }
    fn type_align(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 4,
            Type::TypeDef(def, _) => self.type_def_align(*def),
            Type::Win32Array(ty, len) => self.type_align(ty) * len,
            _ => 4,
        }
    }

    //
    // ModuleRef table queries
    //

    fn module_ref_name(&self, row: ModuleRef) -> &str {
        self.row_str(row.0, 0)
    }

    //
    // Param table queries
    //

    pub fn param_flags(&self, row: Param) -> ParamAttributes {
        ParamAttributes(self.row_usize(row.0, 0) as _)
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
    fn param_kind(&self, row: Param) -> SignatureParamKind {
        for attribute in self.param_attributes(row) {
            match self.attribute_name(attribute) {
                "NativeArrayInfoAttribute" => {
                    for (_, value) in self.attribute_args(attribute) {
                        match value {
                            Value::I16(value) => return SignatureParamKind::ArrayRelativeLen(value as _),
                            Value::I32(value) => return SignatureParamKind::ArrayFixed(value as _),
                            _ => {}
                        }
                    }
                }
                "MemorySizeAttribute" => {
                    for (_, value) in self.attribute_args(attribute) {
                        if let Value::I16(value) = value {
                            return SignatureParamKind::ArrayRelativeByteLen(value as _);
                        }
                    }
                }
                _ => {}
            }
        }
        SignatureParamKind::Other
    }
    pub fn param_is_retval(&self, row: Param) -> bool {
        self.param_attributes(row).any(|attribute| self.attribute_name(attribute) == "RetValAttribute")
    }
    pub fn param_is_reserved(&self, row: Param) -> bool {
        self.param_attributes(row).any(|attribute| self.attribute_name(attribute) == "ReservedAttribute")
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
    pub fn param_or_enum(&self, row: Param) -> Option<String> {
        for attribute in self.param_attributes(row) {
            if self.attribute_name(attribute) == "AssociatedEnumAttribute" {
                for (_, arg) in self.attribute_args(attribute) {
                    if let Value::String(name) = arg {
                        return Some(name);
                    }
                }
            }
        }
        None
    }
    pub fn param_is_const(&self, row: Param) -> bool {
        self.param_attributes(row).any(|attribute| self.attribute_name(attribute) == "ConstAttribute")
    }

    //
    // TypeDef table queries
    //

    pub fn type_def_flags(&self, row: TypeDef) -> TypeAttributes {
        TypeAttributes(self.row_usize(row.0, 0) as _)
    }
    pub fn type_def_name(&self, row: TypeDef) -> &str {
        self.row_str(row.0, 1)
    }
    pub fn type_def_namespace(&self, row: TypeDef) -> &str {
        self.row_str(row.0, 2)
    }
    pub fn type_def_type_name(&self, row: TypeDef) -> TypeName {
        TypeName::new(self.type_def_namespace(row), self.type_def_name(row))
    }
    pub fn type_def_extends(&self, row: TypeDef) -> Option<TypeName> {
        match self.row_usize(row.0, 3) {
            0 => None,
            code => Some(self.type_def_or_ref(TypeDefOrRef::decode(row.0.file as _, code))),
        }
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
    pub fn type_def_generics(&self, row: TypeDef) -> Vec<Type> {
        self.row_equal_range(row.0, TABLE_GENERICPARAM, 2, TypeOrMethodDef::TypeDef(row).encode()).map(|row| Type::GenericParam(GenericParam(row))).collect()
    }
    pub fn type_def_interface_impls(&self, row: TypeDef) -> impl Iterator<Item = InterfaceImpl> {
        self.row_equal_range(row.0, TABLE_INTERFACEIMPL, 0, (row.0.row + 1) as _).map(InterfaceImpl)
    }
    pub fn type_def_enclosing_type(&self, row: TypeDef) -> Option<TypeDef> {
        self.row_equal_range(row.0, TABLE_NESTEDCLASS, 0, (row.0.row + 1) as _).next().map(|row| TypeDef(Row::new(self.files[row.file as usize].usize(row.row as _, row.table as _, 1) - 1, TABLE_TYPEDEF, row.file as _)))
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
        match self.type_def_extends(row) {
            None => TypeKind::Interface,
            Some(TypeName::Enum) => TypeKind::Enum,
            Some(TypeName::Delegate) => TypeKind::Delegate,
            Some(TypeName::Struct) => TypeKind::Struct,
            Some(_) => TypeKind::Class,
        }
    }
    pub fn type_def_stdcall(&self, row: TypeDef) -> usize {
        if self.type_def_kind(row) == TypeKind::Struct {
            if self.type_def_flags(row).contains(TypeAttributes::ExplicitLayout) {
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
                if self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
                    self.type_def_fields(row).all(|field| self.field_is_blittable(field, row))
                } else {
                    true
                }
            }
            TypeKind::Enum => true,
            TypeKind::Delegate => !self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
            _ => false,
        }
    }
    pub fn type_def_is_copyable(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Struct => self.type_def_fields(row).all(|field| self.field_is_copyable(field, row)),
            TypeKind::Enum => true,
            TypeKind::Delegate => !self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
            _ => false,
        }
    }
    pub fn type_def_is_callback(&self, row: TypeDef) -> bool {
        !self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) && self.type_def_kind(row) == TypeKind::Delegate
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
    // TODO: consider removing all the expects and just return Option<T> and let the caller expect it
    // that way the metadata reader is a little more schema-agnostic...
    pub fn type_def_invoke_method(&self, row: TypeDef) -> MethodDef {
        self.type_def_methods(row).find(|method| self.method_def_name(*method) == "Invoke").expect("`Invoke` method not found")
    }
    pub fn type_def_interfaces(&'a self, row: TypeDef, generics: &'a [Type]) -> impl Iterator<Item = Interface> + '_ {
        self.type_def_interface_impls(row).map(move |row| self.interface_impl_type(row, generics))
    }
    pub fn type_def_default_interface(&self, row: TypeDef) -> Option<Type> {
        self.type_def_interfaces(row, &[]).find(|interface| interface.kind == InterfaceKind::Default).map(|interface| interface.ty)
    }
    pub fn type_def_has_default_interface(&self, row: TypeDef) -> bool {
        self.type_def_interface_impls(row).any(|imp| self.interface_impl_is_default(imp))
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
        self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) || self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ScopedEnumAttribute")
    }
    pub fn type_def_is_contract(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ApiContractAttribute")
    }
    fn type_def_is_composable(&self, row: TypeDef) -> bool {
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "ComposableAttribute")
    }
    fn type_def_is_struct(&self, row: TypeDef) -> bool {
        // This check is used to detect virtual functions that return C-style PODs that affect how the stack is packed for x86.
        // It could be defined as a struct with more than one field but that check is complicated as it would have to detect
        // nested structs. Fortunately, this is rare enough that this check is sufficient.
        self.type_def_kind(row) == TypeKind::Struct && !self.type_def_is_handle(row)
    }
    pub fn type_def_is_trivially_convertible(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Struct => self.type_def_is_handle(row),
            _ => false,
        }
    }
    pub fn type_def_is_primitive(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Enum => true,
            TypeKind::Struct => self.type_def_is_handle(row),
            TypeKind::Delegate => !self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
            _ => false,
        }
    }
    pub fn type_def_has_explicit_layout(&self, row: TypeDef) -> bool {
        if self.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        fn check(reader: &Reader, row: TypeDef) -> bool {
            if reader.type_def_flags(row).contains(TypeAttributes::ExplicitLayout) {
                return true;
            }
            if reader.type_def_fields(row).any(|field| reader.type_has_explicit_layout(&reader.field_type(field, Some(row)))) {
                return true;
            }
            false
        }
        let type_name = self.type_def_type_name(row);
        if type_name.namespace.is_empty() {
            check(self, row)
        } else {
            for row in self.get(type_name) {
                if check(self, row) {
                    return true;
                }
            }
            false
        }
    }
    pub fn type_def_has_packing(&self, row: TypeDef) -> bool {
        if self.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        fn check(reader: &Reader, row: TypeDef) -> bool {
            if reader.type_def_class_layout(row).is_some() {
                return true;
            }
            if reader.type_def_fields(row).any(|field| reader.type_has_packing(&reader.field_type(field, Some(row)))) {
                return true;
            }
            false
        }
        let type_name = self.type_def_type_name(row);
        if type_name.namespace.is_empty() {
            check(self, row)
        } else {
            for row in self.get(type_name) {
                if check(self, row) {
                    return true;
                }
            }
            false
        }
    }
    pub fn type_def_has_callback(&self, row: TypeDef) -> bool {
        if self.type_def_is_callback(row) {
            return true;
        }
        if self.type_def_kind(row) != TypeKind::Struct {
            return false;
        }
        fn check(reader: &Reader, row: TypeDef) -> bool {
            if reader.type_def_fields(row).any(|field| reader.type_has_callback(&reader.field_type(field, Some(row)))) {
                return true;
            }
            false
        }
        let type_name = self.type_def_type_name(row);
        if type_name.namespace.is_empty() {
            check(self, row)
        } else {
            for row in self.get(type_name) {
                if check(self, row) {
                    return true;
                }
            }
            false
        }
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
        let mut bases = Vec::new();
        loop {
            match self.type_def_extends(row) {
                Some(base) if base != TypeName::Object => {
                    row = self.get(base).next().expect("Type not found");
                    bases.push(row);
                }
                _ => break,
            }
        }
        bases
    }
    pub fn type_def_is_flags(&self, row: TypeDef) -> bool {
        // Win32 enums use the Flags attribute. WinRT enums don't have the Flags attribute but are paritioned merely based
        // on whether they are signed.
        self.type_def_attributes(row).any(|attribute| self.attribute_name(attribute) == "FlagsAttribute") || (self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) && self.type_def_underlying_type(row) == Type::U32)
    }
    pub fn type_def_is_agile(&self, row: TypeDef) -> bool {
        for attribute in self.type_def_attributes(row) {
            match self.attribute_name(attribute) {
                "AgileAttribute" => return true,
                "MarshalingBehaviorAttribute" => {
                    if let Some((_, Value::EnumDef(_, value))) = self.attribute_args(attribute).get(0) {
                        if let Value::I32(2) = **value {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        matches!(self.type_def_type_name(row), TypeName::IAsyncAction | TypeName::IAsyncActionWithProgress | TypeName::IAsyncOperation | TypeName::IAsyncOperationWithProgress)
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
            // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid them
            // from being doubly wrapped in `Option`.
            TypeKind::Delegate => self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
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
                                if let Type::TypeDef(def, _) = child.ty {
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
                if let Type::TypeDef(default, generics) = self.type_def_interfaces(row, generics).find(|row| row.kind == InterfaceKind::Default).expect("Default interface not found").ty {
                    format!("rc({};{})", self.type_def_type_name(row), self.type_def_interface_signature(default, &generics))
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
        }
    }
    fn type_def_interface_signature(&self, row: TypeDef, generics: &[Type]) -> String {
        let guid = self.type_def_guid(row).unwrap();
        if generics.is_empty() {
            format!("{{{guid:#?}}}")
        } else {
            let mut result = format!("pinterface({{{guid:#?}}}");
            for generic in generics {
                result.push(';');
                result.push_str(&self.type_signature(generic));
            }
            result.push(')');
            result
        }
    }
    pub fn type_def_cfg(&self, row: TypeDef, generics: &[Type]) -> Cfg {
        let mut cfg = Cfg::default();
        self.type_def_cfg_combine(row, generics, &mut cfg);
        self.cfg_add_attributes(&mut cfg, self.type_def_attributes(row));
        cfg
    }
    pub fn type_def_cfg_impl(&self, def: TypeDef, generics: &[Type]) -> Cfg {
        let mut cfg = Cfg { implement: true, ..Default::default() };

        fn combine<'a>(reader: &'a Reader, def: TypeDef, generics: &[Type], cfg: &mut Cfg<'a>) {
            reader.type_def_cfg_combine(def, generics, cfg);

            for method in reader.type_def_methods(def) {
                reader.signature_cfg_combine(&reader.method_def_signature(reader.type_def_namespace(def), method, generics), cfg);
            }
        }

        combine(self, def, generics, &mut cfg);

        for def in self.type_def_vtables(def) {
            if let Type::TypeDef(def, generics) = def {
                combine(self, def, &generics, &mut cfg);
            }
        }

        if self.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) {
            for interface in self.type_def_interfaces(def, generics) {
                if let Type::TypeDef(def, generics) = interface.ty {
                    combine(self, def, &generics, &mut cfg);
                }
            }
        }

        self.cfg_add_attributes(&mut cfg, self.type_def_attributes(def));
        cfg
    }
    pub fn type_def_cfg_combine(&'a self, row: TypeDef, generics: &[Type], cfg: &mut Cfg<'a>) {
        let type_name = self.type_def_type_name(row);

        for generic in generics {
            self.type_cfg_combine(generic, cfg);
        }

        if cfg.types.entry(type_name.namespace).or_default().insert(row) {
            match self.type_def_kind(row) {
                TypeKind::Class => {
                    if let Some(default_interface) = self.type_def_default_interface(row) {
                        self.type_cfg_combine(&default_interface, cfg);
                    }
                }
                TypeKind::Interface => {
                    if !self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
                        for def in self.type_def_vtables(row) {
                            if let Type::TypeDef(def, _) = def {
                                cfg.add_feature(self.type_def_namespace(def));
                            }
                        }
                    }
                }
                TypeKind::Struct => {
                    self.type_def_fields(row).for_each(|field| self.field_cfg_combine(field, Some(row), cfg));
                    if !type_name.namespace.is_empty() {
                        for def in self.get(type_name) {
                            if def != row {
                                self.type_def_cfg_combine(def, &[], cfg);
                            }
                        }
                    }
                }
                TypeKind::Delegate => self.signature_cfg_combine(&self.method_def_signature(type_name.namespace, self.type_def_invoke_method(row), generics), cfg),
                _ => {}
            }
        }
    }
    pub fn type_def_vtables(&self, row: TypeDef) -> Vec<Type> {
        let mut result = Vec::new();
        if self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
            result.push(Type::IUnknown);
            if self.type_def_kind(row) != TypeKind::Delegate {
                result.push(Type::IInspectable);
            }
        } else {
            let mut next = row;
            while let Some(base) = self.type_def_interfaces(next, &[]).next() {
                match base.ty {
                    Type::TypeDef(row, _) => {
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
                    rest => unimplemented!("{rest:?}"),
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
        TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row))
    }
    pub fn type_ref_resolution_scope(&self, row: TypeRef) -> ResolutionScope {
        self.row_decode(row.0, 0)
    }

    //
    // TypeSpec table queries
    //

    pub fn type_spec_signature(&self, row: TypeSpec) -> Blob {
        self.row_blob(row.0, 0)
    }

    //
    // Signature queries
    //

    pub fn signature_cfg(&self, signature: &Signature) -> Cfg {
        let mut cfg = Cfg::default();
        self.signature_cfg_combine(signature, &mut cfg);
        self.cfg_add_attributes(&mut cfg, self.method_def_attributes(signature.def));
        cfg
    }
    fn signature_cfg_combine(&'a self, signature: &Signature, cfg: &mut Cfg<'a>) {
        self.type_cfg_combine(&signature.return_type, cfg);
        signature.params.iter().for_each(|param| self.type_cfg_combine(&param.ty, cfg));
    }
    pub fn signature_param_is_borrowed(&self, param: &SignatureParam) -> bool {
        self.type_is_borrowed(&param.ty)
    }
    pub fn signature_param_is_failible_param(&self, param: &SignatureParam) -> bool {
        self.type_is_non_exclusive_winrt_interface(&param.ty)
    }
    pub fn signature_param_is_trivially_convertible(&self, param: &SignatureParam) -> bool {
        self.type_is_trivially_convertible(&param.ty)
    }
    pub fn signature_param_is_convertible(&self, param: &SignatureParam) -> bool {
        !self.param_flags(param.def).contains(ParamAttributes::Out) && !param.ty.is_winrt_array() && !param.ty.is_pointer() && !param.kind.is_array() && (self.type_is_borrowed(&param.ty) || self.type_is_non_exclusive_winrt_interface(&param.ty) || self.type_is_trivially_convertible(&param.ty))
    }
    pub fn signature_param_is_retval(&self, param: &SignatureParam) -> bool {
        // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
        // very sparingly, so this heuristic is used to apply the transformation more uniformly.
        if self.param_is_retval(param.def) {
            return true;
        }
        if !param.ty.is_pointer() {
            return false;
        }
        if param.ty.is_void() {
            return false;
        }
        let flags = self.param_flags(param.def);
        if flags.contains(ParamAttributes::In) || !flags.contains(ParamAttributes::Out) || flags.contains(ParamAttributes::Optional) || param.kind.is_array() {
            return false;
        }
        if self.param_kind(param.def).is_array() {
            return false;
        }
        // If it's bigger than 128 bits, best to pass as a reference.
        if self.type_size(&param.ty.deref()) > 16 {
            return false;
        }
        // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid
        // producing the `Result<Option<T>>` anti-pattern.
        !self.type_is_callback(&param.ty.deref())
    }
    pub fn signature_kind(&self, signature: &Signature) -> SignatureKind {
        if self.method_def_can_return_multiple_success_values(signature.def) {
            return SignatureKind::PreserveSig;
        }
        match &signature.return_type {
            Type::Void if self.signature_is_retval(signature) => SignatureKind::ReturnValue,
            Type::Void => SignatureKind::ReturnVoid,
            Type::HRESULT => {
                if signature.params.len() >= 2 {
                    if let Some(guid) = self.signature_param_is_query_guid(&signature.params) {
                        if let Some(object) = self.signature_param_is_query_object(&signature.params) {
                            if self.param_flags(signature.params[object].def).contains(ParamAttributes::Optional) {
                                return SignatureKind::QueryOptional(QueryPosition { object, guid });
                            } else {
                                return SignatureKind::Query(QueryPosition { object, guid });
                            }
                        }
                    }
                }
                if self.signature_is_retval(signature) {
                    SignatureKind::ResultValue
                } else {
                    SignatureKind::ResultVoid
                }
            }
            Type::TypeDef(def, _) if self.type_def_type_name(*def) == TypeName::NTSTATUS => SignatureKind::ResultVoid,
            Type::TypeDef(def, _) if self.type_def_type_name(*def) == TypeName::WIN32_ERROR => SignatureKind::ResultVoid,
            Type::TypeDef(def, _) if self.type_def_type_name(*def) == TypeName::BOOL && self.method_def_last_error(signature.def) => SignatureKind::ResultVoid,
            _ if self.type_is_struct(&signature.return_type) => SignatureKind::ReturnStruct,
            _ => SignatureKind::PreserveSig,
        }
    }
    fn signature_is_retval(&self, signature: &Signature) -> bool {
        signature.params.last().map_or(false, |param| self.signature_param_is_retval(param))
            && signature.params[..signature.params.len() - 1].iter().all(|param| {
                let flags = self.param_flags(param.def);
                !flags.contains(ParamAttributes::Out)
            })
    }
    fn signature_param_is_query_guid(&self, params: &[SignatureParam]) -> Option<usize> {
        params.iter().rposition(|param| param.ty == Type::ConstPtr(Box::new(Type::GUID), 1) && !self.param_flags(param.def).contains(ParamAttributes::Out))
    }
    fn signature_param_is_query_object(&self, params: &[SignatureParam]) -> Option<usize> {
        params.iter().rposition(|param| param.ty == Type::MutPtr(Box::new(Type::Void), 2) && self.param_is_com_out_ptr(param.def))
    }

    //
    // Other type queries
    //

    fn cfg_add_attributes(&self, cfg: &mut Cfg, attributes: impl Iterator<Item = Attribute>) {
        for attribute in attributes {
            match self.attribute_name(attribute) {
                "SupportedArchitectureAttribute" => {
                    if let Some((_, Value::EnumDef(_, value))) = self.attribute_args(attribute).get(0) {
                        if let Value::I32(value) = **value {
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
                }
                "DeprecatedAttribute" => {
                    cfg.add_feature("deprecated");
                }
                _ => {}
            }
        }
    }
    pub fn type_collect_standalone(&self, ty: &Type, set: &mut BTreeSet<Type>) {
        let ty = ty.to_underlying_type();
        if !set.insert(ty.clone()) {
            return;
        }

        let Type::TypeDef(def, generics) = &ty else {
            return;
        };

        let def = *def;

        // Ensure that we collect all the typedefs of the same name. We need to
        // do this in the case where the user specifies a top level item that
        // references a typedef by name, but that name resolves to more than 1
        // Type based on target architecture (typically)
        //
        // Note this is a bit overeager as we can collect a typedef that is used
        // by one architecture but not by another
        let type_name = self.type_def_type_name(def);
        if !type_name.namespace.is_empty() {
            for row in self.get(type_name) {
                if def != row {
                    self.type_collect_standalone(&Type::TypeDef(row, Vec::new()), set);
                }
            }
        }

        for generic in generics {
            self.type_collect_standalone(generic, set);
        }
        for field in self.type_def_fields(def) {
            let ty = self.field_type(field, Some(def));
            if let Type::TypeDef(def, _) = &ty {
                if self.type_def_namespace(*def).is_empty() {
                    continue;
                }
            }
            self.type_collect_standalone(&ty, set);
        }
        for method in self.type_def_methods(def) {
            // Skip delegate pseudo-constructors.
            if self.method_def_name(method) == ".ctor" {
                continue;
            }
            let signature = self.method_def_signature(self.type_def_namespace(def), method, generics);
            self.type_collect_standalone(&signature.return_type, set);
            signature.params.iter().for_each(|param| self.type_collect_standalone(&param.ty, set));
        }
        for interface in self.type_interfaces(&ty) {
            self.type_collect_standalone(&interface.ty, set);
        }
        if self.type_def_kind(def) == TypeKind::Struct && self.type_def_fields(def).next().is_none() && self.type_def_guid(def).is_some() {
            set.insert(Type::GUID);
        }

        self.type_collect_standalone_nested(def, set);
    }

    fn type_collect_standalone_nested(&self, td: TypeDef, set: &mut BTreeSet<Type>) {
        for nested in self.nested_types(td) {
            self.type_collect_standalone_nested(nested, set);

            for field in self.type_def_fields(nested) {
                let ty = self.field_type(field, Some(nested));
                if let Type::TypeDef(def, _) = &ty {
                    // Skip the fields that actually refer to the anonymous nested
                    // type, otherwise it will get added to the typeset and emitted
                    if self.type_def_namespace(*def).is_empty() {
                        continue;
                    }

                    self.type_collect_standalone(&ty, set);
                }
            }
        }
    }

    pub fn type_cfg(&self, ty: &Type) -> Cfg {
        let mut cfg = Cfg::default();
        self.type_cfg_combine(ty, &mut cfg);
        cfg
    }
    pub fn type_cfg_combine(&'a self, ty: &Type, cfg: &mut Cfg<'a>) {
        match ty {
            Type::TypeDef(row, generics) => self.type_def_cfg_combine(*row, generics, cfg),
            Type::Win32Array(ty, _) => self.type_cfg_combine(ty, cfg),
            Type::ConstPtr(ty, _) => self.type_cfg_combine(ty, cfg),
            Type::MutPtr(ty, _) => self.type_cfg_combine(ty, cfg),
            Type::WinrtArray(ty) => self.type_cfg_combine(ty, cfg),
            Type::WinrtArrayRef(ty) => self.type_cfg_combine(ty, cfg),
            ty => _ = cfg.core_types.insert(ty.clone()),
        }
    }
    pub fn type_interfaces(&self, ty: &Type) -> Vec<Interface> {
        // TODO: collect into btree map and then return collected vec
        // This will both sort the results and should make finding dupes faster
        fn walk(reader: &Reader, result: &mut Vec<Interface>, parent: &Type, is_base: bool) {
            if let Type::TypeDef(row, generics) = parent {
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
        if let Type::TypeDef(row, _) = ty {
            if self.type_def_kind(*row) == TypeKind::Class {
                for base in self.type_def_bases(*row) {
                    walk(self, &mut result, &Type::TypeDef(base, Vec::new()), true);
                }
                for attribute in self.type_def_attributes(*row) {
                    match self.attribute_name(attribute) {
                        "StaticAttribute" | "ActivatableAttribute" => {
                            for (_, arg) in self.attribute_args(attribute) {
                                if let Value::TypeDef(row) = arg {
                                    result.push(Interface { ty: Type::TypeDef(row, Vec::new()), kind: InterfaceKind::Static });
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        result.sort_by(|a, b| self.type_name(&a.ty).cmp(self.type_name(&b.ty)));
        result
    }
    pub fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.type_def_namespace(row), self.type_def_name(row)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row)),
            rest => unimplemented!("{rest:?}"),
        }
    }
    fn type_stdcall(&self, ty: &Type) -> usize {
        match ty {
            Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::I64 | Type::U64 | Type::F64 => 8,
            Type::GUID => 16,
            Type::TypeDef(row, _) => self.type_def_stdcall(*row),
            _ => 4,
        }
    }
    pub fn type_is_exclusive(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_exclusive(*row),
            _ => false,
        }
    }
    pub fn type_is_blittable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_blittable(*row),
            Type::String | Type::BSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
            Type::Win32Array(kind, _) => self.type_is_blittable(kind),
            Type::WinrtArray(kind) => self.type_is_blittable(kind),
            _ => true,
        }
    }
    pub fn type_is_copyable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_copyable(*row),
            Type::String | Type::BSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
            Type::Win32Array(kind, _) => self.type_is_copyable(kind),
            Type::WinrtArray(kind) => self.type_is_copyable(kind),
            _ => true,
        }
    }
    pub fn type_has_explicit_layout(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_has_explicit_layout(*row),
            Type::Win32Array(ty, _) => self.type_has_explicit_layout(ty),
            _ => false,
        }
    }
    pub fn type_has_packing(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_has_packing(*row),
            Type::Win32Array(ty, _) => self.type_has_packing(ty),
            _ => false,
        }
    }
    pub fn type_has_callback(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_has_callback(*row),
            Type::Win32Array(ty, _) => self.type_has_callback(ty),
            _ => false,
        }
    }
    fn type_from_ref(&self, code: TypeDefOrRef, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = self.type_spec_signature(def);
            return self.type_from_blob_impl(&mut blob, None, generics);
        }

        let mut full_name = self.type_def_or_ref(code);

        for (known_name, kind) in CORE_TYPES {
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
            if full_name.namespace.is_empty() {
                let nested = &self.nested[&outer];
                let Some(inner) = nested.get(full_name.name) else {
                    panic!("Nested type not found: {}.{}", self.type_def_type_name(outer), full_name.name);
                };
                return Type::TypeDef(*inner, Vec::new());
            }
        }

        if let Some(ty) = self.get(full_name).next() {
            Type::TypeDef(ty, Vec::new())
        } else {
            Type::TypeRef(code)
        }
    }
    fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob.read_modifiers().iter().any(|def| self.type_def_or_ref(*def) == TypeName::IsConst);

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.read_expected(ELEMENT_TYPE_BYREF as _);

        if blob.read_expected(ELEMENT_TYPE_VOID as _) {
            return Type::Void;
        }

        let is_array = blob.read_expected(ELEMENT_TYPE_SZARRAY as _); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.read_expected(ELEMENT_TYPE_PTR as _) {
            pointers += 1;
        }

        let kind = self.type_from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            Type::MutPtr(Box::new(kind), pointers)
        } else if is_const {
            Type::ConstRef(Box::new(kind))
        } else if is_array {
            if is_ref {
                Type::WinrtArrayRef(Box::new(kind))
            } else {
                Type::WinrtArray(Box::new(kind))
            }
        } else {
            kind
        }
    }
    fn type_from_blob_impl(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        let code = blob.read_usize();

        if let Some(code) = Type::from_code(code) {
            return code;
        }

        match code as _ {
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => self.type_from_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()), enclosing, generics),
            ELEMENT_TYPE_VAR => generics.get(blob.read_usize()).unwrap_or(&Type::Void).clone(),
            ELEMENT_TYPE_ARRAY => {
                let kind = self.type_from_blob(blob, enclosing, generics);
                let _rank = blob.read_usize();
                let _count = blob.read_usize();
                let bounds = blob.read_usize();
                Type::Win32Array(Box::new(kind), bounds)
            }
            ELEMENT_TYPE_GENERICINST => {
                blob.read_usize();

                let def = self.get(self.type_def_or_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()))).next().expect("Type not found");
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef(def, args)
            }
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_name(&self, ty: &Type) -> &str {
        match ty {
            Type::TypeDef(row, _) => self.type_def_name(*row),
            _ => "",
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
            Type::ISize => "is".to_string(),
            Type::USize => "us".to_string(),
            Type::String => "string".to_string(),
            Type::IInspectable => "cinterface(IInspectable)".to_string(),
            Type::GUID => "g16".to_string(),
            Type::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_string(),
            Type::TypeDef(row, generics) => self.type_def_signature(*row, generics),
            rest => unimplemented!("{rest:?}"),
        }
    }
    pub fn type_is_nullable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_nullable(*row),
            Type::IInspectable | Type::IUnknown => true,
            _ => false,
        }
    }
    fn type_is_borrowed(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => !self.type_def_is_blittable(*row),
            Type::BSTR | Type::PCSTR | Type::PCWSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => true,
            _ => false,
        }
    }
    pub fn type_is_non_exclusive_winrt_interface(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => {
                let flags = self.type_def_flags(*row);
                if !flags.contains(TypeAttributes::WindowsRuntime) {
                    false
                } else {
                    match self.type_def_kind(*row) {
                        TypeKind::Interface => !self.type_def_is_exclusive(*row),
                        TypeKind::Class => self.type_def_is_composable(*row),
                        _ => false,
                    }
                }
            }
            _ => false,
        }
    }
    pub fn type_is_trivially_convertible(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_trivially_convertible(*row),
            Type::PCSTR | Type::PCWSTR => true,
            _ => false,
        }
    }
    pub fn type_is_callback(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_callback(*row),
            _ => false,
        }
    }
    pub fn type_is_primitive(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_primitive(*row),
            Type::Bool | Type::Char | Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::F32 | Type::F64 | Type::ISize | Type::USize | Type::HRESULT | Type::ConstPtr(_, _) | Type::MutPtr(_, _) => true,
            _ => false,
        }
    }
    pub fn type_is_struct(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_struct(*row),
            Type::GUID => true,
            _ => false,
        }
    }
    pub fn type_underlying_type(&self, ty: &Type) -> Type {
        match ty {
            Type::TypeDef(row, _) => self.type_def_underlying_type(*row),
            Type::HRESULT => Type::I32,
            _ => ty.clone(),
        }
    }
    pub fn type_has_replacement(&self, ty: &Type) -> bool {
        match ty {
            Type::HRESULT | Type::PCSTR | Type::PCWSTR => true,
            Type::TypeDef(row, _) => self.type_def_is_handle(*row) || self.type_def_kind(*row) == TypeKind::Enum,
            _ => false,
        }
    }
}

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}

pub const REMAP_TYPES: [(TypeName, TypeName); 2] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2), (TypeName::D3DMATRIX, TypeName::Matrix4x4)];

pub const CORE_TYPES: [(TypeName, Type); 11] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::BSTR, Type::BSTR), (TypeName::IInspectable, Type::IInspectable), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR), (TypeName::Type, Type::TypeName), (TypeName::CHAR, Type::U8)];
