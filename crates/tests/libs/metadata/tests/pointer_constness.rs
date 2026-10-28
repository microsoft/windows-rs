use windows_metadata::{reader::*, writer, *};

fn pointer(constant: bool, ty: Type) -> Type {
    match (constant, ty) {
        (true, Type::PtrConst(inner, count)) => Type::PtrConst(inner, count + 1),
        (false, Type::PtrMut(inner, count)) => Type::PtrMut(inner, count + 1),
        (true, inner) => Type::PtrConst(Box::new(inner), 1),
        (false, inner) => Type::PtrMut(Box::new(inner), 1),
    }
}

fn encode(ty: &Type) -> Index {
    let mut file = writer::File::new("pointers");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    file.TypeDef("Test", "Pointers", base, TypeAttributes::Public);
    file.Field("value", ty, FieldAttributes::Public);
    Index::new(vec![File::new(file.into_stream()).unwrap()])
}

#[test]
fn pointer_constness() {
    for depth in 1..=5 {
        for mask in 0..1 << depth {
            let levels: Vec<_> = (0..depth).map(|bit| mask & (1 << bit) != 0).collect();
            let ty = levels
                .iter()
                .rev()
                .fold(Type::I32, |ty, &constant| pointer(constant, ty));
            let index = encode(&ty);
            let field = index.expect("Test", "Pointers").fields().next().unwrap();
            assert_eq!(field.ty(), ty);

            let mixed = levels.iter().any(|&constant| constant != levels[0]);
            let mut expected = vec![0x06];
            if !mixed && levels[0] {
                expected.push(0x1f);
            }

            for constant in levels {
                expected.push(0x0f);
                if mixed && constant {
                    expected.push(0x1f);
                }
            }
            expected.push(0x08);
            let mut blob = field.blob(2);
            let mut actual = Vec::new();
            while !blob.is_empty() {
                let code = blob.read_u8();
                actual.push(code);
                if code == 0x1f {
                    let modifier = blob.decode::<TypeDefOrRef>();
                    assert_eq!(modifier.namespace(), "System.Runtime.CompilerServices");
                    assert_eq!(modifier.name(), "IsConst");
                }
            }
            assert_eq!(actual, expected, "{ty:?}");

            for wrapped in [
                Type::ArrayFixed(Box::new(ty.clone()), 3),
                Type::RefMut(Box::new(ty.clone())),
                Type::RefConst(Box::new(ty.clone())),
                pointer(false, Type::ArrayFixed(Box::new(ty), 3)),
            ] {
                let index = encode(&wrapped);
                assert_eq!(
                    index
                        .expect("Test", "Pointers")
                        .fields()
                        .next()
                        .unwrap()
                        .ty(),
                    wrapped
                );
            }
        }
    }
}

#[test]
fn leading_and_per_pointer_const() {
    let mut file = writer::File::new("modifiers");
    let is_const = file.TypeRef("System.Runtime.CompilerServices", "IsConst");
    let is_const = u8::try_from(writer::TypeDefOrRef::TypeRef(is_const).encode()).unwrap();
    let index = Index::new(vec![File::new(file.into_stream()).unwrap()]);
    let signature = [0x1f, is_const, 0x0f, 0x1f, is_const, 0x0f, 0x08];
    let mut blob = Blob::new(&index, 0, &signature);
    assert_eq!(
        blob.read_type_signature(&[]),
        Type::PtrConst(Box::new(Type::I32), 2)
    );
    assert!(blob.is_empty());
}

#[test]
fn fixed_pointer_remap() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..\\target");
    let source = root.join("test-metadata-fixed-pointer-source.winmd");
    let output = root.join("test-metadata-fixed-pointer-remapped.winmd");
    let ty = |namespace| {
        Type::ArrayFixed(
            Box::new(Type::Ptr64(
                Box::new(Type::PtrConst(
                    Box::new(Type::value_named(namespace, "Target")),
                    1,
                )),
                false,
            )),
            2,
        )
    };
    let mut file = writer::File::new("fixed");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::Public | TypeAttributes::SequentialLayout;
    file.TypeDef("Source", "Holder", base, flags);
    file.Field("addresses", &ty("Source"), FieldAttributes::Public);
    file.TypeDef("Source", "Target", base, flags);
    file.Field("value", &Type::I32, FieldAttributes::Public);
    std::fs::write(&source, file.into_stream()).unwrap();
    remap()
        .input(&source)
        .source("Source")
        .fallback("Destination")
        .output(&output)
        .remap()
        .unwrap();
    let index = Index::read(&output).unwrap();
    assert_eq!(
        index
            .expect("Destination", "Holder")
            .fields()
            .next()
            .unwrap()
            .ty(),
        ty("Destination")
    );
    index.expect("Destination", "Target");
    for path in [source, output] {
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn malformed_fixed_pointer_modifiers() {
    let mut file = writer::File::new("fixed-modifiers");
    let modifier = file.TypeRef("Windows.Win32.Metadata", "NativePointer64Attribute");
    let modifier = u8::try_from(writer::TypeDefOrRef::TypeRef(modifier).encode()).unwrap();
    let index = Index::new(vec![File::new(file.into_stream()).unwrap()]);

    for (signature, message) in [
        (vec![0x1f, modifier, 0x08], "must follow PTR"),
        (vec![0x1f, modifier, 0x0f, 0x08], "must follow PTR"),
        (vec![0x10, 0x1f, modifier, 0x08], "must follow PTR"),
        (vec![0x0f, 0x20, modifier, 0x08], "must be required"),
        (
            vec![0x0f, 0x1f, modifier, 0x1f, modifier, 0x08],
            "duplicate NativePointer64Attribute",
        ),
    ] {
        let mut blob = Blob::new(&index, 0, &signature);
        let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            blob.read_type_signature(&[])
        }))
        .unwrap_err();
        while !blob.is_empty() {
            blob.read_u8();
        }
        let error = error
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| error.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(error.contains(message), "{signature:?}: {error}");
    }
}

#[test]
fn fixed_pointer_chains() {
    for depth in 1..=3 {
        for mask in 0..1 << (2 * depth) {
            let levels: Vec<_> = (0..depth)
                .map(|bit| (mask & (1 << (2 * bit)) != 0, mask & (2 << (2 * bit)) != 0))
                .collect();
            let ty = levels
                .iter()
                .rev()
                .fold(Type::I32, |inner, &(constant, fixed)| {
                    if fixed {
                        Type::Ptr64(Box::new(inner), constant)
                    } else {
                        pointer(constant, inner)
                    }
                });
            let index = encode(&ty);
            let field = index.expect("Test", "Pointers").fields().next().unwrap();
            assert_eq!(field.ty(), ty);
            if levels.iter().any(|(_, fixed)| *fixed) {
                let mut blob = field.blob(2);
                assert_eq!(blob.read_u8(), 0x06);
                for (constant, fixed) in levels {
                    assert_eq!(blob.read_u8(), 0x0f);
                    if fixed {
                        assert_eq!(blob.read_u8(), 0x1f);
                        let modifier = blob.decode::<TypeDefOrRef>();
                        assert_eq!(modifier.namespace(), "Windows.Win32.Metadata");
                        assert_eq!(modifier.name(), "NativePointer64Attribute");
                    }
                    if constant {
                        assert_eq!(blob.read_u8(), 0x1f);
                        let modifier = blob.decode::<TypeDefOrRef>();
                        assert_eq!(modifier.namespace(), "System.Runtime.CompilerServices");
                        assert_eq!(modifier.name(), "IsConst");
                    }
                }

                assert_eq!(blob.read_u8(), 0x08);
                assert!(blob.is_empty());
            }
            for wrapped in [
                Type::ArrayFixed(Box::new(ty.clone()), 3),
                Type::RefMut(Box::new(ty.clone())),
                Type::RefConst(Box::new(ty.clone())),
                pointer(false, Type::ArrayFixed(Box::new(ty), 3)),
            ] {
                assert_eq!(
                    encode(&wrapped)
                        .expect("Test", "Pointers")
                        .fields()
                        .next()
                        .unwrap()
                        .ty(),
                    wrapped
                );
            }
        }
    }
}
