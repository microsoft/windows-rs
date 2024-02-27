use std::collections::*;
pub use windows_metadata::*;

#[derive(Clone)]
pub struct Interface {
    pub ty: Type,
    pub kind: InterfaceKind,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
    IntoParam,
    OptionalPointer,
    ValueType,
    Blittable,
    Other,
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

#[derive(PartialEq, Eq, Debug)]
pub enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Guid(pub u32, pub u16, pub u16, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl Guid {
    pub fn from_args(args: &[(&str, Value)]) -> Self {
        fn unwrap_u32(value: &Value) -> u32 {
            match value {
                Value::U32(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        fn unwrap_u16(value: &Value) -> u16 {
            match value {
                Value::U16(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        fn unwrap_u8(value: &Value) -> u8 {
            match value {
                Value::U8(value) => *value,
                rest => unimplemented!("{rest:?}"),
            }
        }
        Self(unwrap_u32(&args[0].1), unwrap_u16(&args[1].1), unwrap_u16(&args[2].1), unwrap_u8(&args[3].1), unwrap_u8(&args[4].1), unwrap_u8(&args[5].1), unwrap_u8(&args[6].1), unwrap_u8(&args[7].1), unwrap_u8(&args[8].1), unwrap_u8(&args[9].1), unwrap_u8(&args[10].1))
    }

    pub fn from_string_args(args: &[&str]) -> Self {
        Self(args[0].parse().unwrap(), args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), args[5].parse().unwrap(), args[6].parse().unwrap(), args[7].parse().unwrap(), args[8].parse().unwrap(), args[9].parse().unwrap(), args[10].parse().unwrap())
    }
}

impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x?}-{:04x?}-{:04x?}-{:02x?}{:02x?}-{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}

impl SignatureParamKind {
    fn is_array(&self) -> bool {
        matches!(self, Self::ArrayFixed(_) | Self::ArrayRelativeLen(_) | Self::ArrayRelativeByteLen(_) | Self::ArrayRelativePtr(_))
    }
}

impl SignatureParam {
    pub fn is_convertible(&self) -> bool {
        !self.def.flags().contains(ParamAttributes::Out) && !self.ty.is_winrt_array() && !self.ty.is_pointer() && !self.kind.is_array() && (type_is_borrowed(&self.ty) || type_is_trivially_convertible(&self.ty))
    }

    fn is_retval(&self) -> bool {
        // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
        // very sparingly, so this heuristic is used to apply the transformation more uniformly.
        if self.def.has_attribute("RetValAttribute") {
            return true;
        }
        if !self.ty.is_pointer() {
            return false;
        }
        if self.ty.is_void() {
            return false;
        }
        let flags = self.def.flags();
        if flags.contains(ParamAttributes::In) || !flags.contains(ParamAttributes::Out) || flags.contains(ParamAttributes::Optional) || self.kind.is_array() {
            return false;
        }
        if param_kind(self.def).is_array() {
            return false;
        }
        // If it's bigger than 128 bits, best to pass as a reference.
        if self.ty.deref().size() > 16 {
            return false;
        }
        // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid
        // producing the `Result<Option<T>>` anti-pattern.
        match self.ty.deref() {
            Type::TypeDef(def, _) => !type_def_is_callback(def),
            _ => true,
        }
    }
}

impl Signature {
    pub fn kind(&self) -> SignatureKind {
        if self.def.has_attribute("CanReturnMultipleSuccessValuesAttribute") {
            return SignatureKind::PreserveSig;
        }
        match &self.return_type {
            Type::Void if self.is_retval() => SignatureKind::ReturnValue,
            Type::Void => SignatureKind::ReturnVoid,
            Type::HRESULT => {
                if self.params.len() >= 2 {
                    if let Some((guid, object)) = signature_param_is_query(&self.params) {
                        if self.params[object].def.flags().contains(ParamAttributes::Optional) {
                            return SignatureKind::QueryOptional(QueryPosition { object, guid });
                        } else {
                            return SignatureKind::Query(QueryPosition { object, guid });
                        }
                    }
                }
                if self.is_retval() {
                    SignatureKind::ResultValue
                } else {
                    SignatureKind::ResultVoid
                }
            }
            Type::TypeDef(def, _) if def.type_name() == TypeName::BOOL && method_def_last_error(self.def) => SignatureKind::ResultVoid,
            _ if type_is_struct(&self.return_type) => SignatureKind::ReturnStruct,
            _ => SignatureKind::PreserveSig,
        }
    }

    fn is_retval(&self) -> bool {
        // First we check whether there's an actual retval parameter.
        if let Some(param) = self.params.last() {
            if param.def.has_attribute("RetValAttribute") {
                return true;
            }
        }

        // Then we see if we can infer retval-like behavior more conservatively.
        self.params.last().map_or(false, |param| param.is_retval())
            && self.params[..self.params.len() - 1].iter().all(|param| {
                let flags = param.def.flags();
                !flags.contains(ParamAttributes::Out)
            })
    }
}

pub fn type_def_invoke_method(row: TypeDef) -> MethodDef {
    row.methods().find(|method| method.name() == "Invoke").expect("`Invoke` method not found")
}

pub fn type_def_generics(def: TypeDef) -> Vec<Type> {
    def.generics().map(Type::GenericParam).collect()
}

// TODO: namespace should not be required - it's a hack to accomodate Win32 metadata
// TODO: this is very Rust-specific and Win32-metadata specific with all of its translation. Replace with literal signature parser that just returns slice of types.
pub fn method_def_signature(namespace: &str, row: MethodDef, generics: &[Type]) -> Signature {
    let reader = row.reader();
    let mut blob = row.blob(4);
    let call_flags = MethodCallAttributes(blob.read_usize() as u8);
    let _param_count = blob.read_usize();
    let mut return_type = reader.type_from_blob(&mut blob, None, generics);

    let mut params: Vec<SignatureParam> = row
        .params()
        .filter_map(|param| {
            let param_is_const = param.has_attribute("ConstAttribute");
            if param.sequence() == 0 {
                if param_is_const {
                    return_type = return_type.clone().to_const_type();
                }
                None
            } else {
                let is_output = param.flags().contains(ParamAttributes::Out);
                let mut ty = reader.type_from_blob(&mut blob, None, generics);

                if let Some(name) = param_or_enum(param) {
                    let def = reader.get_type_def(namespace, &name).next().expect("Enum not found");
                    ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(Type::TypeDef(def, Vec::new())));
                }

                if param_is_const || !is_output {
                    ty = ty.to_const_type();
                }
                if !is_output {
                    ty = ty.to_const_ptr();
                }
                let kind = param_kind(param);
                Some(SignatureParam { def: param, ty, kind })
            }
        })
        .collect();

    for position in 0..params.len() {
        // Point len params back to the corresponding ptr params.
        match params[position].kind {
            SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                // The len params must be input only.
                if !params[relative].def.flags().contains(ParamAttributes::Out) && position != relative && !params[relative].ty.is_pointer() {
                    params[relative].kind = SignatureParamKind::ArrayRelativePtr(position);
                } else {
                    params[position].kind = SignatureParamKind::Other;
                }
            }
            SignatureParamKind::ArrayFixed(_) => {
                if params[position].def.has_attribute("FreeWithAttribute") {
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
            if param.is_convertible() {
                param.kind = SignatureParamKind::IntoParam;
            } else {
                let flags = param.def.flags();
                if param.ty.is_pointer() && (flags.contains(ParamAttributes::Optional) || param.def.has_attribute("ReservedAttribute")) {
                    param.kind = SignatureParamKind::OptionalPointer;
                } else if type_is_primitive(&param.ty) && (!param.ty.is_pointer() || type_is_blittable(&param.ty.deref())) {
                    param.kind = SignatureParamKind::ValueType;
                } else if type_is_blittable(&param.ty) {
                    param.kind = SignatureParamKind::Blittable;
                }
            }
        }
    }

    Signature { def: row, params, return_type, call_flags }
}

fn param_kind(row: Param) -> SignatureParamKind {
    for attribute in row.attributes() {
        match attribute.name() {
            "NativeArrayInfoAttribute" => {
                for (_, value) in attribute.args() {
                    match value {
                        Value::I16(value) => return SignatureParamKind::ArrayRelativeLen(value as usize),
                        Value::I32(value) => return SignatureParamKind::ArrayFixed(value as usize),
                        _ => {}
                    }
                }
            }
            "MemorySizeAttribute" => {
                for (_, value) in attribute.args() {
                    if let Value::I16(value) = value {
                        return SignatureParamKind::ArrayRelativeByteLen(value as usize);
                    }
                }
            }
            _ => {}
        }
    }
    SignatureParamKind::Other
}

// TODO: this is a terribly broken Win32 metadata attribute - need to get rid of it.
fn param_or_enum(row: Param) -> Option<String> {
    if row.flags().contains(ParamAttributes::Out) {
        return None;
    }

    row.find_attribute("AssociatedEnumAttribute").and_then(|attribute| {
        for (_, arg) in attribute.args() {
            if let Value::String(name) = arg {
                return Some(name);
            }
        }
        None
    })
}

fn signature_param_is_query(params: &[SignatureParam]) -> Option<(usize, usize)> {
    if let Some(guid) = params.iter().rposition(|param| param.ty == Type::ConstPtr(Box::new(Type::GUID), 1) && !param.def.flags().contains(ParamAttributes::Out)) {
        if let Some(object) = params.iter().rposition(|param| param.ty == Type::MutPtr(Box::new(Type::Void), 2) && param.def.has_attribute("ComOutPtrAttribute")) {
            return Some((guid, object));
        }
    }

    None
}

fn method_def_last_error(row: MethodDef) -> bool {
    if let Some(map) = row.impl_map() {
        map.flags().contains(PInvokeAttributes::SupportsLastError)
    } else {
        false
    }
}

pub fn type_is_borrowed(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => !type_def_is_blittable(*row),
        Type::BSTR | Type::VARIANT | Type::PROPVARIANT | Type::PCSTR | Type::PCWSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => true,
        _ => false,
    }
}

fn type_is_trivially_convertible(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => match row.kind() {
            TypeKind::Struct => type_def_is_handle(*row),
            _ => false,
        },
        _ => false,
    }
}

fn type_def_is_callback(row: TypeDef) -> bool {
    !row.flags().contains(TypeAttributes::WindowsRuntime) && row.kind() == TypeKind::Delegate
}

pub fn type_has_callback(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_callback(*row),
        Type::Win32Array(ty, _) => type_has_callback(ty),
        _ => false,
    }
}

pub fn type_def_has_callback(row: TypeDef) -> bool {
    if type_def_is_callback(row) {
        return true;
    }
    if row.kind() != TypeKind::Struct {
        return false;
    }
    fn check(row: TypeDef) -> bool {
        if row.fields().any(|field| type_has_callback(&field.ty(Some(row)))) {
            return true;
        }
        false
    }
    let type_name = row.type_name();
    if type_name.namespace.is_empty() {
        check(row)
    } else {
        for row in row.reader().get_type_def(type_name.namespace, type_name.name) {
            if check(row) {
                return true;
            }
        }
        false
    }
}

pub fn type_interfaces(ty: &Type) -> Vec<Interface> {
    // TODO: collect into btree map and then return collected vec
    // This will both sort the results and should make finding dupes faster
    fn walk(result: &mut Vec<Interface>, parent: &Type, is_base: bool) {
        if let Type::TypeDef(row, generics) = parent {
            for mut child in type_def_interfaces(*row, generics) {
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
                    walk(result, &child.ty, is_base);
                    result.push(child);
                }
            }
        }
    }
    let mut result = Vec::new();
    walk(&mut result, ty, false);
    if let Type::TypeDef(row, _) = ty {
        if row.kind() == TypeKind::Class {
            for base in type_def_bases(*row) {
                walk(&mut result, &Type::TypeDef(base, Vec::new()), true);
            }
            for attribute in row.attributes() {
                match attribute.name() {
                    "StaticAttribute" | "ActivatableAttribute" => {
                        for (_, arg) in attribute.args() {
                            if let Value::TypeName(type_name) = arg {
                                let def = row.reader().get_type_def(type_name.namespace, type_name.name).next().expect("Type not found");
                                result.push(Interface { ty: Type::TypeDef(def, Vec::new()), kind: InterfaceKind::Static });
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    result.sort_by(|a, b| type_name(&a.ty).cmp(type_name(&b.ty)));
    result
}

fn type_name(ty: &Type) -> &str {
    match ty {
        Type::TypeDef(row, _) => row.name(),
        _ => "",
    }
}

pub fn field_is_blittable(row: Field, enclosing: TypeDef) -> bool {
    type_is_blittable(&row.ty(Some(enclosing)))
}

pub fn field_is_copyable(row: Field, enclosing: TypeDef) -> bool {
    type_is_copyable(&row.ty(Some(enclosing)))
}

pub fn type_is_blittable(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_blittable(*row),
        Type::String | Type::BSTR | Type::VARIANT | Type::PROPVARIANT | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
        Type::Win32Array(kind, _) => type_is_blittable(kind),
        Type::WinrtArray(kind) => type_is_blittable(kind),
        _ => true,
    }
}

fn type_is_copyable(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_copyable(*row),
        Type::String | Type::BSTR | Type::VARIANT | Type::PROPVARIANT | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
        Type::Win32Array(kind, _) => type_is_copyable(kind),
        Type::WinrtArray(kind) => type_is_copyable(kind),
        _ => true,
    }
}

pub fn type_def_is_blittable(row: TypeDef) -> bool {
    match row.kind() {
        TypeKind::Struct => {
            if row.flags().contains(TypeAttributes::WindowsRuntime) {
                row.fields().all(|field| field_is_blittable(field, row))
            } else {
                true
            }
        }
        TypeKind::Enum => true,
        TypeKind::Delegate => !row.flags().contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_def_is_copyable(row: TypeDef) -> bool {
    match row.kind() {
        TypeKind::Struct => row.fields().all(|field| field_is_copyable(field, row)),
        TypeKind::Enum => true,
        TypeKind::Delegate => !row.flags().contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_def_is_exclusive(row: TypeDef) -> bool {
    row.has_attribute("ExclusiveToAttribute")
}

pub fn type_is_struct(ty: &Type) -> bool {
    // This check is used to detect virtual functions that return C-style PODs that affect how the stack is packed for x86.
    // It could be defined as a struct with more than one field but that check is complicated as it would have to detect
    // nested structs. Fortunately, this is rare enough that this check is sufficient.
    match ty {
        Type::TypeDef(row, _) => row.kind() == TypeKind::Struct && !type_def_is_handle(*row),
        Type::GUID => true,
        _ => false,
    }
}

fn type_def_is_primitive(row: TypeDef) -> bool {
    match row.kind() {
        TypeKind::Enum => true,
        TypeKind::Struct => type_def_is_handle(row),
        TypeKind::Delegate => !row.flags().contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_is_primitive(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_primitive(*row),
        Type::Bool | Type::Char | Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::F32 | Type::F64 | Type::ISize | Type::USize | Type::HRESULT | Type::ConstPtr(_, _) | Type::MutPtr(_, _) => true,
        _ => false,
    }
}

fn type_has_explicit_layout(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_explicit_layout(*row),
        Type::Win32Array(ty, _) => type_has_explicit_layout(ty),
        _ => false,
    }
}

pub fn type_def_has_explicit_layout(row: TypeDef) -> bool {
    if row.kind() != TypeKind::Struct {
        return false;
    }
    fn check(row: TypeDef) -> bool {
        if row.flags().contains(TypeAttributes::ExplicitLayout) {
            return true;
        }
        if row.fields().any(|field| type_has_explicit_layout(&field.ty(Some(row)))) {
            return true;
        }
        false
    }
    let type_name = row.type_name();
    if type_name.namespace.is_empty() {
        check(row)
    } else {
        for row in row.reader().get_type_def(type_name.namespace, type_name.name) {
            if check(row) {
                return true;
            }
        }
        false
    }
}

fn type_has_packing(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_packing(*row),
        Type::Win32Array(ty, _) => type_has_packing(ty),
        _ => false,
    }
}

pub fn type_def_has_packing(row: TypeDef) -> bool {
    if row.kind() != TypeKind::Struct {
        return false;
    }
    fn check(row: TypeDef) -> bool {
        if row.class_layout().is_some() {
            return true;
        }
        if row.fields().any(|field| type_has_packing(&field.ty(Some(row)))) {
            return true;
        }
        false
    }
    let type_name = row.type_name();
    if type_name.namespace.is_empty() {
        check(row)
    } else {
        for row in row.reader().get_type_def(type_name.namespace, type_name.name) {
            if check(row) {
                return true;
            }
        }
        false
    }
}

pub fn type_def_interfaces(def: TypeDef, generics: &[Type]) -> impl Iterator<Item = Interface> + '_ {
    def.interface_impls().map(|imp| {
        let kind = if imp.has_attribute("DefaultAttribute") { InterfaceKind::Default } else { InterfaceKind::None };
        Interface { kind, ty: imp.ty(generics) }
    })
}

pub fn type_def_default_interface(row: TypeDef) -> Option<Type> {
    type_def_interfaces(row, &[]).find_map(move |interface| if interface.kind == InterfaceKind::Default { Some(interface.ty) } else { None })
}

fn type_signature(ty: &Type) -> String {
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
        Type::TypeDef(row, generics) => type_def_signature(*row, generics),
        rest => unimplemented!("{rest:?}"),
    }
}

pub fn type_def_signature(row: TypeDef, generics: &[Type]) -> String {
    match row.kind() {
        TypeKind::Interface => type_def_interface_signature(row, generics),
        TypeKind::Class => {
            if let Some(Type::TypeDef(default, generics)) = type_def_default_interface(row) {
                format!("rc({};{})", row.type_name(), type_def_interface_signature(default, &generics))
            } else {
                unimplemented!();
            }
        }
        TypeKind::Enum => format!("enum({};{})", row.type_name(), type_signature(&row.underlying_type())),
        TypeKind::Struct => {
            let mut result = format!("struct({}", row.type_name());
            for field in row.fields() {
                result.push(';');
                result.push_str(&type_signature(&field.ty(Some(row))));
            }
            result.push(')');
            result
        }
        TypeKind::Delegate => {
            if generics.is_empty() {
                format!("delegate({})", type_def_interface_signature(row, generics))
            } else {
                type_def_interface_signature(row, generics)
            }
        }
    }
}

fn type_def_interface_signature(row: TypeDef, generics: &[Type]) -> String {
    let guid = type_def_guid(row).unwrap();
    if generics.is_empty() {
        format!("{{{guid:#?}}}")
    } else {
        let mut result = format!("pinterface({{{guid:#?}}}");
        for generic in generics {
            result.push(';');
            result.push_str(&type_signature(generic));
        }
        result.push(')');
        result
    }
}

pub fn type_def_is_handle(row: TypeDef) -> bool {
    row.has_attribute("NativeTypedefAttribute")
}

pub fn type_def_guid(row: TypeDef) -> Option<Guid> {
    row.find_attribute("GuidAttribute").map(|attribute| Guid::from_args(&attribute.args()))
}

pub fn type_def_bases(mut row: TypeDef) -> Vec<TypeDef> {
    let mut bases = Vec::new();
    loop {
        match row.extends() {
            Some(base) if base != TypeName::Object => {
                row = row.reader().get_type_def(base.namespace, base.name).next().expect("Type not found");
                bases.push(row);
            }
            _ => break,
        }
    }
    bases
}

pub fn type_def_invalid_values(row: TypeDef) -> Vec<i64> {
    let mut values = Vec::new();
    for attribute in row.attributes() {
        if attribute.name() == "InvalidHandleValueAttribute" {
            if let Some((_, Value::I64(value))) = attribute.args().first() {
                values.push(*value);
            }
        }
    }
    values
}

fn type_def_is_nullable(row: TypeDef) -> bool {
    match row.kind() {
        TypeKind::Interface | TypeKind::Class => true,
        // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid them
        // from being doubly wrapped in `Option`.
        TypeKind::Delegate => row.flags().contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_is_nullable(ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_nullable(*row),
        Type::IInspectable | Type::IUnknown => true,
        _ => false,
    }
}

pub fn type_def_vtables(row: TypeDef) -> Vec<Type> {
    let mut result = Vec::new();
    if row.flags().contains(TypeAttributes::WindowsRuntime) {
        result.push(Type::IUnknown);
        if row.kind() != TypeKind::Delegate {
            result.push(Type::IInspectable);
        }
    } else {
        let mut next = row;
        while let Some(base) = next.interface_impls().map(move |imp| imp.ty(&[])).next() {
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
