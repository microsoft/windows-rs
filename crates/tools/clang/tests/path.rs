#![cfg(windows)]

#[test]
fn invalid_external_path_is_not_replaced() {
    let directory =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests\\missing-native-pair");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_tool-clang"))
        .arg("path")
        .env("LIBCLANG_PATH", &directory)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("failed to load libclang"), "{error}");
    assert!(error.contains(directory.to_str().unwrap()), "{error}");
}
