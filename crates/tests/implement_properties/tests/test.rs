#![allow(non_snake_case)]

use windows::{core::*, Win32::System::Com::StructuredStorage::*, Win32::System::Com::*, Win32::UI::Shell::PropertiesSystem::*};

#[implement(IInitializeWithStream, IPropertyStore, IPropertyStoreCapabilities)]
struct Object();

impl IInitializeWithStream_Impl for Object {
    fn Initialize(&self, _: &Option<IStream>, _: u32) -> Result<()> {
        Ok(())
    }
}

impl IPropertyStore_Impl for Object {
    fn GetCount(&self) -> Result<u32> {
        Ok(123)
    }
    fn GetAt(&self, _: u32) -> Result<PROPERTYKEY> {
        todo!()
    }
    fn GetValue(&self, _: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        todo!()
    }
    fn SetValue(&self, _: *const PROPERTYKEY, _: *const PROPVARIANT) -> Result<()> {
        todo!()
    }
    fn Commit(&self) -> Result<()> {
        todo!()
    }
}

impl IPropertyStoreCapabilities_Impl for Object {
    fn IsPropertyWritable(&self, _: *const PROPERTYKEY) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let a: IInitializeWithStream = Object().into();
        a.Initialize(None, 0)?;

        let b: IPropertyStore = a.cast()?;
        assert!(b.GetCount()? == 123);

        let c: IPropertyStoreCapabilities = b.cast()?;
        c.IsPropertyWritable(std::ptr::null())?;

        Ok(())
    }
}
