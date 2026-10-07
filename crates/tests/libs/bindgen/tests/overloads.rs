#[test]
fn overload_syntax_does_not_change_bindings() {
    let source = include_str!("../input/interface_overloads.rdl");
    let raw = r#"
#[winrt]
mod Test {
    interface IReader {
        fn Read(&self) -> u32;
        #[Windows::Foundation::Metadata::Overload("ReadWithOptions")]
        fn Read(&self, options: u32) -> u32;
        #[Windows::Foundation::Metadata::Overload("Read3")]
        fn Read(&self, options: u32, count: u32) -> u32;
    }
}
"#;
    let dir = std::path::Path::new(env!("OUT_DIR"));
    let mut generated = Vec::new();
    for (name, source) in [("intrinsic", source), ("raw", raw)] {
        let winmd = dir.join(format!("overloads_{name}.winmd"));
        windows_rdl::reader()
            .input_text(source)
            .reference_default()
            .output(&winmd)
            .write()
            .unwrap();
        let output = dir.join(format!("overloads_{name}.rs"));
        windows_bindgen::builder()
            .input(&winmd)
            .filter("Test")
            .flat()
            .implement_all()
            .output(&output)
            .write();
        generated.push(std::fs::read_to_string(output).unwrap());
    }
    assert_eq!(generated[0], generated[1]);
}
