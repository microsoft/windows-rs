windows_core::imp::define_interface!(IUserConsentVerifierInterop, IUserConsentVerifierInterop_Vtbl, 0x39e050c3_4e74_441a_8dc0_b81104df949c);
windows_core::imp::interface_hierarchy!(IUserConsentVerifierInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserConsentVerifierInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn RequestVerificationForWindowAsync<T>(&self, appwindow: super::HWND, message: &windows_core::HSTRING) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestVerificationForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, core::mem::transmute_copy(message), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub RequestVerificationForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestVerificationForWindowAsync: usize,
}
#[cfg(feature = "windef")]
pub trait IUserConsentVerifierInterop_Impl: windows_core::IUnknownImpl {
    fn RequestVerificationForWindowAsync(&self, appwindow: super::HWND, message: &windows_core::HSTRING, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IUserConsentVerifierInterop_Vtbl {
    pub const fn new<Identity: IUserConsentVerifierInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Identity: IUserConsentVerifierInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::HWND, message: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserConsentVerifierInterop_Impl::RequestVerificationForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute(&message), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserConsentVerifierInterop, OFFSET>(),
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserConsentVerifierInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUserConsentVerifierInterop {}
