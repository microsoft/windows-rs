#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableMouseInPointer();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerCursorId();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevice();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceCursors();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDeviceProperties();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerDeviceRects();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
    pub fn GetPointerDevices();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFramePenInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerFrameTouchInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPointerInputTransform();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerPenInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfo();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerTouchInfoHistory();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn GetPointerType();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn GetRawPointerDeviceData();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`*"]
    pub fn GetUnpredictedMessagePos();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeTouchInjection();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectSyntheticPointerInput();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn InjectTouchInput();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMouseInPointerEnabled();
    #[doc = "*Required features: `Win32_UI_Input_Pointer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SkipPointerFrameMessages();
}
