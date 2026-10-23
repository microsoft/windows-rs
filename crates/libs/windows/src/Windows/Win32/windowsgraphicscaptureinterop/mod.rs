windows_core::imp::define_interface!(IGraphicsCaptureItemInterop, IGraphicsCaptureItemInterop_Vtbl, 0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
windows_core::imp::interface_hierarchy!(IGraphicsCaptureItemInterop, windows_core::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub unsafe fn CreateForWindow(&self, window: super::HWND, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateForWindow)(windows_core::Interface::as_raw(self), window as _, riid as _, result as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub unsafe fn CreateForMonitor(&self, monitor: super::HMONITOR, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateForMonitor)(windows_core::Interface::as_raw(self), monitor as _, riid as _, result as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub CreateForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "windef")))]
    CreateForWindow: usize,
    #[cfg(all(feature = "guiddef", feature = "windef"))]
    pub CreateForMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, super::HMONITOR, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "windef")))]
    CreateForMonitor: usize,
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
pub trait IGraphicsCaptureItemInterop_Impl: windows_core::IUnknownImpl {
    fn CreateForWindow(&self, window: super::HWND, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateForMonitor(&self, monitor: super::HMONITOR, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl IGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForWindow<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsCaptureItemInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn CreateForMonitor<Identity: IGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: super::HMONITOR, riid: *const super::IID, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGraphicsCaptureItemInterop_Impl::CreateForMonitor(this, core::mem::transmute_copy(&monitor), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&result)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
            CreateForMonitor: CreateForMonitor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGraphicsCaptureItemInterop as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
impl windows_core::RuntimeName for IGraphicsCaptureItemInterop {}
windows_core::imp::define_interface!(IMonitorGraphicsCaptureItemInterop, IMonitorGraphicsCaptureItemInterop_Vtbl, 0x33274d14_a076_4048_8416_747e9b04db7b);
windows_core::imp::interface_hierarchy!(IMonitorGraphicsCaptureItemInterop, windows_core::IUnknown);
impl IMonitorGraphicsCaptureItemInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetMonitor(&self, monitor: *mut super::HMONITOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMonitor)(windows_core::Interface::as_raw(self), monitor as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMonitorGraphicsCaptureItemInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HMONITOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetMonitor: usize,
}
#[cfg(feature = "windef")]
pub trait IMonitorGraphicsCaptureItemInterop_Impl: windows_core::IUnknownImpl {
    fn GetMonitor(&self, monitor: *mut super::HMONITOR) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IMonitorGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: IMonitorGraphicsCaptureItemInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMonitor<Identity: IMonitorGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: *mut super::HMONITOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonitorGraphicsCaptureItemInterop_Impl::GetMonitor(this, core::mem::transmute_copy(&monitor)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMonitor: GetMonitor::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonitorGraphicsCaptureItemInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IMonitorGraphicsCaptureItemInterop {}
windows_core::imp::define_interface!(IWindowGraphicsCaptureItemInterop, IWindowGraphicsCaptureItemInterop_Vtbl, 0x38e4c48b_94e6_4c44_9cfa_968193316c0c);
windows_core::imp::interface_hierarchy!(IWindowGraphicsCaptureItemInterop, windows_core::IUnknown);
impl IWindowGraphicsCaptureItemInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self, window: *mut super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), window as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowGraphicsCaptureItemInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IWindowGraphicsCaptureItemInterop_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self, window: *mut super::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IWindowGraphicsCaptureItemInterop_Vtbl {
    pub const fn new<Identity: IWindowGraphicsCaptureItemInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IWindowGraphicsCaptureItemInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowGraphicsCaptureItemInterop_Impl::GetWindow(this, core::mem::transmute_copy(&window)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowGraphicsCaptureItemInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IWindowGraphicsCaptureItemInterop {}
