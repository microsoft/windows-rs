windows_core::imp::define_interface!(ISpatialInteractionManagerInterop, ISpatialInteractionManagerInterop_Vtbl, 0x5c4ee536_6a98_4b86_a170_587013d6fd4b);
windows_core::imp::interface_hierarchy!(ISpatialInteractionManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ISpatialInteractionManagerInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, window: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait ISpatialInteractionManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, window: super::windef::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl ISpatialInteractionManagerInterop_Vtbl {
    pub const fn new<Identity: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::windef::HWND, riid: *const windows_core::GUID, spatialinteractionmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialInteractionManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&spatialinteractionmanager)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISpatialInteractionManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for ISpatialInteractionManagerInterop {}
