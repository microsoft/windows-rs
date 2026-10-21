include!(concat!(env!("OUT_DIR"), "/generated_error_tests.rs"));

fn run(name: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = format!("errors/input/{name}.rdl");
    let output = std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.winmd"));
    let error = windows_rdl::reader()
        .input(input)
        .output(output)
        .write()
        .unwrap_err();
    let actual = format!("{}\n", error.to_string().trim_start_matches('\n'));
    let expected = root.join("errors").join("expected");
    std::fs::create_dir_all(&expected).unwrap();
    let expected = expected.join(format!("{name}.txt"));
    let original = match std::fs::read_to_string(&expected) {
        Ok(text) => Some(text),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => panic!("failed to read {}: {error}", expected.display()),
    };
    std::fs::write(&expected, &actual).unwrap();
    assert_eq!(
        original.as_deref(),
        Some(actual.as_str()),
        "{name}: diagnostic mismatch - expected output has been updated.\n\
         Review the diff with `git diff` and commit if correct."
    );
}
