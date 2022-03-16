#![allow(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

/// A custom declaration of implementation of `IUri`
#[interface("a39ee748-6a27-4817-a6f2-13914bef5890")]
pub unsafe trait ICustomUri: IUnknown {
    unsafe fn GetPropertyBSTR(&self, property: Uri_PROPERTY, value: *mut BSTR, flags: u32) -> HRESULT;
    unsafe fn GetPropertyLength(&self) -> HRESULT;
    unsafe fn GetPropertyDWORD(&self, property: Uri_PROPERTY, value: *mut u32, flags: u32) -> HRESULT;
    unsafe fn HasProperty(&self); // Note: this definition is missing its return value
    unsafe fn GetAbsoluteUri(&self) -> HRESULT;
    unsafe fn GetAuthority(&self) -> HRESULT;
    unsafe fn GetDisplayUri(&self) -> i32;
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT;
    // etc
}

#[implement(ICustomUri)]
struct CustomUri;

impl ICustomUriImpl for CustomUri {
    unsafe fn GetPropertyBSTR(&self, property: Uri_PROPERTY, value: *mut BSTR, flags: u32) -> HRESULT {
        assert!(flags == 0);
        assert!(property == Uri_PROPERTY_DOMAIN);
        *value = "property".into();
        S_OK
    }
    unsafe fn GetPropertyLength(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetPropertyDWORD(&self, property: Uri_PROPERTY, value: *mut u32, flags: u32) -> HRESULT {
        assert!(flags == 0);
        assert!(property == Uri_PROPERTY_PORT);
        *value = 123;
        S_OK
    }
    unsafe fn HasProperty(&self) {
        todo!()
    }
    unsafe fn GetAbsoluteUri(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetAuthority(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetDisplayUri(&self) -> i32 {
        todo!()
    }
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT {
        *value = "kennykerr.ca".into();
        S_OK
    }
}

#[test]
fn test_custom_interface() -> windows::core::Result<()> {
    unsafe {
        // Use the OS implementation through the OS interface
        let a: IUri = CreateUri("http://kennykerr.ca", Default::default(), 0)?;
        let domain = a.GetDomain()?;
        assert_eq!(domain, "kennykerr.ca");
        let mut property = BSTR::new();
        a.GetPropertyBSTR(Uri_PROPERTY_DOMAIN, &mut property, 0)?;
        assert_eq!(property, "kennykerr.ca");
        let mut property = 0;
        a.GetPropertyDWORD(Uri_PROPERTY_PORT, &mut property, 0)?;
        assert_eq!(property, 80);

        // Call the OS implementation through the custom interface
        let b: ICustomUri = a.cast()?;
        let mut domain = BSTR::new();
        b.GetDomain(&mut domain).ok()?;
        assert_eq!(domain, "kennykerr.ca");
        let mut property = BSTR::new();
        a.GetPropertyBSTR(Uri_PROPERTY_DOMAIN, &mut property, 0)?;
        assert_eq!(property, "kennykerr.ca");
        let mut property = 0;
        a.GetPropertyDWORD(Uri_PROPERTY_PORT, &mut property, 0)?;
        assert_eq!(property, 80);

        // Use the custom implementation through the OS interface
        let c: ICustomUri = CustomUri.into();
        // This works because `ICustomUri` and `IUri` share the same guid
        let c: IUri = c.cast()?;
        let domain = c.GetDomain()?;
        assert_eq!(domain, "kennykerr.ca");
        let mut property = BSTR::new();
        c.GetPropertyBSTR(Uri_PROPERTY_DOMAIN, &mut property, 0)?;
        assert_eq!(property, "property");
        let mut property = 0;
        c.GetPropertyDWORD(Uri_PROPERTY_PORT, &mut property, 0)?;
        assert_eq!(property, 123);

        // Call the custom implementation through the custom interface
        let d: ICustomUri = c.cast()?;
        let mut domain = BSTR::new();
        d.GetDomain(&mut domain).ok()?;
        assert_eq!(domain, "kennykerr.ca");
        let mut property = BSTR::new();
        d.GetPropertyBSTR(Uri_PROPERTY_DOMAIN, &mut property, 0).ok()?;
        assert_eq!(property, "property");
        let mut property = 0;
        d.GetPropertyDWORD(Uri_PROPERTY_PORT, &mut property, 0).ok()?;
        assert_eq!(property, 123);

        Ok(())
    }
}

#[repr(C)]
pub struct Food {
    deliciousness: usize,
}

#[interface("EFF8970E-C50F-45E0-9284-291CE5A6F771")]
pub unsafe trait IAnimal: IUnknown {
    unsafe fn Eat(&self, food: *const Food) -> HRESULT;
    unsafe fn Happiness(&self) -> usize;
}

#[interface("F5353C58-CFD9-4204-8D92-D274C7578B53")]
pub unsafe trait ICat: IAnimal {
    unsafe fn IgnoreHumans(&self) -> HRESULT;
}

#[interface("C22425DF-EFB2-4B85-933E-9CF7B23459E8")]
pub unsafe trait IDomesticAnimal: IAnimal {
    unsafe fn Train(&self) -> HRESULT;
}

#[implement(ICat, IDomesticAnimal, IAnimal)]
struct HouseCat(std::cell::Cell<usize>);

impl HouseCat {
    fn new() -> Self {
        Self(std::cell::Cell::new(0))
    }
}

impl ICatImpl for HouseCat {
    unsafe fn IgnoreHumans(&self) -> HRESULT {
        S_OK
    }
}

impl IDomesticAnimalImpl for HouseCat {
    unsafe fn Train(&self) -> HRESULT {
        S_OK
    }
}

impl IAnimalImpl for HouseCat {
    unsafe fn Eat(&self, food: *const Food) -> HRESULT {
        let h = self.0.get();
        let h = h + (*food).deliciousness;
        self.0.set(h);

        S_OK
    }

    unsafe fn Happiness(&self) -> usize {
        self.0.get()
    }
}

#[test]
fn test_rich_interface_hierarchy() -> windows::core::Result<()> {
    let animal: IAnimal = HouseCat::new().into();
    assert_eq!(unsafe { animal.Happiness() }, 0);
    unsafe { animal.Eat(&Food { deliciousness: 10 }).ok()? };
    assert_eq!(unsafe { animal.Happiness() }, 10);

    Ok(())
}
