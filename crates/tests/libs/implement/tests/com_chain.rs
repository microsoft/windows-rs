use windows::core::*;
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test_Impl {
    fn GetClassID(&self) -> Result<GUID> {
        Ok(GUID::zeroed())
    }
}

impl IPersistStream_Impl for Test_Impl {
    fn IsDirty(&self) -> HRESULT {
        S_OK
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
