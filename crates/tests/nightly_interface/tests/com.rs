#![allow(non_snake_case)]

use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

/// A custom declaration of implementation of `IUri`
#[interface("a39ee748-6a27-4817-a6f2-13914bef5890")]
pub unsafe trait ICustomUri: IUnknown {
    unsafe fn GetPropertyBSTR(&self, property: Uri_PROPERTY, value: *mut BSTR, flags: u32) -> HRESULT;
    unsafe fn GetPropertyLength(&self) -> HRESULT;
    unsafe fn GetPropertyDWORD(&self, property: Uri_PROPERTY, value: *mut u32, flags: u32) -> HRESULT;
    unsafe fn HasProperty(&self) -> HRESULT;
    unsafe fn GetAbsoluteUri(&self) -> HRESULT;
    unsafe fn GetAuthority(&self) -> HRESULT;
    unsafe fn GetDisplayUri(&self) -> HRESULT;
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT;
    // etc
}

#[implement(ICustomUri)]
struct CustomUri;

impl ICustomUri_Impl for CustomUri {
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
    unsafe fn HasProperty(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetAbsoluteUri(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetAuthority(&self) -> HRESULT {
        todo!()
    }
    unsafe fn GetDisplayUri(&self) -> HRESULT {
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
