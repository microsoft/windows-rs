#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ActivateKeyboardLayout();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BlockInput();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragDetect();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableWindow();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActiveWindow();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetAsyncKeyState();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapture();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetDoubleClickTime();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFocus();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKBCodePage();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKeyState();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayout();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayoutList();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardState();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKeyboardType();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastInputInfo();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetMouseMovePointsEx();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowEnabled();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn MapVirtualKeyA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn MapVirtualKeyW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn OemKeyScan();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterHotKey();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseCapture();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn SendInput();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActiveWindow();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCapture();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDoubleClickTime();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFocus();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKeyboardState();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwapMouseButton();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn ToAscii();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ToAsciiEx();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ToUnicode();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ToUnicodeEx();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackMouseEvent();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn UnloadKeyboardLayout();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterHotKey();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VkKeyScanA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn VkKeyScanExA();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn VkKeyScanExW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn VkKeyScanW();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _TrackMouseEvent();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn keybd_event();
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn mouse_event();
}
