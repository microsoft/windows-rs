use windows_metadata::*;

fn fixture(types: &[Type]) -> reader::Index {
    let mut file = writer::File::new("qualifiers");
    for (namespace, name) in [
        ("Windows.Win32.Metadata", "NativeConstAttribute"),
        ("Windows.Win32.Metadata", "NativeVolatileAttribute"),
        ("System.Runtime.CompilerServices", "IsConst"),
        ("Windows.Win32.Metadata", "NativeMutableAttribute"),
    ] {
        file.TypeRef(namespace, name);
    }
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    file.TypeDef(
        "Test",
        "Qualifiers",
        base,
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    for (position, ty) in types.iter().enumerate() {
        file.Field(&format!("p{position}"), ty, FieldAttributes::Public);
    }
    reader::Index::new(vec![reader::File::new(file.into_stream()).unwrap()])
}

fn bytes(field: reader::Field<'_>) -> Vec<u8> {
    let mut blob = field.blob(2);
    let bytes = blob.to_vec();
    while !blob.is_empty() {
        blob.read_u8();
    }
    bytes
}

#[test]
fn qualifiers_have_distinct_required_bytes_at_the_qualified_node() {
    let cases = [
        (Type::I32, vec![0x06, 0x08]),
        (
            Type::Const(Box::new(Type::I32)),
            vec![0x06, 0x1f, 0x05, 0x08],
        ),
        (
            Type::Volatile(Box::new(Type::I32)),
            vec![0x06, 0x1f, 0x09, 0x08],
        ),
        (
            Type::Const(Box::new(Type::PtrMut(Box::new(Type::I32), 1))),
            vec![0x06, 0x1f, 0x05, 0x0f, 0x08],
        ),
        (
            Type::PtrMut(Box::new(Type::Const(Box::new(Type::I32))), 1),
            vec![0x06, 0x0f, 0x1f, 0x05, 0x08],
        ),
        (
            Type::Const(Box::new(Type::Volatile(Box::new(Type::I32)))),
            vec![0x06, 0x1f, 0x05, 0x1f, 0x09, 0x08],
        ),
        (
            Type::Volatile(Box::new(Type::Const(Box::new(Type::I32)))),
            vec![0x06, 0x1f, 0x09, 0x1f, 0x05, 0x08],
        ),
        (
            Type::PtrConst(Box::new(Type::I32), 2),
            vec![0x06, 0x1f, 0x0d, 0x0f, 0x0f, 0x08],
        ),
        (
            Type::Const(Box::new(Type::PtrConst(
                Box::new(Type::PtrMut(Box::new(Type::I32), 1)),
                1,
            ))),
            vec![0x06, 0x1f, 0x05, 0x1f, 0x0d, 0x0f, 0x1f, 0x11, 0x0f, 0x08],
        ),
    ];
    let types: Vec<_> = cases.iter().map(|(ty, _)| ty.clone()).collect();
    let index = fixture(&types);
    let fields = index.expect("Test", "Qualifiers").fields();
    for (field, (ty, expected)) in fields.zip(cases) {
        assert_eq!(bytes(field), expected, "{ty:?}");
        assert_eq!(field.ty(), ty);
    }
}

fn qualify(mut ty: Type, bits: usize) -> Type {
    if bits & 1 != 0 {
        ty = Type::Volatile(Box::new(ty));
    }
    if bits & 2 != 0 {
        ty = Type::Const(Box::new(ty));
    }
    ty
}

#[test]
fn every_node_qualifier_combination_preserves_pointer_and_storage_structure() {
    let mut types = Vec::new();
    for depth in 0..=2 {
        for bits in 0..(1 << (2 + 3 * depth)) {
            let mut ty = qualify(Type::I32, bits & 3);
            for level in 0..depth {
                let node = (bits >> (2 + 3 * level)) & 7;
                ty = if node & 4 != 0 {
                    Type::PtrConst(Box::new(ty), 1)
                } else {
                    Type::PtrMut(Box::new(ty), 1)
                };
                ty = qualify(ty, node & 3);
            }
            types.extend([
                ty.clone(),
                Type::ArrayFixed(Box::new(ty.clone()), 3),
                Type::Array(Box::new(ty.clone())),
                Type::RefConst(Box::new(ty.clone())),
                Type::RefMut(Box::new(ty)),
            ]);
        }
    }
    // Uniform adjacent pointer runs are represented in compressed form by the decoder.
    fn compress(ty: Type) -> Type {
        match ty {
            Type::PtrMut(inner, count) => match compress(*inner) {
                Type::PtrMut(inner, more) => Type::PtrMut(inner, count + more),
                inner => Type::PtrMut(Box::new(inner), count),
            },
            Type::PtrConst(inner, count) => match compress(*inner) {
                Type::PtrConst(inner, more) => Type::PtrConst(inner, count + more),
                inner => Type::PtrConst(Box::new(inner), count),
            },
            Type::Const(inner) => Type::Const(Box::new(compress(*inner))),
            Type::Volatile(inner) => Type::Volatile(Box::new(compress(*inner))),
            Type::Array(inner) => Type::Array(Box::new(compress(*inner))),
            Type::ArrayFixed(inner, len) => Type::ArrayFixed(Box::new(compress(*inner)), len),
            Type::RefConst(inner) => Type::RefConst(Box::new(compress(*inner))),
            Type::RefMut(inner) => Type::RefMut(Box::new(compress(*inner))),
            ty => ty,
        }
    }
    let types: Vec<_> = types.into_iter().map(compress).collect();
    let index = fixture(&types);
    assert_eq!(
        index
            .expect("Test", "Qualifiers")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        types
    );
}

#[test]
fn malformed_native_qualifier_modifiers_are_rejected() {
    let index = fixture(&[]);
    for signature in [
        &[0x20, 0x05, 0x08][..],
        &[0x20, 0x09, 0x08][..],
        &[0x1f, 0x05][..],
        &[0x1f, 0x09][..],
    ] {
        let error = std::panic::catch_unwind(|| {
            reader::Blob::new(&index, 0, signature).read_type_signature(&[]);
        });
        assert!(error.is_err(), "{signature:?}");
    }
}
