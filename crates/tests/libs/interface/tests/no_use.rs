#![cfg(windows)]
#![expect(non_snake_case)]

#[windows_core::interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ITestPersistMemory: test_interface::IPersist {
    unsafe fn IsDirty(&self) -> windows_core::HRESULT;
}

#[windows_core::implement(ITestPersistMemory, test_interface::IPersist)]
struct Test;

impl test_interface::IPersist_Impl for Test_Impl {
    fn GetClassID(&self, class_id: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe {
            *class_id = windows_core::GUID::from_u128(0xCEE1D356_0860_4262_90D4_C77423F0E352);
        };
        test_interface::S_OK
    }
}

impl ITestPersistMemory_Impl for Test_Impl {
    unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        test_interface::S_FALSE
    }
}

#[test]
fn test() -> windows_core::Result<()> {
    unsafe {
        let p: test_interface::IPersist = Test.into();
        let mut class_id = windows_core::GUID::default();
        p.GetClassID(&mut class_id).ok()?;
        assert_eq!(class_id, "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()?);

        let m: ITestPersistMemory = windows_core::Interface::cast(&p)?;
        assert_eq!(m.IsDirty(), test_interface::S_FALSE);

        Ok(())
    }
}
