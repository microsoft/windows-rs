use windows_clang::*;

#[test]
fn empty_active_definitions_need_no_expression_probe() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("macro_empty");
    input.macros = ["REPLACED", "SPACED", "ZERO"].map(String::from).to_vec();
    let explicit = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    let expected = std::fs::read_to_string(test_clang::input_dir().join("macro_empty.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(explicit.partitions["macro_empty"].rdl, expected);
    input.macros.clear();
    input.macro_headers = vec!["MACRO_EMPTY.H".into(), "macro_empty.h".into()];
    let discovered = projection::generate_partial(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!(discovered.output, explicit);
    assert_eq!(discovered.omissions.len(), 5);
    for name in ["EMPTY", "COMMENT_ONLY", "LINE_ONLY", "BECAME_EMPTY"] {
        assert!(discovered.omissions.iter().any(|message| {
            message.starts_with(&format!("Macro(\"{name}\")"))
                && message.contains("empty macro replacement is not an expression")
        }));
    }
    assert!(discovered.omissions.iter().any(|message| {
        message.starts_with("Macro(\"INDIRECT_EMPTY\")")
            && message.contains("expression probe failed")
    }));
    assert!(capture(&[input], &[Target::X86, Target::X64, Target::Arm64]).is_err());
}

#[test]
fn recovery_errors_are_not_evidence_against_valid_macros() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("macro_recovery");
    input.macros = vec!["B_GOOD".into(), "D_GOOD".into()];
    let explicit = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    let expected = std::fs::read_to_string(test_clang::input_dir().join("macro_recovery.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(explicit.partitions["macro_recovery"].rdl, expected);
    input.macros.clear();
    input.macro_headers = vec!["macro_recovery.h".into()];
    let discovered = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(discovered.output, explicit);
    assert_eq!(discovered.omissions.len(), 2);
    for name in ["A_BAD", "C_BAD"] {
        assert!(
            discovered
                .omissions
                .iter()
                .any(|message| { message.starts_with(&format!("Macro(\"{name}\")")) })
        );
    }
}

#[test]
fn malformed_probes_preserve_valid_siblings() {
    helpers::ensure_libclang();
    let mut input = test_clang::selected_fixture("macro_diagnostics", &["Anchor"]);
    input.macro_headers = vec!["macro_diagnostics.h".into()];
    let generate = |targets: &[Target]| {
        projection::project_partial(
            capture_partial(std::slice::from_ref(&input), targets).unwrap(),
            "Fixture",
        )
        .unwrap()
    };
    let output = generate(&[Target::X86, Target::X64, Target::Arm64]);
    let expected = std::fs::read_to_string(test_clang::input_dir().join("macro_diagnostics.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(
        output.output.partitions["macro_diagnostics"].rdl,
        expected,
        "{:?}",
        output
            .omissions
            .iter()
            .filter(|message| message.contains("W_GOOD"))
            .collect::<Vec<_>>()
    );
    for name in [
        "B_CLOSE",
        "D_OPEN",
        "F_BRACE",
        "H_NAMESPACE",
        "J_LAMBDA",
        "L_SEMICOLON",
        "N_TEMPLATE",
        "R_TYPE",
        "X_OPEN",
        "Z_CLOSE",
        "T_FIELDS",
    ]
    .into_iter()
    .map(String::from)
    .chain((0..24).map(|index| format!("V_BAD{index:02}")))
    {
        assert!(
            output.omissions.iter().any(|message| {
                message.starts_with(&format!("Macro(\"{name}\")"))
                    && message.contains("expression probe failed")
            }),
            "{name}: {:?}",
            output.omissions
        );
    }
    assert!(output.omissions.iter().any(|message| {
        message.starts_with("Macro(\"P_EMPTY\")")
            && message.contains("empty macro replacement is not an expression")
    }));
    assert_eq!(generate(&[Target::Arm64, Target::X64, Target::X86]), output);
}

#[test]
fn multi_target_capture_preserves_order() {
    helpers::ensure_libclang();
    let mut inputs = [
        test_clang::selected_fixture("macro_diagnostics", &["Anchor"]),
        test_clang::selected_fixture("macro_recovery", &[]),
    ];
    for input in &mut inputs {
        input.macro_headers = vec![input.name.clone()];
    }
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let captured = capture_partial(&inputs, &targets).unwrap();
    for target in targets {
        let single = capture_partial(&inputs, &[target]).unwrap();
        let expected: Vec<_> = captured
            .snapshots
            .iter()
            .filter(|snapshot| snapshot.target == target)
            .collect();
        assert_eq!(single.snapshots.iter().collect::<Vec<_>>(), expected);
    }
    let expected = projection::project_partial(captured, "Fixture").unwrap();
    inputs.reverse();
    let reordered = capture_partial(&inputs, &[Target::Arm64, Target::X64, Target::X86]).unwrap();
    assert_eq!(
        projection::project_partial(reordered, "Fixture").unwrap(),
        expected
    );
}
