use windows_metadata::{HasAttributes, Type, Value, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute AssociatedEnumAttribute { fn(#[in] enumType: Type); }
}}}
"#;

#[test]
fn associated_enum_preserves_storage_and_remaps_references() {
    let output = std::path::Path::new(env!("OUT_DIR"));
    let winmd = output.join("associated-enum.winmd");
    let remapped = output.join("associated-enum-remapped.winmd");
    windows_rdl::reader()
        .reference_default()
        .input_text(
            r#"
        #[win32] mod Test {
            #[repr(u16)] enum Family { Inet = 2, }
            #[library("test.dll")]
            extern fn Socket(#[associated_enum(Family)] family: i32)
                -> #[associated_enum(Family)] i32;
            struct Address { #[associated_enum(Family)] family: u16, }
        }"#,
        )
        .output(&winmd)
        .write()
        .unwrap();
    windows_metadata::remap()
        .input(&winmd)
        .source("Test")
        .fallback("Api")
        .route("Family", "Values")
        .output(&remapped)
        .remap()
        .unwrap();
    let index = reader::Index::read(&remapped).unwrap();
    let method = index.expect("Api", "Apis").methods().next().unwrap();
    assert_eq!(method.signature(&[]).return_type, Type::I32);
    assert_eq!(method.signature(&[]).types, [Type::I32]);
    let expected = vec![(
        String::new(),
        Value::TypeName(windows_metadata::TypeName::named("Values", "Family")),
    )];
    for param in method.params() {
        assert_eq!(
            param
                .find_attribute("AssociatedEnumAttribute")
                .unwrap()
                .value(),
            expected
        );
    }
    let field = index.expect("Api", "Address").fields().next().unwrap();
    assert_eq!(field.ty(), Type::U16);
    assert_eq!(
        field
            .find_attribute("AssociatedEnumAttribute")
            .unwrap()
            .value(),
        expected
    );
}

#[test]
fn invalid_associated_enum_contracts() {
    let winmd = std::path::Path::new(env!("OUT_DIR")).join("invalid-associated-enum.winmd");
    for item in [
        "#[associated_enum(E)] struct S { value: i32, }",
        "#[associated_enum(E)] const V: u32 = 1;",
        "extern fn F(#[associated_enum] value: i32);",
        "extern fn F(#[associated_enum(E, E)] value: i32);",
        "extern fn F(#[associated_enum(Missing)] value: i32);",
        "extern fn F(#[associated_enum(NotEnum)] value: i32);",
        "extern fn F(#[associated_enum(E)] #[associated_enum(E)] value: i32);",
        "extern fn F(#[associated_enum(E)] value: f32);",
        "extern fn F(#[associated_enum(E)] value: E);",
        "extern fn F() -> #[associated_enum(E)] f64;",
        "struct S { #[associated_enum(E)] value: NotEnum, }",
        "type Cycle = *mut Cycle; extern fn F(#[associated_enum(E)] value: Cycle);",
    ] {
        let result = windows_rdl::reader()
            .input_text(&format!(
                r#"{SCHEMA}
            #[win32] mod Test {{
                #[repr(u32)] enum E {{ One = 1, }}
                struct NotEnum {{ value: u32, }}
                {item}
            }}"#
            ))
            .output(&winmd)
            .write();
        assert!(result.is_err(), "{item}");
    }
}

#[test]
fn associated_enum_storage_variants() {
    let output = std::path::Path::new(env!("OUT_DIR"));
    let reference = output.join("associated-enum-storage-reference.winmd");
    let winmd = output.join("associated-enum-storage.winmd");
    for storage in ["u64", "f32"] {
        let aliases = format!(
            "#[win32] mod Test {{
                    #[arch(X86)] type Number = Word;
                    #[arch(X64 | Arm64)] type Number = {storage};
                    #[arch(X86)] type Word = u32;
                    #[arch(X64 | Arm64)] type Word = f32;
                }}"
        );
        windows_rdl::reader()
            .input_text(&aliases)
            .output(&reference)
            .write()
            .unwrap();
        for external in [false, true] {
            let mut reader = windows_rdl::reader();
            if external {
                reader.reference(&reference);
            } else {
                reader.input_text(&aliases);
            }
            let result = reader
                .input_text(&format!(
                    "{SCHEMA}
                        #[win32] mod Test {{
                            #[repr(u32)] enum E {{ One = 1 }}
                            extern fn F(#[associated_enum(E)] value: *mut Number);
                        }}"
                ))
                .output(&winmd)
                .write();
            if storage == "u64" {
                result.unwrap();
            } else {
                let error = result.unwrap_err().to_string();
                assert!(error.contains("requires integer storage"), "{error}");
            }
        }
    }
}

#[test]
fn associated_enum_architecture_scopes() {
    let output = std::path::Path::new(env!("OUT_DIR"));
    let reference = output.join("associated-enum-arch-reference.winmd");
    let winmd = output.join("associated-enum-arch.winmd");
    let aliases = "#[win32] mod Test {
                #[arch(X86)] type Number = u32;
                #[arch(X64 | Arm64)] type Number = f32;
            }";
    windows_rdl::reader()
        .input_text(aliases)
        .output(&reference)
        .write()
        .unwrap();
    for external in [false, true] {
        for (item, valid) in [
            (
                "#[arch(X86)] #[library(\"test.dll\")] extern fn F(#[associated_enum(E)] value: *mut Number) -> #[associated_enum(E)] Number;",
                true,
            ),
            (
                "#[arch(X64)] #[library(\"test.dll\")] extern fn F(#[associated_enum(E)] value: *mut Number);",
                false,
            ),
            (
                "#[library(\"test.dll\")] extern fn F(#[associated_enum(E)] value: *mut Number);",
                false,
            ),
            (
                "#[arch(X86)] extern fn Callback(#[associated_enum(E)] value: Number) -> #[associated_enum(E)] Number;",
                true,
            ),
            (
                "#[arch(X86)] struct Outer { struct Inner { #[associated_enum(E)] value: Number, } }",
                true,
            ),
            (
                "#[arch(X86)] interface I { fn F(&self, #[associated_enum(E)] value: Number) -> #[associated_enum(E)] Number; }",
                true,
            ),
            (
                "interface I { #[arch(X86)] fn F(&self, #[associated_enum(E)] value: Number); fn Other(&self, value: f32); }",
                true,
            ),
            (
                "interface I { #[arch(X86)] fn F(&self, #[associated_enum(E)] value: Number); fn Other(&self, #[associated_enum(E)] value: Number); }",
                false,
            ),
        ] {
            let mut reader = windows_rdl::reader();
            if external {
                reader.reference(&reference);
            } else {
                reader.input_text(aliases);
            }
            let result = reader
                .input_text(&format!(
                    "{SCHEMA} #[win32] mod Test {{ #[repr(u32)] enum E {{ One = 1 }} {item} }}"
                ))
                .output(&winmd)
                .write();
            if valid {
                result.unwrap_or_else(|error| panic!("{item}: {error}"));
            } else {
                let error = result.unwrap_err().to_string();
                assert!(
                    error.contains("requires integer storage"),
                    "{item}: {error}"
                );
            }
        }
    }
}
