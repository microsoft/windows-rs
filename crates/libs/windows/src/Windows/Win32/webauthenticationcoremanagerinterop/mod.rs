windows_core::imp::define_interface!(IWebAuthenticationCoreManagerInterop, IWebAuthenticationCoreManagerInterop_Vtbl, 0xf4b8e804_811e_4436_b69c_44cb67b72084);
windows_core::imp::interface_hierarchy!(IWebAuthenticationCoreManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IWebAuthenticationCoreManagerInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn RequestTokenForWindowAsync<P1, T>(&self, appwindow: super::HWND, request: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestTokenForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, request.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<P1, P2, T>(&self, appwindow: super::HWND, request: P1, webaccount: P2) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IInspectable>,
        P2: windows_core::Param<windows_core::IInspectable>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).RequestTokenWithWebAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, request.param().abi(), webaccount.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub RequestTokenForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestTokenForWindowAsync: usize,
    #[cfg(feature = "windef")]
    pub RequestTokenWithWebAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RequestTokenWithWebAccountForWindowAsync: usize,
}
#[cfg(feature = "windef")]
pub trait IWebAuthenticationCoreManagerInterop_Impl: windows_core::IUnknownImpl {
    fn RequestTokenForWindowAsync(&self, appwindow: super::HWND, request: windows_core::Ref<windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(&self, appwindow: super::HWND, request: windows_core::Ref<windows_core::IInspectable>, webaccount: windows_core::Ref<windows_core::IInspectable>, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IWebAuthenticationCoreManagerInterop_Vtbl {
    pub const fn new<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RequestTokenForWindowAsync<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::HWND, request: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAuthenticationCoreManagerInterop_Impl::RequestTokenForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&request), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
            }
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Identity: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::HWND, request: *mut core::ffi::c_void, webaccount: *mut core::ffi::c_void, riid: *const windows_core::GUID, asyncinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebAuthenticationCoreManagerInterop_Impl::RequestTokenWithWebAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&request), core::mem::transmute_copy(&webaccount), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncinfo)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationCoreManagerInterop, OFFSET>(),
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Identity, OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWebAuthenticationCoreManagerInterop {}
