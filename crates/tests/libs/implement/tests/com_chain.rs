#![cfg(windows)]
use windows::Win32::{objidl::*, objidlbase::*};
use windows::core::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test_Impl {
    fn GetClassID(&self) -> Result<GUID> {
        Ok(GUID::zeroed())
    }
}

impl IPersistStream_Impl for Test_Impl {
    fn IsDirty(&self) -> Result<()> {
        Ok(())
    }

    fn Load(&self, _: Ref<IStream>) -> Result<()> {
        Ok(())
    }

    fn Save(&self, _: Ref<IStream>, _: BOOL) -> Result<()> {
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
        stream.IsDirty().ok()?; // IPersistStream
        stream.cast::<IPersistStream>()?;
        stream.cast::<IPersist>()?;

        let persist: &IPersist = &stream.cast()?;
        persist.GetClassID()?;
        persist.cast::<IPersistStream>()?;
        persist.cast::<IPersist>()?;

        Ok(())
    }
}
