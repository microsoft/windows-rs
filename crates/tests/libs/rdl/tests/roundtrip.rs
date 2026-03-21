use windows_rdl::*;

#[test]
fn roundtrip() {
    let mut paths: Vec<_> = std::fs::read_dir("tests/roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_dir() || (p.extension().and_then(|e| e.to_str()) == Some("rdl") && !p.with_extension("").is_dir()))
        .collect();
    paths.sort();

    for path in &paths {
        let winmd = path.with_extension("winmd");

        reader()
            .input(path.to_str().unwrap())
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap();

        writer()
            .input(winmd.to_str().unwrap())
            .output(path.with_extension("rdl").to_str().unwrap())
            .filter("Test")
            .write()
            .unwrap();
    }
}
