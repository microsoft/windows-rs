use super::*;

/// Native C layout of a type on one architecture.
pub(super) struct NativeLayout {
    pub size: usize,
    pub alignment: usize,
    pub required: usize,
    pub offsets: Vec<usize>,
}

fn align_up(size: usize, alignment: usize) -> usize {
    size.div_ceil(alignment) * alignment
}

pub(super) fn native_layout(ty: &Type, reader: &Reader, arch: i32) -> NativeLayout {
    let pointer = if arch == X86 { 4 } else { 8 };
    let scalar = |size, alignment| NativeLayout {
        size,
        alignment,
        required: 0,
        offsets: Vec::new(),
    };
    match ty {
        Type::I8 | Type::U8 | Type::Bool => scalar(1, 1),
        Type::I16 | Type::U16 | Type::Char => scalar(2, 2),
        Type::I32
        | Type::U32
        | Type::F32
        | Type::HRESULT
        | Type::BOOL
        | Type::NTSTATUS
        | Type::RPC_STATUS => scalar(4, 4),
        Type::I64 | Type::U64 | Type::F64 | Type::Ptr64(..) => scalar(8, 8),
        Type::GUID => scalar(16, 4),
        Type::PtrMut(..)
        | Type::PtrConst(..)
        | Type::ISize
        | Type::USize
        | Type::FnPtr(..)
        | Type::CppDelegate(..)
        | Type::CppInterface(..)
        | Type::PSTR
        | Type::PCSTR
        | Type::PWSTR
        | Type::PCWSTR
        | Type::IUnknown
        | Type::BSTR
        | Type::String
        | Type::Object
        | Type::Interface(..)
        | Type::Class(..)
        | Type::Delegate(..) => scalar(pointer, pointer),
        Type::Enum(enumeration) => {
            native_layout(&enumeration.def.underlying_type_ext(reader), reader, arch)
        }
        Type::Struct(record) => {
            let mut size = 0;
            let mut alignment = 1;
            for field in record.def.fields() {
                let field = native_layout(&field.field_type(None, reader), reader, arch);
                size = align_up(size, field.alignment) + field.size;
                alignment = alignment.max(field.alignment);
            }
            scalar(align_up(size.max(1), alignment), alignment)
        }
        Type::ArrayFixed(element, count) => {
            let mut layout = native_layout(element, reader, arch);
            layout.size *= count;
            layout
        }
        Type::CppStruct(record) => {
            let full_name = record.def.full_name();
            let variants: Vec<_> = reader
                .with_full_name(&full_name.namespace, &full_name.name)
                .filter_map(|ty| match ty {
                    Type::CppStruct(record)
                        if record.def.arches() == 0 || record.def.arches() & arch != 0 =>
                    {
                        Some(record)
                    }
                    _ => None,
                })
                .collect();
            assert!(
                variants.len() <= 1,
                "ambiguous native layout for {full_name:?}"
            );
            let record = variants.first().unwrap_or(record);
            assert!(
                record.def.arches() == 0 || record.def.arches() & arch != 0,
                "unavailable native layout for {full_name:?}"
            );
            if record.is_native_typedef() {
                return native_layout(
                    &record
                        .def
                        .fields()
                        .next()
                        .unwrap()
                        .field_type(Some(record), reader),
                    reader,
                    arch,
                );
            }
            let packing = record
                .def
                .class_layout()
                .map(|layout| layout.packing_size() as usize)
                .filter(|packing| *packing != 0)
                .unwrap_or(usize::MAX);
            let mut result = NativeLayout {
                size: 0,
                alignment: 1,
                required: record.forced_align().unwrap_or(0),
                offsets: Vec::new(),
            };
            let union = record.def.flags().contains(TypeAttributes::ExplicitLayout);
            for field in record
                .def
                .fields()
                .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            {
                let field_layout =
                    native_layout(&field.field_type(Some(record), reader), reader, arch);
                let required = field_layout
                    .required
                    .max(member_alignment(field).unwrap_or(0));
                let alignment = field_layout.alignment.min(packing).max(required);
                let offset = if union {
                    0
                } else {
                    align_up(result.size, alignment)
                };
                result.offsets.push(offset);
                result.size = result.size.max(offset + field_layout.size);
                result.alignment = result.alignment.max(alignment);
                result.required = result.required.max(required);
            }
            result.alignment = result.alignment.max(result.required);
            result.size = align_up(result.size.max(1), result.alignment);
            result
        }
        Type::CppEnum(enumeration) => {
            let full_name = enumeration.def.full_name();
            let variants: Vec<_> = reader
                .with_full_name(&full_name.namespace, &full_name.name)
                .filter_map(|ty| match ty {
                    Type::CppEnum(enumeration)
                        if enumeration.def.arches() == 0
                            || enumeration.def.arches() & arch != 0 =>
                    {
                        Some(enumeration)
                    }
                    _ => None,
                })
                .collect();
            assert!(
                variants.len() <= 1,
                "ambiguous native enum layout for {full_name:?}"
            );
            let enumeration = variants.first().unwrap_or(enumeration);
            native_layout(&enumeration.def.underlying_type_ext(reader), reader, arch)
        }
        _ => panic!("unsupported native layout: {ty:?}"),
    }
}
