#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("comctl32.dll" "system" fn CreatePropertySheetPageA(constpropsheetpagepointer : LPCPROPSHEETPAGEA) -> HPROPSHEETPAGE);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("comctl32.dll" "system" fn CreatePropertySheetPageW(constpropsheetpagepointer : LPCPROPSHEETPAGEW) -> HPROPSHEETPAGE);
windows_link::link!("comctl32.dll" "system" fn DestroyPropertySheetPage(param0 : *mut _PSP) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("comctl32.dll" "system" fn PropertySheetA(param0 : LPCPROPSHEETHEADERA) -> isize);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
windows_link::link!("comctl32.dll" "system" fn PropertySheetW(param0 : LPCPROPSHEETHEADERW) -> isize);
pub type HPROPSHEETPAGE = *mut _PSP;
pub const ID_PSREBOOTSYSTEM: u32 = 3;
pub const ID_PSRESTARTWINDOWS: u32 = 2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERA = LPCPROPSHEETHEADERA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERA_V1 = *const PROPSHEETHEADERA_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERA_V2 = *const PROPSHEETHEADERA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERW = LPCPROPSHEETHEADERW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERW_V1 = *const PROPSHEETHEADERW_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETHEADERW_V2 = *const PROPSHEETHEADERW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA = LPCPROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA_LATEST = LPCPROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA_V1 = *const PROPSHEETPAGEA_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA_V2 = *const PROPSHEETPAGEA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA_V3 = *const PROPSHEETPAGEA_V3;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEA_V4 = *const PROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW = LPCPROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW_LATEST = LPCPROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW_V1 = *const PROPSHEETPAGEW_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW_V2 = *const PROPSHEETPAGEW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW_V3 = *const PROPSHEETPAGEW_V3;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPCPROPSHEETPAGEW_V4 = *const PROPSHEETPAGEW_V4;
#[cfg(feature = "minwindef")]
pub type LPFNADDPROPSHEETPAGE = Option<unsafe extern "system" fn(param0: *mut _PSP, param1: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type LPFNADDPROPSHEETPAGES = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: LPFNADDPROPSHEETPAGE, param2: super::LPARAM) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPFNPSPCALLBACKA = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, ppsp: *mut PROPSHEETPAGEA_V4) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPFNPSPCALLBACKW = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, ppsp: *mut PROPSHEETPAGEW_V4) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERA = LPPROPSHEETHEADERA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERA_V1 = *mut PROPSHEETHEADERA_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERA_V2 = *mut PROPSHEETHEADERA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERW = LPPROPSHEETHEADERW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERW_V1 = *mut PROPSHEETHEADERW_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETHEADERW_V2 = *mut PROPSHEETHEADERW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA = LPPROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA_LATEST = LPPROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA_V1 = *mut PROPSHEETPAGEA_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA_V2 = *mut PROPSHEETPAGEA_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA_V3 = *mut PROPSHEETPAGEA_V3;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEA_V4 = *mut PROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW = LPPROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW_LATEST = LPPROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW_V1 = *mut PROPSHEETPAGEW_V1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW_V2 = *mut PROPSHEETPAGEW_V2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW_V3 = *mut PROPSHEETPAGEW_V3;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type LPPROPSHEETPAGEW_V4 = *mut PROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type LPPSHNOTIFY = *mut PSHNOTIFY;
pub const MAXPROPPAGES: u32 = 100;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type PFNPROPSHEETCALLBACK = Option<unsafe extern "system" fn(param0: super::HWND, param1: u32, param2: super::LPARAM) -> i32>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETHEADERA = PROPSHEETHEADERA_V2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETHEADERA_V1_0,
    pub pszCaption: windows_sys::core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V1_1,
    pub Anonymous3: PROPSHEETHEADERA_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_0 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V1_2 {
    pub ppsp: LPCPROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETHEADERA_V2_0,
    pub pszCaption: windows_sys::core::PCSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V2_1,
    pub Anonymous3: PROPSHEETHEADERA_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERA_V2_3,
    pub hplWatermark: super::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERA_V2_4,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_0 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_2 {
    pub ppsp: LPCPROPSHEETPAGEA,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_3 {
    pub hbmWatermark: super::HBITMAP,
    pub pszbmWatermark: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERA_V2_4 {
    pub hbmHeader: super::HBITMAP,
    pub pszbmHeader: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERA_V2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETHEADERW = PROPSHEETHEADERW_V2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETHEADERW_V1_0,
    pub pszCaption: windows_sys::core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V1_1,
    pub Anonymous3: PROPSHEETHEADERW_V1_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_0 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V1_2 {
    pub ppsp: LPCPROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETHEADERW_V2_0,
    pub pszCaption: windows_sys::core::PCWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V2_1,
    pub Anonymous3: PROPSHEETHEADERW_V2_2,
    pub pfnCallback: PFNPROPSHEETCALLBACK,
    pub Anonymous4: PROPSHEETHEADERW_V2_3,
    pub hplWatermark: super::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERW_V2_4,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_0 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_2 {
    pub ppsp: LPCPROPSHEETPAGEW,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_3 {
    pub hbmWatermark: super::HBITMAP,
    pub pszbmWatermark: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETHEADERW_V2_4 {
    pub hbmHeader: super::HBITMAP,
    pub pszbmHeader: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETHEADERW_V2_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const PROPSHEETHEADER_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PROPSHEETHEADER_V1_SIZE: u32 = 72;
#[cfg(target_arch = "x86")]
pub const PROPSHEETHEADER_V2_SIZE: u32 = 52;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PROPSHEETHEADER_V2_SIZE: u32 = 96;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETPAGEA = PROPSHEETPAGEA_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETPAGEA_LATEST = PROPSHEETPAGEA_V4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEA_V1_0,
    pub Anonymous2: PROPSHEETPAGEA_V1_1,
    pub pszTitle: windows_sys::core::PCSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V1_0 {
    pub pszTemplate: windows_sys::core::PCSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V1_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEA_V2_0,
    pub Anonymous2: PROPSHEETPAGEA_V2_1,
    pub pszTitle: windows_sys::core::PCSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V2_0 {
    pub pszTemplate: windows_sys::core::PCSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V2_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEA_V3_0,
    pub Anonymous2: PROPSHEETPAGEA_V3_1,
    pub pszTitle: windows_sys::core::PCSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCSTR,
    pub hActCtx: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V3_0 {
    pub pszTemplate: windows_sys::core::PCSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V3_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEA_V4 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEA_V4_0,
    pub Anonymous2: PROPSHEETPAGEA_V4_1,
    pub pszTitle: windows_sys::core::PCSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKA,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCSTR,
    pub hActCtx: super::HANDLE,
    pub Anonymous3: PROPSHEETPAGEA_V4_2,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V4_0 {
    pub pszTemplate: windows_sys::core::PCSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V4_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V4_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEA_V4_2 {
    pub hbmHeader: super::HBITMAP,
    pub pszbmHeader: windows_sys::core::PCSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEA_V4_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETPAGEW = PROPSHEETPAGEW_V4;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PROPSHEETPAGEW_LATEST = PROPSHEETPAGEW_V4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEW_V1_0,
    pub Anonymous2: PROPSHEETPAGEW_V1_1,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V1_0 {
    pub pszTemplate: windows_sys::core::PCWSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V1_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEW_V2_0,
    pub Anonymous2: PROPSHEETPAGEW_V2_1,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCWSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V2_0 {
    pub pszTemplate: windows_sys::core::PCWSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V2_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEW_V3_0,
    pub Anonymous2: PROPSHEETPAGEW_V3_1,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCWSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCWSTR,
    pub hActCtx: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V3_0 {
    pub pszTemplate: windows_sys::core::PCWSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V3_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETPAGEW_V4 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::HINSTANCE,
    pub Anonymous: PROPSHEETPAGEW_V4_0,
    pub Anonymous2: PROPSHEETPAGEW_V4_1,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub pfnDlgProc: super::DLGPROC,
    pub lParam: super::LPARAM,
    pub pfnCallback: LPFNPSPCALLBACKW,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: windows_sys::core::PCWSTR,
    pub pszHeaderSubTitle: windows_sys::core::PCWSTR,
    pub hActCtx: super::HANDLE,
    pub Anonymous3: PROPSHEETPAGEW_V4_2,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V4_0 {
    pub pszTemplate: windows_sys::core::PCWSTR,
    pub pResource: PROPSHEETPAGE_RESOURCE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V4_1 {
    pub hIcon: super::HICON,
    pub pszIcon: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V4_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union PROPSHEETPAGEW_V4_2 {
    pub hbmHeader: super::HBITMAP,
    pub pszbmHeader: windows_sys::core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PROPSHEETPAGEW_V4_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winuser")]
pub type PROPSHEETPAGE_RESOURCE = super::LPCDLGTEMPLATE;
#[cfg(target_arch = "x86")]
pub const PROPSHEETPAGE_V1_SIZE: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PROPSHEETPAGE_V1_SIZE: u32 = 72;
#[cfg(target_arch = "x86")]
pub const PROPSHEETPAGE_V2_SIZE: u32 = 48;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PROPSHEETPAGE_V2_SIZE: u32 = 88;
pub const PROP_LG_CXDLG: u32 = 252;
pub const PROP_LG_CYDLG: u32 = 218;
pub const PROP_MED_CXDLG: u32 = 227;
pub const PROP_MED_CYDLG: u32 = 215;
pub const PROP_SM_CXDLG: u32 = 212;
pub const PROP_SM_CYDLG: u32 = 188;
pub const PSBTN_APPLYNOW: u32 = 4;
pub const PSBTN_BACK: u32 = 0;
pub const PSBTN_CANCEL: u32 = 5;
pub const PSBTN_FINISH: u32 = 2;
pub const PSBTN_HELP: u32 = 6;
pub const PSBTN_MAX: u32 = 6;
pub const PSBTN_NEXT: u32 = 1;
pub const PSBTN_OK: u32 = 3;
pub const PSCB_BUTTONPRESSED: u32 = 3;
pub const PSCB_INITIALIZED: u32 = 1;
pub const PSCB_PRECREATE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct PSHNOTIFY {
    pub hdr: super::NMHDR,
    pub lParam: super::LPARAM,
}
pub const PSH_AEROWIZARD: u32 = 16384;
pub const PSH_DEFAULT: u32 = 0;
pub const PSH_HASHELP: u32 = 512;
pub const PSH_HEADER: u32 = 524288;
pub const PSH_HEADERBITMAP: u32 = 134217728;
pub const PSH_MODELESS: u32 = 1024;
pub const PSH_NOAPPLYNOW: u32 = 128;
pub const PSH_NOCONTEXTHELP: u32 = 33554432;
pub const PSH_NOMARGIN: u32 = 268435456;
pub const PSH_PROPSHEETPAGE: u32 = 8;
pub const PSH_PROPTITLE: u32 = 1;
pub const PSH_RESIZABLE: u32 = 67108864;
pub const PSH_RTLREADING: u32 = 2048;
pub const PSH_STRETCHWATERMARK: u32 = 262144;
pub const PSH_USECALLBACK: u32 = 256;
pub const PSH_USEHBMHEADER: u32 = 1048576;
pub const PSH_USEHBMWATERMARK: u32 = 65536;
pub const PSH_USEHICON: u32 = 2;
pub const PSH_USEHPLWATERMARK: u32 = 131072;
pub const PSH_USEICONID: u32 = 4;
pub const PSH_USEPAGELANG: u32 = 2097152;
pub const PSH_USEPSTARTPAGE: u32 = 64;
pub const PSH_WATERMARK: u32 = 32768;
pub const PSH_WIZARD: u32 = 32;
pub const PSH_WIZARD97: u32 = 16777216;
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096;
pub const PSH_WIZARDHASFINISH: u32 = 16;
pub const PSH_WIZARD_LITE: u32 = 4194304;
pub const PSM_ADDPAGE: u32 = 1127;
pub const PSM_APPLY: u32 = 1134;
pub const PSM_CANCELTOCLOSE: u32 = 1131;
pub const PSM_CHANGED: u32 = 1128;
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163;
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142;
pub const PSM_GETRESULT: u32 = 1159;
pub const PSM_GETTABCONTROL: u32 = 1140;
pub const PSM_HWNDTOINDEX: u32 = 1153;
pub const PSM_IDTOINDEX: u32 = 1157;
pub const PSM_INDEXTOHWND: u32 = 1154;
pub const PSM_INDEXTOID: u32 = 1158;
pub const PSM_INDEXTOPAGE: u32 = 1156;
pub const PSM_INSERTPAGE: u32 = 1143;
pub const PSM_ISDIALOGMESSAGE: u32 = 1141;
pub const PSM_PAGETOINDEX: u32 = 1155;
pub const PSM_PRESSBUTTON: u32 = 1137;
pub const PSM_QUERYSIBLINGS: u32 = 1132;
pub const PSM_REBOOTSYSTEM: u32 = 1130;
pub const PSM_RECALCPAGESIZES: u32 = 1160;
pub const PSM_REMOVEPAGE: u32 = 1126;
pub const PSM_RESTARTWINDOWS: u32 = 1129;
pub const PSM_SETBUTTONTEXT: u32 = 1164;
pub const PSM_SETBUTTONTEXTW: u32 = 1164;
pub const PSM_SETCURSEL: u32 = 1125;
pub const PSM_SETCURSELID: u32 = 1138;
pub const PSM_SETFINISHTEXT: u32 = 1139;
pub const PSM_SETFINISHTEXTA: u32 = 1139;
pub const PSM_SETFINISHTEXTW: u32 = 1145;
pub const PSM_SETHEADERSUBTITLE: u32 = 1151;
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151;
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152;
pub const PSM_SETHEADERTITLE: u32 = 1149;
pub const PSM_SETHEADERTITLEA: u32 = 1149;
pub const PSM_SETHEADERTITLEW: u32 = 1150;
pub const PSM_SETNEXTTEXT: u32 = 1161;
pub const PSM_SETNEXTTEXTW: u32 = 1161;
pub const PSM_SETTITLE: u32 = 1135;
pub const PSM_SETTITLEA: u32 = 1135;
pub const PSM_SETTITLEW: u32 = 1144;
pub const PSM_SETWIZBUTTONS: u32 = 1136;
pub const PSM_SHOWWIZBUTTONS: u32 = 1162;
pub const PSM_UNCHANGED: u32 = 1133;
pub const PSNRET_INVALID: u32 = 1;
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2;
pub const PSNRET_MESSAGEHANDLED: u32 = 3;
pub const PSNRET_NOERROR: u32 = 0;
pub const PSN_APPLY: i32 = -202;
pub const PSN_FIRST: i32 = -200;
pub const PSN_GETOBJECT: i32 = -210;
pub const PSN_HELP: i32 = -205;
pub const PSN_KILLACTIVE: i32 = -201;
pub const PSN_LAST: i32 = -299;
pub const PSN_QUERYCANCEL: i32 = -209;
pub const PSN_QUERYINITIALFOCUS: i32 = -213;
pub const PSN_RESET: i32 = -203;
pub const PSN_SETACTIVE: i32 = -200;
pub const PSN_TRANSLATEACCELERATOR: i32 = -212;
pub const PSN_WIZBACK: i32 = -206;
pub const PSN_WIZFINISH: i32 = -208;
pub const PSN_WIZNEXT: i32 = -207;
pub const PSPCB_ADDREF: u32 = 0;
pub const PSPCB_CREATE: u32 = 2;
pub const PSPCB_RELEASE: u32 = 1;
pub const PSP_DEFAULT: u32 = 0;
pub const PSP_DLGINDIRECT: u32 = 1;
pub const PSP_HASHELP: u32 = 32;
pub const PSP_HIDEHEADER: u32 = 2048;
pub const PSP_PREMATURE: u32 = 1024;
pub const PSP_RTLREADING: u32 = 16;
pub const PSP_USECALLBACK: u32 = 128;
pub const PSP_USEFUSIONCONTEXT: u32 = 16384;
pub const PSP_USEHEADERSUBTITLE: u32 = 8192;
pub const PSP_USEHEADERTITLE: u32 = 4096;
pub const PSP_USEHICON: u32 = 2;
pub const PSP_USEICONID: u32 = 4;
pub const PSP_USEREFPARENT: u32 = 64;
pub const PSP_USETITLE: u32 = 8;
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1;
pub const PSWIZB_BACK: u32 = 1;
pub const PSWIZB_CANCEL: u32 = 16;
pub const PSWIZB_DISABLEDFINISH: u32 = 8;
pub const PSWIZB_FINISH: u32 = 4;
pub const PSWIZB_NEXT: u32 = 2;
pub const PSWIZB_RESTORE: u32 = 1;
pub const PSWIZB_SHOW: u32 = 0;
pub const PSWIZF_SETCOLOR: i32 = -1;
pub const WIZ_BODYCX: u32 = 184;
pub const WIZ_BODYX: u32 = 92;
pub const WIZ_CXBMP: u32 = 80;
pub const WIZ_CXDLG: u32 = 276;
pub const WIZ_CYDLG: u32 = 140;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _PSP(pub u8);
