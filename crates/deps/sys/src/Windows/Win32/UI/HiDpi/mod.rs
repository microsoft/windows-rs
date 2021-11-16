#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRectExForDpi(lprect: *mut super::super::Foundation::RECT, dwstyle: u32, bmenu: super::super::Foundation::BOOL, dwexstyle: u32, dpi: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableNonClientDpiScaling(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    pub fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiAwarenessContextForProcess(hprocess: super::super::Foundation::HANDLE) -> DPI_AWARENESS_CONTEXT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetDpiForMonitor(hmonitor: super::super::Graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetDpiForSystem() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32;
    pub fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDpiAwareness(hprocess: super::super::Foundation::HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32;
    pub fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
    pub fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
    pub fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiAwarenessContext(hwnd: super::super::Foundation::HWND) -> DPI_AWARENESS_CONTEXT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDpiHostingBehavior(hwnd: super::super::Foundation::HWND) -> DPI_HOSTING_BEHAVIOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataForDpi(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dpi: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    pub fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    pub fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT;
    pub fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL;
}
pub type DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = u32;
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 0u32;
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 1u32;
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 2u32;
pub type DIALOG_DPI_CHANGE_BEHAVIORS = u32;
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = 0u32;
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = 1u32;
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = 2u32;
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = 4u32;
pub type DPI_AWARENESS = i32;
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = -1i32;
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = 0i32;
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = 1i32;
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = 2i32;
pub type DPI_AWARENESS_CONTEXT = isize;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = -3i32 as _;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = -4i32 as _;
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = -2i32 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = -1i32 as _;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = -5i32 as _;
pub type DPI_HOSTING_BEHAVIOR = i32;
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = -1i32;
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = 0i32;
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = 1i32;
pub type MONITOR_DPI_TYPE = i32;
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = 0i32;
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = 1i32;
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = 2i32;
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = 0i32;
pub type PROCESS_DPI_AWARENESS = i32;
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = 0i32;
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1i32;
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2i32;
