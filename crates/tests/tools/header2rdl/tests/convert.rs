/// For each `tests/*.h` file, run the `tool_header2rdl` converter, roundtrip
/// the resulting RDL through `windows_rdl::reader()` and `windows_rdl::writer()`,
/// and write the final output back to the matching `tests/*.rdl` file in the
/// source tree.
///
/// Validation relies on `git diff`: if the tool output changes the committed
/// `.rdl` file the CI diff check will fail.  To regenerate the golden files,
/// simply run this test and commit the result.
///
/// A `tests/<name>.h.args` sidecar file may supply extra options
/// (whitespace-separated): supported tokens are `--cpp`, `--library NAME`,
/// `--include PATH` (resolved relative to the `tests/` directory),
/// `--system-include PATH` (resolved relative to `tests/`; adds the directory
/// as a system include so types defined there are NOT emitted in the RDL),
/// `--no-roundtrip` (skip the windows-rdl reader/writer roundtrip for headers
/// that reference external system types not present in the RDL output), and
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
        let mut no_roundtrip = false;
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
                    "--no-roundtrip" => {
                        no_roundtrip = true;
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

        let raw_rdl = c
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));

        if no_roundtrip {
            // Headers that reference external system types (not emitted in
            // the RDL) can't roundtrip standalone.  Write the raw output and
            // skip the reader/writer validation step.
            std::fs::write(&rdl_path, &raw_rdl)
                .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));
        } else {
            // Roundtrip through windows-rdl reader→winmd→writer to validate
            // the generated RDL is well-formed (same as test_rdl roundtrip).
            let winmd_path = tests_dir.join(format!("{stem}.winmd"));

            // Write the raw RDL so the reader can load it from disk.
            std::fs::write(&rdl_path, &raw_rdl)
                .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));

            // reader: .rdl → .winmd
            windows_rdl::reader()
                .input(rdl_path.to_str().unwrap())
                .output(winmd_path.to_str().unwrap())
                .write()
                .unwrap_or_else(|e| panic!("rdl reader failed for {stem}.rdl: {e}"));

            // writer: .winmd → .rdl  (overwrites raw RDL with roundtripped form)
            windows_rdl::writer()
                .input(winmd_path.to_str().unwrap())
                .output(rdl_path.to_str().unwrap())
                .filter("Test")
                .write()
                .unwrap_or_else(|e| panic!("rdl writer failed for {stem}.winmd: {e}"));

            // Remove the temporary winmd file.
            let _ = std::fs::remove_file(&winmd_path);
        }
    }
}
