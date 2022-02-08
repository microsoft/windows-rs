#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub trait IDDEInitializer_Impl: Sized {
    fn Initialize(&self, fileextensionorprotocol: super::super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::super::Foundation::PWSTR, exectarget: &::core::option::Option<super::super::super::UI::Shell::IShellItem>, site: &::core::option::Option<::windows::core::IUnknown>, application: super::super::super::Foundation::PWSTR, targetfile: super::super::super::Foundation::PWSTR, arguments: super::super::super::Foundation::PWSTR, verb: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDEInitializer_Impl, const OFFSET: isize>() -> IDDEInitializer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: super::super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::super::Foundation::PWSTR, exectarget: ::windows::core::RawPtr, site: *mut ::core::ffi::c_void, application: super::super::super::Foundation::PWSTR, targetfile: super::super::super::Foundation::PWSTR, arguments: super::super::super::Foundation::PWSTR, verb: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&fileextensionorprotocol), ::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&currentdirectory), ::core::mem::transmute(&exectarget), ::core::mem::transmute(&site), ::core::mem::transmute_copy(&application), ::core::mem::transmute_copy(&targetfile), ::core::mem::transmute_copy(&arguments), ::core::mem::transmute_copy(&verb)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDDEInitializer as ::windows::core::Interface>::IID
    }
}
