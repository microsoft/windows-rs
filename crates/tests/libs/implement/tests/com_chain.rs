#![cfg(windows)]
use windows::Win32::*;
use windows::core::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test_Impl {
    fn GetClassID(&self, class_id: *mut GUID) -> Result<()> {
        unsafe { *class_id = GUID::zeroed() };
        Ok(())
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

    fn GetSizeMax(&self, size: *mut ULARGE_INTEGER) -> Result<()> {
        unsafe { *size = ULARGE_INTEGER { QuadPart: 0 } };
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream: IPersistStream = Test().into();
        stream.GetClassID(&mut GUID::zeroed()).ok()?; // IPersist
        stream.IsDirty().ok()?; // IPersistStream
        stream.cast::<IPersistStream>()?;
        stream.cast::<IPersist>()?;

        let persist: &IPersist = &stream.cast()?;
        persist.GetClassID(&mut GUID::zeroed()).ok()?;
        persist.cast::<IPersistStream>()?;
        persist.cast::<IPersist>()?;

        Ok(())
    }
}
