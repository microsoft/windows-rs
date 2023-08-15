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

pub fn type_def_invoke_method(reader: &Reader, row: TypeDef) -> MethodDef {
    reader.type_def_methods(row).find(|method| reader.method_def_name(*method) == "Invoke").expect("`Invoke` method not found")
}

pub fn type_def_generics(reader: &Reader, def: TypeDef) -> Vec<Type> {
    reader.type_def_generics(def).map(Type::GenericParam).collect()
}

// TODO: namespace should not be required - it's a hack to accomodate Win32 metadata
// TODO: this is very Rust-specific and Win32-metadata specific with all of its translation. Replace with literal signature parser that just returns slice of types.
pub fn method_def_signature(reader: &Reader, namespace: &str, row: MethodDef, generics: &[Type]) -> Signature {
    let mut blob = reader.row_blob(row, 4);
    let call_flags = MethodCallAttributes(blob.read_usize() as u8);
    let _param_count = blob.read_usize();
    let mut return_type = reader.type_from_blob(&mut blob, None, generics);

    let mut params: Vec<SignatureParam> = reader
        .method_def_params(row)
        .filter_map(|param| {
            let param_is_const = reader.has_attribute(param, "ConstAttribute");
            if reader.param_sequence(param) == 0 {
                if param_is_const {
                    return_type = return_type.clone().to_const_type();
                }
                None
            } else {
                let is_output = reader.param_flags(param).contains(ParamAttributes::Out);
                let mut ty = reader.type_from_blob(&mut blob, None, generics);

                if let Some(name) = param_or_enum(reader, param) {
                    let def = reader.get_type_def(TypeName::new(namespace, &name)).next().expect("Enum not found");
                    ty = Type::PrimitiveOrEnum(Box::new(ty), Box::new(Type::TypeDef(def, Vec::new())));
                }

                if param_is_const || !is_output {
                    ty = ty.to_const_type();
                }
                if !is_output {
                    ty = ty.to_const_ptr();
                }
                let kind = param_kind(reader, param);
                Some(SignatureParam { def: param, ty, kind })
            }
        })
        .collect();

    for position in 0..params.len() {
        // Point len params back to the corresponding ptr params.
        match params[position].kind {
            SignatureParamKind::ArrayRelativeLen(relative) | SignatureParamKind::ArrayRelativeByteLen(relative) => {
                // The len params must be input only.
                if !reader.param_flags(params[relative].def).contains(ParamAttributes::Out) && position != relative && !params[relative].ty.is_pointer() {
                    params[relative].kind = SignatureParamKind::ArrayRelativePtr(position);
                } else {
                    params[position].kind = SignatureParamKind::Other;
                }
            }
            SignatureParamKind::ArrayFixed(_) => {
                if reader.has_attribute(params[position].def, "FreeWithAttribute") {
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
            if signature_param_is_convertible(reader, param) {
                if type_is_non_exclusive_winrt_interface(reader, &param.ty) {
                    param.kind = SignatureParamKind::TryInto;
                } else {
                    param.kind = SignatureParamKind::IntoParam;
                }
            } else {
                let flags = reader.param_flags(param.def);
                if param.ty.is_pointer() && (flags.contains(ParamAttributes::Optional) || reader.has_attribute(param.def, "ReservedAttribute")) {
                    param.kind = SignatureParamKind::OptionalPointer;
                } else if type_is_primitive(reader, &param.ty) && (!param.ty.is_pointer() || type_is_blittable(reader, &param.ty.deref())) {
                    param.kind = SignatureParamKind::ValueType;
                } else if type_is_blittable(reader, &param.ty) {
                    param.kind = SignatureParamKind::Blittable;
                }
            }
        }
    }

    Signature { def: row, params, return_type, call_flags }
}

fn param_kind(reader: &Reader, row: Param) -> SignatureParamKind {
    for attribute in reader.attributes(row) {
        match reader.attribute_name(attribute) {
            "NativeArrayInfoAttribute" => {
                for (_, value) in reader.attribute_args(attribute) {
                    match value {
                        Value::I16(value) => return SignatureParamKind::ArrayRelativeLen(value as usize),
                        Value::I32(value) => return SignatureParamKind::ArrayFixed(value as usize),
                        _ => {}
                    }
                }
            }
            "MemorySizeAttribute" => {
                for (_, value) in reader.attribute_args(attribute) {
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
fn param_or_enum(reader: &Reader, row: Param) -> Option<String> {
    reader.find_attribute(row, "AssociatedEnumAttribute").and_then(|attribute| {
        for (_, arg) in reader.attribute_args(attribute) {
            if let Value::String(name) = arg {
                return Some(name);
            }
        }
        None
    })
}

pub fn signature_param_is_borrowed(reader: &Reader, param: &SignatureParam) -> bool {
    type_is_borrowed(reader, &param.ty)
}

pub fn signature_param_is_convertible(reader: &Reader, param: &SignatureParam) -> bool {
    !reader.param_flags(param.def).contains(ParamAttributes::Out) && !param.ty.is_winrt_array() && !param.ty.is_pointer() && !param.kind.is_array() && (type_is_borrowed(reader, &param.ty) || type_is_non_exclusive_winrt_interface(reader, &param.ty) || type_is_trivially_convertible(reader, &param.ty))
}

fn signature_param_is_retval(reader: &Reader, param: &SignatureParam) -> bool {
    // The Win32 metadata uses `RetValAttribute` to call out retval methods but it is employed
    // very sparingly, so this heuristic is used to apply the transformation more uniformly.
    if reader.has_attribute(param.def, "RetValAttribute") {
        return true;
    }
    if !param.ty.is_pointer() {
        return false;
    }
    if param.ty.is_void() {
        return false;
    }
    let flags = reader.param_flags(param.def);
    if flags.contains(ParamAttributes::In) || !flags.contains(ParamAttributes::Out) || flags.contains(ParamAttributes::Optional) || param.kind.is_array() {
        return false;
    }
    if param_kind(reader, param.def).is_array() {
        return false;
    }
    // If it's bigger than 128 bits, best to pass as a reference.
    if reader.type_size(&param.ty.deref()) > 16 {
        return false;
    }
    // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid
    // producing the `Result<Option<T>>` anti-pattern.
    !type_is_callback(reader, &param.ty.deref())
}

pub fn signature_kind(reader: &Reader, signature: &Signature) -> SignatureKind {
    if reader.has_attribute(signature.def, "CanReturnMultipleSuccessValuesAttribute") {
        return SignatureKind::PreserveSig;
    }
    match &signature.return_type {
        Type::Void if signature_is_retval(reader, signature) => SignatureKind::ReturnValue,
        Type::Void => SignatureKind::ReturnVoid,
        Type::HRESULT => {
            if signature.params.len() >= 2 {
                if let Some(guid) = signature_param_is_query_guid(reader, &signature.params) {
                    if let Some(object) = signature_param_is_query_object(reader, &signature.params) {
                        if reader.param_flags(signature.params[object].def).contains(ParamAttributes::Optional) {
                            return SignatureKind::QueryOptional(QueryPosition { object, guid });
                        } else {
                            return SignatureKind::Query(QueryPosition { object, guid });
                        }
                    }
                }
            }
            if signature_is_retval(reader, signature) {
                SignatureKind::ResultValue
            } else {
                SignatureKind::ResultVoid
            }
        }
        Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::NTSTATUS => SignatureKind::ResultVoid,
        Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::WIN32_ERROR => SignatureKind::ResultVoid,
        Type::TypeDef(def, _) if reader.type_def_type_name(*def) == TypeName::BOOL && method_def_last_error(reader, signature.def) => SignatureKind::ResultVoid,
        _ if type_is_struct(reader, &signature.return_type) => SignatureKind::ReturnStruct,
        _ => SignatureKind::PreserveSig,
    }
}

fn signature_is_retval(reader: &Reader, signature: &Signature) -> bool {
    signature.params.last().map_or(false, |param| signature_param_is_retval(reader, param))
        && signature.params[..signature.params.len() - 1].iter().all(|param| {
            let flags = reader.param_flags(param.def);
            !flags.contains(ParamAttributes::Out)
        })
}

fn signature_param_is_query_guid(reader: &Reader, params: &[SignatureParam]) -> Option<usize> {
    params.iter().rposition(|param| param.ty == Type::ConstPtr(Box::new(Type::GUID), 1) && !reader.param_flags(param.def).contains(ParamAttributes::Out))
}

fn signature_param_is_query_object(reader: &Reader, params: &[SignatureParam]) -> Option<usize> {
    params.iter().rposition(|param| param.ty == Type::MutPtr(Box::new(Type::Void), 2) && reader.has_attribute(param.def, "ComOutPtrAttribute"))
}

fn method_def_last_error(reader: &Reader, row: MethodDef) -> bool {
    if let Some(map) = reader.method_def_impl_map(row) {
        reader.impl_map_flags(map).contains(PInvokeAttributes::SupportsLastError)
    } else {
        false
    }
}

fn type_is_borrowed(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => !type_def_is_blittable(reader, *row),
        Type::BSTR | Type::PCSTR | Type::PCWSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => true,
        _ => false,
    }
}

pub fn type_is_non_exclusive_winrt_interface(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => {
            let flags = reader.type_def_flags(*row);
            if !flags.contains(TypeAttributes::WindowsRuntime) {
                false
            } else {
                match reader.type_def_kind(*row) {
                    TypeKind::Interface => !type_def_is_exclusive(reader, *row),
                    TypeKind::Class => reader.has_attribute(*row, "ComposableAttribute"),
                    _ => false,
                }
            }
        }
        _ => false,
    }
}

fn type_is_trivially_convertible(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => match reader.type_def_kind(*row) {
            TypeKind::Struct => type_def_is_handle(reader, *row),
            _ => false,
        },
        Type::PCSTR | Type::PCWSTR => true,
        _ => false,
    }
}

fn type_is_callback(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_callback(reader, *row),
        _ => false,
    }
}

fn type_def_is_callback(reader: &Reader, row: TypeDef) -> bool {
    !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) && reader.type_def_kind(row) == TypeKind::Delegate
}

pub fn type_has_callback(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_callback(reader, *row),
        Type::Win32Array(ty, _) => type_has_callback(reader, ty),
        _ => false,
    }
}
pub fn type_def_has_callback(reader: &Reader, row: TypeDef) -> bool {
    if type_def_is_callback(reader, row) {
        return true;
    }
    if reader.type_def_kind(row) != TypeKind::Struct {
        return false;
    }
    fn check(reader: &Reader, row: TypeDef) -> bool {
        if reader.type_def_fields(row).any(|field| type_has_callback(reader, &reader.field_type(field, Some(row)))) {
            return true;
        }
        false
    }
    let type_name = reader.type_def_type_name(row);
    if type_name.namespace.is_empty() {
        check(reader, row)
    } else {
        for row in reader.get_type_def(type_name) {
            if check(reader, row) {
                return true;
            }
        }
        false
    }
}

pub fn type_interfaces(reader: &Reader, ty: &Type) -> Vec<Interface> {
    // TODO: collect into btree map and then return collected vec
    // This will both sort the results and should make finding dupes faster
    fn walk(reader: &Reader, result: &mut Vec<Interface>, parent: &Type, is_base: bool) {
        if let Type::TypeDef(row, generics) = parent {
            for imp in reader.type_def_interface_impls(*row) {
                let mut child = Interface {
                    ty: reader.interface_impl_type(imp, generics),
                    kind: if reader.has_attribute(imp, "DefaultAttribute") { InterfaceKind::Default } else { InterfaceKind::None },
                };

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
    walk(reader, &mut result, ty, false);
    if let Type::TypeDef(row, _) = ty {
        if reader.type_def_kind(*row) == TypeKind::Class {
            for base in type_def_bases(reader, *row) {
                walk(reader, &mut result, &Type::TypeDef(base, Vec::new()), true);
            }
            for attribute in reader.attributes(*row) {
                match reader.attribute_name(attribute) {
                    "StaticAttribute" | "ActivatableAttribute" => {
                        for (_, arg) in reader.attribute_args(attribute) {
                            if let Value::TypeName(type_name) = arg {
                                let def = reader.get_type_def(TypeName::parse(&type_name)).next().expect("Type not found");
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
    result.sort_by(|a, b| type_name(reader, &a.ty).cmp(type_name(reader, &b.ty)));
    result
}

fn type_name<'a>(reader: &Reader<'a>, ty: &Type) -> &'a str {
    match ty {
        Type::TypeDef(row, _) => reader.type_def_name(*row),
        _ => "",
    }
}

pub fn type_def_async_kind(reader: &Reader, row: TypeDef) -> AsyncKind {
    match reader.type_def_type_name(row) {
        TypeName::IAsyncAction => AsyncKind::Action,
        TypeName::IAsyncActionWithProgress => AsyncKind::ActionWithProgress,
        TypeName::IAsyncOperation => AsyncKind::Operation,
        TypeName::IAsyncOperationWithProgress => AsyncKind::OperationWithProgress,
        _ => AsyncKind::None,
    }
}

pub fn field_is_blittable(reader: &Reader, row: Field, enclosing: TypeDef) -> bool {
    type_is_blittable(reader, &reader.field_type(row, Some(enclosing)))
}

pub fn field_is_copyable(reader: &Reader, row: Field, enclosing: TypeDef) -> bool {
    type_is_copyable(reader, &reader.field_type(row, Some(enclosing)))
}

pub fn field_guid(reader: &Reader, row: Field) -> Option<GUID> {
    reader.find_attribute(row, "GuidAttribute").map(|attribute| GUID::from_args(&reader.attribute_args(attribute)))
}

pub fn field_is_ansi(reader: &Reader, row: Field) -> bool {
    reader.find_attribute(row, "NativeEncodingAttribute").is_some_and(|attribute| matches!(reader.attribute_args(attribute).get(0), Some((_, Value::String(encoding))) if encoding == "ansi"))
}

pub fn type_is_blittable(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_blittable(reader, *row),
        Type::String | Type::BSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
        Type::Win32Array(kind, _) => type_is_blittable(reader, kind),
        Type::WinrtArray(kind) => type_is_blittable(reader, kind),
        _ => true,
    }
}

fn type_is_copyable(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_copyable(reader, *row),
        Type::String | Type::BSTR | Type::IInspectable | Type::IUnknown | Type::GenericParam(_) => false,
        Type::Win32Array(kind, _) => type_is_copyable(reader, kind),
        Type::WinrtArray(kind) => type_is_copyable(reader, kind),
        _ => true,
    }
}

pub fn type_def_is_blittable(reader: &Reader, row: TypeDef) -> bool {
    match reader.type_def_kind(row) {
        TypeKind::Struct => {
            if reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
                reader.type_def_fields(row).all(|field| field_is_blittable(reader, field, row))
            } else {
                true
            }
        }
        TypeKind::Enum => true,
        TypeKind::Delegate => !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_def_is_copyable(reader: &Reader, row: TypeDef) -> bool {
    match reader.type_def_kind(row) {
        TypeKind::Struct => reader.type_def_fields(row).all(|field| field_is_copyable(reader, field, row)),
        TypeKind::Enum => true,
        TypeKind::Delegate => !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn method_def_special_name(reader: &Reader, row: MethodDef) -> String {
    let name = reader.method_def_name(row);
    if reader.method_def_flags(row).contains(MethodAttributes::SpecialName) {
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
        if let Some(attribute) = reader.find_attribute(row, "OverloadAttribute") {
            for (_, arg) in reader.attribute_args(attribute) {
                if let Value::String(name) = arg {
                    return name;
                }
            }
        }
        name.to_string()
    }
}

pub fn method_def_static_lib(reader: &Reader, row: MethodDef) -> Option<String> {
    reader.find_attribute(row, "StaticLibraryAttribute").and_then(|attribute| {
        let args = reader.attribute_args(attribute);
        if let Value::String(value) = &args[0].1 {
            return Some(value.clone());
        }
        None
    })
}

pub fn method_def_extern_abi(reader: &Reader, def: MethodDef) -> &'static str {
    let impl_map = reader.method_def_impl_map(def).expect("ImplMap not found");
    let flags = reader.impl_map_flags(impl_map);

    if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
        "system"
    } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
        "cdecl"
    } else {
        unimplemented!()
    }
}

pub fn type_def_has_default_constructor(reader: &Reader, row: TypeDef) -> bool {
    for attribute in reader.attributes(row) {
        if reader.attribute_name(attribute) == "ActivatableAttribute" {
            if reader.attribute_args(attribute).iter().any(|arg| matches!(arg.1, Value::TypeName(_))) {
                continue;
            } else {
                return true;
            }
        }
    }
    false
}

pub fn type_def_has_default_interface(reader: &Reader, row: TypeDef) -> bool {
    reader.type_def_interface_impls(row).any(|imp| reader.has_attribute(imp, "DefaultAttribute"))
}

pub fn type_def_is_exclusive(reader: &Reader, row: TypeDef) -> bool {
    reader.has_attribute(row, "ExclusiveToAttribute")
}

pub fn type_is_exclusive(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_exclusive(reader, *row),
        _ => false,
    }
}

pub fn type_is_struct(reader: &Reader, ty: &Type) -> bool {
    // This check is used to detect virtual functions that return C-style PODs that affect how the stack is packed for x86.
    // It could be defined as a struct with more than one field but that check is complicated as it would have to detect
    // nested structs. Fortunately, this is rare enough that this check is sufficient.
    match ty {
        Type::TypeDef(row, _) => reader.type_def_kind(*row) == TypeKind::Struct && !type_def_is_handle(reader, *row),
        Type::GUID => true,
        _ => false,
    }
}

fn type_def_is_primitive(reader: &Reader, row: TypeDef) -> bool {
    match reader.type_def_kind(row) {
        TypeKind::Enum => true,
        TypeKind::Struct => type_def_is_handle(reader, row),
        TypeKind::Delegate => !reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_is_primitive(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_primitive(reader, *row),
        Type::Bool | Type::Char | Type::I8 | Type::U8 | Type::I16 | Type::U16 | Type::I32 | Type::U32 | Type::I64 | Type::U64 | Type::F32 | Type::F64 | Type::ISize | Type::USize | Type::HRESULT | Type::ConstPtr(_, _) | Type::MutPtr(_, _) => true,
        _ => false,
    }
}

fn type_has_explicit_layout(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_explicit_layout(reader, *row),
        Type::Win32Array(ty, _) => type_has_explicit_layout(reader, ty),
        _ => false,
    }
}

pub fn type_def_has_explicit_layout(reader: &Reader, row: TypeDef) -> bool {
    if reader.type_def_kind(row) != TypeKind::Struct {
        return false;
    }
    fn check(reader: &Reader, row: TypeDef) -> bool {
        if reader.type_def_flags(row).contains(TypeAttributes::ExplicitLayout) {
            return true;
        }
        if reader.type_def_fields(row).any(|field| type_has_explicit_layout(reader, &reader.field_type(field, Some(row)))) {
            return true;
        }
        false
    }
    let type_name = reader.type_def_type_name(row);
    if type_name.namespace.is_empty() {
        check(reader, row)
    } else {
        for row in reader.get_type_def(type_name) {
            if check(reader, row) {
                return true;
            }
        }
        false
    }
}

fn type_has_packing(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_has_packing(reader, *row),
        Type::Win32Array(ty, _) => type_has_packing(reader, ty),
        _ => false,
    }
}

pub fn type_def_has_packing(reader: &Reader, row: TypeDef) -> bool {
    if reader.type_def_kind(row) != TypeKind::Struct {
        return false;
    }
    fn check(reader: &Reader, row: TypeDef) -> bool {
        if reader.type_def_class_layout(row).is_some() {
            return true;
        }
        if reader.type_def_fields(row).any(|field| type_has_packing(reader, &reader.field_type(field, Some(row)))) {
            return true;
        }
        false
    }
    let type_name = reader.type_def_type_name(row);
    if type_name.namespace.is_empty() {
        check(reader, row)
    } else {
        for row in reader.get_type_def(type_name) {
            if check(reader, row) {
                return true;
            }
        }
        false
    }
}

pub fn type_def_default_interface(reader: &Reader, row: TypeDef) -> Option<Type> {
    reader.type_def_interface_impls(row).find_map(move |row| if reader.has_attribute(row, "DefaultAttribute") { Some(reader.interface_impl_type(row, &[])) } else { None })
}

fn type_signature(reader: &Reader, ty: &Type) -> String {
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
        Type::TypeDef(row, generics) => type_def_signature(reader, *row, generics),
        rest => unimplemented!("{rest:?}"),
    }
}

pub fn type_def_signature(reader: &Reader, row: TypeDef, generics: &[Type]) -> String {
    match reader.type_def_kind(row) {
        TypeKind::Interface => type_def_interface_signature(reader, row, generics),
        TypeKind::Class => {
            if let Some(Type::TypeDef(default, generics)) = type_def_default_interface(reader, row) {
                format!("rc({};{})", reader.type_def_type_name(row), type_def_interface_signature(reader, default, &generics))
            } else {
                unimplemented!();
            }
        }
        TypeKind::Enum => format!("enum({};{})", reader.type_def_type_name(row), type_signature(reader, &reader.type_def_underlying_type(row))),
        TypeKind::Struct => {
            let mut result = format!("struct({}", reader.type_def_type_name(row));
            for field in reader.type_def_fields(row) {
                result.push(';');
                result.push_str(&type_signature(reader, &reader.field_type(field, Some(row))));
            }
            result.push(')');
            result
        }
        TypeKind::Delegate => {
            if generics.is_empty() {
                format!("delegate({})", type_def_interface_signature(reader, row, generics))
            } else {
                type_def_interface_signature(reader, row, generics)
            }
        }
    }
}

fn type_def_interface_signature(reader: &Reader, row: TypeDef, generics: &[Type]) -> String {
    let guid = type_def_guid(reader, row).unwrap();
    if generics.is_empty() {
        format!("{{{guid:#?}}}")
    } else {
        let mut result = format!("pinterface({{{guid:#?}}}");
        for generic in generics {
            result.push(';');
            result.push_str(&type_signature(reader, generic));
        }
        result.push(')');
        result
    }
}

pub fn type_def_is_handle(reader: &Reader, row: TypeDef) -> bool {
    reader.has_attribute(row, "NativeTypedefAttribute")
}

pub fn type_has_replacement(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::HRESULT | Type::PCSTR | Type::PCWSTR => true,
        Type::TypeDef(row, _) => type_def_is_handle(reader, *row) || reader.type_def_kind(*row) == TypeKind::Enum,
        _ => false,
    }
}

pub fn type_def_guid(reader: &Reader, row: TypeDef) -> Option<GUID> {
    reader.find_attribute(row, "GuidAttribute").map(|attribute| GUID::from_args(&reader.attribute_args(attribute)))
}

pub fn type_def_bases(reader: &Reader, mut row: TypeDef) -> Vec<TypeDef> {
    let mut bases = Vec::new();
    loop {
        match reader.type_def_extends(row) {
            Some(base) if base != TypeName::Object => {
                row = reader.get_type_def(base).next().expect("Type not found");
                bases.push(row);
            }
            _ => break,
        }
    }
    bases
}

pub fn type_def_is_agile(reader: &Reader, row: TypeDef) -> bool {
    for attribute in reader.attributes(row) {
        match reader.attribute_name(attribute) {
            "AgileAttribute" => return true,
            "MarshalingBehaviorAttribute" => {
                if let Some((_, Value::EnumDef(_, value))) = reader.attribute_args(attribute).get(0) {
                    if let Value::I32(2) = **value {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    matches!(reader.type_def_type_name(row), TypeName::IAsyncAction | TypeName::IAsyncActionWithProgress | TypeName::IAsyncOperation | TypeName::IAsyncOperationWithProgress)
}

pub fn type_def_invalid_values(reader: &Reader, row: TypeDef) -> Vec<i64> {
    let mut values = Vec::new();
    for attribute in reader.attributes(row) {
        if reader.attribute_name(attribute) == "InvalidHandleValueAttribute" {
            if let Some((_, Value::I64(value))) = reader.attribute_args(attribute).get(0) {
                values.push(*value);
            }
        }
    }
    values
}

pub fn type_def_usable_for(reader: &Reader, row: TypeDef) -> Option<TypeDef> {
    if let Some(attribute) = reader.find_attribute(row, "AlsoUsableForAttribute") {
        if let Some((_, Value::String(name))) = reader.attribute_args(attribute).get(0) {
            return reader.get_type_def(TypeName::new(reader.type_def_namespace(row), name.as_str())).next();
        }
    }
    None
}

fn type_def_is_nullable(reader: &Reader, row: TypeDef) -> bool {
    match reader.type_def_kind(row) {
        TypeKind::Interface | TypeKind::Class => true,
        // Win32 callbacks are defined as `Option<T>` so we don't include them here to avoid them
        // from being doubly wrapped in `Option`.
        TypeKind::Delegate => reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime),
        _ => false,
    }
}

pub fn type_is_nullable(reader: &Reader, ty: &Type) -> bool {
    match ty {
        Type::TypeDef(row, _) => type_def_is_nullable(reader, *row),
        Type::IInspectable | Type::IUnknown => true,
        _ => false,
    }
}

pub fn type_def_vtables(reader: &Reader, row: TypeDef) -> Vec<Type> {
    let mut result = Vec::new();
    if reader.type_def_flags(row).contains(TypeAttributes::WindowsRuntime) {
        result.push(Type::IUnknown);
        if reader.type_def_kind(row) != TypeKind::Delegate {
            result.push(Type::IInspectable);
        }
    } else {
        let mut next = row;
        while let Some(base) = type_def_interfaces(reader, next, &[]).next() {
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

pub fn type_def_interfaces<'a>(reader: &'a Reader<'a>, row: TypeDef, generics: &'a [Type]) -> impl Iterator<Item = Type> + 'a {
    reader.type_def_interface_impls(row).map(move |row| reader.interface_impl_type(row, generics))
}

pub fn type_underlying_type(reader: &Reader, ty: &Type) -> Type {
    match ty {
        Type::TypeDef(row, _) => reader.type_def_underlying_type(*row),
        Type::HRESULT => Type::I32,
        _ => ty.clone(),
    }
}
