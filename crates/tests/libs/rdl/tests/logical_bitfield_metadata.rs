use std::path::{Path, PathBuf};
use windows_metadata::*;

fn path(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("logical_bitfield_metadata_{name}"))
}

fn attribute(file: &mut writer::File, owner: writer::HasAttribute, name: &str, args: &[Value]) {
    let reference = file.TypeRef("Windows.Win32.Metadata", name);
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: args.iter().map(Value::ty).collect(),
    };
    let ctor = file.MemberRef(
        ".ctor",
        &signature,
        writer::MemberRefParent::TypeRef(reference),
    );
    let values: Vec<_> = args
        .iter()
        .cloned()
        .map(|value| (String::new(), value))
        .collect();
    file.Attribute(owner, writer::AttributeType::MemberRef(ctor), &values);
}

fn fixture(case: &str) -> writer::File {
    let mut file = writer::File::new("logical_bitfield_metadata");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let record = file.TypeDef(
        "Test",
        "Packet",
        base,
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    let flags = match case {
        "static" => FieldAttributes::Public | FieldAttributes::Static,
        "literal" => FieldAttributes::Public | FieldAttributes::Literal,
        _ => FieldAttributes::Public,
    };
    let field = file.Field("bits", &Type::U8, flags);
    if case == "alias_owner" {
        attribute(
            &mut file,
            writer::HasAttribute::TypeDef(record),
            "NativeTypedefAttribute",
            &[],
        );
    }
    let owner = if case == "owner" {
        writer::HasAttribute::TypeDef(record)
    } else {
        writer::HasAttribute::Field(field)
    };
    let name = if case == "padding" { "_" } else { "value" };
    let offset = if case == "negative_offset" { -1 } else { 0 };
    let width = match case {
        "zero_width" => 0,
        "capacity" => 9,
        _ => 3,
    };
    let interval = [
        Value::Utf8(name.to_string()),
        Value::I64(offset),
        Value::I64(width),
    ];
    if case != "orphan" {
        attribute(&mut file, owner, "NativeBitfieldAttribute", &interval);
    }
    if case == "duplicate" {
        attribute(&mut file, owner, "NativeBitfieldAttribute", &interval);
    }
    if case == "overlap" {
        attribute(
            &mut file,
            owner,
            "NativeBitfieldAttribute",
            &[
                Value::Utf8("other".to_string()),
                Value::I64(2),
                Value::I64(2),
            ],
        );
    }
    if !matches!(case, "qualifier_without_type" | "legacy") {
        attribute(
            &mut file,
            owner,
            "NativeBitfieldTypeAttribute",
            &[
                Value::Utf8(name.to_string()),
                Value::TypeName(TypeName::named(
                    "System",
                    if case == "float" { "Single" } else { "SByte" },
                )),
            ],
        );
    }
    if case == "duplicate_type" {
        attribute(
            &mut file,
            owner,
            "NativeBitfieldTypeAttribute",
            &[
                Value::Utf8(name.to_string()),
                Value::TypeName(TypeName::named("System", "Int16")),
            ],
        );
    }
    if matches!(case, "duplicate_qualifier" | "duplicate_annotation") {
        for _ in 0..2 {
            if case == "duplicate_qualifier" {
                attribute(
                    &mut file,
                    owner,
                    "NativeBitfieldQualifierAttribute",
                    &[Value::Utf8(name.to_string()), Value::I32(0), Value::I32(1)],
                );
            } else {
                attribute(
                    &mut file,
                    owner,
                    "NativeBitfieldAnnotationAttribute",
                    &[
                        Value::Utf8(name.to_string()),
                        Value::Utf8("CXX".to_string()),
                        Value::I32(0),
                        Value::Utf8("payload".to_string()),
                    ],
                );
            }
        }
    }
    if matches!(
        case,
        "qualifier_gap" | "qualifier_without_type" | "qualifier_unknown"
    ) {
        attribute(
            &mut file,
            owner,
            "NativeBitfieldQualifierAttribute",
            &[
                Value::Utf8(name.to_string()),
                Value::I32(if case == "qualifier_gap" { 1 } else { 0 }),
                Value::I32(if case == "qualifier_unknown" { 3 } else { 1 }),
            ],
        );
    }
    if case == "orphan_annotation" {
        attribute(
            &mut file,
            owner,
            "NativeBitfieldAnnotationAttribute",
            &[
                Value::Utf8("missing".to_string()),
                Value::Utf8("CXX".to_string()),
                Value::I32(0),
                Value::Utf8("payload".to_string()),
            ],
        );
    }
    file
}

#[test]
fn malformed_raw_descriptors_fail_before_checked_emission_or_output_cleanup() {
    for (case, diagnostic) in [
        ("padding", "cannot name padding"),
        ("negative_offset", "invalid bit-field descriptor"),
        ("zero_width", "invalid bit-field descriptor"),
        ("capacity", "exceeds backing storage"),
        ("orphan", "no matching member"),
        ("duplicate", "duplicate bit-field member"),
        ("duplicate_type", "duplicate bit-field logical type"),
        (
            "duplicate_qualifier",
            "duplicate bit-field qualifier ordinal",
        ),
        (
            "duplicate_annotation",
            "duplicate bit-field annotation identity",
        ),
        ("overlap", "overlapping"),
        ("float", "integral or enum"),
        ("qualifier_gap", "contiguous"),
        ("qualifier_without_type", "require a logical type"),
        ("qualifier_unknown", "invalid bit-field descriptor"),
        ("orphan_annotation", "no matching member"),
        ("owner", "requires a backing field"),
        ("static", "record instance field"),
        ("literal", "record instance field"),
        ("alias_owner", "record instance field"),
    ] {
        let error = fixture(case).try_into_stream().unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        let input = path(&format!("{case}.winmd"));
        std::fs::write(&input, fixture(case).into_stream()).unwrap();
        let rendered = path(&format!("{case}.rdl"));
        std::fs::write(&rendered, b"last good output").unwrap();
        let error = windows_rdl::writer()
            .input(&input)
            .output(&rendered)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(&rendered).unwrap(), b"last good output");
        let split = path(&format!("{case}_split"));
        std::fs::create_dir_all(&split).unwrap();
        let sentinel = split.join("keep.rdl");
        std::fs::write(&sentinel, b"last good output").unwrap();
        windows_rdl::writer()
            .input(&input)
            .output(&split)
            .split()
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        windows_rdl::writer()
            .input(&input)
            .output(&split)
            .partition(
                [(String::from("Packet"), String::from("packet"))]
                    .into_iter()
                    .collect(),
            )
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        let merged = path(&format!("{case}_merged.winmd"));
        std::fs::write(&merged, b"last good output").unwrap();
        merge().input(&input).output(&merged).merge().unwrap_err();
        assert_eq!(std::fs::read(&merged).unwrap(), b"last good output");
        let remapped = path(&format!("{case}_remapped.winmd"));
        std::fs::write(&remapped, b"last good output").unwrap();
        merge::Remapper::new()
            .input(&input)
            .source("Test")
            .fallback("Routed")
            .output(&remapped)
            .remap()
            .unwrap_err();
        assert_eq!(std::fs::read(&remapped).unwrap(), b"last good output");
    }
}

#[test]
fn typed_and_legacy_descriptors_use_identical_interval_bytes_and_field_signatures() {
    let typed = fixture("valid").try_into_stream().unwrap();
    let index = reader::Index::new(vec![reader::File::new(typed).unwrap()]);
    let field = index.expect("Test", "Packet").fields().next().unwrap();
    assert_eq!(field.ty(), Type::U8);
    assert_eq!(field.bitfields().unwrap()[0].logical_type, Some(Type::I8));
    let interval = field
        .attributes()
        .find(|attr| attr.name() == "NativeBitfieldAttribute")
        .unwrap();
    assert_eq!(
        interval.value(),
        [
            (String::new(), Value::Utf8("value".to_string())),
            (String::new(), Value::I64(0)),
            (String::new(), Value::I64(3)),
        ]
    );
    fn bytes(mut blob: reader::Blob<'_>) -> Vec<u8> {
        let bytes = blob.to_vec();
        while !blob.is_empty() {
            blob.read_u8();
        }
        bytes
    }
    let legacy = reader::Index::new(vec![
        reader::File::new(fixture("legacy").try_into_stream().unwrap()).unwrap(),
    ]);
    let legacy_field = legacy.expect("Test", "Packet").fields().next().unwrap();
    assert_eq!(legacy_field.bitfields().unwrap()[0].logical_type, None);
    assert_eq!(bytes(field.blob(2)), [0x06, 0x05]);
    assert_eq!(bytes(field.blob(2)), bytes(legacy_field.blob(2)));
    let legacy_interval = legacy_field
        .attributes()
        .find(|attr| attr.name() == "NativeBitfieldAttribute")
        .unwrap();
    assert_eq!(bytes(interval.blob(2)), bytes(legacy_interval.blob(2)));
}
