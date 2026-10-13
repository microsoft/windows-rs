use std::path::{Path, PathBuf};
use windows_metadata::*;

fn output(name: &str) -> PathBuf {
    Path::new(env!("OUT_DIR")).join(format!("native_annotation_metadata_{name}"))
}

fn attribute(
    file: &mut writer::File,
    owner: writer::HasAttribute,
    namespace: &str,
    name: &str,
    args: &[(String, Value)],
) {
    let reference = file.TypeRef(namespace, name);
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: args
            .iter()
            .filter(|(name, _)| name.is_empty())
            .map(|(_, value)| value.ty())
            .collect(),
    };
    let ctor = file.MemberRef(
        ".ctor",
        &signature,
        writer::MemberRefParent::TypeRef(reference),
    );
    file.Attribute(owner, writer::AttributeType::MemberRef(ctor), args);
}

fn fixture(case: &str) -> writer::File {
    let mut file = writer::File::new("native_annotation_metadata");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let mut flags = TypeAttributes::Public | TypeAttributes::SequentialLayout;
    if case == "winrt" {
        flags |= TypeAttributes::WindowsRuntime;
    }
    let record = file.TypeDef("Test", "Record", base, flags);
    let field = file.Field("data", &Type::I32, FieldAttributes::Public);
    let mut interface_flags =
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract;
    if case == "winrt" {
        interface_flags |= TypeAttributes::WindowsRuntime;
    }
    file.TypeDef("Test", "IContracts", Default::default(), interface_flags);
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![Type::PtrMut(Box::new(Type::I32), 1), Type::USize],
    };
    let method = file.MethodDef(
        "Read",
        &signature,
        MethodAttributes::Public
            | MethodAttributes::Virtual
            | MethodAttributes::Abstract
            | MethodAttributes::NewSlot
            | MethodAttributes::HideBySig,
        Default::default(),
    );
    file.Param("presentation_length", 2, ParamAttributes::In);
    let param = file.Param("presentation_buffer", 1, ParamAttributes::default());
    let owner = match case {
        "field_binding" => writer::HasAttribute::Field(field),
        "type_binding" => writer::HasAttribute::TypeDef(record),
        "invalid_owner" => writer::HasAttribute::TypeRef(file.TypeRef("Test", "Reference")),
        "method" => writer::HasAttribute::MethodDef(method),
        _ => writer::HasAttribute::Param(param),
    };
    let namespace = if case == "other_namespace" {
        "Other"
    } else {
        "Windows.Win32.Metadata"
    };
    let source = if case == "empty_source" { "" } else { "SAL" };
    let ordinal = if case == "negative_ordinal" { -1 } else { 0 };
    let mut evidence = vec![
        (String::new(), Value::Utf8(source.to_string())),
        (String::new(), Value::I32(ordinal)),
        (
            String::new(),
            Value::Utf8("_In_reads_(original_count)".to_string()),
        ),
    ];
    if case == "wrong_type" {
        evidence[1].1 = Value::U32(0);
    }
    if case == "empty_payload" {
        evidence[2].1 = Value::Utf8(String::new());
    }
    if case == "named_arg" {
        evidence.push(("Extra".to_string(), Value::I32(0)));
    }
    if case != "missing_evidence" {
        attribute(
            &mut file,
            owner,
            namespace,
            "NativeAnnotationAttribute",
            &evidence,
        );
    }
    if case == "duplicate_evidence" {
        attribute(
            &mut file,
            owner,
            namespace,
            "NativeAnnotationAttribute",
            &evidence,
        );
    }
    let position = match case {
        "negative_position" => -1,
        "out_of_range" => 2,
        _ => 1,
    };
    let identifier = if case == "empty_identifier" {
        ""
    } else {
        "original_count"
    };
    let binding = vec![
        (String::new(), Value::Utf8("SAL".to_string())),
        (String::new(), Value::I32(0)),
        (String::new(), Value::Utf8(identifier.to_string())),
        (String::new(), Value::I32(position)),
    ];
    attribute(
        &mut file,
        owner,
        namespace,
        "NativeAnnotationBindingAttribute",
        &binding,
    );
    if case == "duplicate_binding" {
        attribute(
            &mut file,
            owner,
            namespace,
            "NativeAnnotationBindingAttribute",
            &binding,
        );
    }
    file
}

#[test]
fn raw_metadata_checks_original_positions_not_physical_param_order_or_names() {
    for case in ["valid", "method", "other_namespace"] {
        let bytes = fixture(case).try_into_stream().unwrap();
        let index = reader::Index::new(vec![reader::File::new(bytes).unwrap()]);
        index.validate_native_annotations().unwrap();
        let method = index.expect("Test", "IContracts").methods().next().unwrap();
        let params = method.params_by_sequence(2).unwrap();
        assert_eq!(params.params()[0].unwrap().name(), "presentation_buffer");
        assert_eq!(params.params()[1].unwrap().name(), "presentation_length");
        assert_eq!(
            params.params()[0].unwrap().direction(),
            reader::ParamDirection::Unspecified
        );
    }
}

#[test]
fn raw_unspecified_rows_roundtrip_through_rdl_with_their_bindings() {
    let input = output("raw_roundtrip.winmd");
    std::fs::write(&input, fixture("valid").into_stream()).unwrap();
    let rdl = output("raw_roundtrip.rdl");
    windows_rdl::writer()
        .input(&input)
        .output(&rdl)
        .write()
        .unwrap();
    let vocabulary = output("raw_vocabulary.winmd");
    windows_rdl::reader()
        .input_text(include_str!("../../../../../metadata/metadata.rdl"))
        .output(&vocabulary)
        .write()
        .unwrap();
    let roundtrip = output("raw_roundtrip_again.winmd");
    windows_rdl::reader()
        .reference(&vocabulary)
        .input(&rdl)
        .output(&roundtrip)
        .write()
        .unwrap();
    for path in [&input, &roundtrip] {
        let index = reader::Index::read(path).unwrap();
        index.validate_native_annotations().unwrap();
        let method = index.expect("Test", "IContracts").methods().next().unwrap();
        let params = method.params_by_sequence(2).unwrap();
        let first = params.params()[0].unwrap();
        assert_eq!(first.direction(), reader::ParamDirection::Unspecified);
        let binding = first
            .attributes()
            .find(|attr| attr.name() == "NativeAnnotationBindingAttribute")
            .unwrap();
        assert_eq!(
            binding
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>(),
            [
                Value::Utf8("SAL".to_string()),
                Value::I32(0),
                Value::Utf8("original_count".to_string()),
                Value::I32(1),
            ]
        );
    }
}

#[test]
fn raw_invalid_contracts_fail_before_render_merge_remap_or_cleanup() {
    for (case, diagnostic) in [
        ("empty_source", "invalid native annotation arguments"),
        ("negative_ordinal", "invalid native annotation arguments"),
        ("wrong_type", "invalid native annotation arguments"),
        ("empty_payload", "invalid native annotation arguments"),
        ("empty_identifier", "invalid native annotation arguments"),
        ("named_arg", "arguments must be positional"),
        (
            "duplicate_evidence",
            "duplicate native annotation source/ordinal",
        ),
        (
            "duplicate_binding",
            "duplicate native annotation identifier",
        ),
        ("missing_evidence", "no matching evidence"),
        ("negative_position", "out of range"),
        ("out_of_range", "out of range"),
        ("field_binding", "requires a callable owner"),
        ("type_binding", "requires a callable owner"),
        ("invalid_owner", "requires a declaration owner"),
        ("winrt", "not valid on a WinRT"),
    ] {
        let error = fixture(case).try_into_stream().unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        let input = output(&format!("{case}.winmd"));
        std::fs::write(&input, fixture(case).into_stream()).unwrap();
        let rendered = output(&format!("{case}.rdl"));
        std::fs::write(&rendered, b"last good output").unwrap();
        let error = windows_rdl::writer()
            .input(&input)
            .output(&rendered)
            .write()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(&rendered).unwrap(), b"last good output");
        let split = output(&format!("{case}_split"));
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
                [(String::from("Record"), String::from("record"))]
                    .into_iter()
                    .collect(),
            )
            .write()
            .unwrap_err();
        assert_eq!(std::fs::read(&sentinel).unwrap(), b"last good output");
        let merged = output(&format!("{case}_merged.winmd"));
        std::fs::write(&merged, b"last good output").unwrap();
        let error = merge().input(&input).output(&merged).merge().unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(&merged).unwrap(), b"last good output");
        let remapped = output(&format!("{case}_remapped.winmd"));
        std::fs::write(&remapped, b"last good output").unwrap();
        let error = merge::Remapper::new()
            .input(&input)
            .source("Test")
            .fallback("Routed")
            .output(&remapped)
            .remap()
            .unwrap_err();
        assert!(error.to_string().contains(diagnostic), "{case}: {error}");
        assert_eq!(std::fs::read(&remapped).unwrap(), b"last good output");
    }
}

fn descriptor_fixture(case: &str, alias: bool) -> writer::File {
    let mut file = writer::File::new("native_descriptors");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    let record = file.TypeDef(
        "Test",
        "Record",
        base,
        TypeAttributes::Public | TypeAttributes::SequentialLayout,
    );
    let field = file.Field("value", &Type::I32, FieldAttributes::Public);
    file.TypeDef(
        "Test",
        "IContracts",
        Default::default(),
        TypeAttributes::Public | TypeAttributes::Interface | TypeAttributes::Abstract,
    );
    let buffer = if case == "scalar_owner" {
        Type::I32
    } else {
        Type::PtrMut(Box::new(Type::Void), 1)
    };
    let count = match case {
        "alias" => Type::value_named("Counts", "Count"),
        "float" => Type::F64,
        "two_pointers" => Type::PtrMut(Box::new(Type::U32), 2),
        "pointer" | "dereference" => Type::PtrMut(Box::new(Type::U32), 1),
        _ => Type::U32,
    };
    let signature = Signature {
        flags: MethodCallAttributes::HASTHIS,
        return_type: Type::Void,
        types: vec![buffer, count],
    };
    let method = file.MethodDef(
        "Read",
        &signature,
        MethodAttributes::Public
            | MethodAttributes::Virtual
            | MethodAttributes::Abstract
            | MethodAttributes::NewSlot
            | MethodAttributes::HideBySig,
        Default::default(),
    );
    let sequence = match case {
        "return" => 0,
        "sequence" => 3,
        _ => 1,
    };
    let param = file.Param("buffer", sequence, ParamAttributes::default());
    file.Param("count", 2, ParamAttributes::In);
    let owner = if alias {
        match case {
            "field_owner" => writer::HasAttribute::Field(field),
            "method_owner" => writer::HasAttribute::MethodDef(method),
            _ => writer::HasAttribute::TypeDef(record),
        }
    } else {
        match case {
            "field_owner" => writer::HasAttribute::Field(field),
            "method_owner" => writer::HasAttribute::MethodDef(method),
            _ => writer::HasAttribute::Param(param),
        }
    };
    let args = if alias {
        vec![
            (
                String::new(),
                Value::Utf8(
                    if case == "name" {
                        "not-an-identifier"
                    } else {
                        "ALIAS"
                    }
                    .into(),
                ),
            ),
            (
                String::new(),
                Value::Utf8(
                    if case == "kind" {
                        "FunctionDecl"
                    } else {
                        "StructDecl"
                    }
                    .into(),
                ),
            ),
            (
                String::new(),
                Value::Utf8(match case {
                    "profile" => "source-sha256=invalid\n-target".into(),
                    "no_arguments" => format!("source-sha256={}", "a".repeat(64)),
                    _ => format!(
                        "source-sha256={}\n-target\nx86_64-pc-windows-msvc",
                        "a".repeat(64)
                    ),
                }),
            ),
        ]
    } else {
        vec![
            (
                String::new(),
                Value::Utf8(if case == "phase" { "other" } else { "capacity" }.into()),
            ),
            (
                String::new(),
                Value::Utf8(if case == "units" { "other" } else { "bytes" }.into()),
            ),
            (
                String::new(),
                Value::I32(match case {
                    "negative" => -1,
                    "position" => 2,
                    _ => 1,
                }),
            ),
            (
                String::new(),
                Value::Bool(matches!(
                    case,
                    "dereference" | "two_pointers" | "not_pointer"
                )),
            ),
        ]
    };
    let name = if alias {
        "NativeMacroAliasAttribute"
    } else {
        "NativeBufferExtentAttribute"
    };
    attribute(&mut file, owner, "Windows.Win32.Metadata", name, &args);
    if case == "duplicate" {
        attribute(&mut file, owner, "Windows.Win32.Metadata", name, &args);
    }
    file
}

#[test]
fn buffer_and_alias_descriptors_validate_owners_types_positions_and_profiles() {
    for (alias, valid, cases) in [
        (
            false,
            &["valid", "dereference"][..],
            &[
                "scalar_owner",
                "float",
                "two_pointers",
                "pointer",
                "not_pointer",
                "return",
                "sequence",
                "field_owner",
                "method_owner",
                "phase",
                "units",
                "negative",
                "position",
                "duplicate",
            ][..],
        ),
        (
            true,
            &["valid"][..],
            &[
                "field_owner",
                "method_owner",
                "name",
                "kind",
                "profile",
                "no_arguments",
                "duplicate",
            ][..],
        ),
    ] {
        for case in valid {
            descriptor_fixture(case, alias).try_into_stream().unwrap();
        }

        for case in cases {
            let error = descriptor_fixture(case, alias)
                .try_into_stream()
                .unwrap_err();
            assert!(!error.to_string().is_empty(), "{alias}/{case}");
            let index = reader::Index::new(vec![
                reader::File::new(descriptor_fixture(case, alias).into_stream()).unwrap(),
            ]);
            assert!(
                index.validate_native_annotations().is_err(),
                "{alias}/{case}"
            );
        }
    }
}

#[test]
fn buffer_extent_integer_aliases_use_supplied_references() {
    let index = reader::Index::new(vec![
        reader::File::new(descriptor_fixture("alias", false).into_stream()).unwrap(),
    ]);
    assert!(index.validate_native_contracts(None).is_err());
    for (name, scalar, valid) in [("integer", "u32", true), ("float", "f64", false)] {
        let path = output(&format!("extent_reference_{name}.winmd"));
        windows_rdl::reader()
            .input_text(&format!("#[win32] mod Counts {{ type Count = {scalar}; }}"))
            .output(&path)
            .write()
            .unwrap();
        let reference = reader::Index::read(&path).unwrap();
        assert_eq!(
            index.validate_native_contracts(Some(&reference)).is_ok(),
            valid
        );
    }
}
