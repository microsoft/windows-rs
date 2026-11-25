#![cfg(windows)]
// Note: this test purposefully does not use the `use` keyword to validate that the implement macro
// doesn't rely on contextual names.

#[windows_core::implement(test_implement::{IStringable, IClosable})]
struct Test(&'static str);

impl test_implement::IStringable_Impl for Test_Impl {
    fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        Ok(self.0.into())
    }
}

impl test_implement::IClosable_Impl for Test_Impl {
    fn Close(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> windows_core::Result<()> {
    let a: test_implement::IStringable = Test("test").into();
    assert!(a.ToString()? == "test");

    let b: test_implement::IClosable = windows_core::Interface::cast(&a)?;
    b.Close()?;

    Ok(())
}
