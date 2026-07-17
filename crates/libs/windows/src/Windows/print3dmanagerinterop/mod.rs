windows_core::imp::define_interface!(IPrinting3DManagerInterop, IPrinting3DManagerInterop_Vtbl, 0x9ca31010_1484_4587_b26b_dddf9f9caecd);
windows_core::imp::interface_hierarchy!(IPrinting3DManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPrinting3DManagerInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ShowPrintUIForWindowAsync<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowPrintUIForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
    #[cfg(feature = "windef")]
    pub ShowPrintUIForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ShowPrintUIForWindowAsync: usize,
}
#[cfg(feature = "windef")]
pub trait IPrinting3DManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPrintUIForWindowAsync(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IPrinting3DManagerInterop_Vtbl {
    pub const fn new<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, printmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinting3DManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&printmanager)).into()
            }
        }
        unsafe extern "system" fn ShowPrintUIForWindowAsync<Identity: IPrinting3DManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncoperation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrinting3DManagerInterop_Impl::ShowPrintUIForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncoperation)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPrinting3DManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPrintUIForWindowAsync: ShowPrintUIForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrinting3DManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IPrinting3DManagerInterop {}
