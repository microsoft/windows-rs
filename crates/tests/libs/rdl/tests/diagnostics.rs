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
    let actual = error.to_string();
    let expected = root.join("errors").join("expected");
    std::fs::create_dir_all(&expected).unwrap();
    std::fs::write(
        expected.join(format!("{name}.txt")),
        format!("{}\n", actual.trim_start_matches('\n')),
    )
    .unwrap();
}
