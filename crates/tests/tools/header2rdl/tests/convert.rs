/// For each `tests/*.h` file, run the `tool_header2rdl` converter and write
/// the output back to the matching `tests/*.rdl` file in the source tree.
///
/// Validation relies on `git diff`: if the tool output changes the committed
/// `.rdl` file the CI diff check will fail.  To regenerate the golden files,
/// simply run this test and commit the result.
///
/// A `tests/<name>.h.args` sidecar file may supply extra options
/// (whitespace-separated): supported tokens are `--cpp`, `--library NAME`,
/// `--include PATH` (resolved relative to the `tests/` directory),
/// `--system-include PATH` (resolved relative to `tests/`; adds the directory
/// as a system include so types defined there are NOT emitted in the RDL), and
/// `--windows-only` (skip the test on non-Windows platforms).
/// All tests use `namespace("Test")` by default.
#[test]
fn convert() {
    if !tool_header2rdl::is_available() {
        return;
    }

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
        c.file(h_path).namespace("Test");

        // Load optional sidecar options from `<name>.h.args`.
        let sidecar = h_path.with_extension("h.args");
        let mut windows_only = false;
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
                    "--include" => {
                        let path = tokens.next().unwrap_or_else(|| {
                            panic!("`--include` requires a path in {}", sidecar.display())
                        });
                        c.include(tests_dir.join(path));
                    }
                    "--system-include" => {
                        let path = tokens.next().unwrap_or_else(|| {
                            panic!(
                                "`--system-include` requires a path in {}",
                                sidecar.display()
                            )
                        });
                        c.system_include(tests_dir.join(path));
                    }
                    "--windows-only" => {
                        windows_only = true;
                    }
                    other => panic!(
                        "unrecognised sidecar token `{other}` in {}",
                        sidecar.display()
                    ),
                }
            }
        }

        if windows_only && !cfg!(target_os = "windows") {
            continue;
        }

        let rdl = c
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));
        std::fs::write(&rdl_path, &rdl)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));
    }
}
