use std::path::{Path, PathBuf};
use windows_metadata::{HasAttributes, Type, TypeAttributes, Value, reader, writer};

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("guid-{name}"))
}

fn guid(ty: reader::TypeDef<'_>) -> Option<Vec<(String, Value)>> {
    ty.find_attribute("GuidAttribute").map(|attr| attr.value())
}

#[test]
fn automatic_guids_preserve_authored_identities() {
    for (fixture, name, literal) in [
        (
            "interface",
            "IInterface",
            "0x4cac7376_063c_5868_a2a0_c4a63a058e81",
        ),
        (
            "interface_generic",
            "IVector",
            "0x85d1b6f2_043b_585c_b11d_7af01b1f0183",
        ),
        (
            "delegate",
            "Handler",
            "0x383d681f_3a02_577c_b363_1fa05a52f086",
        ),
        (
            "delegate_generic",
            "TypedHandler",
            "0x9da88cbe_f2e7_5cd4_ba2f_3a931a52632e",
        ),
    ] {
        let source = std::fs::read_to_string(format!("input/{fixture}.rdl")).unwrap();
        assert!(source.contains(literal));
        let explicit = output(&format!("{fixture}-explicit.winmd"));
        let automatic = output(&format!("{fixture}-automatic.winmd"));
        for (source, path) in [
            (source.clone(), &explicit),
            (source.replace(literal, "auto"), &automatic),
        ] {
            windows_rdl::reader()
                .input_text(&source)
                .output(path)
                .write()
                .unwrap();
        }
        let before = reader::Index::read(explicit).unwrap();
        let after = reader::Index::read(&automatic).unwrap();
        let expected = guid(before.expect("Test", name)).unwrap();
        assert_eq!(guid(after.expect("Test", name)), Some(expected.clone()));
        let rdl = automatic.with_extension("rdl");
        windows_rdl::writer()
            .input(&automatic)
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(&rdl).unwrap();
        assert!(text.contains(&format!("#[guid({literal})]")));
        assert!(!text.contains("guid(auto)"));
        let roundtrip = output(&format!("{fixture}-roundtrip.winmd"));
        windows_rdl::reader()
            .input(&rdl)
            .output(&roundtrip)
            .write()
            .unwrap();
        let index = reader::Index::read(roundtrip).unwrap();
        assert_eq!(guid(index.expect("Test", name)), Some(expected));
    }
}

#[test]
fn native_guid_absence_is_not_zero_or_inherited() {
    let first = output("native.winmd");
    let second = output("native-roundtrip.winmd");
    let rdl = output("native.rdl");
    windows_rdl::reader()
        .input_text(
            r#"
            #[win32] mod Test {
                #[guid(1)] interface Base {}
                interface Absent: Base {}
                #[guid(0)] interface Zero {}
                #[guid(auto)] interface Automatic {}
                type Callback = extern "C" fn();
            }
            "#,
        )
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(&rdl).unwrap();
    assert!(!text.contains("no_guid"));
    assert!(!text.contains("guid(auto)"));
    assert!(text.contains("interface Absent: Base"));
    windows_rdl::reader()
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        assert!(guid(index.expect("Test", "Absent")).is_none());
        assert!(guid(index.expect("Test", "Callback")).is_none());
        assert!(guid(index.expect("Test", "Base")).is_some());
        assert!(guid(index.expect("Test", "Automatic")).is_some());
        let zero = guid(index.expect("Test", "Zero")).unwrap();
        assert_eq!(zero.len(), 11);
        assert!(
            zero.iter()
                .all(|(_, value)| matches!(value, Value::U32(0) | Value::U16(0) | Value::U8(0)))
        );
    }
}

#[test]
fn invalid_guid_declarations_are_rejected() {
    for declaration in [
        "interface I {}",
        "interface I<T> {}",
        "delegate fn Handler();",
        "delegate fn Handler<T>();",
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!("#[winrt] mod Test {{ {declaration} }}"))
            .output(output("missing.winmd"))
            .write()
            .unwrap_err();
        assert!(
            error
                .message
                .contains("require `#[guid(...)]` or `#[guid(auto)]`")
        );
        assert_eq!(error.line, 1);
    }
    let full = "#[Windows::Foundation::Metadata::Guid(1u32, 0u16, 0u16, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8)]";
    for attrs in [
        "#[guid(auto)] #[guid(auto)]".to_string(),
        "#[guid(1)] #[guid(2)]".to_string(),
        "#[guid(auto)] #[guid(1)]".to_string(),
        format!("{full} #[guid(auto)]"),
        format!("#[guid(1)] {full}"),
        format!("{full} {full}"),
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!("#[win32] mod Test {{ {attrs} interface I {{}} }}"))
            .reference_default()
            .output(output("duplicate.winmd"))
            .write()
            .unwrap_err();
        assert_eq!(error.message, "duplicate GUID attribute", "{error}");
    }
    for attrs in [
        "#[no_guid]",
        "#[no_guid] #[guid(1)]",
        "#[guid(1)] #[no_guid]",
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!("#[win32] mod Test {{ {attrs} interface I {{}} }}"))
            .output(output("obsolete.winmd"))
            .write()
            .unwrap_err();
        assert!(error.message.contains("`#[no_guid]` is not supported"));
    }
    for (attrs, expected) in [
        ("#[guid]", "`#[guid]` requires a u128 literal or `auto`"),
        (
            "#[guid(other)]",
            "`#[guid]` requires a u128 literal or `auto`",
        ),
        (
            "#[guid(auto, 1)]",
            "`#[guid]` requires a u128 literal or `auto`",
        ),
        ("#[guid(-1)]", "invalid u128 literal in `#[guid]`"),
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!("#[win32] mod Test {{ {attrs} interface I {{}} }}"))
            .output(output("malformed.winmd"))
            .write()
            .unwrap_err();
        assert_eq!(error.message, expected, "{error}");
    }
    for (item, expected) in [
        (
            "#[guid(auto)] const VALUE: GUID = 0;",
            "GUID constants specify their GUID in the initializer",
        ),
        (
            "struct PROPERTYKEY { fmtid: GUID, pid: u32 } #[guid(auto)] const KEY: PROPERTYKEY = 1;",
            "`#[guid(auto)]` requires an interface or delegate",
        ),
        ("#[guid(auto)] struct Record {}", "attribute type not found"),
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!("#[win32] mod Test {{ {item} }}"))
            .output(output("owner.winmd"))
            .write()
            .unwrap_err();
        assert_eq!(error.message, expected, "{error}");
    }
}

#[test]
fn writer_rejects_ambiguous_or_missing_winrt_identity() {
    for (name, flags, count, expected) in [
        (
            "Duplicate",
            TypeAttributes::Interface,
            2,
            "duplicate GUID attributes",
        ),
        (
            "Missing",
            TypeAttributes::Interface | TypeAttributes::WindowsRuntime,
            0,
            "has no GUID",
        ),
    ] {
        let mut file = writer::File::new("guid");
        let ty = file.TypeDef(
            "Test",
            name,
            Default::default(),
            flags | TypeAttributes::Public,
        );
        let attribute = file.TypeRef("Windows.Foundation.Metadata", "GuidAttribute");
        let constructor = file.MemberRef(
            ".ctor",
            &windows_metadata::Signature {
                flags: windows_metadata::MethodCallAttributes::HASTHIS,
                return_type: Type::Void,
                types: vec![
                    Type::U32,
                    Type::U16,
                    Type::U16,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                    Type::U8,
                ],
            },
            writer::MemberRefParent::TypeRef(attribute),
        );
        let values = [
            Value::U32(1),
            Value::U16(0),
            Value::U16(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
            Value::U8(0),
        ]
        .map(|value| (String::new(), value));
        for _ in 0..count {
            file.Attribute(
                writer::HasAttribute::TypeDef(ty),
                writer::AttributeType::MemberRef(constructor),
                &values,
            );
        }
        let error = windows_rdl::writer()
            .input_bytes(&file.into_stream())
            .output(output(&format!("{name}.rdl")))
            .write()
            .unwrap_err();
        assert!(error.message.contains(expected), "{error}");
    }
}
