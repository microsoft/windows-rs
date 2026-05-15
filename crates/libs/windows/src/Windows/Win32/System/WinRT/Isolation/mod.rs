windows_core::imp::define_interface!(IIsolatedEnvironmentInterop, IIsolatedEnvironmentInterop_Vtbl, 0x85713c2e_8e62_46c5_8de2_c647e1d54636);
windows_core::imp::interface_hierarchy!(IIsolatedEnvironmentInterop, windows_core::IUnknown);
impl IIsolatedEnvironmentInterop {
    pub unsafe fn GetHostHwndInterop(&self, containerhwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND> {
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
    pub GetHostHwndInterop: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IIsolatedEnvironmentInterop_Impl {
    fn GetHostHwndInterop(&self, containerhwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl IIsolatedEnvironmentInterop_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> Self
    where
        <Identity as windows_core::IUnknownImpl>::Impl: IIsolatedEnvironmentInterop_Impl,
    {
        unsafe extern "system" fn GetHostHwndInterop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            <Identity as windows_core::IUnknownImpl>::Impl: IIsolatedEnvironmentInterop_Impl,
        {
            unsafe {
                let this__outer__: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let this: &<Identity as windows_core::IUnknownImpl>::Impl = <Identity as windows_core::IUnknownImpl>::get_impl(this__outer__);
                match IIsolatedEnvironmentInterop_Impl::GetHostHwndInterop(this, core::mem::transmute_copy(&containerhwnd)) {
                    Ok(ok__) => {
                        hosthwnd.write(core::mem::transmute(ok__));
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
impl windows_core::RuntimeName for IIsolatedEnvironmentInterop {}
