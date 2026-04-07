/// For each `tests/*.h` file, run the `tool_header2rdl` converter and compare
/// the output against the matching `tests/*.rdl` golden file.
///
/// A `tests/<name>.h.args` sidecar file may supply extra options
/// (whitespace-separated): the only currently recognised token is `--cpp`.
/// All tests use `namespace("Test")` by default.
///
/// The `.rdl` golden files are stored in git.  If the tool output changes, the
/// test fails.  Set `UPDATE_EXPECT=1` to overwrite them with the current output
/// and commit the diff.
#[test]
fn convert() {
    let tests_dir = std::path::Path::new("tests");
    let update = std::env::var("UPDATE_EXPECT").is_ok();

    let mut headers: Vec<_> = std::fs::read_dir(tests_dir)
        .expect("tests directory not found – run tests from the crate root")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension().is_some_and(|ext| ext == "h") {
                Some(p)
            } else {
                None
            }
        })
        .collect();
    headers.sort();

    for h_path in &headers {
        let stem = h_path.file_stem().unwrap().to_string_lossy();

        let mut c = tool_header2rdl::converter();
        c.input(h_path.to_str().unwrap()).namespace("Test");

        // Load optional sidecar options from `<name>.h.args`.
        let sidecar = h_path.with_extension("h.args");
        if sidecar.exists() {
            let content = std::fs::read_to_string(&sidecar)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", sidecar.display()));
            for token in content.split_whitespace() {
                match token {
                    "--cpp" => {
                        c.cpp(true);
                    }
                    other => panic!(
                        "unrecognised sidecar token `{other}` in {}",
                        sidecar.display()
                    ),
                }
            }
        }

        let actual = c
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));

        if update || !rdl_path.exists() {
            std::fs::write(&rdl_path, &actual)
                .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));
        } else {
            let expected = std::fs::read_to_string(&rdl_path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", rdl_path.display()));
            assert_eq!(
                actual, expected,
                "\noutput changed for {stem}.h – rerun with UPDATE_EXPECT=1 to refresh\n"
            );
        }
    }
}
