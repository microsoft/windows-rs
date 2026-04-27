fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut paths: Vec<_> = std::fs::read_dir("roundtrip")
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rdl"))
        .collect();
    paths.sort();

    let mut lib_includes = String::new();
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

        let rs_path = format!("{out_dir}/{stem}.rs");
        windows_bindgen::builder()
            .input(&winmd_path)
            .output(&rs_path)
            .filter("Test")
            .no_allow()
            .no_comment()
            .specific_deps()
            .write()
            .unwrap();

        lib_includes.push_str(&format!(
            "pub mod {mod_name} {{ include!(concat!(env!(\"OUT_DIR\"), \"/{stem}.rs\")); }}\n"
        ));

        roundtrip_tests.push_str(&format!(
            "#[test]\nfn roundtrip_{name}() {{\n    run_roundtrip({stem:?});\n}}\n"
        ));
    }

    std::fs::write(format!("{out_dir}/lib_includes.rs"), lib_includes).unwrap();
    std::fs::write(format!("{out_dir}/roundtrip_tests.rs"), roundtrip_tests).unwrap();

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
