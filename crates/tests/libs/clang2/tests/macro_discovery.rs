use test_clang2::selected_fixture;
use windows_clang2::*;

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[test]
fn discovers_without_selected_names() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("macro_discovery", &["Safe"]);
    input.macro_headers.push("macro_discovery.h".into());
    input.macros.push("NUMBER".into());
    input.arguments.extend([
        "-include".into(),
        test_clang2::input_dir()
            .join("macro_discovery_forced.h")
            .display()
            .to_string(),
    ]);
    let snapshots = capture(&[input], &TARGETS).unwrap();
    for snapshot in &snapshots {
        let typed = snapshot
            .values
            .iter()
            .find(|value| value.name == "TYPED")
            .unwrap();
        assert!(
            matches!(snapshot.types[typed.observations[0].written.0].kind, TypeKind::Alias(id) if snapshot.aliases[id.0].name == "Code"),
            "typed macro lost its typedef"
        );
        let names: std::collections::BTreeSet<_> = snapshot
            .values
            .iter()
            .map(|value| value.name.as_str())
            .collect();
        assert_eq!(
            names,
            [
                "NUMBER", "SHIFT", "TYPED", "TEXT", "WIDE", "INDIRECT", "PLATFORM"
            ]
            .into_iter()
            .collect()
        );
    }
}

#[test]
fn failed_probes_do_not_poison_capture() {
    helpers::ensure_libclang();
    let mut input = selected_fixture(
        "macro_discovery_rejections",
        &["Safe", "ERASED", "REPLACED"],
    );
    input.macro_headers = vec!["macro_discovery_rejections.h".into()];
    let result = capture_partial(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &result.snapshots {
        let names: std::collections::BTreeSet<_> = snapshot
            .values
            .iter()
            .map(|value| value.name.as_str())
            .collect();
        assert_eq!(
            names,
            ["ERASED", "REPLACED", "GOOD1", "GOOD2"]
                .into_iter()
                .collect()
        );
    }

    for name in ["BAD_TYPE", "BAD_TOKENS", "ARCHFAIL"] {
        assert!(
            result.omissions.iter().any(
                |message| message.contains(name) && message.contains("expression probe failed")
            ),
            "{name}: {:?}",
            result.omissions
        );
    }
    assert!(result.omissions.iter().any(|message| {
        message.contains("EMPTY")
            && message.contains("empty macro replacement is not an expression")
    }));
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("ERASED") && message.contains("not defined"))
    );
    let error = capture(std::slice::from_ref(&input), &TARGETS)
        .unwrap_err()
        .to_string();
    assert!(error.contains("expression probe failed"), "{error}");
    input.source.push_str("\n#error original header failure\n");
    let error = capture_partial(&[input], &TARGETS).unwrap_err().to_string();
    assert!(error.contains("original header failure"), "{error}");
}

#[test]
fn discovery_uses_the_active_definition_owner() {
    helpers::ensure_libclang();
    let mut input = selected_fixture("macro_discovery", &["Safe"]);
    input
        .source
        .push_str("\n#include \"macro_discovery_override.h\"\n");
    input.macro_headers = vec!["macro_discovery.h".into()];
    let snapshots = capture(std::slice::from_ref(&input), &TARGETS).unwrap();
    for snapshot in &snapshots {
        assert!(!snapshot.values.iter().any(|value| value.name == "NUMBER"));
        assert!(
            snapshot
                .diagnostics
                .iter()
                .any(|message| message.contains("NUMBER")
                    && message.contains("outside the selected macro headers"))
        );
    }
    input
        .macro_headers
        .push("macro_discovery_override.h".into());
    let output = projection::generate(&[input], "Fixture").unwrap();
    assert!(
        output.partitions["macro_discovery_override"]
            .rdl
            .contains("const NUMBER: i32 = 99;")
    );
    assert!(
        output.partitions["macro_discovery"]
            .rdl
            .contains("const INDIRECT: i32 = 100;")
    );
}

#[test]
fn constant_probes_do_not_exclude_native_declarations() {
    helpers::ensure_libclang();
    let mut input = test_clang2::fixture("macro_function_aliases");
    input.macro_headers = vec!["macro_function_aliases.h".into()];
    let imports = [
        "Base",
        "FunctionAlias",
        "AddressAlias",
        "DependencyAlias",
        "ProjectionAlias",
        "NativeBad",
    ]
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
    .collect();
    let generate = |targets: &[Target]| {
        projection::project_partial_with_imports(
            capture_partial(std::slice::from_ref(&input), targets).unwrap(),
            "Fixture",
            &imports,
        )
        .unwrap()
    };
    let output = generate(&TARGETS);
    let golden = test_clang2::input_dir().join("macro_function_aliases.rdl");
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(
            &golden,
            &output.output.partitions["macro_function_aliases"].rdl,
        )
        .unwrap();
    }
    let expected = std::fs::read_to_string(golden)
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(
        output.output.partitions["macro_function_aliases"].rdl,
        expected
    );
    assert_eq!(output.output.functions, 5);
    assert_eq!(output.output.constants, 1);
    for name in [
        "FunctionAlias",
        "AddressAlias",
        "DependencyAlias",
        "ProjectionAlias",
        "PARSE_BAD",
        "EVAL_BAD",
        "NativeBad",
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
    assert_eq!(generate(&[Target::Arm64, Target::X64, Target::X86]), output);
    assert!(capture(&[input], &TARGETS).is_err());
}

#[test]
fn native_conflicts_survive_failed_constant_probes() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang2::fixture("macro_function_aliases"),
        test_clang2::fixture("macro_function_aliases"),
    ];
    inputs[1].name = "other.h".into();
    inputs[1].source = inputs[1].source.replace(
        "int FunctionAlias(int value)",
        "float FunctionAlias(int value)",
    );
    for input in &mut inputs {
        input.macro_headers = vec![input.name.clone()];
    }
    for _ in 0..2 {
        let error = capture_partial(&inputs, &TARGETS).unwrap_err().to_string();
        assert!(error.contains("FunctionAlias"), "{error}");
        inputs.reverse();
    }
}
