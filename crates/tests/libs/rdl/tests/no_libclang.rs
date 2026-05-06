//! Negative test that confirms `windows_rdl::clang_version()` returns an
//! error when libclang is not available on the machine. This is the
//! companion test for the `rdl-no-libclang` CI workflow which exercises
//! `windows-rdl`'s non-clang surface (reader/writer) on a runner that has
//! had libclang removed.
//!
//! The test only runs when the `WINDOWS_RDL_NO_LIBCLANG` environment
//! variable is set, so it is a no-op for normal test runs (where libclang
//! is installed and `clang_version()` would succeed).

#[test]
fn clang_version_fails_without_libclang() {
    if std::env::var_os("WINDOWS_RDL_NO_LIBCLANG").is_none() {
        // Not running under the no-libclang workflow; skip.
        return;
    }

    let result = windows_rdl::clang_version();
    let err = result.expect_err(
        "expected clang_version() to fail when libclang is not installed, but it succeeded",
    );
    let message = format!("{err}");
    assert!(
        message.contains("failed to load libclang"),
        "expected error message to mention 'failed to load libclang', got: {message:?}"
    );
}
