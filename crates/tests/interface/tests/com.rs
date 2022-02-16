#![allow(non_snake_case)]
#![feature(const_fn_trait_bound)]

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
};

#[interface("a39ee748-6a27-4817-a6f2-13914bef5890")]
pub unsafe trait ICustomUri : IUnknown {
    unsafe fn GetPropertyBSTR(&self) -> HRESULT;
    unsafe fn GetPropertyLength(&self) -> HRESULT;
    unsafe fn GetPropertyDWORD(&self) -> HRESULT;
    unsafe fn HasProperty(&self) -> HRESULT;
    unsafe fn GetAbsoluteUri(&self) -> HRESULT;
    unsafe fn GetAuthority(&self) -> HRESULT;
    unsafe fn GetDisplayUri(&self) -> HRESULT;
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT;
    // etc
}

#[implement(ICustomUri, IUri)]
struct CustomUri;

impl ICustomUri_Impl for CustomUri {
    unsafe fn GetPropertyBSTR(&self)  -> HRESULT{ todo!() }
    unsafe fn GetPropertyLength(&self)  -> HRESULT{ todo!() }
    unsafe fn GetPropertyDWORD(&self)  -> HRESULT{ todo!() }
    unsafe fn HasProperty(&self)  -> HRESULT{ todo!() }
    unsafe fn GetAbsoluteUri(&self)  -> HRESULT{ todo!() }
    unsafe fn GetAuthority(&self)  -> HRESULT{ todo!() }
    unsafe fn GetDisplayUri(&self)  -> HRESULT{ todo!() }
    unsafe fn GetDomain(&self, value: *mut BSTR) -> HRESULT {
        *value = "kennykerr.ca".into();
        S_OK
    }
}

impl IUri_Impl for CustomUri {
    fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut BSTR, dwflags: u32) -> ::windows::core::Result<()> {
        todo!()
    }
    fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        todo!()
    }
    fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        todo!()
    }
    fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<BOOL> { todo!() }
    fn GetAbsoluteUri(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetAuthority(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetDisplayUri(&self)  -> ::windows::core::Result<BSTR> { todo!() }
    fn GetDomain(&self) -> ::windows::core::Result<BSTR> {
        Ok("kennykerr.ca".into())
    }
    fn GetExtension(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetFragment(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetHost(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetPassword(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetPath(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetPathAndQuery(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetQuery(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetRawUri(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetSchemeName(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetUserInfo(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetUserName(&self) -> ::windows::core::Result<BSTR> { todo!() }
    fn GetHostType(&self) -> ::windows::core::Result<u32> { todo!() }
    fn GetPort(&self) -> ::windows::core::Result<u32> { todo!() }
    fn GetScheme(&self) -> ::windows::core::Result<u32> { todo!() }
    fn GetZone(&self) -> ::windows::core::Result<u32> { todo!() }
    fn GetProperties(&self) -> ::windows::core::Result<u32> { todo!() }
    fn IsEqual(&self, puri: &::core::option::Option<IUri>) -> ::windows::core::Result<BOOL> { todo!() }
}

#[test]
fn test_custom_interface() -> windows::core::Result<()> {
    unsafe {
        // Use the OS implementation through the OS interface
        let a: IUri = CreateUri("http://kennykerr.ca", Default::default(), 0)?;
        let domain = a.GetDomain()?;
        assert_eq!(domain, "kennykerr.ca");

        // Call the OS implementation through the custom interface
        let b: ICustomUri = a.cast()?;
        let mut domain = BSTR::new();
        b.GetDomain(&mut domain).ok()?;
        assert_eq!(domain, "kennykerr.ca");

        // Use the custom implementation through the OS interface
        let c: IUri = CustomUri.into();
        let domain = c.GetDomain()?;
        assert_eq!(domain, "kennykerr.ca");

        // Call the custom implementation through the custom interface
        let d: ICustomUri = c.cast()?;
        let mut domain = BSTR::new();
        d.GetDomain(&mut domain).ok()?;
        assert_eq!(domain, "kennykerr.ca");

        Ok(())
    }
}