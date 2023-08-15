#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`, `\"Win32_UI_Shell\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_Shell")]
pub trait IDDEInitializer_Impl: Sized {
    fn Initialize(&self, fileextensionorprotocol: &::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: &::windows_core::PCWSTR, exectarget: ::core::option::Option<&super::super::super::UI::Shell::IShellItem>, site: ::core::option::Option<&::windows_core::IUnknown>, application: &::windows_core::PCWSTR, targetfile: &::windows_core::PCWSTR, arguments: &::windows_core::PCWSTR, verb: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell")]
impl ::windows_core::RuntimeName for IDDEInitializer {}
#[cfg(feature = "Win32_UI_Shell")]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: isize>() -> IDDEInitializer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows_core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows_core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows_core::PCWSTR, targetfile: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, verb: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&fileextensionorprotocol), ::core::mem::transmute_copy(&method), ::core::mem::transmute(&currentdirectory), ::windows_core::from_raw_borrowed(&exectarget), ::windows_core::from_raw_borrowed(&site), ::core::mem::transmute(&application), ::core::mem::transmute(&targetfile), ::core::mem::transmute(&arguments), ::core::mem::transmute(&verb)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDDEInitializer as ::windows_core::ComInterface>::IID
    }
}
