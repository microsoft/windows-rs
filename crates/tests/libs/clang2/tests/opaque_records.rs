use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn input() -> Input {
    let mut input = test_clang2::fixture("opaque_records");
    input.macros = vec!["INVALID".into()];
    input
}

fn imports() -> projection::Imports {
    ["Echo", "Make", "Consume", "Inherit"]
        .into_iter()
        .map(|name| {
            (
                name.into(),
                projection::Import {
                    library: "fixture.dll".into(),
                    name: None,
                },
            )
        })
        .collect()
}

#[test]
fn nominal_pointees() {
    helpers::ensure_libclang();
    let snapshots = capture(&[input()], &TARGETS).unwrap();
    for snapshot in &snapshots {
        for record in &snapshot.records {
            if matches!(record.name.as_str(), "Alpha" | "Beta") {
                assert!(
                    record
                        .observations
                        .iter()
                        .all(|id| { snapshot.observations[id.0].definition.is_none() })
                );
            }
        }
    }
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Fixture", &imports())
            .unwrap();
    let expected = std::fs::read_to_string(test_clang2::input_dir().join("opaque_records.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["opaque_records"].rdl, expected);
    let reversed = capture(&[input()], &[Target::Arm64, Target::X64, Target::X86]).unwrap();
    assert_eq!(
        projection::project_with_imports(&merge(&reversed).unwrap(), "Fixture", &imports())
            .unwrap(),
        output
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang2-opaque-records.winmd");
    let roundtrip = path.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&path)
        .write()
        .unwrap();
    let index = windows_metadata::reader::Index::new(vec![
        windows_metadata::reader::File::read(&path).unwrap(),
    ]);
    for name in ["Alpha", "Beta"] {
        let ty = index.expect("Fixture", name);
        assert_eq!(ty.fields().count(), 0);
        assert!(ty.class_layout().is_none());
    }
    windows_rdl::writer()
        .input(&path)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), expected);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

#[test]
fn no_fabricated_storage() {
    helpers::ensure_libclang();
    let input = test_clang2::fixture("opaque_records_invalid");
    let output = projection::project_partial_with_imports(
        capture_partial(&[input], &TARGETS).unwrap(),
        "Fixture",
        &imports(),
    )
    .unwrap();
    assert_eq!(output.output.functions, 0);
    assert_eq!(output.output.records, 3);
    assert_eq!(output.output.constants, 1);
    assert!(output.output.partitions.values().any(|partition| {
        partition
            .rdl
            .contains("const Identified: GUID = 0x12345678_1234_1234_1234_123456789abc;")
    }));
    for name in [
        "Make",
        "Consume",
        "Callback",
        "Empty",
        "Unsupported",
        "Consumer",
        "Unknown",
        "Aligned",
        "Parent",
        "Inherit",
    ] {
        assert!(
            output
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            output.omissions
        );
    }
    for name in ["Make", "Consume", "Callback"] {
        assert!(output.omissions.iter().any(|message| {
            message.contains(name) && message.contains("incomplete") && message.contains("by value")
        }));
    }
    assert!(output.omissions.iter().any(|message| {
        message.contains("Inherit") && message.contains("by-value C++ inheritance")
    }));
}

#[test]
fn definitions_and_conflicts() {
    helpers::ensure_libclang();
    for source in [
        "struct Alpha { int value; };",
        "struct Alpha {};",
        "struct Alpha { private: int value; };",
    ] {
        let mut inputs = [
            input(),
            Input {
                name: "definition.h".into(),
                source: source.into(),
                ..Default::default()
            },
        ];
        inputs[0].source.push_str(
            "\nextern \"C\" Alpha Make();\nextern \"C\" int Consume(Alpha value);\n\
             typedef Alpha (*ValueCallback)();\n",
        );
        for _ in 0..2 {
            let output = projection::project_partial_with_imports(
                capture_partial(&inputs, &TARGETS).unwrap(),
                "Fixture",
                &imports(),
            )
            .unwrap();
            if source == "struct Alpha { int value; };" {
                assert!(output.omissions.is_empty(), "{:?}", output.omissions);
                assert_eq!(output.output.functions, 3);
                assert!(output.output.partitions.values().any(|part| {
                    part.rdl.contains("struct Alpha {\n") && part.rdl.contains("value: i32,")
                }));
            } else {
                assert_eq!(output.output.functions, 0);
                assert_eq!(output.output.constants, 0);
                assert!(
                    !output
                        .output
                        .partitions
                        .values()
                        .any(|part| part.rdl.contains("struct Alpha"))
                );
            }
            inputs.reverse();
        }
    }
    let inputs = [
        Input {
            name: "a.h".into(),
            source: "struct A; typedef A* Handle;".into(),
            ..Default::default()
        },
        Input {
            name: "b.h".into(),
            source: "struct B; typedef B* Handle;".into(),
            ..Default::default()
        },
    ];
    assert!(projection::generate_partial(&inputs, "Fixture").is_err());
}
