use windows_metadata::{reader::*, *};

fn const_mut() -> Type {
    Type::PtrConst(Box::new(Type::PtrMut(Box::new(Type::I32), 1)), 1)
}

fn mut_const() -> Type {
    Type::PtrMut(Box::new(Type::PtrConst(Box::new(Type::I32), 1)), 1)
}

fn assert_metadata(path: &std::path::Path, namespace: &str) {
    let index = Index::read(path).unwrap();
    for (name, expected) in [("ConstMut", const_mut()), ("MutConst", mut_const())] {
        let alias = index.expect(namespace, name);
        assert!(alias.has_attribute("NativeTypedefAttribute"));
        assert_eq!(alias.fields().next().unwrap().ty(), expected);
    }
    let expected = [
        const_mut(),
        mut_const(),
        Type::PtrConst(
            Box::new(Type::PtrMut(
                Box::new(Type::PtrConst(
                    Box::new(Type::PtrMut(Box::new(Type::Void), 1)),
                    1,
                )),
                1,
            )),
            1,
        ),
        Type::PtrMut(
            Box::new(Type::PtrConst(
                Box::new(Type::PtrMut(Box::new(Type::I32), 2)),
                2,
            )),
            2,
        ),
        Type::ArrayFixed(Box::new(const_mut()), 3),
        Type::RefConst(Box::new(const_mut())),
        Type::RefMut(Box::new(mut_const())),
    ];
    assert_eq!(
        index
            .expect(namespace, "Pointers")
            .fields()
            .map(|field| field.ty())
            .collect::<Vec<_>>(),
        expected
    );
    for (owner, name, result) in [
        ("Callback", "Invoke", const_mut()),
        ("Apis", "Export", mut_const()),
        ("IPointers", "Method", const_mut()),
    ] {
        let method = index
            .expect(namespace, owner)
            .methods()
            .find(|method| method.name() == name)
            .unwrap();
        let signature = method.signature(&[]);
        assert_eq!(signature.return_type, result);
        assert_eq!(signature.types, [mut_const(), const_mut()]);
        let params = method.params_by_sequence(2).unwrap();
        assert_eq!(
            params.params()[0].unwrap().direction(),
            ParamDirection::Input
        );
        assert_eq!(
            params.params()[1].unwrap().direction(),
            ParamDirection::Output
        );
    }
}

#[test]
fn reviewed_mixed_pointer_fixture_roundtrips_without_updating_the_input() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let fixture = include_str!("../mixed_pointers.rdl");
    let winmd = dir.join("mixed_pointers.winmd");
    windows_rdl::reader()
        .input_text(fixture)
        .output(&winmd)
        .write()
        .unwrap();
    assert_metadata(&winmd, "Test");
    let rdl = dir.join("mixed_pointers.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        fixture.replace("\r\n", "\n")
    );
    let roundtrip_dir = dir.join("mixed_pointers_roundtrip");
    std::fs::create_dir_all(&roundtrip_dir).unwrap();
    let roundtrip = roundtrip_dir.join("mixed_pointers.winmd");
    windows_rdl::reader()
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_metadata(&roundtrip, "Test");
    assert_eq!(
        std::fs::read(&winmd).unwrap(),
        std::fs::read(roundtrip).unwrap()
    );

    let merged = dir.join("mixed_pointers_merged.winmd");
    merge().input(&winmd).output(&merged).merge().unwrap();
    assert_metadata(&merged, "Test");
    let remapped = dir.join("mixed_pointers_remapped.winmd");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .fallback("Remapped")
        .output(&remapped)
        .remap()
        .unwrap();
    assert_metadata(&remapped, "Remapped");
}

#[test]
fn mutable_pointer_modifier_vocabulary_compiles() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("pointer_vocabulary.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&output)
        .write()
        .unwrap();
    let index = Index::read(output).unwrap();
    let marker = index.expect("Windows.Win32.Metadata", "NativeMutableAttribute");
    assert_eq!(marker.category(), TypeCategory::Attribute);
    assert!(marker.fields().next().is_none());
    let ctor = marker.methods().next().unwrap();
    assert_eq!(ctor.name(), ".ctor");
    assert!(ctor.signature(&[]).types.is_empty());
}
