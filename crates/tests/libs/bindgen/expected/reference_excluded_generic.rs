windows_core::imp::define_winrt_interface!(
    Interface,
    Interface_Vtbl,
    0x3493bdcc_e2f7_5c3b_b102_f71b267d972f,
    b"Test.Interface"
);
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
