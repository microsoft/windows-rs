windows_core::imp::define_interface!(IShareWindowCommandEventArgsInterop, IShareWindowCommandEventArgsInterop_Vtbl, 0x6571a721_643d_43d4_aca4_6b6f5f30f1ad);
windows_core::imp::interface_hierarchy!(IShareWindowCommandEventArgsInterop, windows_core::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IShareWindowCommandEventArgsInterop_Impl: windows_core::IUnknownImpl {
    fn GetWindow(&self) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(feature = "windef")]
impl IShareWindowCommandEventArgsInterop_Vtbl {
    pub const fn new<Identity: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWindow<Identity: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShareWindowCommandEventArgsInterop_Impl::GetWindow(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgsInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IShareWindowCommandEventArgsInterop {}
windows_core::imp::define_interface!(IShareWindowCommandSourceInterop, IShareWindowCommandSourceInterop_Vtbl, 0x461a191f_8424_43a6_a0fa_3451a22f56ab);
windows_core::imp::interface_hierarchy!(IShareWindowCommandSourceInterop, windows_core::IUnknown);
impl IShareWindowCommandSourceInterop {
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
pub struct IShareWindowCommandSourceInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetForWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IShareWindowCommandSourceInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::windef::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IShareWindowCommandSourceInterop_Vtbl {
    pub const fn new<Identity: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::windef::HWND, riid: *const windows_core::GUID, sharewindowcommandsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShareWindowCommandSourceInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&sharewindowcommandsource)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetForWindow: GetForWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IShareWindowCommandSourceInterop {}
