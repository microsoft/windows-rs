pub trait IIsolatedEnvironmentInteropImpl: Sized {
    fn GetHostHwndInterop();
}
impl ::windows::core::RuntimeName for IIsolatedEnvironmentInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Isolation.IIsolatedEnvironmentInterop";
}
impl IIsolatedEnvironmentInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedEnvironmentInteropImpl, const OFFSET: isize>() -> IIsolatedEnvironmentInteropVtbl {
        unsafe extern "system" fn GetHostHwndInterop<Impl: IIsolatedEnvironmentInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostHwndInterop(&*(&containerhwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&hosthwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIsolatedEnvironmentInterop>, ::windows::core::GetTrustLevel, GetHostHwndInterop::<Impl, OFFSET>)
    }
}
