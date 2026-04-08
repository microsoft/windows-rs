/// For each `tests/*.h` file, run the `tool_header2rdl` converter and write
/// the output back to the matching `tests/*.rdl` file in the source tree.
///
/// Validation relies on `git diff`: if the tool output changes the committed
/// `.rdl` file the CI diff check will fail.  To regenerate the golden files,
/// simply run this test and commit the result.
///
/// A `tests/<name>.h.args` sidecar file may supply extra options
/// (whitespace-separated): supported tokens are `--cpp` and `--library NAME`.
/// All tests use `namespace("Test")` by default.
#[test]
fn convert() {
    let tests_dir = std::path::Path::new("tests");

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
            let mut tokens = content.split_whitespace();
            while let Some(token) = tokens.next() {
                match token {
                    "--cpp" => {
                        c.cpp(true);
                    }
                    "--library" => {
                        let name = tokens.next().unwrap_or_else(|| {
                            panic!("`--library` requires a name in {}", sidecar.display())
                        });
                        c.library(name);
                    }
                    other => panic!(
                        "unrecognised sidecar token `{other}` in {}",
                        sidecar.display()
                    ),
                }
            }
        }

        let rdl = c
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));
        std::fs::write(&rdl_path, &rdl)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));
    }
}
