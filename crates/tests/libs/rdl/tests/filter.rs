use windows_rdl::*;

const INPUT: &str = r#"
#[win32]
mod Test {
    struct Color {
        R: u8,
        G: u8,
        B: u8,
    }
    struct Point {
        X: i32,
        Y: i32,
    }
}
"#;

fn make_winmd(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!("windows_rdl_filter_{name}.winmd"));
    reader()
        .input_str(INPUT)
        .output(&path.to_string_lossy())
        .write()
        .unwrap();
    path
}

#[test]
fn filter_namespace() {
    let winmd = make_winmd("namespace");
    let rdl = std::env::temp_dir().join("windows_rdl_filter_namespace.rdl");

    writer()
        .input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy())
        .filter("Test")
        .write()
        .unwrap();

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(contents.contains("Color"), "Expected Color:\n{contents}");
    assert!(contents.contains("Point"), "Expected Point:\n{contents}");
}

#[test]
fn filter_unqualified_type() {
    let winmd = make_winmd("unqualified");
    let rdl = std::env::temp_dir().join("windows_rdl_filter_unqualified.rdl");

    writer()
        .input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy())
        .filter("Color")
        .write()
        .unwrap();

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(contents.contains("Color"), "Expected Color:\n{contents}");
    assert!(
        !contents.contains("Point"),
        "Expected no Point:\n{contents}"
    );
}

#[test]
fn filter_qualified_type() {
    let winmd = make_winmd("qualified");
    let rdl = std::env::temp_dir().join("windows_rdl_filter_qualified.rdl");

    writer()
        .input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy())
        .filter("Test.Point")
        .write()
        .unwrap();

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(contents.contains("Point"), "Expected Point:\n{contents}");
    assert!(
        !contents.contains("Color"),
        "Expected no Color:\n{contents}"
    );
}
