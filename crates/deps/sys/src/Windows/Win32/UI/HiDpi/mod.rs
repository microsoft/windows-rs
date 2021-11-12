#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRectExForDpi(lprect: *mut super::super::Foundation::RECT, dwstyle: u32, bmenu: super::super::Foundation::BOOL, dwexstyle: u32, dpi: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableNonClientDpiScaling(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiAwarenessContextForProcess(hprocess: super::super::Foundation::HANDLE) -> DPI_AWARENESS_CONTEXT;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetDpiForMonitor(hmonitor: super::super::Graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetDpiForSystem() -> u32;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDpiAwareness(hprocess: super::super::Foundation::HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiAwarenessContext(hwnd: super::super::Foundation::HWND) -> DPI_AWARENESS_CONTEXT;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiHostingBehavior(hwnd: super::super::Foundation::HWND) -> DPI_HOSTING_BEHAVIOR;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataForDpi(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dpi: u32) -> isize;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT;
    #[doc = "*Required features: `Win32_UI_HiDpi`*"]
    pub fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
    #[doc = "*Required features: `Win32_UI_HiDpi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL;
}
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(i32);
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(i32);
pub struct DPI_AWARENESS(i32);
pub struct DPI_AWARENESS_CONTEXT(i32);
#[doc = "*Required features: `Win32_UI_HiDpi`*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
#[doc = "*Required features: `Win32_UI_HiDpi`*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
#[doc = "*Required features: `Win32_UI_HiDpi`*"]
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
#[doc = "*Required features: `Win32_UI_HiDpi`*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
#[doc = "*Required features: `Win32_UI_HiDpi`*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
pub struct DPI_HOSTING_BEHAVIOR(i32);
pub struct MONITOR_DPI_TYPE(i32);
pub struct PROCESS_DPI_AWARENESS(i32);
