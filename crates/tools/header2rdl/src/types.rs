/// Maps Clang types and calling conventions to `IrType` / `IrAbi`.
use crate::ir::{IrAbi, IrPrimitive, IrType};
use clang::{CallingConvention, Type, TypeKind};

/// Convert a Clang `Type` to an `IrType`.
///
/// `known_names` is the set of type names that the collector has already seen in
/// the current translation unit, allowing us to emit `IrType::Named` for them
/// instead of falling back to a pointer/opaque representation.
pub fn map_type(ty: Type<'_>, known_names: &std::collections::HashSet<String>) -> IrType {
    let ty = ty.get_canonical_type();
    map_canonical(ty, known_names)
}

fn map_canonical(ty: Type<'_>, known_names: &std::collections::HashSet<String>) -> IrType {
    match ty.get_kind() {
        TypeKind::Void => IrType::Void,

        TypeKind::Bool => IrType::Primitive(IrPrimitive::Bool),

        TypeKind::CharS | TypeKind::SChar => IrType::Primitive(IrPrimitive::I8),
        TypeKind::CharU | TypeKind::UChar => IrType::Primitive(IrPrimitive::U8),

        TypeKind::Short => IrType::Primitive(IrPrimitive::I16),
        TypeKind::UShort => IrType::Primitive(IrPrimitive::U16),

        // `int` and `long` are both 32-bit on all Windows targets.
        TypeKind::Int | TypeKind::Long => IrType::Primitive(IrPrimitive::I32),
        TypeKind::UInt | TypeKind::ULong => IrType::Primitive(IrPrimitive::U32),

        TypeKind::LongLong => IrType::Primitive(IrPrimitive::I64),
        TypeKind::ULongLong => IrType::Primitive(IrPrimitive::U64),

        TypeKind::Float => IrType::Primitive(IrPrimitive::F32),
        TypeKind::Double => IrType::Primitive(IrPrimitive::F64),

        TypeKind::Pointer => {
            let pointee = ty
                .get_pointee_type()
                .expect("pointer type must have pointee");
            let is_const = pointee.is_const_qualified();
            let inner = map_canonical(pointee.get_canonical_type(), known_names);
            IrType::Ptr {
                is_const,
                pointee: Box::new(inner),
            }
        }

        TypeKind::ConstantArray => {
            let elem = ty
                .get_element_type()
                .expect("array type must have element type");
            let size = ty.get_size().unwrap_or(0) as u64;
            IrType::Array {
                elem: Box::new(map_canonical(elem.get_canonical_type(), known_names)),
                size,
            }
        }

        TypeKind::IncompleteArray => {
            // Treat as pointer to element.
            let elem = ty
                .get_element_type()
                .expect("array type must have element type");
            IrType::Ptr {
                is_const: false,
                pointee: Box::new(map_canonical(elem.get_canonical_type(), known_names)),
            }
        }

        TypeKind::Record | TypeKind::Enum => {
            // Try to get the display name of the underlying declaration.
            if let Some(decl) = ty.get_declaration() {
                let name = decl.get_name().unwrap_or_default();
                if !name.is_empty() {
                    return IrType::Named(name);
                }
            }
            // Anonymous or unresolvable — use a void pointer as a fallback.
            IrType::Ptr {
                is_const: false,
                pointee: Box::new(IrType::Void),
            }
        }

        TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype => {
            // Pointer to a function type — emit as a `*mut void` (opaque) with a note,
            // since we can't name anonymous function pointers in RDL.
            IrType::Ptr {
                is_const: false,
                pointee: Box::new(IrType::Void),
            }
        }

        _ => {
            // Unknown or exotic type — fall back to opaque void pointer.
            IrType::Ptr {
                is_const: false,
                pointee: Box::new(IrType::Void),
            }
        }
    }
}

/// Convert a Clang calling convention to `IrAbi`.
pub fn map_calling_convention(cc: CallingConvention) -> IrAbi {
    match cc {
        CallingConvention::Stdcall | CallingConvention::Win64 => IrAbi::System,
        CallingConvention::Fastcall => IrAbi::Fastcall,
        _ => IrAbi::C,
    }
}

/// Attempt to identify a well-known Windows typedef name and map it directly to
/// an `IrType` without going through canonical form (which would lose the name).
///
/// Returns `None` if the name is not recognised.
pub fn map_well_known(name: &str) -> Option<IrType> {
    match name {
        // Pointer-sized types
        "SIZE_T" | "ULONG_PTR" | "DWORD_PTR" | "UINT_PTR" | "WPARAM" | "LPARAM" => {
            Some(IrType::Primitive(IrPrimitive::Usize))
        }
        "SSIZE_T" | "LONG_PTR" | "INT_PTR" => Some(IrType::Primitive(IrPrimitive::Isize)),

        // Common Windows aliases
        "BOOL" | "LONG" | "INT" | "HRESULT" | "NTSTATUS" => {
            Some(IrType::Primitive(IrPrimitive::I32))
        }
        "DWORD" | "ULONG" | "UINT" | "COLORREF" | "LCTYPE" | "LCID" | "LANGID" => {
            Some(IrType::Primitive(IrPrimitive::U32))
        }
        "WORD" | "WCHAR" | "ATOM" => Some(IrType::Primitive(IrPrimitive::U16)),
        "BYTE" | "UCHAR" | "BOOLEAN" => Some(IrType::Primitive(IrPrimitive::U8)),
        "CHAR" | "CCHAR" => Some(IrType::Primitive(IrPrimitive::I8)),
        "SHORT" => Some(IrType::Primitive(IrPrimitive::I16)),
        "USHORT" => Some(IrType::Primitive(IrPrimitive::U16)),
        "LONGLONG" | "INT64" | "LARGE_INTEGER" => Some(IrType::Primitive(IrPrimitive::I64)),
        "ULONGLONG" | "UINT64" | "DWORD64" | "ULONG64" => Some(IrType::Primitive(IrPrimitive::U64)),
        "FLOAT" => Some(IrType::Primitive(IrPrimitive::F32)),

        _ => None,
    }
}
