#![cfg(windows)]
#![expect(non_snake_case)]

#[windows::core::interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ITestPersistMemory: windows::Win32::IPersist {
    unsafe fn IsDirty(&self) -> windows::core::HRESULT;
}

#[windows::core::implement(ITestPersistMemory, windows::Win32::IPersist)]
struct Test;

impl windows::Win32::IPersist_Impl for Test_Impl {
    fn GetClassID(&self, class_id: *mut windows::core::GUID) -> windows::core::Result<()> {
        unsafe { *class_id = "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()? };
        Ok(())
    }
}

impl ITestPersistMemory_Impl for Test_Impl {
    unsafe fn IsDirty(&self) -> windows::core::HRESULT {
        windows::Win32::S_FALSE
    }
}

#[test]
fn test() -> windows::core::Result<()> {
    unsafe {
        let p: windows::Win32::IPersist = Test.into();
        let mut class_id = windows::core::GUID::default();
        p.GetClassID(&mut class_id).ok()?;
        assert_eq!(class_id, "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()?);

        let m: ITestPersistMemory = windows_core::Interface::cast(&p)?;
        assert_eq!(m.IsDirty(), windows::Win32::S_FALSE);

        Ok(())
    }
}
