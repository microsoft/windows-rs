#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedEnvironmentInteropImpl: Sized {
    fn GetHostHwndInterop();
}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedEnvironmentInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedEnvironmentInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsolatedEnvironmentInteropVtbl {
        unsafe extern "system" fn GetHostHwndInterop<Impl: IIsolatedEnvironmentInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHostHwndInterop::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsolatedEnvironmentInterop as ::windows::core::Interface>::IID
    }
}
