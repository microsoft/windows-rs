#![allow(non_snake_case)]

use windows::core::*;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Com::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test {
    fn GetClassID(&self) -> Result<GUID> {
        Ok(GUID::zeroed())
    }
}

impl IPersistStream_Impl for Test {
    fn IsDirty(&self) -> Result<()> {
        Ok(())
    }

    fn Load(&self, _: &Option<IStream>) -> Result<()> {
        Ok(())
    }

    fn Save(&self, _: &Option<IStream>, _: BOOL) -> Result<()> {
        Ok(())
    }

    fn GetSizeMax(&self) -> Result<u64> {
        Ok(0)
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream: IPersistStream = Test().into();
        stream.GetClassID()?; // IPersist
        stream.IsDirty()?; // IPersistStream
        stream.cast::<IPersistStream>()?;
        stream.cast::<IPersist>()?;

        let persist: IPersist = stream.into();
        persist.GetClassID()?;
        persist.cast::<IPersistStream>()?;
        persist.cast::<IPersist>()?;

        Ok(())
    }
}
