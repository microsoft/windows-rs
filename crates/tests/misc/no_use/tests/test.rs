// Note: this test purposefully does not use the `use` keyword to validate that the implement macro
// doesn't rely on contextual names.

#[windows::core::implement(windows::Foundation::{IStringable, IClosable})]
struct Test(&'static str);

impl windows::Foundation::IStringable_Impl for Test_Impl {
    fn ToString(&self) -> windows::core::Result<windows::core::HSTRING> {
        Ok(self.0.into())
    }
}

impl windows::Foundation::IClosable_Impl for Test_Impl {
    fn Close(&self) -> windows::core::Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> windows::core::Result<()> {
    let a: windows::Foundation::IStringable = Test("test").into();
    assert!(a.ToString()? == "test");

    let b: windows::Foundation::IClosable = windows::core::Interface::cast(&a)?;
    b.Close()?;

    Ok(())
}
