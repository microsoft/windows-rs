pub trait IIsolatedEnvironmentInteropImpl: Sized {
    fn GetHostHwndInterop();
}
impl ::windows::core::RuntimeName for IIsolatedEnvironmentInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Isolation.IIsolatedEnvironmentInterop";
}
impl IIsolatedEnvironmentInteropVtbl {
    pub const fn new<Impl: IIsolatedEnvironmentInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedEnvironmentInteropVtbl {
        unsafe extern "system" fn GetHostHwndInterop<Impl: IIsolatedEnvironmentInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHostHwndInterop(&*(&containerhwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&hosthwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedEnvironmentInterop>, base.5, GetHostHwndInterop::<Impl, OFFSET>)
    }
}
