pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 0i32;
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 1i32;
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 2i32;
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = 0i32;
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = 1i32;
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = 4i32;
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = 2i32;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = -3i32 as _;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4i32 as _;
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = -2i32 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1i32 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = -5i32 as _;
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = -1i32;
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = 2i32;
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = 1i32;
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = 0i32;
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = 0i32;
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = -1i32;
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = 1i32;
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = 1i32;
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = 0i32;
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = 0i32;
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = 2i32;
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = 0i32;
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2i32;
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub i32);
impl windows_core::TypeKind for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub i32);
impl windows_core::TypeKind for DIALOG_DPI_CHANGE_BEHAVIORS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DPI_AWARENESS(pub i32);
impl windows_core::TypeKind for DPI_AWARENESS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
impl windows_core::TypeKind for DPI_HOSTING_BEHAVIOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MONITOR_DPI_TYPE(pub i32);
impl windows_core::TypeKind for MONITOR_DPI_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
impl windows_core::TypeKind for PROCESS_DPI_AWARENESS {
    type TypeKind = windows_core::CopyType;
}
