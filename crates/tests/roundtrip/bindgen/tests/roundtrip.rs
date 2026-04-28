use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let rdl = std::path::Path::new("src").join(format!("{file}.rdl"));
    let winmd = rdl.with_extension("winmd");

    writer()
        .input(winmd.to_str().unwrap())
        .output(rdl.to_str().unwrap())
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
fn roundtrip_enum_flags() {
    run_roundtrip("enum_flags");
}
#[test]
fn roundtrip_enum_name_conflict() {
    run_roundtrip("enum_name_conflict");
}
#[test]
fn roundtrip_fn() {
    run_roundtrip("fn");
}
#[test]
fn roundtrip_struct() {
    run_roundtrip("struct");
}
