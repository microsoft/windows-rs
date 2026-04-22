fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut paths: Vec<_> = std::fs::read_dir("tests/roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    let mut out = String::new();
    for path in &paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let name = stem.replace('-', "_");
        out.push_str(&format!(
            "#[test]\nfn roundtrip_{name}() {{\n    run_roundtrip({stem:?});\n}}\n"
        ));
    }

    std::fs::write(format!("{out_dir}/roundtrip_tests.rs"), out).unwrap();

    // Re-run the build script whenever a .rdl file is added or removed.
    println!("cargo:rerun-if-changed=tests/roundtrip");
}
