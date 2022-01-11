#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
pub trait IDDEInitializerImpl: Sized {
    fn Initialize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl IDDEInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDDEInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDDEInitializerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDDEInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: super::super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::super::Foundation::PWSTR, exectarget: ::windows::core::RawPtr, site: *mut ::core::ffi::c_void, application: super::super::super::Foundation::PWSTR, targetfile: super::super::super::Foundation::PWSTR, arguments: super::super::super::Foundation::PWSTR, verb: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDDEInitializer as ::windows::core::Interface>::IID
    }
}
