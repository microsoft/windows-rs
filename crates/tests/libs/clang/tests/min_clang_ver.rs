#![cfg(target_pointer_width = "64")]

/// Verifies that the loaded libclang is at least the minimum tested major
/// version (18).  Older LLVM/Clang releases are known to behave unreliably
/// with windows-rdl (e.g. incorrect `is_from_main_file()` results for
/// declarations inside `extern "C" {}` blocks).
#[test]
fn min_clang_ver() {
    let version = windows_rdl::clang_version().expect("failed to get clang version");
    // The version string contains "clang version <MAJOR>.<MINOR>.<PATCH>"
    // and may be prefixed by a vendor string, e.g.
    //   "clang version 18.1.0 (...)"
    //   "Ubuntu clang version 18.1.3 (1ubuntu1)"
    const PREFIX: &str = "clang version ";
    let major: u32 = version
        .find(PREFIX)
        .map(|pos| &version[pos + PREFIX.len()..])
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or_else(|| panic!("unexpected clang version string: {version:?}"));
    assert!(
        major >= 18,
        "libclang {major} is below the minimum tested version (18); got: {version:?}"
    );
}
