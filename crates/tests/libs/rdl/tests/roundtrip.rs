use windows_rdl::*;

fn run_roundtrip(file: &str) {
    let path = std::path::Path::new("tests/roundtrip").join(format!("{file}.rdl"));
    let winmd = path.with_extension("winmd");
    let reference = needs_reference(&path);

    let mut r = reader();
    r.input(path.to_str().unwrap());
    if reference {
        r.input("../../../libs/bindgen/default");
    }
    r.output(winmd.to_str().unwrap()).write().unwrap();

    let mut w = writer();
    w.input(winmd.to_str().unwrap());
    if reference {
        w.input("../../../libs/bindgen/default");
    }
    w.output(path.to_str().unwrap())
        .filter("Test")
        .write()
        .unwrap();
}

fn needs_reference(path: &std::path::Path) -> bool {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
        .contains("Windows::")
}

include!(concat!(env!("OUT_DIR"), "/roundtrip_tests.rs"));
