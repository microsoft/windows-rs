#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableMouseInPointer(fenable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevice(device: super::super::super::Foundation::HANDLE, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceCursors(device: super::super::super::Foundation::HANDLE, cursorcount: *mut u32, devicecursors: *mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceProperties(device: super::super::super::Foundation::HANDLE, propertycount: *mut u32, pointerproperties: *mut super::super::Controls::POINTER_DEVICE_PROPERTY) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerDeviceRects(device: super::super::super::Foundation::HANDLE, pointerdevicerect: *mut super::super::super::Foundation::RECT, displayrect: *mut super::super::super::Foundation::RECT) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevices(devicecount: *mut u32, pointerdevices: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerInputTransform(pointerid: u32, historycount: u32, inputtransform: *mut INPUT_TRANSFORM) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, propertiescount: u32, pproperties: *const super::super::Controls::POINTER_DEVICE_PROPERTY, pvalues: *mut i32) -> super::super::super::Foundation::BOOL;
    pub fn GetUnpredictedMessagePos() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectSyntheticPointerInput(device: super::super::Controls::HSYNTHETICPOINTERDEVICE, pointerinfo: *const super::super::Controls::POINTER_TYPE_INFO, count: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectTouchInput(count: u32, contacts: *const POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMouseInPointerEnabled() -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SkipPointerFrameMessages(pointerid: u32) -> super::super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct INPUT_INJECTION_VALUE(i32);
#[repr(C)]
pub struct INPUT_TRANSFORM(i32);
#[repr(transparent)]
pub struct POINTER_BUTTON_CHANGE_TYPE(pub i32);
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(0i32);
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(1i32);
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(2i32);
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(3i32);
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(4i32);
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(5i32);
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(6i32);
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(7i32);
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(8i32);
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(9i32);
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(10i32);
#[repr(transparent)]
pub struct POINTER_FLAGS(pub u32);
pub const POINTER_FLAG_NONE: POINTER_FLAGS = POINTER_FLAGS(0u32);
pub const POINTER_FLAG_NEW: POINTER_FLAGS = POINTER_FLAGS(1u32);
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = POINTER_FLAGS(2u32);
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = POINTER_FLAGS(4u32);
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = POINTER_FLAGS(16u32);
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = POINTER_FLAGS(32u32);
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = POINTER_FLAGS(64u32);
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(128u32);
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(256u32);
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = POINTER_FLAGS(8192u32);
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = POINTER_FLAGS(16384u32);
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = POINTER_FLAGS(32768u32);
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = POINTER_FLAGS(65536u32);
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = POINTER_FLAGS(131072u32);
pub const POINTER_FLAG_UP: POINTER_FLAGS = POINTER_FLAGS(262144u32);
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = POINTER_FLAGS(524288u32);
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = POINTER_FLAGS(1048576u32);
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = POINTER_FLAGS(2097152u32);
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = POINTER_FLAGS(4194304u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct POINTER_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct POINTER_PEN_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct POINTER_TOUCH_INFO(i32);
#[repr(transparent)]
pub struct TOUCH_FEEDBACK_MODE(pub u32);
pub const TOUCH_FEEDBACK_DEFAULT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(1u32);
pub const TOUCH_FEEDBACK_INDIRECT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(2u32);
pub const TOUCH_FEEDBACK_NONE: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(3u32);
