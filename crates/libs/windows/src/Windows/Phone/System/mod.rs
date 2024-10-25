#[cfg(feature = "Phone_System_Power")]
pub mod Power;
#[cfg(feature = "Phone_System_Profile")]
pub mod Profile;
#[cfg(feature = "Phone_System_UserProfile")]
pub mod UserProfile;
windows_core::imp::define_interface!(ISystemProtectionStatics, ISystemProtectionStatics_Vtbl, 0x49c36560_97e1_4d99_8bfb_befeaa6ace6d);
impl windows_core::RuntimeType for ISystemProtectionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemProtectionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScreenLocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemProtectionUnlockStatics, ISystemProtectionUnlockStatics_Vtbl, 0x0692fa3f_8f11_4c4b_aa0d_87d7af7b1779);
impl windows_core::RuntimeType for ISystemProtectionUnlockStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISystemProtectionUnlockStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestScreenUnlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct SystemProtection;
impl SystemProtection {}
impl windows_core::RuntimeName for SystemProtection {
    const NAME: &'static str = "Windows.Phone.System.SystemProtection";
}
