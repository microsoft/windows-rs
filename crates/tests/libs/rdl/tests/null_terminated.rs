use windows_metadata::{HasAttributes, Value, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute NativeNullTerminatedAttribute {
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
        OnEntry: bool,
        OnSuccess: bool,
        Indirect: bool,
    }
}}}
"#;

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
                "extern fn Callback{index}(#[out] #[opt] #[Windows::Win32::Metadata::NativeNullTerminated({properties})] value: *mut u16);"
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
                "extern fn Full{index}(#[Windows::Win32::Metadata::NativeNullTerminated({properties})] value: *mut u16);"
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
            text.matches("#[Windows::Win32::Metadata::NativeNullTerminated(")
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
                        .find_attribute("NativeNullTerminatedAttribute")
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
                        .find_attribute("NativeNullTerminatedAttribute")
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
        "nullable",
        "nullable()",
        "nullable(double_entry)",
        "nullable(entry, entry)",
        "nullable(success, indirect, indirect)",
        "nullable(entry, indirect)",
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
        "extern fn F(#[nullable(entry)] value: u32);",
        "extern fn F() -> #[nullable] u32;",
        "extern fn F() -> #[nullable(entry)] *mut u8;",
        "extern fn F() -> #[null_terminated(success, indirect)] *mut *mut u8;",
        "#[nullable(entry)] extern fn F();",
        "struct Value { #[nullable(entry)] value: *mut u8, }",
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
fn termination_storage_and_legacy_owners_roundtrip() {
    let full = "#[Windows::Win32::Metadata::NativeNullTerminated(OnEntry = false, OnSuccess = true, DoubleNullOnSuccess = true)]";
    let indirect = "#[Windows::Win32::Metadata::NativeNullTerminated(OnEntry = false, OnSuccess = true, Indirect = true)]";
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
    assert_eq!(text.matches("::NativeNullTerminated(").count(), 6, "{text}");
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
                if attr.name() == "NativeNullTerminatedAttribute" {
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
