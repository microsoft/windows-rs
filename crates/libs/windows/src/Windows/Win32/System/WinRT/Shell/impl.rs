#[cfg(feature = "Win32_UI_Shell")]
pub trait IDDEInitializer_Impl: Sized {
    fn Initialize(&self, fileextensionorprotocol: &::windows::core::PCWSTR, method: CreateProcessMethod, currentdirectory: &::windows::core::PCWSTR, exectarget: &::core::option::Option<super::super::super::UI::Shell::IShellItem>, site: &::core::option::Option<::windows::core::IUnknown>, application: &::windows::core::PCWSTR, targetfile: &::windows::core::PCWSTR, arguments: &::windows::core::PCWSTR, verb: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell")]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDEInitializer_Impl, const OFFSET: isize>() -> IDDEInitializer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows::core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows::core::PCWSTR, exectarget: ::windows::core::RawPtr, site: *mut ::core::ffi::c_void, application: ::windows::core::PCWSTR, targetfile: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, verb: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&fileextensionorprotocol), ::core::mem::transmute_copy(&method), ::core::mem::transmute(&currentdirectory), ::core::mem::transmute(&exectarget), ::core::mem::transmute(&site), ::core::mem::transmute(&application), ::core::mem::transmute(&targetfile), ::core::mem::transmute(&arguments), ::core::mem::transmute(&verb)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDDEInitializer as ::windows::core::Interface>::IID
    }
}
