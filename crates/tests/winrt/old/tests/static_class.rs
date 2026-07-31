#![cfg(windows)]
use windows::Foundation::PropertyValue;
use windows::core::RuntimeName;

#[test]
fn static_class() -> windows::core::Result<()> {
    assert_eq!(PropertyValue::NAME, "Windows.Foundation.PropertyValue");

    Ok(())
}
