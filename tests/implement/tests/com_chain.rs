use test_implement::*;
use windows::*;
use Windows::Win32::Foundation::BOOL;
use Windows::Win32::Storage::StructuredStorage::IStream;
use Windows::Win32::System::Com::{IPersist, IPersistStream};

#[implement(Windows::Win32::System::Com::IPersistStream)]
struct Test();

#[allow(non_snake_case)]
impl Test {
    fn GetClassID(&self) -> Result<Guid> {
        Ok(Guid::zeroed())
    }

    fn IsDirty(&self) -> Result<()> {
        Ok(())
    }

    fn Load(&self, _: &Option<IStream>) -> Result<()> {
        Ok(())
    }

    // TODO: should be able to declare BOOL without a reference.
    fn Save(&self, _: &Option<IStream>, _: &BOOL) -> Result<()> {
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
