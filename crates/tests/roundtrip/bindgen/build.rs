fn main() {
    println!("cargo:rerun-if-changed=src");

    let mut paths: Vec<_> = std::fs::read_dir("src")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();

    paths.sort();

    let mut lib_rs = String::from(
        "#![allow(\n    non_snake_case,\n    non_upper_case_globals,\n    non_camel_case_types,\n    dead_code,\n    clippy::all\n)]\n",
    );

    let mut tests = String::new();

    for path in &paths {
        let name = path.file_stem().unwrap().to_str().unwrap();
        let winmd = format!("src/{name}.winmd");
        let rs = format!("src/{name}.rs");

        windows_rdl::reader()
            .input(path.to_str().unwrap())
            .output(&winmd)
            .write()
            .unwrap();

        lib_rs.push_str(&format!("pub mod r#{name};\n"));

        tests.push_str(&format!(
            "#[test]\nfn roundtrip_{name}() {{\n    run_roundtrip({name:?});\n}}\n"
        ));

        // Use a specific derived class filter for the class test to confirm that
        // base classes are automatically pulled in as dependencies (issue #4320).
        let filter = if name == "class" {
            "Test.Derived"
        } else {
            "Test"
        };

        windows_bindgen::builder()
            .input(&winmd)
            .output(&rs)
            .filter(filter)
            .no_allow()
            .no_comment()
            .specific_deps()
            .write()
            .unwrap();
    }

    std::fs::write("src/lib.rs", lib_rs).unwrap();
    let roundtrip_rs = "tests/roundtrip.rs";
    let current = std::fs::read_to_string(roundtrip_rs).unwrap();
    let marker = "// generated tests\n";

    let base = match current.find(marker) {
        Some(pos) => current[..pos + marker.len()].to_string(),
        None => format!("{current}\n{marker}"),
    };

    std::fs::write(roundtrip_rs, format!("{base}\n{tests}")).unwrap();
}
