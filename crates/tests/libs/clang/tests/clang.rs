include!(concat!(env!("OUT_DIR"), "/tests.rs"));

fn run(name: &str) {
    let input = format!("input/{name}.h");
    let source = std::fs::read_to_string(&input).unwrap();
    let actual = windows_clang::extract([windows_clang::Input::new(input, source)], &["-x", "c++"])
        .unwrap()
        .emit_with_library("Test", "test.dll")
        .unwrap();
    windows_rdl::reader()
        .input_text(&actual)
        .output(std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.winmd")))
        .write()
        .unwrap();
    std::fs::write(format!("expected/{name}.rdl"), actual).unwrap();
}
