use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn path(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_value_metadata_{name}"))
}

fn fixture(case: &str) -> writer::File {
    let mut file = writer::File::new("native_value_metadata");
    let object = file.TypeRef("System", "Object");
    let flags = TypeAttributes::Public
        | TypeAttributes::Sealed
        | if case == "winrt" {
            TypeAttributes::WindowsRuntime
        } else {
            TypeAttributes::default()
        };
    let owner = file.TypeDef(
        "Test",
        if case == "container" { "Other" } else { "Apis" },
        writer::TypeDefOrRef::TypeRef(object),
        flags,
    );
    if case == "container_arch" {
        attr(
            &mut file,
            writer::HasAttribute::TypeDef(owner),
            "SupportedArchitectureAttribute",
            vec![(String::new(), Value::I32(1))],
        );
    }
    let ty = if case == "unavailable" {
        Type::value_named("External", "Missing")
    } else {
        Type::ArrayFixed(Box::new(Type::U16), 2)
    };
    let field = file.Field(
        "Value",
        &ty,
        FieldAttributes::Public
            | FieldAttributes::Static
            | if case == "literal" {
                FieldAttributes::Literal
            } else {
                FieldAttributes::default()
            },
    );
    if case == "constant" {
        file.Constant(writer::HasConstant::Field(field), &Value::U16(0));
    }
    let value = match case {
        "length" => NativeValue::Array(vec![NativeValue::Scalar(Value::U16(1))]),
        "scalar_type" => NativeValue::Array(vec![NativeValue::Scalar(Value::I16(1)); 2]),
        _ => NativeValue::Array(vec![NativeValue::Scalar(Value::U16(1)); 2]),
    };
    let payload = match case {
        "odd" => "0".into(),
        "hex" => "gg".into(),
        "uppercase" => "0A".into(),
        "empty" => String::new(),
        "tag" => "ff".into(),
        "boolean" => "0102".into(),
        "truncated" => "0401".into(),
        "trailing" => format!("{}00", value.to_payload().unwrap()),
        "count" => "14ffffffff".into(),
        "encoding" => "160400000000".into(),
        "duplicate_name" => "15020000000100000078020101000000780202".into(),
        _ => value.to_payload().unwrap(),
    };
    let target = if case == "owner" {
        writer::HasAttribute::TypeDef(owner)
    } else {
        writer::HasAttribute::Field(field)
    };
    let mut args = vec![
        (
            String::new(),
            Value::I32(if case == "version" { 2 } else { 1 }),
        ),
        (
            String::new(),
            if case == "argument_type" {
                Value::I32(0)
            } else {
                Value::Utf8(payload)
            },
        ),
    ];
    if case == "named" {
        args.push(("Extra".into(), Value::I32(0)));
    }
    attr(&mut file, target, "NativeConstantAttribute", args.clone());
    if case == "duplicate" {
        attr(&mut file, target, "NativeConstantAttribute", args);
    }
    if case == "guid" {
        attr(&mut file, target, "GuidAttribute", vec![]);
    }
    file
}

fn attr(
    file: &mut writer::File,
    owner: writer::HasAttribute,
    name: &str,
    args: Vec<(String, Value)>,
) {
    let ty = file.TypeRef("Windows.Win32.Metadata", name);
    let ctor = file.MemberRef(
        ".ctor",
        &Signature {
            flags: MethodCallAttributes::HASTHIS,
            return_type: Type::Void,
            types: args
                .iter()
                .filter(|(name, _)| name.is_empty())
                .map(|(_, value)| value.ty())
                .collect(),
        },
        writer::MemberRefParent::TypeRef(ty),
    );
    file.Attribute(owner, writer::AttributeType::MemberRef(ctor), &args);
}

#[test]
fn malformed_values_reject_before_emission_rendering_cleanup_merge_or_remap() {
    for (case, diagnostic) in [
        ("version", "version 1"),
        ("argument_type", "string payload"),
        ("named", "version 1"),
        ("duplicate", "duplicate"),
        ("owner", "requires a field"),
        ("winrt", "native Apis"),
        ("container", "native Apis"),
        ("container_arch", "neutral"),
        ("literal", "storage flags"),
        ("constant", "scalar"),
        ("guid", "GUID"),
        ("unavailable", "available type"),
        ("length", "length"),
        ("scalar_type", "declared type"),
        ("odd", "odd"),
        ("hex", "hexadecimal"),
        ("uppercase", "lowercase"),
        ("empty", "truncated"),
        ("tag", "tag"),
        ("boolean", "boolean"),
        ("truncated", "truncated"),
        ("trailing", "trailing"),
        ("count", "remaining"),
        ("encoding", "encoding"),
        ("duplicate_name", "unique"),
    ] {
        let error = fixture(case).try_into_stream().unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        let input = path(&format!("{case}.winmd"));
        std::fs::write(&input, fixture(case).into_stream()).unwrap();
        let output = path(&format!("{case}.rdl"));
        std::fs::write(&output, b"last good output").unwrap();
        let error = windows_rdl::writer()
            .input(&input)
            .output(&output)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(output).unwrap(), b"last good output");
        let directory = path(&format!("{case}_split"));
        std::fs::create_dir_all(&directory).unwrap();
        let sentinel = directory.join("keep.rdl");
        std::fs::write(&sentinel, b"last good output").unwrap();
        windows_rdl::writer()
            .input(&input)
            .output(&directory)
            .split()
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        windows_rdl::writer()
            .input(&input)
            .output(&directory)
            .partition([("Value".into(), "values".into())].into_iter().collect())
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(sentinel).unwrap(), b"last good output");
        let output = path(&format!("{case}_merged.winmd"));
        std::fs::write(&output, b"last good output").unwrap();
        merge().input(&input).output(&output).merge().unwrap_err();
        assert_eq!(std::fs::read(&output).unwrap(), b"last good output");
        merge::Remapper::new()
            .input(&input)
            .source("Test")
            .fallback("Routed")
            .output(&output)
            .remap()
            .unwrap_err();
        assert_eq!(std::fs::read(output).unwrap(), b"last good output");
    }
}

#[test]
fn native_value_codec_is_checked_and_keeps_exact_scalar_bits() {
    for value in [
        Value::Bool(true),
        Value::U8(255),
        Value::I8(-128),
        Value::U16(65535),
        Value::I16(-32768),
        Value::U32(u32::MAX),
        Value::I32(i32::MIN),
        Value::U64(u64::MAX),
        Value::I64(i64::MIN),
        Value::USize(u64::MAX),
        Value::ISize(i64::MIN),
        Value::F32(f32::from_bits(0x7f800001)),
        Value::F64(f64::from_bits(0x7ff0000000000001)),
    ] {
        let value = NativeValue::Scalar(value);
        let payload = value.to_payload().unwrap();
        assert_eq!(
            NativeValue::from_payload(&payload)
                .unwrap()
                .to_payload()
                .unwrap(),
            payload
        );
    }
    assert_eq!(
        NativeValue::Scalar(Value::I32(-1)).to_payload().unwrap(),
        "07ffffffff"
    );
    assert_eq!(
        NativeValue::String {
            encoding: NativeStringEncoding::Utf16,
            units: vec![0xd800, 0],
        }
        .to_payload()
        .unwrap(),
        "16020200000000d80000"
    );
    for value in [
        NativeValue::Scalar(Value::Utf8("text".into())),
        NativeValue::String {
            encoding: NativeStringEncoding::Utf8,
            units: vec![256],
        },
        NativeValue::String {
            encoding: NativeStringEncoding::Utf16,
            units: vec![65536],
        },
        NativeValue::Record(vec![(String::new(), NativeValue::Array(vec![]))]),
    ] {
        assert!(value.to_payload().is_err());
    }
    let mut value = NativeValue::Scalar(Value::U8(0));
    for _ in 0..66 {
        value = NativeValue::Array(vec![value]);
    }
    assert!(
        value
            .to_payload()
            .unwrap_err()
            .to_string()
            .contains("nesting")
    );
    let payload = format!("{}0200", "1401000000".repeat(66));
    assert!(
        NativeValue::from_payload(&payload)
            .unwrap_err()
            .to_string()
            .contains("nesting")
    );
}
