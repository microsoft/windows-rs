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
fn unsupported_imported_interface_generics() {
    for (winrt, flags, expected) in [
        (
            true,
            MethodCallAttributes::C,
            "function pointers require Win32 metadata",
        ),
        (
            false,
            MethodCallAttributes::VARARG,
            "unsupported function pointer signature flags",
        ),
    ] {
        let mut file = writer::File::new("interface_generics");
        let mut interface =
            TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract;
        if winrt {
            interface |= TypeAttributes::WindowsRuntime;
        }
        let generic = file.TypeDef("Test", "IA`1", writer::TypeDefOrRef::default(), interface);
        file.GenericParam(
            "T",
            writer::TypeOrMethodDef::TypeDef(generic),
            0,
            GenericParamAttributes::None,
        );
        let derived = file.TypeDef("Test", "IB", writer::TypeDefOrRef::default(), interface);
        if winrt {
            // WinRT interfaces require GUIDs before their signatures are validated.
            let attribute = writer::MemberRefParent::TypeRef(
                file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute"),
            );
            let mut types = vec![Type::U32, Type::U16, Type::U16];
            types.extend(std::iter::repeat_n(Type::U8, 8));
            let signature = Signature {
                flags: MethodCallAttributes::HASTHIS,
                return_type: Type::Void,
                types,
            };
            let ctor = file.MemberRef(".ctor", &signature, attribute);
            let mut values = vec![
                (String::new(), Value::U32(1)),
                (String::new(), Value::U16(2)),
                (String::new(), Value::U16(3)),
            ];
            values.extend((0..8).map(|byte| (String::new(), Value::U8(byte))));
            for def in [generic, derived] {
                file.Attribute(
                    writer::HasAttribute::TypeDef(def),
                    writer::AttributeType::MemberRef(ctor),
                    &values,
                );
            }
        }
        let callback = Type::FnPtr(Box::new(Signature {
            flags,
            return_type: Type::Void,
            types: vec![],
        }));
        file.InterfaceImpl(
            derived,
            &Type::ClassName(TypeName {
                namespace: "Test".to_string(),
                name: "IA".to_string(),
                generics: vec![callback],
            }),
        );
        let error = windows_rdl::writer()
            .input_bytes(&file.into_stream())
            .output(std::path::Path::new(env!("OUT_DIR")).join("interface-generics.rdl"))
            .write()
            .unwrap_err();
        assert!(error.message.contains(expected), "{error}");
    }
}

#[test]
fn nested_function_pointer_references() {
    let declared = "#[win32]\nmod Test {\n    struct Outer {\n        struct Inner {\n            value: i32,\n        }\n        value: Outer::Inner,\n        pointer: *mut Outer::Inner,\n        callback: extern \"C\" fn(&Outer::Inner, &mut Outer::Inner) -> *mut Outer::Inner,\n    }\n}\n";
    let inline = "#[win32]\nmod Test {\n    struct Outer {\n        value: struct {\n            value: i32,\n        },\n        pointer: *mut Outer::Outer_0,\n        callback: extern \"C\" fn(&Outer::Outer_0, &mut Outer::Outer_0) -> *mut Outer::Outer_0,\n    }\n}\n";
    for (name, expected) in [("Inner", declared), ("Outer_0", inline)] {
        let mut file = writer::File::new("nested_function_pointer");
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
        let flags = TypeAttributes::SequentialLayout | TypeAttributes::Sealed;
        let outer = file.TypeDef("Test", "Outer", base, flags | TypeAttributes::Public);
        let inner = Type::value_named("Test", &format!("Outer/{name}"));
        let pointer = Type::PtrMut(Box::new(inner.clone()), 1);
        let callback = Type::FnPtr(Box::new(Signature {
            flags: MethodCallAttributes::C,
            return_type: pointer.clone(),
            types: vec![
                Type::RefConst(Box::new(inner.clone())),
                Type::RefMut(Box::new(inner.clone())),
            ],
        }));
        let fields = [
            ("value", inner),
            ("pointer", pointer),
            ("callback", callback),
        ];
        for (field, ty) in &fields {
            file.Field(field, ty, FieldAttributes::Public);
        }
        let child = file.TypeDef("", name, base, flags | TypeAttributes::NestedPublic);
        file.NestedClass(child, outer);
        if name == "Outer_0" {
            let attribute = writer::MemberRefParent::TypeRef(
                file.TypeRef("Windows.Win32.Metadata", "NativeAnonymousAttribute"),
            );
            let signature = Signature {
                types: vec![Type::I32],
                ..Default::default()
            };
            let ctor = file.MemberRef(".ctor", &signature, attribute);
            file.Attribute(
                writer::HasAttribute::TypeDef(child),
                writer::AttributeType::MemberRef(ctor),
                &[(String::new(), Value::I32(0))],
            );
        }
        file.Field("value", &Type::I32, FieldAttributes::Public);
        let rdl = std::path::Path::new(env!("OUT_DIR")).join(format!("nested-fnptr-{name}.rdl"));
        windows_rdl::writer()
            .input_bytes(&file.into_stream())
            .output(&rdl)
            .write()
            .unwrap();
        assert_eq!(std::fs::read_to_string(&rdl).unwrap(), expected);
        windows_rdl::reader()
            .input(&rdl)
            .output(rdl.with_extension("winmd"))
            .write()
            .unwrap();
        let index = reader::Index::read(rdl.with_extension("winmd")).unwrap();
        let outer = index.expect("Test", "Outer");
        let children: Vec<_> = index.nested(outer).collect();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name(), name);
        assert_eq!(
            outer.fields().map(|field| field.ty()).collect::<Vec<_>>(),
            fields.map(|(_, ty)| ty)
        );
    }
}
