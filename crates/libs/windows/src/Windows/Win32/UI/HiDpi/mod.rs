#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi<'a, P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: P0, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AdjustWindowRectExForDpi(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: super::super::Foundation::BOOL, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> super::super::Foundation::BOOL;
    }
    AdjustWindowRectExForDpi(::core::mem::transmute(lprect), dwstyle, bmenu.into(), dwexstyle, dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual<'a, P0, P1>(dpicontexta: P0, dpicontextb: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
    P1: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn AreDpiAwarenessContextsEqual(dpicontexta: DPI_AWARENESS_CONTEXT, dpicontextb: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    }
    AreDpiAwarenessContextsEqual(dpicontexta.into(), dpicontextb.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2u32);
impl ::core::marker::Copy for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {}
impl ::core::clone::Clone for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2u32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(4u32);
impl ::core::marker::Copy for DIALOG_DPI_CHANGE_BEHAVIORS {}
impl ::core::clone::Clone for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DPI_AWARENESS(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = DPI_AWARENESS(-1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = DPI_AWARENESS(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = DPI_AWARENESS(1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = DPI_AWARENESS(2i32);
impl ::core::marker::Copy for DPI_AWARENESS {}
impl ::core::clone::Clone for DPI_AWARENESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DPI_AWARENESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DPI_AWARENESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DPI_AWARENESS_CONTEXT(pub isize);
impl DPI_AWARENESS_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<DPI_AWARENESS_CONTEXT>> for DPI_AWARENESS_CONTEXT {
    fn from(optional: ::core::option::Option<DPI_AWARENESS_CONTEXT>) -> DPI_AWARENESS_CONTEXT {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for DPI_AWARENESS_CONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-3i32 as _);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-4i32 as _);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-2i32 as _);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-1i32 as _);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: DPI_AWARENESS_CONTEXT = DPI_AWARENESS_CONTEXT(-5i32 as _);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(-1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(1i32);
impl ::core::marker::Copy for DPI_HOSTING_BEHAVIOR {}
impl ::core::clone::Clone for DPI_HOSTING_BEHAVIOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DPI_HOSTING_BEHAVIOR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DPI_HOSTING_BEHAVIOR {
    type Abi = Self;
}
impl ::core::fmt::Debug for DPI_HOSTING_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_HOSTING_BEHAVIOR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableNonClientDpiScaling<'a, P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnableNonClientDpiScaling(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    }
    EnableNonClientDpiScaling(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<'a, P0>(value: P0) -> DPI_AWARENESS
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetAwarenessFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS;
    }
    GetAwarenessFromDpiAwarenessContext(value.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior<'a, P0>(hwnd: P0) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
    }
    GetDialogControlDpiChangeBehavior(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior<'a, P0>(hdlg: P0) -> DIALOG_DPI_CHANGE_BEHAVIORS
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND) -> DIALOG_DPI_CHANGE_BEHAVIORS;
    }
    GetDialogDpiChangeBehavior(hdlg.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess<'a, P0>(hprocess: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDpiAwarenessContextForProcess(hprocess: super::super::Foundation::HANDLE) -> DPI_AWARENESS_CONTEXT;
    }
    GetDpiAwarenessContextForProcess(hprocess.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDpiForMonitor<'a, P0>(hmonitor: P0, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HMONITOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDpiForMonitor(hmonitor: super::super::Graphics::Gdi::HMONITOR, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows::core::HRESULT;
    }
    GetDpiForMonitor(hmonitor.into(), dpitype, ::core::mem::transmute(dpix), ::core::mem::transmute(dpiy)).ok()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDpiForSystem() -> u32;
    }
    GetDpiForSystem()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiForWindow<'a, P0>(hwnd: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32;
    }
    GetDpiForWindow(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext<'a, P0>(value: P0) -> u32
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetDpiFromDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> u32;
    }
    GetDpiFromDpiAwarenessContext(value.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDpiAwareness<'a, P0>(hprocess: P0) -> ::windows::core::Result<PROCESS_DPI_AWARENESS>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetProcessDpiAwareness(hprocess: super::super::Foundation::HANDLE, value: *mut PROCESS_DPI_AWARENESS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetProcessDpiAwareness(hprocess.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<PROCESS_DPI_AWARENESS>(result__)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDpiForProcess<'a, P0>(hprocess: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32;
    }
    GetSystemDpiForProcess(hprocess.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
    }
    GetSystemMetricsForDpi(nindex, dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT;
    }
    GetThreadDpiAwarenessContext()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
    }
    GetThreadDpiHostingBehavior()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext<'a, P0>(hwnd: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowDpiAwarenessContext(hwnd: super::super::Foundation::HWND) -> DPI_AWARENESS_CONTEXT;
    }
    GetWindowDpiAwarenessContext(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior<'a, P0>(hwnd: P0) -> DPI_HOSTING_BEHAVIOR
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetWindowDpiHostingBehavior(hwnd: super::super::Foundation::HWND) -> DPI_HOSTING_BEHAVIOR;
    }
    GetWindowDpiHostingBehavior(hwnd.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidDpiAwarenessContext<'a, P0>(value: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn IsValidDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    }
    IsValidDpiAwarenessContext(value.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LogicalToPhysicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    LogicalToPhysicalPointForPerMonitorDPI(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MONITOR_DPI_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(2i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
impl ::core::marker::Copy for MONITOR_DPI_TYPE {}
impl ::core::clone::Clone for MONITOR_DPI_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MONITOR_DPI_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MONITOR_DPI_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MONITOR_DPI_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_DPI_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeDataForDpi<'a, P0, P1>(hwnd: P0, pszclasslist: P1, dpi: u32) -> isize
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenThemeDataForDpi(hwnd: super::super::Foundation::HWND, pszclasslist: ::windows::core::PCWSTR, dpi: u32) -> isize;
    }
    OpenThemeDataForDpi(hwnd.into(), pszclasslist.into(), dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(2i32);
impl ::core::marker::Copy for PROCESS_DPI_AWARENESS {}
impl ::core::clone::Clone for PROCESS_DPI_AWARENESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_DPI_AWARENESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DPI_AWARENESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DPI_AWARENESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<'a, P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PhysicalToLogicalPointForPerMonitorDPI(hwnd: super::super::Foundation::HWND, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    }
    PhysicalToLogicalPointForPerMonitorDPI(hwnd.into(), ::core::mem::transmute(lppoint))
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior<'a, P0>(hwnd: P0, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDialogControlDpiChangeBehavior(hwnd: super::super::Foundation::HWND, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    }
    SetDialogControlDpiChangeBehavior(hwnd.into(), mask, values)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior<'a, P0>(hdlg: P0, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetDialogDpiChangeBehavior(hdlg: super::super::Foundation::HWND, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL;
    }
    SetDialogDpiChangeBehavior(hdlg.into(), mask, values)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::core::HRESULT;
    }
    SetProcessDpiAwareness(value).ok()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext<'a, P0>(value: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> super::super::Foundation::BOOL;
    }
    SetProcessDpiAwarenessContext(value.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext<'a, P0>(dpicontext: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::std::convert::Into<DPI_AWARENESS_CONTEXT>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetThreadDpiAwarenessContext(dpicontext: DPI_AWARENESS_CONTEXT) -> DPI_AWARENESS_CONTEXT;
    }
    SetThreadDpiAwarenessContext(dpicontext.into())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
    }
    SetThreadDpiHostingBehavior(value)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: *mut ::core::ffi::c_void, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL;
    }
    SystemParametersInfoForDpi(uiaction, uiparam, ::core::mem::transmute(pvparam), fwinini, dpi)
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
