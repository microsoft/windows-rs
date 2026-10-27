use windows_metadata::{HasAttributes, IntegerExpression, IntegerLiteral, IntegerRange, reader};

#[test]
fn ranges_roundtrip() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let directory = std::path::Path::new(env!("OUT_DIR")).join("integer-ranges");
    std::fs::create_dir_all(&directory).unwrap();
    let first = directory.join("first.winmd");
    let second = directory.join("second.winmd");
    let rdl = directory.join("roundtrip.rdl");
    let source = r#"
        #[win32] mod Test {
            type Count = u32;
            extern fn F(
                #[range(entry, 0..=14 - start)] count: Count,
                #[range(entry, >=0)] start: i32,
                #[range(entry, -2147483648i32..=2147483647)] signed: i32,
                #[range(entry, 0u32..=4294967295u32)] unsigned: u32,
                #[range(entry, -9223372036854775808i64..=9223372036854775807i64)] wide: i64,
                #[range(entry, 0u64..=18446744073709551615u64)] unsigned_wide: u64,
                #[range(success, ..=(return))] result_bound: i32,
                #[range(entry, <*limit)] indirect_bound: i32, limit: *const i32,
                #[range(success, 0..=start, indirect)] written: *mut i32,
            ) -> #[range(success, 0..=start)] i32;
            struct Packet {
                capacity: u32,
                #[range(0..capacity)] count: u32,
            }
        }
    "#;
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(source)
        .output(&first)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&first)
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&second)
        .write()
        .unwrap();
    let mut previous = None;
    for path in [first, second] {
        let index = reader::Index::read(path).unwrap();
        let mut values = Vec::new();
        for ty in index.types().filter(|ty| ty.namespace() == "Test") {
            for field in ty.fields() {
                if let Some(attr) = field.find_attribute("IntegerRangeAttribute") {
                    values.push((
                        ty.name().to_string(),
                        field.name().to_string(),
                        IntegerRange::from_values(&attr.value()).unwrap(),
                    ));
                }
            }
            for method in ty.methods() {
                for parameter in method.params() {
                    if let Some(attr) = parameter.find_attribute("IntegerRangeAttribute") {
                        values.push((
                            ty.name().to_string(),
                            parameter.name().to_string(),
                            IntegerRange::from_values(&attr.value()).unwrap(),
                        ));
                    }
                }
            }
        }
        values.sort_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
        assert_eq!(values.len(), 11);
        if let Some(previous) = &previous {
            assert_eq!(&values, previous);
        }
        previous = Some(values);
    }
    for parameter in [
        "#[range(entry, 0..1)] value: f32",
        "#[range(entry, 0..1)] value: *mut i32",
        "#[range(entry, 0..missing)] value: i32",
        "#[range(entry, 0..=2147483647 + 1)] value: i32",
        "#[range(entry, 0..=value + 1)] value: f32",
        "#[range(entry, 0..=(return))] value: i32",
        "#[range(success, 0..=1, indirect)] value: i32",
        "#[range(success, 0..=1, indirect)] value: *mut f32",
    ] {
        let source = format!("#[win32] mod Test {{ extern fn F({parameter}) -> i32; }}");
        assert!(
            windows_rdl::reader()
                .input(root.join("metadata\\metadata.rdl"))
                .input_text(&source)
                .output(directory.join("invalid.winmd"))
                .write()
                .is_err(),
            "{parameter}"
        );
    }
    for result in [
        "#[range(entry, 0..=capacity)] i32",
        "#[range(success, 0..=capacity, indirect)] *mut i32",
        "#[range(success, 0..=capacity)] f32",
        "#[range(success, 0..=missing)] i32",
    ] {
        let source = format!("#[win32] mod Test {{ extern fn F(capacity: i32) -> {result}; }}");
        assert!(
            windows_rdl::reader()
                .input(root.join("metadata\\metadata.rdl"))
                .input_text(&source)
                .output(directory.join("invalid.winmd"))
                .write()
                .is_err(),
            "{result}",
        );
    }
}

#[test]
fn integer_expression_encoding() {
    let overflow = "v1 i32:2147483647 i32:1 add";
    assert!(IntegerExpression::decode(overflow).is_none());
    assert!(IntegerExpression::decode("v1 i32:50000 i32:50000 mul").is_none());
    for (encoded, expected) in [
        ("v1 u32:4294967295 i32:1 add", IntegerLiteral::U32(0)),
        ("v1 u32:0 i32:1 sub", IntegerLiteral::U32(u32::MAX)),
        (
            "v1 u32:4294967295 i64:1 add",
            IntegerLiteral::I64(4294967296),
        ),
        ("v1 i64:-1 u64:1 add", IntegerLiteral::U64(0)),
        (
            "v1 u32:4294967295 i32:2 mul",
            IntegerLiteral::U32(4294967294),
        ),
    ] {
        let expression = IntegerExpression::decode(encoded).unwrap();
        assert_eq!(expression.constant_value().unwrap(), Some(expected));
        assert_eq!(expression.encode().unwrap(), encoded);
    }
    for text in ["v2 i32:1", "v1 i32:01", "v1  i32:1", "v1 p:0 i32:1 div"] {
        assert!(IntegerExpression::decode(text).is_none(), "{text}");
    }
}
