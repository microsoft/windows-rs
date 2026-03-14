#[test]
fn test() {
    windows_rdl::Reader::new()
        .input("tests/fn_abi.rdl")
        .output("tests/fn_abi.winmd")
        .write()
        .unwrap();

    windows_rdl::Writer::new()
        .input("tests/fn_abi.winmd")
        .output("tests/fn_abi_writer.rdl")
        .namespace("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/fn_abi.winmd",
        "--out",
        "src/fn_abi.rs",
        "--filter",
        "Test",
        "--sys",
    ])
    .unwrap();
}
