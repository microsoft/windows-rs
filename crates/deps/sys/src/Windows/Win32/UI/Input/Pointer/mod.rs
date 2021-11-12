#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableMouseInPointer(fenable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevice(device: super::super::super::Foundation::HANDLE, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceCursors(device: super::super::super::Foundation::HANDLE, cursorcount: *mut u32, devicecursors: *mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceProperties(device: super::super::super::Foundation::HANDLE, propertycount: *mut u32, pointerproperties: *mut super::super::Controls::POINTER_DEVICE_PROPERTY) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerDeviceRects(device: super::super::super::Foundation::HANDLE, pointerdevicerect: *mut super::super::super::Foundation::RECT, displayrect: *mut super::super::super::Foundation::RECT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevices(devicecount: *mut u32, pointerdevices: *mut super::super::Controls::POINTER_DEVICE_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: *mut POINTER_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerInputTransform(pointerid: u32, historycount: u32, inputtransform: *mut INPUT_TRANSFORM) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: *mut POINTER_PEN_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, propertiescount: u32, pproperties: *const super::super::Controls::POINTER_DEVICE_PROPERTY, pvalues: *mut i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`*"]
    pub fn GetUnpredictedMessagePos() -> u32;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectSyntheticPointerInput(device: super::super::Controls::HSYNTHETICPOINTERDEVICE, pointerinfo: *const super::super::Controls::POINTER_TYPE_INFO, count: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectTouchInput(count: u32, contacts: *const POINTER_TOUCH_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMouseInPointerEnabled() -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SkipPointerFrameMessages(pointerid: u32) -> super::super::super::Foundation::BOOL;
}
pub struct INPUT_INJECTION_VALUE(i32);
pub struct INPUT_TRANSFORM(i32);
pub struct POINTER_BUTTON_CHANGE_TYPE(i32);
pub struct POINTER_FLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_PEN_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct POINTER_TOUCH_INFO(i32);
pub struct TOUCH_FEEDBACK_MODE(i32);
