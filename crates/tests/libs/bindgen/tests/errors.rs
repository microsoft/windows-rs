// Negative tests for windows-bindgen. The golden harness only feeds valid
// input, so this exercises the panic path in `src/io.rs::read_file_lines`,
// reached when an `--etc` response file cannot be opened.

#[test]
#[should_panic(expected = "failed to open file")]
fn etc_missing_response_file_panics() {
    let missing = std::env::temp_dir()
        .join("test_bindgen_missing_response_file.rsp")
        .to_string_lossy()
        .into_owned();

    windows_bindgen::bindgen(["--etc", &missing]);
}
