use std::path::{Path, PathBuf};
use windows_metadata::{reader::*, *};

fn path(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_values_{name}.winmd"))
}

fn compile(source: &str, output: &Path) {
    windows_rdl::reader()
        .input_text(source)
        .output(output)
        .write()
        .unwrap();
}

fn field<'a>(index: &'a Index, namespace: &str, name: &str) -> Field<'a> {
    let Item::Const(field) = index.get_item(namespace, name).next().unwrap() else {
        panic!("expected a constant");
    };
    field
}

fn check(index: &Index, namespace: &str) {
    index.validate_native_contracts(None).unwrap();
    let value = field(index, namespace, "Raw16");
    assert!(value.constant().is_none());
    assert_eq!(
        value.flags(),
        FieldAttributes::Public | FieldAttributes::Static
    );
    assert_eq!(value.ty(), Type::ArrayFixed(Box::new(Type::U16), 4));
    assert_eq!(
        value.native_value().unwrap(),
        Some(NativeValue::String {
            encoding: NativeStringEncoding::Utf16,
            units: vec![55296, 0, 56320, 0],
        })
    );
    assert_eq!(
        field(index, namespace, "Narrow").native_value().unwrap(),
        Some(NativeValue::String {
            encoding: NativeStringEncoding::Narrow,
            units: vec![233, 0, 65]
        })
    );
    let NativeValue::Scalar(Value::F32(value)) = field(index, namespace, "Float")
        .native_value()
        .unwrap()
        .unwrap()
    else {
        panic!("expected float bits");
    };
    assert_eq!(value.to_bits(), 0x7fc00001);
    let NativeValue::Record(values) = field(index, namespace, "Nested")
        .native_value()
        .unwrap()
        .unwrap()
    else {
        panic!("expected record");
    };
    assert_eq!(
        values
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>(),
        ["inner", "values", "text"]
    );
    let NativeValue::Record(inner) = &values[0].1 else {
        panic!("expected nested record")
    };
    let NativeValue::Scalar(Value::F64(value)) = inner[1].1 else {
        panic!("expected float bits")
    };
    assert_eq!(value.to_bits(), (-0.0f64).to_bits());
}

#[test]
fn reviewed_values_preserve_types_members_code_units_and_float_bits() {
    let source = include_str!("../native_values.rdl");
    let input = path("reviewed");
    compile(source, &input);
    check(&Index::read(&input).unwrap(), "Test");
    let rdl = input.with_extension("rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    let rendered = std::fs::read_to_string(rdl).unwrap();
    assert_eq!(rendered, source.replace("\r\n", "\n"));
    let directory = Path::new(env!("OUT_DIR")).join("native_values_roundtrip");
    std::fs::create_dir_all(&directory).unwrap();
    let recompiled = directory.join(input.file_name().unwrap());
    compile(&rendered, &recompiled);
    assert_eq!(
        std::fs::read(&input).unwrap(),
        std::fs::read(recompiled).unwrap()
    );
    let merged = path("merged");
    merge().input(&input).output(&merged).merge().unwrap();
    check(&Index::read(&merged).unwrap(), "Test");
    let remapped = path("remapped");
    merge::Remapper::new()
        .input(&merged)
        .source("Test")
        .route("Wide", "Chars")
        .route("Inner", "Records")
        .route("Outer", "Records")
        .fallback("Values")
        .output(&remapped)
        .remap()
        .unwrap();
    let index = Index::read(&remapped).unwrap();
    check(&index, "Values");
    assert_eq!(
        field(&index, "Values", "Nested").ty(),
        Type::value_named("Records", "Outer")
    );
    let rdl = remapped.with_extension("rdl");
    windows_rdl::writer()
        .input(&remapped)
        .output(&rdl)
        .write()
        .unwrap();
    let recompiled = path("remapped_roundtrip");
    compile(&std::fs::read_to_string(rdl).unwrap(), &recompiled);
    check(&Index::read(&recompiled).unwrap(), "Values");
}

#[test]
fn invalid_values_reject_before_output_replacement() {
    for (declaration, diagnostic) in [
        ("const V: [u8; 2] = [1u8];", "length"),
        ("const V: [u8; 1] = [1i8];", "declared type"),
        ("const V: [u8; 1] = [1];", "type suffix"),
        ("const V: R = record { x: 1i32 };", "cover members"),
        ("const V: R = record { x: 1i32, x: 2i32 };", "unique"),
        (
            "const V: R = record { y: 1u16, x: 1i32 };",
            "declaration order",
        ),
        ("const V: R = record { x: 1i32, y: 1i16 };", "declared type"),
        (
            "const V: R = record { x: 1i32, y: 1u16, ..Other };",
            "every member",
        ),
        ("const V: U = record { x: 1i32 };", "non-union"),
        ("const V: O = record {};", "opaque"),
        ("const V: *const u16 = utf8([255]);", "encoding"),
        ("const V: [u16; 2] = utf16([0]);", "matching array"),
        ("const V: *const u16 = utf16([65536]);", "value not valid"),
        (
            "const V: *const *const u8 = utf8([0]);",
            "character pointer",
        ),
        ("const V: [i32; 1] = [address(Symbol)];", "constructor"),
        ("const V: [i32; 1] = [1i32 + 1i32];", "expression"),
        ("const V: [i32; 1] = [\"text\"];", "expression"),
        ("const V: [u8; 1] = [256u8];", "value not valid"),
        ("const V: [i8; 1] = [128i8];", "value not valid"),
        (
            "const V: *const u32 = utf32([4294967296]);",
            "value not valid",
        ),
        ("const V: f32 = f32bits(4294967296);", "value not valid"),
        (
            "#[arch(X86)] const V: [usize; 1] = [4294967296usize];",
            "architecture width",
        ),
        (
            "const V: [f32; 1] = [1e100f32];",
            "explicit bit constructors",
        ),
    ] {
        let output = path("invalid");
        std::fs::write(&output, b"last good output").unwrap();
        let result = windows_rdl::reader()
            .input_text(&format!(
                "#[win32] mod Test {{ struct R {{ x:i32,y:u16 }} union U {{ x:i32 }}
            #[opaque] struct O {{}} {declaration} }}"
            ))
            .output(&output)
            .write();
        let error = result.unwrap_err().to_string();
        assert!(error.contains(diagnostic), "{declaration}: {error}");
        assert_eq!(std::fs::read(output).unwrap(), b"last good output");
    }
    let output = path("winrt");
    assert!(
        windows_rdl::reader()
            .input_text("#[winrt] mod Test { const V: [u8;1] = [1u8]; }")
            .output(output)
            .write()
            .unwrap_err()
            .to_string()
            .contains("native Apis")
    );
}

#[test]
fn architecture_merge_preserves_distinct_values_and_member_evidence() {
    let mut merger = merge();
    for arch in [1, 2] {
        let input = path(&format!("arch_{arch}"));
        windows_rdl::reader()
            .input_text(include_str!("../../../../../metadata/metadata.rdl"))
            .input_text(&format!(
                "#[win32] mod Test {{
            #[Windows::Win32::Metadata::NativeAnnotation(\"CXX\",0,\"profile {arch}\")]
            const Same: [u8;1] = [1u8];
            const Different: [u8;1] = [{arch}u8];
            }}"
            ))
            .output(&input)
            .write()
            .unwrap();
        merger.arch_input(input, arch);
    }

    let output = path("arch_merged");
    merger.output(&output).merge().unwrap();
    let index = Index::read(&output).unwrap();
    for name in ["Same", "Different"] {
        assert_eq!(index.get_item("Test", name).count(), 2);
    }
    for arch in [1, 2] {
        let index = Index::new_for_architecture(vec![File::read(&output).unwrap()], arch);
        assert_eq!(
            field(&index, "Test", "Different").native_value().unwrap(),
            Some(NativeValue::Array(vec![NativeValue::Scalar(Value::U8(
                arch as u8
            ))]))
        );
        assert_eq!(
            field(&index, "Test", "Same")
                .find_attribute("NativeAnnotationAttribute")
                .unwrap()
                .value()[2]
                .1,
            Value::Utf8(format!("profile {arch}"))
        );
    }
}

#[test]
fn native_guid_and_property_key_values_preserve_declared_types_arches_and_evidence() {
    let declarations = "#[win32] mod Types {
            struct GUID { data1:u32,data2:u16,data3:u16,data4:[u8;8] }
            struct PROPERTYKEY { fmtid:GUID,pid:u32 }
        }";
    let mut merger = merge();
    for arch in [1, 2] {
        let input = path(&format!("guid_{arch}"));
        windows_rdl::reader()
            .input_text(include_str!("../../../../../metadata/metadata.rdl"))
            .input_text(declarations)
            .input_text(&format!(
                "#[win32] mod Test {{
                #[Windows::Win32::Metadata::NativeAnnotation(\"CXX\",0,\"UUID {arch}\")]
                #[arch({})] const Id: Types::GUID = {arch};
                #[Windows::Win32::Metadata::NativeAnnotation(\"CXX\",0,\"KEY {arch}\")]
                #[guid({arch})] const Key: Types::PROPERTYKEY = 7;
                }}",
                if arch == 1 { "X86" } else { "X64" }
            ))
            .output(&input)
            .write()
            .unwrap();
        let index = Index::read(&input).unwrap();
        assert_eq!(field(&index, "Test", "Id").arches(), arch);
        let rdl = input.with_extension("rdl");
        windows_rdl::writer()
            .input(&input)
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(rdl).unwrap();
        assert!(text.contains("const Id: Types::GUID"));
        assert!(text.contains("const Key: Types::PROPERTYKEY"));
        let directory = Path::new(env!("OUT_DIR")).join(format!("native_values_guid_{arch}"));
        std::fs::create_dir_all(&directory).unwrap();
        let recompiled = directory.join(input.file_name().unwrap());
        compile(&text, &recompiled);
        assert_eq!(
            std::fs::read(&input).unwrap(),
            std::fs::read(recompiled).unwrap()
        );
        merger.arch_input(input, arch);
    }
    let output = path("guid_merged");
    merger.output(&output).merge().unwrap();
    let index = Index::read(&output).unwrap();
    for name in ["Id", "Key"] {
        assert_eq!(index.get_item("Test", name).count(), 2);
    }
    for arch in [1, 2] {
        let index = Index::new_for_architecture(vec![File::read(&output).unwrap()], arch);
        let id = field(&index, "Test", "Id");
        assert_eq!(id.ty(), Type::value_named("Types", "GUID"));
        assert_eq!(
            id.find_attribute("GuidAttribute").unwrap().value()[10].1,
            Value::U8(arch as u8)
        );
        assert_eq!(
            id.find_attribute("NativeAnnotationAttribute")
                .unwrap()
                .value()[2]
                .1,
            Value::Utf8(format!("UUID {arch}"))
        );
        assert_eq!(
            field(&index, "Test", "Key").constant().unwrap().value(),
            Value::U32(7)
        );
    }
}

#[test]
fn aliases_reference_values_and_architecture_specific_character_types_are_checked() {
    let reference = path("reference");
    compile(
        "#[win32] mod External {
            type Signed = i8;
            type Wide = u16;
            type Pointer = *const Wide;
            #[repr(i32)] enum Kind { A=1 }
            struct Record { kind:Kind, bytes:[Signed;1] }
            type Alias = Record;
            #[arch(X86)] type Char = i8;
            #[arch(X64)] type Char = u8;
        }",
        &reference,
    );
    let source = "#[win32] mod Test {
            const Record: External::Alias = record { kind:1i32, bytes:[-128i8] };
            const Text: External::Pointer = utf16([55296,0]);
            const Bytes: *const External::Char = utf8([255,0]);
            const Records: [External::Record;1] = [record { kind:1i32, bytes:[-1i8] }];
        }";
    let output = path("reference_values");
    windows_rdl::reader()
        .reference(&reference)
        .input_text(source)
        .output(&output)
        .write()
        .unwrap();
    let index = Index::read(&output).unwrap();
    index
        .validate_native_contracts(Some(&Index::read(&reference).unwrap()))
        .unwrap();
    assert_eq!(
        field(&index, "Test", "Record").ty(),
        Type::value_named("External", "Alias")
    );
    let rdl = output.with_extension("rdl");
    windows_rdl::writer()
        .input(&output)
        .input(&reference)
        .filter("Test")
        .output(&rdl)
        .write()
        .unwrap();
    let text = std::fs::read_to_string(rdl).unwrap();
    assert!(!text.contains("mod External"));
    let recompiled = path("reference_roundtrip");
    windows_rdl::reader()
        .reference(&reference)
        .input_text(&text)
        .output(&recompiled)
        .write()
        .unwrap();
    assert_eq!(
        field(&index, "Test", "Record").native_value().unwrap(),
        field(&Index::read(recompiled).unwrap(), "Test", "Record")
            .native_value()
            .unwrap()
    );
}
