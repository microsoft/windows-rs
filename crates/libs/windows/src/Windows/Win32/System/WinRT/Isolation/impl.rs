#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedEnvironmentInterop_Impl: Sized {
    fn GetHostHwndInterop(&self, containerhwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedEnvironmentInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>() -> IIsolatedEnvironmentInterop_Vtbl {
        unsafe extern "system" fn GetHostHwndInterop<Identity: ::windows::core::IUnknownImpl, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHostHwndInterop(::core::mem::transmute_copy(&containerhwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *hosthwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetHostHwndInterop: GetHostHwndInterop::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsolatedEnvironmentInterop as ::windows::core::Interface>::IID
    }
}
