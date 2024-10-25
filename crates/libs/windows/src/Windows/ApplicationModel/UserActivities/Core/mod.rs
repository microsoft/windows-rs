windows_core::imp::define_interface!(ICoreUserActivityManagerStatics, ICoreUserActivityManagerStatics_Vtbl, 0xca3adb02_a4be_4d4d_bfa8_6795f4264efb);
impl windows_core::RuntimeType for ICoreUserActivityManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ICoreUserActivityManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUserActivitySessionInBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeleteUserActivitySessionsInTimeRangeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, super::super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct CoreUserActivityManager;
impl CoreUserActivityManager {}
impl windows_core::RuntimeName for CoreUserActivityManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.Core.CoreUserActivityManager";
}
