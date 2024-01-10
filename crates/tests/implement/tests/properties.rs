#![allow(non_snake_case)]

use windows::{core::*, Win32::System::Com::*, Win32::UI::Shell::PropertiesSystem::*};

#[implement(IInitializeWithStream, IPropertyStore, IPropertyStoreCapabilities)]
struct Object(std::sync::RwLock<PROPVARIANT>);

impl IInitializeWithStream_Impl for Object {
    fn Initialize(&self, _: Option<&IStream>, _: u32) -> Result<()> {
        Ok(())
    }
}

impl IPropertyStore_Impl for Object {
    fn GetCount(&self) -> Result<u32> {
        Ok(123)
    }
    fn GetAt(&self, _: u32, _: *mut PROPERTYKEY) -> Result<()> {
        unimplemented!()
    }
    fn GetValue(&self, _: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        let reader = self.0.read().unwrap();
        Ok(reader.clone())
    }
    fn SetValue(&self, _: *const PROPERTYKEY, value: *const PROPVARIANT) -> Result<()> {
        let mut writer = self.0.write().unwrap();
        *writer = unsafe { (*value).clone() };
        Ok(())
    }
    fn Commit(&self) -> Result<()> {
        unimplemented!()
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
        let a: IInitializeWithStream = Object(std::sync::RwLock::new(Default::default())).into();
        a.Initialize(None, 0)?;

        let b: IPropertyStore = a.cast()?;
        assert!(b.GetCount()? == 123);

        b.SetValue(std::ptr::null(), &PROPVARIANT::from(123))?;
        assert_eq!(PROPVARIANT::from(123), b.GetValue(std::ptr::null())?);

        let c: IPropertyStoreCapabilities = b.cast()?;
        c.IsPropertyWritable(&PROPERTYKEY::default())?;

        Ok(())
    }
}
