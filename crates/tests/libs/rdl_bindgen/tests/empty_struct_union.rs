#[test]
fn test() {
    windows_rdl::Reader::new()
        .input("tests/empty_struct_union.rdl")
        .output("tests/empty_struct_union.winmd")
        .write()
        .unwrap();

    windows_rdl::Writer::new()
        .input("tests/empty_struct_union.winmd")
        .output("tests/empty_struct_union.rdl")
        .namespace("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/empty_struct_union.winmd",
        "--out",
        "src/empty_struct_union.rs",
        "--filter",
        "Test",
        "--sys",
        "--no-comment",
    ])
    .unwrap();
}
