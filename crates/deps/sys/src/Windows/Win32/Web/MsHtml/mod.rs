#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ComputeInvCMAP();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_DirectDraw`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Graphics_DirectDraw", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateDDrawSurfaceOnDIB();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn CreateMIMEMap();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DecodeImage();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DecodeImageEx();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DitherTo8();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoPrivacyDlg();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn GetMaxMIMEIDBytes();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn IdentifyMIMEType();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialog2W();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAccessDeniedDialogW();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingAddToApprovedSites();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccess();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingCheckUserAccessW();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnPRFInternal();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingClickedOnRATInternal();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnable();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingEnableW();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingEnabledQuery();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingFreeDetails();
    #[doc = "*Required features: `Win32_Web_MsHtml`*"]
    pub fn RatingInit();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainCancel();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQuery();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingObtainQueryW();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUI();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RatingSetupUIW();
    #[doc = "*Required features: `Win32_Web_MsHtml`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SniffStream();
}
