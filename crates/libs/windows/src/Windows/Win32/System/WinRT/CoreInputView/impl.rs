#[cfg(feature = "Win32_Foundation")]
pub trait ICoreFrameworkInputViewInteropImpl: Sized {
    fn GetForWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl ICoreFrameworkInputViewInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreFrameworkInputViewInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ICoreFrameworkInputViewInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewInterop>, ::windows::core::GetTrustLevel, GetForWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewInterop as ::windows::core::Interface>::IID
    }
}
