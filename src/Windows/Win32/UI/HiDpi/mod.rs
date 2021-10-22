#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AdjustWindowRectExForDpi<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lprect: *mut super::super::Foundation::RECT,
    dwstyle: u32,
    bmenu: Param2,
    dwexstyle: u32,
    dpi: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn AdjustWindowRectExForDpi(
                lprect: *mut super::super::Foundation::RECT,
                dwstyle: u32,
                bmenu: super::super::Foundation::BOOL,
                dwexstyle: u32,
                dpi: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AdjustWindowRectExForDpi(
            ::std::mem::transmute(lprect),
            ::std::mem::transmute(dwstyle),
            bmenu.into_param().abi(),
            ::std::mem::transmute(dwexstyle),
            ::std::mem::transmute(dpi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn AreDpiAwarenessContextsEqual<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    dpicontexta: Param0,
    dpicontextb: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn AreDpiAwarenessContextsEqual(
                dpicontexta: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
                dpicontextb: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(AreDpiAwarenessContextsEqual(
            dpicontexta.into_param().abi(),
            dpicontextb.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(pub u32);
pub const DCDC_DEFAULT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS =
    DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(0u32);
pub const DCDC_DISABLE_FONT_UPDATE: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS =
    DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(1u32);
pub const DCDC_DISABLE_RELAYOUT: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS =
    DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS(2u32);
impl ::std::convert::From<u32> for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DIALOG_DPI_CHANGE_BEHAVIORS(pub u32);
pub const DDC_DEFAULT: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(0u32);
pub const DDC_DISABLE_ALL: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(1u32);
pub const DDC_DISABLE_RESIZE: DIALOG_DPI_CHANGE_BEHAVIORS = DIALOG_DPI_CHANGE_BEHAVIORS(2u32);
pub const DDC_DISABLE_CONTROL_RELAYOUT: DIALOG_DPI_CHANGE_BEHAVIORS =
    DIALOG_DPI_CHANGE_BEHAVIORS(4u32);
impl ::std::convert::From<u32> for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DIALOG_DPI_CHANGE_BEHAVIORS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DIALOG_DPI_CHANGE_BEHAVIORS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DPI_AWARENESS(pub i32);
pub const DPI_AWARENESS_INVALID: DPI_AWARENESS = DPI_AWARENESS(-1i32);
pub const DPI_AWARENESS_UNAWARE: DPI_AWARENESS = DPI_AWARENESS(0i32);
pub const DPI_AWARENESS_SYSTEM_AWARE: DPI_AWARENESS = DPI_AWARENESS(1i32);
pub const DPI_AWARENESS_PER_MONITOR_AWARE: DPI_AWARENESS = DPI_AWARENESS(2i32);
impl ::std::convert::From<i32> for DPI_AWARENESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DPI_AWARENESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE:
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT =
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT(-3i32 as _);
#[cfg(feature = "Win32_System_SystemServices")]
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2:
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT =
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT(-4i32 as _);
#[cfg(feature = "Win32_System_SystemServices")]
pub const DPI_AWARENESS_CONTEXT_SYSTEM_AWARE:
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT =
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT(-2i32 as _);
#[cfg(feature = "Win32_System_SystemServices")]
pub const DPI_AWARENESS_CONTEXT_UNAWARE:
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT =
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT(-1i32 as _);
#[cfg(feature = "Win32_System_SystemServices")]
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED:
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT =
    super::super::System::SystemServices::DPI_AWARENESS_CONTEXT(-5i32 as _);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DPI_HOSTING_BEHAVIOR(pub i32);
pub const DPI_HOSTING_BEHAVIOR_INVALID: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(-1i32);
pub const DPI_HOSTING_BEHAVIOR_DEFAULT: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(0i32);
pub const DPI_HOSTING_BEHAVIOR_MIXED: DPI_HOSTING_BEHAVIOR = DPI_HOSTING_BEHAVIOR(1i32);
impl ::std::convert::From<i32> for DPI_HOSTING_BEHAVIOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DPI_HOSTING_BEHAVIOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EnableNonClientDpiScaling<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn EnableNonClientDpiScaling(
                hwnd: super::super::Foundation::HWND,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnableNonClientDpiScaling(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn GetAwarenessFromDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    value: Param0,
) -> DPI_AWARENESS {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetAwarenessFromDpiAwarenessContext(
                value: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> DPI_AWARENESS;
        }
        ::std::mem::transmute(GetAwarenessFromDpiAwarenessContext(
            value.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetDialogControlDpiChangeBehavior<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetDialogControlDpiChangeBehavior(
                hwnd: super::super::Foundation::HWND,
            ) -> DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS;
        }
        ::std::mem::transmute(GetDialogControlDpiChangeBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetDialogDpiChangeBehavior<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hdlg: Param0,
) -> DIALOG_DPI_CHANGE_BEHAVIORS {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetDialogDpiChangeBehavior(
                hdlg: super::super::Foundation::HWND,
            ) -> DIALOG_DPI_CHANGE_BEHAVIORS;
        }
        ::std::mem::transmute(GetDialogDpiChangeBehavior(hdlg.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub unsafe fn GetDpiForMonitor<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>,
>(
    hmonitor: Param0,
    dpitype: MONITOR_DPI_TYPE,
    dpix: *mut u32,
    dpiy: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-shcore-scaling-l1-1-1")]
        extern "system" {
            fn GetDpiForMonitor(
                hmonitor: super::super::Graphics::Gdi::HMONITOR,
                dpitype: MONITOR_DPI_TYPE,
                dpix: *mut u32,
                dpiy: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        GetDpiForMonitor(
            hmonitor.into_param().abi(),
            ::std::mem::transmute(dpitype),
            ::std::mem::transmute(dpix),
            ::std::mem::transmute(dpiy),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetDpiForSystem() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetDpiForSystem() -> u32;
        }
        ::std::mem::transmute(GetDpiForSystem())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetDpiForWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetDpiForWindow(hwnd: super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(GetDpiForWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn GetDpiFromDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    value: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetDpiFromDpiAwarenessContext(
                value: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> u32;
        }
        ::std::mem::transmute(GetDpiFromDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetProcessDpiAwareness<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> ::windows::runtime::Result<PROCESS_DPI_AWARENESS> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-shcore-scaling-l1-1-1")]
        extern "system" {
            fn GetProcessDpiAwareness(
                hprocess: super::super::Foundation::HANDLE,
                value: *mut PROCESS_DPI_AWARENESS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROCESS_DPI_AWARENESS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetProcessDpiAwareness(hprocess.into_param().abi(), &mut result__)
            .from_abi::<PROCESS_DPI_AWARENESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetSystemDpiForProcess<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hprocess: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetSystemDpiForProcess(hprocess: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(GetSystemDpiForProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetSystemMetricsForDpi(nindex: i32, dpi: u32) -> i32;
        }
        ::std::mem::transmute(GetSystemMetricsForDpi(
            ::std::mem::transmute(nindex),
            ::std::mem::transmute(dpi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn GetThreadDpiAwarenessContext(
) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetThreadDpiAwarenessContext(
            ) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT;
        }
        ::std::mem::transmute(GetThreadDpiAwarenessContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetThreadDpiHostingBehavior() -> DPI_HOSTING_BEHAVIOR;
        }
        ::std::mem::transmute(GetThreadDpiHostingBehavior())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn GetWindowDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetWindowDpiAwarenessContext(
                hwnd: super::super::Foundation::HWND,
            ) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT;
        }
        ::std::mem::transmute(GetWindowDpiAwarenessContext(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetWindowDpiHostingBehavior<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn GetWindowDpiHostingBehavior(
                hwnd: super::super::Foundation::HWND,
            ) -> DPI_HOSTING_BEHAVIOR;
        }
        ::std::mem::transmute(GetWindowDpiHostingBehavior(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn IsValidDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    value: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn IsValidDpiAwarenessContext(
                value: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsValidDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn LogicalToPhysicalPointForPerMonitorDPI<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    lppoint: *mut super::super::Foundation::POINT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn LogicalToPhysicalPointForPerMonitorDPI(
                hwnd: super::super::Foundation::HWND,
                lppoint: *mut super::super::Foundation::POINT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(LogicalToPhysicalPointForPerMonitorDPI(
            hwnd.into_param().abi(),
            ::std::mem::transmute(lppoint),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MONITOR_DPI_TYPE(pub i32);
pub const MDT_EFFECTIVE_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
pub const MDT_ANGULAR_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(1i32);
pub const MDT_RAW_DPI: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(2i32);
pub const MDT_DEFAULT: MONITOR_DPI_TYPE = MONITOR_DPI_TYPE(0i32);
impl ::std::convert::From<i32> for MONITOR_DPI_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MONITOR_DPI_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn OpenThemeDataForDpi<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hwnd: Param0,
    pszclasslist: Param1,
    dpi: u32,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "uxtheme")]
        extern "system" {
            fn OpenThemeDataForDpi(
                hwnd: super::super::Foundation::HWND,
                pszclasslist: super::super::Foundation::PWSTR,
                dpi: u32,
            ) -> isize;
        }
        ::std::mem::transmute(OpenThemeDataForDpi(
            hwnd.into_param().abi(),
            pszclasslist.into_param().abi(),
            ::std::mem::transmute(dpi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PROCESS_DPI_AWARENESS(pub i32);
pub const PROCESS_DPI_UNAWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(0i32);
pub const PROCESS_SYSTEM_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(1i32);
pub const PROCESS_PER_MONITOR_DPI_AWARE: PROCESS_DPI_AWARENESS = PROCESS_DPI_AWARENESS(2i32);
impl ::std::convert::From<i32> for PROCESS_DPI_AWARENESS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROCESS_DPI_AWARENESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn PhysicalToLogicalPointForPerMonitorDPI<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    lppoint: *mut super::super::Foundation::POINT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn PhysicalToLogicalPointForPerMonitorDPI(
                hwnd: super::super::Foundation::HWND,
                lppoint: *mut super::super::Foundation::POINT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PhysicalToLogicalPointForPerMonitorDPI(
            hwnd.into_param().abi(),
            ::std::mem::transmute(lppoint),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetDialogControlDpiChangeBehavior<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
    values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SetDialogControlDpiChangeBehavior(
                hwnd: super::super::Foundation::HWND,
                mask: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
                values: DIALOG_CONTROL_DPI_CHANGE_BEHAVIORS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDialogControlDpiChangeBehavior(
            hwnd.into_param().abi(),
            ::std::mem::transmute(mask),
            ::std::mem::transmute(values),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SetDialogDpiChangeBehavior<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hdlg: Param0,
    mask: DIALOG_DPI_CHANGE_BEHAVIORS,
    values: DIALOG_DPI_CHANGE_BEHAVIORS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SetDialogDpiChangeBehavior(
                hdlg: super::super::Foundation::HWND,
                mask: DIALOG_DPI_CHANGE_BEHAVIORS,
                values: DIALOG_DPI_CHANGE_BEHAVIORS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDialogDpiChangeBehavior(
            hdlg.into_param().abi(),
            ::std::mem::transmute(mask),
            ::std::mem::transmute(values),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetProcessDpiAwareness(
    value: PROCESS_DPI_AWARENESS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-shcore-scaling-l1-1-1")]
        extern "system" {
            fn SetProcessDpiAwareness(value: PROCESS_DPI_AWARENESS) -> ::windows::runtime::HRESULT;
        }
        SetProcessDpiAwareness(::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SetProcessDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    value: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SetProcessDpiAwarenessContext(
                value: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetProcessDpiAwarenessContext(value.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_SystemServices")]
pub unsafe fn SetThreadDpiAwarenessContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::DPI_AWARENESS_CONTEXT>,
>(
    dpicontext: Param0,
) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SetThreadDpiAwarenessContext(
                dpicontext: super::super::System::SystemServices::DPI_AWARENESS_CONTEXT,
            ) -> super::super::System::SystemServices::DPI_AWARENESS_CONTEXT;
        }
        ::std::mem::transmute(SetThreadDpiAwarenessContext(dpicontext.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SetThreadDpiHostingBehavior(value: DPI_HOSTING_BEHAVIOR) -> DPI_HOSTING_BEHAVIOR;
        }
        ::std::mem::transmute(SetThreadDpiHostingBehavior(::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SystemParametersInfoForDpi(
    uiaction: u32,
    uiparam: u32,
    pvparam: *mut ::std::ffi::c_void,
    fwinini: u32,
    dpi: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "user32")]
        extern "system" {
            fn SystemParametersInfoForDpi(
                uiaction: u32,
                uiparam: u32,
                pvparam: *mut ::std::ffi::c_void,
                fwinini: u32,
                dpi: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SystemParametersInfoForDpi(
            ::std::mem::transmute(uiaction),
            ::std::mem::transmute(uiparam),
            ::std::mem::transmute(pvparam),
            ::std::mem::transmute(fwinini),
            ::std::mem::transmute(dpi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
