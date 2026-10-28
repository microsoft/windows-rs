use windows_clang::*;

#[test]
fn initializer_fragments_are_not_scalar_constants() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("macro_initializers");
    input.macros = vec!["ARITHMETIC".into(), "GROUPED_COMMA".into()];
    let expected = std::fs::read_to_string(test_clang::input_dir().join("macro_initializers.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    let explicit = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!(explicit.partitions["macro_initializers"].rdl, expected);

    input.macros.clear();
    input.macro_headers = vec!["macro_initializers.h".into()];
    let discovered = projection::generate_partial(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!(discovered.output, explicit);
    for name in ["PAIR_FIELDS", "GUID_FIELDS"] {
        assert!(
            discovered.omissions.iter().any(|message| {
                message.contains(name) && message.contains("expression probe failed")
            }),
            "{name}: {:?}",
            discovered.omissions
        );
        input.macro_headers.clear();
        input.macros = vec![name.into()];
        let error = projection::generate(std::slice::from_ref(&input), "Fixture")
            .unwrap_err()
            .to_string();
        assert!(error.contains("libclang parse failed"), "{error}");
    }
}
