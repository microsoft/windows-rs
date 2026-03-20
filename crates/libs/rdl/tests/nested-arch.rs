use windows_rdl::*;

/// Verifies that when a struct carrying `SupportedArchitectureAttribute` has
/// nested types, the synthesised flat types (`OUTER_0`, `OUTER_1`, …) each
/// receive the same architecture attribute so that the round-trip is lossless.
///
/// `SLIST_HEADER` in `Windows.Win32.System.Kernel` is a concrete example: it
/// has three arch-specific definitions (arm64, x64, x86), each with nested
/// types.  Before the fix the synthesised `SLIST_HEADER_0` / `SLIST_HEADER_1`
/// types had no `SupportedArchitecture` attribute, losing the arch constraint.
#[test]
pub fn arch_attr_propagates_to_flat_nested_types() {
    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arch-out.rdl")
        .filter("Windows.Win32.System.Kernel")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string("tests/nested-arch-out.rdl").unwrap();

    // With arch attributes each definition is distinct, so dedup keeps all
    // three: arm64, x64, and x86 each produce a separate SLIST_HEADER_0.
    let definitions: Vec<_> = rdl.match_indices("struct SLIST_HEADER_0").collect();
    assert_eq!(
        definitions.len(),
        3,
        "SLIST_HEADER_0 should have 3 arch-specific definitions (arm64, x64, x86)"
    );

    // Every SLIST_HEADER_0 definition must be immediately preceded (within the
    // same type block) by a SupportedArchitecture attribute.
    for (pos, _) in &definitions {
        let preceding = &rdl[pos.saturating_sub(200)..*pos];
        assert!(
            preceding.contains("SupportedArchitecture"),
            "struct SLIST_HEADER_0 at offset {pos} must be preceded by a SupportedArchitecture attribute"
        );
    }

    // The round-trip through the reader must also succeed.
    reader()
        .input("tests/nested-arch-out.rdl")
        .reference("../bindgen/default/Windows.Win32.winmd")
        .output("tests/nested-arch-out.winmd")
        .write()
        .unwrap();
}
