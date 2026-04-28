#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/merge_a.rdl")
        .output("tests/merge_a.winmd")
        .write()
        .unwrap();

    windows_rdl::reader()
        .input("tests/merge_b.rdl")
        .output("tests/merge_b.winmd")
        .write()
        .unwrap();

    windows_metadata::merge()
        .input("tests/merge_a.winmd")
        .input("tests/merge_b.winmd")
        .output("tests/merged.winmd")
        .merge()
        .unwrap();

    windows_rdl::writer()
        .input("tests/merged.winmd")
        .output("tests/merge_out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

#[test]
fn test_arch() {
    // Build per-arch winmds from plain RDL files (no arch attributes).
    windows_rdl::reader()
        .input("tests/merge_arch_x86.rdl")
        .output("tests/merge_arch_x86.winmd")
        .write()
        .unwrap();

    windows_rdl::reader()
        .input("tests/merge_arch_x64.rdl")
        .output("tests/merge_arch_x64.winmd")
        .write()
        .unwrap();

    // Merge with arch awareness: x86=1, x64=2.
    windows_metadata::merge()
        .arch_input("tests/merge_arch_x86.winmd", 1)
        .arch_input("tests/merge_arch_x64.winmd", 2)
        .output("tests/merge_arch_merged.winmd")
        .merge()
        .unwrap();

    // Write back to RDL for golden-file comparison.
    windows_rdl::writer()
        .input("tests/merge_arch_merged.winmd")
        .output("tests/merge_arch_out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
