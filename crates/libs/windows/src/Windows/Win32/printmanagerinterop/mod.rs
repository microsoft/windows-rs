windows_core::imp::define_interface!(IPrintManagerInterop, IPrintManagerInterop_Vtbl, 0xc5435a42_8d43_4e7b_a68a_ef311e392087);
windows_core::imp::interface_hierarchy!(IPrintManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrintManagerInterop {
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub unsafe fn ShowPrintUIForWindowAsync<T>(&self, appwindow: super::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "windef")))]
    GetForWindow: usize,
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "windef")))]
    ShowPrintUIForWindowAsync: usize,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub trait IPrintManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::HWND, riid: *const super::IID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::HWND, riid: *const super::IID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl IPrintManagerInterop_Vtbl {
    pub const fn new<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::HWND, riid: *const super::IID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: IPrintManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::HWND, riid: *const super::IID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrintManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl windows_core::RuntimeName for IPrintManagerInterop {}
