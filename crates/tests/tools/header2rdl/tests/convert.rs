/// For each `tests/*.h` file, run the `tool_header2rdl` converter and compare
/// the output against the matching `tests/*.rdl` golden file.
///
/// After converting, the RDL is also roundtripped through `windows_rdl::reader`
/// (`.rdl` → `.winmd`) and `windows_rdl::writer` (`.winmd` → `.rdl`) to verify
/// validity.  The golden files store the writer's canonical output.
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

        let raw = c
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        // Roundtrip through the windows-rdl reader and writer to validate the
        // generated RDL and produce the canonical (writer-normalised) output.
        let tmp_winmd = std::env::temp_dir().join(format!("header2rdl_{stem}.winmd"));
        let tmp_rdl = std::env::temp_dir().join(format!("header2rdl_{stem}_out.rdl"));

        windows_rdl::reader()
            .input_str(&raw)
            .output(tmp_winmd.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| panic!("rdl reader failed for {stem}: {e}"));

        windows_rdl::writer()
            .input(tmp_winmd.to_str().unwrap())
            .filter("Test")
            .output(tmp_rdl.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| panic!("rdl writer failed for {stem}: {e}"));

        let actual = std::fs::read_to_string(&tmp_rdl)
            .unwrap_or_else(|e| panic!("failed to read roundtrip output for {stem}: {e}"));

        // Best-effort cleanup; leftover temp files are harmless and will be
        // overwritten on the next run.
        let _ = std::fs::remove_file(&tmp_winmd);
        let _ = std::fs::remove_file(&tmp_rdl);

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
