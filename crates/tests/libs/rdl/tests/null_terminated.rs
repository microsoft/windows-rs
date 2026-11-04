use windows_metadata::{HasAttributes, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute NullTerminatedAttribute {
        fn();
        OnEntry: bool,
        OnSuccess: bool,
    }
    attribute NullNullTerminatedAttribute { fn(); }
}}}
"#;

#[test]
fn writer_references_resolve_without_emitting_definitions() {
    let out = std::path::Path::new(env!("OUT_DIR")).join("writer-references");
    std::fs::create_dir_all(&out).unwrap();
    let reference = out.join("reference.winmd");
    let input = out.join("input.winmd");
    windows_rdl::reader()
        .input_text(&format!(
            r#"{SCHEMA}
            #[win32] mod Test {{
                type Text = *const u16;
                type TextAlias = Text;
                type Scalar = *const u16;
                type Record = *const u16;
                #[arch(X86)] type Variant = *const u16;
                #[arch(X64 | Arm64)] type Variant = u64;
                const Colliding: u32 = 10;
                #[repr(i32)] enum Shared {{ ReferenceMember = 2, }}
                attribute ChoiceAttribute {{ fn(#[in] value: Shared); }}
                #[library("reference.dll")] extern fn Call(value: i32);
            }}
            #[win32] mod ReferenceOnly {{ struct Hidden {{ value: i32, }} }}
            "#
        ))
        .output(&reference)
        .write()
        .unwrap();
    let full = "#[Windows::Win32::Metadata::NullTerminated]";
    windows_rdl::reader()
        .reference(&reference)
        .input_text(&format!(
            r#"#[win32] mod Test {{
                type Scalar = u32;
                struct Record {{ value: u32, }}
                #[Choice(2)] struct Colliding {{ value: u32, }}
                #[repr(i32)] enum Shared {{ LocalMember = 1, }}
                #[library("local.dll")]
                extern fn Call(
                    #[in] #[null_terminated] text: TextAlias,
                    {full} scalar: Scalar,
                    {full} record: Record,
                    {full} variant: Variant,
                );
            }}"#
        ))
        .output(&input)
        .write()
        .unwrap();

    let unresolved = out.join("unresolved.rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&unresolved)
        .write()
        .unwrap();
    assert!(
        !std::fs::read_to_string(unresolved)
            .unwrap()
            .contains("#[null_terminated]")
    );

    let mut expected = None;
    for mode in ["single", "split", "partition"] {
        let destination = if mode == "single" {
            out.join("single.rdl")
        } else {
            out.join(mode)
        };
        let mut writer = windows_rdl::writer();
        writer
            .input_bytes(&std::fs::read(&input).unwrap())
            .references([&reference])
            .output(&destination);
        let path = match mode {
            "split" => {
                writer.split();
                destination.join("Test.rdl")
            }
            "partition" => {
                writer.partition(
                    [
                        "Call",
                        "Scalar",
                        "Record",
                        "Colliding",
                        "Shared",
                        "Text",
                        "Hidden",
                    ]
                    .map(|name| (name.to_string(), "test".to_string()))
                    .into(),
                );
                destination.join("test.rdl")
            }
            _ => destination.clone(),
        };
        writer.write().unwrap();
        if mode != "single" {
            assert_eq!(std::fs::read_dir(&destination).unwrap().count(), 1);
        }
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text.matches("#[null_terminated]").count(), 1, "{text}");
        assert!(text.contains("#[in] #[null_terminated] text: TextAlias"));
        assert_eq!(text.matches(full).count(), 3, "{text}");
        assert!(text.contains("LocalMember = 1"));
        assert!(text.contains("struct Colliding"));
        for absent in [
            "ReferenceMember",
            "ReferenceOnly",
            "reference.dll",
            "type Text",
        ] {
            assert!(!text.contains(absent), "{text}");
        }
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        }
        expected = Some(text);
        let again = out.join(format!("{mode}.winmd"));
        windows_rdl::reader()
            .input(&path)
            .reference(&reference)
            .output(&again)
            .write()
            .unwrap();
        let before = reader::Index::read(&input).unwrap();
        let after = reader::Index::read(&again).unwrap();
        assert_eq!(
            before
                .expect("Test", "Colliding")
                .find_attribute("ChoiceAttribute")
                .unwrap()
                .value(),
            after
                .expect("Test", "Colliding")
                .find_attribute("ChoiceAttribute")
                .unwrap()
                .value()
        );
        let before = before.expect("Test", "Apis").methods().next().unwrap();
        let after = after.expect("Test", "Apis").methods().next().unwrap();
        assert_eq!(before.signature(&[]), after.signature(&[]));
        for (before, after) in before.params().zip(after.params()) {
            assert_eq!(before.flags(), after.flags());
            assert_eq!(
                before
                    .find_attribute("NullTerminatedAttribute")
                    .unwrap()
                    .value(),
                after
                    .find_attribute("NullTerminatedAttribute")
                    .unwrap()
                    .value()
            );
        }
    }
}

#[test]
fn termination_rejects_arguments() {
    for attribute in [
        "null_terminated()",
        "null_terminated = true",
        "null_terminated(entry)",
        "null_terminated(double_success)",
        "null_terminated(success, indirect)",
        "null_null_terminated()",
        "null_null_terminated = true",
        "null_null_terminated(entry)",
    ] {
        assert!(
            windows_rdl::reader()
                .input_text(&format!(
                    "{SCHEMA} #[win32] mod Test {{ extern fn Callback(#[{attribute}] value: *mut u16); }}"
                ))
                .output(std::path::Path::new(env!("OUT_DIR")).join("bad-termination.winmd"))
                .write()
                .is_err(),
            "{attribute}"
        );
    }
}

#[test]
fn pointer_contract_owners_are_checked() {
    for declaration in [
        "extern fn F(#[null_terminated] value: u32);",
        "extern fn F(#[null_null_terminated] value: f64);",
        "type Scalar = u32; extern fn F(#[null_terminated] value: Scalar);",
        "struct Value { value: u32, } extern fn F(#[null_terminated] value: Value);",
        "extern fn Callback(); extern fn F(#[null_terminated] value: Callback);",
        "extern fn F(#[null_null_terminated] value: extern \"C\" fn());",
        "extern fn F() -> #[null_terminated] *mut u16;",
        "extern fn F() -> #[null_null_terminated] *mut u16;",
        "#[null_terminated] extern fn F();",
        "#[null_null_terminated] type Text = *const u16;",
        "#[null_terminated] struct Value { value: u32, }",
        "struct Value { #[null_terminated] value: *mut u16, }",
        "struct Value { #[null_null_terminated] value: [u16; 8], }",
        "extern fn F(#[null_terminated] #[null_terminated] value: *const u16);",
        "extern fn F(#[null_null_terminated] #[null_null_terminated] value: *const u16);",
        "extern fn F(#[null_terminated] #[null_null_terminated] value: *const u16);",
    ] {
        assert!(
            windows_rdl::reader()
                .input_text(&format!("{SCHEMA} #[win32] mod Test {{ {declaration} }}"))
                .output(std::path::Path::new(env!("OUT_DIR")).join("bad-pointer-contract.winmd"))
                .write()
                .is_err(),
            "{declaration}"
        );
    }
}

#[test]
fn termination_storage_and_full_attributes_roundtrip() {
    let full = "#[Windows::Win32::Metadata::NullTerminated]";
    let double = "#[Windows::Win32::Metadata::NullNullTerminated]";
    let legacy = "#[Windows::Win32::Metadata::NullTerminated(OnEntry = true, OnSuccess = false)]";
    let source = format!(
        r#"{SCHEMA}
        #[win32] mod Test {{
            type Text = *const u16;
            type TextAlias = Text;
            extern fn Valid(
                #[in] #[opt] #[null_terminated] direct: TextAlias,
                #[out] #[null_null_terminated] array: [u16; 8],
                #[null_terminated] reference: &u8,
                {double} pointers: *const *const u8,
                {full} text: *const u16,
            );
            {full} extern fn Callback();
            {full} type LegacyText = *const u16;
            struct Record {{ {full} text: *const u16, }}
            extern fn Legacy({full} scalar: i32, {legacy} text: *const u16)
                -> {full} *const u16;
            extern fn Repeated({full} {full} value: *const u16);
            extern fn Combined({full} {double} value: *const u16);
        }}
        "#
    );
    let out = std::path::Path::new(env!("OUT_DIR")).join("termination-storage");
    std::fs::create_dir_all(&out).unwrap();
    let first = out.join("first.winmd");
    let rdl = out.join("roundtrip.rdl");
    let second = out.join("second.winmd");
    windows_rdl::reader()
        .input_text(&source)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(&rdl).unwrap();
    assert_eq!(text.matches("#[null_terminated]").count(), 3, "{text}");
    assert_eq!(text.matches("#[null_null_terminated]").count(), 2, "{text}");
    assert_eq!(text.matches(full).count(), 8, "{text}");
    assert_eq!(text.matches(double).count(), 1, "{text}");
    assert!(text.contains(legacy), "{text}");
    windows_rdl::reader()
        .input_text(SCHEMA)
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let mut previous = None;
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        let method = index.expect("Test", "Valid").methods().next().unwrap();
        let mut parameters = method.params();
        let input = parameters.next().unwrap();
        assert!(input.is_optional());
        assert_eq!(input.direction(), reader::ParamDirection::Input);
        assert_eq!(
            parameters.next().unwrap().direction(),
            reader::ParamDirection::Output
        );
        assert_eq!(
            parameters.next().unwrap().direction(),
            reader::ParamDirection::Unspecified
        );
        let mut contracts = Vec::new();
        let mut collect = |owner: String, attributes: Vec<reader::Attribute<'_>>| {
            for attr in attributes {
                if matches!(
                    attr.name(),
                    "NullTerminatedAttribute" | "NullNullTerminatedAttribute"
                ) {
                    contracts.push((owner.clone(), attr.name().to_string(), attr.value()));
                }
            }
        };
        for ty in index.types().filter(|ty| ty.namespace() == "Test") {
            collect(ty.name().to_string(), ty.attributes().collect());
            for field in ty.fields() {
                collect(
                    format!("{}.{}", ty.name(), field.name()),
                    field.attributes().collect(),
                );
            }
            for method in ty.methods() {
                collect(
                    format!("{}.{}", ty.name(), method.name()),
                    method.attributes().collect(),
                );
                for param in method.params() {
                    collect(
                        format!("{}.{}.{}", ty.name(), method.name(), param.sequence()),
                        param.attributes().collect(),
                    );
                }
            }
        }
        contracts.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
        assert_eq!(contracts.len(), 15);
        if let Some(previous) = &previous {
            assert_eq!(&contracts, previous);
        }
        previous = Some(contracts);
    }
}
