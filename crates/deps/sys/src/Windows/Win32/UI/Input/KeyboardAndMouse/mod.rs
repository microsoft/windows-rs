#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ActivateKeyboardLayout(hkl: super::super::TextServices::HKL, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BlockInput(fblockit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragDetect(hwnd: super::super::super::Foundation::HWND, pt: super::super::super::Foundation::POINT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableWindow(hwnd: super::super::super::Foundation::HWND, benable: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActiveWindow() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetAsyncKeyState(vkey: i32) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCapture() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetDoubleClickTime() -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFocus() -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKBCodePage() -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextA(lparam: i32, lpstring: super::super::super::Foundation::PSTR, cchsize: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyNameTextW(lparam: i32, lpstring: super::super::super::Foundation::PWSTR, cchsize: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKeyState(nvirtkey: i32) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayout(idthread: u32) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn GetKeyboardLayoutList(nbuff: i32, lplist: *mut super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameA(pwszklid: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardLayoutNameW(pwszklid: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetKeyboardState(lpkeystate: *mut u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetKeyboardType(ntypeflag: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn GetMouseMovePointsEx(cbsize: u32, lppt: *const MOUSEMOVEPOINT, lpptbuf: *mut MOUSEMOVEPOINT, nbufpoints: i32, resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowEnabled(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutA(pwszklid: super::super::super::Foundation::PSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn LoadKeyboardLayoutW(pwszklid: super::super::super::Foundation::PWSTR, flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS) -> super::super::TextServices::HKL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExA(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn MapVirtualKeyExW(ucode: u32, umaptype: u32, dwhkl: super::super::TextServices::HKL) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn OemKeyScan(woemchar: u16) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32, fsmodifiers: HOT_KEY_MODIFIERS, vk: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseCapture() -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActiveWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCapture(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDoubleClickTime(param0: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFocus(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetKeyboardState(lpkeystate: *const u8) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwapMouseButton(fswap: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn ToAscii(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ToAsciiEx(uvirtkey: u32, uscancode: u32, lpkeystate: *const u8, lpchar: *mut u16, uflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ToUnicode(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: super::super::super::Foundation::PWSTR, cchbuff: i32, wflags: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ToUnicodeEx(wvirtkey: u32, wscancode: u32, lpkeystate: *const u8, pwszbuff: super::super::super::Foundation::PWSTR, cchbuff: i32, wflags: u32, dwhkl: super::super::TextServices::HKL) -> i32;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn UnloadKeyboardLayout(hkl: super::super::TextServices::HKL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterHotKey(hwnd: super::super::super::Foundation::HWND, id: i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VkKeyScanA(ch: super::super::super::Foundation::CHAR) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn VkKeyScanExA(ch: super::super::super::Foundation::CHAR, dwhkl: super::super::TextServices::HKL) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn VkKeyScanExW(ch: u16, dwhkl: super::super::TextServices::HKL) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn VkKeyScanW(ch: u16) -> i16;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _TrackMouseEvent(lpeventtrack: *mut TRACKMOUSEEVENT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn keybd_event(bvk: u8, bscan: u8, dwflags: KEYBD_EVENT_FLAGS, dwextrainfo: usize);
    #[doc = "*Required features: `Win32_UI_Input_KeyboardAndMouse`*"]
    pub fn mouse_event(dwflags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, dwdata: u32, dwextrainfo: usize);
}
