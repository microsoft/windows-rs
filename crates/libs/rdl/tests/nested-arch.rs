use windows_rdl::*;

/// Verifies that `SupportedArchitecture` attributes on arch-specific
/// struct/union pairs survive a full RDL reader → writer roundtrip.
///
/// Two separate scenarios are tested:
///
/// 1. **Reader roundtrip** (`nested-arch.rdl` → `nested-arch.winmd` →
///    `nested-arch-out.rdl`): a simplified, manually-defined `SLIST_HEADER`
///    pair with `SupportedArchitecture(X86)` roundtrips correctly.
///
/// 2. **Writer with real nested multi-arch types** (Windows.Win32.winmd):
///    `SLIST_HEADER` in the Windows metadata has anonymous nested TypeDefs
///    (one per architecture).  The writer un-nests them into flat helper
///    types (`SLIST_HEADER_0`, etc.) and must propagate the outer type's
///    `SupportedArchitecture` attribute to each synthesised type.  Before
///    the fix those synthesised types had no arch attribute, so identical
///    definitions across arm64, x64 and x86 were incorrectly deduplicated
///    into a single entry.
#[test]
pub fn parse() {
    // Scenario 1: single-arch RDL roundtrip.
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

    // Scenario 2: multi-arch nested-type propagation via real Windows metadata.
    // Windows.Win32.System.Kernel.SLIST_HEADER has three arch-specific variants
    // (Arm64, X64, X86), each containing an anonymous nested struct.  The
    // writer synthesises a flat helper type named SLIST_HEADER_0 for every
    // variant; each must carry the corresponding SupportedArchitecture
    // attribute so they are not merged.
    let out = std::env::temp_dir().join("windows_rdl_nested_arch_kernel.rdl");
    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output(out.to_str().unwrap())
        .filter("Windows.Win32.System.Kernel")
        .write()
        .unwrap();

    let content = std::fs::read_to_string(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    // All three arch-specific SLIST_HEADER_0 definitions must be present.
    // Without the fix the synthesised flat types had no SupportedArchitecture
    // attribute and were deduplicated down to one definition.
    assert_eq!(
        content.matches("struct SLIST_HEADER_0").count(),
        3,
        "expected 3 arch-specific SLIST_HEADER_0 definitions (Arm64, X64, X86)"
    );
    // Each definition must carry the correct arch attribute.
    assert!(content.contains("SupportedArchitecture(Arm64)"));
    assert!(content.contains("SupportedArchitecture(X64)"));
    assert!(content.contains("SupportedArchitecture(X86)"));
}
