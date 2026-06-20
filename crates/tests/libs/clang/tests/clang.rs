#![cfg(target_pointer_width = "64")]

include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));

fn run(name: &str) {
    let input_path = format!("input/{name}.h");
    let expected_path = format!("expected/{name}.rdl");
    let scratch = format!("{}/{name}", env!("OUT_DIR"));
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract directives from `//!` comment lines at the top of the .h file.
    // Supported directives:
    //   //! namespace <Name>       — sets the RDL namespace (default: "Test")
    //   //! library <name.dll>     — sets the library name
    //   //! filter <suffix>        — adds a filter (may repeat)
    //   //! args <arg> ...         — extra clang args
    //   //! include <path>         — adds an include directory (-I)
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut namespace = "Test".to_string();
    let mut library = String::new();
    let mut filters: Vec<String> = vec![];
    let mut extra_args: Vec<String> = vec![];

    for line in contents.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            continue;
        };
        let rest = rest.trim();
        if let Some(ns) = rest.strip_prefix("namespace ") {
            namespace = ns.trim().to_string();
        } else if let Some(lib) = rest.strip_prefix("library ") {
            library = lib.trim().to_string();
        } else if let Some(f) = rest.strip_prefix("filter ") {
            filters.push(f.trim().to_string());
        } else if let Some(a) = rest.strip_prefix("args ") {
            extra_args.extend(a.split_whitespace().map(String::from));
        } else if let Some(inc) = rest.strip_prefix("include ") {
            extra_args.push("-I".to_string());
            extra_args.push(inc.trim().to_string());
        }
    }

    let rdl_out = format!("{scratch}/{name}.rdl");

    let mut clang = windows_rdl::clang();
    clang
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
        ])
        .args(extra_args.iter().map(|s| s.as_str()))
        .input(&input_path)
        .output(&rdl_out)
        .namespace(&namespace);

    if !library.is_empty() {
        clang.library(&library);
    }

    for f in &filters {
        clang.filter(f);
    }

    clang.write().unwrap();

    let actual = std::fs::read_to_string(&rdl_out).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}
