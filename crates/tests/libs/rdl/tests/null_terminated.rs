use windows_metadata::{HasAttributes, Value, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute NullTerminatedAttribute {
        fn();
        OnEntry: bool,
        OnSuccess: bool,
        DoubleNullOnEntry: bool,
        DoubleNullOnSuccess: bool,
        Indirect: bool,
        Future: bool,
    }
    attribute NullableAttribute {
        fn();
    }
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
    let full = "#[Windows::Win32::Metadata::NullTerminated(OnEntry = true, OnSuccess = false)]";
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
                    #[in] #[null_terminated(entry)] text: TextAlias,
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
            .contains("#[null_terminated(")
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
        assert_eq!(text.matches("#[null_terminated(").count(), 1, "{text}");
        assert!(text.contains("#[in] #[null_terminated(entry)] text: TextAlias"));
        assert_eq!(text.matches("::NullTerminated(").count(), 3, "{text}");
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
fn termination_preserves_phases_and_properties() {
    let mut full = format!("{SCHEMA} #[win32] mod Test {{");
    let mut compact = full.clone();
    let mut expected = Vec::new();
    for entry in [None, Some(false), Some(true)] {
        for success in [None, Some(false), Some(true)] {
            if entry.is_none() && success.is_none() {
                continue;
            }
            let index = expected.len();
            let short = windows_rdl::emit::write_null_terminated(entry, success, false);
            let mut properties = format!(
                "OnEntry = {}, OnSuccess = {}",
                entry.is_some(),
                success.is_some()
            );
            let mut values = vec![
                ("OnEntry".into(), Value::Bool(entry.is_some())),
                ("OnSuccess".into(), Value::Bool(success.is_some())),
            ];
            for (phase, double) in [
                ("DoubleNullOnEntry", entry),
                ("DoubleNullOnSuccess", success),
            ] {
                if double == Some(true) {
                    properties.push_str(&format!(", {phase} = true"));
                    values.push((phase.into(), Value::Bool(true)));
                }
            }
            expected.push(values);
            full.push_str(&format!(
                "extern fn Callback{index}(#[out] #[opt] #[Windows::Win32::Metadata::NullTerminated({properties})] value: *mut u16);"
            ));
            compact.push_str(&format!(
                "extern fn Callback{index}(#[out] #[opt] {short} value: *mut u16);"
            ));
        }
    }
    for source in [&mut full, &mut compact] {
        for (index, properties) in [
            "OnEntry = true",
            "OnEntry = true, OnSuccess = false, DoubleNullOnEntry = false",
            "OnEntry = false, OnSuccess = true, DoubleNullOnEntry = true",
            "OnEntry = false, OnSuccess = false",
            "OnEntry = true, OnSuccess = false, Future = true",
        ]
        .iter()
        .enumerate()
        {
            source.push_str(&format!(
                "extern fn Full{index}(#[Windows::Win32::Metadata::NullTerminated({properties})] value: *mut u16);"
            ));
        }
        source.push('}');
    }
    let output = std::path::Path::new(env!("OUT_DIR"));
    let mut canonical = None;
    for (index, source) in [full, compact].iter().enumerate() {
        let before = output.join(format!("null-terminated-{index}.winmd"));
        let rdl = before.with_extension("rdl");
        let after = before.with_extension("again.winmd");
        windows_rdl::reader()
            .input_text(source)
            .output(&before)
            .write()
            .unwrap();
        windows_rdl::writer()
            .input(&before)
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(&rdl).unwrap();
        assert_eq!(text.matches("#[null_terminated(").count(), 8);
        assert_eq!(
            text.matches("#[Windows::Win32::Metadata::NullTerminated(")
                .count(),
            5
        );
        if let Some(canonical) = &canonical {
            assert_eq!(&text, canonical);
        }
        canonical = Some(text);
        windows_rdl::reader()
            .input(&rdl)
            .output(&after)
            .write()
            .unwrap();
        let mut full_properties = None;
        for path in [before, after] {
            let metadata = reader::Index::read(path).unwrap();
            for (index, expected) in expected.iter().enumerate() {
                let method = metadata
                    .expect("Test", &format!("Callback{index}"))
                    .methods()
                    .next()
                    .unwrap();
                let parameter = method.params().next().unwrap();
                assert!(parameter.is_optional());
                assert_eq!(parameter.direction(), reader::ParamDirection::Output);
                assert_eq!(
                    &parameter
                        .find_attribute("NullTerminatedAttribute")
                        .unwrap()
                        .value(),
                    expected
                );
            }
            let properties: Vec<_> = (0..5)
                .map(|index| {
                    metadata
                        .expect("Test", &format!("Full{index}"))
                        .methods()
                        .next()
                        .unwrap()
                        .params()
                        .next()
                        .unwrap()
                        .find_attribute("NullTerminatedAttribute")
                        .unwrap()
                        .value()
                })
                .collect();
            if let Some(expected) = &full_properties {
                assert_eq!(&properties, expected);
            }
            full_properties = Some(properties);
        }
    }
}

#[test]
fn termination_requires_unambiguous_phases() {
    for attribute in [
        "null_terminated",
        "null_terminated()",
        "null_terminated = true",
        "null_terminated(unknown)",
        "null_terminated(entry, entry)",
        "null_terminated(success, success)",
        "null_terminated(entry, double_entry)",
        "null_terminated(double_entry, entry)",
        "null_terminated(success, double_success)",
        "null_terminated(entry = true)",
        "null_terminated(indirect)",
        "null_terminated(success, indirect, indirect)",
        "null_terminated(success, indirect)",
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
        "extern fn F(#[null_terminated(entry)] value: u32);",
        "extern fn F(#[null_terminated(double_success)] value: f64);",
        "type Scalar = u32; extern fn F(#[null_terminated(entry)] value: Scalar);",
        "struct Value { value: u32, } extern fn F(#[null_terminated(entry)] value: Value);",
        "extern fn Callback(); extern fn F(#[null_terminated(entry)] value: Callback);",
        "extern fn F(#[null_terminated(entry)] value: extern \"C\" fn());",
        "extern fn F(#[null_terminated(success, indirect)] value: *mut u16);",
        "extern fn F() -> #[null_terminated(success)] *mut u16;",
        "extern fn F() -> #[null_terminated(entry)] *mut u16;",
        "#[null_terminated(success)] extern fn F();",
        "#[null_terminated(entry)] type Text = *const u16;",
        "#[null_terminated(entry)] struct Value { value: u32, }",
        "struct Value { #[null_terminated(entry)] value: *mut u16, }",
        "struct Value { #[null_terminated(double_success)] value: [u16; 8], }",
        "extern fn F(#[out] #[nullable(success, indirect)] value: *mut *mut u8);",
        "extern fn F(#[out] #[nullable()] value: *mut *mut u8);",
        "extern fn F(#[nullable] value: *mut *mut u8);",
        "extern fn F(#[out] #[nullable] value: *mut u8);",
        "extern fn F(#[out] #[nullable] value: u32);",
        "extern fn F() -> #[nullable] u32;",
        "extern fn F() -> #[nullable(success)] *mut u8;",
        "extern fn F() -> #[null_terminated(success, indirect)] *mut *mut u8;",
        "#[nullable] extern fn F();",
        "struct Value { #[nullable(entry)] value: *mut u8, }",
        "struct Value { #[nullable] value: u32, }",
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
fn nullable_shorthand_requires_valid_owner() {
    let full = "#[Windows::Win32::Metadata::Nullable]";
    let source = format!(
        r#"{SCHEMA}
        #[win32] mod Test {{
            extern fn Valid(#[out] {full} value: *mut *mut u8) -> {full} *mut u8;
            struct Record {{ {full} valid: *mut u8, {full} scalar: u32, }}
            extern fn Input(#[in] {full} value: *mut *mut u8);
            extern fn Direct(#[out] {full} value: *mut u8);
            extern fn Scalar() -> {full} u32;
        }}
        "#
    );
    let out = std::path::Path::new(env!("OUT_DIR")).join("nullable-owners");
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
    assert_eq!(text.matches("#[nullable]").count(), 3, "{text}");
    assert_eq!(text.matches(full).count(), 4, "{text}");
    windows_rdl::reader()
        .input_text(SCHEMA)
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
}

#[test]
fn termination_storage_and_legacy_owners_roundtrip() {
    let full = "#[Windows::Win32::Metadata::NullTerminated(OnEntry = false, OnSuccess = true, DoubleNullOnSuccess = true)]";
    let indirect = "#[Windows::Win32::Metadata::NullTerminated(OnEntry = false, OnSuccess = true, Indirect = true)]";
    let source = format!(
        r#"{SCHEMA}
        #[win32] mod Test {{
            type Text = *const u16;
            type TextAlias = Text;
            type ReturnedText = *mut TextAlias;
            extern fn Valid(
                #[null_terminated(entry, double_success)] direct: TextAlias,
                #[null_terminated(double_entry)] array: [u16; 8],
                #[null_terminated(entry)] reference: &u8,
                #[null_terminated(success, indirect)] returned: ReturnedText,
                #[null_terminated(double_success, indirect)] referenced: &mut Text,
            );
            {full} extern fn Callback();
            {full} type LegacyText = *const u16;
            struct Record {{ {full} text: *const u16, }}
            extern fn Legacy({full} scalar: i32, {indirect} single: *mut u16)
                -> {full} *const u16;
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
    assert_eq!(text.matches("#[null_terminated(").count(), 5, "{text}");
    assert_eq!(text.matches("::NullTerminated(").count(), 6, "{text}");
    windows_rdl::reader()
        .input_text(SCHEMA)
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let mut previous = None;
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        let mut contracts = Vec::new();
        let mut collect = |owner: String, attributes: Vec<reader::Attribute<'_>>| {
            for attr in attributes {
                if attr.name() == "NullTerminatedAttribute" {
                    contracts.push((owner.clone(), attr.value()));
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
        contracts.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(contracts.len(), 11);
        if let Some(previous) = &previous {
            assert_eq!(&contracts, previous);
        }
        previous = Some(contracts);
    }
}
