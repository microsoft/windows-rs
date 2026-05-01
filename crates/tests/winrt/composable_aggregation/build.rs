fn main() {
    println!("cargo:rerun-if-changed=src/metadata.rdl");

    windows_rdl::reader()
        .output("metadata.winmd")
        .input("src/metadata.rdl")
        .input("../../../libs/bindgen/default")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "metadata.winmd",
        "../../../libs/bindgen/default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "test_composable_aggregation",
        "--implement",
        "--no-comment",
        "--flat",
    ])
    .unwrap();

    // Post-process: bindgen emits `imp::FactoryCache` references for `Foo::new`,
    // `Foo::compose`, and the private `Foo::IFooFactory` helper. `FactoryCache`
    // is `#[cfg(windows)]`-only in `windows-core`. Gate just those three methods
    // so the rest of the bindings (the types, vtables, and `_Impl` traits we
    // actually drive in the harness) compile on Linux.
    let path = "src/bindings.rs";
    let src = std::fs::read_to_string(path).unwrap();
    let gated = gate_factory_cache_methods(&src);
    if gated != src {
        std::fs::write(path, gated).unwrap();
    }
}

/// Prepends `#[cfg(windows)]` to the three bindgen-emitted methods that depend
/// on `windows_core::imp::FactoryCache` (which is Windows-only). Operates on
/// the formatted output rustfmt produces, where each gated method body starts
/// with eight spaces of indent and ends at a matching closing brace at the
/// same indent.
fn gate_factory_cache_methods(src: &str) -> String {
    const STARTS: &[&str] = &[
        "    pub fn new() ->",
        "    pub fn compose<T>(",
        "    fn IFooFactory<R, F:",
    ];

    let mut out = String::with_capacity(src.len() + 128);
    let mut lines = src.lines().peekable();
    while let Some(line) = lines.next() {
        if STARTS.iter().any(|s| line.starts_with(s)) {
            out.push_str("    #[cfg(windows)]\n");
            out.push_str(line);
            out.push('\n');
            // Consume up to and including the closing `    }` line.
            for inner in lines.by_ref() {
                out.push_str(inner);
                out.push('\n');
                if inner == "    }" {
                    break;
                }
            }
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}
