use windows_rdl::*;

#[test]
fn roundtrip() {
    let temp_dir = std::env::temp_dir();
    let prefix = format!("windows_rdl_{}", std::process::id());

    let mut paths: Vec<_> = std::fs::read_dir("tests/roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    for path in &paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let winmd = temp_dir.join(format!("{prefix}_{stem}.winmd"));
        let out_rdl = temp_dir.join(format!("{prefix}_{stem}.rdl"));

        reader()
            .input(path.to_str().unwrap())
            .output(winmd.to_str().unwrap())
            .write()
            .unwrap();

        writer()
            .input(winmd.to_str().unwrap())
            .output(out_rdl.to_str().unwrap())
            .filter("Test")
            .write()
            .unwrap();

        let original = std::fs::read_to_string(path).unwrap();
        let roundtripped = std::fs::read_to_string(&out_rdl).unwrap();
        let _ = std::fs::remove_file(&winmd);
        let _ = std::fs::remove_file(&out_rdl);
        assert_eq!(
            original,
            roundtripped,
            "Roundtrip failed for {}",
            path.display()
        );
    }
}
