fn main() {
    println!("cargo:rerun-if-changed=src");

    let mut paths: Vec<_> = std::fs::read_dir("src")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("h"))
        .collect();

    paths.sort();

    let mut tests = String::new();
    for path in &paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        tests.push_str(&format!(
            "#[test]\nfn roundtrip_{stem}() {{\n    run_roundtrip({stem:?});\n}}\n"
        ));
    }

    let roundtrip_rs = "tests/roundtrip.rs";
    let current = std::fs::read_to_string(roundtrip_rs).unwrap();
    let marker = "// generated tests\n";

    let base = match current.find(marker) {
        Some(pos) => current[..pos + marker.len()].to_string(),
        None => format!("{current}\n{marker}"),
    };

    std::fs::write(roundtrip_rs, format!("{base}\n{tests}")).unwrap();
}
