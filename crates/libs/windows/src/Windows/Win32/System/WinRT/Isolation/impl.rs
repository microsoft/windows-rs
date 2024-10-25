pub trait IIsolatedEnvironmentInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn GetHostHwndInterop(&self, containerhwnd: super::super::super::Foundation::HWND) -> windows_core::Result<super::super::super::Foundation::HWND>;
}
impl windows_core::RuntimeName for IIsolatedEnvironmentInterop {}
impl IIsolatedEnvironmentInterop_Vtbl {
    pub const fn new<Identity: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>() -> IIsolatedEnvironmentInterop_Vtbl {
        unsafe extern "system" fn GetHostHwndInterop<Identity: IIsolatedEnvironmentInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIsolatedEnvironmentInterop_Impl::GetHostHwndInterop(this, core::mem::transmute_copy(&containerhwnd)) {
                Ok(ok__) => {
                    hosthwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetHostHwndInterop: GetHostHwndInterop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsolatedEnvironmentInterop as windows_core::Interface>::IID
    }
}
