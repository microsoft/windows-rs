windows_core::imp::define_interface!(IApplicationProfileStatics, IApplicationProfileStatics_Vtbl, 0xd5008ab4_7e7a_11e1_a7f2_b0a14824019b);
impl windows_core::RuntimeType for IApplicationProfileStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApplicationProfileStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Modes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationProfileModes) -> windows_core::HRESULT,
}
pub struct ApplicationProfile;
impl ApplicationProfile {}
impl windows_core::RuntimeName for ApplicationProfile {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.ApplicationProfile";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationProfileModes(pub u32);
impl ApplicationProfileModes {
    pub const Default: Self = Self(0u32);
    pub const Alternate: Self = Self(1u32);
}
impl windows_core::TypeKind for ApplicationProfileModes {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ApplicationProfileModes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.ApplicationModel.ApplicationProfileModes;u4)");
}
