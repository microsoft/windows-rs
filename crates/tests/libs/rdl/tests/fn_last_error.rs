#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/fn_last_error.rdl")
        .output("tests/fn_last_error.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/fn_last_error.winmd")
        .output("tests/fn_last_error_writer.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/fn_last_error.winmd",
        "--out",
        "src/fn_last_error.rs",
        "--filter",
        "Test",
        "--sys",
    ])
    .unwrap();
}
