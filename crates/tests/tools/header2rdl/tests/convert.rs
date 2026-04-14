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
/// `test.dll`, the Windows SDK include directories on the include path, and the
/// `Windows.Win32.winmd` reference metadata.  The reference WINMD is also
/// passed to the `windows_rdl` reader and writer during the roundtrip so that
/// types defined there (e.g. `Windows::Win32::System::Com::IUnknown`) are
/// resolved correctly.
#[cfg(target_arch = "x86_64")]
#[test]
fn convert() {
    let tests_dir = std::path::Path::new("tests");
    let include_dir = tests_dir.join("include");
    let reference = std::path::Path::new("../../../libs/bindgen/default/Windows.Win32.winmd");

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
            .system_include(include_dir.join("um"))
            .system_include(include_dir.join("shared"))
            .system_include(include_dir.join("ucrt"))
            .system_include(include_dir.join("msvc"))
            .system_include(include_dir.join("winrt"))
            .reference(reference)
            .convert()
            .unwrap_or_else(|e| panic!("convert failed for {stem}.h: {e}"));

        let rdl_path = tests_dir.join(format!("{stem}.rdl"));
        let winmd_path = tests_dir.join(format!("{stem}.winmd"));

        std::fs::write(&rdl_path, &raw_rdl)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", rdl_path.display()));

        windows_rdl::reader()
            .input(rdl_path.to_str().unwrap())
            .input(reference.to_str().unwrap())
            .output(winmd_path.to_str().unwrap())
            .write()
            .unwrap_or_else(|e| panic!("rdl reader failed for {stem}.rdl: {e}"));

        windows_rdl::writer()
            .input(winmd_path.to_str().unwrap())
            .input(reference.to_str().unwrap())
            .output(rdl_path.to_str().unwrap())
            .filter("Test")
            .write()
            .unwrap_or_else(|e| panic!("rdl writer failed for {stem}.winmd: {e}"));

        let _ = std::fs::remove_file(&winmd_path);
    }
}
