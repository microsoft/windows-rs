#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmAssociateContextEx();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmConfigureIMEW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmCreateIMCC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmCreateSoftKeyboard();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmDestroyContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmDestroyIMCC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDestroySoftKeyboard();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableIME();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableLegacyIME();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmDisableTextFrameService();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmEnumInputContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmEnumRegisterWordW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmEscapeW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGenerateMessage();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListCountW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCandidateListW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCandidateWindow();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetCompositionFontW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetCompositionStringW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetCompositionWindow();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetConversionListW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetConversionStatus();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetDefaultIMEWnd();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetDescriptionW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetGuideLineW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetHotKey();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCLockCount();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCCSize();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmGetIMCLockCount();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetIMEFileNameW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmGetImeMenuItemsW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetOpenStatus();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetProperty();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmGetRegisterWordStyleA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_UI_TextServices`*"]
    #[cfg(feature = "Win32_UI_TextServices")]
    pub fn ImmGetRegisterWordStyleW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmGetStatusWindowPos();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmGetVirtualKey();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmInstallIMEW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmIsIME();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmIsUIMessageW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmLockIMC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmLockIMCC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmNotifyIME();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Globalization`*"]
    #[cfg(feature = "Win32_Globalization")]
    pub fn ImmReSizeIMCC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmRegisterWordW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmReleaseContext();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmRequestMessageW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCandidateWindow();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
    pub fn ImmSetCompositionFontW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionStringW();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetCompositionWindow();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetConversionStatus();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmSetHotKey();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetOpenStatus();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmSetStatusWindowPos();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmShowSoftKeyboard();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImmSimulateHotKey();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_Globalization`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization"))]
    pub fn ImmUnlockIMCC();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordA();
    #[doc = "*Required features: `Win32_UI_Input_Ime`, `Win32_Foundation`, `Win32_UI_TextServices`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
    pub fn ImmUnregisterWordW();
}
