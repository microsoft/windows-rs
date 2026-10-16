use windows_clang2::*;

fn input(name: &str) -> Input {
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = format!("{name}.h");
    input.source = test_clang2::fixture(name).source;
    input.roots.clear();
    input
}

#[test]
#[ignore = "requires the pinned SDK"]
fn buffer_capacity_rdl() {
    helpers::ensure_libclang();
    let mut inputs = [input("buffer_sizes"), input("buffer_sizes")];
    inputs[1].name = "second.h".into();
    let mut targets = [Target::X86, Target::X64, Target::Arm64];
    let expected = std::fs::read_to_string(test_clang2::input_dir().join("buffer_sizes.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    for _ in 0..2 {
        let snapshots = capture(&inputs, &targets).unwrap();
        let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
        let actual: String = output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert_eq!(actual, expected);
        inputs.reverse();
        targets.reverse();
    }
}

#[test]
#[ignore = "requires the pinned SDK"]
fn invalid_buffer_capacity() {
    helpers::ensure_libclang();
    for (case, message) in [
        (1, "conflicting SAL buffer capacity"),
        (2, "requires a by-value integer"),
        (3, "requires a buffer pointer"),
        (4, "conflicting SAL buffer capacity"),
    ] {
        let mut input = input("buffer_sizes_invalid");
        input
            .source
            .insert_str(0, &format!("#define CASE {case}\n"));
        let error = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap_err();
        assert!(error.to_string().contains(message), "{error}");
        let partial = projection::generate_partial(&[input], "Fixture").unwrap();
        assert_eq!(partial.output.aliases, 1);
        assert!(
            partial
                .omissions
                .iter()
                .any(|value| value.contains(message)),
            "{:?}",
            partial.omissions
        );
        let actual: String = partial
            .output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect();
        assert!(actual.contains("#[size_param(1)] buffer:"), "{actual}");
        assert!(!actual.contains("fn Invalid("), "{actual}");
    }
}
