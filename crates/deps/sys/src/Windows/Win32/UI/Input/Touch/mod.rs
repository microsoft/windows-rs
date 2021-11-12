#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseGestureInfoHandle(hgestureinfo: HGESTUREINFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseTouchInputHandle(htouchinput: HTOUCHINPUT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, dwflags: u32, pcids: *const u32, pgestureconfig: *mut GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureExtraArgs(hgestureinfo: HGESTUREINFO, cbextraargs: u32, pextraargs: *mut u8) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGestureInfo(hgestureinfo: HGESTUREINFO, pgestureinfo: *mut GESTUREINFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTouchInputInfo(htouchinput: HTOUCHINPUT, cinputs: u32, pinputs: *mut TOUCHINPUT, cbsize: i32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTouchWindow(hwnd: super::super::super::Foundation::HWND, pulflags: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchWindow(hwnd: super::super::super::Foundation::HWND, ulflags: REGISTER_TOUCH_WINDOW_FLAGS) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGestureConfig(hwnd: super::super::super::Foundation::HWND, dwreserved: u32, cids: u32, pgestureconfig: *const GESTURECONFIG, cbsize: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterTouchWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct GESTURECONFIG(i32);
#[repr(transparent)]
pub struct GESTURECONFIG_ID(pub u32);
pub const GID_BEGIN: GESTURECONFIG_ID = GESTURECONFIG_ID(1u32);
pub const GID_END: GESTURECONFIG_ID = GESTURECONFIG_ID(2u32);
pub const GID_ZOOM: GESTURECONFIG_ID = GESTURECONFIG_ID(3u32);
pub const GID_PAN: GESTURECONFIG_ID = GESTURECONFIG_ID(4u32);
pub const GID_ROTATE: GESTURECONFIG_ID = GESTURECONFIG_ID(5u32);
pub const GID_TWOFINGERTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(6u32);
pub const GID_PRESSANDTAP: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
pub const GID_ROLLOVER: GESTURECONFIG_ID = GESTURECONFIG_ID(7u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GESTUREINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GESTURENOTIFYSTRUCT(i32);
#[repr(C)]
pub struct HGESTUREINFO(i32);
#[repr(C)]
pub struct HTOUCHINPUT(i32);
#[repr(transparent)]
pub struct IInertiaProcessor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationProcessor(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct InertiaProcessor(i32);
#[repr(transparent)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
#[repr(C)]
pub struct ManipulationProcessor(i32);
#[repr(transparent)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
#[repr(transparent)]
pub struct TOUCHEVENTF_FLAGS(pub u32);
pub const TOUCHEVENTF_MOVE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(1u32);
pub const TOUCHEVENTF_DOWN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(2u32);
pub const TOUCHEVENTF_UP: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(4u32);
pub const TOUCHEVENTF_INRANGE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(8u32);
pub const TOUCHEVENTF_PRIMARY: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(16u32);
pub const TOUCHEVENTF_NOCOALESCE: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(32u32);
pub const TOUCHEVENTF_PEN: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(64u32);
pub const TOUCHEVENTF_PALM: TOUCHEVENTF_FLAGS = TOUCHEVENTF_FLAGS(128u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TOUCHINPUT(i32);
#[repr(transparent)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
#[repr(transparent)]
pub struct _IManipulationEvents(pub *mut ::core::ffi::c_void);
