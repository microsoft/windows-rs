use windows_clang::*;
use windows_metadata::{HasAttributes, Type, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn golden(name: &str, output: &projection::Output) -> String {
    let actual: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG_RDL").is_some() {
        std::fs::write(&path, &actual).unwrap();
    }
    assert_eq!(
        actual,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    actual
}

#[test]
fn clang_flag_enum_preserves_flags_attribute() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("enum_flag_attribute");
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert_eq!(
            snapshot
                .enums
                .iter()
                .filter(|value| value.observations.iter().any(|observation| {
                    observation
                        .attributes
                        .iter()
                        .any(|attribute| attribute.kind == AttributeKind::FlagEnum)
                }))
                .count(),
            4
        );
    }
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(output.enums, 5);
    let rdl = golden("enum_flag_attribute", &output);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-flag-enum.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    for architecture in [1, 2, 4] {
        let index = reader::Index::new_for_architecture(
            vec![reader::File::read(&winmd).unwrap()],
            architecture,
        );
        for name in [
            "NativeFlags",
            "PlainEnum",
            "ScopedFlags",
            "GnuFlags",
            "SIGNED_FLAGS",
        ] {
            let definition = index.expect("Fixture", name);
            assert_eq!(
                definition
                    .attributes()
                    .any(|a| a.name() == "FlagsAttribute"),
                name != "PlainEnum"
            );
            assert_eq!(
                definition
                    .attributes()
                    .any(|a| a.name() == "ScopedEnumAttribute"),
                name == "ScopedFlags"
            );
            assert_eq!(
                definition.underlying_type(),
                Some(if matches!(name, "GnuFlags" | "SIGNED_FLAGS") {
                    Type::I32
                } else {
                    Type::U32
                })
            );
        }
        let signed = index.expect("Fixture", "SIGNED_FLAGS");
        let all = signed
            .fields()
            .find(|field| field.name() == "SignedAll")
            .unwrap();
        assert_eq!(all.constant().unwrap().value(), Value::I32(-1));
        let native = index.expect("Fixture", "NativeFlags");
        let values: Vec<_> = native
            .fields()
            .filter_map(|field| field.constant().map(|value| value.value()))
            .collect();
        assert_eq!(
            values,
            [Value::U32(0), Value::U32(1), Value::U32(2), Value::U32(3)]
        );
    }
    let roundtrip = root.join("target\\test-clang-flag-enum.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), rdl);
    std::fs::remove_file(winmd).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

#[test]
fn flags_conflict_remains_fatal_before_pruning() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::fixture("enum_flag_attribute"),
        test_clang::fixture("enum_flag_attribute"),
    ];
    inputs[1].name = "plain.h".into();
    inputs[1].source.insert_str(0, "#define FLAG_ATTRIBUTE\n");
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
        assert!(
            error.contains("attributes") && error.contains("NativeFlags"),
            "{error}"
        );
        inputs.reverse();
    }
}

fn imports(names: &[&str]) -> projection::Imports {
    names
        .iter()
        .map(|name| {
            (
                (*name).into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect()
}

#[test]
fn concrete_classes_preserve_public_data() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("data_classes")], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for name in ["Property", "CProperty", "BitmapData"] {
            let record = snapshot
                .records
                .iter()
                .find(|record| record.name == name)
                .unwrap();
            assert_eq!(record.kind, RecordKind::Class);
            assert!(record.observations.iter().all(|id| {
                snapshot.observations[id.0]
                    .definition
                    .as_ref()
                    .unwrap()
                    .methods
                    .is_empty()
            }));
        }
    }
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports(&["UseProperty", "UseBitmapData"]),
    )
    .unwrap();
    assert_eq!(output.interfaces, 1);
    assert_eq!(output.functions, 2);
    golden("data_classes", &output);
}

#[test]
fn private_templates_do_not_promote_members() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("template_private_records");
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    assert!(!captured.omissions.is_empty());
    assert!(captured.omissions.iter().all(|message| {
        (message.contains("ItemFactory") || message.contains("PartialFactory"))
            && !message.contains("PRIVATE_")
    }));
    for snapshot in &captured.snapshots {
        assert!(
            snapshot
                .records
                .iter()
                .all(|record| !record.name.contains("PRIVATE_"))
        );
        assert!(
            snapshot
                .aliases
                .iter()
                .all(|alias| !alias.name.contains("PRIVATE_"))
        );
    }
    let imports = imports(&["Keep"]);
    let output = projection::project_partial_with_imports(captured, "Fixture", &imports)
        .unwrap()
        .output;
    assert_eq!(output.functions, 1);
    golden("template_private_records", &output);
    let input = test_clang::selected_fixture(
        "template_private_records",
        &["GLOBAL_ITEM", "GLOBAL_PTR", "NODE", "Keep"],
    );
    let snapshots = capture(&[input], &TARGETS).unwrap();
    assert_eq!(
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap(),
        output
    );
}

#[test]
fn non_finite_constants_are_explicit_omissions() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("non_finite_floats");
    input.macros = [
        "MACRO_INFINITY",
        "MACRO_NAN",
        "NARROWED_INFINITY",
        "FINITE_MACRO",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    for name in [
        "FLOAT_INFINITY",
        "DOUBLE_INFINITY",
        "NOT_A_NUMBER",
        "MACRO_INFINITY",
        "MACRO_NAN",
        "NARROWED_INFINITY",
        "SDK_POSITIVE_INFINITY",
        "SDK_NEGATIVE_INFINITY",
        "SDK_NAN",
    ] {
        assert!(
            captured
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
    for snapshot in &captured.snapshots {
        assert_eq!(snapshot.values.len(), 2);
        for value in &snapshot.values {
            let expected = match value.name.as_str() {
                "FINITE_FLOAT" => ConstantValue::Float32(1.5f32.to_bits()),
                "FINITE_MACRO" => ConstantValue::Float64(2.5f64.to_bits()),
                other => panic!("unexpected constant {other}"),
            };
            assert!(value.observations.iter().all(|observation| matches!(
                &observation.kind, ValueKind::Constant { value: Some(actual), .. } if actual == &expected
            )));
        }
    }
    let result = projection::project_partial(captured, "Fixture").unwrap();
    assert_eq!(result.output.constants, 2);
    golden("non_finite_floats", &result.output);
}

#[test]
fn identical_function_declarations_merge() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::fixture("duplicate_functions"),
        test_clang::fixture("duplicate_functions"),
    ];
    inputs[1].name = "second.h".into();
    let imports = imports(&["SharedFunction"]);
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 1);
    golden("duplicate_functions", &output);
    inputs.reverse();
    let snapshots = capture(&inputs, &TARGETS).unwrap();
    assert_eq!(
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap(),
        output
    );
    inputs[1].source.insert_str(0, "#define RESULT double\n");
    let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
    assert!(
        error.contains("SharedFunction") && error.contains("differ"),
        "{error}"
    );
}

#[test]
fn macro_expansion_keeps_distinct_native_link_names() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("macro_link_names");
    let imports = imports(&["PublicValue", "PrivateValue"]);
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports).unwrap();
    assert_eq!(output.functions, 2);
    golden("macro_link_names", &output);
    input.macros.push("PublicValue".into());
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let partial = projection::project_partial_with_imports(captured, "Fixture", &imports).unwrap();
    assert_eq!(partial.output, output);
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("collides with a native declaration"))
    );
}

#[test]
fn unnamed_enum_field_is_not_silently_flattened() {
    helpers::ensure_libclang();
    let input = test_clang::fixture("unnamed_enum_field");
    let captured = capture_partial(&[input], &TARGETS).unwrap();
    let partial = projection::project_partial(captured, "Fixture").unwrap();
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("WITH_ENUM")),
        "{:?}",
        partial.omissions
    );
    assert_eq!(partial.output.records, 1);
    assert_eq!(partial.output.enums, 0);
    assert_eq!(partial.output.constants, 0);
    golden("unnamed_enum_field", &partial.output);
}

#[test]
fn authored_type_name_does_not_imply_a_primitive() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("canonical_name_collision")], &TARGETS).unwrap();
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports(&["UseByte"]),
    )
    .unwrap();
    assert_eq!(output.records, 1);
    assert_eq!(output.functions, 1);
    golden("canonical_name_collision", &output);
}

#[test]
fn string_aliases_preserve_parameter_directions() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("string_alias_direction")], &TARGETS).unwrap();
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports(&["Strings"]),
    )
    .unwrap();
    assert_eq!(output.functions, 1);
    golden("string_alias_direction", &output);
}

#[test]
fn double_null_termination_preserves_success_scope() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang::fixture("nullnull_termination")], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let value = snapshot
            .values
            .iter()
            .find(|value| value.name == "MultiString")
            .unwrap();
        for observation in &value.observations {
            let ValueKind::Function { parameters, .. } = &observation.kind else {
                panic!()
            };
            let annotations: Vec<_> = parameters[0]
                .attributes
                .iter()
                .filter_map(|attribute| {
                    if let AttributeKind::Sal(value) = &attribute.kind {
                        Some(value.name.as_str())
                    } else {
                        None
                    }
                })
                .collect();
            assert_eq!(annotations, ["_Post_", "_NullNull_terminated_"]);
        }
    }
    let output = projection::project_with_imports(
        &merge(&snapshots).unwrap(),
        "Fixture",
        &imports(&["MultiString"]),
    )
    .unwrap();
    golden("nullnull_termination", &output);
}
