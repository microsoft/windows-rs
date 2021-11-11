#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AddStroke();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddWordsToWordList();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdviseInkChange();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateContext();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateRecognizer();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyContext();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyRecognizer();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyWordList();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn EndInkInput();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetAllRecognizers();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBestResultString();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetLatticePtr();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLeftSeparator();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetRecoAttributes();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetResultPropertyList();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRightSeparator();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetUnicodeRanges();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsStringSupported();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn LoadCachedAttributes();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeWordList();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetEnabledUnicodeRanges();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFactoid();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetFlags();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetGuide();
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextContext();
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetWordList();
}
