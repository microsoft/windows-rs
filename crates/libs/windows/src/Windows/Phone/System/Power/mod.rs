windows_core::imp::define_interface!(IPowerManagerStatics, IPowerManagerStatics_Vtbl, 0x25de8fd0_1c5b_11e1_bddb_0800200c9a66);
impl windows_core::RuntimeType for IPowerManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerSavingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerSavingMode) -> windows_core::HRESULT,
    pub PowerSavingModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemovePowerSavingModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerManagerStatics2, IPowerManagerStatics2_Vtbl, 0x596236cf_1918_4551_a466_c51aae373bf8);
impl windows_core::RuntimeType for IPowerManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPowerManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PowerSavingModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
pub struct PowerManager;
impl PowerManager {}
impl windows_core::RuntimeName for PowerManager {
    const NAME: &'static str = "Windows.Phone.System.Power.PowerManager";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PowerSavingMode(pub i32);
impl PowerSavingMode {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
}
impl windows_core::TypeKind for PowerSavingMode {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PowerSavingMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.Power.PowerSavingMode;i4)");
}
