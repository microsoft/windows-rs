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
    //   //! map <symbol>=<dll>      — overrides the DLL for one symbol (may repeat)
    //   //! filter <suffix>        — adds a filter (may repeat)
    //   //! args <arg> ...         — extra clang args
    //   //! include <path>         — adds an include directory (-I)
    //   //! reference <file>       — compiles `input/<file>.rdl` to a winmd and
    //                               feeds it as a reference (cross-namespace
    //                               resolution; the target namespace is excluded)
    //   //! flat                    — use the faithful per-header (flat) scrape
    //                               (`write_by_header`, as `tool_win32`) instead of
    //                               the namespaced scrape (`write`, as `tool_webview`).
    //                               Enables the flat-mode collapses/normalizations
    //                               (`header_root.is_some()`); references/filters/library
    //                               do not apply.
    //   //! symbols <a>,<b>,...     — restricts emission to an allowlist of function
    //                               names (comma-separated); only those functions and
    //                               their transitive type/const closure are emitted,
    //                               every other root is suppressed.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut namespace = "Test".to_string();
    let mut library = String::new();
    let mut filters: Vec<String> = vec![];
    let mut extra_args: Vec<String> = vec![];
    let mut map: Vec<(String, String)> = vec![];
    let mut references: Vec<String> = vec![];
    let mut flat = false;
    let mut symbols: Vec<String> = vec![];

    for line in contents.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            continue;
        };
        let rest = rest.trim();
        if let Some(ns) = rest.strip_prefix("namespace ") {
            namespace = ns.trim().to_string();
        } else if let Some(lib) = rest.strip_prefix("library ") {
            library = lib.trim().to_string();
        } else if let Some(m) = rest.strip_prefix("map ") {
            if let Some((symbol, dll)) = m.trim().split_once('=') {
                map.push((symbol.trim().to_string(), dll.trim().to_string()));
            }
        } else if let Some(f) = rest.strip_prefix("filter ") {
            filters.push(f.trim().to_string());
        } else if let Some(a) = rest.strip_prefix("args ") {
            extra_args.extend(a.split_whitespace().map(String::from));
        } else if let Some(inc) = rest.strip_prefix("include ") {
            extra_args.push("-I".to_string());
            extra_args.push(inc.trim().to_string());
        } else if let Some(r) = rest.strip_prefix("reference ") {
            references.push(r.trim().to_string());
        } else if let Some(s) = rest.strip_prefix("symbols ") {
            symbols.extend(s.split(',').map(|t| t.trim().to_string()));
        } else if rest == "flat" {
            flat = true;
        }
    }

    // Compile any reference RDL fixtures to winmd so they can be fed to the
    // scraper as cross-namespace references.
    let mut reference_winmds: Vec<String> = vec![];
    for reference in &references {
        let ref_rdl = format!("input/{reference}.rdl");
        let ref_winmd = format!("{scratch}/{reference}.winmd");
        windows_rdl::reader()
            .input(&ref_rdl)
            .output(&ref_winmd)
            .write()
            .unwrap();
        reference_winmds.push(ref_winmd);
    }

    let rdl_out = format!("{scratch}/{name}.rdl");

    let mut clang = windows_clang::clang();
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

    for ref_winmd in &reference_winmds {
        clang.input(ref_winmd);
    }

    if !library.is_empty() {
        clang.library(&library);
    }

    if !map.is_empty() {
        clang.libraries(map.iter().map(|(s, d)| (s.as_str(), d.as_str())));
    }

    for f in &filters {
        clang.filter(f);
    }

    if !symbols.is_empty() {
        clang.symbols(symbols.iter().map(|s| s.as_str()));
    }

    if flat {
        // Faithful per-header (flat) scrape, as `tool_win32`: one flat root namespace,
        // `header_root.is_some()`. Emits every defining header in the parse (empty
        // partition allowlist) into `scratch`; a self-contained fixture yields a single
        // `<stem>.rdl` (the lowercased header stem, which matches `rdl_out`).
        clang.write_by_header(&namespace, &[], &scratch).unwrap();
    } else {
        // Namespaced scrape, as `tool_webview`: `header_root.is_none()`, resolves external
        // types via the reference winmds.
        clang.write().unwrap();
    }

    let actual = std::fs::read_to_string(&rdl_out).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}
