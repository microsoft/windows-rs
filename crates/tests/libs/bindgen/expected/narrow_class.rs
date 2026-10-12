windows_core::imp::define_interface!(
    IWidget,
    IWidget_Vtbl,
    0x05cda87c_844a_5686_8429_f335c8ee6276
);
impl windows_core::RuntimeType for IWidget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IWidget");
}
windows_core::imp::interface_hierarchy!(
    IWidget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for IWidget {
    const NAME: &'static str = "Test.IWidget";
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Widget,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidget
);
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
