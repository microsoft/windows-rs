// This tests uses PropertyValue to test static WinRT classes - those classes that lack a default interface
// and thus only provide static methods.

use windows::core::RuntimeName;
use windows::Foundation::PropertyValue;

#[test]
fn static_class() -> windows::core::Result<()> {
    assert_eq!(PropertyValue::NAME, "Windows.Foundation.PropertyValue");

    // TODO: test PropertyValue's methods here

    Ok(())
}
