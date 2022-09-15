#![allow(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

// This defines a new local interface (based on IPersistMemory) that derives from an existing interface defined by the `windows` crate.
#[interface("BD1AE5E0-A6AE-11CE-BD37-504200C10000")]
unsafe trait ITestPersistMemory: IPersist {
    unsafe fn IsDirty(&self) -> HRESULT;
}

#[implement(ITestPersistMemory, IPersist)]
struct Test;

impl IPersist_Impl for Test {
    fn GetClassID(&self) -> Result<GUID> {
        Ok("CEE1D356-0860-4262-90D4-C77423F0E352".into())
    }
}

impl ITestPersistMemory_Impl for Test {
    unsafe fn IsDirty(&self) -> HRESULT {
        S_FALSE
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
    let p: IPersist = Test.into();
    assert_eq!(p.GetClassID()?, "CEE1D356-0860-4262-90D4-C77423F0E352".into());

    let m: ITestPersistMemory = p.cast()?;
    assert_eq!(m.IsDirty(), S_FALSE);

    Ok(())
    }
}

