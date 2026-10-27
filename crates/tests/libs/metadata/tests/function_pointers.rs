use windows_metadata::{reader::*, writer, *};

fn callback(flags: MethodCallAttributes, result: Type, types: Vec<Type>) -> Type {
    Type::FnPtr(Box::new(Signature {
        flags,
        return_type: result,
        types,
    }))
}

fn file(ty: &Type) -> Vec<u8> {
    let mut file = writer::File::new("callbacks");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::Public | TypeAttributes::SequentialLayout;
    file.TypeDef("Test", "Callbacks", base, flags);
    file.Field("callback", ty, FieldAttributes::Public);
    for name in ["Input", "Output"] {
        file.TypeDef("Test", name, base, flags);
        file.Field("value", &Type::I32, FieldAttributes::Public);
    }
    file.into_stream()
}

#[test]
fn function_pointer_signatures() {
    let empty = callback(MethodCallAttributes::STDCALL, Type::Void, vec![]);
    let c = callback(
        MethodCallAttributes::C,
        Type::I32,
        vec![Type::I32, Type::PtrMut(Box::new(Type::Void), 1)],
    );
    for (ty, expected) in [
        (&empty, vec![0x06, 0x1b, 0x02, 0x00, 0x01]),
        (&c, vec![0x06, 0x1b, 0x01, 0x02, 0x08, 0x08, 0x0f, 0x01]),
    ] {
        let index = Index::new(vec![File::new(file(ty)).unwrap()]);
        let field = index.expect("Test", "Callbacks").fields().next().unwrap();
        assert_eq!(field.ty(), *ty);
        let mut blob = field.blob(2);
        for byte in expected {
            assert_eq!(blob.read_u8(), byte);
        }
        assert!(blob.is_empty());
    }
    let nested = callback(MethodCallAttributes::C, empty, vec![c]);
    for ty in [
        nested.clone(),
        Type::PtrConst(Box::new(nested.clone()), 1),
        Type::ArrayFixed(Box::new(nested.clone()), 3),
        Type::PtrMut(Box::new(Type::ArrayFixed(Box::new(nested), 2)), 1),
    ] {
        let index = Index::new(vec![File::new(file(&ty)).unwrap()]);
        assert_eq!(
            index
                .expect("Test", "Callbacks")
                .fields()
                .next()
                .unwrap()
                .ty(),
            ty
        );
    }
}

#[test]
fn function_pointer_merge_and_remap() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..\\target");
    let source = root.join("test-metadata-function-pointers-source.winmd");
    let other = root.join("test-metadata-function-pointers-other.winmd");
    let merged = root.join("test-metadata-function-pointers-merged.winmd");
    let remapped = root.join("test-metadata-function-pointers-remapped.winmd");
    let signature = |flags, input_ns, output_ns| {
        callback(
            flags,
            Type::PtrMut(Box::new(Type::value_named(output_ns, "Output")), 1),
            vec![
                Type::PtrConst(Box::new(Type::value_named(input_ns, "Input")), 1),
                callback(
                    MethodCallAttributes::C,
                    Type::value_named(output_ns, "Output"),
                    vec![],
                ),
            ],
        )
    };
    std::fs::write(
        &source,
        file(&signature(MethodCallAttributes::C, "Test", "Test")),
    )
    .unwrap();
    std::fs::write(
        &other,
        file(&signature(MethodCallAttributes::STDCALL, "Test", "Test")),
    )
    .unwrap();
    merge()
        .arch_input(&source, 6)
        .arch_input(&other, 1)
        .output(&merged)
        .merge()
        .unwrap();
    let index = Index::read(&merged).unwrap();
    let mut variants: Vec<_> = index
        .get("Test", "Callbacks")
        .map(|def| (def.arches(), def.fields().next().unwrap().ty()))
        .collect();
    variants.sort_by_key(|(arches, _)| *arches);
    assert_eq!(
        variants,
        [
            (1, signature(MethodCallAttributes::STDCALL, "Test", "Test")),
            (6, signature(MethodCallAttributes::C, "Test", "Test")),
        ]
    );
    remap()
        .input(&merged)
        .source("Test")
        .fallback("Callbacks")
        .route("Input", "Parameters")
        .route("Output", "Results")
        .output(&remapped)
        .remap()
        .unwrap();
    let index = Index::read(&remapped).unwrap();
    for def in index.get("Callbacks", "Callbacks") {
        let flags = if def.arches() == 1 {
            MethodCallAttributes::STDCALL
        } else {
            MethodCallAttributes::C
        };
        assert_eq!(
            def.fields().next().unwrap().ty(),
            signature(flags, "Parameters", "Results")
        );
    }
    assert_eq!(index.get("Callbacks", "Callbacks").count(), 2);
    index.expect("Parameters", "Input");
    index.expect("Results", "Output");
    for path in [source, other, merged, remapped] {
        std::fs::remove_file(path).unwrap();
    }
}
