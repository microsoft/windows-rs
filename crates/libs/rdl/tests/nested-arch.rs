use windows_rdl::*;

/// Verifies that when the RDL writer un-nests the arch-specific variants of
/// `SLIST_HEADER` from `Windows.Win32.winmd`, the synthesised flat helper
/// types (`SLIST_HEADER_0`, `SLIST_HEADER_1`) each inherit the enclosing
/// union's `SupportedArchitecture` attribute.
///
/// `SLIST_HEADER` has three arch variants (arm64, x64, x86) each with nested
/// types.  Before the fix the synthesised flat types had no arch attribute, so
/// the arm64 and x64 `SLIST_HEADER_0` (identical fields) were incorrectly
/// deduplicated into a single definition.  After the fix all three variants
/// are kept as distinct definitions.
#[test]
pub fn arch_attr_propagates_to_flat_nested_types() {
    // Write only the Kernel namespace so the output is focused.
    // Use a process-scoped temp path to avoid committing the generated file.
    let out = std::env::temp_dir()
        .join(format!("nested-arch-{}.rdl", std::process::id()))
        .to_string_lossy()
        .to_string();

    writer()
        .input("../bindgen/default/Windows.Win32.winmd")
        .output(&out)
        .filter("Windows.Win32.System.Kernel")
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(&out).unwrap();
    let _ = std::fs::remove_file(&out);

    // Three arch-specific SLIST_HEADER_0 variants must be present.
    // arm64 and x64 share identical fields but differ in their arch attribute,
    // so they must NOT be deduplicated.
    let definitions: Vec<_> = rdl.match_indices("struct SLIST_HEADER_0").collect();
    assert_eq!(
        definitions.len(),
        3,
        "expected 3 arch-specific SLIST_HEADER_0 definitions (Arm64, X64, X86); \
         if this is 1 or 2, the arch attribute was not propagated to flat nested types"
    );

    // Every definition must be directly preceded by its own SupportedArchitecture
    // attribute (within 200 characters, which is always satisfied in practice).
    for (pos, _) in &definitions {
        let preceding = &rdl[pos.saturating_sub(200)..*pos];
        assert!(
            preceding.contains("SupportedArchitecture"),
            "SLIST_HEADER_0 at offset {pos} is missing its SupportedArchitecture attribute"
        );
    }
}
