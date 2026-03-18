windows_core::imp::define_interface!(ICoreFrameworkInputViewInterop, ICoreFrameworkInputViewInterop_Vtbl, 0xce57e61e_6089_55fc_b990_c9560708c926);
windows_core::imp::interface_hierarchy!(ICoreFrameworkInputViewInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreFrameworkInputViewInterop {
    pub unsafe fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *mut windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow, riid as _, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *mut windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreFrameworkInputViewInterop_Impl: windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *mut windows_core::GUID) -> windows_core::Result<*mut core::ffi::c_void>;
}
impl ICoreFrameworkInputViewInterop_Vtbl {
    pub const fn new<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindow<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *mut windows_core::GUID, coreframeworkinputview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreFrameworkInputViewInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid)) {
                    Ok(ok__) => {
                        coreframeworkinputview.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreFrameworkInputViewInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreFrameworkInputViewInterop as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICoreFrameworkInputViewInterop {}
