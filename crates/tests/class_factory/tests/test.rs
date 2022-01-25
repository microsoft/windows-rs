#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;

#[implement(IClosable, IStringable)]
struct Object();

impl IStringable_Impl for Object {
    fn ToString(&mut self) -> Result<HSTRING> {
        Ok("Object".into())
    }
}

impl IClosable_Impl for Object {
    fn Close(&mut self) -> Result<()> {
        Ok(())
    }
}

#[implement(IClassFactory)]
struct Factory();

impl IClassFactory_Impl for Factory {
    fn CreateInstance(&mut self, outer: &Option<IUnknown>, iid: *const GUID, object: *mut RawPtr) -> Result<()> {
        assert!(outer.is_none());
        let unknown: IInspectable = Object().into();
        // TODO: https://github.com/microsoft/windows-rs/issues/1441
        unsafe { unknown.query(&*iid, object).ok() }
    }

    fn LockServer(&mut self, lock: BOOL) -> Result<()> {
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
