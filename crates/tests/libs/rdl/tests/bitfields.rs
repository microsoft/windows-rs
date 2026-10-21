use windows_metadata::{HasAttributes, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute NativeBitfieldAttribute {
        fn(name: String, offset: i64, width: i64);
        MemberType: Type,
        Future: bool,
    }
    attribute NativeTypedefAttribute { fn(); }
}}}
"#;

#[test]
fn compact_members_preserve_all_attribute_values() {
    let input = format!(
        r#"{SCHEMA}
        #[win32] mod Test {{
            type Integer = i32;
            #[repr(u32)] enum Mode {{ Off = 0, On = 1 }}
            struct Bits {{
                #[Windows::Win32::Metadata::NativeBitfield("signed", 2, 3, MemberType = i32)]
                #[Windows::Win32::Metadata::NativeBitfield("enabled", 5, 1, MemberType = bool)]
                #[Windows::Win32::Metadata::NativeBitfield("alias", 6, 3, MemberType = Integer)]
                #[Windows::Win32::Metadata::NativeBitfield("mode", 9, 2, MemberType = Test::Mode)]
                #[Windows::Win32::Metadata::NativeBitfield("legacy", 11, 2)]
                #[Windows::Win32::Metadata::NativeBitfield("self", 13, 1, MemberType = i32)]
                #[Windows::Win32::Metadata::NativeBitfield("_", 14, 1, MemberType = i32)]
                #[Windows::Win32::Metadata::NativeBitfield("crate", 15, 1, MemberType = i32)]
                #[Windows::Win32::Metadata::NativeBitfield("bad-name", 16, 1, MemberType = i32)]
                #[Windows::Win32::Metadata::NativeBitfield("r#raw", 17, 1, MemberType = i32)]
                storage: u32,
                #[Windows::Win32::Metadata::NativeBitfield("future", 0, 1, MemberType = u32, Future = true)]
                extended: u32,
                #[Windows::Win32::Metadata::NativeBitfield("negative", -1, 1)]
                negative: u32,
                #[Windows::Win32::Metadata::NativeBitfield("large", 4294967296, 1)]
                large: u32,
                #[Windows::Win32::Metadata::NativeBitfield("a", 0, 3)]
                #[Windows::Win32::Metadata::NativeBitfield("b", 2, 2, MemberType = u32)]
                overlap: u32,
            }}
            union Union {{
                #[Windows::Win32::Metadata::NativeBitfield("signed", 0, 3, MemberType = i32)]
                storage: u32,
                value: u32,
            }}
        }}
        "#
    );
    let root = std::path::Path::new(env!("OUT_DIR"));
    let before = root.join("compact-members-before.winmd");
    let after = root.join("compact-members-after.winmd");
    let rdl = root.join("compact-members.rdl");
    windows_rdl::reader()
        .input_text(&input)
        .output(&before)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&before)
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(&rdl).unwrap();
    for spelling in [
        "_: 2,",
        "signed: i32: 3,",
        "enabled: bool: 1,",
        "alias: Integer: 3,",
        "mode: Mode: 2,",
        "legacy: 2,",
        "\"self\": i32: 1,",
        "\"_\": i32: 1,",
        "\"crate\": i32: 1,",
        "\"bad-name\": i32: 1,",
        "\"r#raw\": i32: 1,",
        "Future = true",
        "\"negative\", -1, 1",
        "\"large\", 4294967296, 1",
        "\"b\", 2, 2, MemberType = u32",
    ] {
        assert!(text.contains(spelling), "{spelling}\n{text}");
    }
    windows_rdl::reader()
        .input(&rdl)
        .output(&after)
        .write()
        .unwrap();
    let indexes = [before, after].map(|path| reader::Index::read(path).unwrap());
    for name in ["Bits", "Union"] {
        let [before, after] = indexes.each_ref().map(|index| index.expect("Test", name));
        assert_eq!(before.flags(), after.flags());
        let fields = |item: reader::TypeDef| {
            item.fields()
                .map(|field| {
                    let mut values: Vec<_> = field.attributes().map(|attr| attr.value()).collect();
                    values.sort_by_key(|value| format!("{value:?}"));
                    (field.name().to_string(), field.ty(), values)
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(fields(before), fields(after));
    }
}

#[test]
fn invalid_compact_members_are_rejected() {
    for (members, message) in [
        (
            "_: i32: 2",
            "anonymous bitfield padding cannot have a member type",
        ),
        (
            "value: *mut i32: 2",
            "expected a primitive or nongeneric bitfield type",
        ),
        (
            "_: 4294967295, value: u32: 1",
            "bitfield offset exceeds u32",
        ),
    ] {
        let error = windows_rdl::reader()
            .input_text(&format!(
                "{SCHEMA} #[win32] mod Test {{ struct Bits {{ storage: u32 {{ {members} }}, }} }}"
            ))
            .output(std::path::Path::new(env!("OUT_DIR")).join("bad-compact-members.winmd"))
            .write()
            .unwrap_err();
        assert!(error.message.contains(message), "{error}");
    }
}
