#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl core::ops::Deref for Class {
    type Target = IClass;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "Test.Class";
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x327dfd65_5684_57ee_85ab_168eff31a838);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
