use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let path = std::path::Path::new("roundtrip").join(format!("{file}.rdl"));
    let winmd = path.with_extension("winmd");

    reader()
        .input(path.to_str().unwrap())
        .output(winmd.to_str().unwrap())
        .write()
        .unwrap();

    writer()
        .input(winmd.to_str().unwrap())
        .output(path.to_str().unwrap())
        .filter("Test")
        .write()
        .unwrap();
}

// generated tests
#[test]
fn roundtrip_const() {
    run_roundtrip("const");
}
#[test]
fn roundtrip_enum() {
    run_roundtrip("enum");
}
#[test]
fn roundtrip_fn() {
    run_roundtrip("fn");
}
#[test]
fn roundtrip_struct() {
    run_roundtrip("struct");
}
