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
pub use row::*;
use std::collections::*;
pub use type_name::TypeName;

// TODO: move to riddle
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
    TypeName(String),
    TypeRef(TypeDefOrRef),
    EnumDef(TypeDef, Box<Self>),
}

pub struct MethodDefSig {
    pub call_flags: MethodCallAttributes,
    pub return_type: Type,
    pub params: Vec<Type>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Type(TypeDef),
    Const(Field),
    // TODO: get rid of the trailing String - that's just a hack to get around a silly Win32 metadata deficiency where parsing method signatures
    // requires knowing which namespace the method's surrounding interface was defined in.
    Fn(MethodDef, String),
}

pub struct Reader<'a> {
    files: &'a [File],
    items: BTreeMap<&'a str, BTreeMap<&'a str, Vec<Item>>>,

    // TODO: riddle should just avoid nested structs
    nested: HashMap<TypeDef, BTreeMap<&'a str, TypeDef>>,
}

impl<'a> Reader<'a> {
    pub fn new(files: &'a [File]) -> Self {
        let mut items = BTreeMap::<&'a str, BTreeMap<&'a str, Vec<Item>>>::new();
        let mut nested = HashMap::<TypeDef, BTreeMap<&'a str, TypeDef>>::new();
        for (file_index, file) in files.iter().enumerate() {
            for def in file.table::<TypeDef>(file_index) {
                let namespace = files.type_def_namespace(def);
                if namespace.is_empty() {
                    continue;
                }
                let namespace_items = items.entry(namespace).or_default();
                let name = files.type_def_name(def);
                if name == "Apis" {
                    for method in files.type_def_methods(def) {
                        let name = files.method_def_name(method);
                        namespace_items.entry(name).or_default().push(Item::Fn(method, namespace.to_string()));
                    }
                    for field in files.type_def_fields(def) {
                        let name = files.field_name(field);
                        namespace_items.entry(name).or_default().push(Item::Const(field));
                    }
                } else {
                    namespace_items.entry(trim_tick(name)).or_default().push(Item::Type(def));

                    // TODO: these should all be fields on the Apis class so we don't have to go looking for all of these as well.
                    if files.type_def_extends(def) == Some(TypeName::Enum) && !files.type_def_flags(def).contains(TypeAttributes::WindowsRuntime) && !files.has_attribute(def, "ScopedEnumAttribute") {
                        for field in files.type_def_fields(def).filter(|field| files.field_flags(*field).contains(FieldAttributes::Literal)) {
                            let name = files.field_name(field);
                            namespace_items.entry(name).or_default().push(Item::Const(field));
                        }
                    }
                }
            }
            for key in file.table::<NestedClass>(file_index) {
                let inner = files.nested_class_inner(key);
                let outer = files.nested_class_outer(key);
                let name = files.type_def_name(inner);
                nested.entry(outer).or_default().insert(name, inner);
            }
        }
        Self { files, items, nested }
    }

    //
    // Hash functions for fast type lookup
    //

    pub fn namespaces(&self) -> impl Iterator<Item = &str> + '_ {
        self.items.keys().copied()
    }
    pub fn items(&'a self, filter: &'a Filter) -> impl Iterator<Item = Item> + '_ {
        self.items.iter().filter(move |(namespace, _)| filter.includes_namespace(namespace)).flat_map(move |(namespace, items)| items.iter().filter(move |(name, _)| filter.includes_type_name(TypeName::new(namespace, name)))).flat_map(move |(_, items)| items).cloned()
    }
    pub fn namespace_items(&'a self, namespace: &str, filter: &'a Filter) -> impl Iterator<Item = Item> + '_ {
        self.items.get_key_value(namespace).into_iter().flat_map(move |(namespace, items)| items.iter().filter(move |(name, _)| filter.includes_type_name(TypeName::new(namespace, name)))).flat_map(move |(_, items)| items).cloned()
    }
    fn get_item(&self, type_name: TypeName) -> impl Iterator<Item = Item> + '_ {
        if let Some(items) = self.items.get(type_name.namespace) {
            if let Some(items) = items.get(type_name.name) {
                return Some(items.iter().cloned()).into_iter().flatten();
            }
        }
        None.into_iter().flatten()
    }
    pub fn get_type_def(&self, type_name: TypeName) -> impl Iterator<Item = TypeDef> + '_ {
        self.get_item(type_name).filter_map(|item| if let Item::Type(def) = item { Some(def) } else { None })
    }
    pub fn get_method_def(&self, type_name: TypeName) -> impl Iterator<Item = (MethodDef, String)> + '_ {
        self.get_item(type_name).filter_map(|item| if let Item::Fn(def, namespace) = item { Some((def, namespace)) } else { None })
    }

    pub fn nested_types(&self, type_def: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested.get(&type_def).map(|map| map.values().copied()).into_iter().flatten()
    }

    //
    // Attribute table queries
    //

    pub fn attribute_args(&self, row: Attribute) -> Vec<(String, Value)> {
        let AttributeType::MemberRef(member) = self.row_decode(row, 1);
        let mut sig = self.member_ref_signature(member);
        let mut values = self.row_blob(row, 2);
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
                Type::TypeName => Value::TypeName(values.read_str().to_string()),
                Type::TypeDef(def, _) => Value::EnumDef(def, Box::new(values.read_integer(self.type_def_underlying_type(def)))),
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
            let arg = match arg_type {
                ELEMENT_TYPE_BOOLEAN => Value::Bool(values.read_bool()),
                ELEMENT_TYPE_I2 => Value::I16(values.read_i16()),
                ELEMENT_TYPE_I4 => Value::I32(values.read_i32()),
                ELEMENT_TYPE_U4 => Value::U32(values.read_u32()),
                ELEMENT_TYPE_STRING => Value::String(values.read_str().to_string()),
                0x50 => Value::TypeName(values.read_str().to_string()),
                0x55 => {
                    let def = self.get_type_def(TypeName::parse(&name)).next().expect("Type not found");
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
    // Field table queries
    //

    // TODO: enclosing craziness is only needed for nested structs - get rid of those in riddle and this goes away.
    pub fn field_type(&self, row: Field, enclosing: Option<TypeDef>) -> Type {
        let mut blob = self.row_blob(row, 2);
        blob.read_usize();
        blob.read_modifiers();
        let def = self.type_from_blob(&mut blob, enclosing, &[]);

        if self.has_attribute(row, "ConstAttribute") {
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
        self.find_attribute(row, "GuidAttribute").map(|attribute| GUID::from_args(&self.attribute_args(attribute)))
    }
    pub fn field_is_ansi(&self, row: Field) -> bool {
        self.find_attribute(row, "NativeEncodingAttribute").is_some_and(|attribute| matches!(self.attribute_args(attribute).get(0), Some((_, Value::String(encoding))) if encoding == "ansi"))
    }

    //
    // InterfaceImpl table queries
    //

    pub fn interface_impl_type(&self, row: InterfaceImpl, generics: &[Type]) -> Type {
        self.type_from_ref(self.row_decode(row, 1), None, generics)
    }

    //
    // MethodDef table queries
    //

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
            if let Some(attribute) = self.find_attribute(row, "OverloadAttribute") {
                for (_, arg) in self.attribute_args(attribute) {
                    if let Value::String(name) = arg {
                        return name;
                    }
                }
            }
            name.to_string()
        }
    }
    pub fn method_def_static_lib(&self, row: MethodDef) -> Option<String> {
        self.find_attribute(row, "StaticLibraryAttribute").and_then(|attribute| {
            let args = self.attribute_args(attribute);
            if let Value::String(value) = &args[0].1 {
                return Some(value.clone());
            }
            None
        })
    }

    pub fn method_def_signature(&self, method: MethodDef, generics: &[Type]) -> MethodDefSig {
        let mut blob = self.row_blob(method, 4);
        let call_flags = MethodCallAttributes(blob.read_usize() as u8);
        let params = blob.read_usize();
        let return_type = self.type_from_blob(&mut blob, None, generics);

        MethodDefSig { call_flags, return_type, params: (0..params).map(|_| self.type_from_blob(&mut blob, None, generics)).collect() }
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
    pub fn method_def_size(&self, method: MethodDef) -> usize {
        let sig = self.method_def_signature(method, &[]);
        sig.params.iter().fold(0, |sum, param| sum + std::cmp::max(4, self.type_size(param)))
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
    // TODO: this shouldn't be public - needed to work around Win32 metadata hackery.
    pub fn type_size(&self, ty: &Type) -> usize {
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
    // TypeDef table queries
    //

    pub fn type_def_type_name(&self, row: TypeDef) -> TypeName {
        TypeName::new(self.type_def_namespace(row), self.type_def_name(row))
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

    pub fn type_def_has_default_constructor(&self, row: TypeDef) -> bool {
        for attribute in self.attributes(row) {
            if self.attribute_name(attribute) == "ActivatableAttribute" {
                if self.attribute_args(attribute).iter().any(|arg| matches!(arg.1, Value::TypeName(_))) {
                    continue;
                } else {
                    return true;
                }
            }
        }
        false
    }

    pub fn type_def_interfaces(&'a self, row: TypeDef, generics: &'a [Type]) -> impl Iterator<Item = Type> + '_ {
        self.type_def_interface_impls(row).map(move |row| self.interface_impl_type(row, generics))
    }

    pub fn type_def_default_interface(&self, row: TypeDef) -> Option<Type> {
        self.type_def_interface_impls(row).find_map(move |row| if self.has_attribute(row, "DefaultAttribute") { Some(self.interface_impl_type(row, &[])) } else { None })
    }
    pub fn type_def_has_default_interface(&self, row: TypeDef) -> bool {
        self.type_def_interface_impls(row).any(|imp| self.has_attribute(imp, "DefaultAttribute"))
    }
    pub fn type_def_is_handle(&self, row: TypeDef) -> bool {
        self.has_attribute(row, "NativeTypedefAttribute")
    }
    pub fn type_def_is_exclusive(&self, row: TypeDef) -> bool {
        self.has_attribute(row, "ExclusiveToAttribute")
    }
    fn type_def_is_struct(&self, row: TypeDef) -> bool {
        // This check is used to detect virtual functions that return C-style PODs that affect how the stack is packed for x86.
        // It could be defined as a struct with more than one field but that check is complicated as it would have to detect
        // nested structs. Fortunately, this is rare enough that this check is sufficient.
        self.type_def_kind(row) == TypeKind::Struct && !self.type_def_is_handle(row)
    }

    fn type_def_is_primitive(&self, row: TypeDef) -> bool {
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
            for row in self.get_type_def(type_name) {
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
            for row in self.get_type_def(type_name) {
                if check(self, row) {
                    return true;
                }
            }
            false
        }
    }

    pub fn type_def_guid(&self, row: TypeDef) -> Option<GUID> {
        self.find_attribute(row, "GuidAttribute").map(|attribute| GUID::from_args(&self.attribute_args(attribute)))
    }
    pub fn type_def_bases(&self, mut row: TypeDef) -> Vec<TypeDef> {
        let mut bases = Vec::new();
        loop {
            match self.type_def_extends(row) {
                Some(base) if base != TypeName::Object => {
                    row = self.get_type_def(base).next().expect("Type not found");
                    bases.push(row);
                }
                _ => break,
            }
        }
        bases
    }
    pub fn type_def_is_agile(&self, row: TypeDef) -> bool {
        for attribute in self.attributes(row) {
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
        for attribute in self.attributes(row) {
            if self.attribute_name(attribute) == "InvalidHandleValueAttribute" {
                if let Some((_, Value::I64(value))) = self.attribute_args(attribute).get(0) {
                    values.push(*value);
                }
            }
        }
        values
    }
    pub fn type_def_usable_for(&self, row: TypeDef) -> Option<TypeDef> {
        if let Some(attribute) = self.find_attribute(row, "AlsoUsableForAttribute") {
            if let Some((_, Value::String(name))) = self.attribute_args(attribute).get(0) {
                return self.get_type_def(TypeName::new(self.type_def_namespace(row), name.as_str())).next();
            }
        }
        None
    }
    fn type_def_is_nullable(&self, row: TypeDef) -> bool {
        match self.type_def_kind(row) {
            TypeKind::Interface | TypeKind::Class => true,
            // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid them
            // from being doubly wrapped in `Option`.
            TypeKind::Delegate => self.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
            _ => false,
        }
    }

    pub fn type_def_signature(&self, row: TypeDef, generics: &[Type]) -> String {
        match self.type_def_kind(row) {
            TypeKind::Interface => self.type_def_interface_signature(row, generics),
            TypeKind::Class => {
                if let Some(Type::TypeDef(default, generics)) = self.type_def_default_interface(row) {
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
                match base {
                    Type::TypeDef(row, _) => {
                        next = row;
                        result.insert(0, base);
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
    // Signature queries
    //

    //
    // Other type queries
    //

    pub fn type_def_or_ref(&self, code: TypeDefOrRef) -> TypeName {
        match code {
            TypeDefOrRef::TypeDef(row) => TypeName::new(self.type_def_namespace(row), self.type_def_name(row)),
            TypeDefOrRef::TypeRef(row) => TypeName::new(self.type_ref_namespace(row), self.type_ref_name(row)),
            rest => unimplemented!("{rest:?}"),
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
    fn type_is_copyable(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_is_copyable(*row),
            Type::String | Type::BSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
            Type::Win32Array(kind, _) => self.type_is_copyable(kind),
            Type::WinrtArray(kind) => self.type_is_copyable(kind),
            _ => true,
        }
    }
    fn type_has_explicit_layout(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_has_explicit_layout(*row),
            Type::Win32Array(ty, _) => self.type_has_explicit_layout(ty),
            _ => false,
        }
    }
    fn type_has_packing(&self, ty: &Type) -> bool {
        match ty {
            Type::TypeDef(row, _) => self.type_def_has_packing(*row),
            Type::Win32Array(ty, _) => self.type_has_packing(ty),
            _ => false,
        }
    }

    fn type_from_ref(&self, code: TypeDefOrRef, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = self.type_spec_signature(def);
            return self.type_from_blob_impl(&mut blob, None, generics);
        }

        let mut full_name = self.type_def_or_ref(code);

        // TODO: remove this
        for (known_name, kind) in CORE_TYPES {
            if full_name == known_name {
                return kind;
            }
        }

        // TODO: remove this
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

        if let Some(def) = self.get_type_def(full_name).next() {
            Type::TypeDef(def, Vec::new())
        } else {
            Type::TypeRef(code)
        }
    }
    // TODO: this shouldn't be public
    pub fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob.read_modifiers().iter().any(|def| self.type_def_or_ref(*def) == TypeName::IsConst);

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.read_expected(ELEMENT_TYPE_BYREF as usize);

        if blob.read_expected(ELEMENT_TYPE_VOID as usize) {
            return Type::Void;
        }

        let is_array = blob.read_expected(ELEMENT_TYPE_SZARRAY as usize); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.read_expected(ELEMENT_TYPE_PTR as usize) {
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

        match code as u8 {
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
                blob.read_usize(); // ELEMENT_TYPE_VALUETYPE or ELEMENT_TYPE_CLASS

                let type_name = self.type_def_or_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()));
                let def = self.get_type_def(type_name).next().unwrap_or_else(|| panic!("Type not found: {}", type_name));
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef(def, args)
            }
            rest => unimplemented!("{rest:?}"),
        }
    }

    fn type_signature(&self, ty: &Type) -> String {
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

impl<'a> RowReader<'a> for Reader<'a> {
    fn row_file<R: AsRow>(&self, row: R) -> &'a File {
        &self.files[row.to_row().file]
    }
}

fn trim_tick(name: &str) -> &str {
    if name.as_bytes().iter().rev().nth(1) == Some(&b'`') {
        &name[..name.len() - 2]
    } else {
        name
    }
}

// TODO: this should be in riddle's Rust generator if at all - perhaps as convertible types rather than remapped types since there's already some precedent for that.
pub const REMAP_TYPES: [(TypeName, TypeName); 2] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2), (TypeName::D3DMATRIX, TypeName::Matrix4x4)];

// TODO: get rid of at least the second tuple if not the whole thing.
pub const CORE_TYPES: [(TypeName, Type); 11] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::BSTR, Type::BSTR), (TypeName::IInspectable, Type::IInspectable), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR), (TypeName::Type, Type::TypeName), (TypeName::CHAR, Type::U8)];
