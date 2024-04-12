// This test just makes it easier to ensure that the lib file names match the crate versions
// so we don't accidentally forget to bump the version numbers embedded in the lib file names.

#[test]
fn test() {
    let targets = tool_lib::crates("../../targets");
    assert_eq!(8, targets.len());
    assert!(targets.iter().all(|(_, version)| version == "0.52.5"));

    // The lib names can't change for minor (semver) updates as that breaks linker search.
    // https://github.com/microsoft/windows-rs/issues/2869
    std::include_bytes!("../../../targets/aarch64_gnullvm/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/aarch64_msvc/lib/windows.0.52.0.lib");
    std::include_bytes!("../../../targets/i686_gnu/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/i686_gnullvm/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/i686_msvc/lib/windows.0.52.0.lib");
    std::include_bytes!("../../../targets/x86_64_gnu/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/x86_64_gnullvm/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/x86_64_msvc/lib/windows.0.52.0.lib");
}
