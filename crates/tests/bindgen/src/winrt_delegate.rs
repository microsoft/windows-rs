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
impl DeferralCompletedHandler {
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct DeferralCompletedHandler_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
