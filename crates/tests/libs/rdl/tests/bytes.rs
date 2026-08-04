fn temp_path(name: &str, extension: &str) -> String {
    std::env::temp_dir()
        .join(format!("windows_rdl_{name}.{extension}"))
        .to_string_lossy()
        .into_owned()
}

#[test]
fn default_input_resolves_default_metadata() {
    windows_rdl::reader()
        .input_str(
            r#"
use Windows::Foundation::*;

#[winrt]
mod Test {
    struct Wrapper {
        value: Point,
    }
}
"#,
        )
        .input("default")
        .output(&temp_path("default_input", "winmd"))
        .write()
        .unwrap();
}

#[test]
fn reference_bytes_resolve_metadata() {
    let reference = temp_path("reference_bytes_reference", "winmd");

    windows_rdl::reader()
        .input_str(
            r#"
#[winrt]
mod Other {
    struct Point {
        x: i32,
        y: i32,
    }
}
"#,
        )
        .output(&reference)
        .write()
        .unwrap();

    let bytes = std::fs::read(reference).unwrap();
    windows_rdl::reader()
        .input_str(
            r#"
use Other::*;

#[winrt]
mod Test {
    struct Wrapper {
        value: Point,
    }
}
"#,
        )
        .reference_bytes(&bytes)
        .output(&temp_path("reference_bytes", "winmd"))
        .write()
        .unwrap();
}

#[test]
fn writer_accepts_metadata_bytes() {
    let winmd = temp_path("writer_bytes_input", "winmd");
    let rdl = temp_path("writer_bytes_output", "rdl");

    windows_rdl::reader()
        .input_str(
            r#"
#[win32]
mod Test {
    struct Value {
        value: u32,
    }
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();

    let bytes = std::fs::read(&winmd).unwrap();
    windows_rdl::writer()
        .input_bytes(&bytes)
        .output(&rdl)
        .write()
        .unwrap();

    assert!(
        std::fs::read_to_string(rdl)
            .unwrap()
            .contains("struct Value")
    );
}
