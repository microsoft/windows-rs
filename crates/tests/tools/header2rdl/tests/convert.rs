/// For each `tests/*.h` file, run `tool_header2rdl::convert` and compare the
/// output against the matching `tests/*.rdl` golden file.
///
/// A `tests/<name>.h.args` sidecar file may supply extra CLI tokens (one token
/// per line, or whitespace-separated) that are prepended to the argument list.
/// All tests use `--namespace Test` by default.
///
/// The `.rdl` golden files are stored in git.  If the tool output changes, the
/// test fails and the developer must update the golden files (by running the
/// test once with `UPDATE_EXPECT=1` or by re-running `cargo run -p
/// tool_header2rdl` manually).
///
/// Setting the environment variable `UPDATE_EXPECT=1` causes the test to
/// overwrite the golden `.rdl` files with the current tool output instead of
/// failing on a mismatch.  This makes it easy to refresh the golden files when
/// tool behaviour is intentionally changed.
#[test]
fn convert() {
    let tests_dir = std::path::Path::new("tests");
    let update = std::env::var("UPDATE_EXPECT").is_ok();

    let mut headers: Vec<_> = std::fs::read_dir(tests_dir)
        .expect("tests directory not found – run tests from the crate root")
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension().map_or(false, |ext| ext == "h") {
                Some(p)
            } else {
                None
            }
        })
        .collect();
    headers.sort();

    for h_path in &headers {
        let stem = h_path.file_stem().unwrap().to_string_lossy();

        // Base args: namespace + input file.
        let mut args = vec!["--namespace".to_string(), "Test".to_string()];

        // Load optional sidecar args from `<name>.h.args`.
        let sidecar = h_path.with_extension("h.args");
        if sidecar.exists() {
            let content = std::fs::read_to_string(&sidecar)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", sidecar.display()));
            for token in content.split_whitespace() {
                args.push(token.to_string());
            }
        }

        args.push(h_path.to_str().unwrap().to_string());

        let actual = tool_header2rdl::convert(args)
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));

        if update || !rdl_path.exists() {
            std::fs::write(&rdl_path, &actual)
                .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));
        } else {
            let expected = std::fs::read_to_string(&rdl_path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", rdl_path.display()));
            assert_eq!(
                actual,
                expected,
                "\noutput changed for {stem}.h – rerun with UPDATE_EXPECT=1 to refresh\n"
            );
        }
    }
}
