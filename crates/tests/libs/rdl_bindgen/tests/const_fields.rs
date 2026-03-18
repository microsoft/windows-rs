use test_rdl_bindgen::const_fields::Test::*;

#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/const_fields.rdl")
        .output("tests/const_fields.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/const_fields.winmd")
        .output("tests/const_fields.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/const_fields.winmd",
        "--out",
        "src/const_fields.rs",
        "--filter",
        "Test",
        "--sys",
        "--no-comment",
    ])
    .unwrap();

    // Validates that a *const pointer field does not require the pointed-to value to be mutable.
    unsafe {
        let marker = Marker { X: 1, Y: 2 };
        let mut container = core::mem::zeroed::<Container>();
        container.Ptr = &marker;
        assert_eq!((*container.Ptr).X, 1);
        assert_eq!((*container.Ptr).Y, 2);
    }
}
