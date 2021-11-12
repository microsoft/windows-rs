#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct GESTURECONFIG_ID(i32);
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
#[repr(C)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(i32);
#[repr(C)]
pub struct ManipulationProcessor(i32);
#[repr(C)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(i32);
#[repr(C)]
pub struct TOUCHEVENTF_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TOUCHINPUT(i32);
#[repr(C)]
pub struct TOUCHINPUTMASKF_MASK(i32);
#[repr(transparent)]
pub struct _IManipulationEvents(pub *mut ::core::ffi::c_void);
