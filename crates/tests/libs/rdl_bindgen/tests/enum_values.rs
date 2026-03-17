use test_rdl_bindgen::enum_values::Test::*;

#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/enum_values.rdl")
        .output("tests/enum_values.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/enum_values.winmd")
        .output("tests/enum_values.rdl")
        .namespace("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/enum_values.winmd",
        "--out",
        "src/enum_values.rs",
        "--filter",
        "Test",
        "--sys",
        "--no-comment",
    ])
    .unwrap();

    assert_eq!(Status::Active.0, 1);
    assert_eq!(Status::Inactive.0, 2);
    assert_eq!(Status::Pending.0, 3);
}
