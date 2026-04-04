use windows_rdl::*;

#[test]
fn roundtrip() {
    let mut paths: Vec<_> = std::fs::read_dir("tests/roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    for path in &paths {
        let winmd = path.with_extension("winmd");
        let reference = needs_reference(path);

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
}

fn needs_reference(path: &std::path::Path) -> bool {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
        .contains("Windows::")
}
