use windows_metadata::{BufferInfo, BufferSize, HasAttributes, Value, reader};

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..")
}

#[test]
fn roundtrip() {
    let source = r#"
        #[win32] mod Test {
            type Count = u32;
            type Buffer = *mut u8;
            extern fn Input(#[in] #[buffer(count)] data: *const u32, #[in] count: u32);
            extern fn Fixed(#[in] #[buffer(4)] data: *const u32);
            extern fn Output(
                #[out] #[buffer(bytes(*size))] data: Buffer,
                #[in] #[out] size: *mut Count,
            );
            extern fn Names(#[buffer(bytes)] data: Buffer, bytes: Count);
        }
    "#;
    let out = std::path::Path::new(env!("OUT_DIR")).join("buffers");
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
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input(&text)
        .output(&second)
        .write()
        .unwrap();
    let contracts = |path| {
        let index = reader::Index::read(path).unwrap();
        let mut contracts = Vec::new();
        for ty in index.types().filter(|ty| ty.namespace() == "Test") {
            for method in ty.methods() {
                for param in method.params() {
                    if let Some(attr) = param.find_attribute("BufferAttribute") {
                        contracts.push((
                            ty.name().to_string(),
                            BufferInfo::from_values(&attr.value()).unwrap(),
                        ));
                    }
                }
            }
        }
        contracts.sort_by_key(|value| format!("{value:?}"));
        contracts
    };
    let before = contracts(first);
    assert_eq!(before, contracts(second));
    assert!(before.iter().any(|(name, info)| name == "Output"
        && info.bytes
        && info.count == BufferSize::Dereference(1)));
    assert!(before.iter().any(|(name, info)| name == "Names"
        && !info.bytes
        && info.count == BufferSize::Parameter(1)));
}

#[test]
fn unsupported_buffer_attributes_keep_full_spelling() {
    let source = r#"
        #[win32] mod Test {
            extern fn Float(
                #[Windows::Win32::Metadata::Buffer(CountParamIndex = 1u16)] data: *mut u8,
                count: f32,
            );
            extern fn Optional(
                #[Windows::Win32::Metadata::Buffer(CountParamIndex = 1u16, Dereference = true)]
                data: *mut u8,
                #[opt] count: *mut u32,
            );
            extern fn Duplicate(
                #[Windows::Win32::Metadata::Buffer(CountConst = 1u64)]
                #[Windows::Win32::Metadata::Buffer(CountConst = 2u64)]
                data: *mut u8,
            );
        }
    "#;
    let path = std::path::Path::new(env!("OUT_DIR")).join("buffer-full.winmd");
    let text = path.with_extension("rdl");
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input_text(source)
        .output(&path)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&path)
        .filter("Test")
        .output(&text)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root().join("metadata\\metadata.rdl"))
        .input(&text)
        .output(&path)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(text).unwrap();
    assert_eq!(text.matches("::Buffer(").count(), 4, "{text}");
}

#[test]
fn boundaries() {
    for declaration in [
        "extern fn F(#[buffer(n + 1)] data: *mut u8, n: u32);",
        "extern fn F(#[buffer(if_present(*n))] data: *mut u8, n: *mut u32);",
        "extern fn F(#[buffer(return)] data: *mut u8) -> u32;",
        "extern fn F(#[buffer(*n)] data: *mut u8, n: u32);",
        "extern fn F(#[buffer(*n)] data: *mut u8, #[opt] n: *mut u32);",
        "extern fn F(#[buffer(n)] data: *mut u8, n: f32);",
        "extern fn F(#[buffer(n)] data: *mut u8, n: *mut u32);",
        "extern fn F(#[buffer(*n)] data: *mut u8, n: *mut f32);",
        "extern fn F(#[buffer(n)] data: u8, n: u32);",
        "extern fn F(#[buffer(missing)] data: *mut u8);",
        "extern fn F(#[buffer(1)] #[buffer(2)] data: *mut u8);",
        "extern fn F(#[buffer(1u32)] data: *mut u8);",
        "struct S { #[buffer(1)] data: *mut u8, }",
        "#[buffer(1)] struct S { data: *mut u8, }",
        "extern fn F(#[readable(entry, n)] data: *mut u8, n: u32);",
        "extern fn F(#[writable(entry, n)] data: *mut u8, n: u32);",
        "extern fn F(#[out] #[written(*n)] data: *mut u8, #[out] n: *mut u32);",
    ] {
        let source = format!("#[win32] mod Test {{ {declaration} }}");
        assert!(
            windows_rdl::reader()
                .input(root().join("metadata\\metadata.rdl"))
                .input_text(&source)
                .output(std::path::Path::new(env!("OUT_DIR")).join("invalid-buffer.winmd"))
                .write()
                .is_err(),
            "{declaration}"
        );
    }
}

#[test]
fn payload_validation() {
    let info = BufferInfo {
        count: BufferSize::Parameter(0),
        bytes: false,
    };
    assert!(windows_rdl::write_buffer_info(&info, &["count", "count"]).is_none());
    for property in [
        ("Unknown", Value::Bool(true)),
        ("CountConst", Value::U64(4)),
        ("CountParamIndex", Value::U16(1)),
        ("CountReturn", Value::Bool(true)),
    ] {
        let mut values = info.to_values();
        values.push((property.0.into(), property.1));
        assert!(BufferInfo::from_values(&values).is_none(), "{values:?}");
    }
    assert!(
        BufferInfo::from_values(&[
            ("CountConst".into(), Value::U64(4)),
            ("Dereference".into(), Value::Bool(true)),
        ])
        .is_none()
    );
}
