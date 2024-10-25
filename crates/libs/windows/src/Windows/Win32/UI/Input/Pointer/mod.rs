pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 9i32;
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 10i32;
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 1i32;
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 2i32;
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 7i32;
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 8i32;
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = 0i32;
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 3i32;
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 4i32;
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = 5i32;
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = 6i32;
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = 32768u32;
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = 2097152u32;
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = 16384u32;
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = 65536u32;
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = 256u32;
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = 16u32;
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = 128u32;
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = 4194304u32;
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = 1048576u32;
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = 4u32;
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = 2u32;
pub const POINTER_FLAG_NEW: POINTER_FLAGS = 1u32;
pub const POINTER_FLAG_NONE: POINTER_FLAGS = 0u32;
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = 8192u32;
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = 32u32;
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = 64u32;
pub const POINTER_FLAG_UP: POINTER_FLAGS = 262144u32;
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = 131072u32;
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = 524288u32;
pub const TOUCH_FEEDBACK_DEFAULT: TOUCH_FEEDBACK_MODE = 1u32;
pub const TOUCH_FEEDBACK_INDIRECT: TOUCH_FEEDBACK_MODE = 2u32;
pub const TOUCH_FEEDBACK_NONE: TOUCH_FEEDBACK_MODE = 3u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct POINTER_BUTTON_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for POINTER_BUTTON_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct POINTER_FLAGS(pub u32);
impl windows_core::TypeKind for POINTER_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TOUCH_FEEDBACK_MODE(pub u32);
impl windows_core::TypeKind for TOUCH_FEEDBACK_MODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
impl Default for INPUT_INJECTION_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_INJECTION_VALUE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [f32; 16],
}
impl Default for INPUT_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INPUT_TRANSFORM_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl Default for INPUT_TRANSFORM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_INFO {
    pub pointerType: super::super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: super::super::super::Foundation::HANDLE,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptPixelLocation: super::super::super::Foundation::POINT,
    pub ptHimetricLocation: super::super::super::Foundation::POINT,
    pub ptPixelLocationRaw: super::super::super::Foundation::POINT,
    pub ptHimetricLocationRaw: super::super::super::Foundation::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_PEN_INFO {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: u32,
    pub penMask: u32,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_PEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_PEN_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: super::super::super::Foundation::RECT,
    pub rcContactRaw: super::super::super::Foundation::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_TOUCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_TOUCH_INFO {
    type TypeKind = windows_core::CloneType;
}
