use windows::core::*;
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Com::{IPersist, IPersistStream, IStream};
use windows as Windows;

#[implement(Windows::Win32::System::Com::IPersistStream)]
struct Test();

#[allow(non_snake_case)]
impl Test {
    fn GetClassID(&self) -> Result<GUID> {
        Ok(GUID::zeroed())
    }

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
