#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    DeferralCompletedHandler,
    DeferralCompletedHandler_Vtbl,
    0xed32a372_f3c8_4faa_9cfb_470148da3888
);
windows_core::imp::interface_hierarchy!(
    DeferralCompletedHandler,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for DeferralCompletedHandler {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl DeferralCompletedHandler {
    pub fn Invoke(&self) {}
}
impl windows_core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DeferralCompletedHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
