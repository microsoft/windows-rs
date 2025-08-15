#![expect(non_snake_case)]

// This is a variant of the interface_core/no_use.rs test for use with more complex paths provided by the windows crate.

#[windows::core::interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ITestPersistMemory: windows::Win32::System::Com::IPersist {
    unsafe fn IsDirty(&self) -> windows::core::HRESULT;
}

#[windows::core::implement(ITestPersistMemory, windows::Win32::System::Com::IPersist)]
struct Test;

impl windows::Win32::System::Com::IPersist_Impl for Test_Impl {
    fn GetClassID(&self) -> windows::core::Result<windows::core::GUID> {
        "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()
    }
}

impl ITestPersistMemory_Impl for Test_Impl {
    unsafe fn IsDirty(&self) -> windows::core::HRESULT {
        windows::Win32::Foundation::S_FALSE
    }
}

#[test]
fn test() -> windows::core::Result<()> {
    unsafe {
        let p: windows::Win32::System::Com::IPersist = Test.into();
        assert_eq!(
            p.GetClassID()?,
            "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()?
        );

        let m: ITestPersistMemory = windows_core::Interface::cast(&p)?;
        assert_eq!(m.IsDirty(), windows::Win32::Foundation::S_FALSE);

        Ok(())
    }
}
