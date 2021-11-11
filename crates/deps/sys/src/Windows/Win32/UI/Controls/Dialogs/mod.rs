#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorA(param0: *mut ::core::mem::ManuallyDrop<CHOOSECOLORA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorW(param0: *mut ::core::mem::ManuallyDrop<CHOOSECOLORW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontA(param0: *mut ::core::mem::ManuallyDrop<CHOOSEFONTA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontW(param0: *mut ::core::mem::ManuallyDrop<CHOOSEFONTW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
    pub fn CommDlgExtendedError() -> COMMON_DLG_ERRORS;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextA(param0: *mut ::core::mem::ManuallyDrop<FINDREPLACEA>) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextW(param0: *mut ::core::mem::ManuallyDrop<FINDREPLACEW>) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleA(param0: super::super::super::Foundation::PSTR, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleW(param0: super::super::super::Foundation::PWSTR, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameA(param0: *mut ::core::mem::ManuallyDrop<OPENFILENAMEA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameW(param0: *mut ::core::mem::ManuallyDrop<OPENFILENAMEW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameA(param0: *mut ::core::mem::ManuallyDrop<OPENFILENAMEA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameW(param0: *mut ::core::mem::ManuallyDrop<OPENFILENAMEW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgA(param0: *mut ::core::mem::ManuallyDrop<PAGESETUPDLGA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgW(param0: *mut ::core::mem::ManuallyDrop<PAGESETUPDLGW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgA(ppd: *mut ::core::mem::ManuallyDrop<PRINTDLGA>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExA(ppd: *mut ::core::mem::ManuallyDrop<PRINTDLGEXA>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExW(ppd: *mut ::core::mem::ManuallyDrop<PRINTDLGEXW>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgW(ppd: *mut ::core::mem::ManuallyDrop<PRINTDLGW>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextA(param0: *mut ::core::mem::ManuallyDrop<FINDREPLACEA>) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextW(param0: *mut ::core::mem::ManuallyDrop<FINDREPLACEW>) -> super::super::super::Foundation::HWND;
}
