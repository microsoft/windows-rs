use windows_metadata::{
    BufferAccess, BufferCount, BufferExtent, BufferPhase, BufferUnits, HasAttributes, reader,
};

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..")
}

#[test]
fn named_extents_roundtrip() {
    let source = r#"
        #[win32] mod Test {
            #[library("test.dll")]
            extern fn Query(
                #[out] #[opt] #[writable(entry, bytes(*size))]
                #[readable(success, bytes(*size))] data: *mut u8,
                #[in] #[out] size: *mut u32,
            );
            extern fn Fetch(
                #[writable(entry, capacity)] #[readable(success, or(*written, capacity))]
                data: *mut u16, capacity: u32, written: *mut u32,
            );
            extern fn Full(#[readable(success, on_entry(*count))] data: *mut u8, count: *mut u32);
            extern fn Path(#[readable(success, return + 1)] data: *mut u16) -> u32;
            extern fn Lock(#[writable(success, bytes(*size), indirect, nullable)] data: *mut *mut u8, size: *mut u32);
            extern fn Optional(
                #[writable(success, bytes(if_present(*capacity)), indirect)]
                #[readable(success, bytes(if_present(*length)), indirect)]
                data: *mut *mut u8, #[out] #[opt] capacity: *mut u32, #[out] #[opt] length: *mut u32,
            );
            extern fn OptionalIncoming(
                #[readable(success, on_entry(if_present(*count)))] data: *mut u8, #[opt] count: *mut u32,
            );
            struct Packet {
                used: u32,
                #[writable(126)] #[readable(used)] data: [u8; 126],
                #[writable(used, nullable)] extra: *mut u8,
            }
        }
    "#;
    let out = std::path::Path::new(env!("OUT_DIR")).join("buffer-extents");
    std::fs::create_dir_all(&out).unwrap();
    let first = out.join("first.winmd");
    let text = out.join("roundtrip.rdl");
    let second = out.join("second.winmd");
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input_text(source)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Test")
        .output(&text)
        .write()
        .unwrap();
    let rendered = std::fs::read_to_string(&text).unwrap();
    assert!(
        rendered.contains("readable(success, bytes(*size))"),
        "{rendered}"
    );
    assert!(rendered.contains("writable(126)"), "{rendered}");
    assert!(rendered.contains("readable(used)"), "{rendered}");
    assert!(
        rendered.contains("readable(success, on_entry(*count))"),
        "{rendered}"
    );
    assert!(!rendered.contains("BufferExtent"));
    assert!(
        rendered.contains("bytes(if_present(*capacity)), indirect"),
        "{rendered}"
    );
    assert!(
        rendered.contains("on_entry(if_present(*count))"),
        "{rendered}"
    );
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input(&text)
        .output(&second)
        .write()
        .unwrap();
    let mut expected = None;
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        let mut extents = Vec::new();
        for ty in index.types().filter(|ty| ty.namespace() == "Test") {
            for method in ty.methods() {
                for param in method.params() {
                    for attr in param
                        .attributes()
                        .filter(|a| a.name() == "BufferExtentAttribute")
                    {
                        extents.push((
                            format!("{}.{}", ty.name(), method.name()),
                            param.name().to_string(),
                            BufferExtent::from_values(&attr.value()).unwrap(),
                        ));
                    }
                }
            }
            for field in ty.fields() {
                for attr in field
                    .attributes()
                    .filter(|a| a.name() == "BufferExtentAttribute")
                {
                    extents.push((
                        ty.name().to_string(),
                        field.name().to_string(),
                        BufferExtent::from_values(&attr.value()).unwrap(),
                    ));
                }
            }
        }
        extents.sort_by_key(|value| format!("{value:?}"));
        assert_eq!(extents.len(), 13);
        assert_eq!(
            extents
                .iter()
                .filter(|(_, _, extent)| extent.count_optional)
                .count(),
            3
        );
        let mut full = BufferExtent::new(
            BufferAccess::Read,
            BufferPhase::Success,
            BufferUnits::Elements,
            BufferCount::Dereference(1),
        );
        full.count_on_entry = true;
        assert!(
            extents
                .iter()
                .any(|(owner, _, extent)| owner == "Full.Invoke" && extent == &full)
        );
        assert!(
            extents
                .iter()
                .any(|(owner, field, extent)| owner == "Packet"
                    && field == "data"
                    && extent
                        == &BufferExtent::new(
                            BufferAccess::Read,
                            BufferPhase::Value,
                            BufferUnits::Elements,
                            BufferCount::Field("used".into())
                        ))
        );
        assert!(extents.iter().any(|(_, _, e)| e
            == &BufferExtent::new(
                BufferAccess::Write,
                BufferPhase::Entry,
                BufferUnits::Bytes,
                BufferCount::Dereference(1)
            )));
        if let Some(expected) = &expected {
            assert_eq!(&extents, expected);
        }
        expected = Some(extents);
    }
}

#[test]
fn malformed_extents_are_rejected() {
    for attribute in [
        "readable(entry, missing)",
        "readable(success, **size)",
        "readable(value, size)",
        "readable(entry, return)",
        "readable(entry, on_entry(*size))",
        "readable(success, on_entry(on_entry(*size)))",
        "readable(success, old(*size))",
        "readable(success, if_present(size))",
        "readable(success, if_present(0))",
        "readable(success, if_present(return))",
        "readable(success, if_present(if_present(*size)))",
        "readable(success, if_present(or(*size, size)))",
        "readable(success, or(if_present(*size), size))",
        "readable(*size)",
        "readable(success, or(size, size))",
        "readable(success, bytes(*size), unknown)",
        "writable(entry, bytes(*size), indirect)",
        "readable(success, size - 1)",
    ] {
        let text = format!(
            "#[win32] mod Test {{ extern fn F(#[{attribute}] data: *mut u8, size: *mut u32); }}"
        );
        assert!(
            windows_rdl::reader()
                .input_text(&text)
                .input(root().join("metadata\\metadata.rdl"))
                .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-extent.winmd"))
                .write()
                .is_err(),
            "{attribute}"
        );
    }
    for attribute in [
        "writable(value, count)",
        "writable(entry, count)",
        "readable(success, count)",
        "readable(on_entry(count))",
        "readable(if_present(count))",
    ] {
        let text = format!(
            "#[win32] mod Test {{ struct S {{ count: u32, #[{attribute}] data: *mut u8, }} }}"
        );
        assert!(
            windows_rdl::reader()
                .input(root().join("metadata\\metadata.rdl"))
                .input_text(&text)
                .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-field-extent.winmd"))
                .write()
                .is_err(),
            "{attribute}"
        );
    }
}

#[test]
fn typed_count_and_storage_validation() {
    for declaration in [
        "extern fn F(#[readable(entry, count)] data: *mut u8, count: f32);",
        "extern fn F(#[readable(entry, count)] data: *mut u8, count: *mut u32);",
        "extern fn F(#[readable(entry, *count)] data: *mut u8, count: u32);",
        "extern fn F(#[readable(entry, *count)] data: *mut u8, count: *mut f32);",
        "extern fn F(#[readable(success, if_present(*count))] data: *mut u8, count: *mut f32);",
        "extern fn F(#[readable(success, return)] data: *mut u8) -> *mut u32;",
        "extern fn F(#[readable(success, return)] data: *mut u8);",
        "extern fn F(#[readable(entry, count)] data: u8, count: u32);",
        "extern fn F(#[readable(success, count, indirect)] data: *mut u8, count: u32);",
        "extern fn F(#[readable(success, or(*count, fallback))] data: *mut u8, count: *mut u32, fallback: *mut u32);",
        "struct S { count: f32, #[writable(count)] data: *mut u8, }",
        "struct Count { value: u32, } extern fn F(#[readable(entry, count)] data: *mut u8, count: Count);",
        "extern fn F(#[readable(entry, 18446744073709551615 + 1)] data: *mut u8);",
    ] {
        let source = format!("#[win32] mod Test {{ {declaration} }}");
        let error = windows_rdl::reader()
            .input(root().join("metadata\\metadata.rdl"))
            .input_text(&source)
            .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-typed-extent.winmd"))
            .write()
            .unwrap_err();
        assert!(
            error.to_string().contains("buffer"),
            "{declaration}: {error}"
        );
    }
    let source = "#[win32] mod Test {
        type Count = u32;
        type Buffer = *mut u8;
        extern fn F(#[readable(entry, bytes)] data: Buffer, bytes: Count);
        extern fn G(#[readable(success, on_entry)] data: Buffer, on_entry: Count);
        extern fn H(#[readable(entry, or)] data: Buffer, or: Count);
        extern fn I(#[readable(entry, if_present)] data: Buffer, if_present: Count);
        struct S {
            entry: Count,
            success: Count,
            value: Count,
            #[writable(bytes(entry))] #[readable(success)] data: Buffer,
            #[writable(value)] other: Buffer,
        }
    }";
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input_text(source)
        .output(std::path::Path::new(env!("OUT_DIR")).join("typed-extent.winmd"))
        .write()
        .unwrap();
}

#[test]
fn unrecognized_extent_payloads_keep_full_spelling() {
    let full = |phase: &str, properties: &str| {
        format!(
            "#[Windows::Win32::Metadata::BufferExtent(
            Read, {phase}, Elements, {properties})]"
        )
    };
    let source = format!(
        "#[win32] mod Test {{
        {} struct S {{
            {} data: *mut u8,
        }}
        extern fn F({} data: *mut u8, {} other: *mut u8, {} future: *mut u8);
    }}",
        full("Value", "CountConst = 4u64"),
        full("Value", "CountFieldName = \"missing.member\""),
        full("Entry", "CountConst = 4u64, Offset = 0u64"),
        full("Entry", "CountParamIndex = 500u16"),
        full("99", "CountConst = 4u64"),
    );
    let out = std::path::Path::new(env!("OUT_DIR")).join("unknown-buffer-extents");
    std::fs::create_dir_all(&out).unwrap();
    let first = out.join("first.winmd");
    let rdl = out.join("roundtrip.rdl");
    let second = out.join("second.winmd");
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
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
    assert_eq!(text.matches("::BufferExtent(").count(), 5, "{text}");
    assert!(!text.contains("#[readable("), "{text}");
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let values = |path| {
        let index = reader::Index::read(path).unwrap();
        let mut values = Vec::new();
        for ty in index.types().filter(|ty| ty.namespace() == "Test") {
            for attr in ty
                .attributes()
                .chain(ty.fields().flat_map(|field| field.attributes()))
                .chain(
                    ty.methods()
                        .flat_map(|method| method.params().flat_map(|param| param.attributes())),
                )
                .filter(|attr| attr.name() == "BufferExtentAttribute")
            {
                values.push(attr.value());
            }
        }
        values.sort_by_key(|value| format!("{value:?}"));
        assert_eq!(values.len(), 5);
        values
    };
    assert_eq!(values(first), values(second));
}

#[test]
fn extent_encoding_is_checked() {
    use windows_metadata::Value;
    let extent = BufferExtent::new(
        BufferAccess::Read,
        BufferPhase::Entry,
        BufferUnits::Elements,
        BufferCount::Parameter(0),
    );
    assert_eq!(
        BufferExtent::from_values(&extent.to_values()),
        Some(extent.clone())
    );
    assert!(windows_rdl::write_buffer_extent(&extent, &["count", "count"]).is_none());
    for property in [
        ("Unknown", Value::Bool(true)),
        ("Indirect", Value::Bool(true)),
        ("Nullable", Value::Bool(true)),
        ("CountConst", Value::U64(4)),
        ("CountParamIndex", Value::U16(1)),
        ("Dereference", Value::Bool(false)),
        ("CountOptional", Value::Bool(true)),
        ("CountOptional", Value::Bool(false)),
    ] {
        let mut values = extent.to_values();
        values.push((property.0.into(), property.1));
        assert!(BufferExtent::from_values(&values).is_none(), "{values:?}");
    }
}
