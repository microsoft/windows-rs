use test_rdl_bindgen::struct_fields::Test::*;

#[test]
fn test() {
    windows_rdl::Reader::new()
        .input("tests/struct_fields.rdl")
        .output("tests/struct_fields.winmd")
        .write()
        .unwrap();

    windows_rdl::Writer::new()
        .input("tests/struct_fields.winmd")
        .output("tests/struct_fields.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/struct_fields.winmd",
        "--out",
        "src/struct_fields.rs",
        "--filter",
        "Test",
        "--sys",
        "--no-comment",
    ])
    .unwrap();

    assert_eq!(size_of::<Point>(), 8);
    assert_eq!(align_of::<Point>(), 4);
}
