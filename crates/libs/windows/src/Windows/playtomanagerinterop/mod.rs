windows_core::imp::define_interface!(IPlayToManagerInterop, IPlayToManagerInterop_Vtbl, 0x24394699_1f2c_4eb3_8cd7_0ec1da42a540);
windows_core::imp::interface_hierarchy!(IPlayToManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IPlayToManagerInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ShowPlayToUIForWindow(&self, appwindow: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowPlayToUIForWindow)(windows_core::Interface::as_raw(self), appwindow) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
    #[cfg(feature = "windef")]
    pub ShowPlayToUIForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ShowPlayToUIForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IPlayToManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowPlayToUIForWindow(&self, appwindow: super::windef::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IPlayToManagerInterop_Vtbl {
    pub const fn new<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, playtomanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayToManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&playtomanager)).into()
            }
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Identity: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPlayToManagerInterop_Impl::ShowPlayToUIForWindow(this, core::mem::transmute_copy(&appwindow)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPlayToManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPlayToManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IPlayToManagerInterop {}
