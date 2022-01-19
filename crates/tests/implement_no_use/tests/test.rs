#![allow(non_snake_case)]

// Note: this test purposefully does not use the `use` keyword to validate that hte implement macro
// doesn't rely on contextual names.

#[windows::core::implement(windows::Foundation::IStringable, windows::Foundation::IClosable)]
struct Test();

impl windows::Foundation::IStringable_Impl for Test {
    fn ToString(&mut self) -> windows::core::Result<windows::core::HSTRING> {
        Ok("test".into())
    }
}

impl windows::Foundation::IClosable_Impl for Test {
    fn Close(&mut self) -> windows::core::Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> windows::core::Result<()> {
    let a: windows::Foundation::IStringable = Test().into();
    assert!(a.ToString()? == "test");

    let b: windows::Foundation::IClosable = windows::core::Interface::cast(&a)?;
    b.Close()?;

    Ok(())
}
