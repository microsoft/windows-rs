use windows::core::*;
use windows::Foundation::*;
use windows::Win32::System::Com::*;

#[implement(IClosable, IStringable)]
struct Object();

impl IStringable_Impl for Object_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Object".into())
    }
}

impl IClosable_Impl for Object_Impl {
    fn Close(&self) -> Result<()> {
        Ok(())
    }
}

#[implement(IClassFactory)]
struct Factory();

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        outer: Ref<IUnknown>,
        iid: *const GUID,
        object: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        assert!(outer.is_null());
        let unknown: IInspectable = Object().into();
        unsafe { unknown.query(iid, object).ok() }
    }

    fn LockServer(&self, lock: BOOL) -> Result<()> {
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
        assert_eq!(stringable.ToString()?, "Object");

        let closable: IClosable = factory.CreateInstance(None)?;
        closable.Close()?;

        Ok(())
    }
}
