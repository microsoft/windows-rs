use windows_clang::*;
use windows_metadata::{Value, reader, reader::HasAttributes};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

fn attributes<'a>(value: impl HasAttributes<'a>) -> Vec<Vec<Value>> {
    value
        .attributes()
        .filter(|attribute| attribute.name() == "NativeSalAttribute")
        .map(|attribute| {
            attribute
                .value()
                .into_iter()
                .map(|(_, value)| value)
                .collect()
        })
        .collect()
}

fn annotation(name: &str, arguments: &str, expanded: &str) -> Vec<Value> {
    vec![
        Value::Utf8("2".into()),
        Value::Utf8(name.into()),
        Value::Utf8(arguments.into()),
        Value::Utf8(expanded.into()),
    ]
}

#[test]
fn alias_annotation_reconciliation() {
    helpers::ensure_libclang();
    let input = |contract| {
        let mut input = test_clang::fixture("annotation_merge");
        input.name = format!("contract{contract}.h");
        input
            .source
            .insert_str(0, &format!("#define CONTRACT {contract}\n"));
        input.roots = vec!["Status".into()];
        input
    };
    let mut inputs: Vec<_> = [0, 1, 2, 3].into_iter().map(input).collect();
    let mut expected = None;
    for _ in 0..inputs.len() {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let text: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert_eq!(text.matches("NativeSal(").count(), 1);
        assert!(text.contains("#[success(return >= 0)]"));
        if let Some(expected) = &expected {
            assert_eq!(&output, expected);
        } else {
            expected = Some(output);
        }
        inputs.rotate_left(1);
    }
    for contracts in [[0, 1, 2], [0, 1, 4]] {
        let mut inputs = contracts.map(input);
        for _ in 0..2 {
            let snapshots = capture(&inputs, &TARGETS).unwrap();
            let error = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap_err();
            assert!(
                error.to_string().contains("no observed SAL annotation set"),
                "{error}"
            );
            assert!(
                projection::project_partial(capture_partial(&inputs, &TARGETS).unwrap(), "Fixture")
                    .is_err()
            );
            inputs.reverse();
        }
    }
}

#[test]
fn expanded_annotation_owner_is_not_discarded() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("annotation_merge");
    input.arguments.push("-DCONTRACT=0".into());
    input.roots = vec!["UsesVoid".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    let text: String = output
        .partitions
        .values()
        .map(|part| part.rdl.as_str())
        .collect();
    assert!(text.contains("value: *mut AnnotatedVoid,"));
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang-annotation-void-owner.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&winmd)
        .write()
        .unwrap();
    let index = reader::Index::read(&winmd).unwrap();
    assert_eq!(
        attributes(index.expect("Fixture", "AnnotatedVoid")),
        [annotation(
            "_Return_type_success_",
            "return >= 0",
            "return >= 0"
        )]
    );

    input.roots = vec!["UsesConstVoid".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
    let result =
        projection::project_partial(capture_partial(&[input], &TARGETS).unwrap(), "Fixture")
            .unwrap();
    assert!(result.output.partitions.is_empty());
    assert!(
        result
            .omissions
            .iter()
            .any(|value| value.contains("AnnotatedConstVoid"))
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|value| value.contains("UsesConstVoid"))
    );
}

#[test]
fn invalid_annotation_transport() {
    helpers::ensure_libclang();
    for payload in [
        "windows_clang:sal:3:_In_\n()\n()",
        "windows_clang:sal:2:1Invalid\n()\n()",
        "windows_clang:sal:2:_In_\n()\n()\nextra",
        "windows_clang:sal:2:_In_\n()\nmissing-parentheses",
    ] {
        let mut input = test_clang::fixture("annotation_transport");
        input
            .source
            .insert_str(0, &format!("#define PAYLOAD {payload:?}\n"));
        let error = capture(&[input], &[Target::X64]).unwrap_err().to_string();
        assert!(
            error.contains("invalid SAL annotation transport"),
            "{error}"
        );
    }

    let mut input = test_clang::fixture("annotation_transport");
    input
        .source
        .insert_str(0, "#define PAYLOAD \"unrecognized annotation\"\n");
    let snapshots = capture(&[input], &[Target::X64]).unwrap();
    assert!(merge(&snapshots).is_err());
}

#[test]
fn nested_annotation_transport() {
    helpers::ensure_libclang();
    let nested = |arguments: &str| {
        let payload = format!("windows_clang:sal:2:_In_reads_\n({arguments})\n(16)");
        format!("__attribute__((annotate({payload:?})))")
    };
    let native = nested("LIMIT");
    let input = |expanded: &str| {
        let payload =
            format!("windows_clang:sal:2:_When_\n(n > 0,_In_reads_(LIMIT))\n({expanded})");
        let mut input = test_clang::fixture("annotation_transport");
        input
            .source
            .insert_str(0, &format!("#define PAYLOAD {payload:?}\n"));
        input
    };
    let ordinary_literal = format!("{native:?}");
    let raw_literal = format!("u8R\"scope({native})scope\"");
    let expanded = format!("1'000, {ordinary_literal}, {raw_literal}, {native}");
    let expected = format!("1'000, {ordinary_literal}, {raw_literal}, _In_reads_(16)");
    let snapshots = capture(&[input(&expanded)], &TARGETS).unwrap();
    let evidence = format!("{snapshots:?}");
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert_eq!(format!("{snapshots:?}"), evidence);
    let text: String = output.partitions.values().map(|p| p.rdl.as_str()).collect();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang-nested-transport.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&text)
        .output(&path)
        .write()
        .unwrap();
    let index = reader::Index::read(&path).unwrap();
    assert_eq!(
        attributes(index.expect("Fixture", "Annotated")),
        vec![annotation("_When_", "n > 0,_In_reads_(LIMIT)", &expected)]
    );
    std::fs::remove_file(path).unwrap();

    for malformed in [
        native.replace("sal:2:", "sal:3:"),
        native.trim_end_matches(')').into(),
        native.replace("\\n", "\\t"),
    ] {
        let input = input(&malformed);
        let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
        assert!(projection::project(&merge(&snapshots).unwrap(), "Fixture").is_err());
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.records, 0);
        assert!(partial.omissions.iter().any(|error| error.contains("SAL")));
    }
    let mut inputs = [input(&native), input(&nested("OTHER_LIMIT"))];
    inputs[1].name = "other.h".into();
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        assert!(merge(&snapshots).is_err());
        assert!(capture_partial(&inputs, &TARGETS).is_err());
        inputs.reverse();
    }
}

#[test]
fn callback_contract_boundaries() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("callback_contracts", &["Reader"]),
        test_clang::selected_fixture("callback_contracts", &["Reader"]),
    ];
    inputs[1].name = "other.h".into();
    inputs[1]
        .source
        .insert_str(0, "#define DIRECTION \"_Out_\"\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &TARGETS).unwrap();
        let merged = merge(&snapshots).unwrap();
        let error = projection::project(&merged, "Fixture")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("conflicting public names or contracts"),
            "{error}"
        );
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
    let input = test_clang::selected_fixture("callback_contracts", &["UsesNested"]);
    let output = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(output.output.partitions.is_empty());
    assert!(
        output
            .omissions
            .iter()
            .any(|value| value.contains("nested callback contract"))
    );
    assert!(
        output
            .omissions
            .iter()
            .any(|value| value.contains("UsesNested"))
    );
}
