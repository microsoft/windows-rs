windows_core::imp::define_interface!(IIsolatedEnvironmentInterop, IIsolatedEnvironmentInterop_Vtbl, 0x85713c2e_8e62_46c5_8de2_c647e1d54636);
windows_core::imp::interface_hierarchy!(IIsolatedEnvironmentInterop, windows_core::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[cfg(feature = "windef")]
    pub unsafe fn GetHostHwndInterop(&self, containerhwnd: super::windef::HWND) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostHwndInterop)(windows_core::Interface::as_raw(self), containerhwnd, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetHostHwndInterop: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetHostHwndInterop: usize,
}
#[cfg(feature = "windef")]
pub trait IIsolatedEnvironmentInterop_Impl: windows_core::IUnknownImpl {
    fn GetHostHwndInterop(&self, containerhwnd: super::windef::HWND) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(feature = "windef")]
impl IIsolatedEnvironmentInterop_Vtbl {
    pub const fn new<Identity: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetHostHwndInterop<Identity: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, containerhwnd: super::windef::HWND, hosthwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsolatedEnvironmentInterop_Impl::GetHostHwndInterop(this, core::mem::transmute_copy(&containerhwnd)) {
                    Ok(ok__) => {
                        hosthwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHostHwndInterop: GetHostHwndInterop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedEnvironmentInterop as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IIsolatedEnvironmentInterop {}
