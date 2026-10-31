use windows_metadata::{HasAttributes, Value, reader};

const SCHEMA: &str = r#"
#[win32] mod Windows { mod Win32 { mod Metadata {
    attribute ReturnedPointerAttribute {
        fn();
        NullOnFailure: bool,
        Future: bool,
    }
}}}
"#;

#[test]
fn returned_pointer_spelling_preserves_properties() {
    let mut explicit = format!("{SCHEMA} #[win32] mod Test {{");
    let mut compact = explicit.clone();
    for (index, null_on_failure) in [false, true].into_iter().enumerate() {
        let short = windows_rdl::emit::write_returned_pointer(null_on_failure);
        explicit.push_str(&format!(
            "extern fn Callback{index}(#[out] #[opt] #[Windows::Win32::Metadata::ReturnedPointer(NullOnFailure = {null_on_failure})] value: *mut *mut u8);"
        ));
        compact.push_str(&format!(
            "extern fn Callback{index}(#[out] #[opt] {short} value: *mut *mut u8);"
        ));
    }
    for source in [&mut explicit, &mut compact] {
        source.push_str(
            r#"
            extern fn Missing(#[Windows::Win32::Metadata::ReturnedPointer()] value: *mut *mut u8);
            extern fn Future(#[Windows::Win32::Metadata::ReturnedPointer(NullOnFailure = true, Future = true)] value: *mut *mut u8);
            }"#,
        );
    }
    let output = std::path::Path::new(env!("OUT_DIR"));
    let mut expected = None;
    for (index, source) in [explicit, compact].iter().enumerate() {
        let winmd = output.join(format!("returned-pointers-{index}.winmd"));
        let rdl = winmd.with_extension("rdl");
        windows_rdl::reader()
            .input_text(source)
            .output(&winmd)
            .write()
            .unwrap();
        let metadata = reader::Index::read(&winmd).unwrap();
        for (index, null_on_failure) in [false, true].into_iter().enumerate() {
            let method = metadata
                .expect("Test", &format!("Callback{index}"))
                .methods()
                .next()
                .unwrap();
            let parameter = method.params().next().unwrap();
            assert!(parameter.is_optional());
            assert_eq!(parameter.direction(), reader::ParamDirection::Output);
            assert_eq!(
                parameter
                    .find_attribute("ReturnedPointerAttribute")
                    .unwrap()
                    .value(),
                vec![("NullOnFailure".into(), Value::Bool(null_on_failure))]
            );
        }
        windows_rdl::writer()
            .input(&winmd)
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(&rdl).unwrap();
        assert_eq!(text.matches("#[out_ptr").count(), 2);
        assert_eq!(
            text.matches("#[Windows::Win32::Metadata::ReturnedPointer")
                .count(),
            2
        );
        if let Some(expected) = &expected {
            assert_eq!(&text, expected);
        }
        expected = Some(text);
    }
}

#[test]
fn returned_pointer_flags_are_checked() {
    for attr in [
        "out_ptr(unknown)",
        "out_ptr(nullable)",
        "out_ptr(null_on_failure, null_on_failure)",
        "out_ptr = true",
        "out_ptr(null_on_failure = true)",
    ] {
        assert!(windows_rdl::reader()
            .input_text(&format!(
                "{SCHEMA} #[win32] mod Test {{ extern fn Callback(#[{attr}] value: *mut *mut u8); }}"
            ))
            .output(std::path::Path::new(env!("OUT_DIR")).join("bad-out-ptr.winmd"))
            .write()
            .is_err(), "{attr}");
    }
}
