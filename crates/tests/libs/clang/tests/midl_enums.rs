use windows_clang::*;

#[test]
fn helper_enums_publish_constants() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::fixture("midl_enums"),
        test_clang::fixture("midl_enums"),
    ];
    inputs[1].name = "other.h".into();
    inputs[1]
        .source
        .insert_str(0, "#define LOOSE_TAG __MIDL_Example_0009\n");
    let expected = std::fs::read_to_string(test_clang::input_dir().join("midl_enums.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    let mut targets = [Target::X86, Target::X64, Target::Arm64];
    for _ in 0..2 {
        let snapshots = capture(&inputs, &targets).unwrap();
        for snapshot in &snapshots {
            assert!(snapshot.enums.iter().all(|value| {
                value
                    .observations
                    .iter()
                    .all(|observation| !observation.anonymous)
            }));
        }
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert_eq!(text, expected);
        assert_eq!(output.constants, 2);
        inputs.reverse();
        targets.reverse();
    }
}

#[test]
fn helper_enum_contracts_are_not_erased() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("midl_enums");
    input
        .source
        .push_str("\nstruct PrivateUse { __MIDL_Example_0001 value; };\n");
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 2);
    assert_eq!(partial.output.records, 2);
    let rdl = &partial.output.partitions["midl_enums"].rdl;
    assert!(
        rdl.contains("struct PrivateUse {\n        value: i32,\n    }"),
        "{rdl}"
    );
    let input = Input {
        name: "annotated.h".into(),
        source: "enum [[deprecated(\"old enum\")]] __MIDL_Example_0001 { DeprecatedMember = 1 };"
            .into(),
        ..Default::default()
    };
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 0);
    assert!(
        partial
            .omissions
            .iter()
            .any(|message| message.contains("no public annotation owner"))
    );
    let input = Input {
        name: "ambiguous.h".into(),
        source: "enum __MIDL_Example_0001 { Member = 1 }; typedef __MIDL_Example_0001 First; typedef __MIDL_Example_0001 Second;".into(),
        ..Default::default()
    };
    let error = projection::generate(&[input], "Fixture").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("multiple authored typedef names"),
        "{error}"
    );
}
