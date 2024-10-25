#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[cfg(feature = "System_RemoteDesktop_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IInteractiveSessionStatics, IInteractiveSessionStatics_Vtbl, 0x60884631_dd3a_4576_9c8d_e8027618bdce);
impl windows_core::RuntimeType for IInteractiveSessionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IInteractiveSessionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsRemote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct InteractiveSession;
impl InteractiveSession {}
impl windows_core::RuntimeName for InteractiveSession {
    const NAME: &'static str = "Windows.System.RemoteDesktop.InteractiveSession";
}
