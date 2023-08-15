#[doc = "*Required features: `\"Win32_System_WinRT_Isolation\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IIsolatedEnvironmentInterop_Impl: Sized {
    fn GetHostHwndInterop(&self, containerhwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IIsolatedEnvironmentInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IIsolatedEnvironmentInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>() -> IIsolatedEnvironmentInterop_Vtbl {
        unsafe extern "system" fn GetHostHwndInterop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHostHwndInterop(::core::mem::transmute_copy(&containerhwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hosthwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHostHwndInterop: GetHostHwndInterop::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIsolatedEnvironmentInterop as ::windows_core::ComInterface>::IID
    }
}
