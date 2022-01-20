#[cfg(feature = "Win32_Foundation")]
pub trait ICoreFrameworkInputViewInterop_Impl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl ICoreFrameworkInputViewInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>() -> ICoreFrameworkInputViewInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&coreframeworkinputview)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreFrameworkInputViewInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewInterop as ::windows::core::Interface>::IID
    }
}
