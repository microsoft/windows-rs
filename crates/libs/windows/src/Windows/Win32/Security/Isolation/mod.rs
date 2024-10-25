pub const WDAG_CLIPBOARD_TAG: windows_core::PCWSTR = windows_core::w!("CrossIsolatedEnvironmentContent");
pub const IsolatedAppLauncher: windows_core::GUID = windows_core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: windows_core::GUID,
}
impl Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IsolatedAppLauncherTelemetryParameters {
    type TypeKind = windows_core::CloneType;
}
