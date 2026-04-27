fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut paths: Vec<_> = std::fs::read_dir("roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    let mut lib_content = String::from(
        "#![allow(\n    non_snake_case,\n    non_upper_case_globals,\n    non_camel_case_types,\n    dead_code,\n    clippy::all\n)]\n",
    );
    let mut roundtrip_tests = String::new();

    for path in &paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let name = stem.replace('-', "_");
        let mod_name = escape_keyword_mod_name(&name);

        let winmd_path = format!("{out_dir}/{stem}.winmd");
        windows_rdl::reader()
            .input(path.to_str().unwrap())
            .output(&winmd_path)
            .write()
            .unwrap();

        let rs_path = format!("src/{stem}.rs");
        windows_bindgen::builder()
            .input(&winmd_path)
            .output(&rs_path)
            .filter("Test")
            .no_allow()
            .no_comment()
            .specific_deps()
            .write()
            .unwrap();

        if mod_name == name {
            lib_content.push_str(&format!("pub mod {mod_name};\n"));
        } else {
            lib_content.push_str(&format!("#[path = \"{stem}.rs\"]\npub mod {mod_name};\n"));
        }

        roundtrip_tests.push_str(&format!(
            "#[test]\nfn roundtrip_{name}() {{\n    run_roundtrip({stem:?});\n}}\n"
        ));
    }

    std::fs::write("src/lib.rs", lib_content).unwrap();

    let roundtrip_path = "tests/roundtrip.rs";
    let current = std::fs::read_to_string(roundtrip_path).unwrap();
    let marker = "// generated tests\n";
    let base = match current.find(marker) {
        Some(pos) => &current[..pos + marker.len()],
        None => current.as_str(),
    };
    std::fs::write(roundtrip_path, format!("{base}{roundtrip_tests}")).unwrap();

    // Re-run the build script whenever a .rdl file is added or removed.
    println!("cargo:rerun-if-changed=roundtrip");
}

fn escape_keyword_mod_name(name: &str) -> String {
    // Append _ to Rust keywords to avoid module name conflicts.
    match name {
        "abstract" | "as" | "async" | "await" | "become" | "box" | "break" | "const"
        | "continue" | "crate" | "do" | "dyn" | "else" | "enum" | "extern" | "false" | "final"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod"
        | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "self" | "static"
        | "struct" | "super" | "trait" | "true" | "try" | "type" | "typeof" | "union"
        | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" => {
            format!("{name}_")
        }
        _ => name.to_string(),
    }
}
