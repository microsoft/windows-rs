windows_core::imp::define_interface!(IDisplayInformationStaticsInterop, IDisplayInformationStaticsInterop_Vtbl, 0x7449121c_382b_4705_8da7_a795ba482013);
windows_core::imp::interface_hierarchy!(IDisplayInformationStaticsInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IDisplayInformationStaticsInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, window: super::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetForMonitor<T>(&self, monitor: super::HMONITOR) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForMonitor)(windows_core::Interface::as_raw(self), monitor, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayInformationStaticsInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
    #[cfg(feature = "windef")]
    pub GetForMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMONITOR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForMonitor: usize,
}
#[cfg(feature = "windef")]
pub trait IDisplayInformationStaticsInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, window: super::HWND, riid: *const windows_core::GUID, displayinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetForMonitor(&self, monitor: super::HMONITOR, riid: *const windows_core::GUID, displayinfo: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDisplayInformationStaticsInterop_Vtbl {
    pub const fn new<Identity: IDisplayInformationStaticsInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IDisplayInformationStaticsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, riid: *const windows_core::GUID, displayinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDisplayInformationStaticsInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&displayinfo)).into()
            }
        }
        unsafe extern "system" fn GetForMonitor<Identity: IDisplayInformationStaticsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::HMONITOR, riid: *const windows_core::GUID, displayinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDisplayInformationStaticsInterop_Impl::GetForMonitor(this, core::mem::transmute_copy(&monitor), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&displayinfo)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDisplayInformationStaticsInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            GetForMonitor: GetForMonitor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDisplayInformationStaticsInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDisplayInformationStaticsInterop {}
