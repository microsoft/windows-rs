windows_core::imp::define_interface!(ISoundLevelBrokerStatics, ISoundLevelBrokerStatics_Vtbl, 0x6a633961_dbed_464c_a09a_33412f5caa3f);
impl windows_core::RuntimeType for ISoundLevelBrokerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISoundLevelBrokerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SoundLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::SoundLevel) -> windows_core::HRESULT,
    pub SoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSoundLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
pub struct SoundLevelBroker;
impl SoundLevelBroker {}
impl windows_core::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
