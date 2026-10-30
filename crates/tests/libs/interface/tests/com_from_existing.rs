#![cfg(windows)]
#![expect(non_snake_case)]

use windows::{Win32::*, core::*};

// A local interface derives from an existing `windows` interface.
#[interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ITestPersistMemory: IPersist {
    unsafe fn IsDirty(&self) -> HRESULT;
}

#[implement(ITestPersistMemory, IPersist)]
struct Test;

impl IPersist_Impl for Test_Impl {
    fn GetClassID(&self, class_id: *mut GUID) -> Result<()> {
        unsafe { *class_id = "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()? };
        Ok(())
    }
}

impl ITestPersistMemory_Impl for Test_Impl {
    unsafe fn IsDirty(&self) -> HRESULT {
        S_FALSE
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let p: IPersist = Test.into();
        let mut class_id = GUID::default();
        p.GetClassID(&mut class_id).ok()?;
        assert_eq!(class_id, "CEE1D356-0860-4262-90D4-C77423F0E352".try_into()?);

        let m: ITestPersistMemory = p.cast()?;
        assert_eq!(m.IsDirty(), S_FALSE);

        Ok(())
    }
}
