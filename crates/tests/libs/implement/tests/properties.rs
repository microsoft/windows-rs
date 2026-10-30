#![cfg(windows)]
use std::mem::ManuallyDrop;
use windows::{Win32::*, core::*};

#[implement(IInitializeWithStream, IPropertyStore, IPropertyStoreCapabilities)]
struct Object(std::sync::RwLock<PROPVARIANT>);

fn propvariant(value: i32) -> PROPVARIANT {
    PROPVARIANT {
        Anonymous: tagPROPVARIANT_0 {
            Anonymous: ManuallyDrop::new(tagPROPVARIANT_0_0 {
                vt: VT_I4 as VARTYPE,
                wReserved1: 0,
                wReserved2: 0,
                wReserved3: 0,
                Anonymous: tagPROPVARIANT_0_0_0 { lVal: value },
            }),
        },
    }
}

fn propvariant_value(value: &PROPVARIANT) -> i32 {
    unsafe { value.Anonymous.Anonymous.Anonymous.lVal }
}

impl IInitializeWithStream_Impl for Object_Impl {
    fn Initialize(&self, _: Ref<IStream>, _: u32) -> Result<()> {
        Ok(())
    }
}

impl IPropertyStore_Impl for Object_Impl {
    fn GetCount(&self, count: *mut u32) -> Result<()> {
        unsafe { *count = 123 };
        Ok(())
    }
    fn GetAt(&self, _: u32, _: *mut PROPERTYKEY) -> Result<()> {
        unimplemented!()
    }
    fn GetValue(&self, _: *const PROPERTYKEY, value: *mut PROPVARIANT) -> Result<()> {
        let reader = self.0.read().unwrap();
        unsafe { value.write(reader.clone()) };
        Ok(())
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

impl IPropertyStoreCapabilities_Impl for Object_Impl {
    fn IsPropertyWritable(&self, _: *const PROPERTYKEY) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let a: IInitializeWithStream = Object(std::sync::RwLock::new(Default::default())).into();
        a.Initialize(None, 0).ok()?;

        let b: IPropertyStore = a.cast()?;
        let mut count = 0;
        b.GetCount(&mut count).ok()?;
        assert!(count == 123);

        b.SetValue(std::ptr::null(), &propvariant(123)).ok()?;
        let mut value = PROPVARIANT::default();
        b.GetValue(std::ptr::null(), &mut value).ok()?;
        assert_eq!(123, propvariant_value(&value));

        let c: IPropertyStoreCapabilities = b.cast()?;
        c.IsPropertyWritable(&PROPERTYKEY::default()).ok()?;

        Ok(())
    }
}
