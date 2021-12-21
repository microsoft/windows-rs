#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_FIRST: u32 = 1124u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_GETFILEPATH: u32 = 1125u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_GETSPEC: u32 = 1124u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_HIDECONTROL: u32 = 1129u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_LAST: u32 = 1224u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDM_SETDEFEXT: u32 = 1130u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CD_LBSELADD: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CD_LBSELCHANGE: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CD_LBSELNOITEMS: i32 = -1i32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CD_LBSELSUB: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHOOSECOLORA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHOOSECOLORA").field("lStructSize", &self.lStructSize).field("hwndOwner", &self.hwndOwner).field("hInstance", &self.hInstance).field("rgbResult", &self.rgbResult).field("lpCustColors", &self.lpCustColors).field("Flags", &self.Flags).field("lCustData", &self.lCustData).field("lpfnHook", &self.lpfnHook.map(|f| f as usize)).field("lpTemplateName", &self.lpTemplateName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHOOSECOLORA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHOOSECOLORA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSECOLORA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHOOSECOLORA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHOOSECOLORA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHOOSECOLORA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSECOLORA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHOOSECOLORA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHOOSECOLORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHOOSECOLORW").field("lStructSize", &self.lStructSize).field("hwndOwner", &self.hwndOwner).field("hInstance", &self.hInstance).field("rgbResult", &self.rgbResult).field("lpCustColors", &self.lpCustColors).field("Flags", &self.Flags).field("lCustData", &self.lCustData).field("lpfnHook", &self.lpfnHook.map(|f| f as usize)).field("lpTemplateName", &self.lpTemplateName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHOOSECOLORW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHOOSECOLORW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSECOLORW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHOOSECOLORW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CHOOSECOLORW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHOOSECOLORW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSECOLORW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHOOSECOLORW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHOOSEFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHOOSEFONTA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDC", &self.hDC)
            .field("lpLogFont", &self.lpLogFont)
            .field("iPointSize", &self.iPointSize)
            .field("Flags", &self.Flags)
            .field("rgbColors", &self.rgbColors)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .field("hInstance", &self.hInstance)
            .field("lpszStyle", &self.lpszStyle)
            .field("nFontType", &self.nFontType)
            .field("___MISSING_ALIGNMENT__", &self.___MISSING_ALIGNMENT__)
            .field("nSizeMin", &self.nSizeMin)
            .field("nSizeMax", &self.nSizeMax)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CHOOSEFONTA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHOOSEFONTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSEFONTA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHOOSEFONTA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CHOOSEFONTA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHOOSEFONTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSEFONTA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHOOSEFONTA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for CHOOSEFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHOOSEFONTW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDC", &self.hDC)
            .field("lpLogFont", &self.lpLogFont)
            .field("iPointSize", &self.iPointSize)
            .field("Flags", &self.Flags)
            .field("rgbColors", &self.rgbColors)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .field("hInstance", &self.hInstance)
            .field("lpszStyle", &self.lpszStyle)
            .field("nFontType", &self.nFontType)
            .field("___MISSING_ALIGNMENT__", &self.___MISSING_ALIGNMENT__)
            .field("nSizeMin", &self.nSizeMin)
            .field("nSizeMax", &self.nSizeMax)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CHOOSEFONTW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHOOSEFONTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSEFONTW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHOOSEFONTW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for CHOOSEFONTW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for CHOOSEFONTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CHOOSEFONTW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for CHOOSEFONTW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type CHOOSEFONT_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_APPLY: CHOOSEFONT_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_BOTH: CHOOSEFONT_FLAGS = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_TTONLY: CHOOSEFONT_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type CHOOSEFONT_FONT_TYPE = u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = 256u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = 512u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = 16384u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = 1024u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = 8192u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = 32768u16;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_ADD: u32 = 712u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_BLUE: u32 = 708u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_BLUEACCEL: u32 = 728u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_BOX1: u32 = 720u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_CURRENT: u32 = 709u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_CUSTOM1: u32 = 721u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_ELEMENT: u32 = 716u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_GREEN: u32 = 707u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_GREENACCEL: u32 = 727u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_HUE: u32 = 703u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_HUEACCEL: u32 = 723u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_HUESCROLL: u32 = 700u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_LUM: u32 = 705u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_LUMACCEL: u32 = 725u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_LUMSCROLL: u32 = 702u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_MIX: u32 = 719u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_PALETTE: u32 = 718u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_RAINBOW: u32 = 710u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_RED: u32 = 706u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_REDACCEL: u32 = 726u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SAMPLES: u32 = 717u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SAT: u32 = 704u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SATACCEL: u32 = 724u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SATSCROLL: u32 = 701u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SAVE: u32 = 711u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SCHEMES: u32 = 715u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SOLID: u32 = 713u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SOLID_LEFT: u32 = 730u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const COLOR_TUNE: u32 = 714u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type COMMON_DLG_ERRORS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = 65535u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = 3u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = 5u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = 6u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = 7u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = 9u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = 10u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = 11u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = 12u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = 4097u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = 4098u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = 4099u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = 4100u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = 4101u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = 4102u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = 4103u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = 4104u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = 4105u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = 4106u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = 4107u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = 4108u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = 8193u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = 8194u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = 12288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = 12289u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = 12290u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = 12291u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = 16385u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = 20480u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChooseColorA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChooseColorW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChooseFontA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChooseFontW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[inline]
pub unsafe fn CommDlgExtendedError() -> COMMON_DLG_ERRORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommDlgExtendedError() -> COMMON_DLG_ERRORS;
        }
        ::core::mem::transmute(CommDlgExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DEVNAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVNAMES").field("wDriverOffset", &self.wDriverOffset).field("wDeviceOffset", &self.wDeviceOffset).field("wOutputOffset", &self.wOutputOffset).field("wDefault", &self.wDefault).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DEVNAMES {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DEVNAMES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVNAMES>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DEVNAMES {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DEVNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[cfg(target_arch = "x86")]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for DEVNAMES {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for DEVNAMES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVNAMES>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for DEVNAMES {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DEVNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const DLG_COLOR: u32 = 10u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const DN_DEFAULTPRN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FINDREPLACEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FINDREPLACEA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("Flags", &self.Flags)
            .field("lpstrFindWhat", &self.lpstrFindWhat)
            .field("lpstrReplaceWith", &self.lpstrReplaceWith)
            .field("wFindWhatLen", &self.wFindWhatLen)
            .field("wReplaceWithLen", &self.wReplaceWithLen)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDREPLACEA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDREPLACEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDREPLACEA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDREPLACEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDREPLACEA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDREPLACEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDREPLACEA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDREPLACEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PWSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FINDREPLACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FINDREPLACEW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("Flags", &self.Flags)
            .field("lpstrFindWhat", &self.lpstrFindWhat)
            .field("lpstrReplaceWith", &self.lpstrReplaceWith)
            .field("wFindWhatLen", &self.wFindWhatLen)
            .field("wReplaceWithLen", &self.wReplaceWithLen)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDREPLACEW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDREPLACEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDREPLACEW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDREPLACEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PWSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FINDREPLACEW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FINDREPLACEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FINDREPLACEW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FINDREPLACEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type FINDREPLACE_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_DOWN: FINDREPLACE_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_REPLACE: FINDREPLACE_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRM_FIRST: u32 = 1124u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRM_LAST: u32 = 1224u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_NOWRAPAROUND: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_RAW: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_SHOWWRAPAROUND: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const FR_WRAPAROUND: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(FindTextA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(FindTextW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileTitleA<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PSTR>>(param0: Param0, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileTitleA(param0: super::super::super::Foundation::PSTR, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16;
        }
        ::core::mem::transmute(GetFileTitleA(param0.into_param().abi(), ::core::mem::transmute(buf), ::core::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFileTitleW<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(param0: Param0, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFileTitleW(param0: super::super::super::Foundation::PWSTR, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16;
        }
        ::core::mem::transmute(GetFileTitleW(param0.into_param().abi(), ::core::mem::transmute(buf), ::core::mem::transmute(cchsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOpenFileNameA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOpenFileNameW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSaveFileNameA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSaveFileNameW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[repr(transparent)]
pub struct IPrintDialogCallback(::windows::core::IUnknown);
impl IPrintDialogCallback {
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
    pub unsafe fn InitDone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
    pub unsafe fn SelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandleMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::WPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::LPARAM>>(&self, hdlg: Param0, umsg: u32, wparam: Param2, lparam: Param3, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), hdlg.into_param().abi(), ::core::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(presult)).ok()
    }
}
impl ::core::convert::From<IPrintDialogCallback> for ::windows::core::IUnknown {
    fn from(value: IPrintDialogCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDialogCallback> for ::windows::core::IUnknown {
    fn from(value: &IPrintDialogCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDialogCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPrintDialogCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintDialogCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintDialogCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDialogCallback {}
impl ::core::fmt::Debug for IPrintDialogCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDialogCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintDialogCallback {
    type Vtable = IPrintDialogCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5852a2c3_6530_11d1_b6a3_0000f8757bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[repr(transparent)]
pub struct IPrintDialogServices(::windows::core::IUnknown);
impl IPrintDialogServices {
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdevmode), ::core::mem::transmute(pcbsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pprintername), ::core::mem::transmute(pcchsize)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentPortName(&self, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pportname), ::core::mem::transmute(pcchsize)).ok()
    }
}
impl ::core::convert::From<IPrintDialogServices> for ::windows::core::IUnknown {
    fn from(value: IPrintDialogServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPrintDialogServices> for ::windows::core::IUnknown {
    fn from(value: &IPrintDialogServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintDialogServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPrintDialogServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPrintDialogServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPrintDialogServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDialogServices {}
impl ::core::fmt::Debug for IPrintDialogServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDialogServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintDialogServices {
    type Vtable = IPrintDialogServicesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509aaeda_5639_11d1_b6a1_0000f8757bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServicesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintername: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportname: super::super::super::Foundation::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPCCHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPCFHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFRHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOFNHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGEPAINTHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGESETUPHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPRINTHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPSETUPHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const NUM_BASIC_COLORS: u32 = 48u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFNOTIFYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFNOTIFYA").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("pszFile", &self.pszFile).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: super::super::super::Foundation::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFNOTIFYEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFNOTIFYEXA").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("psf", &self.psf).field("pidl", &self.pidl).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYEXA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYEXA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYEXA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYEXA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYEXA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYEXA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFNOTIFYEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFNOTIFYEXW").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("psf", &self.psf).field("pidl", &self.pidl).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYEXW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYEXW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYEXW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYEXW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYEXW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYEXW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFNOTIFYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFNOTIFYW").field("hdr", &self.hdr).field("lpOFN", &self.lpOFN).field("pszFile", &self.pszFile).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: super::super::super::Foundation::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OFNOTIFYW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFNOTIFYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OFNOTIFYW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFNOTIFYW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_SHARENOWARN: u32 = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_SHAREWARN: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENFILENAMEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENFILENAMEA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .field("pvReserved", &self.pvReserved)
            .field("dwReserved", &self.dwReserved)
            .field("FlagsEx", &self.FlagsEx)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAMEA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAMEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAMEA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAMEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAMEA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAMEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAMEA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAMEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENFILENAMEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENFILENAMEW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .field("pvReserved", &self.pvReserved)
            .field("dwReserved", &self.dwReserved)
            .field("FlagsEx", &self.FlagsEx)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAMEW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAMEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAMEW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAMEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAMEW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAMEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAMEW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAMEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENFILENAME_NT4A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENFILENAME_NT4A")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAME_NT4A {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAME_NT4A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAME_NT4A>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAME_NT4A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAME_NT4A {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAME_NT4A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAME_NT4A>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAME_NT4A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OPENFILENAME_NT4W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPENFILENAME_NT4W")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hInstance", &self.hInstance)
            .field("lpstrFilter", &self.lpstrFilter)
            .field("lpstrCustomFilter", &self.lpstrCustomFilter)
            .field("nMaxCustFilter", &self.nMaxCustFilter)
            .field("nFilterIndex", &self.nFilterIndex)
            .field("lpstrFile", &self.lpstrFile)
            .field("nMaxFile", &self.nMaxFile)
            .field("lpstrFileTitle", &self.lpstrFileTitle)
            .field("nMaxFileTitle", &self.nMaxFileTitle)
            .field("lpstrInitialDir", &self.lpstrInitialDir)
            .field("lpstrTitle", &self.lpstrTitle)
            .field("Flags", &self.Flags)
            .field("nFileOffset", &self.nFileOffset)
            .field("nFileExtension", &self.nFileExtension)
            .field("lpstrDefExt", &self.lpstrDefExt)
            .field("lCustData", &self.lCustData)
            .field("lpfnHook", &self.lpfnHook.map(|f| f as usize))
            .field("lpTemplateName", &self.lpTemplateName)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAME_NT4W {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAME_NT4W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAME_NT4W>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAME_NT4W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OPENFILENAME_NT4W {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OPENFILENAME_NT4W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OPENFILENAME_NT4W>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OPENFILENAME_NT4W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type OPEN_FILENAME_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = 33554432u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type OPEN_FILENAME_FLAGS_EX = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAGESETUPDLGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGESETUPDLGA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("Flags", &self.Flags)
            .field("ptPaperSize", &self.ptPaperSize)
            .field("rtMinMargin", &self.rtMinMargin)
            .field("rtMargin", &self.rtMargin)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpfnPageSetupHook", &self.lpfnPageSetupHook.map(|f| f as usize))
            .field("lpfnPagePaintHook", &self.lpfnPagePaintHook.map(|f| f as usize))
            .field("lpPageSetupTemplateName", &self.lpPageSetupTemplateName)
            .field("hPageSetupTemplate", &self.hPageSetupTemplate)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAGESETUPDLGA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESETUPDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAGESETUPDLGA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESETUPDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAGESETUPDLGA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESETUPDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAGESETUPDLGA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESETUPDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAGESETUPDLGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAGESETUPDLGW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("Flags", &self.Flags)
            .field("ptPaperSize", &self.ptPaperSize)
            .field("rtMinMargin", &self.rtMinMargin)
            .field("rtMargin", &self.rtMargin)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpfnPageSetupHook", &self.lpfnPageSetupHook.map(|f| f as usize))
            .field("lpfnPagePaintHook", &self.lpfnPagePaintHook.map(|f| f as usize))
            .field("lpPageSetupTemplateName", &self.lpPageSetupTemplateName)
            .field("hPageSetupTemplate", &self.hPageSetupTemplate)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAGESETUPDLGW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESETUPDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAGESETUPDLGW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESETUPDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PAGESETUPDLGW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAGESETUPDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PAGESETUPDLGW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAGESETUPDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type PAGESETUPDLG_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RESULT_APPLY: u32 = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RESULT_CANCEL: u32 = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RESULT_PRINT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTDLGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTDLGA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("nFromPage", &self.nFromPage)
            .field("nToPage", &self.nToPage)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpfnPrintHook", &self.lpfnPrintHook.map(|f| f as usize))
            .field("lpfnSetupHook", &self.lpfnSetupHook.map(|f| f as usize))
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpSetupTemplateName", &self.lpSetupTemplateName)
            .field("hPrintTemplate", &self.hPrintTemplate)
            .field("hSetupTemplate", &self.hSetupTemplate)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGA {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGA>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGA {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpCallback: ::core::option::Option<::windows::core::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXA {
    fn clone(&self) -> Self {
        Self {
            lStructSize: self.lStructSize,
            hwndOwner: self.hwndOwner,
            hDevMode: self.hDevMode,
            hDevNames: self.hDevNames,
            hDC: self.hDC,
            Flags: self.Flags,
            Flags2: self.Flags2,
            ExclusionFlags: self.ExclusionFlags,
            nPageRanges: self.nPageRanges,
            nMaxPageRanges: self.nMaxPageRanges,
            lpPageRanges: self.lpPageRanges,
            nMinPage: self.nMinPage,
            nMaxPage: self.nMaxPage,
            nCopies: self.nCopies,
            hInstance: self.hInstance,
            lpPrintTemplateName: self.lpPrintTemplateName,
            lpCallback: self.lpCallback.clone(),
            nPropertyPages: self.nPropertyPages,
            lphPropertyPages: self.lphPropertyPages,
            nStartPage: self.nStartPage,
            dwResultAction: self.dwResultAction,
        }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTDLGEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTDLGEXA")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("Flags2", &self.Flags2)
            .field("ExclusionFlags", &self.ExclusionFlags)
            .field("nPageRanges", &self.nPageRanges)
            .field("nMaxPageRanges", &self.nMaxPageRanges)
            .field("lpPageRanges", &self.lpPageRanges)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpCallback", &self.lpCallback)
            .field("nPropertyPages", &self.nPropertyPages)
            .field("lphPropertyPages", &self.lphPropertyPages)
            .field("nStartPage", &self.nStartPage)
            .field("dwResultAction", &self.dwResultAction)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGEXA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGEXA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.Flags2 == other.Flags2
            && self.ExclusionFlags == other.ExclusionFlags
            && self.nPageRanges == other.nPageRanges
            && self.nMaxPageRanges == other.nMaxPageRanges
            && self.lpPageRanges == other.lpPageRanges
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpCallback == other.lpCallback
            && self.nPropertyPages == other.nPropertyPages
            && self.lphPropertyPages == other.lphPropertyPages
            && self.nStartPage == other.nStartPage
            && self.dwResultAction == other.dwResultAction
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGEXA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpCallback: ::core::option::Option<::windows::core::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGEXA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGEXA>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGEXA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpCallback: ::core::option::Option<::windows::core::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXW {
    fn clone(&self) -> Self {
        Self {
            lStructSize: self.lStructSize,
            hwndOwner: self.hwndOwner,
            hDevMode: self.hDevMode,
            hDevNames: self.hDevNames,
            hDC: self.hDC,
            Flags: self.Flags,
            Flags2: self.Flags2,
            ExclusionFlags: self.ExclusionFlags,
            nPageRanges: self.nPageRanges,
            nMaxPageRanges: self.nMaxPageRanges,
            lpPageRanges: self.lpPageRanges,
            nMinPage: self.nMinPage,
            nMaxPage: self.nMaxPage,
            nCopies: self.nCopies,
            hInstance: self.hInstance,
            lpPrintTemplateName: self.lpPrintTemplateName,
            lpCallback: self.lpCallback.clone(),
            nPropertyPages: self.nPropertyPages,
            lphPropertyPages: self.lphPropertyPages,
            nStartPage: self.nStartPage,
            dwResultAction: self.dwResultAction,
        }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTDLGEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTDLGEXW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("Flags2", &self.Flags2)
            .field("ExclusionFlags", &self.ExclusionFlags)
            .field("nPageRanges", &self.nPageRanges)
            .field("nMaxPageRanges", &self.nMaxPageRanges)
            .field("lpPageRanges", &self.lpPageRanges)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpCallback", &self.lpCallback)
            .field("nPropertyPages", &self.nPropertyPages)
            .field("lphPropertyPages", &self.lphPropertyPages)
            .field("nStartPage", &self.nStartPage)
            .field("dwResultAction", &self.dwResultAction)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGEXW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGEXW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize
            && self.hwndOwner == other.hwndOwner
            && self.hDevMode == other.hDevMode
            && self.hDevNames == other.hDevNames
            && self.hDC == other.hDC
            && self.Flags == other.Flags
            && self.Flags2 == other.Flags2
            && self.ExclusionFlags == other.ExclusionFlags
            && self.nPageRanges == other.nPageRanges
            && self.nMaxPageRanges == other.nMaxPageRanges
            && self.lpPageRanges == other.lpPageRanges
            && self.nMinPage == other.nMinPage
            && self.nMaxPage == other.nMaxPage
            && self.nCopies == other.nCopies
            && self.hInstance == other.hInstance
            && self.lpPrintTemplateName == other.lpPrintTemplateName
            && self.lpCallback == other.lpCallback
            && self.nPropertyPages == other.nPropertyPages
            && self.lphPropertyPages == other.lphPropertyPages
            && self.nStartPage == other.nStartPage
            && self.dwResultAction == other.dwResultAction
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGEXW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpCallback: ::core::option::Option<::windows::core::IUnknown>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGEXW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGEXW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGEXW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub type PRINTDLGEX_FLAGS = u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_COLLATE: PRINTDLGEX_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = 4194304u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = 16384u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = 16777216u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = 1048576u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = 8388608u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = 256u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = 512u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_SELECTION: PRINTDLGEX_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = 268435456u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = 8192u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = 32768u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = 2097152u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = 64u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = 2048u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for PRINTDLGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTDLGW")
            .field("lStructSize", &self.lStructSize)
            .field("hwndOwner", &self.hwndOwner)
            .field("hDevMode", &self.hDevMode)
            .field("hDevNames", &self.hDevNames)
            .field("hDC", &self.hDC)
            .field("Flags", &self.Flags)
            .field("nFromPage", &self.nFromPage)
            .field("nToPage", &self.nToPage)
            .field("nMinPage", &self.nMinPage)
            .field("nMaxPage", &self.nMaxPage)
            .field("nCopies", &self.nCopies)
            .field("hInstance", &self.hInstance)
            .field("lCustData", &self.lCustData)
            .field("lpfnPrintHook", &self.lpfnPrintHook.map(|f| f as usize))
            .field("lpfnSetupHook", &self.lpfnSetupHook.map(|f| f as usize))
            .field("lpPrintTemplateName", &self.lpPrintTemplateName)
            .field("lpSetupTemplateName", &self.lpSetupTemplateName)
            .field("hPrintTemplate", &self.hPrintTemplate)
            .field("hSetupTemplate", &self.hSetupTemplate)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGW {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGW>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::core::Abi for PRINTDLGW {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for PRINTDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTDLGW>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for PRINTDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for PRINTPAGERANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRINTPAGERANGE").field("nFromPage", &self.nFromPage).field("nToPage", &self.nToPage).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for PRINTPAGERANGE {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for PRINTPAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTPAGERANGE>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for PRINTPAGERANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for PRINTPAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
#[cfg(target_arch = "x86")]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for PRINTPAGERANGE {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for PRINTPAGERANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PRINTPAGERANGE>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for PRINTPAGERANGE {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for PRINTPAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PageSetupDlgA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PageSetupDlgW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PrintDlgA(::core::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows::core::HRESULT;
        }
        PrintDlgExA(::core::mem::transmute(ppd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows::core::HRESULT;
        }
        PrintDlgExW(::core::mem::transmute(ppd)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation', 'Win32_Graphics_Gdi'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PrintDlgW(::core::mem::transmute(ppd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ReplaceTextA(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
        }
        ::core::mem::transmute(ReplaceTextW(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const TYPE1_FONTTYPE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
#[doc = "*Required features: 'Win32_UI_Controls_Dialogs'*"]
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
