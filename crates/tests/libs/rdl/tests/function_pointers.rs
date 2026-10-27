use windows_metadata::*;

#[test]
fn unsupported_imported_signatures() {
    for flags in [
        MethodCallAttributes::HASTHIS,
        MethodCallAttributes::VARARG,
        MethodCallAttributes(3),
    ] {
        let mut file = writer::File::new("unsupported");
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
        file.TypeDef("Test", "Callbacks", base, TypeAttributes::Public);
        file.Field(
            "callback",
            &Type::FnPtr(Box::new(Signature {
                flags,
                return_type: Type::Void,
                types: vec![],
            })),
            FieldAttributes::Public,
        );
        let error = windows_rdl::writer()
            .input_bytes(&file.into_stream())
            .output(std::path::Path::new(env!("OUT_DIR")).join("unsupported-fnptr.rdl"))
            .write()
            .unwrap_err();
        assert!(
            error
                .message
                .contains("unsupported function pointer signature flags"),
            "{error}"
        );
    }
}

#[test]
fn nested_function_pointer_references() {
    let mut file = writer::File::new("nested_function_pointer");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
    let outer = file.TypeDef("Test", "Outer", base, flags | TypeAttributes::Public);
    let inner = Type::value_named("Test", "Outer/Inner");
    file.Field(
        "callback",
        &Type::FnPtr(Box::new(Signature {
            flags: MethodCallAttributes::C,
            return_type: Type::PtrMut(Box::new(inner.clone()), 1),
            types: vec![
                Type::RefConst(Box::new(inner.clone())),
                Type::RefMut(Box::new(inner)),
            ],
        })),
        FieldAttributes::Public,
    );
    let child = file.TypeDef("", "Inner", base, flags | TypeAttributes::NestedPublic);
    file.NestedClass(child, outer);
    file.Field("value", &Type::I32, FieldAttributes::Public);
    let rdl = std::path::Path::new(env!("OUT_DIR")).join("nested-function-pointer.rdl");
    windows_rdl::writer()
        .input_bytes(&file.into_stream())
        .output(&rdl)
        .write()
        .unwrap();
    assert_eq!(
        std::fs::read_to_string(&rdl).unwrap(),
        "#[win32]\nmod Test {\n    struct Outer {\n        callback: extern \"C\" fn(&Outer_0, &mut Outer_0) -> *mut Outer_0,\n    }\n    struct Outer_0 {\n        value: i32,\n    }\n}\n"
    );
    windows_rdl::reader()
        .input(&rdl)
        .output(rdl.with_extension("winmd"))
        .write()
        .unwrap();
    let index = reader::Index::read(rdl.with_extension("winmd")).unwrap();
    let Type::FnPtr(signature) = index.expect("Test", "Outer").fields().next().unwrap().ty() else {
        panic!();
    };
    let record = Type::value_named("Test", "Outer_0");
    assert_eq!(
        signature.return_type,
        Type::PtrMut(Box::new(record.clone()), 1)
    );
    assert_eq!(
        signature.types,
        [
            Type::RefConst(Box::new(record.clone())),
            Type::RefMut(Box::new(record))
        ]
    );
}
