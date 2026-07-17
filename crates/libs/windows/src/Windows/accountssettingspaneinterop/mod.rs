windows_core::imp::define_interface!(IAccountsSettingsPaneInterop, IAccountsSettingsPaneInterop_Vtbl, 0xd3ee12ad_3865_4362_9746_b75a682df0e6);
windows_core::imp::interface_hierarchy!(IAccountsSettingsPaneInterop, windows_core::IUnknown, windows_core::IInspectable);
impl IAccountsSettingsPaneInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetForWindow<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ShowManageAccountsForWindowAsync<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowManageAccountsForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ShowAddAccountForWindowAsync<T>(&self, appwindow: super::windef::HWND) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ShowAddAccountForWindowAsync)(windows_core::Interface::as_raw(self), appwindow, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
    #[cfg(feature = "windef")]
    pub ShowManageAccountsForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ShowManageAccountsForWindowAsync: usize,
    #[cfg(feature = "windef")]
    pub ShowAddAccountForWindowAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ShowAddAccountForWindowAsync: usize,
}
#[cfg(feature = "windef")]
pub trait IAccountsSettingsPaneInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowManageAccountsForWindowAsync(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ShowAddAccountForWindowAsync(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAccountsSettingsPaneInterop_Vtbl {
    pub const fn new<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, accountssettingspane: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&accountssettingspane)).into()
            }
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::ShowManageAccountsForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
            }
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Identity: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, asyncaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccountsSettingsPaneInterop_Impl::ShowAddAccountForWindowAsync(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&asyncaction)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccountsSettingsPaneInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Identity, OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAccountsSettingsPaneInterop {}
