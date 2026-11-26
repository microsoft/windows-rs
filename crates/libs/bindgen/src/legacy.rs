//! Projection rules for the legacy Win32 metadata used by the `windows` 0.62 releases. See
//! [`Reader::legacy`]. These reproduce the heuristics of that generator so the compatibility
//! packages keep their published signatures.

use super::*;

/// Native size estimate used by the legacy return-value heuristic. Pointers and other
/// unlisted types count as four bytes.
fn size(ty: &Type, reader: &Reader) -> usize {
    match ty {
        Type::I8 | Type::U8 => 1,
        Type::I16 | Type::U16 => 2,
        Type::I64 | Type::U64 | Type::F64 => 8,
        Type::GUID => 16,
        Type::ArrayFixed(ty, len) => size(ty, reader) * len,
        Type::CppStruct(ty) => {
            let fields = ty
                .def
                .fields()
                .map(|field| field.field_type(Some(ty), reader));
            if ty.def.flags().contains(TypeAttributes::ExplicitLayout) {
                fields.map(|ty| size(&ty, reader)).max().unwrap_or(1)
            } else {
                fields.fold(0, |sum, ty| {
                    let align = align(&ty, reader);
                    ((sum + (align - 1)) & !(align - 1)) + size(&ty, reader)
                })
            }
        }
        Type::Struct(ty) => ty
            .def
            .fields()
            .map(|field| field.field_type(None, reader))
            .fold(0, |sum, ty| {
                let align = align(&ty, reader);
                ((sum + (align - 1)) & !(align - 1)) + size(&ty, reader)
            }),
        Type::CppEnum(ty) => size(&ty.def.underlying_type_ext(reader), reader),
        _ => 4,
    }
}

fn align(ty: &Type, reader: &Reader) -> usize {
    match ty {
        Type::I8 | Type::U8 => 1,
        Type::I16 | Type::U16 => 2,
        Type::I64 | Type::U64 | Type::F64 => 8,
        Type::ArrayFixed(ty, len) => align(ty, reader) * len,
        Type::CppStruct(ty) => ty
            .def
            .fields()
            .map(|field| align(&field.field_type(Some(ty), reader), reader))
            .max()
            .unwrap_or(1),
        Type::Struct(ty) => ty
            .def
            .fields()
            .map(|field| align(&field.field_type(None, reader), reader))
            .max()
            .unwrap_or(1),
        Type::CppEnum(ty) => align(&ty.def.underlying_type_ext(reader), reader),
        _ => 4,
    }
}

fn is_byte_size(ty: &Type) -> bool {
    match ty {
        Type::PtrConst(ty, _) | Type::PtrMut(ty, _) => is_byte_size(ty),
        Type::I8 | Type::U8 | Type::PSTR | Type::PCSTR => true,
        _ => false,
    }
}

fn is_void(ty: &Type) -> bool {
    match ty {
        Type::PtrConst(ty, _) | Type::PtrMut(ty, _) => is_void(ty),
        Type::Void => true,
        _ => false,
    }
}

fn is_array_attribute(param: &Param) -> bool {
    param.attributes().any(|attribute| {
        matches!(
            attribute.name(),
            "NativeArrayInfoAttribute" | "MemorySizeAttribute"
        )
    })
}

/// The trailing parameter is the result when it has `RetValAttribute`, or when it is a small,
/// required, output-only pointer that follows only input parameters.
pub fn is_retval(signature: &Signature, reader: &Reader) -> bool {
    let Some((last, rest)) = signature.params.split_last() else {
        return false;
    };

    if last.is_retval_attribute() {
        return true;
    }

    let Some(def) = last.def else {
        return false;
    };

    let flags = def.flags();

    last.ty.is_pointer()
        && !is_void(&last.ty)
        && !flags.contains(windows_metadata::ParamAttributes::In)
        && flags.contains(windows_metadata::ParamAttributes::Out)
        && !flags.contains(windows_metadata::ParamAttributes::Optional)
        && !is_array_attribute(last)
        && size(&last.ty.deref(), reader) <= 16
        && rest.iter().all(Param::is_input_only)
}

/// Returns `Result<()>` for `HRESULT` and for `BOOL` with `SetLastError`. Methods that may return
/// several success codes keep the native result.
pub fn return_hint(
    def: MethodDef,
    signature: &Signature,
    is_retval: bool,
    reader: &Reader,
) -> ReturnHint {
    if def.has_attribute("CanReturnMultipleSuccessValuesAttribute") {
        return ReturnHint::None;
    }

    match signature.return_type.projection_type(reader) {
        Type::Void if is_retval => ReturnHint::ReturnValue,
        Type::HRESULT => {
            if signature.params.len() >= 2
                && let Some((guid, object)) = signature_param_is_query(&signature.params, reader)
            {
                if signature.params[object].is_optional_or_reserved() {
                    ReturnHint::QueryOptional(object, guid)
                } else {
                    ReturnHint::Query(object, guid)
                }
            } else if is_retval {
                ReturnHint::ResultValue
            } else {
                ReturnHint::ResultVoid
            }
        }
        Type::BOOL if supports_last_error(def) => ReturnHint::ResultVoid,
        Type::GUID => ReturnHint::ReturnStruct,
        Type::CppStruct(ty) if !ty.is_handle(reader) => ReturnHint::ReturnStruct,
        _ => ReturnHint::None,
    }
}

fn supports_last_error(def: MethodDef) -> bool {
    def.impl_map().is_some_and(|map| {
        map.flags()
            .contains(windows_metadata::PInvokeAttributes::SupportsLastError)
    })
}

/// Whether a function returning a handle reports failure as an invalid handle with the thread's
/// last error. `LocalFree` returns null on success.
pub fn handle_last_error(def: MethodDef, signature: &Signature, reader: &Reader) -> bool {
    if def.name() == "LocalFree" || !supports_last_error(def) {
        return false;
    }

    let Type::CppStruct(ty) = &signature.return_type else {
        return false;
    };

    ty.is_handle(reader)
        && (ty.def.underlying_type_ext(reader).is_pointer()
            || ty.def.has_attribute("InvalidHandleValueAttribute"))
}

/// The legacy packages link these COM functions from `combase.dll`, where `ole32.dll` forwards
/// them.
pub fn module_name(def: MethodDef) -> String {
    if matches!(
        def.name(),
        "CoCreateFreeThreadedMarshaler"
            | "CoIncrementMTAUsage"
            | "CoTaskMemAlloc"
            | "CoTaskMemFree"
            | "RoGetAgileReference"
    ) {
        "combase.dll".to_string()
    } else {
        def.module_name()
    }
}

/// The legacy `windows` crate implements `Clone`, `Drop`, and conversions for these variant
/// types by hand, since they own the values they hold.
pub fn is_variant(name: TypeName) -> bool {
    name == TypeName("Windows.Win32.System.Variant", "VARIANT")
        || name == TypeName("Windows.Win32.System.Com.StructuredStorage", "PROPVARIANT")
}

/// Resolves the enum named by a legacy `AssociatedEnumAttribute` in the method's namespace.
pub fn associated_enum(param: MethodParam, namespace: &str, reader: &Reader) -> Option<Type> {
    let attribute = param
        .attributes()
        .find(|attribute| attribute.name() == "AssociatedEnumAttribute")?;
    let values = attribute.value();
    let [(_, Value::Utf8(name))] = values.as_slice() else {
        panic!("AssociatedEnumAttribute requires one string argument");
    };
    let ty = reader.with_full_name(namespace, name).next();
    assert!(
        matches!(ty, Some(Type::CppEnum(_))),
        "AssociatedEnumAttribute target `{namespace}.{name}` must be a Win32 enum"
    );
    ty
}

fn array_hint(param: &Param) -> ParamHint {
    for attribute in param.attributes() {
        match attribute.name() {
            "NativeArrayInfoAttribute" => {
                for (_, value) in attribute.value() {
                    match value {
                        Value::I16(value) => {
                            return ParamHint::ArrayRelativeLen(value as usize);
                        }
                        Value::I32(value) => return ParamHint::ArrayFixed(value as usize),
                        _ => {}
                    }
                }
            }
            "MemorySizeAttribute" => {
                for (_, value) in attribute.value() {
                    if let Value::I16(value) = value {
                        return ParamHint::ArrayRelativeByteLen(value as usize);
                    }
                }
            }
            _ => {}
        }
    }

    ParamHint::None
}

/// Projects array parameters as slices. A relative length must be a distinct, input-only,
/// non-pointer parameter that describes a single array; it is then computed from the slice.
pub fn array_hints(signature: &Signature) -> Vec<ParamHint> {
    let params = &signature.params;
    let mut hints: Vec<_> = params.iter().map(array_hint).collect();

    for position in 0..hints.len() {
        if let ParamHint::ArrayRelativeLen(relative) | ParamHint::ArrayRelativeByteLen(relative) =
            hints[position]
        {
            if position != relative
                && params
                    .get(relative)
                    .is_some_and(|len| len.is_input_only() && !len.ty.is_pointer())
            {
                hints[relative] = ParamHint::ArrayRelativePtr(position);
            } else {
                hints[position] = ParamHint::None;
            }
        }
    }

    let mut sets = BTreeMap::<usize, Vec<usize>>::new();

    for (position, hint) in hints.iter().enumerate() {
        if let ParamHint::ArrayRelativeLen(relative) | ParamHint::ArrayRelativeByteLen(relative) =
            hint
        {
            sets.entry(*relative).or_default().push(position);
        }
    }

    for (len, ptrs) in sets {
        if ptrs.len() > 1 {
            hints[len] = ParamHint::None;
            for ptr in ptrs {
                hints[ptr] = ParamHint::None;
            }
        }
    }

    for position in 0..hints.len() {
        if let ParamHint::ArrayRelativeByteLen(relative) = hints[position]
            && !is_byte_size(&params[position].ty)
        {
            hints[position] = ParamHint::None;
            hints[relative] = ParamHint::None;
        }
    }

    hints
}
