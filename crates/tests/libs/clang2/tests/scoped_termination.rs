use windows_clang2::*;
use windows_metadata::{HasAttributes, Value, reader};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn check_output(name: &str, snapshots: &[Snapshot]) {
    let imports = snapshots
        .iter()
        .flat_map(|snapshot| &snapshot.values)
        .map(|value| {
            (
                value.name.clone(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(snapshots).unwrap(), "Fixture", &imports).unwrap();
    let rdl: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    let path = test_clang2::input_dir().join(format!("{name}.rdl"));
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(&path, &rdl).unwrap();
    }
    assert_eq!(
        rdl,
        std::fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join(format!("target\\test-clang2-{name}.winmd"));
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
        for (owner, method, entry, success, double_entry, double_success) in [
            ("Apis", "InputSingle", true, false, false, false),
            ("Apis", "OutputSingle", false, true, false, false),
            ("Apis", "InputMulti", true, false, true, false),
            ("Apis", "OutputMulti", false, true, false, true),
            ("Apis", "Both", true, true, true, true),
            ("Apis", "Mixed", true, true, false, true),
            ("MultiCallback", "Invoke", false, true, false, true),
            ("IMultiString", "Get", false, true, false, true),
        ] {
            let parameter = index
                .expect("Fixture", owner)
                .methods()
                .find(|value| value.name() == method)
                .unwrap()
                .params()
                .next()
                .unwrap();
            let attribute = parameter
                .find_attribute("NativeNullTerminatedAttribute")
                .unwrap();
            let mut expected = vec![
                ("OnEntry".into(), Value::Bool(entry)),
                ("OnSuccess".into(), Value::Bool(success)),
            ];
            if double_entry {
                expected.push(("DoubleNullOnEntry".into(), Value::Bool(true)));
            }
            if double_success {
                expected.push(("DoubleNullOnSuccess".into(), Value::Bool(true)));
            }
            assert_eq!(attribute.value(), expected, "{owner}::{method}");
            assert!(parameter.find_attribute("NativeSalAttribute").is_none());
        }
        for name in [
            "OtherProperty",
            "Conditional",
            "Grouped",
            "Indirect",
            "Modified",
            "Bare",
            "QualifiedContract",
        ] {
            let parameter = index
                .expect("Fixture", "Apis")
                .methods()
                .find(|method| method.name() == name)
                .unwrap()
                .params()
                .next()
                .unwrap();
            assert!(
                parameter
                    .find_attribute("NativeNullTerminatedAttribute")
                    .is_none(),
                "{name}"
            );
            let evidence: Vec<_> = parameter
                .attributes()
                .filter(|attribute| attribute.name() == "NativeSalAttribute")
                .flat_map(|attribute| attribute.value())
                .collect();
            assert!(!evidence.is_empty(), "{name}");
            let modifiers = match name {
                "OtherProperty" | "Grouped" | "QualifiedContract" => Some("_Post_"),
                "Modified" => Some("_Post_ _Notref_"),
                _ => None,
            };
            assert_eq!(
                evidence
                    .iter()
                    .find(|(name, _)| name == "Modifiers")
                    .map(|(_, value)| value),
                modifiers.map(|value| Value::Utf8(value.into())).as_ref(),
            );
        }
    }
    let roundtrip = root.join(format!("target\\test-clang2-{name}.rdl"));
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
fn scoped_termination_metadata() {
    helpers::ensure_libclang();
    let snapshots = capture(&[test_clang2::fixture("scoped_termination")], &TARGETS).unwrap();
    check_output("scoped_termination", &snapshots);
}

#[test]
fn modifiers_bind_before_comparison() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::selected_fixture("scoped_termination", &["Mixed"]),
        test_clang2::selected_fixture("scoped_termination", &["Mixed"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(
        0,
        "#define FIRST_PHASE _Post_\n#define SECOND_PHASE _Pre_\n",
    );
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS)
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("Mixed") && error.contains("differ"),
            "{error}"
        );
        inputs.reverse();
    }
}

#[test]
fn complete_scoped_pairs_may_reorder() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::selected_fixture("scoped_termination", &["Both"]),
        test_clang2::selected_fixture("scoped_termination", &["Both"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(
        0,
        "#define FIRST_PHASE _Post_\n#define SECOND_PHASE _Pre_\n",
    );
    for _ in 0..2 {
        let captured = capture_partial(&inputs, &TARGETS).unwrap();
        assert!(captured.omissions.is_empty());
        inputs.reverse();
    }
}

#[test]
fn typedef_scopes_require_an_observed_covering_set() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::selected_fixture("scoped_termination", &["ScopedAlias"]),
        test_clang2::selected_fixture("scoped_termination", &["ScopedAlias"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source.insert_str(
        0,
        "#define FIRST_PHASE _Post_\n#define SECOND_PHASE _Pre_\n",
    );
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture")
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("no observed SAL annotation set") && error.contains("ScopedAlias"),
            "{error}"
        );
        inputs.reverse();
    }
}

#[test]
fn scoped_termination_requires_buffer_storage() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("scoped_termination", &["Invalid"]);
    input.source.insert_str(0, "#define INVALID_BUFFER\n");
    let error = projection::generate(&[input], "Fixture")
        .err()
        .unwrap()
        .to_string();
    assert!(error.contains("requires a buffer pointer"), "{error}");
}

#[test]
#[ignore = "requires the pinned SDK and clang resource headers"]
fn sdk_preserves_phase_markers() {
    helpers::ensure_libclang();
    let fixture = test_clang2::fixture("scoped_termination");
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = fixture.name;
    input.source = format!("#define SDK_SAL\n{}", fixture.source);
    input.roots.clear();
    input.root_paths = Some(Vec::new());
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for (name, expected) in [
            ("OutputMulti", vec!["_Post_", "_NullNull_terminated_"]),
            (
                "OtherProperty",
                vec!["_Post_", "_Valid_", "_NullNull_terminated_"],
            ),
        ] {
            let value = snapshot
                .values
                .iter()
                .find(|value| value.name == name)
                .unwrap();
            for observation in &value.observations {
                let ValueKind::Function { parameters, .. } = &observation.kind else {
                    panic!()
                };
                let actual: Vec<_> = parameters[0]
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
                assert_eq!(actual, expected);
            }
            check_output("scoped_termination_sdk", &snapshots);
        }
    }
}
