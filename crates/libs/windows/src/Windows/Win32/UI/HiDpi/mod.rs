#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn AdjustWindowRectExForDpi<P0>(lprect: *mut super::super::Foundation::RECT, dwstyle: super::WindowsAndMessaging::WINDOW_STYLE, bmenu: P0, dwexstyle: super::WindowsAndMessaging::WINDOW_EX_STYLE, dpi: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn AdjustWindowRectExForDpi ( lprect : *mut super::super::Foundation:: RECT , dwstyle : super::WindowsAndMessaging:: WINDOW_STYLE , bmenu : super::super::Foundation:: BOOL , dwexstyle : super::WindowsAndMessaging:: WINDOW_EX_STYLE , dpi : u32 ) -> super::super::Foundation:: BOOL );
    AdjustWindowRectExForDpi(lprect, dwstyle, bmenu.into_param().abi(), dwexstyle, dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual<P0, P1>(dpicontexta: P0, dpicontextb: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
    P1: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn AreDpiAwarenessContextsEqual ( dpicontexta : DPI_AWARENESS_CONTEXT , dpicontextb : DPI_AWARENESS_CONTEXT ) -> super::super::Foundation:: BOOL );
    AreDpiAwarenessContextsEqual(dpicontexta.into_param().abi(), dpicontextb.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableNonClientDpiScaling<P0>(hwnd: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn EnableNonClientDpiScaling ( hwnd : super::super::Foundation:: HWND ) -> super::super::Foundation:: BOOL );
    EnableNonClientDpiScaling(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<P0>(value: P0) -> DPI_AWARENESS
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetAwarenessFromDpiAwarenessContext ( value : DPI_AWARENESS_CONTEXT ) -> DPI_AWARENESS );
    GetAwarenessFromDpiAwarenessContext(value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogControlDpiChangeBehavior<P0>(hwnd: P0) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetDialogControlDpiChangeBehavior ( hwnd : super::super::Foundation:: HWND ) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS );
    GetDialogControlDpiChangeBehavior(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDialogDpiChangeBehavior<P0>(hdlg: P0) -> DIALOG_DPI_CHANGE_BEHAVIORS
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetDialogDpiChangeBehavior ( hdlg : super::super::Foundation:: HWND ) -> DIALOG_DPI_CHANGE_BEHAVIORS );
    GetDialogDpiChangeBehavior(hdlg.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiAwarenessContextForProcess<P0>(hprocess: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetDpiAwarenessContextForProcess ( hprocess : super::super::Foundation:: HANDLE ) -> DPI_AWARENESS_CONTEXT );
    GetDpiAwarenessContextForProcess(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetDpiForMonitor<P0>(hmonitor: P0, dpitype: MONITOR_DPI_TYPE, dpix: *mut u32, dpiy: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HMONITOR>,
{
    ::windows::imp::link ! ( "api-ms-win-shcore-scaling-l1-1-1.dll""system" fn GetDpiForMonitor ( hmonitor : super::super::Graphics::Gdi:: HMONITOR , dpitype : MONITOR_DPI_TYPE , dpix : *mut u32 , dpiy : *mut u32 ) -> :: windows::core::HRESULT );
    GetDpiForMonitor(hmonitor.into_param().abi(), dpitype, dpix, dpiy).ok()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetDpiForSystem() -> u32 {
    ::windows::imp::link ! ( "user32.dll""system" fn GetDpiForSystem ( ) -> u32 );
    GetDpiForSystem()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDpiForWindow<P0>(hwnd: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetDpiForWindow ( hwnd : super::super::Foundation:: HWND ) -> u32 );
    GetDpiForWindow(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetDpiFromDpiAwarenessContext<P0>(value: P0) -> u32
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetDpiFromDpiAwarenessContext ( value : DPI_AWARENESS_CONTEXT ) -> u32 );
    GetDpiFromDpiAwarenessContext(value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDpiAwareness<P0>(hprocess: P0) -> ::windows::core::Result<PROCESS_DPI_AWARENESS>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "api-ms-win-shcore-scaling-l1-1-1.dll""system" fn GetProcessDpiAwareness ( hprocess : super::super::Foundation:: HANDLE , value : *mut PROCESS_DPI_AWARENESS ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<PROCESS_DPI_AWARENESS>();
    GetProcessDpiAwareness(hprocess.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemDpiForProcess<P0>(hprocess: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetSystemDpiForProcess ( hprocess : super::super::Foundation:: HANDLE ) -> u32 );
    GetSystemDpiForProcess(hprocess.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetSystemMetricsForDpi(nindex: super::WindowsAndMessaging::SYSTEM_METRICS_INDEX, dpi: u32) -> i32 {
    ::windows::imp::link ! ( "user32.dll""system" fn GetSystemMetricsForDpi ( nindex : super::WindowsAndMessaging:: SYSTEM_METRICS_INDEX , dpi : u32 ) -> i32 );
    GetSystemMetricsForDpi(nindex, dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    ::windows::imp::link ! ( "user32.dll""system" fn GetThreadDpiAwarenessContext ( ) -> DPI_AWARENESS_CONTEXT );
    GetThreadDpiAwarenessContext()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    ::windows::imp::link ! ( "user32.dll""system" fn GetThreadDpiHostingBehavior ( ) -> DPI_HOSTING_BEHAVIOR );
    GetThreadDpiHostingBehavior()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiAwarenessContext<P0>(hwnd: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetWindowDpiAwarenessContext ( hwnd : super::super::Foundation:: HWND ) -> DPI_AWARENESS_CONTEXT );
    GetWindowDpiAwarenessContext(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowDpiHostingBehavior<P0>(hwnd: P0) -> DPI_HOSTING_BEHAVIOR
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn GetWindowDpiHostingBehavior ( hwnd : super::super::Foundation:: HWND ) -> DPI_HOSTING_BEHAVIOR );
    GetWindowDpiHostingBehavior(hwnd.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsValidDpiAwarenessContext<P0>(value: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn IsValidDpiAwarenessContext ( value : DPI_AWARENESS_CONTEXT ) -> super::super::Foundation:: BOOL );
    IsValidDpiAwarenessContext(value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn LogicalToPhysicalPointForPerMonitorDPI ( hwnd : super::super::Foundation:: HWND , lppoint : *mut super::super::Foundation:: POINT ) -> super::super::Foundation:: BOOL );
    LogicalToPhysicalPointForPerMonitorDPI(hwnd.into_param().abi(), lppoint)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn OpenThemeDataForDpi<P0, P1>(hwnd: P0, pszclasslist: P1, dpi: u32) -> super::Controls::HTHEME
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "uxtheme.dll""system" fn OpenThemeDataForDpi ( hwnd : super::super::Foundation:: HWND , pszclasslist : :: windows::core::PCWSTR , dpi : u32 ) -> super::Controls:: HTHEME );
    OpenThemeDataForDpi(hwnd.into_param().abi(), pszclasslist.into_param().abi(), dpi)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<P0>(hwnd: P0, lppoint: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn PhysicalToLogicalPointForPerMonitorDPI ( hwnd : super::super::Foundation:: HWND , lppoint : *mut super::super::Foundation:: POINT ) -> super::super::Foundation:: BOOL );
    PhysicalToLogicalPointForPerMonitorDPI(hwnd.into_param().abi(), lppoint)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogControlDpiChangeBehavior<P0>(hwnd: P0, mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS, values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetDialogControlDpiChangeBehavior ( hwnd : super::super::Foundation:: HWND , mask : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS , values : DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS ) -> super::super::Foundation:: BOOL );
    SetDialogControlDpiChangeBehavior(hwnd.into_param().abi(), mask, values)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDialogDpiChangeBehavior<P0>(hdlg: P0, mask: DIALOG_DPI_CHANGE_BEHAVIORS, values: DIALOG_DPI_CHANGE_BEHAVIORS) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetDialogDpiChangeBehavior ( hdlg : super::super::Foundation:: HWND , mask : DIALOG_DPI_CHANGE_BEHAVIORS , values : DIALOG_DPI_CHANGE_BEHAVIORS ) -> super::super::Foundation:: BOOL );
    SetDialogDpiChangeBehavior(hdlg.into_param().abi(), mask, values)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "api-ms-win-shcore-scaling-l1-1-1.dll""system" fn SetProcessDpiAwareness ( value : PROCESS_DPI_AWARENESS ) -> :: windows::core::HRESULT );
    SetProcessDpiAwareness(value).ok()
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext<P0>(value: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetProcessDpiAwarenessContext ( value : DPI_AWARENESS_CONTEXT ) -> super::super::Foundation:: BOOL );
    SetProcessDpiAwarenessContext(value.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetThreadDpiAwarenessContext<P0>(dpicontext: P0) -> DPI_AWARENESS_CONTEXT
where
    P0: ::windows::core::IntoParam<DPI_AWARENESS_CONTEXT>,
{
    ::windows::imp::link ! ( "user32.dll""system" fn SetThreadDpiAwarenessContext ( dpicontext : DPI_AWARENESS_CONTEXT ) -> DPI_AWARENESS_CONTEXT );
    SetThreadDpiAwarenessContext(dpicontext.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
#[inline]
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    ::windows::imp::link ! ( "user32.dll""system" fn SetThreadDpiHostingBehavior ( value : DPI_HOSTING_BEHAVIOR ) -> DPI_HOSTING_BEHAVIOR );
    SetThreadDpiHostingBehavior(value)
}
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SystemParametersInfoForDpi(uiaction: u32, uiparam: u32, pvparam: ::core::option::Option<*mut ::core::ffi::c_void>, fwinini: u32, dpi: u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "user32.dll""system" fn SystemParametersInfoForDpi ( uiaction : u32 , uiparam : u32 , pvparam : *mut ::core::ffi::c_void , fwinini : u32 , dpi : u32 ) -> super::super::Foundation:: BOOL );
    SystemParametersInfoForDpi(uiaction, uiparam, ::core::mem::transmute(pvparam.unwrap_or(::std::ptr::null_mut())), fwinini, dpi)
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
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS = DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2i32);
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
impl ::windows::core::TypeKind for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2i32);
#[doc = "*Required features: `\"Win32_UI_HiDpi\"`*"]
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(4i32);
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
impl ::windows::core::TypeKind for DIALOG_DPI_CHANGE_BEHAVIORS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIALOG_DPI_CHANGE_BEHAVIORS").field(&self.0).finish()
    }
}
impl DIALOG_DPI_CHANGE_BEHAVIORS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows::core::TypeKind for DPI_AWARENESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_AWARENESS").field(&self.0).finish()
    }
}
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
impl ::windows::core::TypeKind for DPI_HOSTING_BEHAVIOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DPI_HOSTING_BEHAVIOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DPI_HOSTING_BEHAVIOR").field(&self.0).finish()
    }
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
impl ::windows::core::TypeKind for MONITOR_DPI_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MONITOR_DPI_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_DPI_TYPE").field(&self.0).finish()
    }
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
impl ::windows::core::TypeKind for PROCESS_DPI_AWARENESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESS_DPI_AWARENESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DPI_AWARENESS").field(&self.0).finish()
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
impl ::windows::core::TypeKind for DPI_AWARENESS_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
