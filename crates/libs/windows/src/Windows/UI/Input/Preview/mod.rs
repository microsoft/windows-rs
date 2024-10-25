#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
windows_core::imp::define_interface!(IInputActivationListenerPreviewStatics, IInputActivationListenerPreviewStatics_Vtbl, 0xf0551ce5_0de6_5be0_a589_f737201a4582);
impl windows_core::RuntimeType for IInputActivationListenerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInputActivationListenerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateForApplicationWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateForApplicationWindow: usize,
}
pub struct InputActivationListenerPreview;
impl InputActivationListenerPreview {}
impl windows_core::RuntimeName for InputActivationListenerPreview {
    const NAME: &'static str = "Windows.UI.Input.Preview.InputActivationListenerPreview";
}
