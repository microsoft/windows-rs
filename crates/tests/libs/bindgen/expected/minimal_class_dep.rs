windows_core::imp::define_interface!(
    Factory,
    Factory_Vtbl,
    0x8722b2b2_1d05_5551_87c8_d46311b7873d
);
impl windows_core::RuntimeType for Factory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Factory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for Factory {
    const NAME: &'static str = "Test.Factory";
}
#[repr(C)]
pub struct Factory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWidget,
    IWidget_Vtbl,
    0x62d365dd_b506_534c_a235_93129deef881
);
impl windows_core::RuntimeType for IWidget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Widget, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
}
impl core::ops::Deref for Widget {
    type Target = IWidget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
