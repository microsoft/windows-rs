use windows_rdl::*;

/// Verifies that multiple arch-specific definitions of the same type name
/// survive a full RDL reader → winmd writer roundtrip.
///
/// `nested-arch.rdl` defines X86 and X64 variants of `SLIST_HEADER` and its
/// flat helper `SLIST_HEADER_0`.  Before the reader fix the `Index` used a
/// plain `BTreeMap<name, item>` so only one variant (the last) survived
/// encoding; both must now appear in the output.
///
/// A second scenario exercises the writer's un-nesting of real nested
/// TypeDefs from `Windows.Win32.winmd`: `SLIST_HEADER` has three arch
/// variants (Arm64, X64, X86) each with an anonymous nested struct.  The
/// writer must propagate `SupportedArchitecture` to every synthesised flat
/// helper so all three appear distinctly in the output.
#[test]
pub fn parse() {
    // Scenario 1: multi-arch RDL roundtrip (exercises the reader fix).
    reader()
        .input("tests/nested-arch.rdl")
        .reference("../bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arch.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/nested-arch.winmd")
        .input("../bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arch-out.rdl")
        .filter("Test")
        .write()
        .unwrap();

    let rdl_out = std::fs::read_to_string("tests/nested-arch-out.rdl").unwrap();
    // Both arch-specific variants of each type must survive the roundtrip.
    assert_eq!(
        rdl_out.matches("union SLIST_HEADER").count(),
        2,
        "expected 2 arch-specific SLIST_HEADER definitions (X64, X86)"
    );
    assert_eq!(
        rdl_out.matches("struct SLIST_HEADER_0").count(),
        2,
        "expected 2 arch-specific SLIST_HEADER_0 definitions (X64, X86)"
    );
    assert!(rdl_out.contains("SupportedArchitecture(X64)"));
    assert!(rdl_out.contains("SupportedArchitecture(X86)"));

    // Scenario 2: multi-arch nested-type propagation via real Windows metadata
    // (exercises the writer fix).  SLIST_HEADER has three arch variants each
    // with an anonymous nested struct; the writer synthesises a flat
    // SLIST_HEADER_0 per variant and must propagate SupportedArchitecture.
    let out = std::env::temp_dir().join("windows_rdl_nested_arch_kernel.rdl");
    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output(out.to_str().unwrap())
        .filter("Windows.Win32.System.Kernel")
        .write()
        .unwrap();

    let content = std::fs::read_to_string(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    assert_eq!(
        content.matches("struct SLIST_HEADER_0").count(),
        3,
        "expected 3 arch-specific SLIST_HEADER_0 definitions (Arm64, X64, X86)"
    );
    assert!(content.contains("SupportedArchitecture(Arm64)"));
    assert!(content.contains("SupportedArchitecture(X64)"));
    assert!(content.contains("SupportedArchitecture(X86)"));
}
