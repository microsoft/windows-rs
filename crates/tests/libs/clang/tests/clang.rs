include!(concat!(env!("OUT_DIR"), "/tests.rs"));

fn run(name: &str) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = root.join("input").join(format!("{name}.h"));
    let source = std::fs::read_to_string(&input).unwrap();
    let mut namespace = "Test".to_string();
    let mut library = "test.dll".to_string();
    let mut args = vec!["-x", "c++"];
    let mut reference_default = false;

    for line in source.lines() {
        let Some(directive) = line.strip_prefix("//!") else {
            break;
        };
        let directive = directive.trim();
        if let Some(value) = directive.strip_prefix("namespace ") {
            namespace = value.trim().to_string();
        } else if let Some(value) = directive.strip_prefix("library ") {
            library = value.trim().to_string();
        } else if let Some(value) = directive.strip_prefix("args ") {
            args = value.split_whitespace().collect();
        } else if directive == "reference-default" {
            reference_default = true;
        } else {
            panic!("unknown fixture directive `{directive}`");
        }
    }

    let output = std::path::Path::new(env!("OUT_DIR")).join(format!("{name}.rdl"));
    let mut clang = windows_clang::clang();
    clang
        .input(&input)
        .args(args)
        .namespace(&namespace)
        .library(library)
        .output(&output);
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
    let expected = root.join("expected");
    std::fs::create_dir_all(&expected).unwrap();
    std::fs::write(expected.join(format!("{name}.rdl")), actual).unwrap();
}
