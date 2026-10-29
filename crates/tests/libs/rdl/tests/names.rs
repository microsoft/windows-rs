use windows_metadata::{HasAttributes, Value, reader::Index};

#[test]
fn enum_member_names_roundtrip_without_collisions() {
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let names = ["Self", "Self_", "self", "self_", "_", "unused", "type"];
    for mode in ["winrt", "win32"] {
        let path = dir.join(format!("enum-names-{mode}.winmd"));
        let rdl = path.with_extension("rdl");
        let source = format!(
            r#"
#[{mode}]
mod Test {{
    #[repr(i32)]
    enum Names {{
        Self = 1,
        Self_ = 2,
        self = 3,
        self_ = 4,
        _ = 5,
        unused = 6,
        r#type = 7,
    }}
    attribute ChoiceAttribute {{ fn(value: Names); }}
    #[Choice(Self)]
    struct First {{ value: i32, }}
    #[Choice(Self_)]
    struct Second {{ value: i32, }}
    #[Choice(5)]
    struct Underscore {{ value: i32, }}
    #[Choice(r#type)]
    struct Keyword {{ value: i32, }}
    #[flags]
    #[repr(u32)]
    enum Bits {{ Self = 1, r#type = 2, _ = 4, }}
    attribute MaskAttribute {{ fn(value: Bits); }}
    #[Mask(Self | r#type)]
    struct Combined {{ value: i32, }}
    #[Mask(5)]
    struct Numeric {{ value: i32, }}
}}
"#
        );
        windows_rdl::reader()
            .input_text(&source)
            .output(&path)
            .write()
            .unwrap();
        for _ in 0..2 {
            let index = Index::read(&path).unwrap();
            let fields: Vec<_> = index
                .expect("Test", "Names")
                .fields()
                .filter_map(|field| {
                    field
                        .constant()
                        .map(|constant| (field.name(), constant.value()))
                })
                .collect();
            let expected: Vec<_> = names
                .iter()
                .enumerate()
                .map(|(i, name)| (*name, Value::I32(i as i32 + 1)))
                .collect();
            assert_eq!(fields, expected);
            for (name, attribute, value) in [
                ("First", "ChoiceAttribute", 1),
                ("Second", "ChoiceAttribute", 2),
                ("Underscore", "ChoiceAttribute", 5),
                ("Keyword", "ChoiceAttribute", 7),
                ("Combined", "MaskAttribute", 3),
                ("Numeric", "MaskAttribute", 5),
            ] {
                let attribute = index
                    .expect("Test", name)
                    .find_attribute(attribute)
                    .unwrap();
                let Value::EnumValue(_, actual) = &attribute.value()[0].1 else {
                    panic!("expected enum value");
                };
                assert_eq!(**actual, Value::I32(value));
            }
            windows_rdl::writer()
                .input(&path)
                .output(&rdl)
                .write()
                .unwrap();
            windows_rdl::reader()
                .input(&rdl)
                .output(&path)
                .write()
                .unwrap();
        }
    }
}

#[test]
fn enum_members_are_separate_from_items() {
    let path = std::path::Path::new(env!("OUT_DIR")).join("names.rdl");
    std::fs::write(
        &path,
        r#"
        #[win32]
        mod Test {
            #[arch(X86)]
            #[repr(i32)]
            enum Plain { VALUE = 1, FIRST = 0 }
            #[arch(X64 | Arm64)]
            #[repr(i32)]
            enum Plain { VALUE = 1, SECOND = 2 }
            #[scoped]
            #[repr(i32)]
            enum Scoped { VALUE = 2, r#type = 3 }
            const IGNORED: u32 = 5;
            mod Other {
                #[repr(i32)]
                enum Different { OUTSIDE = 4 }
            }
        }
    "#,
    )
    .unwrap();
    assert_eq!(
        windows_rdl::item_names(&path, "Test").unwrap(),
        ["Plain", "Scoped", "IGNORED"]
    );
    assert_eq!(
        windows_rdl::enum_member_names(&path, "Test").unwrap(),
        ["FIRST", "SECOND", "VALUE", "type"]
    );
    assert_eq!(
        windows_rdl::enum_member_names(&path, "Test.Other").unwrap(),
        ["OUTSIDE"]
    );
    assert!(
        windows_rdl::enum_member_names(&path, "Absent")
            .unwrap()
            .is_empty()
    );
    std::fs::write(&path, "#[win32] mod Test { enum Broken {").unwrap();
    assert!(windows_rdl::enum_member_names(&path, "Test").is_err());
    std::fs::remove_file(&path).unwrap();
    assert!(windows_rdl::enum_member_names(&path, "Test").is_err());
}
