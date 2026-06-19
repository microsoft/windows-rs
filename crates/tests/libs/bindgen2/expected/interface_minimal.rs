windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0xb20ce735_33d8_5ac3_be2d_4b562284044e
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
