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
pub struct GESTURECONFIG {
    pub dwID: GESTURECONFIG_ID,
    pub dwWant: u32,
    pub dwBlock: u32,
}
impl ::core::marker::Copy for GESTURECONFIG {}
impl ::core::clone::Clone for GESTURECONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for GESTURECONFIG_ID {}
impl ::core::clone::Clone for GESTURECONFIG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTUREINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GESTURENOTIFYSTRUCT {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptsLocation: super::super::super::Foundation::POINTS,
    pub dwInstanceID: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GESTURENOTIFYSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GESTURENOTIFYSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HGESTUREINFO(pub isize);
impl ::core::marker::Copy for HGESTUREINFO {}
impl ::core::clone::Clone for HGESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HTOUCHINPUT(pub isize);
impl ::core::marker::Copy for HTOUCHINPUT {}
impl ::core::clone::Clone for HTOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInertiaProcessor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManipulationProcessor(pub *mut ::core::ffi::c_void);
pub const InertiaProcessor: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2880598151,
    data2: 19680,
    data3: 20056,
    data4: [160, 203, 226, 77, 249, 104, 20, 190],
};
#[repr(transparent)]
pub struct MANIPULATION_PROCESSOR_MANIPULATIONS(pub i32);
pub const MANIPULATION_NONE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(0i32);
pub const MANIPULATION_TRANSLATE_X: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(1i32);
pub const MANIPULATION_TRANSLATE_Y: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(2i32);
pub const MANIPULATION_SCALE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(4i32);
pub const MANIPULATION_ROTATE: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(8i32);
pub const MANIPULATION_ALL: MANIPULATION_PROCESSOR_MANIPULATIONS = MANIPULATION_PROCESSOR_MANIPULATIONS(15i32);
impl ::core::marker::Copy for MANIPULATION_PROCESSOR_MANIPULATIONS {}
impl ::core::clone::Clone for MANIPULATION_PROCESSOR_MANIPULATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ManipulationProcessor: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1501384624,
    data2: 18429,
    data3: 19199,
    data4: [137, 185, 198, 207, 174, 140, 240, 142],
};
#[repr(transparent)]
pub struct REGISTER_TOUCH_WINDOW_FLAGS(pub u32);
pub const TWF_FINETOUCH: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(1u32);
pub const TWF_WANTPALM: REGISTER_TOUCH_WINDOW_FLAGS = REGISTER_TOUCH_WINDOW_FLAGS(2u32);
impl ::core::marker::Copy for REGISTER_TOUCH_WINDOW_FLAGS {}
impl ::core::clone::Clone for REGISTER_TOUCH_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for TOUCHEVENTF_FLAGS {}
impl ::core::clone::Clone for TOUCHEVENTF_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOUCHINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOUCHINPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TOUCHINPUTMASKF_MASK(pub u32);
pub const TOUCHINPUTMASKF_TIMEFROMSYSTEM: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(1u32);
pub const TOUCHINPUTMASKF_EXTRAINFO: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(2u32);
pub const TOUCHINPUTMASKF_CONTACTAREA: TOUCHINPUTMASKF_MASK = TOUCHINPUTMASKF_MASK(4u32);
impl ::core::marker::Copy for TOUCHINPUTMASKF_MASK {}
impl ::core::clone::Clone for TOUCHINPUTMASKF_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IManipulationEvents(pub *mut ::core::ffi::c_void);
