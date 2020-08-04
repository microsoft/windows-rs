// A simple test to ensure that the Windows.Foundation.Diagnostics namespace can be imported
// as it lives within the foundation namespace that is partially included by the winrt crate.

winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
        nuget: KennyKerr.Windows.TestWinRT
    types
        windows::foundation::diagnostics::*
);

use windows::foundation::diagnostics::*;

#[test]
fn diagnostics() {
    let _ = ErrorDetails::default();
}
