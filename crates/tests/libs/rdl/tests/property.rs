use windows_rdl::*;

// Regression test: `#[get] foo: T; #[set] foo: T;` (consecutive, same type)
// must be accepted by the reader and normalized to `foo: T;` on roundtrip.
#[test]
pub fn property_get_set_consecutive_normalizes() {
    let winmd = std::env::temp_dir().join("windows_rdl_property_get_set_consecutive.winmd");
    let rdl = std::env::temp_dir().join("windows_rdl_property_get_set_consecutive.rdl");

    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface ITest {
        #[get]
        foo: u32;
        #[set]
        foo: u32;
    }
}
        "#,
        )
        .output(&winmd.to_string_lossy())
        .write()
        .unwrap();

    writer()
        .input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy())
        .filter("Test")
        .write()
        .unwrap();

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(
        contents.contains("foo: u32;"),
        "Expected `foo: u32;` in output but got:\n{contents}"
    );
    assert!(
        !contents.contains("#[get]") && !contents.contains("#[set]"),
        "Expected no `#[get]`/`#[set]` in output but got:\n{contents}"
    );
}
