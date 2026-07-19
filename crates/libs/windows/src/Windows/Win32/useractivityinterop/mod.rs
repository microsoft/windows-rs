windows_core::imp::define_interface!(IUserActivityInterop, IUserActivityInterop_Vtbl, 0x1ade314d_0e0a_40d9_824c_9a088a50059f);
windows_core::imp::interface_hierarchy!(IUserActivityInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn CreateSessionForWindow<T>(&self, window: super::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateSessionForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub CreateSessionForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateSessionForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IUserActivityInterop_Impl: windows_core::IUnknownImpl {
    fn CreateSessionForWindow(&self, window: super::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IUserActivityInterop_Vtbl {
    pub const fn new<Identity: IUserActivityInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSessionForWindow<Identity: IUserActivityInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivityInterop_Impl::CreateSessionForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityInterop, OFFSET>(),
            CreateSessionForWindow: CreateSessionForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUserActivityInterop {}
windows_core::imp::define_interface!(IUserActivityRequestManagerInterop, IUserActivityRequestManagerInterop_Vtbl, 0xdd69f876_9699_4715_9095_e37ea30dfa1b);
windows_core::imp::interface_hierarchy!(IUserActivityRequestManagerInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivityRequestManagerInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, window: super::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IUserActivityRequestManagerInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, window: super::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IUserActivityRequestManagerInterop_Vtbl {
    pub const fn new<Identity: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, iid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivityRequestManagerInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivityRequestManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IUserActivityRequestManagerInterop {}
windows_core::imp::define_interface!(IUserActivitySourceHostInterop, IUserActivitySourceHostInterop_Vtbl, 0xc15df8bc_8844_487a_b85b_7578e0f61419);
windows_core::imp::interface_hierarchy!(IUserActivitySourceHostInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IUserActivitySourceHostInterop {
    pub unsafe fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetActivitySourceHost)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(activitysourcehost)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetActivitySourceHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUserActivitySourceHostInterop_Impl: windows_core::IUnknownImpl {
    fn SetActivitySourceHost(&self, activitysourcehost: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl IUserActivitySourceHostInterop_Vtbl {
    pub const fn new<Identity: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetActivitySourceHost<Identity: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activitysourcehost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserActivitySourceHostInterop_Impl::SetActivitySourceHost(this, core::mem::transmute(&activitysourcehost)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUserActivitySourceHostInterop, OFFSET>(),
            SetActivitySourceHost: SetActivitySourceHost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserActivitySourceHostInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserActivitySourceHostInterop {}
