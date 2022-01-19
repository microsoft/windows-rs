use test_win32_class_factory::*;
use windows::core::*;
use Windows::Foundation::*;
use Windows::Win32::Foundation::BOOL;
use Windows::Win32::System::Com::IClassFactory;

#[implement(Windows::Foundation::{IClosable, IStringable})]
struct Object();

#[allow(non_snake_case)]
impl Object {
    pub fn ToString(&self) -> Result<HSTRING> {
        Ok("Object".into())
    }

    pub fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[implement(Windows::Win32::System::Com::IClassFactory)]
struct Factory();

#[allow(non_snake_case)]
impl Factory {
    pub fn CreateInstance(&self, outer: &Option<IUnknown>, iid: &GUID, object: *mut RawPtr) -> HRESULT {
        assert!(outer.is_none());
        let unknown: IUnknown = Object().into();
        unsafe { unknown.query(iid, object) }
    }

    pub fn LockServer(&self, lock: BOOL) -> Result<()> {
        assert!(lock.as_bool());
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let factory: IClassFactory = Factory().into();
        factory.LockServer(true)?;

        let stringable: IStringable = factory.CreateInstance(None)?;
        assert!(stringable.ToString()? == "Object");

        let closable: IClosable = factory.CreateInstance(None)?;
        closable.Close()?;

        Ok(())
    }
}
