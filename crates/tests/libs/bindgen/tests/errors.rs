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

fn author_variadic(name: &str) -> (String, String) {
    let scratch = std::path::Path::new(env!("OUT_DIR")).join(name);
    std::fs::create_dir_all(&scratch).unwrap();
    let winmd = scratch.join("out.winmd");
    windows_rdl::reader()
        .input("input/variadic_fn_sys.rdl")
        .output(&winmd)
        .write()
        .unwrap();
    (
        winmd.to_string_lossy().into_owned(),
        scratch.join("out.rs").to_string_lossy().into_owned(),
    )
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFunc` cannot be projected \
                by rich or minimal bindings; use `--sys` for its raw declaration"
)]
fn exact_rich_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_rich_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--flat",
        "--filter",
        "Test.VariadicFunc",
    ]);
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFunc` cannot be projected \
                by rich or minimal bindings; use `--sys` for its raw declaration"
)]
fn exact_minimal_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_minimal_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--minimal",
        "--flat",
        "--filter",
        "Test.VariadicFunc",
    ]);
}

#[test]
#[should_panic(
    expected = "windows-bindgen: selected variadic function `Test.VariadicFastcall` uses a calling \
                convention that stable Rust cannot represent for C variadics"
)]
fn exact_fastcall_variadic_selection_panics() {
    let (winmd, output) = author_variadic("exact_fastcall_variadic");
    windows_bindgen::bindgen([
        "--in",
        &winmd,
        "--out",
        &output,
        "--sys",
        "--flat",
        "--filter",
        "Test.VariadicFastcall",
    ]);
}

#[test]
#[should_panic(expected = "windows-bindgen: selected variadic function \
                `Windows.Win32.AuthzReportSecurityEvent` cannot be projected by rich or minimal \
                bindings; use `--sys` for its raw declaration")]
fn exact_real_variadic_selection_panics() {
    let output = std::path::Path::new(env!("OUT_DIR")).join("real_variadic.rs");
    windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        output.to_str().unwrap(),
        "--flat",
        "--filter",
        "Windows.Win32.AuthzReportSecurityEvent",
    ]);
}
