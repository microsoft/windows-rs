windows_core::imp::define_interface!(ICoreFrameworkInputViewInterop, ICoreFrameworkInputViewInterop_Vtbl, 0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
impl core::ops::Deref for ICoreFrameworkInputViewInterop {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICoreFrameworkInputViewInterop, windows_core::IUnknown, windows_core::IInspectable);
impl ICoreFrameworkInputViewInterop {
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<super::super::super::Foundation::HWND>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        (windows_core::Interface::vtable(self).GetForWindow)(windows_core::Interface::as_raw(self), appwindow.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICoreFrameworkInputViewInterop_Impl: Sized + windows_core::IUnknownImpl {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, coreframeworkinputview: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICoreFrameworkInputViewInterop {}
impl ICoreFrameworkInputViewInterop_Vtbl {
    pub const fn new<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>() -> ICoreFrameworkInputViewInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const windows_core::GUID, coreframeworkinputview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICoreFrameworkInputViewInterop_Impl::GetForWindow(this, core::mem::transmute_copy(&appwindow), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&coreframeworkinputview)).into()
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
