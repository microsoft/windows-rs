#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRectExForDpi();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreDpiAwarenessContextsEqual();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableNonClientDpiScaling();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetAwarenessFromDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogControlDpiChangeBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogDpiChangeBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiAwarenessContextForProcess();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetDpiForMonitor();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetDpiForSystem();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiForWindow();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetDpiFromDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDpiAwareness();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDpiForProcess();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetSystemMetricsForDpi();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetThreadDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetThreadDpiHostingBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiHostingBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogicalToPhysicalPointForPerMonitorDPI();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataForDpi();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PhysicalToLogicalPointForPerMonitorDPI();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogControlDpiChangeBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogDpiChangeBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetProcessDpiAwareness();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetThreadDpiAwarenessContext();
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetThreadDpiHostingBehavior();
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoForDpi();
}
