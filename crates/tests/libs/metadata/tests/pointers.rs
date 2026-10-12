use windows_metadata::*;

fn fixture(types: &[Type]) -> Vec<u8> {
    let mut file = writer::File::new("pointers");
    file.TypeRef("System.Runtime.CompilerServices", "IsConst");
    file.TypeRef("Windows.Win32.Metadata", "NativeMutableAttribute");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    file.TypeDef(
        "Test",
        "Pointers",
        base,
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    for (position, ty) in types.iter().enumerate() {
        file.Field(&format!("p{position}"), ty, FieldAttributes::Public);
    }
    file.into_stream()
}

fn index(types: &[Type]) -> reader::Index {
    reader::Index::new(vec![reader::File::new(fixture(types)).unwrap()])
}

fn bytes(field: reader::Field<'_>) -> Vec<u8> {
    let mut blob = field.blob(2);
    let bytes = blob.to_vec();
    while !blob.is_empty() {
        blob.read_u8();
    }
    bytes
}

fn pointer(ty: Type, is_const: bool) -> Type {
    match ty {
        Type::PtrConst(inner, count) if is_const => Type::PtrConst(inner, count + 1),
        Type::PtrMut(inner, count) if !is_const => Type::PtrMut(inner, count + 1),
        ty if is_const => Type::PtrConst(Box::new(ty), 1),
        ty => Type::PtrMut(Box::new(ty), 1),
    }
}

#[test]
fn pointer_run_encoding_preserves_uniform_bytes_and_both_mixed_orders() {
    let cases = [
        (
            Type::PtrMut(Box::new(Type::I32), 2),
            vec![0x06, 0x0f, 0x0f, 0x08],
        ),
        (
            Type::PtrConst(Box::new(Type::I32), 2),
            vec![0x06, 0x1f, 0x05, 0x0f, 0x0f, 0x08],
        ),
        (
            Type::PtrMut(Box::new(Type::PtrConst(Box::new(Type::I32), 1)), 1),
            vec![0x06, 0x0f, 0x1f, 0x05, 0x0f, 0x08],
        ),
        (
            Type::PtrConst(Box::new(Type::PtrMut(Box::new(Type::I32), 1)), 1),
            vec![0x06, 0x1f, 0x05, 0x0f, 0x1f, 0x09, 0x0f, 0x08],
        ),
        (
            Type::PtrConst(Box::new(Type::PtrMut(Box::new(Type::I32), 2)), 2),
            vec![0x06, 0x1f, 0x05, 0x0f, 0x0f, 0x1f, 0x09, 0x0f, 0x0f, 0x08],
        ),
        (
            Type::PtrConst(
                Box::new(Type::PtrMut(
                    Box::new(Type::PtrConst(Box::new(Type::I32), 1)),
                    1,
                )),
                1,
            ),
            vec![
                0x06, 0x1f, 0x05, 0x0f, 0x1f, 0x09, 0x0f, 0x1f, 0x05, 0x0f, 0x08,
            ],
        ),
    ];
    let types = cases.iter().map(|(ty, _)| ty.clone()).collect::<Vec<_>>();
    let index = index(&types);
    let fields = index
        .expect("Test", "Pointers")
        .fields()
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), cases.len());
    for (field, (ty, expected)) in fields.into_iter().zip(cases) {
        assert_eq!(bytes(field), expected, "{ty:?}");
        assert_eq!(field.ty(), ty);
    }
}

#[test]
fn every_qualifier_order_survives_nested_signature_contexts() {
    let mut types = Vec::new();
    for depth in 1..=5 {
        for bits in 0..(1 << depth) {
            let mut ty = Type::Void;
            for level in 0..depth {
                ty = pointer(ty, bits & (1 << level) != 0);
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
    let index = index(&types);
    let actual = index
        .expect("Test", "Pointers")
        .fields()
        .map(|field| field.ty())
        .collect::<Vec<_>>();
    assert_eq!(actual, types);
}

#[test]
fn mixed_pointer_targets_survive_merge_and_namespace_remapping() {
    let types = |namespace: &str| {
        let target = Type::value_named(namespace, "Pointers");
        vec![
            Type::PtrConst(Box::new(Type::PtrMut(Box::new(target.clone()), 1)), 1),
            Type::PtrMut(Box::new(Type::PtrConst(Box::new(target), 1)), 1),
        ]
    };
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .join("target")
        .join("test_metadata_pointers");
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("input.winmd");
    std::fs::write(&input, fixture(&types("Test"))).unwrap();
    let merged = dir.join("merged.winmd");
    merge().input(&input).output(&merged).merge().unwrap();
    let remapped = dir.join("remapped.winmd");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .route("Pointers", "Routed")
        .fallback("Wrong")
        .output(&remapped)
        .remap()
        .unwrap();
    for (path, namespace) in [(&merged, "Test"), (&remapped, "Routed")] {
        let index = reader::Index::read(path).unwrap();
        let actual = index
            .expect(namespace, "Pointers")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>();
        assert_eq!(actual, types(namespace));
    }
}

fn reject(signature: &[u8], diagnostic: &str) {
    let index = index(&[]);
    let error = std::panic::catch_unwind(|| {
        let mut blob = reader::Blob::new(&index, 0, signature);
        blob.read_type_signature(&[]);
    })
    .unwrap_err();
    let message = error
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| error.downcast_ref::<String>().map(String::as_str))
        .unwrap();
    assert!(message.contains(diagnostic), "{message}");
}

#[test]
fn malformed_pointer_modifiers_fail_explicitly() {
    reject(
        &[0x1f, 0x05, 0x1f, 0x09, 0x0f, 0x08],
        "conflicting const and mutable",
    );
    reject(
        &[0x1f, 0x09, 0x08],
        "mutable pointer modifier requires a pointer run",
    );
    reject(
        &[0x1f, 0x09, 0x10, 0x08],
        "mutable pointer modifier requires a pointer run",
    );
    reject(
        &[0x1f, 0x09, 0x1d, 0x08],
        "mutable pointer modifier requires a pointer run",
    );
    reject(&[0x0f], "pointer signature is missing its pointee type");
    reject(
        &[0x1f, 0x09, 0x0f],
        "pointer signature is missing its pointee type",
    );
    reject(
        &[0x0f, 0x1f, 0x09],
        "type signature is missing its type code",
    );
}
