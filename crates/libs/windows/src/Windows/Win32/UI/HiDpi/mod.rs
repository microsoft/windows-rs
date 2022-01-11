#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation', 'Win32_UI_WindowsAndMessaging'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: Param2, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdjustWindowRectExForDpi(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: super::super::Foundation::BOOL, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AdjustWindowRectExForDpi(::core::mem::transmute(lprect), ::core::mem::transmute(dwstyle), bmenu.into_param().abi(), ::core::mem::transmute(dwexstyle), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>, Param1: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(dpicontexta: Param0, dpicontextb: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AreDpiAwarenessContextsEqual(dpicontexta.into_param().abi(), dpicontextb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 0u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 1u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = 2u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type DIALOG_DPI_CHANGE_BEHAVIORS = u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = 0u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = 1u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = 2u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = 4u32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type DPI_AWARENESS = i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = -1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = 0i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = 1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = 2i32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub isize);
impl DPI_AWARENESS_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for DPI_AWARENESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for DPI_AWARENESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for DPI_AWARENESS_CONTEXT {}
impl ::core::fmt::Debug for DPI_AWARENESS_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS_CONTEXT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for DPI_AWARENESS_CONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type DPI_HOSTING_BEHAVIOR = i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = -1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = 0i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = 1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableNonClientDpiScaling<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableNonClientDpiScaling(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnableNonClientDpiScaling(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> DPI_AWARENESS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS;
        }
        ::core::mem::transmute(GetAwarenessFromDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
        }
        ::core::mem::transmute(GetDialogControlDpiChangeBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hdlg: Param0) -> DIALOG_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS;
        }
        ::core::mem::transmute(GetDialogDpiChangeBehavior(hdlg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiAwarenessContextForProcess(hprocess: super::super::Foundation::HANDLE) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetDpiAwarenessContextForProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Graphics_Gdi'*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDpiForMonitor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>>(hmonitor: Param0, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForMonitor(hmonitor: super::super::Graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows::core::HRESULT;
        }
        GetDpiForMonitor(hmonitor.into_param().abi(), ::core::mem::transmute(dpitype), ::core::mem::transmute(dpix), ::core::mem::transmute(dpiy)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForSystem() -> u32;
        }
        ::core::mem::transmute(GetDpiForSystem())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32;
        }
        ::core::mem::transmute(GetDpiForWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32;
        }
        ::core::mem::transmute(GetDpiFromDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDpiAwareness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> ::windows::core::Result<PROCESS_DPI_AWARENESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDpiAwareness(hprocess: super::super::Foundation::HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> ::windows::core::HRESULT;
        }
        let mut result__: PROCESS_DPI_AWARENESS = ::core::mem::zeroed();
        GetProcessDpiAwareness(hprocess.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<PROCESS_DPI_AWARENESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDpiForProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetSystemDpiForProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
        }
        ::core::mem::transmute(GetSystemMetricsForDpi(::core::mem::transmute(nindex), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetThreadDpiAwarenessContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(GetThreadDpiHostingBehavior())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowDpiAwarenessContext(hwnd: super::super::Foundation::HWND) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(GetWindowDpiAwarenessContext(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowDpiHostingBehavior(hwnd: super::super::Foundation::HWND) -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(GetWindowDpiHostingBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsValidDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(LogicalToPhysicalPointForPerMonitorDPI(hwnd.into_param().abi(), ::core::mem::transmute(lppoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type MONITOR_DPI_TYPE = i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = 1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = 2i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = 0i32;
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeDataForDpi<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pszclasslist: Param1, dpi: u32) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeDataForDpi(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dpi: u32) -> isize;
        }
        ::core::mem::transmute(OpenThemeDataForDpi(hwnd.into_param().abi(), pszclasslist.into_param().abi(), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub type PROCESS_DPI_AWARENESS = i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = 0i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = 1i32;
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = 2i32;
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PhysicalToLogicalPointForPerMonitorDPI(hwnd.into_param().abi(), ::core::mem::transmute(lppoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDialogControlDpiChangeBehavior(hwnd.into_param().abi(), ::core::mem::transmute(mask), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hdlg: Param0, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDialogDpiChangeBehavior(hdlg.into_param().abi(), ::core::mem::transmute(mask), ::core::mem::transmute(values)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::core::HRESULT;
        }
        SetProcessDpiAwareness(::core::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(value: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext<'a, Param0: ::windows::core::IntoParam<'a, DPI_AWARENESS_CONTEXT>>(dpicontext: Param0) -> DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT;
        }
        ::core::mem::transmute(SetThreadDpiAwarenessContext(dpicontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi'*"]
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
        }
        ::core::mem::transmute(SetThreadDpiHostingBehavior(::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_HiDpi', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SystemParametersInfoForDpi(::core::mem::transmute(uiaction), ::core::mem::transmute(uiparam), ::core::mem::transmute(pvparam), ::core::mem::transmute(fwinini), ::core::mem::transmute(dpi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
