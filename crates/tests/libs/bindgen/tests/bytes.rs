#[test]
fn bindgen_accepts_metadata_bytes() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_bytes.winmd");
    let output = temp.join("windows_bindgen_bytes.rs");

    windows_rdl::reader()
        .input_str(
            r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();

    let bytes = std::fs::read(winmd).unwrap();
    windows_bindgen::builder()
        .input_byte_sets([bytes])
        .output(&output)
        .filter("Test")
        .flat()
        .write();

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}
