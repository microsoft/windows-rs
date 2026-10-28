use windows_clang::*;

fn input() -> Input {
    let mut input = test_clang::selected_fixture("macro_declarations", &["NUMBER"]);
    input.macros = vec!["NUMBER".into()];
    input
}

#[test]
fn macro_value_does_not_claim_native_linkage() {
    helpers::ensure_libclang();
    let mut input = input();
    input.source = input.source.replace("extern \"C\" const Word NUMBER;", "");
    let output = projection::generate(&[input], "Fixture").unwrap();
    let expected = std::fs::read_to_string(test_clang::input_dir().join("macro_declarations.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["macro_declarations"].rdl, expected);
}

#[test]
fn macro_value_does_not_supply_a_native_definition() {
    helpers::ensure_libclang();
    let error = projection::generate(&[input()], "Fixture").unwrap_err();
    assert!(
        error.to_string().contains("has no captured definition"),
        "{error}"
    );
    let result = projection::generate_partial(&[input()], "Fixture").unwrap();
    assert_eq!(result.output.constants, 1);
    assert_eq!(
        result.output.partitions["macro_declarations"].rdl,
        std::fs::read_to_string(test_clang::input_dir().join("macro_declarations.rdl"))
            .unwrap()
            .replace("\r\n", "\n")
    );
    assert!(
        result
            .omissions
            .iter()
            .any(|message| message.contains("has no captured definition"))
    );
    assert!(
        !result
            .omissions
            .iter()
            .any(|message| message.contains("collides with a native declaration"))
    );
}

#[test]
fn macro_does_not_mask_native_conflicts() {
    helpers::ensure_libclang();
    for (declaration, reason) in [
        (
            "extern \"C++\" const Word NUMBER;",
            "native constant linkages differ",
        ),
        (
            "static const Word NUMBER = 42;",
            "native constant linkages differ",
        ),
        (
            "extern \"C\" const int NUMBER;",
            "native field types differ",
        ),
    ] {
        let original = input();
        let arguments = original.arguments.clone();
        let mut inputs = [
            original,
            Input {
                name: "other.h".into(),
                source: format!("typedef unsigned int Word;\n{declaration}\n"),
                roots: vec!["NUMBER".into()],
                arguments,
                ..Default::default()
            },
        ];
        for _ in 0..2 {
            let strict = projection::generate(&inputs, "Fixture");
            let partial = projection::generate_partial(&inputs, "Fixture");
            for error in [strict.unwrap_err(), partial.unwrap_err()] {
                assert!(error.to_string().contains(reason), "{declaration}: {error}");
            }
            for value in [42, 43] {
                let original = input();
                let mut inputs = [
                    Input {
                        name: "native_definition.h".into(),
                        source: format!(
                            "typedef unsigned int Word;\nextern \"C\" const Word NUMBER = {value};\n"
                        ),
                        roots: vec!["NUMBER".into()],
                        arguments: original.arguments.clone(),
                        ..Default::default()
                    },
                    original,
                ];
                for _ in 0..2 {
                    let error = projection::generate(&inputs, "Fixture").unwrap_err();
                    assert!(
                        error
                            .to_string()
                            .contains("collides with a native declaration"),
                        "{error}"
                    );
                    let result = projection::generate_partial(&inputs, "Fixture").unwrap();
                    assert_eq!(result.output.constants, 1);
                    assert!(
                        result.output.partitions.values().any(|part| part
                            .rdl
                            .contains(&format!("const NUMBER: Word = {value};")))
                    );
                    assert!(
                        result
                            .omissions
                            .iter()
                            .any(|message| message.contains("collides with a native declaration"))
                    );
                    inputs.reverse();
                }
            }
            inputs.reverse();
        }
    }
    let mut inputs = [input(), input()];
    inputs[1].name = "different_macro.h".into();
    inputs[1].source = inputs[1].source.replace("42", "43");
    for _ in 0..2 {
        assert!(projection::generate_partial(&inputs, "Fixture").is_err());
        inputs.reverse();
    }
}
