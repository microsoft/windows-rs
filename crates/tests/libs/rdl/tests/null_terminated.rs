use windows_metadata::{HasAttributes, Value, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute NativeNullTerminatedAttribute {
        fn();
        OnEntry: bool,
        OnSuccess: bool,
        DoubleNullOnEntry: bool,
        DoubleNullOnSuccess: bool,
        Future: bool,
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
            let short = windows_rdl::emit::write_null_terminated(entry, success);
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
