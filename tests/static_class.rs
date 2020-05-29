// This tests uses PropertyValue to test static WinRT classes - those classes that lack a default interface
// and thus only provide static methods.

winrt::import!(
    dependencies
        os
    types
        windows::foundation::PropertyValue
);
use windows::foundation::PropertyValue;
use winrt::RuntimeName;

#[test]
fn static_class() -> winrt::Result<()> {
    assert_eq!(PropertyValue::NAME, "Windows.Foundation.PropertyValue");

    // TODO: test PropertyValue's methods here

    Ok(())
}
