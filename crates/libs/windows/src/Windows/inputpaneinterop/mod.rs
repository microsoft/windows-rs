windows_core::imp::define_interface!(IInputPaneInterop, IInputPaneInterop_Vtbl, 0x75cf2c57_9195_4931_8332_f0b409e916af);
windows_core::imp::interface_hierarchy!(IInputPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IInputPaneInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IInputPaneInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IInputPaneInterop_Vtbl {
    pub const fn new<Identity: IInputPaneInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IInputPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, inputpane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&inputpane)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPaneInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPaneInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IInputPaneInterop {}
