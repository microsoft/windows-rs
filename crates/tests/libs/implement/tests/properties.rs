#![cfg(windows)]
use std::mem::ManuallyDrop;
use windows::{core::*, Win32::*};

#[implement(IInitializeWithStream, IPropertyStore, IPropertyStoreCapabilities)]
struct Object(std::sync::RwLock<PROPVARIANT>);

fn propvariant(value: i32) -> PROPVARIANT {
    PROPVARIANT {
        Anonymous: PROPVARIANT_0 {
            Anonymous: ManuallyDrop::new(PROPVARIANT_0_0 {
                vt: VARTYPE(VT_I4 as u16),
                wReserved1: PROPVAR_PAD1(0),
                wReserved2: PROPVAR_PAD2(0),
                wReserved3: PROPVAR_PAD3(0),
                Anonymous: PROPVARIANT_0_0_0 { lVal: value },
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
        assert!(b.GetCount()? == 123);

        b.SetValue(std::ptr::null(), &propvariant(123)).ok()?;
        assert_eq!(123, propvariant_value(&b.GetValue(std::ptr::null())?));

        let c: IPropertyStoreCapabilities = b.cast()?;
        c.IsPropertyWritable(&PROPERTYKEY::default()).ok()?;

        Ok(())
    }
}
