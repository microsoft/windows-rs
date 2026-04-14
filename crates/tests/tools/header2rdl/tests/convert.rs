/// For each `tests/*.h` file, run the `tool_header2rdl` converter, roundtrip
/// the resulting RDL through `windows_rdl::reader()` and `windows_rdl::writer()`,
/// and write the final output back to the matching `tests/*.rdl` file in the
/// source tree.
///
/// Validation relies on `git diff`: if the tool output changes the committed
/// `.rdl` file the CI diff check will fail.  To regenerate the golden files,
/// simply run this test and commit the result.
///
/// All headers are converted with the same options: C++ mode, library name
/// `test.dll`, and the `tests/include` directory on the include path.
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

        let raw_rdl = tool_header2rdl::converter()
            .file(h_path)
            .namespace("Test")
            .cpp(true)
            .library("test.dll")
            .include(tests_dir.join("include"))
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));
        let winmd_path = tests_dir.join(format!("{stem}.winmd"));

        std::fs::write(&rdl_path, &raw_rdl)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));

        windows_rdl::reader()
            .input(rdl_path.to_str().unwrap())
            .output(winmd_path.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| panic!("rdl reader failed for {stem}.rdl: {e}"));

        windows_rdl::writer()
            .input(winmd_path.to_str().unwrap())
            .output(rdl_path.to_str().unwrap())
            .filter("Test")
            .write()
            .unwrap_or_else(|e| panic!("rdl writer failed for {stem}.winmd: {e}"));

        let _ = std::fs::remove_file(&winmd_path);
    }
}
