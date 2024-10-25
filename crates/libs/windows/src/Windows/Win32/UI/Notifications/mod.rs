#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NOTIFICATION_USER_INPUT_DATA {
    pub Key: windows_core::PCWSTR,
    pub Value: windows_core::PCWSTR,
}
impl Default for NOTIFICATION_USER_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NOTIFICATION_USER_INPUT_DATA {
    type TypeKind = windows_core::CopyType;
}
