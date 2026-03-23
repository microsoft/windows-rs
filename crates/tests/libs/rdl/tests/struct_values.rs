use test_rdl::struct_values::Test::*;

#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/struct_values.rdl")
        .output("tests/struct_values.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/struct_values.winmd")
        .output("tests/struct_values.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/struct_values.winmd",
        "--out",
        "src/struct_values.rs",
        "--filter",
        "Test",
        "--no-comment",
    ])
    .unwrap();

    let c = Color {
        R: 255,
        G: 128,
        B: 0,
    };
    assert_eq!(format!("{c:?}"), "Color { R: 255, G: 128, B: 0 }");
    assert_eq!(
        c,
        Color {
            R: 255,
            G: 128,
            B: 0
        }
    );
    assert_eq!(size_of::<Color>(), 3);
    assert_eq!(align_of::<Color>(), 1);
}
