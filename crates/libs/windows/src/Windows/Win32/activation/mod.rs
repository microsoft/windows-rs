windows_core::imp::define_interface!(IActivationFactory, IActivationFactory_Vtbl, 0x00000035_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IActivationFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IActivationFactory {
    pub unsafe fn ActivateInstance(&self, instance: *mut Option<windows_core::IInspectable>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute(instance)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ActivateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivationFactory_Impl: windows_core::IUnknownImpl {
    fn ActivateInstance(&self, instance: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: IActivationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateInstance<Identity: IActivationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActivationFactory_Impl::ActivateInstance(this, core::mem::transmute_copy(&instance)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivationFactory, OFFSET>(), ActivateInstance: ActivateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivationFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivationFactory {}
pub type PACTIVATIONFACTORY = IActivationFactory;
