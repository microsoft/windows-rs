#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub(crate) fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub(crate) fn Method(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub(crate) fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "Test.Class";
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x327dfd65_5684_57ee_85ab_168eff31a838);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IClass");
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
