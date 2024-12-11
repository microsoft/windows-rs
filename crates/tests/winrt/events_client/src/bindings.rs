#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
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
    pub fn Signal(&self, value: i32) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Signal)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Event<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<windows::Foundation::TypedEventHandler<Class, i32>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Event)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn RemoveEvent(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveEvent)(
                windows_core::Interface::as_raw(this),
                token,
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
    const NAME: &'static str = "test_events.Class";
}
unsafe impl Send for Class {}
unsafe impl Sync for Class {}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0xad3fd5e5_03a8_5c1e_ab60_efa5e9379730);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Signal:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Event: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveEvent:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
