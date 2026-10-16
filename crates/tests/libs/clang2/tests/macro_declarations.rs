use windows_clang2::*;

fn input() -> Input {
    let mut input = test_clang2::selected_fixture("macro_declarations", &["NUMBER"]);
    input.macros = vec!["NUMBER".into()];
    input
}

#[test]
fn macro_value_does_not_claim_native_linkage() {
    helpers::ensure_libclang();
    let output = projection::generate(&[input()], "Fixture").unwrap();
    let expected = std::fs::read_to_string(test_clang2::input_dir().join("macro_declarations.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["macro_declarations"].rdl, expected);
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
            "extern \"C\" const Word NUMBER = 43;",
            "native value declarations differ",
        ),
        (
            "extern \"C\" const int NUMBER;",
            "native field types differ",
        ),
        ("extern \"C\" const Word NUMBER = 42;", ""),
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
            if reason.is_empty() {
                let strict = strict.unwrap();
                let partial = partial.unwrap();
                assert_eq!(strict.constants, 1);
                assert_eq!(partial.output, strict);
                assert!(partial.omissions.is_empty());
            } else {
                for error in [strict.unwrap_err(), partial.unwrap_err()] {
                    assert!(error.to_string().contains(reason), "{declaration}: {error}");
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
