include!(concat!(env!("OUT_DIR"), "/tests.rs"));

fn run(name: &str) {
    let input = format!("input/{name}.h");
    let source = std::fs::read_to_string(&input).unwrap();
    let mut namespace = "Test".to_string();
    let mut library = Some("test.dll".to_string());
    let mut args = vec!["-x", "c++"];
    let mut reference_default = false;
    for line in source.lines().filter_map(|line| line.strip_prefix("//!")) {
        let directive = line.trim();
        if let Some(value) = directive.strip_prefix("namespace ") {
            namespace = value.trim().to_string();
        } else if let Some(value) = directive.strip_prefix("library ") {
            library = Some(value.trim().to_string());
        } else if let Some(value) = directive.strip_prefix("args ") {
            args = value.split_whitespace().collect();
        } else if directive == "no-library" {
            library = None;
        } else if directive == "reference-default" {
            reference_default = true;
        }
    }
    let output = std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.rdl"));
    let mut clang = windows_clang::clang();
    clang
        .input(&input)
        .args(args)
        .namespace(&namespace)
        .output(&output);
    if let Some(library) = library {
        clang.library(library);
    }
    if reference_default {
        clang.reference_default();
    }
    clang.write().unwrap();
    let actual = std::fs::read_to_string(output).unwrap();
    let mut reader = windows_rdl::reader();
    reader
        .input_text(&actual)
        .reference_default()
        .output(std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.winmd")));
    reader.write().unwrap();
    std::fs::write(format!("expected/{name}.rdl"), actual).unwrap();
}
