pub trait IRadialControllerConfigurationInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn GetForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRadialControllerConfigurationInterop {}
impl IRadialControllerConfigurationInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>() -> IRadialControllerConfigurationInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: IRadialControllerConfigurationInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadialControllerConfigurationInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerConfigurationInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerConfigurationInterop as windows_core::Interface>::IID
    }
}
pub trait IRadialControllerIndependentInputSourceInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRadialControllerIndependentInputSourceInterop {}
impl IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>() -> IRadialControllerIndependentInputSourceInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: IRadialControllerIndependentInputSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadialControllerIndependentInputSourceInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerIndependentInputSourceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSourceInterop as windows_core::Interface>::IID
    }
}
pub trait IRadialControllerInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateForWindow(&self, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRadialControllerInterop {}
impl IRadialControllerInterop_Vtbl {
    pub const fn new<Identity: IRadialControllerInterop_Impl, const OFFSET: isize>() -> IRadialControllerInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: IRadialControllerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRadialControllerInterop_Impl::CreateForWindow(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRadialControllerInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRadialControllerInterop as windows_core::Interface>::IID
    }
}
