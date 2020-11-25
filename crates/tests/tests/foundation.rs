// A simple test to ensure that the Windows.Foundation.Diagnostics namespace can be imported
// as it lives within the foundation namespace that is partially included by the winrt crate.

use tests::windows::foundation::diagnostics::*;

#[test]
fn diagnostics() {
    let _: Option<ErrorDetails> = None;
}
