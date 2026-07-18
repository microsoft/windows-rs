windows_core::imp::define_interface!(ICreatePropBagOnRegKey, ICreatePropBagOnRegKey_Vtbl, 0x8a674b48_1f63_11d3_b64c_00c04f79498e);
windows_core::imp::interface_hierarchy!(ICreatePropBagOnRegKey, windows_core::IUnknown);
impl ICreatePropBagOnRegKey {
    #[cfg(feature = "minwindef")]
    pub unsafe fn Create<P1, T>(&self, hkey: super::HKEY, subkey: P1, uloptions: u32, samdesired: u32) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), hkey, subkey.param().abi(), uloptions, samdesired, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICreatePropBagOnRegKey_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, super::HKEY, windows_core::PCWSTR, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    Create: usize,
}
#[cfg(feature = "minwindef")]
pub trait ICreatePropBagOnRegKey_Impl: windows_core::IUnknownImpl {
    fn Create(&self, hkey: super::HKEY, subkey: &windows_core::PCWSTR, uloptions: u32, samdesired: u32, iid: *const windows_core::GUID, ppbag: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "minwindef")]
impl ICreatePropBagOnRegKey_Vtbl {
    pub const fn new<Identity: ICreatePropBagOnRegKey_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: ICreatePropBagOnRegKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkey: super::HKEY, subkey: windows_core::PCWSTR, uloptions: u32, samdesired: u32, iid: *const windows_core::GUID, ppbag: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICreatePropBagOnRegKey_Impl::Create(this, core::mem::transmute_copy(&hkey), core::mem::transmute(&subkey), core::mem::transmute_copy(&uloptions), core::mem::transmute_copy(&samdesired), core::mem::transmute_copy(&iid), core::mem::transmute_copy(&ppbag)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreatePropBagOnRegKey as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for ICreatePropBagOnRegKey {}
