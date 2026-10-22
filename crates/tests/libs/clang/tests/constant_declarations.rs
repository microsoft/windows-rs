use windows_clang::*;

fn input(name: &str) -> Input {
    test_clang::selected_fixture(name, &["NUMBER", "ADDRESS", "GUID_VALUE"])
}

#[test]
fn declarations_and_definitions() {
    helpers::ensure_libclang();
    let mut inputs = [
        input("constant_declarations"),
        input("constant_definitions"),
    ];
    let expected =
        std::fs::read_to_string(test_clang::input_dir().join("constant_declarations.rdl"))
            .unwrap()
            .replace("\r\n", "\n");
    for snapshot in capture(&inputs[..1], &[Target::X86, Target::X64, Target::Arm64]).unwrap() {
        assert_eq!(snapshot.values.len(), 3);
        assert!(
            snapshot
                .values
                .iter()
                .all(|value| value.observations.iter().all(|observation| {
                    matches!(
                        &observation.kind,
                        ValueKind::Constant {
                            value: None,
                            linkage: ConstantLinkage::External {
                                language: LanguageLinkage::C,
                                ..
                            },
                            ..
                        }
                    ) && snapshot.types[observation.canonical.0].qualifiers.constant
                }))
        );
    }
    for _ in 0..2 {
        let output = projection::generate(&inputs, "Fixture").unwrap();
        assert_eq!(output.constants, 3);
        assert_eq!(output.partitions["foundation"].rdl, expected);
        let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
        assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
        assert_eq!(output, partial.output);
        inputs.reverse();
    }
}

#[test]
fn declarations_do_not_mask_conflicts() {
    helpers::ensure_libclang();
    for (old, new) in [
        ("NUMBER = 42", "NUMBER = 43"),
        ("unsigned int Word", "int Word"),
        ("extern \"C\"", "extern \"C++\""),
        ("extern const Word NUMBER", "const Word NUMBER"),
        (
            "extern const Word NUMBER",
            "extern const volatile Word NUMBER",
        ),
    ] {
        let mut different = input("constant_definitions");
        different.name = "z_different.h".into();
        different.source = different.source.replace(old, new);
        let mut inputs = [
            input("constant_declarations"),
            input("constant_definitions"),
            different,
        ];
        for _ in 0..3 {
            assert!(
                projection::generate(&inputs, "Fixture").is_err(),
                "{old} -> {new}"
            );
            assert!(
                projection::generate_partial(&inputs, "Fixture").is_err(),
                "{old} -> {new}"
            );
            inputs.rotate_left(1);
        }
    }
}

#[test]
fn declaration_dependency_conflicts_survive_pruning() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("constant_decl_record", &["NUMBER"]),
        test_clang::selected_fixture("constant_def_record", &["NUMBER"]),
    ];
    for _ in 0..2 {
        let error = projection::generate_partial(&inputs, "Fixture").unwrap_err();
        assert!(
            error.to_string().contains("native field types differ"),
            "{error}"
        );
        inputs.reverse();
    }
}

#[test]
fn definitions_are_target_specific() {
    helpers::ensure_libclang();
    let mut definition = input("constant_definitions");
    definition.source = definition.source.replace(
        "extern const Word NUMBER = 42;",
        "#ifdef _M_IX86\nextern const Word NUMBER = 42;\n#else\nextern const Word NUMBER;\n#endif",
    );
    let inputs = [input("constant_declarations"), definition];
    assert!(projection::generate(&inputs, "Fixture").is_err());
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!(partial.output.constants, 2);
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("NUMBER") && error.contains("no captured definition"))
    );
}

#[test]
fn same_input_keeps_both_observations() {
    helpers::ensure_libclang();
    let mut source = input("constant_declarations");
    source
        .source
        .push_str(&input("constant_definitions").source);
    let snapshots = capture(&[source], &[Target::X86, Target::X64, Target::Arm64]).unwrap();
    for snapshot in &snapshots {
        for value in &snapshot.values {
            assert_eq!(value.observations.len(), 2);
            assert!(matches!(
                value.observations[0].kind,
                ValueKind::Constant { value: None, .. }
            ));
            assert!(matches!(
                value.observations[1].kind,
                ValueKind::Constant { value: Some(_), .. }
            ));
        }
    }
    assert_eq!(
        projection::project(&merge(&snapshots).unwrap(), "Fixture")
            .unwrap()
            .constants,
        3
    );
}

#[test]
fn missing_and_invalid_definitions() {
    helpers::ensure_libclang();
    let declaration = input("constant_declarations");
    assert!(projection::generate(std::slice::from_ref(&declaration), "Fixture").is_err());
    let partial = projection::generate_partial(&[declaration], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 0);
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("NUMBER") && error.contains("no captured definition"))
    );

    let mut definition = input("constant_definitions");
    definition.source = definition
        .source
        .replace("NUMBER = 42", "NUMBER = Runtime()");
    definition.source.insert_str(0, "int Runtime();\n");
    let inputs = [input("constant_declarations"), definition];
    assert!(projection::generate(&inputs, "Fixture").is_err());
    let partial = projection::generate_partial(&inputs, "Fixture").unwrap();
    assert_eq!(partial.output.constants, 2);
    assert!(
        partial
            .omissions
            .iter()
            .any(|error| error.contains("NUMBER") && error.contains("constant expression"))
    );
}

#[test]
fn unsupported_storage_is_not_a_declaration_fallback() {
    helpers::ensure_libclang();
    for (replacement, message) in [
        ("extern Word NUMBER", "requires a const-qualified type"),
        (
            "extern thread_local const Word NUMBER",
            "thread-local constants are unsupported",
        ),
    ] {
        let mut declaration = input("constant_declarations");
        declaration.source = declaration
            .source
            .replace("extern const Word NUMBER", replacement);
        let error =
            capture(&[declaration], &[Target::X86, Target::X64, Target::Arm64]).unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
    }
}
