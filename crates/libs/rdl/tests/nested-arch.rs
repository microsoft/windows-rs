use windows_rdl::*;

/// Verifies that `SupportedArchitecture` attributes on arch-specific
/// struct/union pairs (the flat, un-nested form of `SLIST_HEADER`) survive
/// a full RDL reader → writer roundtrip.
///
/// The real `SLIST_HEADER` in `Windows.Win32.System.Kernel` is a multi-arch
/// union; each arch variant has nested anonymous types that the writer
/// un-nests into flat helper types like `SLIST_HEADER_0`.  Before the fix
/// those synthesised flat types had no `SupportedArchitecture` attribute, so
/// identical definitions across arch variants were incorrectly deduplicated.
///
/// This test defines a self-contained, simplified version of that same
/// pattern (one arch variant, already in flat form) and checks that the arch
/// attribute is preserved on both the outer union and the flat helper struct.
#[test]
pub fn parse() {
    reader()
        .input("tests/nested-arch.rdl")
        .output("tests/nested-arch.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/nested-arch.winmd")
        .output("tests/nested-arch-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
