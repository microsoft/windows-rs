use windows_metadata::*;

#[test]
fn unsupported_function_pointers() {
    for (index, (ty, message)) in [
        ("fn(i32)", "requires an extern ABI"),
        ("extern \"fastcall\" fn(i32)", "ABI is unsupported"),
        ("unsafe extern \"C\" fn(i32)", "qualified or variadic"),
        ("for<'a> extern \"C\" fn(&'a i32)", "qualified or variadic"),
        ("extern \"C\" fn(i32, ...)", "qualified or variadic"),
        ("extern \"C\" fn(value: i32)", "unnamed and unattributed"),
        (
            "extern \"C\" fn(#[optional] i32)",
            "unnamed and unattributed",
        ),
        ("extern \"C\" fn(void)", "parameter cannot be void"),
    ]
    .into_iter()
    .enumerate()
    {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct S {{ callback: {ty} }} }}"
            ))
            .output(std::path::Path::new(env!("OUT_DIR")).join(format!("bad-fnptr-{index}.winmd")))
            .write()
            .unwrap_err();
        assert!(error.message.contains(message), "{ty}: {error}");
    }
    for (index, ty) in ["extern \"C\" fn()", "*mut extern fn()", "[extern fn(); 2]"]
        .into_iter()
        .enumerate()
    {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "#[winrt] mod Test {{ struct S {{ callback: {ty} }} }}"
            ))
            .output(
                std::path::Path::new(env!("OUT_DIR")).join(format!("winrt-fnptr-{index}.winmd")),
            )
            .write()
            .unwrap_err();
        assert!(
            error.message.contains("function pointers require Win32"),
            "{error}"
        );
    }
}

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
    let inner = Type::PtrMut(Box::new(Type::value_named("Test", "Outer/Inner")), 1);
    file.Field(
        "callback",
        &Type::FnPtr(Box::new(Signature {
            flags: MethodCallAttributes::C,
            return_type: inner.clone(),
            types: vec![inner],
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
        "#[win32]\nmod Test {\n    struct Outer {\n        callback: extern \"C\" fn(*mut Outer_0) -> *mut Outer_0,\n    }\n    struct Outer_0 {\n        value: i32,\n    }\n}\n"
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
    assert_eq!(signature.return_type, signature.types[0]);
    assert_eq!(
        signature.return_type,
        Type::PtrMut(Box::new(Type::value_named("Test", "Outer_0")), 1)
    );
}
