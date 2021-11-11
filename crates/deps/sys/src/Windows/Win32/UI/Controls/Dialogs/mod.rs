#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
    pub fn CommDlgExtendedError();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgW();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextA();
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextW();
}
