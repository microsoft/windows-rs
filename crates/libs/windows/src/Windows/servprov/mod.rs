windows_core::imp::define_interface!(IServiceProvider, IServiceProvider_Vtbl, 0x6d5140c1_7436_11ce_8034_00aa006009fa);
windows_core::imp::interface_hierarchy!(IServiceProvider, windows_core::IUnknown);
impl IServiceProvider {
    pub unsafe fn QueryService<T>(&self, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).QueryService)(windows_core::Interface::as_raw(self), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IServiceProvider_Impl: windows_core::IUnknownImpl {
    fn QueryService(&self, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IServiceProvider_Vtbl {
    pub const fn new<Identity: IServiceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryService<Identity: IServiceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IServiceProvider_Impl::QueryService(this, core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryService: QueryService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IServiceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IServiceProvider {}
