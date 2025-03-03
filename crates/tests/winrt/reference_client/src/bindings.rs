#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IReference,
    IReference_Vtbl,
    0xdb426a55_0a8f_5cd8_8618_2926436cb7e6
);
impl windows_core::RuntimeType for IReference {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Reference,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Reference, windows::Foundation::IStringable);
impl Reference {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            Reference,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method<P0>(&self, stringable: P0) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<windows::Foundation::IStringable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Method)(
                windows_core::Interface::as_raw(this),
                stringable.param().abi(),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for Reference {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IReference>();
}
unsafe impl windows_core::Interface for Reference {
    type Vtable = <IReference as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Reference {
    const NAME: &'static str = "test_reference.Reference";
}
unsafe impl Send for Reference {}
unsafe impl Sync for Reference {}
