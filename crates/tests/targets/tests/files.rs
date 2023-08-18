// This test just makes it easier to ensure that the lib file names match the crate versions
// so we don't accidentally forget to bump the version numbers embedded in the lib file names.

#[test]
fn test() {
    std::include_bytes!("../../../targets/aarch64_gnullvm/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/aarch64_msvc/lib/windows.0.52.0.lib");
    std::include_bytes!("../../../targets/i686_gnu/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/i686_msvc/lib/windows.0.52.0.lib");
    std::include_bytes!("../../../targets/x86_64_gnu/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/x86_64_gnullvm/lib/libwindows.0.52.0.a");
    std::include_bytes!("../../../targets/x86_64_msvc/lib/windows.0.52.0.lib");
}
