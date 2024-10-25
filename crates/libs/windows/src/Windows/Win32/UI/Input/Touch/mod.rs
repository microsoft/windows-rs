pub const GID_BEGIN: GESTURECONFIG_ID = 1u32;
pub const GID_END: GESTURECONFIG_ID = 2u32;
pub const GID_PAN: GESTURECONFIG_ID = 4u32;
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = 7u32;
pub const GID_ROLLOVER: GESTURECONFIG_ID = 7u32;
pub const GID_ROTATE: GESTURECONFIG_ID = 5u32;
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = 6u32;
pub const GID_ZOOM: GESTURECONFIG_ID = 3u32;
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = 15i32;
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = 0i32;
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = 8i32;
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = 4i32;
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = 1i32;
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = 2i32;
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = 2u32;
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = 8u32;
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = 1u32;
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = 32u32;
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = 128u32;
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = 64u32;
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = 16u32;
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = 4u32;
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = 4u32;
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = 2u32;
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = 1u32;
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = 1u32;
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = 2u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GESTURECONFIG_ID(pub u32);
impl windows_core::TypeKind for GESTURECONFIG_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
impl windows_core::TypeKind for MANIPULATION_PROCESSOR_MANIPULATIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
impl windows_core::TypeKind for REGISTER_TOUCH_WINDOW_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
impl windows_core::TypeKind for TOUCHEVENTF_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
impl windows_core::TypeKind for TOUCHINPUTMASKF_MASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl Default for GESTURECONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GESTURECONFIG {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GESTUREINFO {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub dwID: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
    pub dwSequenceID: u32,
    pub ullArguments: u64,
    pub cbExtraArgs: u32,
}
impl Default for GESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GESTUREINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
impl Default for GESTURENOTIFYSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GESTURENOTIFYSTRUCT {
    type TypeKind = windows_core::CloneType;
}
pub const InertiaProcessor: windows_core::GUID = windows_core::GUID::from_u128(0xabb27087_4ce0_4e58_a0cb_e24df96814be);
pub const ManipulationProcessor: windows_core::GUID = windows_core::GUID::from_u128(0x597d4fb0_47fd_4aff_89b9_c6cfae8cf08e);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOUCHINPUT {
    pub x: i32,
    pub y: i32,
    pub hSource: super::super::super::Foundation::HANDLE,
    pub dwID: u32,
    pub dwFlags: TOUCHEVENTF_FLAGS,
    pub dwMask: TOUCHINPUTMASKF_MASK,
    pub dwTime: u32,
    pub dwExtraInfo: usize,
    pub cxContact: u32,
    pub cyContact: u32,
}
impl Default for TOUCHINPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TOUCHINPUT {
    type TypeKind = windows_core::CloneType;
}
