#[doc = "*Required features: `\"Win32_System_WinRT_Shell\"`, `\"Win32_UI_Shell\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_Shell")]
pub trait IDDEInitializer_Impl: Sized {
    fn Initialize(&self, fileextensionorprotocol: &::windows::core::PCWSTR, method: CreateProcessMethod, currentdirectory: &::windows::core::PCWSTR, exectarget: ::core::option::Option<&super::super::super::UI::Shell::IShellItem>, site: ::core::option::Option<&::windows::core::IUnknown>, application: &::windows::core::PCWSTR, targetfile: &::windows::core::PCWSTR, arguments: &::windows::core::PCWSTR, verb: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell")]
impl ::windows::core::RuntimeName for IDDEInitializer {}
#[cfg(feature = "Win32_UI_Shell")]
impl IDDEInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: isize>() -> IDDEInitializer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDDEInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: ::windows::core::PCWSTR, method: CreateProcessMethod, currentdirectory: ::windows::core::PCWSTR, exectarget: *mut ::core::ffi::c_void, site: *mut ::core::ffi::c_void, application: ::windows::core::PCWSTR, targetfile: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, verb: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&fileextensionorprotocol), ::core::mem::transmute_copy(&method), ::core::mem::transmute(&currentdirectory), ::windows::core::from_raw_borrowed(&exectarget), ::windows::core::from_raw_borrowed(&site), ::core::mem::transmute(&application), ::core::mem::transmute(&targetfile), ::core::mem::transmute(&arguments), ::core::mem::transmute(&verb)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDDEInitializer as ::windows::core::ComInterface>::IID
    }
}
