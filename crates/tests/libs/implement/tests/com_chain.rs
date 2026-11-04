#![cfg(windows)]
use windows::Win32::*;
use windows::core::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test_Impl {
    fn GetClassID(&self, class_id: *mut GUID) -> HRESULT {
        unsafe { *class_id = GUID::zeroed() };
        S_OK
    }
}

impl IPersistStream_Impl for Test_Impl {
    fn IsDirty(&self) -> HRESULT {
        S_OK
    }

    fn Load(&self, _: Ref<IStream>) -> HRESULT {
        S_OK
    }

    fn Save(&self, _: Ref<IStream>, _: BOOL) -> HRESULT {
        S_OK
    }

    fn GetSizeMax(&self, size: *mut ULARGE_INTEGER) -> HRESULT {
        unsafe { *size = ULARGE_INTEGER { QuadPart: 0 } };
        S_OK
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
