#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ACMDM_BASE: u32 = 24576u32;
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
pub const ACMDM_USER: u32 = 16384u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [super::super::Foundation::CHAR; 32],
    pub szLongName: [super::super::Foundation::CHAR; 128],
    pub szCopyright: [super::super::Foundation::CHAR; 80],
    pub szLicensing: [super::super::Foundation::CHAR; 128],
    pub szFeatures: [super::super::Foundation::CHAR; 512],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for ACMDRIVERDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for ACMDRIVERDETAILSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::super::UI::WindowsAndMessaging::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::default::Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::PartialEq for ACMDRIVERDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::std::cmp::Eq for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::runtime::Abi for ACMDRIVERDETAILSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: super::super::Foundation::PSTR,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
    pub szFilter: [super::super::Foundation::CHAR; 128],
    pub pszName: super::super::Foundation::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: super::super::Foundation::PSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ::std::option::Option<ACMFILTERCHOOSEHOOKPROCA>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFILTERCHOOSEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFILTERCHOOSEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    umsg: u32,
    wparam: super::super::Foundation::WPARAM,
    lparam: super::super::Foundation::LPARAM,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    umsg: u32,
    wparam: super::super::Foundation::WPARAM,
    lparam: super::super::Foundation::LPARAM,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for ACMFILTERCHOOSEW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: super::super::Foundation::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: super::super::Foundation::PWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ::std::option::Option<ACMFILTERCHOOSEHOOKPROCW>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFILTERCHOOSEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFILTERCHOOSEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFILTERDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFILTERDETAILSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl ACMFILTERDETAILSW {}
impl ::std::default::Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ACMFILTERDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ACMFILTERDETAILSW {}
unsafe impl ::windows::runtime::Abi for ACMFILTERDETAILSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    pafd: *mut ACMFILTERDETAILSA,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    pafd: *mut ACMFILTERDETAILSW,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFILTERTAGDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFILTERTAGDETAILSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl ACMFILTERTAGDETAILSW {}
impl ::std::default::Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ACMFILTERTAGDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ACMFILTERTAGDETAILSW {}
unsafe impl ::windows::runtime::Abi for ACMFILTERTAGDETAILSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    paftd: *mut ACMFILTERTAGDETAILSA,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    paftd: *mut ACMFILTERTAGDETAILSW,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for ACMFORMATCHOOSEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: super::super::Foundation::PSTR,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
    pub szFormat: [super::super::Foundation::CHAR; 128],
    pub pszName: super::super::Foundation::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: super::super::Foundation::PSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ::std::option::Option<ACMFORMATCHOOSEHOOKPROCA>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFORMATCHOOSEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFORMATCHOOSEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    umsg: u32,
    wparam: super::super::Foundation::WPARAM,
    lparam: super::super::Foundation::LPARAM,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    umsg: u32,
    wparam: super::super::Foundation::WPARAM,
    lparam: super::super::Foundation::LPARAM,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for ACMFORMATCHOOSEW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: super::super::Foundation::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: super::super::Foundation::PWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ::std::option::Option<ACMFORMATCHOOSEHOOKPROCW>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFORMATCHOOSEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFORMATCHOOSEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFORMATDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFORMATDETAILSA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    pafd: *mut ACMFORMATDETAILSA,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    pafd: *mut tACMFORMATDETAILSW,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACMFORMATTAGDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACMFORMATTAGDETAILSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl ACMFORMATTAGDETAILSW {}
impl ::std::default::Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ACMFORMATTAGDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ACMFORMATTAGDETAILSW {}
unsafe impl ::windows::runtime::Abi for ACMFORMATTAGDETAILSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    paftd: *mut ACMFORMATTAGDETAILSA,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = unsafe extern "system" fn(
    hadid: HACMDRIVERID,
    paftd: *mut ACMFORMATTAGDETAILSW,
    dwinstance: usize,
    fdwsupport: u32,
) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
impl ACMSTREAMHEADER {}
impl ::std::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ACMSTREAMHEADER {}
unsafe impl ::windows::runtime::Abi for ACMSTREAMHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
pub const ACM_MPEG_COPYRIGHT: u32 = 2u32;
pub const ACM_MPEG_DUALCHANNEL: u32 = 4u32;
pub const ACM_MPEG_ID_MPEG1: u32 = 16u32;
pub const ACM_MPEG_JOINTSTEREO: u32 = 2u32;
pub const ACM_MPEG_LAYER1: u32 = 1u32;
pub const ACM_MPEG_LAYER2: u32 = 2u32;
pub const ACM_MPEG_LAYER3: u32 = 4u32;
pub const ACM_MPEG_ORIGINALHOME: u32 = 4u32;
pub const ACM_MPEG_PRIVATEBIT: u32 = 1u32;
pub const ACM_MPEG_PROTECTIONBIT: u32 = 8u32;
pub const ACM_MPEG_SINGLECHANNEL: u32 = 8u32;
pub const ACM_MPEG_STEREO: u32 = 1u32;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ADPCMCOEFSET {
    pub iCoef1: i16,
    pub iCoef2: i16,
}
impl ADPCMCOEFSET {}
impl ::std::default::Default for ADPCMCOEFSET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ADPCMCOEFSET {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ADPCMCOEFSET {}
unsafe impl ::windows::runtime::Abi for ADPCMCOEFSET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ADPCMEWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl ADPCMEWAVEFORMAT {}
impl ::std::default::Default for ADPCMEWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ADPCMEWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ADPCMEWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for ADPCMEWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wNumCoef: u16,
    pub aCoef: [ADPCMCOEFSET; 1],
}
impl ADPCMWAVEFORMAT {}
impl ::std::default::Default for ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for ADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct APTXWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl APTXWAVEFORMAT {}
impl ::std::default::Default for APTXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for APTXWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for APTXWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for APTXWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct AUDIOFILE_AF10WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl AUDIOFILE_AF10WAVEFORMAT {}
impl ::std::default::Default for AUDIOFILE_AF10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AUDIOFILE_AF10WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AUDIOFILE_AF10WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for AUDIOFILE_AF10WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct AUDIOFILE_AF36WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl AUDIOFILE_AF36WAVEFORMAT {}
impl ::std::default::Default for AUDIOFILE_AF36WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AUDIOFILE_AF36WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AUDIOFILE_AF36WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for AUDIOFILE_AF36WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct AUXCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AUXCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUXCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl AUXCAPS2W {}
impl ::std::default::Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AUXCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AUXCAPS2W {}
unsafe impl ::windows::runtime::Abi for AUXCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct AUXCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AUXCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUXCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl AUXCAPSW {}
impl ::std::default::Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for AUXCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for AUXCAPSW {}
unsafe impl ::windows::runtime::Abi for AUXCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const AUXCAPS_AUXIN: u32 = 2u32;
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
pub const AUXCAPS_VOLUME: u32 = 1u32;
pub const AUXDM_GETDEVCAPS: u32 = 4u32;
pub const AUXDM_GETNUMDEVS: u32 = 3u32;
pub const AUXDM_GETVOLUME: u32 = 5u32;
pub const AUXDM_SETVOLUME: u32 = 6u32;
pub const AUXM_INIT: u32 = 100u32;
pub const AUXM_INIT_EX: u32 = 104u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIBuildFilterA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszfilter: super::super::Foundation::PSTR,
    cbfilter: i32,
    fsaving: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIBuildFilterA(
                lpszfilter: super::super::Foundation::PSTR,
                cbfilter: i32,
                fsaving: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIBuildFilterA(
            ::std::mem::transmute(lpszfilter),
            ::std::mem::transmute(cbfilter),
            fsaving.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIBuildFilterW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpszfilter: super::super::Foundation::PWSTR,
    cbfilter: i32,
    fsaving: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIBuildFilterW(
                lpszfilter: super::super::Foundation::PWSTR,
                cbfilter: i32,
                fsaving: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIBuildFilterW(
            ::std::mem::transmute(lpszfilter),
            ::std::mem::transmute(cbfilter),
            fsaving.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AVICOMPRESSF_DATARATE: u32 = 2u32;
pub const AVICOMPRESSF_INTERLEAVE: u32 = 1u32;
pub const AVICOMPRESSF_KEYFRAMES: u32 = 4u32;
pub const AVICOMPRESSF_VALID: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct AVICOMPRESSOPTIONS {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwKeyFrameEvery: u32,
    pub dwQuality: u32,
    pub dwBytesPerSecond: u32,
    pub dwFlags: u32,
    pub lpFormat: *mut ::std::ffi::c_void,
    pub cbFormat: u32,
    pub lpParms: *mut ::std::ffi::c_void,
    pub cbParms: u32,
    pub dwInterleaveEvery: u32,
}
impl AVICOMPRESSOPTIONS {}
impl ::std::default::Default for AVICOMPRESSOPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AVICOMPRESSOPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AVICOMPRESSOPTIONS")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwKeyFrameEvery", &self.dwKeyFrameEvery)
            .field("dwQuality", &self.dwQuality)
            .field("dwBytesPerSecond", &self.dwBytesPerSecond)
            .field("dwFlags", &self.dwFlags)
            .field("lpFormat", &self.lpFormat)
            .field("cbFormat", &self.cbFormat)
            .field("lpParms", &self.lpParms)
            .field("cbParms", &self.cbParms)
            .field("dwInterleaveEvery", &self.dwInterleaveEvery)
            .finish()
    }
}
impl ::std::cmp::PartialEq for AVICOMPRESSOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwKeyFrameEvery == other.dwKeyFrameEvery
            && self.dwQuality == other.dwQuality
            && self.dwBytesPerSecond == other.dwBytesPerSecond
            && self.dwFlags == other.dwFlags
            && self.lpFormat == other.lpFormat
            && self.cbFormat == other.cbFormat
            && self.lpParms == other.lpParms
            && self.cbParms == other.cbParms
            && self.dwInterleaveEvery == other.dwInterleaveEvery
    }
}
impl ::std::cmp::Eq for AVICOMPRESSOPTIONS {}
unsafe impl ::windows::runtime::Abi for AVICOMPRESSOPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn AVIClearClipboard() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIClearClipboard() -> ::windows::runtime::HRESULT;
        }
        AVIClearClipboard().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AVIERR_OK: i32 = 0i32;
pub const AVIFILECAPS_ALLKEYFRAMES: u32 = 16u32;
pub const AVIFILECAPS_CANREAD: u32 = 1u32;
pub const AVIFILECAPS_CANWRITE: u32 = 2u32;
pub const AVIFILECAPS_NOCOMPRESSION: u32 = 32u32;
pub const AVIFILEHANDLER_CANACCEPTNONRGB: u32 = 4u32;
pub const AVIFILEHANDLER_CANREAD: u32 = 1u32;
pub const AVIFILEHANDLER_CANWRITE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AVIFILEINFOA {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl AVIFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AVIFILEINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for AVIFILEINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AVIFILEINFOA")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AVIFILEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.dwStreams == other.dwStreams
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwLength == other.dwLength
            && self.dwEditCount == other.dwEditCount
            && self.szFileType == other.szFileType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AVIFILEINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AVIFILEINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct AVIFILEINFOW {
    pub dwMaxBytesPerSec: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub dwStreams: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwLength: u32,
    pub dwEditCount: u32,
    pub szFileType: [u16; 64],
}
impl AVIFILEINFOW {}
impl ::std::default::Default for AVIFILEINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for AVIFILEINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AVIFILEINFOW")
            .field("dwMaxBytesPerSec", &self.dwMaxBytesPerSec)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("dwStreams", &self.dwStreams)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwLength", &self.dwLength)
            .field("dwEditCount", &self.dwEditCount)
            .field("szFileType", &self.szFileType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for AVIFILEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwMaxBytesPerSec == other.dwMaxBytesPerSec
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.dwStreams == other.dwStreams
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwLength == other.dwLength
            && self.dwEditCount == other.dwEditCount
            && self.szFileType == other.szFileType
    }
}
impl ::std::cmp::Eq for AVIFILEINFOW {}
unsafe impl ::windows::runtime::Abi for AVIFILEINFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const AVIFILEINFO_COPYRIGHTED: u32 = 131072u32;
pub const AVIFILEINFO_HASINDEX: u32 = 16u32;
pub const AVIFILEINFO_ISINTERLEAVED: u32 = 256u32;
pub const AVIFILEINFO_MUSTUSEINDEX: u32 = 32u32;
pub const AVIFILEINFO_WASCAPTUREFILE: u32 = 65536u32;
#[inline]
pub unsafe fn AVIFileAddRef<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileAddRef(pfile: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(AVIFileAddRef(pfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIFileCreateStreamA<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    ppavi: *mut ::std::option::Option<IAVIStream>,
    psi: *const AVISTREAMINFOA,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileCreateStreamA(
                pfile: ::windows::runtime::RawPtr,
                ppavi: *mut ::windows::runtime::RawPtr,
                psi: *const AVISTREAMINFOA,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileCreateStreamA(
            pfile.into_param().abi(),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(psi),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIFileCreateStreamW<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    ppavi: *mut ::std::option::Option<IAVIStream>,
    psi: *const AVISTREAMINFOW,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileCreateStreamW(
                pfile: ::windows::runtime::RawPtr,
                ppavi: *mut ::windows::runtime::RawPtr,
                psi: *const AVISTREAMINFOW,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileCreateStreamW(
            pfile.into_param().abi(),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(psi),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIFileEndRecord<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileEndRecord(pfile: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        AVIFileEndRecord(pfile.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIFileExit() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileExit();
        }
        ::std::mem::transmute(AVIFileExit())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIFileGetStream<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    ppavi: *mut ::std::option::Option<IAVIStream>,
    fcctype: u32,
    lparam: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileGetStream(
                pfile: ::windows::runtime::RawPtr,
                ppavi: *mut ::windows::runtime::RawPtr,
                fcctype: u32,
                lparam: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileGetStream(
            pfile.into_param().abi(),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(lparam),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIFileInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    pfi: *mut AVIFILEINFOA,
    lsize: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInfoA(
                pfile: ::windows::runtime::RawPtr,
                pfi: *mut AVIFILEINFOA,
                lsize: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileInfoA(
            pfile.into_param().abi(),
            ::std::mem::transmute(pfi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIFileInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    pfi: *mut AVIFILEINFOW,
    lsize: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInfoW(
                pfile: ::windows::runtime::RawPtr,
                pfi: *mut AVIFILEINFOW,
                lsize: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileInfoW(
            pfile.into_param().abi(),
            ::std::mem::transmute(pfi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIFileInit() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileInit();
        }
        ::std::mem::transmute(AVIFileInit())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIFileOpenA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    ppfile: *mut ::std::option::Option<IAVIFile>,
    szfile: Param1,
    umode: u32,
    lphandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileOpenA(
                ppfile: *mut ::windows::runtime::RawPtr,
                szfile: super::super::Foundation::PSTR,
                umode: u32,
                lphandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileOpenA(
            ::std::mem::transmute(ppfile),
            szfile.into_param().abi(),
            ::std::mem::transmute(umode),
            ::std::mem::transmute(lphandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIFileOpenW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    ppfile: *mut ::std::option::Option<IAVIFile>,
    szfile: Param1,
    umode: u32,
    lphandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileOpenW(
                ppfile: *mut ::windows::runtime::RawPtr,
                szfile: super::super::Foundation::PWSTR,
                umode: u32,
                lphandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileOpenW(
            ::std::mem::transmute(ppfile),
            szfile.into_param().abi(),
            ::std::mem::transmute(umode),
            ::std::mem::transmute(lphandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIFileReadData<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    ckid: u32,
    lpdata: *mut ::std::ffi::c_void,
    lpcbdata: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileReadData(
                pfile: ::windows::runtime::RawPtr,
                ckid: u32,
                lpdata: *mut ::std::ffi::c_void,
                lpcbdata: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileReadData(
            pfile.into_param().abi(),
            ::std::mem::transmute(ckid),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIFileRelease<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileRelease(pfile: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(AVIFileRelease(pfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIFileWriteData<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pfile: Param0,
    ckid: u32,
    lpdata: *const ::std::ffi::c_void,
    cbdata: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIFileWriteData(
                pfile: ::windows::runtime::RawPtr,
                ckid: u32,
                lpdata: *const ::std::ffi::c_void,
                cbdata: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIFileWriteData(
            pfile.into_param().abi(),
            ::std::mem::transmute(ckid),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AVIGETFRAMEF_BESTDISPLAYFMT: u32 = 1u32;
pub unsafe fn AVIGetFromClipboard() -> ::windows::runtime::Result<IAVIFile> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIGetFromClipboard(
                lppf: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAVIFile as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        AVIGetFromClipboard(&mut result__).from_abi::<IAVIFile>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AVIIF_CONTROLFRAME: i32 = 512i32;
pub const AVIIF_TWOCC: i32 = 2i32;
pub unsafe fn AVIMakeCompressedStream<'a, Param1: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    ppscompressed: *mut ::std::option::Option<IAVIStream>,
    ppssource: Param1,
    lpoptions: *const AVICOMPRESSOPTIONS,
    pclsidhandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeCompressedStream(
                ppscompressed: *mut ::windows::runtime::RawPtr,
                ppssource: ::windows::runtime::RawPtr,
                lpoptions: *const AVICOMPRESSOPTIONS,
                pclsidhandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIMakeCompressedStream(
            ::std::mem::transmute(ppscompressed),
            ppssource.into_param().abi(),
            ::std::mem::transmute(lpoptions),
            ::std::mem::transmute(pclsidhandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIMakeFileFromStreams(
    ppfile: *mut ::std::option::Option<IAVIFile>,
    nstreams: i32,
    papstreams: *const ::std::option::Option<IAVIStream>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeFileFromStreams(
                ppfile: *mut ::windows::runtime::RawPtr,
                nstreams: i32,
                papstreams: *const ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIMakeFileFromStreams(
            ::std::mem::transmute(ppfile),
            ::std::mem::transmute(nstreams),
            ::std::mem::transmute(papstreams),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIMakeStreamFromClipboard<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    cfformat: u32,
    hglobal: Param1,
) -> ::windows::runtime::Result<IAVIStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIMakeStreamFromClipboard(
                cfformat: u32,
                hglobal: super::super::Foundation::HANDLE,
                ppstream: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAVIStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        AVIMakeStreamFromClipboard(
            ::std::mem::transmute(cfformat),
            hglobal.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IAVIStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIPutFileOnClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIFile>>(
    pf: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIPutFileOnClipboard(pf: ::windows::runtime::RawPtr)
                -> ::windows::runtime::HRESULT;
        }
        AVIPutFileOnClipboard(pf.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type AVISAVECALLBACK = unsafe extern "system" fn(param0: i32) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AVISTREAMINFOA {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl AVISTREAMINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AVISTREAMINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for AVISTREAMINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AVISTREAMINFOA")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AVISTREAMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.wPriority == other.wPriority
            && self.wLanguage == other.wLanguage
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwStart == other.dwStart
            && self.dwLength == other.dwLength
            && self.dwInitialFrames == other.dwInitialFrames
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwQuality == other.dwQuality
            && self.dwSampleSize == other.dwSampleSize
            && self.rcFrame == other.rcFrame
            && self.dwEditCount == other.dwEditCount
            && self.dwFormatChangeCount == other.dwFormatChangeCount
            && self.szName == other.szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AVISTREAMINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AVISTREAMINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AVISTREAMINFOW {
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwCaps: u32,
    pub wPriority: u16,
    pub wLanguage: u16,
    pub dwScale: u32,
    pub dwRate: u32,
    pub dwStart: u32,
    pub dwLength: u32,
    pub dwInitialFrames: u32,
    pub dwSuggestedBufferSize: u32,
    pub dwQuality: u32,
    pub dwSampleSize: u32,
    pub rcFrame: super::super::Foundation::RECT,
    pub dwEditCount: u32,
    pub dwFormatChangeCount: u32,
    pub szName: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl AVISTREAMINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AVISTREAMINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for AVISTREAMINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AVISTREAMINFOW")
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwCaps", &self.dwCaps)
            .field("wPriority", &self.wPriority)
            .field("wLanguage", &self.wLanguage)
            .field("dwScale", &self.dwScale)
            .field("dwRate", &self.dwRate)
            .field("dwStart", &self.dwStart)
            .field("dwLength", &self.dwLength)
            .field("dwInitialFrames", &self.dwInitialFrames)
            .field("dwSuggestedBufferSize", &self.dwSuggestedBufferSize)
            .field("dwQuality", &self.dwQuality)
            .field("dwSampleSize", &self.dwSampleSize)
            .field("rcFrame", &self.rcFrame)
            .field("dwEditCount", &self.dwEditCount)
            .field("dwFormatChangeCount", &self.dwFormatChangeCount)
            .field("szName", &self.szName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AVISTREAMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwCaps == other.dwCaps
            && self.wPriority == other.wPriority
            && self.wLanguage == other.wLanguage
            && self.dwScale == other.dwScale
            && self.dwRate == other.dwRate
            && self.dwStart == other.dwStart
            && self.dwLength == other.dwLength
            && self.dwInitialFrames == other.dwInitialFrames
            && self.dwSuggestedBufferSize == other.dwSuggestedBufferSize
            && self.dwQuality == other.dwQuality
            && self.dwSampleSize == other.dwSampleSize
            && self.rcFrame == other.rcFrame
            && self.dwEditCount == other.dwEditCount
            && self.dwFormatChangeCount == other.dwFormatChangeCount
            && self.szName == other.szName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AVISTREAMINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AVISTREAMINFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const AVISTREAMINFO_DISABLED: u32 = 1u32;
pub const AVISTREAMINFO_FORMATCHANGES: u32 = 65536u32;
pub const AVISTREAMREAD_CONVENIENT: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVISaveA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param4: ::windows::runtime::IntoParam<'a, IAVIStream>,
>(
    szfile: Param0,
    pclsidhandler: *const ::windows::runtime::GUID,
    lpfncallback: ::std::option::Option<AVISAVECALLBACK>,
    nstreams: i32,
    pfile: Param4,
    lpoptions: *const AVICOMPRESSOPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveA(
                szfile: super::super::Foundation::PSTR,
                pclsidhandler: *const ::windows::runtime::GUID,
                lpfncallback: ::windows::runtime::RawPtr,
                nstreams: i32,
                pfile: ::windows::runtime::RawPtr,
                lpoptions: *const AVICOMPRESSOPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        AVISaveA(
            szfile.into_param().abi(),
            ::std::mem::transmute(pclsidhandler),
            ::std::mem::transmute(lpfncallback),
            ::std::mem::transmute(nstreams),
            pfile.into_param().abi(),
            ::std::mem::transmute(lpoptions),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AVISaveOptions<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    uiflags: u32,
    nstreams: i32,
    ppavi: *const ::std::option::Option<IAVIStream>,
    plpoptions: *mut *mut AVICOMPRESSOPTIONS,
) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveOptions(
                hwnd: super::super::Foundation::HWND,
                uiflags: u32,
                nstreams: i32,
                ppavi: *const ::windows::runtime::RawPtr,
                plpoptions: *mut *mut AVICOMPRESSOPTIONS,
            ) -> isize;
        }
        ::std::mem::transmute(AVISaveOptions(
            hwnd.into_param().abi(),
            ::std::mem::transmute(uiflags),
            ::std::mem::transmute(nstreams),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(plpoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVISaveOptionsFree(
    nstreams: i32,
    plpoptions: *const *const AVICOMPRESSOPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveOptionsFree(
                nstreams: i32,
                plpoptions: *const *const AVICOMPRESSOPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        AVISaveOptionsFree(
            ::std::mem::transmute(nstreams),
            ::std::mem::transmute(plpoptions),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVISaveVA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    szfile: Param0,
    pclsidhandler: *const ::windows::runtime::GUID,
    lpfncallback: ::std::option::Option<AVISAVECALLBACK>,
    nstreams: i32,
    ppavi: *const ::std::option::Option<IAVIStream>,
    plpoptions: *const *const AVICOMPRESSOPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveVA(
                szfile: super::super::Foundation::PSTR,
                pclsidhandler: *const ::windows::runtime::GUID,
                lpfncallback: ::windows::runtime::RawPtr,
                nstreams: i32,
                ppavi: *const ::windows::runtime::RawPtr,
                plpoptions: *const *const AVICOMPRESSOPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        AVISaveVA(
            szfile.into_param().abi(),
            ::std::mem::transmute(pclsidhandler),
            ::std::mem::transmute(lpfncallback),
            ::std::mem::transmute(nstreams),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(plpoptions),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVISaveVW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    szfile: Param0,
    pclsidhandler: *const ::windows::runtime::GUID,
    lpfncallback: ::std::option::Option<AVISAVECALLBACK>,
    nstreams: i32,
    ppavi: *const ::std::option::Option<IAVIStream>,
    plpoptions: *const *const AVICOMPRESSOPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveVW(
                szfile: super::super::Foundation::PWSTR,
                pclsidhandler: *const ::windows::runtime::GUID,
                lpfncallback: ::windows::runtime::RawPtr,
                nstreams: i32,
                ppavi: *const ::windows::runtime::RawPtr,
                plpoptions: *const *const AVICOMPRESSOPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        AVISaveVW(
            szfile.into_param().abi(),
            ::std::mem::transmute(pclsidhandler),
            ::std::mem::transmute(lpfncallback),
            ::std::mem::transmute(nstreams),
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(plpoptions),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVISaveW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param4: ::windows::runtime::IntoParam<'a, IAVIStream>,
>(
    szfile: Param0,
    pclsidhandler: *const ::windows::runtime::GUID,
    lpfncallback: ::std::option::Option<AVISAVECALLBACK>,
    nstreams: i32,
    pfile: Param4,
    lpoptions: *const AVICOMPRESSOPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVISaveW(
                szfile: super::super::Foundation::PWSTR,
                pclsidhandler: *const ::windows::runtime::GUID,
                lpfncallback: ::windows::runtime::RawPtr,
                nstreams: i32,
                pfile: ::windows::runtime::RawPtr,
                lpoptions: *const AVICOMPRESSOPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        AVISaveW(
            szfile.into_param().abi(),
            ::std::mem::transmute(pclsidhandler),
            ::std::mem::transmute(lpfncallback),
            ::std::mem::transmute(nstreams),
            pfile.into_param().abi(),
            ::std::mem::transmute(lpoptions),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamAddRef<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamAddRef(pavi: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(AVIStreamAddRef(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamBeginStreaming<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lstart: i32,
    lend: i32,
    lrate: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamBeginStreaming(
                pavi: ::windows::runtime::RawPtr,
                lstart: i32,
                lend: i32,
                lrate: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamBeginStreaming(
            pavi.into_param().abi(),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lend),
            ::std::mem::transmute(lrate),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamCreate(
    ppavi: *mut ::std::option::Option<IAVIStream>,
    lparam1: i32,
    lparam2: i32,
    pclsidhandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamCreate(
                ppavi: *mut ::windows::runtime::RawPtr,
                lparam1: i32,
                lparam2: i32,
                pclsidhandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamCreate(
            ::std::mem::transmute(ppavi),
            ::std::mem::transmute(lparam1),
            ::std::mem::transmute(lparam2),
            ::std::mem::transmute(pclsidhandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamEndStreaming<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamEndStreaming(
                pavi: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamEndStreaming(pavi.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamFindSample<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpos: i32,
    lflags: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamFindSample(pavi: ::windows::runtime::RawPtr, lpos: i32, lflags: i32)
                -> i32;
        }
        ::std::mem::transmute(AVIStreamFindSample(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamGetFrame<'a, Param0: ::windows::runtime::IntoParam<'a, IGetFrame>>(
    pg: Param0,
    lpos: i32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrame(
                pg: ::windows::runtime::RawPtr,
                lpos: i32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(AVIStreamGetFrame(
            pg.into_param().abi(),
            ::std::mem::transmute(lpos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamGetFrameClose<'a, Param0: ::windows::runtime::IntoParam<'a, IGetFrame>>(
    pg: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrameClose(
                pg: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamGetFrameClose(pg.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AVIStreamGetFrameOpen<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpbiwanted: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
) -> ::std::option::Option<IGetFrame> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamGetFrameOpen(
                pavi: ::windows::runtime::RawPtr,
                lpbiwanted: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
            ) -> ::std::option::Option<IGetFrame>;
        }
        ::std::mem::transmute(AVIStreamGetFrameOpen(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpbiwanted),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIStreamInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    psi: *mut AVISTREAMINFOA,
    lsize: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamInfoA(
                pavi: ::windows::runtime::RawPtr,
                psi: *mut AVISTREAMINFOA,
                lsize: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamInfoA(
            pavi.into_param().abi(),
            ::std::mem::transmute(psi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIStreamInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    psi: *mut AVISTREAMINFOW,
    lsize: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamInfoW(
                pavi: ::windows::runtime::RawPtr,
                psi: *mut AVISTREAMINFOW,
                lsize: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamInfoW(
            pavi.into_param().abi(),
            ::std::mem::transmute(psi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamLength<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamLength(pavi: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(AVIStreamLength(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIStreamOpenFromFileA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    ppavi: *mut ::std::option::Option<IAVIStream>,
    szfile: Param1,
    fcctype: u32,
    lparam: i32,
    mode: u32,
    pclsidhandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamOpenFromFileA(
                ppavi: *mut ::windows::runtime::RawPtr,
                szfile: super::super::Foundation::PSTR,
                fcctype: u32,
                lparam: i32,
                mode: u32,
                pclsidhandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamOpenFromFileA(
            ::std::mem::transmute(ppavi),
            szfile.into_param().abi(),
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(mode),
            ::std::mem::transmute(pclsidhandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn AVIStreamOpenFromFileW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    ppavi: *mut ::std::option::Option<IAVIStream>,
    szfile: Param1,
    fcctype: u32,
    lparam: i32,
    mode: u32,
    pclsidhandler: *const ::windows::runtime::GUID,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamOpenFromFileW(
                ppavi: *mut ::windows::runtime::RawPtr,
                szfile: super::super::Foundation::PWSTR,
                fcctype: u32,
                lparam: i32,
                mode: u32,
                pclsidhandler: *const ::windows::runtime::GUID,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamOpenFromFileW(
            ::std::mem::transmute(ppavi),
            szfile.into_param().abi(),
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(mode),
            ::std::mem::transmute(pclsidhandler),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamRead<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lstart: i32,
    lsamples: i32,
    lpbuffer: *mut ::std::ffi::c_void,
    cbbuffer: i32,
    plbytes: *mut i32,
    plsamples: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamRead(
                pavi: ::windows::runtime::RawPtr,
                lstart: i32,
                lsamples: i32,
                lpbuffer: *mut ::std::ffi::c_void,
                cbbuffer: i32,
                plbytes: *mut i32,
                plsamples: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamRead(
            pavi.into_param().abi(),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lsamples),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(cbbuffer),
            ::std::mem::transmute(plbytes),
            ::std::mem::transmute(plsamples),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamReadData<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    fcc: u32,
    lp: *mut ::std::ffi::c_void,
    lpcb: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamReadData(
                pavi: ::windows::runtime::RawPtr,
                fcc: u32,
                lp: *mut ::std::ffi::c_void,
                lpcb: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamReadData(
            pavi.into_param().abi(),
            ::std::mem::transmute(fcc),
            ::std::mem::transmute(lp),
            ::std::mem::transmute(lpcb),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamReadFormat<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpos: i32,
    lpformat: *mut ::std::ffi::c_void,
    lpcbformat: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamReadFormat(
                pavi: ::windows::runtime::RawPtr,
                lpos: i32,
                lpformat: *mut ::std::ffi::c_void,
                lpcbformat: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamReadFormat(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lpformat),
            ::std::mem::transmute(lpcbformat),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamRelease<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamRelease(pavi: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(AVIStreamRelease(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamSampleToTime<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lsample: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamSampleToTime(pavi: ::windows::runtime::RawPtr, lsample: i32) -> i32;
        }
        ::std::mem::transmute(AVIStreamSampleToTime(
            pavi.into_param().abi(),
            ::std::mem::transmute(lsample),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamSetFormat<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpos: i32,
    lpformat: *const ::std::ffi::c_void,
    cbformat: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamSetFormat(
                pavi: ::windows::runtime::RawPtr,
                lpos: i32,
                lpformat: *const ::std::ffi::c_void,
                cbformat: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamSetFormat(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lpformat),
            ::std::mem::transmute(cbformat),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamStart<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamStart(pavi: ::windows::runtime::RawPtr) -> i32;
        }
        ::std::mem::transmute(AVIStreamStart(pavi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn AVIStreamTimeToSample<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    ltime: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamTimeToSample(pavi: ::windows::runtime::RawPtr, ltime: i32) -> i32;
        }
        ::std::mem::transmute(AVIStreamTimeToSample(
            pavi.into_param().abi(),
            ::std::mem::transmute(ltime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamWrite<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lstart: i32,
    lsamples: i32,
    lpbuffer: *const ::std::ffi::c_void,
    cbbuffer: i32,
    dwflags: u32,
    plsampwritten: *mut i32,
    plbyteswritten: *mut i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamWrite(
                pavi: ::windows::runtime::RawPtr,
                lstart: i32,
                lsamples: i32,
                lpbuffer: *const ::std::ffi::c_void,
                cbbuffer: i32,
                dwflags: u32,
                plsampwritten: *mut i32,
                plbyteswritten: *mut i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamWrite(
            pavi.into_param().abi(),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lsamples),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(cbbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(plsampwritten),
            ::std::mem::transmute(plbyteswritten),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn AVIStreamWriteData<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    fcc: u32,
    lp: *const ::std::ffi::c_void,
    cb: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AVIStreamWriteData(
                pavi: ::windows::runtime::RawPtr,
                fcc: u32,
                lp: *const ::std::ffi::c_void,
                cb: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        AVIStreamWriteData(
            pavi.into_param().abi(),
            ::std::mem::transmute(fcc),
            ::std::mem::transmute(lp),
            ::std::mem::transmute(cb),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const AVSTREAMMASTER_AUDIO: u32 = 0u32;
pub const AVSTREAMMASTER_NONE: u32 = 1u32;
pub const BI_1632: u32 = 842217009u32;
#[cfg(feature = "Win32_Foundation")]
pub type CAPCONTROLCALLBACK = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    nstate: i32,
) -> super::super::Foundation::LRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPDRIVERCAPS {
    pub wDeviceIndex: u32,
    pub fHasOverlay: super::super::Foundation::BOOL,
    pub fHasDlgVideoSource: super::super::Foundation::BOOL,
    pub fHasDlgVideoFormat: super::super::Foundation::BOOL,
    pub fHasDlgVideoDisplay: super::super::Foundation::BOOL,
    pub fCaptureInitialized: super::super::Foundation::BOOL,
    pub fDriverSuppliesPalettes: super::super::Foundation::BOOL,
    pub hVideoIn: super::super::Foundation::HANDLE,
    pub hVideoOut: super::super::Foundation::HANDLE,
    pub hVideoExtIn: super::super::Foundation::HANDLE,
    pub hVideoExtOut: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CAPDRIVERCAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAPDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAPDRIVERCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPDRIVERCAPS")
            .field("wDeviceIndex", &self.wDeviceIndex)
            .field("fHasOverlay", &self.fHasOverlay)
            .field("fHasDlgVideoSource", &self.fHasDlgVideoSource)
            .field("fHasDlgVideoFormat", &self.fHasDlgVideoFormat)
            .field("fHasDlgVideoDisplay", &self.fHasDlgVideoDisplay)
            .field("fCaptureInitialized", &self.fCaptureInitialized)
            .field("fDriverSuppliesPalettes", &self.fDriverSuppliesPalettes)
            .field("hVideoIn", &self.hVideoIn)
            .field("hVideoOut", &self.hVideoOut)
            .field("hVideoExtIn", &self.hVideoExtIn)
            .field("hVideoExtOut", &self.hVideoExtOut)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAPDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wDeviceIndex == other.wDeviceIndex
            && self.fHasOverlay == other.fHasOverlay
            && self.fHasDlgVideoSource == other.fHasDlgVideoSource
            && self.fHasDlgVideoFormat == other.fHasDlgVideoFormat
            && self.fHasDlgVideoDisplay == other.fHasDlgVideoDisplay
            && self.fCaptureInitialized == other.fCaptureInitialized
            && self.fDriverSuppliesPalettes == other.fDriverSuppliesPalettes
            && self.hVideoIn == other.hVideoIn
            && self.hVideoOut == other.hVideoOut
            && self.hVideoExtIn == other.hVideoExtIn
            && self.hVideoExtOut == other.hVideoExtOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAPDRIVERCAPS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAPDRIVERCAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type CAPERRORCALLBACKA = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    nid: i32,
    lpsz: super::super::Foundation::PSTR,
) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CAPERRORCALLBACKW = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    nid: i32,
    lpsz: super::super::Foundation::PWSTR,
) -> super::super::Foundation::LRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CAPINFOCHUNK {
    pub fccInfoID: u32,
    pub lpData: *mut ::std::ffi::c_void,
    pub cbData: i32,
}
impl CAPINFOCHUNK {}
impl ::std::default::Default for CAPINFOCHUNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CAPINFOCHUNK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPINFOCHUNK")
            .field("fccInfoID", &self.fccInfoID)
            .field("lpData", &self.lpData)
            .field("cbData", &self.cbData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CAPINFOCHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.fccInfoID == other.fccInfoID
            && self.lpData == other.lpData
            && self.cbData == other.cbData
    }
}
impl ::std::cmp::Eq for CAPINFOCHUNK {}
unsafe impl ::windows::runtime::Abi for CAPINFOCHUNK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CAPSTATUS {
    pub uiImageWidth: u32,
    pub uiImageHeight: u32,
    pub fLiveWindow: super::super::Foundation::BOOL,
    pub fOverlayWindow: super::super::Foundation::BOOL,
    pub fScale: super::super::Foundation::BOOL,
    pub ptScroll: super::super::Foundation::POINT,
    pub fUsingDefaultPalette: super::super::Foundation::BOOL,
    pub fAudioHardware: super::super::Foundation::BOOL,
    pub fCapFileExists: super::super::Foundation::BOOL,
    pub dwCurrentVideoFrame: u32,
    pub dwCurrentVideoFramesDropped: u32,
    pub dwCurrentWaveSamples: u32,
    pub dwCurrentTimeElapsedMS: u32,
    pub hPalCurrent: super::super::Graphics::Gdi::HPALETTE,
    pub fCapturingNow: super::super::Foundation::BOOL,
    pub dwReturn: u32,
    pub wNumVideoAllocated: u32,
    pub wNumAudioAllocated: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CAPSTATUS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CAPSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CAPSTATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPSTATUS")
            .field("uiImageWidth", &self.uiImageWidth)
            .field("uiImageHeight", &self.uiImageHeight)
            .field("fLiveWindow", &self.fLiveWindow)
            .field("fOverlayWindow", &self.fOverlayWindow)
            .field("fScale", &self.fScale)
            .field("ptScroll", &self.ptScroll)
            .field("fUsingDefaultPalette", &self.fUsingDefaultPalette)
            .field("fAudioHardware", &self.fAudioHardware)
            .field("fCapFileExists", &self.fCapFileExists)
            .field("dwCurrentVideoFrame", &self.dwCurrentVideoFrame)
            .field(
                "dwCurrentVideoFramesDropped",
                &self.dwCurrentVideoFramesDropped,
            )
            .field("dwCurrentWaveSamples", &self.dwCurrentWaveSamples)
            .field("dwCurrentTimeElapsedMS", &self.dwCurrentTimeElapsedMS)
            .field("hPalCurrent", &self.hPalCurrent)
            .field("fCapturingNow", &self.fCapturingNow)
            .field("dwReturn", &self.dwReturn)
            .field("wNumVideoAllocated", &self.wNumVideoAllocated)
            .field("wNumAudioAllocated", &self.wNumAudioAllocated)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CAPSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.uiImageWidth == other.uiImageWidth
            && self.uiImageHeight == other.uiImageHeight
            && self.fLiveWindow == other.fLiveWindow
            && self.fOverlayWindow == other.fOverlayWindow
            && self.fScale == other.fScale
            && self.ptScroll == other.ptScroll
            && self.fUsingDefaultPalette == other.fUsingDefaultPalette
            && self.fAudioHardware == other.fAudioHardware
            && self.fCapFileExists == other.fCapFileExists
            && self.dwCurrentVideoFrame == other.dwCurrentVideoFrame
            && self.dwCurrentVideoFramesDropped == other.dwCurrentVideoFramesDropped
            && self.dwCurrentWaveSamples == other.dwCurrentWaveSamples
            && self.dwCurrentTimeElapsedMS == other.dwCurrentTimeElapsedMS
            && self.hPalCurrent == other.hPalCurrent
            && self.fCapturingNow == other.fCapturingNow
            && self.dwReturn == other.dwReturn
            && self.wNumVideoAllocated == other.wNumVideoAllocated
            && self.wNumAudioAllocated == other.wNumAudioAllocated
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CAPSTATUS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CAPSTATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type CAPSTATUSCALLBACKA = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    nid: i32,
    lpsz: super::super::Foundation::PSTR,
) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CAPSTATUSCALLBACKW = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    nid: i32,
    lpsz: super::super::Foundation::PWSTR,
) -> super::super::Foundation::LRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CAPTUREPARMS {
    pub dwRequestMicroSecPerFrame: u32,
    pub fMakeUserHitOKToCapture: super::super::Foundation::BOOL,
    pub wPercentDropForError: u32,
    pub fYield: super::super::Foundation::BOOL,
    pub dwIndexSize: u32,
    pub wChunkGranularity: u32,
    pub fUsingDOSMemory: super::super::Foundation::BOOL,
    pub wNumVideoRequested: u32,
    pub fCaptureAudio: super::super::Foundation::BOOL,
    pub wNumAudioRequested: u32,
    pub vKeyAbort: u32,
    pub fAbortLeftMouse: super::super::Foundation::BOOL,
    pub fAbortRightMouse: super::super::Foundation::BOOL,
    pub fLimitEnabled: super::super::Foundation::BOOL,
    pub wTimeLimit: u32,
    pub fMCIControl: super::super::Foundation::BOOL,
    pub fStepMCIDevice: super::super::Foundation::BOOL,
    pub dwMCIStartTime: u32,
    pub dwMCIStopTime: u32,
    pub fStepCaptureAt2x: super::super::Foundation::BOOL,
    pub wStepCaptureAverageFrames: u32,
    pub dwAudioBufferSize: u32,
    pub fDisableWriteCache: super::super::Foundation::BOOL,
    pub AVStreamMaster: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CAPTUREPARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CAPTUREPARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CAPTUREPARMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CAPTUREPARMS")
            .field("dwRequestMicroSecPerFrame", &self.dwRequestMicroSecPerFrame)
            .field("fMakeUserHitOKToCapture", &self.fMakeUserHitOKToCapture)
            .field("wPercentDropForError", &self.wPercentDropForError)
            .field("fYield", &self.fYield)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("wChunkGranularity", &self.wChunkGranularity)
            .field("fUsingDOSMemory", &self.fUsingDOSMemory)
            .field("wNumVideoRequested", &self.wNumVideoRequested)
            .field("fCaptureAudio", &self.fCaptureAudio)
            .field("wNumAudioRequested", &self.wNumAudioRequested)
            .field("vKeyAbort", &self.vKeyAbort)
            .field("fAbortLeftMouse", &self.fAbortLeftMouse)
            .field("fAbortRightMouse", &self.fAbortRightMouse)
            .field("fLimitEnabled", &self.fLimitEnabled)
            .field("wTimeLimit", &self.wTimeLimit)
            .field("fMCIControl", &self.fMCIControl)
            .field("fStepMCIDevice", &self.fStepMCIDevice)
            .field("dwMCIStartTime", &self.dwMCIStartTime)
            .field("dwMCIStopTime", &self.dwMCIStopTime)
            .field("fStepCaptureAt2x", &self.fStepCaptureAt2x)
            .field("wStepCaptureAverageFrames", &self.wStepCaptureAverageFrames)
            .field("dwAudioBufferSize", &self.dwAudioBufferSize)
            .field("fDisableWriteCache", &self.fDisableWriteCache)
            .field("AVStreamMaster", &self.AVStreamMaster)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CAPTUREPARMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRequestMicroSecPerFrame == other.dwRequestMicroSecPerFrame
            && self.fMakeUserHitOKToCapture == other.fMakeUserHitOKToCapture
            && self.wPercentDropForError == other.wPercentDropForError
            && self.fYield == other.fYield
            && self.dwIndexSize == other.dwIndexSize
            && self.wChunkGranularity == other.wChunkGranularity
            && self.fUsingDOSMemory == other.fUsingDOSMemory
            && self.wNumVideoRequested == other.wNumVideoRequested
            && self.fCaptureAudio == other.fCaptureAudio
            && self.wNumAudioRequested == other.wNumAudioRequested
            && self.vKeyAbort == other.vKeyAbort
            && self.fAbortLeftMouse == other.fAbortLeftMouse
            && self.fAbortRightMouse == other.fAbortRightMouse
            && self.fLimitEnabled == other.fLimitEnabled
            && self.wTimeLimit == other.wTimeLimit
            && self.fMCIControl == other.fMCIControl
            && self.fStepMCIDevice == other.fStepMCIDevice
            && self.dwMCIStartTime == other.dwMCIStartTime
            && self.dwMCIStopTime == other.dwMCIStopTime
            && self.fStepCaptureAt2x == other.fStepCaptureAt2x
            && self.wStepCaptureAverageFrames == other.wStepCaptureAverageFrames
            && self.dwAudioBufferSize == other.dwAudioBufferSize
            && self.fDisableWriteCache == other.fDisableWriteCache
            && self.AVStreamMaster == other.AVStreamMaster
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CAPTUREPARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CAPTUREPARMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type CAPVIDEOCALLBACK = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    lpvhdr: *const VIDEOHDR,
) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CAPWAVECALLBACK = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    lpwhdr: *const WAVEHDR,
) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type CAPYIELDCALLBACK = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
) -> super::super::Foundation::LRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CHANNEL_CAPS {
    pub dwFlags: u32,
    pub dwSrcRectXMod: u32,
    pub dwSrcRectYMod: u32,
    pub dwSrcRectWidthMod: u32,
    pub dwSrcRectHeightMod: u32,
    pub dwDstRectXMod: u32,
    pub dwDstRectYMod: u32,
    pub dwDstRectWidthMod: u32,
    pub dwDstRectHeightMod: u32,
}
impl CHANNEL_CAPS {}
impl ::std::default::Default for CHANNEL_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CHANNEL_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHANNEL_CAPS")
            .field("dwFlags", &self.dwFlags)
            .field("dwSrcRectXMod", &self.dwSrcRectXMod)
            .field("dwSrcRectYMod", &self.dwSrcRectYMod)
            .field("dwSrcRectWidthMod", &self.dwSrcRectWidthMod)
            .field("dwSrcRectHeightMod", &self.dwSrcRectHeightMod)
            .field("dwDstRectXMod", &self.dwDstRectXMod)
            .field("dwDstRectYMod", &self.dwDstRectYMod)
            .field("dwDstRectWidthMod", &self.dwDstRectWidthMod)
            .field("dwDstRectHeightMod", &self.dwDstRectHeightMod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for CHANNEL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.dwSrcRectXMod == other.dwSrcRectXMod
            && self.dwSrcRectYMod == other.dwSrcRectYMod
            && self.dwSrcRectWidthMod == other.dwSrcRectWidthMod
            && self.dwSrcRectHeightMod == other.dwSrcRectHeightMod
            && self.dwDstRectXMod == other.dwDstRectXMod
            && self.dwDstRectYMod == other.dwDstRectYMod
            && self.dwDstRectWidthMod == other.dwDstRectWidthMod
            && self.dwDstRectHeightMod == other.dwDstRectHeightMod
    }
}
impl ::std::cmp::Eq for CHANNEL_CAPS {}
unsafe impl ::windows::runtime::Abi for CHANNEL_CAPS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CLSID_AVIFile: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(131072, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
pub const CLSID_AVISimpleUnMarshal: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(131081, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct COMPVARS {
    pub cbSize: i32,
    pub dwFlags: u32,
    pub hic: HIC,
    pub fccType: u32,
    pub fccHandler: u32,
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpbiOut: *mut super::super::Graphics::Gdi::BITMAPINFO,
    pub lpBitsOut: *mut ::std::ffi::c_void,
    pub lpBitsPrev: *mut ::std::ffi::c_void,
    pub lFrame: i32,
    pub lKey: i32,
    pub lDataRate: i32,
    pub lQ: i32,
    pub lKeyCount: i32,
    pub lpState: *mut ::std::ffi::c_void,
    pub cbState: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl COMPVARS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for COMPVARS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for COMPVARS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPVARS")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hic", &self.hic)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("lpbiIn", &self.lpbiIn)
            .field("lpbiOut", &self.lpbiOut)
            .field("lpBitsOut", &self.lpBitsOut)
            .field("lpBitsPrev", &self.lpBitsPrev)
            .field("lFrame", &self.lFrame)
            .field("lKey", &self.lKey)
            .field("lDataRate", &self.lDataRate)
            .field("lQ", &self.lQ)
            .field("lKeyCount", &self.lKeyCount)
            .field("lpState", &self.lpState)
            .field("cbState", &self.cbState)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for COMPVARS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.hic == other.hic
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.lpbiIn == other.lpbiIn
            && self.lpbiOut == other.lpbiOut
            && self.lpBitsOut == other.lpBitsOut
            && self.lpBitsPrev == other.lpBitsPrev
            && self.lFrame == other.lFrame
            && self.lKey == other.lKey
            && self.lDataRate == other.lDataRate
            && self.lQ == other.lQ
            && self.lKeyCount == other.lKeyCount
            && self.lpState == other.lpState
            && self.cbState == other.cbState
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for COMPVARS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for COMPVARS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CONTRESCR10WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl CONTRESCR10WAVEFORMAT {}
impl ::std::default::Default for CONTRESCR10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CONTRESCR10WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CONTRESCR10WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CONTRESCR10WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CONTRESVQLPCWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl CONTRESVQLPCWAVEFORMAT {}
impl ::std::default::Default for CONTRESVQLPCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CONTRESVQLPCWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CONTRESVQLPCWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CONTRESVQLPCWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CONTROLCALLBACK_CAPTURING: u32 = 2u32;
pub const CONTROLCALLBACK_PREROLL: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CREATIVEADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl CREATIVEADPCMWAVEFORMAT {}
impl ::std::default::Default for CREATIVEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CREATIVEADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CREATIVEADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CREATIVEADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CREATIVEFASTSPEECH10WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl CREATIVEFASTSPEECH10WAVEFORMAT {}
impl ::std::default::Default for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CREATIVEFASTSPEECH10WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CREATIVEFASTSPEECH10WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CREATIVEFASTSPEECH10WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct CREATIVEFASTSPEECH8WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl CREATIVEFASTSPEECH8WAVEFORMAT {}
impl ::std::default::Default for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CREATIVEFASTSPEECH8WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CREATIVEFASTSPEECH8WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CREATIVEFASTSPEECH8WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CRYSTAL_NET_SFM_CODEC: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CSIMAADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl CSIMAADPCMWAVEFORMAT {}
impl ::std::default::Default for CSIMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CSIMAADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CSIMAADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for CSIMAADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseDriver<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDRVR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hdriver: Param0,
    lparam1: Param1,
    lparam2: Param2,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseDriver(
                hdriver: HDRVR,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(CloseDriver(
            hdriver.into_param().abi(),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn CreateEditableStream<'a, Param1: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    ppseditable: *mut ::std::option::Option<IAVIStream>,
    pssource: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEditableStream(
                ppseditable: *mut ::windows::runtime::RawPtr,
                pssource: ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        CreateEditableStream(
            ::std::mem::transmute(ppseditable),
            pssource.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DCB_EVENT: u32 = 5u32;
pub const DCB_FUNCTION: u32 = 3u32;
pub const DCB_NOSWITCH: u32 = 8u32;
pub const DCB_NULL: u32 = 0u32;
pub const DCB_TASK: u32 = 2u32;
pub const DCB_TYPEMASK: u32 = 7u32;
pub const DCB_WINDOW: u32 = 1u32;
pub const DDF_0001: u32 = 1u32;
pub const DDF_2000: u32 = 8192u32;
pub const DDF_ANIMATE: u32 = 32u32;
pub const DDF_BACKGROUNDPAL: u32 = 512u32;
pub const DDF_BUFFER: u32 = 64u32;
pub const DDF_DONTDRAW: u32 = 16u32;
pub const DDF_FULLSCREEN: u32 = 256u32;
pub const DDF_HALFTONE: u32 = 4096u32;
pub const DDF_HURRYUP: u32 = 2048u32;
pub const DDF_JUSTDRAWIT: u32 = 128u32;
pub const DDF_NOTKEYFRAME: u32 = 1024u32;
pub const DDF_PREROLL: u32 = 16u32;
pub const DDF_SAME_DIB: u32 = 8u32;
pub const DDF_SAME_DRAW: u32 = 8u32;
pub const DDF_SAME_HDC: u32 = 4u32;
pub const DDF_SAME_SIZE: u32 = 8u32;
pub const DDF_UPDATE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIALOGICOKIADPCMWAVEFORMAT {
    pub ewf: WAVEFORMATEX,
}
impl DIALOGICOKIADPCMWAVEFORMAT {}
impl ::std::default::Default for DIALOGICOKIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DIALOGICOKIADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DIALOGICOKIADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DIALOGICOKIADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DIGIADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl DIGIADPCMWAVEFORMAT {}
impl ::std::default::Default for DIGIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DIGIADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DIGIADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DIGIADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIGIFIXWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl DIGIFIXWAVEFORMAT {}
impl ::std::default::Default for DIGIFIXWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DIGIFIXWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DIGIFIXWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DIGIFIXWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DIGIREALWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl DIGIREALWAVEFORMAT {}
impl ::std::default::Default for DIGIREALWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DIGIREALWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DIGIREALWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DIGIREALWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DIGISTDWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl DIGISTDWAVEFORMAT {}
impl ::std::default::Default for DIGISTDWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DIGISTDWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DIGISTDWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DIGISTDWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DLG_ACMFILTERCHOOSE_ID: u32 = 71u32;
pub const DLG_ACMFORMATCHOOSE_ID: u32 = 70u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DOLBYAC2WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub nAuxBitsCode: u16,
}
impl DOLBYAC2WAVEFORMAT {}
impl ::std::default::Default for DOLBYAC2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DOLBYAC2WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DOLBYAC2WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DOLBYAC2WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DRAWDIBTIME {
    pub timeCount: i32,
    pub timeDraw: i32,
    pub timeDecompress: i32,
    pub timeDither: i32,
    pub timeStretch: i32,
    pub timeBlt: i32,
    pub timeSetDIBits: i32,
}
impl DRAWDIBTIME {}
impl ::std::default::Default for DRAWDIBTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DRAWDIBTIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRAWDIBTIME")
            .field("timeCount", &self.timeCount)
            .field("timeDraw", &self.timeDraw)
            .field("timeDecompress", &self.timeDecompress)
            .field("timeDither", &self.timeDither)
            .field("timeStretch", &self.timeStretch)
            .field("timeBlt", &self.timeBlt)
            .field("timeSetDIBits", &self.timeSetDIBits)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DRAWDIBTIME {
    fn eq(&self, other: &Self) -> bool {
        self.timeCount == other.timeCount
            && self.timeDraw == other.timeDraw
            && self.timeDecompress == other.timeDecompress
            && self.timeDither == other.timeDither
            && self.timeStretch == other.timeStretch
            && self.timeBlt == other.timeBlt
            && self.timeSetDIBits == other.timeSetDIBits
    }
}
impl ::std::cmp::Eq for DRAWDIBTIME {}
unsafe impl ::windows::runtime::Abi for DRAWDIBTIME {
    type Abi = Self;
    type DefaultType = Self;
}
pub type DRIVERMSGPROC = unsafe extern "system" fn(
    param0: u32,
    param1: u32,
    param2: usize,
    param3: usize,
    param4: usize,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type DRIVERPROC = unsafe extern "system" fn(
    param0: usize,
    param1: HDRVR,
    param2: u32,
    param3: super::super::Foundation::LPARAM,
    param4: super::super::Foundation::LPARAM,
) -> super::super::Foundation::LRESULT;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DRMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wReserved: u16,
    pub ulContentId: u32,
    pub wfxSecure: WAVEFORMATEX,
}
impl DRMWAVEFORMAT {}
impl ::std::default::Default for DRMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DRMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DRMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DRMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DRVCNF_CANCEL: u32 = 0u32;
pub const DRVCNF_OK: u32 = 1u32;
pub const DRVCNF_RESTART: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct DRVCONFIGINFO {
    pub dwDCISize: u32,
    pub lpszDCISectionName: super::super::Foundation::PWSTR,
    pub lpszDCIAliasName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DRVCONFIGINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DRVCONFIGINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DRVCONFIGINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DRVCONFIGINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DRVCONFIGINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct DRVCONFIGINFOEX {
    pub dwDCISize: u32,
    pub lpszDCISectionName: super::super::Foundation::PWSTR,
    pub lpszDCIAliasName: super::super::Foundation::PWSTR,
    pub dnDevNode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DRVCONFIGINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DRVCONFIGINFOEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DRVCONFIGINFOEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DRVCONFIGINFOEX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DRVCONFIGINFOEX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DRVM_ADD_THRU: u32 = 257u32;
pub const DRVM_DISABLE: u32 = 102u32;
pub const DRVM_ENABLE: u32 = 103u32;
pub const DRVM_EXIT: u32 = 101u32;
pub const DRVM_INIT: u32 = 100u32;
pub const DRVM_INIT_EX: u32 = 104u32;
pub const DRVM_IOCTL: u32 = 256u32;
pub const DRVM_IOCTL_CMD_SYSTEM: i32 = -2147483648i32;
pub const DRVM_IOCTL_CMD_USER: i32 = 0i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DRVM_IOCTL_DATA {
    pub dwSize: u32,
    pub dwCmd: u32,
}
impl DRVM_IOCTL_DATA {}
impl ::std::default::Default for DRVM_IOCTL_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DRVM_IOCTL_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DRVM_IOCTL_DATA {}
unsafe impl ::windows::runtime::Abi for DRVM_IOCTL_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DRVM_IOCTL_LAST: u32 = 261u32;
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_CONSOLEVOICECOM_GET: u32 = 8215u32;
pub const DRVM_MAPPER_PREFERRED_FLAGS_PREFERREDONLY: u32 = 1u32;
pub const DRVM_MAPPER_PREFERRED_GET: u32 = 8213u32;
pub const DRVM_MAPPER_RECONFIGURE: u32 = 8193u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRVM_REMOVE_THRU: u32 = 258u32;
pub const DRVM_USER: u32 = 16384u32;
pub const DRV_CANCEL: u32 = 0u32;
pub const DRV_CLOSE: u32 = 4u32;
pub const DRV_CONFIGURE: u32 = 7u32;
pub const DRV_DISABLE: u32 = 5u32;
pub const DRV_ENABLE: u32 = 2u32;
pub const DRV_EXITSESSION: u32 = 11u32;
pub const DRV_FREE: u32 = 6u32;
pub const DRV_INSTALL: u32 = 9u32;
pub const DRV_LOAD: u32 = 1u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
pub const DRV_MCI_FIRST: u32 = 2048u32;
pub const DRV_MCI_LAST: u32 = 6143u32;
pub const DRV_OK: u32 = 1u32;
pub const DRV_OPEN: u32 = 3u32;
pub const DRV_PNPINSTALL: u32 = 2059u32;
pub const DRV_POWER: u32 = 15u32;
pub const DRV_QUERYCONFIGURE: u32 = 8u32;
pub const DRV_QUERYDEVICEINTERFACE: u32 = 2060u32;
pub const DRV_QUERYDEVICEINTERFACESIZE: u32 = 2061u32;
pub const DRV_QUERYDEVNODE: u32 = 2050u32;
pub const DRV_QUERYFUNCTIONINSTANCEID: u32 = 2065u32;
pub const DRV_QUERYFUNCTIONINSTANCEIDSIZE: u32 = 2066u32;
pub const DRV_QUERYIDFROMSTRINGID: u32 = 2064u32;
pub const DRV_QUERYMAPPABLE: u32 = 2053u32;
pub const DRV_QUERYMODULE: u32 = 2057u32;
pub const DRV_QUERYSTRINGID: u32 = 2062u32;
pub const DRV_QUERYSTRINGIDSIZE: u32 = 2063u32;
pub const DRV_REMOVE: u32 = 10u32;
pub const DRV_RESERVED: u32 = 2048u32;
pub const DRV_RESTART: u32 = 2u32;
pub const DRV_USER: u32 = 16384u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct DVIADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl DVIADPCMWAVEFORMAT {}
impl ::std::default::Default for DVIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DVIADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DVIADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for DVIADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DVM_CONFIGURE_END: u32 = 8191u32;
pub const DVM_CONFIGURE_START: u32 = 4096u32;
pub const DVM_DST_RECT: u32 = 4101u32;
pub const DVM_FORMAT: u32 = 4098u32;
pub const DVM_PALETTE: u32 = 4097u32;
pub const DVM_PALETTERGB555: u32 = 4099u32;
pub const DVM_SRC_RECT: u32 = 4100u32;
pub const DVM_USER: u32 = 16384u32;
pub const DV_ERR_13: u32 = 16u32;
pub const DV_ERR_ALLOCATED: u32 = 19u32;
pub const DV_ERR_BADDEVICEID: u32 = 20u32;
pub const DV_ERR_BADERRNUM: u32 = 22u32;
pub const DV_ERR_BADFORMAT: u32 = 2u32;
pub const DV_ERR_BADINSTALL: u32 = 8u32;
pub const DV_ERR_BASE: u32 = 1u32;
pub const DV_ERR_CONFIG1: u32 = 13u32;
pub const DV_ERR_CONFIG2: u32 = 14u32;
pub const DV_ERR_CREATEPALETTE: u32 = 9u32;
pub const DV_ERR_DMA_CONFLICT: u32 = 26u32;
pub const DV_ERR_FLAGS: u32 = 15u32;
pub const DV_ERR_INT_CONFLICT: u32 = 27u32;
pub const DV_ERR_INVALHANDLE: u32 = 21u32;
pub const DV_ERR_IO_CONFLICT: u32 = 25u32;
pub const DV_ERR_LASTERROR: u32 = 28u32;
pub const DV_ERR_MEM_CONFLICT: u32 = 24u32;
pub const DV_ERR_NOMEM: u32 = 18u32;
pub const DV_ERR_NONSPECIFIC: u32 = 1u32;
pub const DV_ERR_NOTDETECTED: u32 = 7u32;
pub const DV_ERR_NOTSUPPORTED: u32 = 17u32;
pub const DV_ERR_NO_BUFFERS: u32 = 23u32;
pub const DV_ERR_OK: u32 = 0u32;
pub const DV_ERR_PARAM1: u32 = 11u32;
pub const DV_ERR_PARAM2: u32 = 12u32;
pub const DV_ERR_PROTECT_ONLY: u32 = 28u32;
pub const DV_ERR_SIZEFIELD: u32 = 10u32;
pub const DV_ERR_STILLPLAYING: u32 = 3u32;
pub const DV_ERR_SYNC: u32 = 5u32;
pub const DV_ERR_TOOMANYCHANNELS: u32 = 6u32;
pub const DV_ERR_UNPREPARED: u32 = 4u32;
pub const DV_ERR_USER_MSG: u32 = 1001u32;
pub const DV_VM_CLOSE: u32 = 977u32;
pub const DV_VM_DATA: u32 = 978u32;
pub const DV_VM_ERROR: u32 = 979u32;
pub const DV_VM_OPEN: u32 = 976u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefDriverProc<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, HDRVR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    dwdriveridentifier: usize,
    hdrvr: Param1,
    umsg: u32,
    lparam1: Param3,
    lparam2: Param4,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefDriverProc(
                dwdriveridentifier: usize,
                hdrvr: HDRVR,
                umsg: u32,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(DefDriverProc(
            ::std::mem::transmute(dwdriveridentifier),
            hdrvr.into_param().abi(),
            ::std::mem::transmute(umsg),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibBegin<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdd: isize,
    hdc: Param1,
    dxdst: i32,
    dydst: i32,
    lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    dxsrc: i32,
    dysrc: i32,
    wflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibBegin(
                hdd: isize,
                hdc: super::super::Graphics::Gdi::HDC,
                dxdst: i32,
                dydst: i32,
                lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                dxsrc: i32,
                dysrc: i32,
                wflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibBegin(
            ::std::mem::transmute(hdd),
            hdc.into_param().abi(),
            ::std::mem::transmute(dxdst),
            ::std::mem::transmute(dydst),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(dxsrc),
            ::std::mem::transmute(dysrc),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibChangePalette(
    hdd: isize,
    istart: i32,
    ilen: i32,
    lppe: *const super::super::Graphics::Gdi::PALETTEENTRY,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibChangePalette(
                hdd: isize,
                istart: i32,
                ilen: i32,
                lppe: *const super::super::Graphics::Gdi::PALETTEENTRY,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibChangePalette(
            ::std::mem::transmute(hdd),
            ::std::mem::transmute(istart),
            ::std::mem::transmute(ilen),
            ::std::mem::transmute(lppe),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibClose(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibClose(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibClose(::std::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibDraw<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hdd: isize,
    hdc: Param1,
    xdst: i32,
    ydst: i32,
    dxdst: i32,
    dydst: i32,
    lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpbits: *const ::std::ffi::c_void,
    xsrc: i32,
    ysrc: i32,
    dxsrc: i32,
    dysrc: i32,
    wflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibDraw(
                hdd: isize,
                hdc: super::super::Graphics::Gdi::HDC,
                xdst: i32,
                ydst: i32,
                dxdst: i32,
                dydst: i32,
                lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpbits: *const ::std::ffi::c_void,
                xsrc: i32,
                ysrc: i32,
                dxsrc: i32,
                dysrc: i32,
                wflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibDraw(
            ::std::mem::transmute(hdd),
            hdc.into_param().abi(),
            ::std::mem::transmute(xdst),
            ::std::mem::transmute(ydst),
            ::std::mem::transmute(dxdst),
            ::std::mem::transmute(dydst),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(xsrc),
            ::std::mem::transmute(ysrc),
            ::std::mem::transmute(dxsrc),
            ::std::mem::transmute(dysrc),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibEnd(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibEnd(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibEnd(::std::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawDibGetBuffer(
    hdd: isize,
    lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    dwsize: u32,
    dwflags: u32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibGetBuffer(
                hdd: isize,
                lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
                dwsize: u32,
                dwflags: u32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(DrawDibGetBuffer(
            ::std::mem::transmute(hdd),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(dwsize),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DrawDibGetPalette(hdd: isize) -> super::super::Graphics::Gdi::HPALETTE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibGetPalette(hdd: isize) -> super::super::Graphics::Gdi::HPALETTE;
        }
        ::std::mem::transmute(DrawDibGetPalette(::std::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DrawDibOpen() -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibOpen() -> isize;
        }
        ::std::mem::transmute(DrawDibOpen())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibProfileDisplay(
    lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibProfileDisplay(
                lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(DrawDibProfileDisplay(::std::mem::transmute(lpbi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibRealize<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hdd: isize,
    hdc: Param1,
    fbackground: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibRealize(
                hdd: isize,
                hdc: super::super::Graphics::Gdi::HDC,
                fbackground: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(DrawDibRealize(
            ::std::mem::transmute(hdd),
            hdc.into_param().abi(),
            fbackground.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawDibSetPalette<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>,
>(
    hdd: isize,
    hpal: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibSetPalette(
                hdd: isize,
                hpal: super::super::Graphics::Gdi::HPALETTE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibSetPalette(
            ::std::mem::transmute(hdd),
            hpal.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibStart(hdd: isize, rate: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibStart(hdd: isize, rate: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibStart(
            ::std::mem::transmute(hdd),
            ::std::mem::transmute(rate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibStop(hdd: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibStop(hdd: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibStop(::std::mem::transmute(hdd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawDibTime(
    hdd: isize,
    lpddtime: *mut DRAWDIBTIME,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawDibTime(
                hdd: isize,
                lpddtime: *mut DRAWDIBTIME,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DrawDibTime(
            ::std::mem::transmute(hdd),
            ::std::mem::transmute(lpddtime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DriverCallback<'a, Param2: ::windows::runtime::IntoParam<'a, HDRVR>>(
    dwcallback: usize,
    dwflags: u32,
    hdevice: Param2,
    dwmsg: u32,
    dwuser: usize,
    dwparam1: usize,
    dwparam2: usize,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DriverCallback(
                dwcallback: usize,
                dwflags: u32,
                hdevice: HDRVR,
                dwmsg: u32,
                dwuser: usize,
                dwparam1: usize,
                dwparam2: usize,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DriverCallback(
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwflags),
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwmsg),
            ::std::mem::transmute(dwuser),
            ::std::mem::transmute(dwparam1),
            ::std::mem::transmute(dwparam2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrvGetModuleHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HDRVR>>(
    hdriver: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrvGetModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(DrvGetModuleHandle(hdriver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ECHOSC1WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl ECHOSC1WAVEFORMAT {}
impl ::std::default::Default for ECHOSC1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ECHOSC1WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ECHOSC1WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for ECHOSC1WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ECHOWAVEFILTER {}
impl ::std::default::Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ECHOWAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ECHOWAVEFILTER {}
unsafe impl ::windows::runtime::Abi for ECHOWAVEFILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct EXBMINFOHEADER {
    pub bmi: super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub biExtDataOffset: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl EXBMINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for EXBMINFOHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for EXBMINFOHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for EXBMINFOHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for EXBMINFOHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn EditStreamClone<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
) -> ::windows::runtime::Result<IAVIStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamClone(
                pavi: ::windows::runtime::RawPtr,
                ppresult: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAVIStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        EditStreamClone(pavi.into_param().abi(), &mut result__).from_abi::<IAVIStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EditStreamCopy<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    plstart: *mut i32,
    pllength: *mut i32,
    ppresult: *mut ::std::option::Option<IAVIStream>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamCopy(
                pavi: ::windows::runtime::RawPtr,
                plstart: *mut i32,
                pllength: *mut i32,
                ppresult: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamCopy(
            pavi.into_param().abi(),
            ::std::mem::transmute(plstart),
            ::std::mem::transmute(pllength),
            ::std::mem::transmute(ppresult),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EditStreamCut<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    plstart: *mut i32,
    pllength: *mut i32,
    ppresult: *mut ::std::option::Option<IAVIStream>,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamCut(
                pavi: ::windows::runtime::RawPtr,
                plstart: *mut i32,
                pllength: *mut i32,
                ppresult: *mut ::windows::runtime::RawPtr,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamCut(
            pavi.into_param().abi(),
            ::std::mem::transmute(plstart),
            ::std::mem::transmute(pllength),
            ::std::mem::transmute(ppresult),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EditStreamPaste<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IAVIStream>,
    Param3: ::windows::runtime::IntoParam<'a, IAVIStream>,
>(
    pavi: Param0,
    plpos: *mut i32,
    pllength: *mut i32,
    pstream: Param3,
    lstart: i32,
    lend: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamPaste(
                pavi: ::windows::runtime::RawPtr,
                plpos: *mut i32,
                pllength: *mut i32,
                pstream: ::windows::runtime::RawPtr,
                lstart: i32,
                lend: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamPaste(
            pavi.into_param().abi(),
            ::std::mem::transmute(plpos),
            ::std::mem::transmute(pllength),
            pstream.into_param().abi(),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lend),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EditStreamSetInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpinfo: *const AVISTREAMINFOA,
    cbinfo: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetInfoA(
                pavi: ::windows::runtime::RawPtr,
                lpinfo: *const AVISTREAMINFOA,
                cbinfo: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamSetInfoA(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpinfo),
            ::std::mem::transmute(cbinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EditStreamSetInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, IAVIStream>>(
    pavi: Param0,
    lpinfo: *const AVISTREAMINFOW,
    cbinfo: i32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetInfoW(
                pavi: ::windows::runtime::RawPtr,
                lpinfo: *const AVISTREAMINFOW,
                cbinfo: i32,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamSetInfoW(
            pavi.into_param().abi(),
            ::std::mem::transmute(lpinfo),
            ::std::mem::transmute(cbinfo),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EditStreamSetNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IAVIStream>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pavi: Param0,
    lpszname: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetNameA(
                pavi: ::windows::runtime::RawPtr,
                lpszname: super::super::Foundation::PSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamSetNameA(pavi.into_param().abi(), lpszname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EditStreamSetNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IAVIStream>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pavi: Param0,
    lpszname: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditStreamSetNameW(
                pavi: ::windows::runtime::RawPtr,
                lpszname: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        EditStreamSetNameW(pavi.into_param().abi(), lpszname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FACILITY_NS: u32 = 13u32;
pub const FACILITY_NS_WIN32: u32 = 7u32;
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FIND_ANY: i32 = 32i32;
pub const FIND_DIR: i32 = 15i32;
pub const FIND_FORMAT: i32 = 64i32;
pub const FIND_FROM_START: i32 = 8i32;
pub const FIND_INDEX: i32 = 16384i32;
pub const FIND_KEY: i32 = 16i32;
pub const FIND_LENGTH: i32 = 4096i32;
pub const FIND_NEXT: i32 = 1i32;
pub const FIND_OFFSET: i32 = 8192i32;
pub const FIND_POS: i32 = 0i32;
pub const FIND_PREV: i32 = 4i32;
pub const FIND_RET: i32 = 61440i32;
pub const FIND_SIZE: i32 = 12288i32;
pub const FIND_TYPE: i32 = 240i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct FMTOWNS_SND_WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl FMTOWNS_SND_WAVEFORMAT {}
impl ::std::default::Default for FMTOWNS_SND_WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FMTOWNS_SND_WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FMTOWNS_SND_WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for FMTOWNS_SND_WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct G721_ADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub nAuxBlockSize: u16,
}
impl G721_ADPCMWAVEFORMAT {}
impl ::std::default::Default for G721_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for G721_ADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for G721_ADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for G721_ADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct G723_ADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub cbExtraSize: u16,
    pub nAuxBlockSize: u16,
}
impl G723_ADPCMWAVEFORMAT {}
impl ::std::default::Default for G723_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for G723_ADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for G723_ADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for G723_ADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct GSM610WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl GSM610WAVEFORMAT {}
impl ::std::default::Default for GSM610WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for GSM610WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for GSM610WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for GSM610WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDriverModuleHandle<'a, Param0: ::windows::runtime::IntoParam<'a, HDRVR>>(
    hdriver: Param0,
) -> super::super::Foundation::HINSTANCE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDriverModuleHandle(hdriver: HDRVR) -> super::super::Foundation::HINSTANCE;
        }
        ::std::mem::transmute(GetDriverModuleHandle(hdriver.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewA(
    lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNamePreviewA(
                lpofn: *mut ::std::mem::ManuallyDrop<
                    super::super::UI::Controls::Dialogs::OPENFILENAMEA,
                >,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOpenFileNamePreviewA(::std::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetOpenFileNamePreviewW(
    lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOpenFileNamePreviewW(
                lpofn: *mut ::std::mem::ManuallyDrop<
                    super::super::UI::Controls::Dialogs::OPENFILENAMEW,
                >,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOpenFileNamePreviewW(::std::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewA(
    lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEA,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNamePreviewA(
                lpofn: *mut ::std::mem::ManuallyDrop<
                    super::super::UI::Controls::Dialogs::OPENFILENAMEA,
                >,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSaveFileNamePreviewA(::std::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
#[inline]
pub unsafe fn GetSaveFileNamePreviewW(
    lpofn: *mut super::super::UI::Controls::Dialogs::OPENFILENAMEW,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSaveFileNamePreviewW(
                lpofn: *mut ::std::mem::ManuallyDrop<
                    super::super::UI::Controls::Dialogs::OPENFILENAMEW,
                >,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSaveFileNamePreviewW(::std::mem::transmute(lpofn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HACMDRIVER(pub isize);
impl ::std::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HACMDRIVER {}
unsafe impl ::windows::runtime::Abi for HACMDRIVER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HACMDRIVERID(pub isize);
impl ::std::default::Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HACMDRIVERID {}
unsafe impl ::windows::runtime::Abi for HACMDRIVERID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HACMOBJ(pub isize);
impl ::std::default::Default for HACMOBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HACMOBJ {}
unsafe impl ::windows::runtime::Abi for HACMOBJ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HACMSTREAM(pub isize);
impl ::std::default::Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HACMSTREAM {}
unsafe impl ::windows::runtime::Abi for HACMSTREAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HDRVR(pub isize);
impl ::std::default::Default for HDRVR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HDRVR {}
unsafe impl ::windows::runtime::Abi for HDRVR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HIC(pub isize);
impl ::std::default::Default for HIC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HIC {}
unsafe impl ::windows::runtime::Abi for HIC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIDI(pub isize);
impl ::std::default::Default for HMIDI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIDI {}
unsafe impl ::windows::runtime::Abi for HMIDI {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIDIIN(pub isize);
impl ::std::default::Default for HMIDIIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIDIIN {}
unsafe impl ::windows::runtime::Abi for HMIDIIN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIDIOUT(pub isize);
impl ::std::default::Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIDIOUT {}
unsafe impl ::windows::runtime::Abi for HMIDIOUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIDISTRM(pub isize);
impl ::std::default::Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIDISTRM {}
unsafe impl ::windows::runtime::Abi for HMIDISTRM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIXER(pub isize);
impl ::std::default::Default for HMIXER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIXER {}
unsafe impl ::windows::runtime::Abi for HMIXER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMIXEROBJ(pub isize);
impl ::std::default::Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMIXEROBJ {}
unsafe impl ::windows::runtime::Abi for HMIXEROBJ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HMMIO(pub isize);
impl ::std::default::Default for HMMIO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HMMIO {}
unsafe impl ::windows::runtime::Abi for HMMIO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HVIDEO(pub isize);
impl ::std::default::Default for HVIDEO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HVIDEO {}
unsafe impl ::windows::runtime::Abi for HVIDEO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HWAVE(pub isize);
impl ::std::default::Default for HWAVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWAVE {}
unsafe impl ::windows::runtime::Abi for HWAVE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HWAVEIN(pub isize);
impl ::std::default::Default for HWAVEIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWAVEIN {}
unsafe impl ::windows::runtime::Abi for HWAVEIN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HWAVEOUT(pub isize);
impl ::std::default::Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWAVEOUT {}
unsafe impl ::windows::runtime::Abi for HWAVEOUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAVIEditStream(::windows::runtime::IUnknown);
impl IAVIEditStream {
    pub unsafe fn Cut(
        &self,
        plstart: *mut i32,
        pllength: *mut i32,
        ppresult: *mut ::std::option::Option<IAVIStream>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plstart),
            ::std::mem::transmute(pllength),
            ::std::mem::transmute(ppresult),
        )
        .ok()
    }
    pub unsafe fn Copy(
        &self,
        plstart: *mut i32,
        pllength: *mut i32,
        ppresult: *mut ::std::option::Option<IAVIStream>,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plstart),
            ::std::mem::transmute(pllength),
            ::std::mem::transmute(ppresult),
        )
        .ok()
    }
    pub unsafe fn Paste<'a, Param2: ::windows::runtime::IntoParam<'a, IAVIStream>>(
        &self,
        plpos: *mut i32,
        pllength: *mut i32,
        pstream: Param2,
        lstart: i32,
        lend: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(plpos),
            ::std::mem::transmute(pllength),
            pstream.into_param().abi(),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lend),
        )
        .ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IAVIStream> {
        let mut result__: <IAVIStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IAVIStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfo(
        &self,
        lpinfo: *const AVISTREAMINFOW,
        cbinfo: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpinfo),
            ::std::mem::transmute(cbinfo),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAVIEditStream {
    type Vtable = IAVIEditStream_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131108, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAVIEditStream> for ::windows::runtime::IUnknown {
    fn from(value: IAVIEditStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAVIEditStream> for ::windows::runtime::IUnknown {
    fn from(value: &IAVIEditStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAVIEditStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAVIEditStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIEditStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plstart: *mut i32,
        pllength: *mut i32,
        ppresult: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plstart: *mut i32,
        pllength: *mut i32,
        ppresult: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        plpos: *mut i32,
        pllength: *mut i32,
        pstream: ::windows::runtime::RawPtr,
        lstart: i32,
        lend: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppresult: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpinfo: *const AVISTREAMINFOW,
        cbinfo: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAVIFile(::windows::runtime::IUnknown);
impl IAVIFile {
    pub unsafe fn Info(
        &self,
        pfi: *mut AVIFILEINFOW,
        lsize: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pfi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    pub unsafe fn GetStream(
        &self,
        ppstream: *mut ::std::option::Option<IAVIStream>,
        fcctype: u32,
        lparam: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppstream),
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(lparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateStream(
        &self,
        ppstream: *mut ::std::option::Option<IAVIStream>,
        psi: *const AVISTREAMINFOW,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ppstream),
            ::std::mem::transmute(psi),
        )
        .ok()
    }
    pub unsafe fn WriteData(
        &self,
        ckid: u32,
        lpdata: *const ::std::ffi::c_void,
        cbdata: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ckid),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
        )
        .ok()
    }
    pub unsafe fn ReadData(
        &self,
        ckid: u32,
        lpdata: *mut ::std::ffi::c_void,
        lpcbdata: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ckid),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpcbdata),
        )
        .ok()
    }
    pub unsafe fn EndRecord(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DeleteStream(&self, fcctype: u32, lparam: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(lparam),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAVIFile {
    type Vtable = IAVIFile_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131104, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAVIFile> for ::windows::runtime::IUnknown {
    fn from(value: IAVIFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAVIFile> for ::windows::runtime::IUnknown {
    fn from(value: &IAVIFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAVIFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAVIFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIFile_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfi: *mut AVIFILEINFOW,
        lsize: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppstream: *mut ::windows::runtime::RawPtr,
        fcctype: u32,
        lparam: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppstream: *mut ::windows::runtime::RawPtr,
        psi: *const AVISTREAMINFOW,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ckid: u32,
        lpdata: *const ::std::ffi::c_void,
        cbdata: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ckid: u32,
        lpdata: *mut ::std::ffi::c_void,
        lpcbdata: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fcctype: u32,
        lparam: i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAVIPersistFile(::windows::runtime::IUnknown);
impl IAVIPersistFile {
    pub unsafe fn GetClassID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsDirty(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Load<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszfilename: Param0,
        dwmode: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszfilename.into_param().abi(),
            ::std::mem::transmute(dwmode),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        pszfilename: Param0,
        fremember: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pszfilename.into_param().abi(),
            fremember.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszfilename: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pszfilename.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurFile(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn Reserved1(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAVIPersistFile {
    type Vtable = IAVIPersistFile_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131109, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAVIPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAVIPersistFile> for ::windows::runtime::IUnknown {
    fn from(value: &IAVIPersistFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAVIPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAVIPersistFile {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IAVIPersistFile> for super::super::System::Com::IPersistFile {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IAVIPersistFile> for super::super::System::Com::IPersistFile {
    fn from(value: &IAVIPersistFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersistFile>
    for IAVIPersistFile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersistFile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Com::IPersistFile,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersistFile>
    for &IAVIPersistFile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersistFile> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Com::IPersistFile,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<IAVIPersistFile> for super::super::System::Com::IPersist {
    fn from(value: IAVIPersistFile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::convert::From<&IAVIPersistFile> for super::super::System::Com::IPersist {
    fn from(value: &IAVIPersistFile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersist>
    for IAVIPersistFile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersist> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::System::Com::IPersist>::into(self),
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IPersist>
    for &IAVIPersistFile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IPersist> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<super::super::System::Com::IPersist>::into(
                ::std::clone::Clone::clone(self),
            ),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIPersistFile_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pclassid: *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfilename: super::super::Foundation::PWSTR,
        dwmode: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfilename: super::super::Foundation::PWSTR,
        fremember: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszfilename: super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ppszfilename: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAVIStream(::windows::runtime::IUnknown);
impl IAVIStream {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    >(
        &self,
        lparam1: Param0,
        lparam2: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Info(
        &self,
        psi: *mut AVISTREAMINFOW,
        lsize: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(psi),
            ::std::mem::transmute(lsize),
        )
        .ok()
    }
    pub unsafe fn FindSample(&self, lpos: i32, lflags: i32) -> i32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lflags),
        ))
    }
    pub unsafe fn ReadFormat(
        &self,
        lpos: i32,
        lpformat: *mut ::std::ffi::c_void,
        lpcbformat: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lpformat),
            ::std::mem::transmute(lpcbformat),
        )
        .ok()
    }
    pub unsafe fn SetFormat(
        &self,
        lpos: i32,
        lpformat: *const ::std::ffi::c_void,
        cbformat: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpos),
            ::std::mem::transmute(lpformat),
            ::std::mem::transmute(cbformat),
        )
        .ok()
    }
    pub unsafe fn Read(
        &self,
        lstart: i32,
        lsamples: i32,
        lpbuffer: *mut ::std::ffi::c_void,
        cbbuffer: i32,
        plbytes: *mut i32,
        plsamples: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lsamples),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(cbbuffer),
            ::std::mem::transmute(plbytes),
            ::std::mem::transmute(plsamples),
        )
        .ok()
    }
    pub unsafe fn Write(
        &self,
        lstart: i32,
        lsamples: i32,
        lpbuffer: *const ::std::ffi::c_void,
        cbbuffer: i32,
        dwflags: u32,
        plsampwritten: *mut i32,
        plbyteswritten: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lsamples),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(cbbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(plsampwritten),
            ::std::mem::transmute(plbyteswritten),
        )
        .ok()
    }
    pub unsafe fn Delete(&self, lstart: i32, lsamples: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lsamples),
        )
        .ok()
    }
    pub unsafe fn ReadData(
        &self,
        fcc: u32,
        lp: *mut ::std::ffi::c_void,
        lpcb: *mut i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fcc),
            ::std::mem::transmute(lp),
            ::std::mem::transmute(lpcb),
        )
        .ok()
    }
    pub unsafe fn WriteData(
        &self,
        fcc: u32,
        lp: *const ::std::ffi::c_void,
        cb: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(fcc),
            ::std::mem::transmute(lp),
            ::std::mem::transmute(cb),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInfo(
        &self,
        lpinfo: *const AVISTREAMINFOW,
        cbinfo: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpinfo),
            ::std::mem::transmute(cbinfo),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAVIStream {
    type Vtable = IAVIStream_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131105, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAVIStream> for ::windows::runtime::IUnknown {
    fn from(value: IAVIStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAVIStream> for ::windows::runtime::IUnknown {
    fn from(value: &IAVIStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAVIStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAVIStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStream_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lparam1: super::super::Foundation::LPARAM,
        lparam2: super::super::Foundation::LPARAM,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psi: *mut AVISTREAMINFOW,
        lsize: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lpos: i32, lflags: i32) -> i32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpos: i32,
        lpformat: *mut ::std::ffi::c_void,
        lpcbformat: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpos: i32,
        lpformat: *const ::std::ffi::c_void,
        cbformat: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lstart: i32,
        lsamples: i32,
        lpbuffer: *mut ::std::ffi::c_void,
        cbbuffer: i32,
        plbytes: *mut i32,
        plsamples: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lstart: i32,
        lsamples: i32,
        lpbuffer: *const ::std::ffi::c_void,
        cbbuffer: i32,
        dwflags: u32,
        plsampwritten: *mut i32,
        plbyteswritten: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lstart: i32,
        lsamples: i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fcc: u32,
        lp: *mut ::std::ffi::c_void,
        lpcb: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        fcc: u32,
        lp: *const ::std::ffi::c_void,
        cb: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpinfo: *const AVISTREAMINFOW,
        cbinfo: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAVIStreaming(::windows::runtime::IUnknown);
impl IAVIStreaming {
    pub unsafe fn Begin(
        &self,
        lstart: i32,
        lend: i32,
        lrate: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lend),
            ::std::mem::transmute(lrate),
        )
        .ok()
    }
    pub unsafe fn End(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAVIStreaming {
    type Vtable = IAVIStreaming_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131106, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IAVIStreaming> for ::windows::runtime::IUnknown {
    fn from(value: IAVIStreaming) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAVIStreaming> for ::windows::runtime::IUnknown {
    fn from(value: &IAVIStreaming) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAVIStreaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IAVIStreaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAVIStreaming_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lstart: i32,
        lend: i32,
        lrate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICCOMPRESS {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut ::std::ffi::c_void,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut ::std::ffi::c_void,
    pub lpckid: *mut u32,
    pub lpdwFlags: *mut u32,
    pub lFrameNum: i32,
    pub dwFrameSize: u32,
    pub dwQuality: u32,
    pub lpbiPrev: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpPrev: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ICCOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for ICCOMPRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for ICCOMPRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICCOMPRESS")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lpOutput", &self.lpOutput)
            .field("lpbiInput", &self.lpbiInput)
            .field("lpInput", &self.lpInput)
            .field("lpckid", &self.lpckid)
            .field("lpdwFlags", &self.lpdwFlags)
            .field("lFrameNum", &self.lFrameNum)
            .field("dwFrameSize", &self.dwFrameSize)
            .field("dwQuality", &self.dwQuality)
            .field("lpbiPrev", &self.lpbiPrev)
            .field("lpPrev", &self.lpPrev)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for ICCOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiOutput == other.lpbiOutput
            && self.lpOutput == other.lpOutput
            && self.lpbiInput == other.lpbiInput
            && self.lpInput == other.lpInput
            && self.lpckid == other.lpckid
            && self.lpdwFlags == other.lpdwFlags
            && self.lFrameNum == other.lFrameNum
            && self.dwFrameSize == other.dwFrameSize
            && self.dwQuality == other.dwQuality
            && self.lpbiPrev == other.lpbiPrev
            && self.lpPrev == other.lpPrev
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for ICCOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for ICCOMPRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICCOMPRESSFRAMES {
    pub dwFlags: u32,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lOutput: super::super::Foundation::LPARAM,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lInput: super::super::Foundation::LPARAM,
    pub lStartFrame: i32,
    pub lFrameCount: i32,
    pub lQuality: i32,
    pub lDataRate: i32,
    pub lKeyRate: i32,
    pub dwRate: u32,
    pub dwScale: u32,
    pub dwOverheadPerFrame: u32,
    pub dwReserved2: u32,
    pub GetData: isize,
    pub PutData: isize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ICCOMPRESSFRAMES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for ICCOMPRESSFRAMES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for ICCOMPRESSFRAMES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICCOMPRESSFRAMES")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lOutput", &self.lOutput)
            .field("lpbiInput", &self.lpbiInput)
            .field("lInput", &self.lInput)
            .field("lStartFrame", &self.lStartFrame)
            .field("lFrameCount", &self.lFrameCount)
            .field("lQuality", &self.lQuality)
            .field("lDataRate", &self.lDataRate)
            .field("lKeyRate", &self.lKeyRate)
            .field("dwRate", &self.dwRate)
            .field("dwScale", &self.dwScale)
            .field("dwOverheadPerFrame", &self.dwOverheadPerFrame)
            .field("dwReserved2", &self.dwReserved2)
            .field("GetData", &self.GetData)
            .field("PutData", &self.PutData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for ICCOMPRESSFRAMES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiOutput == other.lpbiOutput
            && self.lOutput == other.lOutput
            && self.lpbiInput == other.lpbiInput
            && self.lInput == other.lInput
            && self.lStartFrame == other.lStartFrame
            && self.lFrameCount == other.lFrameCount
            && self.lQuality == other.lQuality
            && self.lDataRate == other.lDataRate
            && self.lKeyRate == other.lKeyRate
            && self.dwRate == other.dwRate
            && self.dwScale == other.dwScale
            && self.dwOverheadPerFrame == other.dwOverheadPerFrame
            && self.dwReserved2 == other.dwReserved2
            && self.GetData == other.GetData
            && self.PutData == other.PutData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for ICCOMPRESSFRAMES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for ICCOMPRESSFRAMES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICCOMPRESSFRAMES_PADDING: u32 = 1u32;
pub const ICCOMPRESS_KEYFRAME: i32 = 1i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICClose<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICClose(hic: HIC) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(ICClose(hic.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICCompress<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    dwflags: u32,
    lpbioutput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpdata: *mut ::std::ffi::c_void,
    lpbiinput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpbits: *const ::std::ffi::c_void,
    lpckid: *mut u32,
    lpdwflags: *mut u32,
    lframenum: i32,
    dwframesize: u32,
    dwquality: u32,
    lpbiprev: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpprev: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompress(
                hic: HIC,
                dwflags: u32,
                lpbioutput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpdata: *mut ::std::ffi::c_void,
                lpbiinput: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpbits: *const ::std::ffi::c_void,
                lpckid: *mut u32,
                lpdwflags: *mut u32,
                lframenum: i32,
                dwframesize: u32,
                dwquality: u32,
                lpbiprev: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpprev: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(ICCompress(
            hic.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpbioutput),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpbiinput),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(lpckid),
            ::std::mem::transmute(lpdwflags),
            ::std::mem::transmute(lframenum),
            ::std::mem::transmute(dwframesize),
            ::std::mem::transmute(dwquality),
            ::std::mem::transmute(lpbiprev),
            ::std::mem::transmute(lpprev),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICCompressorChoose<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hwnd: Param0,
    uiflags: u32,
    pvin: *const ::std::ffi::c_void,
    lpdata: *const ::std::ffi::c_void,
    pc: *mut COMPVARS,
    lpsztitle: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompressorChoose(
                hwnd: super::super::Foundation::HWND,
                uiflags: u32,
                pvin: *const ::std::ffi::c_void,
                lpdata: *const ::std::ffi::c_void,
                pc: *mut COMPVARS,
                lpsztitle: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ICCompressorChoose(
            hwnd.into_param().abi(),
            ::std::mem::transmute(uiflags),
            ::std::mem::transmute(pvin),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(pc),
            lpsztitle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICCompressorFree(pc: *const COMPVARS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICCompressorFree(pc: *const COMPVARS);
        }
        ::std::mem::transmute(ICCompressorFree(::std::mem::transmute(pc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDECOMPRESS {
    pub dwFlags: u32,
    pub lpbiInput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpInput: *mut ::std::ffi::c_void,
    pub lpbiOutput: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpOutput: *mut ::std::ffi::c_void,
    pub ckid: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ICDECOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for ICDECOMPRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for ICDECOMPRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICDECOMPRESS")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiInput", &self.lpbiInput)
            .field("lpInput", &self.lpInput)
            .field("lpbiOutput", &self.lpbiOutput)
            .field("lpOutput", &self.lpOutput)
            .field("ckid", &self.ckid)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for ICDECOMPRESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiInput == other.lpbiInput
            && self.lpInput == other.lpInput
            && self.lpbiOutput == other.lpbiOutput
            && self.lpOutput == other.lpOutput
            && self.ckid == other.ckid
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for ICDECOMPRESS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for ICDECOMPRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDECOMPRESSEX {
    pub dwFlags: u32,
    pub lpbiSrc: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpSrc: *mut ::std::ffi::c_void,
    pub lpbiDst: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpDst: *mut ::std::ffi::c_void,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ICDECOMPRESSEX {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for ICDECOMPRESSEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for ICDECOMPRESSEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICDECOMPRESSEX")
            .field("dwFlags", &self.dwFlags)
            .field("lpbiSrc", &self.lpbiSrc)
            .field("lpSrc", &self.lpSrc)
            .field("lpbiDst", &self.lpbiDst)
            .field("lpDst", &self.lpDst)
            .field("xDst", &self.xDst)
            .field("yDst", &self.yDst)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for ICDECOMPRESSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpbiSrc == other.lpbiSrc
            && self.lpSrc == other.lpSrc
            && self.lpbiDst == other.lpbiDst
            && self.lpDst == other.lpDst
            && self.xDst == other.xDst
            && self.yDst == other.yDst
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for ICDECOMPRESSEX {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for ICDECOMPRESSEX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICDECOMPRESS_HURRYUP: i32 = -2147483648i32;
pub const ICDECOMPRESS_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDECOMPRESS_NULLFRAME: i32 = 268435456i32;
pub const ICDECOMPRESS_PREROLL: i32 = 536870912i32;
pub const ICDECOMPRESS_UPDATE: i32 = 1073741824i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICDRAW {
    pub dwFlags: u32,
    pub lpFormat: *mut ::std::ffi::c_void,
    pub lpData: *mut ::std::ffi::c_void,
    pub cbData: u32,
    pub lTime: i32,
}
impl ICDRAW {}
impl ::std::default::Default for ICDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ICDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICDRAW")
            .field("dwFlags", &self.dwFlags)
            .field("lpFormat", &self.lpFormat)
            .field("lpData", &self.lpData)
            .field("cbData", &self.cbData)
            .field("lTime", &self.lTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ICDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.lpFormat == other.lpFormat
            && self.lpData == other.lpData
            && self.cbData == other.cbData
            && self.lTime == other.lTime
    }
}
impl ::std::cmp::Eq for ICDRAW {}
unsafe impl ::windows::runtime::Abi for ICDRAW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ICDRAWBEGIN {
    pub dwFlags: u32,
    pub hpal: super::super::Graphics::Gdi::HPALETTE,
    pub hwnd: super::super::Foundation::HWND,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub xDst: i32,
    pub yDst: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub lpbi: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub xSrc: i32,
    pub ySrc: i32,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dwRate: u32,
    pub dwScale: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ICDRAWBEGIN {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for ICDRAWBEGIN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for ICDRAWBEGIN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICDRAWBEGIN")
            .field("dwFlags", &self.dwFlags)
            .field("hpal", &self.hpal)
            .field("hwnd", &self.hwnd)
            .field("hdc", &self.hdc)
            .field("xDst", &self.xDst)
            .field("yDst", &self.yDst)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("lpbi", &self.lpbi)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .field("dwRate", &self.dwRate)
            .field("dwScale", &self.dwScale)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for ICDRAWBEGIN {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.hpal == other.hpal
            && self.hwnd == other.hwnd
            && self.hdc == other.hdc
            && self.xDst == other.xDst
            && self.yDst == other.yDst
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.lpbi == other.lpbi
            && self.xSrc == other.xSrc
            && self.ySrc == other.ySrc
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
            && self.dwRate == other.dwRate
            && self.dwScale == other.dwScale
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for ICDRAWBEGIN {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for ICDRAWBEGIN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICDRAWSUGGEST {
    pub lpbiIn: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub lpbiSuggest: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    pub dxSrc: i32,
    pub dySrc: i32,
    pub dxDst: i32,
    pub dyDst: i32,
    pub hicDecompressor: HIC,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ICDRAWSUGGEST {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for ICDRAWSUGGEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for ICDRAWSUGGEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICDRAWSUGGEST")
            .field("lpbiIn", &self.lpbiIn)
            .field("lpbiSuggest", &self.lpbiSuggest)
            .field("dxSrc", &self.dxSrc)
            .field("dySrc", &self.dySrc)
            .field("dxDst", &self.dxDst)
            .field("dyDst", &self.dyDst)
            .field("hicDecompressor", &self.hicDecompressor)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for ICDRAWSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        self.lpbiIn == other.lpbiIn
            && self.lpbiSuggest == other.lpbiSuggest
            && self.dxSrc == other.dxSrc
            && self.dySrc == other.dySrc
            && self.dxDst == other.dxDst
            && self.dyDst == other.dyDst
            && self.hicDecompressor == other.hicDecompressor
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for ICDRAWSUGGEST {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for ICDRAWSUGGEST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICDRAW_ANIMATE: i32 = 8i32;
pub const ICDRAW_BUFFER: i32 = 256i32;
pub const ICDRAW_CONTINUE: i32 = 16i32;
pub const ICDRAW_FULLSCREEN: i32 = 2i32;
pub const ICDRAW_HDC: i32 = 4i32;
pub const ICDRAW_HURRYUP: i32 = -2147483648i32;
pub const ICDRAW_MEMORYDC: i32 = 32i32;
pub const ICDRAW_NOTKEYFRAME: i32 = 134217728i32;
pub const ICDRAW_NULLFRAME: i32 = 268435456i32;
pub const ICDRAW_PREROLL: i32 = 536870912i32;
pub const ICDRAW_QUERY: i32 = 1i32;
pub const ICDRAW_RENDER: i32 = 128i32;
pub const ICDRAW_UPDATE: i32 = 1073741824i32;
pub const ICDRAW_UPDATING: i32 = 64i32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICDecompress<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    dwflags: u32,
    lpbiformat: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpdata: *const ::std::ffi::c_void,
    lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpbits: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDecompress(
                hic: HIC,
                dwflags: u32,
                lpbiformat: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpdata: *const ::std::ffi::c_void,
                lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpbits: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(ICDecompress(
            hic.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpbiformat),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(lpbits),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ICDraw<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    dwflags: u32,
    lpformat: *const ::std::ffi::c_void,
    lpdata: *const ::std::ffi::c_void,
    cbdata: u32,
    ltime: i32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDraw(
                hic: HIC,
                dwflags: u32,
                lpformat: *const ::std::ffi::c_void,
                lpdata: *const ::std::ffi::c_void,
                cbdata: u32,
                ltime: i32,
            ) -> u32;
        }
        ::std::mem::transmute(ICDraw(
            hic.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpformat),
            ::std::mem::transmute(lpdata),
            ::std::mem::transmute(cbdata),
            ::std::mem::transmute(ltime),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICDrawBegin<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HIC>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HPALETTE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>,
>(
    hic: Param0,
    dwflags: u32,
    hpal: Param2,
    hwnd: Param3,
    hdc: Param4,
    xdst: i32,
    ydst: i32,
    dxdst: i32,
    dydst: i32,
    lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    xsrc: i32,
    ysrc: i32,
    dxsrc: i32,
    dysrc: i32,
    dwrate: u32,
    dwscale: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICDrawBegin(
                hic: HIC,
                dwflags: u32,
                hpal: super::super::Graphics::Gdi::HPALETTE,
                hwnd: super::super::Foundation::HWND,
                hdc: super::super::Graphics::Gdi::HDC,
                xdst: i32,
                ydst: i32,
                dxdst: i32,
                dydst: i32,
                lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                xsrc: i32,
                ysrc: i32,
                dxsrc: i32,
                dysrc: i32,
                dwrate: u32,
                dwscale: u32,
            ) -> u32;
        }
        ::std::mem::transmute(ICDrawBegin(
            hic.into_param().abi(),
            ::std::mem::transmute(dwflags),
            hpal.into_param().abi(),
            hwnd.into_param().abi(),
            hdc.into_param().abi(),
            ::std::mem::transmute(xdst),
            ::std::mem::transmute(ydst),
            ::std::mem::transmute(dxdst),
            ::std::mem::transmute(dydst),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(xsrc),
            ::std::mem::transmute(ysrc),
            ::std::mem::transmute(dxsrc),
            ::std::mem::transmute(dysrc),
            ::std::mem::transmute(dwrate),
            ::std::mem::transmute(dwscale),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ICERR_ABORT: i32 = -10i32;
pub const ICERR_BADBITDEPTH: i32 = -200i32;
pub const ICERR_BADFLAGS: i32 = -5i32;
pub const ICERR_BADFORMAT: i32 = -2i32;
pub const ICERR_BADHANDLE: i32 = -8i32;
pub const ICERR_BADIMAGESIZE: i32 = -201i32;
pub const ICERR_BADPARAM: i32 = -6i32;
pub const ICERR_BADSIZE: i32 = -7i32;
pub const ICERR_CANTUPDATE: i32 = -9i32;
pub const ICERR_CUSTOM: i32 = -400i32;
pub const ICERR_DONTDRAW: i32 = 1i32;
pub const ICERR_ERROR: i32 = -100i32;
pub const ICERR_GOTOKEYFRAME: i32 = 3i32;
pub const ICERR_INTERNAL: i32 = -4i32;
pub const ICERR_MEMORY: i32 = -3i32;
pub const ICERR_NEWPALETTE: i32 = 2i32;
pub const ICERR_OK: i32 = 0i32;
pub const ICERR_STOPDRAWING: i32 = 4i32;
pub const ICERR_UNSUPPORTED: i32 = -1i32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICGetDisplayFormat<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpbiout: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
    bitdepth: i32,
    dx: i32,
    dy: i32,
) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICGetDisplayFormat(
                hic: HIC,
                lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpbiout: *mut super::super::Graphics::Gdi::BITMAPINFOHEADER,
                bitdepth: i32,
                dx: i32,
                dy: i32,
            ) -> HIC;
        }
        ::std::mem::transmute(ICGetDisplayFormat(
            hic.into_param().abi(),
            ::std::mem::transmute(lpbiin),
            ::std::mem::transmute(lpbiout),
            ::std::mem::transmute(bitdepth),
            ::std::mem::transmute(dx),
            ::std::mem::transmute(dy),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    picinfo: *mut ICINFO,
    cb: u32,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICGetInfo(
                hic: HIC,
                picinfo: *mut ICINFO,
                cb: u32,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(ICGetInfo(
            hic.into_param().abi(),
            ::std::mem::transmute(picinfo),
            ::std::mem::transmute(cb),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICINFO {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwFlags: u32,
    pub dwVersion: u32,
    pub dwVersionICM: u32,
    pub szName: [u16; 16],
    pub szDescription: [u16; 128],
    pub szDriver: [u16; 128],
}
impl ICINFO {}
impl ::std::default::Default for ICINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ICINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICINFO")
            .field("dwSize", &self.dwSize)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwFlags", &self.dwFlags)
            .field("dwVersion", &self.dwVersion)
            .field("dwVersionICM", &self.dwVersionICM)
            .field("szName", &self.szName)
            .field("szDescription", &self.szDescription)
            .field("szDriver", &self.szDriver)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ICINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwFlags == other.dwFlags
            && self.dwVersion == other.dwVersion
            && self.dwVersionICM == other.dwVersionICM
            && self.szName == other.szName
            && self.szDescription == other.szDescription
            && self.szDriver == other.szDriver
    }
}
impl ::std::cmp::Eq for ICINFO {}
unsafe impl ::windows::runtime::Abi for ICINFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICINSTALL_DRIVER: u32 = 2u32;
pub const ICINSTALL_DRIVERW: u32 = 32770u32;
pub const ICINSTALL_FUNCTION: u32 = 1u32;
pub const ICINSTALL_HDRV: u32 = 4u32;
pub const ICINSTALL_UNICODE: u32 = 32768u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICImageCompress<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    uiflags: u32,
    lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
    lpbits: *const ::std::ffi::c_void,
    lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO,
    lquality: i32,
    plsize: *mut i32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICImageCompress(
                hic: HIC,
                uiflags: u32,
                lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
                lpbits: *const ::std::ffi::c_void,
                lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO,
                lquality: i32,
                plsize: *mut i32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(ICImageCompress(
            hic.into_param().abi(),
            ::std::mem::transmute(uiflags),
            ::std::mem::transmute(lpbiin),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(lpbiout),
            ::std::mem::transmute(lquality),
            ::std::mem::transmute(plsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICImageDecompress<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    uiflags: u32,
    lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
    lpbits: *const ::std::ffi::c_void,
    lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICImageDecompress(
                hic: HIC,
                uiflags: u32,
                lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
                lpbits: *const ::std::ffi::c_void,
                lpbiout: *const super::super::Graphics::Gdi::BITMAPINFO,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(ICImageDecompress(
            hic.into_param().abi(),
            ::std::mem::transmute(uiflags),
            ::std::mem::transmute(lpbiin),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(lpbiout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICInfo(
    fcctype: u32,
    fcchandler: u32,
    lpicinfo: *mut ICINFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICInfo(
                fcctype: u32,
                fcchandler: u32,
                lpicinfo: *mut ICINFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ICInfo(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            ::std::mem::transmute(lpicinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICInstall<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    fcctype: u32,
    fcchandler: u32,
    lparam: Param2,
    szdesc: Param3,
    wflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICInstall(
                fcctype: u32,
                fcchandler: u32,
                lparam: super::super::Foundation::LPARAM,
                szdesc: super::super::Foundation::PSTR,
                wflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ICInstall(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            lparam.into_param().abi(),
            szdesc.into_param().abi(),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICLocate(
    fcctype: u32,
    fcchandler: u32,
    lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    lpbiout: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
    wflags: u16,
) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICLocate(
                fcctype: u32,
                fcchandler: u32,
                lpbiin: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                lpbiout: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
                wflags: u16,
            ) -> HIC;
        }
        ::std::mem::transmute(ICLocate(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            ::std::mem::transmute(lpbiin),
            ::std::mem::transmute(lpbiout),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ICMF_ABOUT_QUERY: u32 = 1u32;
pub const ICMF_CHOOSE_ALLCOMPRESSORS: u32 = 8u32;
pub const ICMF_CHOOSE_DATARATE: u32 = 2u32;
pub const ICMF_CHOOSE_KEYFRAME: u32 = 1u32;
pub const ICMF_CHOOSE_PREVIEW: u32 = 4u32;
pub const ICMF_COMPVARS_VALID: u32 = 1u32;
pub const ICMF_CONFIGURE_QUERY: u32 = 1u32;
pub const ICMODE_COMPRESS: u32 = 1u32;
pub const ICMODE_DECOMPRESS: u32 = 2u32;
pub const ICMODE_DRAW: u32 = 8u32;
pub const ICMODE_FASTCOMPRESS: u32 = 5u32;
pub const ICMODE_FASTDECOMPRESS: u32 = 3u32;
pub const ICMODE_INTERNALF_FUNCTION32: u32 = 32768u32;
pub const ICMODE_INTERNALF_MASK: u32 = 32768u32;
pub const ICMODE_QUERY: u32 = 4u32;
pub const ICM_ABOUT: u32 = 20491u32;
pub const ICM_COMPRESS: u32 = 16392u32;
pub const ICM_COMPRESS_BEGIN: u32 = 16391u32;
pub const ICM_COMPRESS_END: u32 = 16393u32;
pub const ICM_COMPRESS_FRAMES: u32 = 16455u32;
pub const ICM_COMPRESS_FRAMES_INFO: u32 = 16454u32;
pub const ICM_COMPRESS_GET_FORMAT: u32 = 16388u32;
pub const ICM_COMPRESS_GET_SIZE: u32 = 16389u32;
pub const ICM_COMPRESS_QUERY: u32 = 16390u32;
pub const ICM_CONFIGURE: u32 = 20490u32;
pub const ICM_DECOMPRESS: u32 = 16397u32;
pub const ICM_DECOMPRESSEX: u32 = 16446u32;
pub const ICM_DECOMPRESSEX_BEGIN: u32 = 16444u32;
pub const ICM_DECOMPRESSEX_END: u32 = 16447u32;
pub const ICM_DECOMPRESSEX_QUERY: u32 = 16445u32;
pub const ICM_DECOMPRESS_BEGIN: u32 = 16396u32;
pub const ICM_DECOMPRESS_END: u32 = 16398u32;
pub const ICM_DECOMPRESS_GET_FORMAT: u32 = 16394u32;
pub const ICM_DECOMPRESS_GET_PALETTE: u32 = 16414u32;
pub const ICM_DECOMPRESS_QUERY: u32 = 16395u32;
pub const ICM_DECOMPRESS_SET_PALETTE: u32 = 16413u32;
pub const ICM_DRAW: u32 = 16417u32;
pub const ICM_DRAW_BEGIN: u32 = 16399u32;
pub const ICM_DRAW_BITS: u32 = 16404u32;
pub const ICM_DRAW_CHANGEPALETTE: u32 = 16435u32;
pub const ICM_DRAW_END: u32 = 16405u32;
pub const ICM_DRAW_FLUSH: u32 = 16421u32;
pub const ICM_DRAW_GETTIME: u32 = 16416u32;
pub const ICM_DRAW_GET_PALETTE: u32 = 16400u32;
pub const ICM_DRAW_IDLE: u32 = 16436u32;
pub const ICM_DRAW_QUERY: u32 = 16415u32;
pub const ICM_DRAW_REALIZE: u32 = 16420u32;
pub const ICM_DRAW_RENDERBUFFER: u32 = 16422u32;
pub const ICM_DRAW_SETTIME: u32 = 16419u32;
pub const ICM_DRAW_START: u32 = 16402u32;
pub const ICM_DRAW_START_PLAY: u32 = 16423u32;
pub const ICM_DRAW_STOP: u32 = 16403u32;
pub const ICM_DRAW_STOP_PLAY: u32 = 16424u32;
pub const ICM_DRAW_SUGGESTFORMAT: u32 = 16434u32;
pub const ICM_DRAW_UPDATE: u32 = 16401u32;
pub const ICM_DRAW_WINDOW: u32 = 16418u32;
pub const ICM_ENUMFORMATS: u32 = 20501u32;
pub const ICM_GET: u32 = 20521u32;
pub const ICM_GETBUFFERSWANTED: u32 = 16425u32;
pub const ICM_GETDEFAULTKEYFRAMERATE: u32 = 16426u32;
pub const ICM_GETDEFAULTQUALITY: u32 = 20510u32;
pub const ICM_GETERRORTEXT: u32 = 20492u32;
pub const ICM_GETFORMATNAME: u32 = 20500u32;
pub const ICM_GETINFO: u32 = 20482u32;
pub const ICM_GETQUALITY: u32 = 20511u32;
pub const ICM_GETSTATE: u32 = 20480u32;
pub const ICM_RESERVED: u32 = 20480u32;
pub const ICM_RESERVED_HIGH: u32 = 24576u32;
pub const ICM_RESERVED_LOW: u32 = 20480u32;
pub const ICM_SET: u32 = 20520u32;
pub const ICM_SETQUALITY: u32 = 20512u32;
pub const ICM_SETSTATE: u32 = 20481u32;
pub const ICM_SET_STATUS_PROC: u32 = 16456u32;
pub const ICM_USER: u32 = 16384u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ICOPEN {
    pub dwSize: u32,
    pub fccType: u32,
    pub fccHandler: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: super::super::Foundation::LRESULT,
    pub pV1Reserved: *mut ::std::ffi::c_void,
    pub pV2Reserved: *mut ::std::ffi::c_void,
    pub dnDevNode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ICOPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ICOPEN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ICOPEN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICOPEN")
            .field("dwSize", &self.dwSize)
            .field("fccType", &self.fccType)
            .field("fccHandler", &self.fccHandler)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("dwError", &self.dwError)
            .field("pV1Reserved", &self.pV1Reserved)
            .field("pV2Reserved", &self.pV2Reserved)
            .field("dnDevNode", &self.dnDevNode)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ICOPEN {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fccType == other.fccType
            && self.fccHandler == other.fccHandler
            && self.dwVersion == other.dwVersion
            && self.dwFlags == other.dwFlags
            && self.dwError == other.dwError
            && self.pV1Reserved == other.pV1Reserved
            && self.pV2Reserved == other.pV2Reserved
            && self.dnDevNode == other.dnDevNode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ICOPEN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ICOPEN {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICOpen(fcctype: u32, fcchandler: u32, wmode: u32) -> HIC;
        }
        ::std::mem::transmute(ICOpen(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            ::std::mem::transmute(wmode),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICOpenFunction(
    fcctype: u32,
    fcchandler: u32,
    wmode: u32,
    lpfnhandler: ::std::option::Option<super::super::Foundation::FARPROC>,
) -> HIC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICOpenFunction(
                fcctype: u32,
                fcchandler: u32,
                wmode: u32,
                lpfnhandler: ::windows::runtime::RawPtr,
            ) -> HIC;
        }
        ::std::mem::transmute(ICOpenFunction(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            ::std::mem::transmute(wmode),
            ::std::mem::transmute(lpfnhandler),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct ICPALETTE {
    pub dwFlags: u32,
    pub iStart: i32,
    pub iLen: i32,
    pub lppe: *mut super::super::Graphics::Gdi::PALETTEENTRY,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ICPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for ICPALETTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for ICPALETTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICPALETTE")
            .field("dwFlags", &self.dwFlags)
            .field("iStart", &self.iStart)
            .field("iLen", &self.iLen)
            .field("lppe", &self.lppe)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for ICPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
            && self.iStart == other.iStart
            && self.iLen == other.iLen
            && self.lppe == other.lppe
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for ICPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for ICPALETTE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICQUALITY_DEFAULT: i32 = -1i32;
pub const ICQUALITY_HIGH: u32 = 10000u32;
pub const ICQUALITY_LOW: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICRemove(
    fcctype: u32,
    fcchandler: u32,
    wflags: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICRemove(
                fcctype: u32,
                fcchandler: u32,
                wflags: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ICRemove(
            ::std::mem::transmute(fcctype),
            ::std::mem::transmute(fcchandler),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ICSETSTATUSPROC {
    pub dwFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub Status: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ICSETSTATUSPROC {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ICSETSTATUSPROC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ICSETSTATUSPROC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICSETSTATUSPROC")
            .field("dwFlags", &self.dwFlags)
            .field("lParam", &self.lParam)
            .field("Status", &self.Status)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ICSETSTATUSPROC {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lParam == other.lParam && self.Status == other.Status
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ICSETSTATUSPROC {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ICSETSTATUSPROC {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICSTATUS_END: u32 = 2u32;
pub const ICSTATUS_ERROR: u32 = 3u32;
pub const ICSTATUS_START: u32 = 0u32;
pub const ICSTATUS_STATUS: u32 = 1u32;
pub const ICSTATUS_YIELD: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ICSendMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HIC>>(
    hic: Param0,
    msg: u32,
    dw1: usize,
    dw2: usize,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSendMessage(
                hic: HIC,
                msg: u32,
                dw1: usize,
                dw2: usize,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(ICSendMessage(
            hic.into_param().abi(),
            ::std::mem::transmute(msg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICSeqCompressFrame(
    pc: *const COMPVARS,
    uiflags: u32,
    lpbits: *const ::std::ffi::c_void,
    pfkey: *mut super::super::Foundation::BOOL,
    plsize: *mut i32,
) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrame(
                pc: *const COMPVARS,
                uiflags: u32,
                lpbits: *const ::std::ffi::c_void,
                pfkey: *mut super::super::Foundation::BOOL,
                plsize: *mut i32,
            ) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(ICSeqCompressFrame(
            ::std::mem::transmute(pc),
            ::std::mem::transmute(uiflags),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(pfkey),
            ::std::mem::transmute(plsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ICSeqCompressFrameEnd(pc: *const COMPVARS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrameEnd(pc: *const COMPVARS);
        }
        ::std::mem::transmute(ICSeqCompressFrameEnd(::std::mem::transmute(pc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ICSeqCompressFrameStart(
    pc: *const COMPVARS,
    lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ICSeqCompressFrameStart(
                pc: *const COMPVARS,
                lpbiin: *const super::super::Graphics::Gdi::BITMAPINFO,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ICSeqCompressFrameStart(
            ::std::mem::transmute(pc),
            ::std::mem::transmute(lpbiin),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ICVERSION: u32 = 260u32;
pub const IDD_ACMFILTERCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFILTERCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFILTERCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFILTERCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTER: u32 = 102u32;
pub const IDD_ACMFILTERCHOOSE_CMB_FILTERTAG: u32 = 101u32;
pub const IDD_ACMFORMATCHOOSE_BTN_DELNAME: u32 = 104u32;
pub const IDD_ACMFORMATCHOOSE_BTN_HELP: u32 = 9u32;
pub const IDD_ACMFORMATCHOOSE_BTN_SETNAME: u32 = 103u32;
pub const IDD_ACMFORMATCHOOSE_CMB_CUSTOM: u32 = 100u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMAT: u32 = 102u32;
pub const IDD_ACMFORMATCHOOSE_CMB_FORMATTAG: u32 = 101u32;
pub const IDS_CAP_AUDIO_DROP_COMPERROR: u32 = 442u32;
pub const IDS_CAP_AUDIO_DROP_ERROR: u32 = 441u32;
pub const IDS_CAP_AVI_DRAWDIB_ERROR: u32 = 439u32;
pub const IDS_CAP_AVI_INIT_ERROR: u32 = 433u32;
pub const IDS_CAP_BEGIN: u32 = 300u32;
pub const IDS_CAP_CANTOPEN: u32 = 409u32;
pub const IDS_CAP_COMPRESSOR_ERROR: u32 = 440u32;
pub const IDS_CAP_DEFAVIEXT: u32 = 407u32;
pub const IDS_CAP_DEFPALEXT: u32 = 408u32;
pub const IDS_CAP_DRIVER_ERROR: u32 = 418u32;
pub const IDS_CAP_END: u32 = 301u32;
pub const IDS_CAP_ERRORDIBSAVE: u32 = 406u32;
pub const IDS_CAP_ERRORPALOPEN: u32 = 404u32;
pub const IDS_CAP_ERRORPALSAVE: u32 = 405u32;
pub const IDS_CAP_FILEEXISTS: u32 = 403u32;
pub const IDS_CAP_FILE_OPEN_ERROR: u32 = 429u32;
pub const IDS_CAP_FILE_WRITE_ERROR: u32 = 430u32;
pub const IDS_CAP_INFO: u32 = 401u32;
pub const IDS_CAP_MCI_CANT_STEP_ERROR: u32 = 437u32;
pub const IDS_CAP_MCI_CONTROL_ERROR: u32 = 436u32;
pub const IDS_CAP_NODISKSPACE: u32 = 415u32;
pub const IDS_CAP_NO_AUDIO_CAP_ERROR: u32 = 438u32;
pub const IDS_CAP_NO_FRAME_CAP_ERROR: u32 = 434u32;
pub const IDS_CAP_NO_PALETTE_WARN: u32 = 435u32;
pub const IDS_CAP_OUTOFMEM: u32 = 402u32;
pub const IDS_CAP_READONLYFILE: u32 = 413u32;
pub const IDS_CAP_RECORDING_ERROR: u32 = 431u32;
pub const IDS_CAP_RECORDING_ERROR2: u32 = 432u32;
pub const IDS_CAP_SAVEASPERCENT: u32 = 417u32;
pub const IDS_CAP_SEQ_MSGSTART: u32 = 410u32;
pub const IDS_CAP_SEQ_MSGSTOP: u32 = 411u32;
pub const IDS_CAP_SETFILESIZE: u32 = 416u32;
pub const IDS_CAP_STAT_CAP_AUDIO: u32 = 509u32;
pub const IDS_CAP_STAT_CAP_FINI: u32 = 503u32;
pub const IDS_CAP_STAT_CAP_INIT: u32 = 502u32;
pub const IDS_CAP_STAT_CAP_L_FRAMES: u32 = 508u32;
pub const IDS_CAP_STAT_FRAMESDROPPED: u32 = 513u32;
pub const IDS_CAP_STAT_I_FRAMES: u32 = 506u32;
pub const IDS_CAP_STAT_LIVE_MODE: u32 = 500u32;
pub const IDS_CAP_STAT_L_FRAMES: u32 = 507u32;
pub const IDS_CAP_STAT_OPTPAL_BUILD: u32 = 505u32;
pub const IDS_CAP_STAT_OVERLAY_MODE: u32 = 501u32;
pub const IDS_CAP_STAT_PALETTE_BUILD: u32 = 504u32;
pub const IDS_CAP_STAT_VIDEOAUDIO: u32 = 511u32;
pub const IDS_CAP_STAT_VIDEOCURRENT: u32 = 510u32;
pub const IDS_CAP_STAT_VIDEOONLY: u32 = 512u32;
pub const IDS_CAP_VIDEDITERR: u32 = 412u32;
pub const IDS_CAP_VIDEO_ADD_ERROR: u32 = 427u32;
pub const IDS_CAP_VIDEO_ALLOC_ERROR: u32 = 425u32;
pub const IDS_CAP_VIDEO_OPEN_ERROR: u32 = 424u32;
pub const IDS_CAP_VIDEO_PREPARE_ERROR: u32 = 426u32;
pub const IDS_CAP_VIDEO_SIZE_ERROR: u32 = 428u32;
pub const IDS_CAP_WAVE_ADD_ERROR: u32 = 422u32;
pub const IDS_CAP_WAVE_ALLOC_ERROR: u32 = 420u32;
pub const IDS_CAP_WAVE_OPEN_ERROR: u32 = 419u32;
pub const IDS_CAP_WAVE_PREPARE_ERROR: u32 = 421u32;
pub const IDS_CAP_WAVE_SIZE_ERROR: u32 = 423u32;
pub const IDS_CAP_WRITEERROR: u32 = 414u32;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGetFrame(::windows::runtime::IUnknown);
impl IGetFrame {
    pub unsafe fn GetFrame(&self, lpos: i32) -> *mut ::std::ffi::c_void {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpos),
        ))
    }
    pub unsafe fn Begin(
        &self,
        lstart: i32,
        lend: i32,
        lrate: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lstart),
            ::std::mem::transmute(lend),
            ::std::mem::transmute(lrate),
        )
        .ok()
    }
    pub unsafe fn End(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetFormat(
        &self,
        lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
        lpbits: *const ::std::ffi::c_void,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(lpbi),
            ::std::mem::transmute(lpbits),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(dx),
            ::std::mem::transmute(dy),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGetFrame {
    type Vtable = IGetFrame_abi;
    const IID: ::windows::runtime::GUID =
        ::windows::runtime::GUID::from_values(131107, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IGetFrame> for ::windows::runtime::IUnknown {
    fn from(value: IGetFrame) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGetFrame> for ::windows::runtime::IUnknown {
    fn from(value: &IGetFrame) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGetFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGetFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetFrame_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpos: i32,
    ) -> *mut ::std::ffi::c_void,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lstart: i32,
        lend: i32,
        lrate: i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER,
        lpbits: *const ::std::ffi::c_void,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct IMAADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl IMAADPCMWAVEFORMAT {}
impl ::std::default::Default for IMAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IMAADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IMAADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for IMAADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JDD_CONFIGCHANGED: u32 = 2307u32;
pub const JDD_GETDEVCAPS: u32 = 2050u32;
pub const JDD_GETNUMDEVS: u32 = 2049u32;
pub const JDD_GETPOS: u32 = 2305u32;
pub const JDD_GETPOSEX: u32 = 2308u32;
pub const JDD_SETCALIBRATION: u32 = 2306u32;
pub const JIFMK_00: u32 = 65280u32;
pub const JIFMK_APP0: u32 = 65504u32;
pub const JIFMK_APP1: u32 = 65505u32;
pub const JIFMK_APP2: u32 = 65506u32;
pub const JIFMK_APP3: u32 = 65507u32;
pub const JIFMK_APP4: u32 = 65508u32;
pub const JIFMK_APP5: u32 = 65509u32;
pub const JIFMK_APP6: u32 = 65510u32;
pub const JIFMK_APP7: u32 = 65511u32;
pub const JIFMK_COM: u32 = 65534u32;
pub const JIFMK_DAC: u32 = 65484u32;
pub const JIFMK_DHP: u32 = 65502u32;
pub const JIFMK_DHT: u32 = 65476u32;
pub const JIFMK_DNL: u32 = 65500u32;
pub const JIFMK_DQT: u32 = 65499u32;
pub const JIFMK_DRI: u32 = 65501u32;
pub const JIFMK_EOI: u32 = 65497u32;
pub const JIFMK_EXP: u32 = 65503u32;
pub const JIFMK_FF: u32 = 65535u32;
pub const JIFMK_JPG: u32 = 65480u32;
pub const JIFMK_JPG0: u32 = 65520u32;
pub const JIFMK_JPG1: u32 = 65521u32;
pub const JIFMK_JPG10: u32 = 65530u32;
pub const JIFMK_JPG11: u32 = 65531u32;
pub const JIFMK_JPG12: u32 = 65532u32;
pub const JIFMK_JPG13: u32 = 65533u32;
pub const JIFMK_JPG2: u32 = 65522u32;
pub const JIFMK_JPG3: u32 = 65523u32;
pub const JIFMK_JPG4: u32 = 65524u32;
pub const JIFMK_JPG5: u32 = 65525u32;
pub const JIFMK_JPG6: u32 = 65526u32;
pub const JIFMK_JPG7: u32 = 65527u32;
pub const JIFMK_JPG8: u32 = 65528u32;
pub const JIFMK_JPG9: u32 = 65529u32;
pub const JIFMK_RES: u32 = 65282u32;
pub const JIFMK_RST0: u32 = 65488u32;
pub const JIFMK_RST1: u32 = 65489u32;
pub const JIFMK_RST2: u32 = 65490u32;
pub const JIFMK_RST3: u32 = 65491u32;
pub const JIFMK_RST4: u32 = 65492u32;
pub const JIFMK_RST5: u32 = 65493u32;
pub const JIFMK_RST6: u32 = 65494u32;
pub const JIFMK_RST7: u32 = 65495u32;
pub const JIFMK_SOF0: u32 = 65472u32;
pub const JIFMK_SOF1: u32 = 65473u32;
pub const JIFMK_SOF10: u32 = 65482u32;
pub const JIFMK_SOF11: u32 = 65483u32;
pub const JIFMK_SOF13: u32 = 65485u32;
pub const JIFMK_SOF14: u32 = 65486u32;
pub const JIFMK_SOF15: u32 = 65487u32;
pub const JIFMK_SOF2: u32 = 65474u32;
pub const JIFMK_SOF3: u32 = 65475u32;
pub const JIFMK_SOF5: u32 = 65477u32;
pub const JIFMK_SOF6: u32 = 65478u32;
pub const JIFMK_SOF7: u32 = 65479u32;
pub const JIFMK_SOF9: u32 = 65481u32;
pub const JIFMK_SOI: u32 = 65496u32;
pub const JIFMK_SOS: u32 = 65498u32;
pub const JIFMK_TEM: u32 = 65281u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYCALIBRATE {
    pub wXbase: u16,
    pub wXdelta: u16,
    pub wYbase: u16,
    pub wYdelta: u16,
    pub wZbase: u16,
    pub wZdelta: u16,
}
impl JOYCALIBRATE {}
impl ::std::default::Default for JOYCALIBRATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYCALIBRATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYCALIBRATE {}
unsafe impl ::windows::runtime::Abi for JOYCALIBRATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct JOYCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl JOYCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JOYCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JOYCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JOYCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JOYCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl JOYCAPS2W {}
impl ::std::default::Default for JOYCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYCAPS2W {}
unsafe impl ::windows::runtime::Abi for JOYCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct JOYCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [super::super::Foundation::CHAR; 32],
    pub szOEMVxD: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl JOYCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JOYCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JOYCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JOYCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JOYCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub szPname: [u16; 32],
    pub wXmin: u32,
    pub wXmax: u32,
    pub wYmin: u32,
    pub wYmax: u32,
    pub wZmin: u32,
    pub wZmax: u32,
    pub wNumButtons: u32,
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
    pub wRmin: u32,
    pub wRmax: u32,
    pub wUmin: u32,
    pub wUmax: u32,
    pub wVmin: u32,
    pub wVmax: u32,
    pub wCaps: u32,
    pub wMaxAxes: u32,
    pub wNumAxes: u32,
    pub wMaxButtons: u32,
    pub szRegKey: [u16; 32],
    pub szOEMVxD: [u16; 260],
}
impl JOYCAPSW {}
impl ::std::default::Default for JOYCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYCAPSW {}
unsafe impl ::windows::runtime::Abi for JOYCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JOYCAPS_HASPOV: u32 = 16u32;
pub const JOYCAPS_HASR: u32 = 2u32;
pub const JOYCAPS_HASU: u32 = 4u32;
pub const JOYCAPS_HASV: u32 = 8u32;
pub const JOYCAPS_HASZ: u32 = 1u32;
pub const JOYCAPS_POV4DIR: u32 = 32u32;
pub const JOYCAPS_POVCTS: u32 = 64u32;
pub const JOYERR_BASE: u32 = 160u32;
pub const JOYERR_NOCANDO: u32 = 166u32;
pub const JOYERR_NOERROR: u32 = 0u32;
pub const JOYERR_PARMS: u32 = 165u32;
pub const JOYERR_UNPLUGGED: u32 = 167u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYINFO {
    pub wXpos: u32,
    pub wYpos: u32,
    pub wZpos: u32,
    pub wButtons: u32,
}
impl JOYINFO {}
impl ::std::default::Default for JOYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYINFO {}
unsafe impl ::windows::runtime::Abi for JOYINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYINFOEX {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwXpos: u32,
    pub dwYpos: u32,
    pub dwZpos: u32,
    pub dwRpos: u32,
    pub dwUpos: u32,
    pub dwVpos: u32,
    pub dwButtons: u32,
    pub dwButtonNumber: u32,
    pub dwPOV: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl JOYINFOEX {}
impl ::std::default::Default for JOYINFOEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYINFOEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYINFOEX {}
unsafe impl ::windows::runtime::Abi for JOYINFOEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYPOS {
    pub dwX: u32,
    pub dwY: u32,
    pub dwZ: u32,
    pub dwR: u32,
    pub dwU: u32,
    pub dwV: u32,
}
impl JOYPOS {}
impl ::std::default::Default for JOYPOS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYPOS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYPOS {}
unsafe impl ::windows::runtime::Abi for JOYPOS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JOYRANGE {
    pub jpMin: JOYPOS,
    pub jpMax: JOYPOS,
    pub jpCenter: JOYPOS,
}
impl JOYRANGE {}
impl ::std::default::Default for JOYRANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYRANGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYRANGE {}
unsafe impl ::windows::runtime::Abi for JOYRANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
pub struct JOYREGHWCONFIG {
    pub hws: JOYREGHWSETTINGS,
    pub dwUsageSettings: u32,
    pub hwv: super::super::Devices::HumanInterfaceDevice::JOYREGHWVALUES,
    pub dwType: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
impl JOYREGHWCONFIG {}
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
impl ::std::default::Default for JOYREGHWCONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
impl ::std::cmp::PartialEq for JOYREGHWCONFIG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
impl ::std::cmp::Eq for JOYREGHWCONFIG {}
#[cfg(feature = "Win32_Devices_HumanInterfaceDevice")]
unsafe impl ::windows::runtime::Abi for JOYREGHWCONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYREGHWSETTINGS {
    pub dwFlags: u32,
    pub dwNumButtons: u32,
}
impl JOYREGHWSETTINGS {}
impl ::std::default::Default for JOYREGHWSETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYREGHWSETTINGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYREGHWSETTINGS {}
unsafe impl ::windows::runtime::Abi for JOYREGHWSETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JOYREGUSERVALUES {
    pub dwTimeOut: u32,
    pub jrvRanges: JOYRANGE,
    pub jpDeadZone: JOYPOS,
}
impl JOYREGUSERVALUES {}
impl ::std::default::Default for JOYREGUSERVALUES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JOYREGUSERVALUES {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JOYREGUSERVALUES {}
unsafe impl ::windows::runtime::Abi for JOYREGUSERVALUES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JOYSTICKID1: u32 = 0u32;
pub const JOYSTICKID2: u32 = 1u32;
pub const JOY_BUTTON1: u32 = 1u32;
pub const JOY_BUTTON10: i32 = 512i32;
pub const JOY_BUTTON11: i32 = 1024i32;
pub const JOY_BUTTON12: i32 = 2048i32;
pub const JOY_BUTTON13: i32 = 4096i32;
pub const JOY_BUTTON14: i32 = 8192i32;
pub const JOY_BUTTON15: i32 = 16384i32;
pub const JOY_BUTTON16: i32 = 32768i32;
pub const JOY_BUTTON17: i32 = 65536i32;
pub const JOY_BUTTON18: i32 = 131072i32;
pub const JOY_BUTTON19: i32 = 262144i32;
pub const JOY_BUTTON1CHG: u32 = 256u32;
pub const JOY_BUTTON2: u32 = 2u32;
pub const JOY_BUTTON20: i32 = 524288i32;
pub const JOY_BUTTON21: i32 = 1048576i32;
pub const JOY_BUTTON22: i32 = 2097152i32;
pub const JOY_BUTTON23: i32 = 4194304i32;
pub const JOY_BUTTON24: i32 = 8388608i32;
pub const JOY_BUTTON25: i32 = 16777216i32;
pub const JOY_BUTTON26: i32 = 33554432i32;
pub const JOY_BUTTON27: i32 = 67108864i32;
pub const JOY_BUTTON28: i32 = 134217728i32;
pub const JOY_BUTTON29: i32 = 268435456i32;
pub const JOY_BUTTON2CHG: u32 = 512u32;
pub const JOY_BUTTON3: u32 = 4u32;
pub const JOY_BUTTON30: i32 = 536870912i32;
pub const JOY_BUTTON31: i32 = 1073741824i32;
pub const JOY_BUTTON32: i32 = -2147483648i32;
pub const JOY_BUTTON3CHG: u32 = 1024u32;
pub const JOY_BUTTON4: u32 = 8u32;
pub const JOY_BUTTON4CHG: u32 = 2048u32;
pub const JOY_BUTTON5: i32 = 16i32;
pub const JOY_BUTTON6: i32 = 32i32;
pub const JOY_BUTTON7: i32 = 64i32;
pub const JOY_BUTTON8: i32 = 128i32;
pub const JOY_BUTTON9: i32 = 256i32;
pub const JOY_CAL_READ3: i32 = 262144i32;
pub const JOY_CAL_READ4: i32 = 524288i32;
pub const JOY_CAL_READ5: i32 = 4194304i32;
pub const JOY_CAL_READ6: i32 = 8388608i32;
pub const JOY_CAL_READALWAYS: i32 = 65536i32;
pub const JOY_CAL_READRONLY: i32 = 33554432i32;
pub const JOY_CAL_READUONLY: i32 = 67108864i32;
pub const JOY_CAL_READVONLY: i32 = 134217728i32;
pub const JOY_CAL_READXONLY: i32 = 1048576i32;
pub const JOY_CAL_READXYONLY: i32 = 131072i32;
pub const JOY_CAL_READYONLY: i32 = 2097152i32;
pub const JOY_CAL_READZONLY: i32 = 16777216i32;
pub const JOY_POVBACKWARD: u32 = 18000u32;
pub const JOY_POVFORWARD: u32 = 0u32;
pub const JOY_POVLEFT: u32 = 27000u32;
pub const JOY_POVRIGHT: u32 = 9000u32;
pub const JOY_RETURNBUTTONS: i32 = 128i32;
pub const JOY_RETURNCENTERED: i32 = 1024i32;
pub const JOY_RETURNPOV: i32 = 64i32;
pub const JOY_RETURNPOVCTS: i32 = 512i32;
pub const JOY_RETURNR: i32 = 8i32;
pub const JOY_RETURNRAWDATA: i32 = 256i32;
pub const JOY_RETURNU: i32 = 16i32;
pub const JOY_RETURNV: i32 = 32i32;
pub const JOY_RETURNX: i32 = 1i32;
pub const JOY_RETURNY: i32 = 2i32;
pub const JOY_RETURNZ: i32 = 4i32;
pub const JOY_USEDEADZONE: i32 = 2048i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct JPEGINFOHEADER {
    pub JPEGSize: u32,
    pub JPEGProcess: u32,
    pub JPEGColorSpaceID: u32,
    pub JPEGBitsPerSample: u32,
    pub JPEGHSubSampling: u32,
    pub JPEGVSubSampling: u32,
}
impl JPEGINFOHEADER {}
impl ::std::default::Default for JPEGINFOHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for JPEGINFOHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for JPEGINFOHEADER {}
unsafe impl ::windows::runtime::Abi for JPEGINFOHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const JPEG_PROCESS_BASELINE: u32 = 0u32;
pub const JPEG_RGB: u32 = 3u32;
pub const JPEG_Y: u32 = 1u32;
pub const JPEG_YCbCr: u32 = 2u32;
pub const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(3, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_PCM: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(1, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
pub const KSDATAFORMAT_SUBTYPE_WAVEFORMATEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 16, [128, 0, 0, 170, 0, 56, 155, 113]);
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = unsafe extern "system" fn(
    param0: usize,
    param1: HACMDRIVERID,
    param2: u32,
    param3: super::super::Foundation::LPARAM,
    param4: super::super::Foundation::LPARAM,
) -> super::super::Foundation::LRESULT;
pub type LPDRVCALLBACK =
    unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFNEXTDEVIO = unsafe extern "system" fn(
    lparam: super::super::Foundation::LPARAM,
    dwflags: u32,
    dwiocontrolcode: u32,
    lpinbuffer: *mut ::std::ffi::c_void,
    ninbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    noutbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpoverlapped: *mut super::super::System::IO::OVERLAPPED,
) -> super::super::Foundation::BOOL;
pub type LPJOYDEVMSGPROC =
    unsafe extern "system" fn(param0: u32, param1: u32, param2: i32, param3: i32) -> u32;
pub type LPMIDICALLBACK =
    unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[cfg(feature = "Win32_Foundation")]
pub type LPMMIOPROC = unsafe extern "system" fn(
    lpmmioinfo: super::super::Foundation::PSTR,
    umsg: u32,
    lparam1: super::super::Foundation::LPARAM,
    lparam2: super::super::Foundation::LPARAM,
) -> super::super::Foundation::LRESULT;
pub type LPTASKCALLBACK = unsafe extern "system" fn(dwinst: usize);
pub type LPWAVECALLBACK =
    unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
pub const MAXERRORLENGTH: u32 = 256u32;
pub const MAXPNAMELEN: u32 = 32u32;
pub const MCIERR_AVI_AUDIOERROR: u32 = 619u32;
pub const MCIERR_AVI_BADPALETTE: u32 = 620u32;
pub const MCIERR_AVI_CANTPLAYFULLSCREEN: u32 = 615u32;
pub const MCIERR_AVI_DISPLAYERROR: u32 = 618u32;
pub const MCIERR_AVI_NOCOMPRESSOR: u32 = 617u32;
pub const MCIERR_AVI_NODISPDIB: u32 = 614u32;
pub const MCIERR_AVI_NOTINTERLEAVED: u32 = 613u32;
pub const MCIERR_AVI_OLDAVIFORMAT: u32 = 612u32;
pub const MCIERR_AVI_TOOBIGFORVGA: u32 = 616u32;
pub const MCIERR_BASE: u32 = 256u32;
pub const MCIERR_DGV_BAD_CLIPBOARD_RANGE: u32 = 517u32;
pub const MCIERR_DGV_DEVICE_LIMIT: u32 = 512u32;
pub const MCIERR_DGV_DEVICE_MEMORY_FULL: u32 = 516u32;
pub const MCIERR_DGV_DISK_FULL: u32 = 515u32;
pub const MCIERR_DGV_IOERR: u32 = 513u32;
pub const MCIERR_DGV_WORKSPACE_EMPTY: u32 = 514u32;
pub const MCIWNDF_NOAUTOSIZEMOVIE: u32 = 4u32;
pub const MCIWNDF_NOAUTOSIZEWINDOW: u32 = 1u32;
pub const MCIWNDF_NOERRORDLG: u32 = 16384u32;
pub const MCIWNDF_NOMENU: u32 = 8u32;
pub const MCIWNDF_NOOPEN: u32 = 32768u32;
pub const MCIWNDF_NOPLAYBAR: u32 = 2u32;
pub const MCIWNDF_NOTIFYALL: u32 = 7936u32;
pub const MCIWNDF_NOTIFYANSI: u32 = 128u32;
pub const MCIWNDF_NOTIFYERROR: u32 = 4096u32;
pub const MCIWNDF_NOTIFYMEDIA: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMEDIAA: u32 = 2176u32;
pub const MCIWNDF_NOTIFYMEDIAW: u32 = 2048u32;
pub const MCIWNDF_NOTIFYMODE: u32 = 256u32;
pub const MCIWNDF_NOTIFYPOS: u32 = 512u32;
pub const MCIWNDF_NOTIFYSIZE: u32 = 1024u32;
pub const MCIWNDF_RECORD: u32 = 8192u32;
pub const MCIWNDF_SHOWALL: u32 = 112u32;
pub const MCIWNDF_SHOWMODE: u32 = 64u32;
pub const MCIWNDF_SHOWNAME: u32 = 16u32;
pub const MCIWNDF_SHOWPOS: u32 = 32u32;
pub const MCIWNDM_CAN_CONFIG: u32 = 1173u32;
pub const MCIWNDM_CAN_EJECT: u32 = 1172u32;
pub const MCIWNDM_CAN_PLAY: u32 = 1168u32;
pub const MCIWNDM_CAN_RECORD: u32 = 1170u32;
pub const MCIWNDM_CAN_SAVE: u32 = 1171u32;
pub const MCIWNDM_CAN_WINDOW: u32 = 1169u32;
pub const MCIWNDM_CHANGESTYLES: u32 = 1159u32;
pub const MCIWNDM_EJECT: u32 = 1131u32;
pub const MCIWNDM_GETACTIVETIMER: u32 = 1156u32;
pub const MCIWNDM_GETALIAS: u32 = 1161u32;
pub const MCIWNDM_GETDEVICE: u32 = 1249u32;
pub const MCIWNDM_GETDEVICEA: u32 = 1149u32;
pub const MCIWNDM_GETDEVICEID: u32 = 1124u32;
pub const MCIWNDM_GETDEVICEW: u32 = 1249u32;
pub const MCIWNDM_GETEND: u32 = 1129u32;
pub const MCIWNDM_GETERROR: u32 = 1252u32;
pub const MCIWNDM_GETERRORA: u32 = 1152u32;
pub const MCIWNDM_GETERRORW: u32 = 1252u32;
pub const MCIWNDM_GETFILENAME: u32 = 1248u32;
pub const MCIWNDM_GETFILENAMEA: u32 = 1148u32;
pub const MCIWNDM_GETFILENAMEW: u32 = 1248u32;
pub const MCIWNDM_GETINACTIVETIMER: u32 = 1157u32;
pub const MCIWNDM_GETLENGTH: u32 = 1128u32;
pub const MCIWNDM_GETMODE: u32 = 1230u32;
pub const MCIWNDM_GETMODEA: u32 = 1130u32;
pub const MCIWNDM_GETMODEW: u32 = 1230u32;
pub const MCIWNDM_GETPALETTE: u32 = 1150u32;
pub const MCIWNDM_GETPOSITION: u32 = 1226u32;
pub const MCIWNDM_GETPOSITIONA: u32 = 1126u32;
pub const MCIWNDM_GETPOSITIONW: u32 = 1226u32;
pub const MCIWNDM_GETREPEAT: u32 = 1139u32;
pub const MCIWNDM_GETSPEED: u32 = 1137u32;
pub const MCIWNDM_GETSTART: u32 = 1127u32;
pub const MCIWNDM_GETSTYLES: u32 = 1160u32;
pub const MCIWNDM_GETTIMEFORMAT: u32 = 1244u32;
pub const MCIWNDM_GETTIMEFORMATA: u32 = 1144u32;
pub const MCIWNDM_GETTIMEFORMATW: u32 = 1244u32;
pub const MCIWNDM_GETVOLUME: u32 = 1135u32;
pub const MCIWNDM_GETZOOM: u32 = 1133u32;
pub const MCIWNDM_GET_DEST: u32 = 1166u32;
pub const MCIWNDM_GET_SOURCE: u32 = 1164u32;
pub const MCIWNDM_NEW: u32 = 1258u32;
pub const MCIWNDM_NEWA: u32 = 1158u32;
pub const MCIWNDM_NEWW: u32 = 1258u32;
pub const MCIWNDM_NOTIFYERROR: u32 = 1229u32;
pub const MCIWNDM_NOTIFYMEDIA: u32 = 1227u32;
pub const MCIWNDM_NOTIFYMODE: u32 = 1224u32;
pub const MCIWNDM_NOTIFYPOS: u32 = 1225u32;
pub const MCIWNDM_NOTIFYSIZE: u32 = 1226u32;
pub const MCIWNDM_OPEN: u32 = 1276u32;
pub const MCIWNDM_OPENA: u32 = 1177u32;
pub const MCIWNDM_OPENINTERFACE: u32 = 1175u32;
pub const MCIWNDM_OPENW: u32 = 1276u32;
pub const MCIWNDM_PALETTEKICK: u32 = 1174u32;
pub const MCIWNDM_PLAYFROM: u32 = 1146u32;
pub const MCIWNDM_PLAYREVERSE: u32 = 1163u32;
pub const MCIWNDM_PLAYTO: u32 = 1147u32;
pub const MCIWNDM_PUT_DEST: u32 = 1167u32;
pub const MCIWNDM_PUT_SOURCE: u32 = 1165u32;
pub const MCIWNDM_REALIZE: u32 = 1142u32;
pub const MCIWNDM_RETURNSTRING: u32 = 1262u32;
pub const MCIWNDM_RETURNSTRINGA: u32 = 1162u32;
pub const MCIWNDM_RETURNSTRINGW: u32 = 1262u32;
pub const MCIWNDM_SENDSTRING: u32 = 1225u32;
pub const MCIWNDM_SENDSTRINGA: u32 = 1125u32;
pub const MCIWNDM_SENDSTRINGW: u32 = 1225u32;
pub const MCIWNDM_SETACTIVETIMER: u32 = 1154u32;
pub const MCIWNDM_SETINACTIVETIMER: u32 = 1155u32;
pub const MCIWNDM_SETOWNER: u32 = 1176u32;
pub const MCIWNDM_SETPALETTE: u32 = 1151u32;
pub const MCIWNDM_SETREPEAT: u32 = 1138u32;
pub const MCIWNDM_SETSPEED: u32 = 1136u32;
pub const MCIWNDM_SETTIMEFORMAT: u32 = 1243u32;
pub const MCIWNDM_SETTIMEFORMATA: u32 = 1143u32;
pub const MCIWNDM_SETTIMEFORMATW: u32 = 1243u32;
pub const MCIWNDM_SETTIMERS: u32 = 1153u32;
pub const MCIWNDM_SETVOLUME: u32 = 1134u32;
pub const MCIWNDM_SETZOOM: u32 = 1132u32;
pub const MCIWNDM_VALIDATEMEDIA: u32 = 1145u32;
pub const MCIWNDOPENF_NEW: u32 = 1u32;
pub const MCIWND_END: i32 = -2i32;
pub const MCIWND_START: i32 = -1i32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndCreateA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hwndparent: Param0,
    hinstance: Param1,
    dwstyle: u32,
    szfile: Param3,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndCreateA(
                hwndparent: super::super::Foundation::HWND,
                hinstance: super::super::Foundation::HINSTANCE,
                dwstyle: u32,
                szfile: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(MCIWndCreateA(
            hwndparent.into_param().abi(),
            hinstance.into_param().abi(),
            ::std::mem::transmute(dwstyle),
            szfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndCreateW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hwndparent: Param0,
    hinstance: Param1,
    dwstyle: u32,
    szfile: Param3,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndCreateW(
                hwndparent: super::super::Foundation::HWND,
                hinstance: super::super::Foundation::HINSTANCE,
                dwstyle: u32,
                szfile: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(MCIWndCreateW(
            hwndparent.into_param().abi(),
            hinstance.into_param().abi(),
            ::std::mem::transmute(dwstyle),
            szfile.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MCIWndRegisterClass() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MCIWndRegisterClass() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MCIWndRegisterClass())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MCI_AVI_SETVIDEO_DRAW_PROCEDURE: i32 = 32768i32;
pub const MCI_AVI_SETVIDEO_PALETTE_COLOR: i32 = 33024i32;
pub const MCI_AVI_SETVIDEO_PALETTE_HALFTONE: i32 = 65535i32;
pub const MCI_AVI_STATUS_AUDIO_BREAKS: i32 = 32771i32;
pub const MCI_AVI_STATUS_FRAMES_SKIPPED: i32 = 32769i32;
pub const MCI_AVI_STATUS_LAST_PLAY_SPEED: i32 = 32770i32;
pub const MCI_CAPTURE: u32 = 2160u32;
pub const MCI_CD_OFFSET: u32 = 1088u32;
pub const MCI_CLOSE_DRIVER: u32 = 2050u32;
pub const MCI_COLONIZED3_RETURN: u32 = 131072u32;
pub const MCI_COLONIZED4_RETURN: u32 = 262144u32;
pub const MCI_COMMAND_HEAD: u32 = 0u32;
pub const MCI_CONFIGURE: u32 = 2170u32;
pub const MCI_CONSTANT: u32 = 8u32;
pub const MCI_DGV_CAPTURE_AS: i32 = 65536i32;
pub const MCI_DGV_CAPTURE_AT: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CAPTURE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_CAPTURE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_CAPTURE_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_CAPTURE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_CAPTURE_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CAPTURE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_CAPTURE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_CAPTURE_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_CAPTURE_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_CAPTURE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_CAPTURE_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_COPY_AT: i32 = 65536i32;
pub const MCI_DGV_COPY_AUDIO_STREAM: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_COPY_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_COPY_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_COPY_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_COPY_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_COPY_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_COPY_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_COPY_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_CUE_INPUT: i32 = 65536i32;
pub const MCI_DGV_CUE_NOSHOW: i32 = 262144i32;
pub const MCI_DGV_CUE_OUTPUT: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MCI_DGV_CUE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
}
impl MCI_DGV_CUE_PARMS {}
impl ::std::default::Default for MCI_DGV_CUE_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCI_DGV_CUE_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCI_DGV_CUE_PARMS {}
unsafe impl ::windows::runtime::Abi for MCI_DGV_CUE_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_CUT_AT: i32 = 65536i32;
pub const MCI_DGV_CUT_AUDIO_STREAM: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_CUT_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_CUT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_CUT_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_CUT_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_CUT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_CUT_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_CUT_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_DELETE_AT: i32 = 65536i32;
pub const MCI_DGV_DELETE_AUDIO_STREAM: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_DELETE_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_DELETE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_DELETE_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_DELETE_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_DELETE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_DELETE_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_DELETE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_FF_AVI: i32 = 16385i32;
pub const MCI_DGV_FF_AVSS: i32 = 16384i32;
pub const MCI_DGV_FF_DIB: i32 = 16386i32;
pub const MCI_DGV_FF_JFIF: i32 = 16390i32;
pub const MCI_DGV_FF_JPEG: i32 = 16388i32;
pub const MCI_DGV_FF_MPEG: i32 = 16391i32;
pub const MCI_DGV_FF_RDIB: i32 = 16387i32;
pub const MCI_DGV_FF_RJPEG: i32 = 16389i32;
pub const MCI_DGV_FILE_MODE_EDITING: u32 = 3u32;
pub const MCI_DGV_FILE_MODE_EDITING_S: i32 = 32774i32;
pub const MCI_DGV_FILE_MODE_IDLE: u32 = 4u32;
pub const MCI_DGV_FILE_MODE_IDLE_S: i32 = 32775i32;
pub const MCI_DGV_FILE_MODE_LOADING: u32 = 2u32;
pub const MCI_DGV_FILE_MODE_LOADING_S: i32 = 32773i32;
pub const MCI_DGV_FILE_MODE_SAVING: u32 = 1u32;
pub const MCI_DGV_FILE_MODE_SAVING_S: i32 = 32772i32;
pub const MCI_DGV_FILE_S: i32 = 32770i32;
pub const MCI_DGV_FREEZE_AT: i32 = 65536i32;
pub const MCI_DGV_FREEZE_OUTSIDE: i32 = 131072i32;
pub const MCI_DGV_GETDEVCAPS_CAN_FREEZE: i32 = 16386i32;
pub const MCI_DGV_GETDEVCAPS_CAN_LOCK: i32 = 16384i32;
pub const MCI_DGV_GETDEVCAPS_CAN_REVERSE: i32 = 16388i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STRETCH: i32 = 16385i32;
pub const MCI_DGV_GETDEVCAPS_CAN_STR_IN: i32 = 16392i32;
pub const MCI_DGV_GETDEVCAPS_CAN_TEST: i32 = 16393i32;
pub const MCI_DGV_GETDEVCAPS_HAS_STILL: i32 = 16389i32;
pub const MCI_DGV_GETDEVCAPS_MAXIMUM_RATE: i32 = 16394i32;
pub const MCI_DGV_GETDEVCAPS_MAX_WINDOWS: i32 = 16387i32;
pub const MCI_DGV_GETDEVCAPS_MINIMUM_RATE: i32 = 16395i32;
pub const MCI_DGV_GETDEVCAPS_PALETTES: i32 = 16390i32;
pub const MCI_DGV_INFO_AUDIO_ALG: i32 = 16388i32;
pub const MCI_DGV_INFO_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_INFO_ITEM: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_INFO_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: super::super::Foundation::PSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_INFO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_INFO_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_INFO_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_INFO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_INFO_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_INFO_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: super::super::Foundation::PWSTR,
    pub dwRetSize: u32,
    pub dwItem: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_INFO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_INFO_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_INFO_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_INFO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_INFO_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_INFO_STILL_ALG: i32 = 16389i32;
pub const MCI_DGV_INFO_STILL_QUALITY: i32 = 16386i32;
pub const MCI_DGV_INFO_TEXT: i32 = 65536i32;
pub const MCI_DGV_INFO_USAGE: i32 = 16384i32;
pub const MCI_DGV_INFO_VIDEO_ALG: i32 = 16390i32;
pub const MCI_DGV_INFO_VIDEO_QUALITY: i32 = 16387i32;
pub const MCI_DGV_INPUT_S: i32 = 32771i32;
pub const MCI_DGV_LIST_ALG: i32 = 524288i32;
pub const MCI_DGV_LIST_AUDIO_ALG: i32 = 16384i32;
pub const MCI_DGV_LIST_AUDIO_QUALITY: i32 = 16385i32;
pub const MCI_DGV_LIST_AUDIO_STREAM: i32 = 16386i32;
pub const MCI_DGV_LIST_COUNT: i32 = 131072i32;
pub const MCI_DGV_LIST_ITEM: i32 = 65536i32;
pub const MCI_DGV_LIST_NUMBER: i32 = 262144i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_LIST_PARMSA {
    pub dwCallback: usize,
    pub lpstrReturn: super::super::Foundation::PSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_LIST_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_LIST_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_LIST_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_LIST_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_LIST_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_LIST_PARMSW {
    pub dwCallback: usize,
    pub lpstrReturn: super::super::Foundation::PWSTR,
    pub dwLength: u32,
    pub dwNumber: u32,
    pub dwItem: u32,
    pub lpstrAlgorithm: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_LIST_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_LIST_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_LIST_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_LIST_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_LIST_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_LIST_STILL_ALG: i32 = 16387i32;
pub const MCI_DGV_LIST_STILL_QUALITY: i32 = 16388i32;
pub const MCI_DGV_LIST_VIDEO_ALG: i32 = 16389i32;
pub const MCI_DGV_LIST_VIDEO_QUALITY: i32 = 16390i32;
pub const MCI_DGV_LIST_VIDEO_SOURCE: i32 = 16392i32;
pub const MCI_DGV_LIST_VIDEO_STREAM: i32 = 16391i32;
pub const MCI_DGV_METHOD_DIRECT: i32 = 40962i32;
pub const MCI_DGV_METHOD_POST: i32 = 40961i32;
pub const MCI_DGV_METHOD_PRE: i32 = 40960i32;
pub const MCI_DGV_MONITOR_FILE: i32 = 16385i32;
pub const MCI_DGV_MONITOR_INPUT: i32 = 16384i32;
pub const MCI_DGV_MONITOR_METHOD: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MCI_DGV_MONITOR_PARMS {
    pub dwCallback: usize,
    pub dwSource: u32,
    pub dwMethod: u32,
}
impl MCI_DGV_MONITOR_PARMS {}
impl ::std::default::Default for MCI_DGV_MONITOR_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCI_DGV_MONITOR_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCI_DGV_MONITOR_PARMS {}
unsafe impl ::windows::runtime::Abi for MCI_DGV_MONITOR_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_MONITOR_SOURCE: i32 = 131072i32;
pub const MCI_DGV_OPEN_16BIT: i32 = 524288i32;
pub const MCI_DGV_OPEN_32BIT: i32 = 1048576i32;
pub const MCI_DGV_OPEN_NOSTATIC: i32 = 262144i32;
pub const MCI_DGV_OPEN_PARENT: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_OPEN_PARMSA {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: super::super::Foundation::PSTR,
    pub lpstrElementName: super::super::Foundation::PSTR,
    pub lpstrAlias: super::super::Foundation::PSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_OPEN_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_OPEN_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_OPEN_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_OPEN_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_OPEN_PARMSW {
    pub dwCallback: usize,
    pub wDeviceID: u32,
    pub lpstrDeviceType: super::super::Foundation::PWSTR,
    pub lpstrElementName: super::super::Foundation::PWSTR,
    pub lpstrAlias: super::super::Foundation::PWSTR,
    pub dwStyle: u32,
    pub hWndParent: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_OPEN_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_OPEN_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_OPEN_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_OPEN_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_OPEN_WS: i32 = 65536i32;
pub const MCI_DGV_PASTE_AT: i32 = 65536i32;
pub const MCI_DGV_PASTE_AUDIO_STREAM: i32 = 131072i32;
pub const MCI_DGV_PASTE_INSERT: i32 = 524288i32;
pub const MCI_DGV_PASTE_OVERWRITE: i32 = 1048576i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_PASTE_PARMS {
    pub dwCallback: usize,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_PASTE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_PASTE_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_PASTE_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_PASTE_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_PASTE_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_PASTE_VIDEO_STREAM: i32 = 262144i32;
pub const MCI_DGV_PLAY_REPEAT: i32 = 65536i32;
pub const MCI_DGV_PLAY_REVERSE: i32 = 131072i32;
pub const MCI_DGV_PUT_CLIENT: i32 = 4194304i32;
pub const MCI_DGV_PUT_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_PUT_FRAME: i32 = 524288i32;
pub const MCI_DGV_PUT_SOURCE: i32 = 131072i32;
pub const MCI_DGV_PUT_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_PUT_WINDOW: i32 = 2097152i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_QUALITY_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: super::super::Foundation::PSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_QUALITY_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_QUALITY_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_QUALITY_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_QUALITY_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_QUALITY_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_QUALITY_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub lpstrName: super::super::Foundation::PWSTR,
    pub lpstrAlgorithm: u32,
    pub dwHandle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_QUALITY_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_QUALITY_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_QUALITY_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_QUALITY_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_QUALITY_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_REALIZE_BKGD: i32 = 131072i32;
pub const MCI_DGV_REALIZE_NORM: i32 = 65536i32;
pub const MCI_DGV_RECORD_AUDIO_STREAM: i32 = 262144i32;
pub const MCI_DGV_RECORD_HOLD: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RECORD_PARMS {
    pub dwCallback: usize,
    pub dwFrom: u32,
    pub dwTo: u32,
    pub rc: super::super::Foundation::RECT,
    pub dwAudioStream: u32,
    pub dwVideoStream: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RECORD_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RECORD_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RECORD_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RECORD_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RECORD_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_RECORD_VIDEO_STREAM: i32 = 524288i32;
pub const MCI_DGV_RECT: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RECT_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RECT_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RECT_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RECT_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RECT_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_RESERVE_IN: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESERVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrPath: super::super::Foundation::PSTR,
    pub dwSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RESERVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RESERVE_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RESERVE_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RESERVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RESERVE_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESERVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrPath: super::super::Foundation::PWSTR,
    pub dwSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RESERVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RESERVE_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RESERVE_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RESERVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RESERVE_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_RESERVE_SIZE: i32 = 131072i32;
pub const MCI_DGV_RESTORE_AT: i32 = 131072i32;
pub const MCI_DGV_RESTORE_FROM: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESTORE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RESTORE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RESTORE_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RESTORE_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RESTORE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RESTORE_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_RESTORE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_RESTORE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_RESTORE_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_RESTORE_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_RESTORE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_RESTORE_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SAVE_ABORT: i32 = 131072i32;
pub const MCI_DGV_SAVE_KEEPRESERVE: i32 = 262144i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SAVE_PARMSA {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SAVE_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SAVE_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SAVE_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SAVE_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SAVE_PARMSW {
    pub dwCallback: usize,
    pub lpstrFileName: super::super::Foundation::PWSTR,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SAVE_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SAVE_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SAVE_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SAVE_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SETAUDIO_ALG: i32 = 262144i32;
pub const MCI_DGV_SETAUDIO_AVGBYTESPERSEC: i32 = 16390i32;
pub const MCI_DGV_SETAUDIO_BASS: i32 = 16385i32;
pub const MCI_DGV_SETAUDIO_BITSPERSAMPLE: i32 = 16392i32;
pub const MCI_DGV_SETAUDIO_BLOCKALIGN: i32 = 16391i32;
pub const MCI_DGV_SETAUDIO_CLOCKTIME: i32 = 131072i32;
pub const MCI_DGV_SETAUDIO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETAUDIO_ITEM: i32 = 8388608i32;
pub const MCI_DGV_SETAUDIO_LEFT: i32 = 2097152i32;
pub const MCI_DGV_SETAUDIO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETAUDIO_OVER: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SETAUDIO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: super::super::Foundation::PSTR,
    pub lpstrQuality: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SETAUDIO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SETAUDIO_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SETAUDIO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SETAUDIO_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SETAUDIO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: super::super::Foundation::PWSTR,
    pub lpstrQuality: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SETAUDIO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SETAUDIO_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SETAUDIO_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SETAUDIO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SETAUDIO_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SETAUDIO_QUALITY: i32 = 524288i32;
pub const MCI_DGV_SETAUDIO_RECORD: i32 = 1048576i32;
pub const MCI_DGV_SETAUDIO_RIGHT: i32 = 4194304i32;
pub const MCI_DGV_SETAUDIO_SAMPLESPERSEC: i32 = 16389i32;
pub const MCI_DGV_SETAUDIO_SOURCE: i32 = 16388i32;
pub const MCI_DGV_SETAUDIO_SOURCE_AVERAGE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_SOURCE_LEFT: i32 = 1i32;
pub const MCI_DGV_SETAUDIO_SOURCE_RIGHT: i32 = 2i32;
pub const MCI_DGV_SETAUDIO_SOURCE_STEREO: i32 = 0i32;
pub const MCI_DGV_SETAUDIO_SRC_AVERAGE_S: i32 = 32802i32;
pub const MCI_DGV_SETAUDIO_SRC_LEFT_S: i32 = 32800i32;
pub const MCI_DGV_SETAUDIO_SRC_RIGHT_S: i32 = 32801i32;
pub const MCI_DGV_SETAUDIO_SRC_STEREO_S: i32 = 32803i32;
pub const MCI_DGV_SETAUDIO_STREAM: i32 = 16387i32;
pub const MCI_DGV_SETAUDIO_TREBLE: i32 = 16384i32;
pub const MCI_DGV_SETAUDIO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SETAUDIO_VOLUME: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_ALG: i32 = 131072i32;
pub const MCI_DGV_SETVIDEO_BITSPERPEL: i32 = 16396i32;
pub const MCI_DGV_SETVIDEO_BRIGHTNESS: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_CLOCKTIME: i32 = 262144i32;
pub const MCI_DGV_SETVIDEO_COLOR: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_CONTRAST: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_FRAME_RATE: i32 = 16392i32;
pub const MCI_DGV_SETVIDEO_GAMMA: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_INPUT: i32 = 33554432i32;
pub const MCI_DGV_SETVIDEO_ITEM: i32 = 1048576i32;
pub const MCI_DGV_SETVIDEO_KEY_COLOR: i32 = 16395i32;
pub const MCI_DGV_SETVIDEO_KEY_INDEX: i32 = 16394i32;
pub const MCI_DGV_SETVIDEO_OUTPUT: i32 = 67108864i32;
pub const MCI_DGV_SETVIDEO_OVER: i32 = 2097152i32;
pub const MCI_DGV_SETVIDEO_PALHANDLE: i32 = 16391i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SETVIDEO_PARMSA {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: super::super::Foundation::PSTR,
    pub lpstrQuality: super::super::Foundation::PSTR,
    pub dwSourceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SETVIDEO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SETVIDEO_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SETVIDEO_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SETVIDEO_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_SETVIDEO_PARMSW {
    pub dwCallback: usize,
    pub dwItem: u32,
    pub dwValue: u32,
    pub dwOver: u32,
    pub lpstrAlgorithm: super::super::Foundation::PWSTR,
    pub lpstrQuality: super::super::Foundation::PWSTR,
    pub dwSourceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_SETVIDEO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_SETVIDEO_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_SETVIDEO_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_SETVIDEO_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_SETVIDEO_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SETVIDEO_QUALITY: i32 = 65536i32;
pub const MCI_DGV_SETVIDEO_RECORD: i32 = 4194304i32;
pub const MCI_DGV_SETVIDEO_SHARPNESS: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC: i32 = 16389i32;
pub const MCI_DGV_SETVIDEO_SRC_GENERIC_S: i32 = 32789i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC: i32 = 16384i32;
pub const MCI_DGV_SETVIDEO_SRC_NTSC_S: i32 = 32784i32;
pub const MCI_DGV_SETVIDEO_SRC_NUMBER: i32 = 524288i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_SRC_PAL_S: i32 = 32787i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB: i32 = 16385i32;
pub const MCI_DGV_SETVIDEO_SRC_RGB_S: i32 = 32785i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM: i32 = 16388i32;
pub const MCI_DGV_SETVIDEO_SRC_SECAM_S: i32 = 32788i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO: i32 = 16386i32;
pub const MCI_DGV_SETVIDEO_SRC_SVIDEO_S: i32 = 32786i32;
pub const MCI_DGV_SETVIDEO_STILL: i32 = 8388608i32;
pub const MCI_DGV_SETVIDEO_STREAM: i32 = 16390i32;
pub const MCI_DGV_SETVIDEO_TINT: i32 = 16387i32;
pub const MCI_DGV_SETVIDEO_VALUE: i32 = 16777216i32;
pub const MCI_DGV_SET_FILEFORMAT: i32 = 524288i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MCI_DGV_SET_PARMS {
    pub dwCallback: usize,
    pub dwTimeFormat: u32,
    pub dwAudio: u32,
    pub dwFileFormat: u32,
    pub dwSpeed: u32,
}
impl MCI_DGV_SET_PARMS {}
impl ::std::default::Default for MCI_DGV_SET_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCI_DGV_SET_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCI_DGV_SET_PARMS {}
unsafe impl ::windows::runtime::Abi for MCI_DGV_SET_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SET_SEEK_EXACTLY: i32 = 65536i32;
pub const MCI_DGV_SET_SPEED: i32 = 131072i32;
pub const MCI_DGV_SET_STILL: i32 = 262144i32;
pub const MCI_DGV_SIGNAL_AT: i32 = 65536i32;
pub const MCI_DGV_SIGNAL_CANCEL: i32 = 524288i32;
pub const MCI_DGV_SIGNAL_EVERY: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MCI_DGV_SIGNAL_PARMS {
    pub dwCallback: usize,
    pub dwPosition: u32,
    pub dwPeriod: u32,
    pub dwUserParm: u32,
}
impl MCI_DGV_SIGNAL_PARMS {}
impl ::std::default::Default for MCI_DGV_SIGNAL_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCI_DGV_SIGNAL_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCI_DGV_SIGNAL_PARMS {}
unsafe impl ::windows::runtime::Abi for MCI_DGV_SIGNAL_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_SIGNAL_POSITION: i32 = 1048576i32;
pub const MCI_DGV_SIGNAL_USERVAL: i32 = 262144i32;
pub const MCI_DGV_STATUS_AUDIO: i32 = 16404i32;
pub const MCI_DGV_STATUS_AUDIO_INPUT: i32 = 16384i32;
pub const MCI_DGV_STATUS_AUDIO_RECORD: i32 = 16410i32;
pub const MCI_DGV_STATUS_AUDIO_SOURCE: i32 = 16393i32;
pub const MCI_DGV_STATUS_AUDIO_STREAM: i32 = 16429i32;
pub const MCI_DGV_STATUS_AVGBYTESPERSEC: i32 = 16424i32;
pub const MCI_DGV_STATUS_BASS: i32 = 16399i32;
pub const MCI_DGV_STATUS_BITSPERPEL: i32 = 16427i32;
pub const MCI_DGV_STATUS_BITSPERSAMPLE: i32 = 16426i32;
pub const MCI_DGV_STATUS_BLOCKALIGN: i32 = 16425i32;
pub const MCI_DGV_STATUS_BRIGHTNESS: i32 = 16389i32;
pub const MCI_DGV_STATUS_COLOR: i32 = 16390i32;
pub const MCI_DGV_STATUS_CONTRAST: i32 = 16391i32;
pub const MCI_DGV_STATUS_DISKSPACE: i32 = 2097152i32;
pub const MCI_DGV_STATUS_FILEFORMAT: i32 = 16392i32;
pub const MCI_DGV_STATUS_FILE_COMPLETION: i32 = 16416i32;
pub const MCI_DGV_STATUS_FILE_MODE: i32 = 16415i32;
pub const MCI_DGV_STATUS_FORWARD: i32 = 16428i32;
pub const MCI_DGV_STATUS_FRAME_RATE: i32 = 16398i32;
pub const MCI_DGV_STATUS_GAMMA: i32 = 16394i32;
pub const MCI_DGV_STATUS_HPAL: i32 = 16388i32;
pub const MCI_DGV_STATUS_HWND: i32 = 16385i32;
pub const MCI_DGV_STATUS_INPUT: i32 = 4194304i32;
pub const MCI_DGV_STATUS_KEY_COLOR: i32 = 16421i32;
pub const MCI_DGV_STATUS_KEY_INDEX: i32 = 16420i32;
pub const MCI_DGV_STATUS_LEFT: i32 = 524288i32;
pub const MCI_DGV_STATUS_MONITOR: i32 = 16395i32;
pub const MCI_DGV_STATUS_MONITOR_METHOD: i32 = 16396i32;
pub const MCI_DGV_STATUS_NOMINAL: i32 = 131072i32;
pub const MCI_DGV_STATUS_OUTPUT: i32 = 8388608i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_STATUS_PARMSA {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: super::super::Foundation::PSTR,
    pub dwReference: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_STATUS_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_STATUS_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_STATUS_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_STATUS_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_STATUS_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_STATUS_PARMSW {
    pub dwCallback: usize,
    pub dwReturn: usize,
    pub dwItem: u32,
    pub dwTrack: u32,
    pub lpstrDrive: super::super::Foundation::PWSTR,
    pub dwReference: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_STATUS_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_STATUS_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_STATUS_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_STATUS_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_STATUS_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_STATUS_PAUSE_MODE: i32 = 16422i32;
pub const MCI_DGV_STATUS_RECORD: i32 = 16777216i32;
pub const MCI_DGV_STATUS_REFERENCE: i32 = 262144i32;
pub const MCI_DGV_STATUS_RIGHT: i32 = 1048576i32;
pub const MCI_DGV_STATUS_SAMPLESPERSEC: i32 = 16423i32;
pub const MCI_DGV_STATUS_SEEK_EXACTLY: i32 = 16401i32;
pub const MCI_DGV_STATUS_SHARPNESS: i32 = 16402i32;
pub const MCI_DGV_STATUS_SIZE: i32 = 16400i32;
pub const MCI_DGV_STATUS_SMPTE: i32 = 16403i32;
pub const MCI_DGV_STATUS_SPEED: i32 = 16387i32;
pub const MCI_DGV_STATUS_STILL_FILEFORMAT: i32 = 16413i32;
pub const MCI_DGV_STATUS_TINT: i32 = 16405i32;
pub const MCI_DGV_STATUS_TREBLE: i32 = 16406i32;
pub const MCI_DGV_STATUS_UNSAVED: i32 = 16407i32;
pub const MCI_DGV_STATUS_VIDEO: i32 = 16408i32;
pub const MCI_DGV_STATUS_VIDEO_RECORD: i32 = 16412i32;
pub const MCI_DGV_STATUS_VIDEO_SOURCE: i32 = 16411i32;
pub const MCI_DGV_STATUS_VIDEO_SRC_NUM: i32 = 16414i32;
pub const MCI_DGV_STATUS_VIDEO_STREAM: i32 = 16430i32;
pub const MCI_DGV_STATUS_VOLUME: i32 = 16409i32;
pub const MCI_DGV_STATUS_WINDOW_MAXIMIZED: i32 = 16419i32;
pub const MCI_DGV_STATUS_WINDOW_MINIMIZED: i32 = 16418i32;
pub const MCI_DGV_STATUS_WINDOW_VISIBLE: i32 = 16417i32;
pub const MCI_DGV_STEP_FRAMES: i32 = 131072i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MCI_DGV_STEP_PARMS {
    pub dwCallback: usize,
    pub dwFrames: u32,
}
impl MCI_DGV_STEP_PARMS {}
impl ::std::default::Default for MCI_DGV_STEP_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCI_DGV_STEP_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCI_DGV_STEP_PARMS {}
unsafe impl ::windows::runtime::Abi for MCI_DGV_STEP_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_STEP_REVERSE: i32 = 65536i32;
pub const MCI_DGV_STOP_HOLD: i32 = 65536i32;
pub const MCI_DGV_UPDATE_HDC: i32 = 131072i32;
pub const MCI_DGV_UPDATE_PAINT: i32 = 262144i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct MCI_DGV_UPDATE_PARMS {
    pub dwCallback: usize,
    pub rc: super::super::Foundation::RECT,
    pub hDC: super::super::Graphics::Gdi::HDC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl MCI_DGV_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for MCI_DGV_UPDATE_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for MCI_DGV_UPDATE_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for MCI_DGV_UPDATE_PARMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for MCI_DGV_UPDATE_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_WHERE_DESTINATION: i32 = 262144i32;
pub const MCI_DGV_WHERE_FRAME: i32 = 524288i32;
pub const MCI_DGV_WHERE_MAX: i32 = 4194304i32;
pub const MCI_DGV_WHERE_SOURCE: i32 = 131072i32;
pub const MCI_DGV_WHERE_VIDEO: i32 = 1048576i32;
pub const MCI_DGV_WHERE_WINDOW: i32 = 2097152i32;
pub const MCI_DGV_WINDOW_DEFAULT: i32 = 0i32;
pub const MCI_DGV_WINDOW_HWND: i32 = 65536i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_WINDOW_PARMSA {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_WINDOW_PARMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_WINDOW_PARMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_WINDOW_PARMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_WINDOW_PARMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_DGV_WINDOW_PARMSW {
    pub dwCallback: usize,
    pub hWnd: super::super::Foundation::HWND,
    pub nCmdShow: u32,
    pub lpstrText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_DGV_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_DGV_WINDOW_PARMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_DGV_WINDOW_PARMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_DGV_WINDOW_PARMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_DGV_WINDOW_PARMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_DGV_WINDOW_STATE: i32 = 262144i32;
pub const MCI_DGV_WINDOW_TEXT: i32 = 524288i32;
pub const MCI_END_COMMAND: u32 = 3u32;
pub const MCI_END_COMMAND_LIST: u32 = 6u32;
pub const MCI_END_CONSTANT: u32 = 9u32;
pub const MCI_FALSE: u32 = 531u32;
pub const MCI_FLAG: u32 = 5u32;
pub const MCI_FORMAT_BYTES_S: u32 = 541u32;
pub const MCI_FORMAT_FRAMES_S: u32 = 536u32;
pub const MCI_FORMAT_HMS_S: u32 = 534u32;
pub const MCI_FORMAT_MILLISECONDS_S: u32 = 533u32;
pub const MCI_FORMAT_MSF_S: u32 = 535u32;
pub const MCI_FORMAT_SAMPLES_S: u32 = 542u32;
pub const MCI_FORMAT_SMPTE_24_S: u32 = 537u32;
pub const MCI_FORMAT_SMPTE_25_S: u32 = 538u32;
pub const MCI_FORMAT_SMPTE_30DROP_S: u32 = 540u32;
pub const MCI_FORMAT_SMPTE_30_S: u32 = 539u32;
pub const MCI_FORMAT_TMSF_S: u32 = 543u32;
pub const MCI_HDC: u32 = 12u32;
pub const MCI_HPAL: u32 = 11u32;
pub const MCI_HWND: u32 = 10u32;
pub const MCI_INFO_VERSION: i32 = 1024i32;
pub const MCI_INTEGER: u32 = 2u32;
pub const MCI_INTEGER64: u32 = 13u32;
pub const MCI_INTEGER_RETURNED: u32 = 524288u32;
pub const MCI_LIST: u32 = 2168u32;
pub const MCI_MAX_DEVICE_TYPE_LENGTH: u32 = 80u32;
pub const MCI_MCIAVI_PLAY_FULLBY2: i32 = 67108864i32;
pub const MCI_MCIAVI_PLAY_FULLSCREEN: i32 = 33554432i32;
pub const MCI_MCIAVI_PLAY_WINDOW: i32 = 16777216i32;
pub const MCI_MONITOR: u32 = 2161u32;
pub const MCI_OFF: u32 = 0u32;
pub const MCI_OFF_S: i32 = 32769i32;
pub const MCI_ON: u32 = 1u32;
pub const MCI_ON_S: i32 = 32768i32;
pub const MCI_OPEN_DRIVER: u32 = 2049u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MCI_OPEN_DRIVER_PARMS {
    pub wDeviceID: u32,
    pub lpstrParams: super::super::Foundation::PWSTR,
    pub wCustomCommandTable: u32,
    pub wType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCI_OPEN_DRIVER_PARMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCI_OPEN_DRIVER_PARMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCI_OPEN_DRIVER_PARMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCI_OPEN_DRIVER_PARMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCI_OPEN_DRIVER_PARMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCI_QUALITY: u32 = 2167u32;
pub const MCI_QUALITY_ALG: i32 = 262144i32;
pub const MCI_QUALITY_DIALOG: i32 = 524288i32;
pub const MCI_QUALITY_HANDLE: i32 = 1048576i32;
pub const MCI_QUALITY_ITEM: i32 = 65536i32;
pub const MCI_QUALITY_ITEM_AUDIO: i32 = 16384i32;
pub const MCI_QUALITY_ITEM_STILL: i32 = 16385i32;
pub const MCI_QUALITY_ITEM_VIDEO: i32 = 16386i32;
pub const MCI_QUALITY_NAME: i32 = 131072i32;
pub const MCI_RECT: u32 = 7u32;
pub const MCI_RESERVE: u32 = 2162u32;
pub const MCI_RESOURCE_DRIVER: u32 = 1048576u32;
pub const MCI_RESOURCE_RETURNED: u32 = 65536u32;
pub const MCI_RESTORE: u32 = 2171u32;
pub const MCI_RETURN: u32 = 4u32;
pub const MCI_SEQ_FILE_S: u32 = 1222u32;
pub const MCI_SEQ_FORMAT_SONGPTR_S: u32 = 1225u32;
pub const MCI_SEQ_MAPPER_S: u32 = 1221u32;
pub const MCI_SEQ_MIDI_S: u32 = 1223u32;
pub const MCI_SEQ_NONE_S: u32 = 1226u32;
pub const MCI_SEQ_OFFSET: u32 = 1216u32;
pub const MCI_SEQ_SMPTE_S: u32 = 1224u32;
pub const MCI_SETAUDIO: u32 = 2163u32;
pub const MCI_SETVIDEO: u32 = 2166u32;
pub const MCI_SIGNAL: u32 = 2165u32;
pub const MCI_STRING: u32 = 1u32;
pub const MCI_STRING_OFFSET: u32 = 512u32;
pub const MCI_TEST: i32 = 32i32;
pub const MCI_TRUE: u32 = 532u32;
pub const MCI_UNDO: u32 = 2169u32;
pub const MCI_VD_FORMAT_TRACK_S: u32 = 1029u32;
pub const MCI_VD_OFFSET: u32 = 1024u32;
pub const MCI_WAVE_OFFSET: u32 = 1152u32;
pub const MCMADM_E_REGKEY_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889750i32 as _);
pub const MCMADM_I_NO_EVENTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593897i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MEDIASPACEADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl MEDIASPACEADPCMWAVEFORMAT {}
impl ::std::default::Default for MEDIASPACEADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MEDIASPACEADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MEDIASPACEADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for MEDIASPACEADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
pub const MEVT_F_LONG: i32 = -2147483648i32;
pub const MEVT_F_SHORT: i32 = 0i32;
pub const MHDR_DONE: u32 = 1u32;
pub const MHDR_INQUEUE: u32 = 4u32;
pub const MHDR_ISSTRM: u32 = 8u32;
pub const MHDR_PREPARED: u32 = 2u32;
pub const MIDICAPS_CACHE: u32 = 4u32;
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
pub const MIDICAPS_STREAM: u32 = 8u32;
pub const MIDICAPS_VOLUME: u32 = 1u32;
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
pub const MIDIERR_BASE: u32 = 64u32;
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
pub const MIDIERR_LASTERROR: u32 = 71u32;
pub const MIDIERR_NODEVICE: u32 = 68u32;
pub const MIDIERR_NOMAP: u32 = 66u32;
pub const MIDIERR_NOTREADY: u32 = 67u32;
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl MIDIEVENT {}
impl ::std::default::Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIEVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIEVENT {}
unsafe impl ::windows::runtime::Abi for MIDIEVENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIHDR {
    pub lpData: super::super::Foundation::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut MIDIHDR,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIDIHDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIDIHDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIDIHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIDIHDR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIDIINCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIDIINCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl MIDIINCAPS2W {}
impl ::std::default::Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIINCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIINCAPS2W {}
unsafe impl ::windows::runtime::Abi for MIDIINCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIDIINCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIDIINCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl MIDIINCAPSW {}
impl ::std::default::Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIINCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIINCAPSW {}
unsafe impl ::windows::runtime::Abi for MIDIINCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIDIMAPPER_S: u32 = 1227u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIOPENSTRMID {
    pub dwStreamID: u32,
    pub uDeviceID: u32,
}
impl MIDIOPENSTRMID {}
impl ::std::default::Default for MIDIOPENSTRMID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIOPENSTRMID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIOPENSTRMID {}
unsafe impl ::windows::runtime::Abi for MIDIOPENSTRMID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIDIOUTCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl MIDIOUTCAPS2W {}
impl ::std::default::Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIOUTCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIOUTCAPS2W {}
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIDIOUTCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wVoices: u16,
    pub wNotes: u16,
    pub wChannelMask: u16,
    pub dwSupport: u32,
}
impl MIDIOUTCAPSW {}
impl ::std::default::Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIOUTCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIOUTCAPSW {}
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIDIPATCHSIZE: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl MIDIPROPTEMPO {}
impl ::std::default::Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIPROPTEMPO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIPROPTEMPO {}
unsafe impl ::windows::runtime::Abi for MIDIPROPTEMPO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl MIDIPROPTIMEDIV {}
impl ::std::default::Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDIPROPTIMEDIV {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDIPROPTIMEDIV {}
unsafe impl ::windows::runtime::Abi for MIDIPROPTIMEDIV {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl MIDISTRMBUFFVER {}
impl ::std::default::Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIDISTRMBUFFVER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIDISTRMBUFFVER {}
unsafe impl ::windows::runtime::Abi for MIDISTRMBUFFVER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_IO_COOKED: i32 = 2i32;
pub const MIDI_IO_PACKED: i32 = 0i32;
pub const MIDI_UNCACHE: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE =
    MIDI_WAVE_OPEN_TYPE(16u32);
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
impl ::std::convert::From<u32> for MIDI_WAVE_OPEN_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIDI_WAVE_OPEN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MIDM_ADDBUFFER: u32 = 59u32;
pub const MIDM_CLOSE: u32 = 56u32;
pub const MIDM_GETDEVCAPS: u32 = 54u32;
pub const MIDM_GETNUMDEVS: u32 = 53u32;
pub const MIDM_INIT: u32 = 100u32;
pub const MIDM_INIT_EX: u32 = 104u32;
pub const MIDM_MAPPER: u32 = 8192u32;
pub const MIDM_OPEN: u32 = 55u32;
pub const MIDM_PREPARE: u32 = 57u32;
pub const MIDM_RESET: u32 = 62u32;
pub const MIDM_START: u32 = 60u32;
pub const MIDM_STOP: u32 = 61u32;
pub const MIDM_UNPREPARE: u32 = 58u32;
pub const MIDM_USER: u32 = 16384u32;
pub const MIM_CLOSE: u32 = 962u32;
pub const MIM_DATA: u32 = 963u32;
pub const MIM_ERROR: u32 = 965u32;
pub const MIM_LONGDATA: u32 = 964u32;
pub const MIM_LONGERROR: u32 = 966u32;
pub const MIM_MOREDATA: u32 = 972u32;
pub const MIM_OPEN: u32 = 961u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl MIXERCAPS2W {}
impl ::std::default::Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCAPS2W {}
unsafe impl ::windows::runtime::Abi for MIXERCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl MIXERCAPSW {}
impl ::std::default::Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCAPSW {}
unsafe impl ::windows::runtime::Abi for MIXERCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Bounds: MIXERCONTROLA_0,
    pub Metrics: MIXERCONTROLA_1,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCONTROLA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLA_0 {}
impl ::std::default::Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLA_0 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl MIXERCONTROLA_0_0 {}
impl ::std::default::Default for MIXERCONTROLA_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLA_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLA_0_0 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl MIXERCONTROLA_0_1 {}
impl ::std::default::Default for MIXERCONTROLA_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLA_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLA_0_1 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLA_1 {}
impl ::std::default::Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLA_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLA_1 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl MIXERCONTROLDETAILS_BOOLEAN {}
impl ::std::default::Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_BOOLEAN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_BOOLEAN {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_BOOLEAN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_LISTTEXTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::std::default::Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTW {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_LISTTEXTW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl MIXERCONTROLDETAILS_SIGNED {}
impl ::std::default::Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_SIGNED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_SIGNED {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_SIGNED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl MIXERCONTROLDETAILS_UNSIGNED {}
impl ::std::default::Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLDETAILS_UNSIGNED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLDETAILS_UNSIGNED {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_UNSIGNED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLW {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub dwControlType: u32,
    pub fdwControl: u32,
    pub cMultipleItems: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Bounds: MIXERCONTROLW_0,
    pub Metrics: MIXERCONTROLW_1,
}
impl MIXERCONTROLW {}
impl ::std::default::Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLW {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLW_0 {}
impl ::std::default::Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLW_0 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl MIXERCONTROLW_0_0 {}
impl ::std::default::Default for MIXERCONTROLW_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLW_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLW_0_0 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl MIXERCONTROLW_0_1 {}
impl ::std::default::Default for MIXERCONTROLW_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLW_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLW_0_1 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLW_1 {}
impl ::std::default::Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERCONTROLW_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERCONTROLW_1 {}
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_MTS: u32 = 536936454u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_ONOFF: u32 = 536936455u32;
pub const MIXERCONTROL_CONTROLTYPE_SRS_SYNTHSELECT: u32 = 536936456u32;
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINEA {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [super::super::Foundation::CHAR; 16],
    pub szName: [super::super::Foundation::CHAR; 64],
    pub Target: MIXERLINEA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERLINEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERLINEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINEA_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERLINEA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERLINEA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERLINECONTROLSA {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSA_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLA,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIXERLINECONTROLSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl MIXERLINECONTROLSA_0 {}
impl ::std::default::Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERLINECONTROLSA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERLINECONTROLSA_0 {}
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLW,
}
impl MIXERLINECONTROLSW {}
impl ::std::default::Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERLINECONTROLSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERLINECONTROLSW {}
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl MIXERLINECONTROLSW_0 {}
impl ::std::default::Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERLINECONTROLSW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERLINECONTROLSW_0 {}
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSW_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERLINEW {
    pub cbStruct: u32,
    pub dwDestination: u32,
    pub dwSource: u32,
    pub dwLineID: u32,
    pub fdwLine: u32,
    pub dwUser: usize,
    pub dwComponentType: MIXERLINE_COMPONENTTYPE,
    pub cChannels: u32,
    pub cConnections: u32,
    pub cControls: u32,
    pub szShortName: [u16; 16],
    pub szName: [u16; 64],
    pub Target: MIXERLINEW_0,
}
impl MIXERLINEW {}
impl ::std::default::Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERLINEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERLINEW {}
unsafe impl ::windows::runtime::Abi for MIXERLINEW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl MIXERLINEW_0 {}
impl ::std::default::Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXERLINEW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXERLINEW_0 {}
unsafe impl ::windows::runtime::Abi for MIXERLINEW_0 {
    type Abi = Self;
    type DefaultType = Self;
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
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(1u32);
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(5u32);
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(3u32);
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4u32);
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(6u32);
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(0u32);
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(8u32);
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(7u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4106u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4105u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4101u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4097u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4098u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4099u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4103u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4100u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4102u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4096u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE =
    MIXERLINE_COMPONENTTYPE(4104u32);
impl ::std::convert::From<u32> for MIXERLINE_COMPONENTTYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIXERLINE_COMPONENTTYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MIXERLINE_COMPONENTTYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MIXERLINE_COMPONENTTYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MIXERLINE_COMPONENTTYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MIXERLINE_COMPONENTTYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MIXERLINE_COMPONENTTYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXEROPENDESC {
    pub hmx: HMIXER,
    pub pReserved0: *mut ::std::ffi::c_void,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub dnDevNode: usize,
}
impl MIXEROPENDESC {}
impl ::std::default::Default for MIXEROPENDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MIXEROPENDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MIXEROPENDESC {}
unsafe impl ::windows::runtime::Abi for MIXEROPENDESC {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MIXERR_BASE: u32 = 1024u32;
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
pub const MIXERR_INVALLINE: u32 = 1024u32;
pub const MIXERR_INVALVALUE: u32 = 1026u32;
pub const MIXERR_LASTERROR: u32 = 1026u32;
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MMCKINFO {
    pub ckid: u32,
    pub cksize: u32,
    pub fccType: u32,
    pub dwDataOffset: u32,
    pub dwFlags: u32,
}
impl MMCKINFO {}
impl ::std::default::Default for MMCKINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MMCKINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MMCKINFO {}
unsafe impl ::windows::runtime::Abi for MMCKINFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MMIOERR_ACCESSDENIED: u32 = 268u32;
pub const MMIOERR_BASE: u32 = 256u32;
pub const MMIOERR_CANNOTCLOSE: u32 = 260u32;
pub const MMIOERR_CANNOTEXPAND: u32 = 264u32;
pub const MMIOERR_CANNOTOPEN: u32 = 259u32;
pub const MMIOERR_CANNOTREAD: u32 = 261u32;
pub const MMIOERR_CANNOTSEEK: u32 = 263u32;
pub const MMIOERR_CANNOTWRITE: u32 = 262u32;
pub const MMIOERR_CHUNKNOTFOUND: u32 = 265u32;
pub const MMIOERR_FILENOTFOUND: u32 = 257u32;
pub const MMIOERR_INVALIDFILE: u32 = 272u32;
pub const MMIOERR_NETWORKERROR: u32 = 270u32;
pub const MMIOERR_OUTOFMEMORY: u32 = 258u32;
pub const MMIOERR_PATHNOTFOUND: u32 = 267u32;
pub const MMIOERR_SHARINGVIOLATION: u32 = 269u32;
pub const MMIOERR_TOOMANYOPENFILES: u32 = 271u32;
pub const MMIOERR_UNBUFFERED: u32 = 266u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
impl ::std::clone::Clone for MMIOINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
pub struct MMIOINFO {
    pub dwFlags: u32,
    pub fccIOProc: u32,
    pub pIOProc: ::std::option::Option<LPMMIOPROC>,
    pub wErrorRet: u32,
    pub htask: super::Audio::CoreAudio::HTASK,
    pub cchBuffer: i32,
    pub pchBuffer: *mut i8,
    pub pchNext: *mut i8,
    pub pchEndRead: *mut i8,
    pub pchEndWrite: *mut i8,
    pub lBufOffset: i32,
    pub lDiskOffset: i32,
    pub adwInfo: [u32; 3],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub hmmio: HMMIO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
impl MMIOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
impl ::std::default::Default for MMIOINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
impl ::std::cmp::PartialEq for MMIOINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
impl ::std::cmp::Eq for MMIOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
unsafe impl ::windows::runtime::Abi for MMIOINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const MMIOM_CLOSE: u32 = 4u32;
pub const MMIOM_OPEN: u32 = 3u32;
pub const MMIOM_READ: u32 = 0u32;
pub const MMIOM_RENAME: u32 = 6u32;
pub const MMIOM_SEEK: u32 = 2u32;
pub const MMIOM_USER: u32 = 32768u32;
pub const MMIOM_WRITE: u32 = 1u32;
pub const MMIOM_WRITEFLUSH: u32 = 5u32;
pub const MMIO_ALLOCBUF: u32 = 65536u32;
pub const MMIO_COMPAT: u32 = 0u32;
pub const MMIO_CREATE: u32 = 4096u32;
pub const MMIO_CREATELIST: u32 = 64u32;
pub const MMIO_CREATERIFF: u32 = 32u32;
pub const MMIO_DEFAULTBUFFER: u32 = 8192u32;
pub const MMIO_DELETE: u32 = 512u32;
pub const MMIO_DENYNONE: u32 = 64u32;
pub const MMIO_DENYREAD: u32 = 48u32;
pub const MMIO_DENYWRITE: u32 = 32u32;
pub const MMIO_DIRTY: u32 = 268435456u32;
pub const MMIO_EMPTYBUF: u32 = 16u32;
pub const MMIO_EXCLUSIVE: u32 = 16u32;
pub const MMIO_EXIST: u32 = 16384u32;
pub const MMIO_FHOPEN: u32 = 16u32;
pub const MMIO_FINDCHUNK: u32 = 16u32;
pub const MMIO_FINDLIST: u32 = 64u32;
pub const MMIO_FINDPROC: u32 = 262144u32;
pub const MMIO_FINDRIFF: u32 = 32u32;
pub const MMIO_GETTEMP: u32 = 131072u32;
pub const MMIO_GLOBALPROC: u32 = 268435456u32;
pub const MMIO_INSTALLPROC: u32 = 65536u32;
pub const MMIO_PARSE: u32 = 256u32;
pub const MMIO_READ: u32 = 0u32;
pub const MMIO_READWRITE: u32 = 2u32;
pub const MMIO_REMOVEPROC: u32 = 131072u32;
pub const MMIO_RWMODE: u32 = 3u32;
pub const MMIO_SHAREMODE: u32 = 112u32;
pub const MMIO_TOUPPER: u32 = 16u32;
pub const MMIO_UNICODEPROC: u32 = 16777216u32;
pub const MMIO_WRITE: u32 = 1u32;
pub const MMSYSERR_ALLOCATED: u32 = 4u32;
pub const MMSYSERR_BADDB: u32 = 14u32;
pub const MMSYSERR_BADDEVICEID: u32 = 2u32;
pub const MMSYSERR_BADERRNUM: u32 = 9u32;
pub const MMSYSERR_BASE: u32 = 0u32;
pub const MMSYSERR_DELETEERROR: u32 = 18u32;
pub const MMSYSERR_ERROR: u32 = 1u32;
pub const MMSYSERR_HANDLEBUSY: u32 = 12u32;
pub const MMSYSERR_INVALFLAG: u32 = 10u32;
pub const MMSYSERR_INVALHANDLE: u32 = 5u32;
pub const MMSYSERR_INVALIDALIAS: u32 = 13u32;
pub const MMSYSERR_INVALPARAM: u32 = 11u32;
pub const MMSYSERR_KEYNOTFOUND: u32 = 15u32;
pub const MMSYSERR_LASTERROR: u32 = 21u32;
pub const MMSYSERR_MOREDATA: u32 = 21u32;
pub const MMSYSERR_NODRIVER: u32 = 6u32;
pub const MMSYSERR_NODRIVERCB: u32 = 20u32;
pub const MMSYSERR_NOERROR: u32 = 0u32;
pub const MMSYSERR_NOMEM: u32 = 7u32;
pub const MMSYSERR_NOTENABLED: u32 = 3u32;
pub const MMSYSERR_NOTSUPPORTED: u32 = 8u32;
pub const MMSYSERR_READERROR: u32 = 16u32;
pub const MMSYSERR_VALNOTFOUND: u32 = 19u32;
pub const MMSYSERR_WRITEERROR: u32 = 17u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MMTIME {
    pub wType: u32,
    pub u: MMTIME_0,
}
impl MMTIME {}
impl ::std::default::Default for MMTIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MMTIME {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MMTIME {}
unsafe impl ::windows::runtime::Abi for MMTIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MMTIME_0 {
    pub ms: u32,
    pub sample: u32,
    pub cb: u32,
    pub ticks: u32,
    pub smpte: MMTIME_0_1,
    pub midi: MMTIME_0_0,
}
impl MMTIME_0 {}
impl ::std::default::Default for MMTIME_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MMTIME_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MMTIME_0 {}
unsafe impl ::windows::runtime::Abi for MMTIME_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MMTIME_0_0 {
    pub songptrpos: u32,
}
impl MMTIME_0_0 {}
impl ::std::default::Default for MMTIME_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MMTIME_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MMTIME_0_0 {}
unsafe impl ::windows::runtime::Abi for MMTIME_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MMTIME_0_1 {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub frame: u8,
    pub fps: u8,
    pub dummy: u8,
    pub pad: [u8; 2],
}
impl MMTIME_0_1 {}
impl ::std::default::Default for MMTIME_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MMTIME_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_smpte_e__Struct")
            .field("hour", &self.hour)
            .field("min", &self.min)
            .field("sec", &self.sec)
            .field("frame", &self.frame)
            .field("fps", &self.fps)
            .field("dummy", &self.dummy)
            .field("pad", &self.pad)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MMTIME_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour
            && self.min == other.min
            && self.sec == other.sec
            && self.frame == other.frame
            && self.fps == other.fps
            && self.dummy == other.dummy
            && self.pad == other.pad
    }
}
impl ::std::cmp::Eq for MMTIME_0_1 {}
unsafe impl ::windows::runtime::Abi for MMTIME_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MM_3COM: u32 = 260u32;
pub const MM_3COM_CB_MIXER: u32 = 1u32;
pub const MM_3COM_CB_WAVEIN: u32 = 2u32;
pub const MM_3COM_CB_WAVEOUT: u32 = 3u32;
pub const MM_3DFX: u32 = 262u32;
pub const MM_AARDVARK: u32 = 11u32;
pub const MM_AARDVARK_STUDIO12_WAVEIN: u32 = 2u32;
pub const MM_AARDVARK_STUDIO12_WAVEOUT: u32 = 1u32;
pub const MM_AARDVARK_STUDIO88_WAVEIN: u32 = 4u32;
pub const MM_AARDVARK_STUDIO88_WAVEOUT: u32 = 3u32;
pub const MM_ACM_CLOSE: u32 = 981u32;
pub const MM_ACM_DONE: u32 = 982u32;
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MM_ACM_OPEN: u32 = 980u32;
pub const MM_ACTIVEVOICE: u32 = 225u32;
pub const MM_ACTIVEVOICE_ACM_VOXADPCM: u32 = 1u32;
pub const MM_ACULAB: u32 = 14u32;
pub const MM_ADDX: u32 = 118u32;
pub const MM_ADDX_PCTV_AUX_CD: u32 = 5u32;
pub const MM_ADDX_PCTV_AUX_LINE: u32 = 6u32;
pub const MM_ADDX_PCTV_DIGITALMIX: u32 = 1u32;
pub const MM_ADDX_PCTV_MIXER: u32 = 4u32;
pub const MM_ADDX_PCTV_WAVEIN: u32 = 2u32;
pub const MM_ADDX_PCTV_WAVEOUT: u32 = 3u32;
pub const MM_ADLACC: u32 = 91u32;
pub const MM_ADLIB: u32 = 9u32;
pub const MM_ADMOS: u32 = 235u32;
pub const MM_ADMOS_FM_SYNTH: u32 = 1u32;
pub const MM_ADMOS_QS3AMIDIIN: u32 = 3u32;
pub const MM_ADMOS_QS3AMIDIOUT: u32 = 2u32;
pub const MM_ADMOS_QS3AWAVEIN: u32 = 5u32;
pub const MM_ADMOS_QS3AWAVEOUT: u32 = 4u32;
pub const MM_AHEAD: u32 = 77u32;
pub const MM_AHEAD_GENERIC: u32 = 4u32;
pub const MM_AHEAD_MULTISOUND: u32 = 1u32;
pub const MM_AHEAD_PROAUDIO: u32 = 3u32;
pub const MM_AHEAD_SOUNDBLASTER: u32 = 2u32;
pub const MM_ALARIS: u32 = 174u32;
pub const MM_ALDIGITAL: u32 = 143u32;
pub const MM_ALESIS: u32 = 243u32;
pub const MM_ALGOVISION: u32 = 266u32;
pub const MM_ALGOVISION_VB80AUX: u32 = 4u32;
pub const MM_ALGOVISION_VB80AUX2: u32 = 5u32;
pub const MM_ALGOVISION_VB80MIXER: u32 = 3u32;
pub const MM_ALGOVISION_VB80WAVEIN: u32 = 2u32;
pub const MM_ALGOVISION_VB80WAVEOUT: u32 = 1u32;
pub const MM_AMD: u32 = 146u32;
pub const MM_AMD_INTERWAVE_AUX1: u32 = 10u32;
pub const MM_AMD_INTERWAVE_AUX2: u32 = 11u32;
pub const MM_AMD_INTERWAVE_AUX_CD: u32 = 13u32;
pub const MM_AMD_INTERWAVE_AUX_MIC: u32 = 12u32;
pub const MM_AMD_INTERWAVE_EX_CD: u32 = 7u32;
pub const MM_AMD_INTERWAVE_EX_TELEPHONY: u32 = 16u32;
pub const MM_AMD_INTERWAVE_JOYSTICK: u32 = 6u32;
pub const MM_AMD_INTERWAVE_MIDIIN: u32 = 8u32;
pub const MM_AMD_INTERWAVE_MIDIOUT: u32 = 9u32;
pub const MM_AMD_INTERWAVE_MIXER1: u32 = 4u32;
pub const MM_AMD_INTERWAVE_MIXER2: u32 = 5u32;
pub const MM_AMD_INTERWAVE_MONO_IN: u32 = 14u32;
pub const MM_AMD_INTERWAVE_MONO_OUT: u32 = 15u32;
pub const MM_AMD_INTERWAVE_STEREO_ENHANCED: u32 = 19u32;
pub const MM_AMD_INTERWAVE_SYNTH: u32 = 3u32;
pub const MM_AMD_INTERWAVE_WAVEIN: u32 = 1u32;
pub const MM_AMD_INTERWAVE_WAVEOUT: u32 = 2u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_BASE: u32 = 17u32;
pub const MM_AMD_INTERWAVE_WAVEOUT_TREBLE: u32 = 18u32;
pub const MM_ANALOGDEVICES: u32 = 252u32;
pub const MM_ANTEX: u32 = 31u32;
pub const MM_ANTEX_AUDIOPORT22_FEEDTHRU: u32 = 9u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEIN: u32 = 7u32;
pub const MM_ANTEX_AUDIOPORT22_WAVEOUT: u32 = 8u32;
pub const MM_ANTEX_SX12_WAVEIN: u32 = 1u32;
pub const MM_ANTEX_SX12_WAVEOUT: u32 = 2u32;
pub const MM_ANTEX_SX15_WAVEIN: u32 = 3u32;
pub const MM_ANTEX_SX15_WAVEOUT: u32 = 4u32;
pub const MM_ANTEX_VP625_WAVEIN: u32 = 5u32;
pub const MM_ANTEX_VP625_WAVEOUT: u32 = 6u32;
pub const MM_APICOM: u32 = 116u32;
pub const MM_APPLE: u32 = 99u32;
pub const MM_APPS: u32 = 42u32;
pub const MM_APT: u32 = 56u32;
pub const MM_APT_ACE100CD: u32 = 1u32;
pub const MM_ARRAY: u32 = 231u32;
pub const MM_ARTISOFT: u32 = 20u32;
pub const MM_ARTISOFT_SBWAVEIN: u32 = 1u32;
pub const MM_ARTISOFT_SBWAVEOUT: u32 = 2u32;
pub const MM_AST: u32 = 64u32;
pub const MM_AST_MODEMWAVE_WAVEIN: u32 = 13u32;
pub const MM_AST_MODEMWAVE_WAVEOUT: u32 = 14u32;
pub const MM_ATI: u32 = 27u32;
pub const MM_ATT: u32 = 185u32;
pub const MM_ATT_G729A: u32 = 1u32;
pub const MM_ATT_MICROELECTRONICS: u32 = 139u32;
pub const MM_AU8820_AUX: u32 = 21u32;
pub const MM_AU8820_MIDIIN: u32 = 23u32;
pub const MM_AU8820_MIDIOUT: u32 = 22u32;
pub const MM_AU8820_MIXER: u32 = 20u32;
pub const MM_AU8820_SYNTH: u32 = 17u32;
pub const MM_AU8820_WAVEIN: u32 = 19u32;
pub const MM_AU8820_WAVEOUT: u32 = 18u32;
pub const MM_AU8830_AUX: u32 = 37u32;
pub const MM_AU8830_MIDIIN: u32 = 39u32;
pub const MM_AU8830_MIDIOUT: u32 = 38u32;
pub const MM_AU8830_MIXER: u32 = 36u32;
pub const MM_AU8830_SYNTH: u32 = 33u32;
pub const MM_AU8830_WAVEIN: u32 = 35u32;
pub const MM_AU8830_WAVEOUT: u32 = 34u32;
pub const MM_AUDIOFILE: u32 = 47u32;
pub const MM_AUDIOPT: u32 = 74u32;
pub const MM_AUDIOSCIENCE: u32 = 217u32;
pub const MM_AURAVISION: u32 = 80u32;
pub const MM_AUREAL: u32 = 181u32;
pub const MM_AUREAL_AU8820: u32 = 16u32;
pub const MM_AUREAL_AU8830: u32 = 32u32;
pub const MM_AZTECH: u32 = 52u32;
pub const MM_AZTECH_AUX: u32 = 404u32;
pub const MM_AZTECH_AUX_CD: u32 = 401u32;
pub const MM_AZTECH_AUX_LINE: u32 = 402u32;
pub const MM_AZTECH_AUX_MIC: u32 = 403u32;
pub const MM_AZTECH_DSP16_FMSYNTH: u32 = 68u32;
pub const MM_AZTECH_DSP16_WAVEIN: u32 = 65u32;
pub const MM_AZTECH_DSP16_WAVEOUT: u32 = 66u32;
pub const MM_AZTECH_DSP16_WAVESYNTH: u32 = 70u32;
pub const MM_AZTECH_FMSYNTH: u32 = 20u32;
pub const MM_AZTECH_MIDIIN: u32 = 4u32;
pub const MM_AZTECH_MIDIOUT: u32 = 3u32;
pub const MM_AZTECH_MIXER: u32 = 21u32;
pub const MM_AZTECH_NOVA16_MIXER: u32 = 73u32;
pub const MM_AZTECH_NOVA16_WAVEIN: u32 = 71u32;
pub const MM_AZTECH_NOVA16_WAVEOUT: u32 = 72u32;
pub const MM_AZTECH_PRO16_FMSYNTH: u32 = 38u32;
pub const MM_AZTECH_PRO16_WAVEIN: u32 = 33u32;
pub const MM_AZTECH_PRO16_WAVEOUT: u32 = 34u32;
pub const MM_AZTECH_WASH16_MIXER: u32 = 76u32;
pub const MM_AZTECH_WASH16_WAVEIN: u32 = 74u32;
pub const MM_AZTECH_WASH16_WAVEOUT: u32 = 75u32;
pub const MM_AZTECH_WAVEIN: u32 = 17u32;
pub const MM_AZTECH_WAVEOUT: u32 = 18u32;
pub const MM_BCB: u32 = 192u32;
pub const MM_BCB_NETBOARD_10: u32 = 1u32;
pub const MM_BCB_TT75_10: u32 = 2u32;
pub const MM_BECUBED: u32 = 10u32;
pub const MM_BERCOS: u32 = 199u32;
pub const MM_BERCOS_MIXER: u32 = 2u32;
pub const MM_BERCOS_WAVEIN: u32 = 1u32;
pub const MM_BERCOS_WAVEOUT: u32 = 3u32;
pub const MM_BERKOM: u32 = 189u32;
pub const MM_BINTEC: u32 = 12u32;
pub const MM_BINTEC_TAPI_WAVE: u32 = 1u32;
pub const MM_BROOKTREE: u32 = 121u32;
pub const MM_BTV_AUX_CD: u32 = 8u32;
pub const MM_BTV_AUX_LINE: u32 = 6u32;
pub const MM_BTV_AUX_MIC: u32 = 7u32;
pub const MM_BTV_DIGITALIN: u32 = 9u32;
pub const MM_BTV_DIGITALOUT: u32 = 10u32;
pub const MM_BTV_MIDIIN: u32 = 3u32;
pub const MM_BTV_MIDIOUT: u32 = 4u32;
pub const MM_BTV_MIDISYNTH: u32 = 5u32;
pub const MM_BTV_MIDIWAVESTREAM: u32 = 11u32;
pub const MM_BTV_MIXER: u32 = 12u32;
pub const MM_BTV_WAVEIN: u32 = 1u32;
pub const MM_BTV_WAVEOUT: u32 = 2u32;
pub const MM_CANAM: u32 = 148u32;
pub const MM_CANAM_CBXWAVEIN: u32 = 2u32;
pub const MM_CANAM_CBXWAVEOUT: u32 = 1u32;
pub const MM_CANOPUS: u32 = 49u32;
pub const MM_CANOPUS_ACM_DVREX: u32 = 1u32;
pub const MM_CASIO: u32 = 162u32;
pub const MM_CASIO_LSG_MIDIOUT: u32 = 3u32;
pub const MM_CASIO_WP150_MIDIIN: u32 = 2u32;
pub const MM_CASIO_WP150_MIDIOUT: u32 = 1u32;
pub const MM_CAT: u32 = 41u32;
pub const MM_CAT_WAVEOUT: u32 = 1u32;
pub const MM_CDPC_AUX: u32 = 119u32;
pub const MM_CDPC_MIDIIN: u32 = 114u32;
pub const MM_CDPC_MIDIOUT: u32 = 113u32;
pub const MM_CDPC_MIXER: u32 = 118u32;
pub const MM_CDPC_SYNTH: u32 = 115u32;
pub const MM_CDPC_WAVEIN: u32 = 117u32;
pub const MM_CDPC_WAVEOUT: u32 = 116u32;
pub const MM_CHROMATIC: u32 = 155u32;
pub const MM_CHROMATIC_M1: u32 = 1u32;
pub const MM_CHROMATIC_M1_AUX: u32 = 6u32;
pub const MM_CHROMATIC_M1_AUX_CD: u32 = 7u32;
pub const MM_CHROMATIC_M1_FMSYNTH: u32 = 4u32;
pub const MM_CHROMATIC_M1_MIDIIN: u32 = 8u32;
pub const MM_CHROMATIC_M1_MIDIOUT: u32 = 9u32;
pub const MM_CHROMATIC_M1_MIXER: u32 = 5u32;
pub const MM_CHROMATIC_M1_MPEGWAVEIN: u32 = 17u32;
pub const MM_CHROMATIC_M1_MPEGWAVEOUT: u32 = 18u32;
pub const MM_CHROMATIC_M1_WAVEIN: u32 = 2u32;
pub const MM_CHROMATIC_M1_WAVEOUT: u32 = 3u32;
pub const MM_CHROMATIC_M1_WTSYNTH: u32 = 16u32;
pub const MM_CHROMATIC_M2: u32 = 19u32;
pub const MM_CHROMATIC_M2_AUX: u32 = 24u32;
pub const MM_CHROMATIC_M2_AUX_CD: u32 = 25u32;
pub const MM_CHROMATIC_M2_FMSYNTH: u32 = 22u32;
pub const MM_CHROMATIC_M2_MIDIIN: u32 = 32u32;
pub const MM_CHROMATIC_M2_MIDIOUT: u32 = 33u32;
pub const MM_CHROMATIC_M2_MIXER: u32 = 23u32;
pub const MM_CHROMATIC_M2_MPEGWAVEIN: u32 = 35u32;
pub const MM_CHROMATIC_M2_MPEGWAVEOUT: u32 = 36u32;
pub const MM_CHROMATIC_M2_WAVEIN: u32 = 20u32;
pub const MM_CHROMATIC_M2_WAVEOUT: u32 = 21u32;
pub const MM_CHROMATIC_M2_WTSYNTH: u32 = 34u32;
pub const MM_CIRRUSLOGIC: u32 = 105u32;
pub const MM_COLORGRAPH: u32 = 179u32;
pub const MM_COMPAQ: u32 = 92u32;
pub const MM_COMPAQ_BB_WAVEAUX: u32 = 3u32;
pub const MM_COMPAQ_BB_WAVEIN: u32 = 1u32;
pub const MM_COMPAQ_BB_WAVEOUT: u32 = 2u32;
pub const MM_COMPUSIC: u32 = 89u32;
pub const MM_COMPUTER_FRIENDS: u32 = 45u32;
pub const MM_CONCEPTS: u32 = 108u32;
pub const MM_CONNECTIX: u32 = 158u32;
pub const MM_CONNECTIX_VIDEC_CODEC: u32 = 1u32;
pub const MM_CONTROLRES: u32 = 84u32;
pub const MM_COREDYNAMICS: u32 = 147u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_VGA: u32 = 9u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_IN: u32 = 10u32;
pub const MM_COREDYNAMICS_DYNAGRAFX_WAVE_OUT: u32 = 11u32;
pub const MM_COREDYNAMICS_DYNAMIXHR: u32 = 1u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_IN: u32 = 7u32;
pub const MM_COREDYNAMICS_DYNASONIX_AUDIO_OUT: u32 = 8u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_IN: u32 = 3u32;
pub const MM_COREDYNAMICS_DYNASONIX_MIDI_OUT: u32 = 4u32;
pub const MM_COREDYNAMICS_DYNASONIX_SYNTH: u32 = 2u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_IN: u32 = 5u32;
pub const MM_COREDYNAMICS_DYNASONIX_WAVE_OUT: u32 = 6u32;
pub const MM_CREATIVE: u32 = 2u32;
pub const MM_CREATIVE_AUX_CD: u32 = 401u32;
pub const MM_CREATIVE_AUX_LINE: u32 = 402u32;
pub const MM_CREATIVE_AUX_MASTER: u32 = 404u32;
pub const MM_CREATIVE_AUX_MIC: u32 = 403u32;
pub const MM_CREATIVE_AUX_MIDI: u32 = 407u32;
pub const MM_CREATIVE_AUX_PCSPK: u32 = 405u32;
pub const MM_CREATIVE_AUX_WAVE: u32 = 406u32;
pub const MM_CREATIVE_FMSYNTH_MONO: u32 = 301u32;
pub const MM_CREATIVE_FMSYNTH_STEREO: u32 = 302u32;
pub const MM_CREATIVE_MIDIIN: u32 = 202u32;
pub const MM_CREATIVE_MIDIOUT: u32 = 201u32;
pub const MM_CREATIVE_MIDI_AWE32: u32 = 303u32;
pub const MM_CREATIVE_PHNBLST_WAVEIN: u32 = 5u32;
pub const MM_CREATIVE_PHNBLST_WAVEOUT: u32 = 105u32;
pub const MM_CREATIVE_SB15_WAVEIN: u32 = 1u32;
pub const MM_CREATIVE_SB15_WAVEOUT: u32 = 101u32;
pub const MM_CREATIVE_SB16_MIXER: u32 = 409u32;
pub const MM_CREATIVE_SB20_WAVEIN: u32 = 2u32;
pub const MM_CREATIVE_SB20_WAVEOUT: u32 = 102u32;
pub const MM_CREATIVE_SBP16_WAVEIN: u32 = 4u32;
pub const MM_CREATIVE_SBP16_WAVEOUT: u32 = 104u32;
pub const MM_CREATIVE_SBPRO_MIXER: u32 = 408u32;
pub const MM_CREATIVE_SBPRO_WAVEIN: u32 = 3u32;
pub const MM_CREATIVE_SBPRO_WAVEOUT: u32 = 103u32;
pub const MM_CRYSTAL: u32 = 132u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_AUX1: u32 = 13u32;
pub const MM_CRYSTAL_CS4232_INPUTGAIN_LOOP: u32 = 14u32;
pub const MM_CRYSTAL_CS4232_MIDIIN: u32 = 9u32;
pub const MM_CRYSTAL_CS4232_MIDIOUT: u32 = 10u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX1: u32 = 4u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_AUX2: u32 = 5u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_LINE: u32 = 6u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MASTER: u32 = 8u32;
pub const MM_CRYSTAL_CS4232_WAVEAUX_MONO: u32 = 7u32;
pub const MM_CRYSTAL_CS4232_WAVEIN: u32 = 1u32;
pub const MM_CRYSTAL_CS4232_WAVEMIXER: u32 = 3u32;
pub const MM_CRYSTAL_CS4232_WAVEOUT: u32 = 2u32;
pub const MM_CRYSTAL_NET: u32 = 154u32;
pub const MM_CRYSTAL_SOUND_FUSION_JOYSTICK: u32 = 26u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIIN: u32 = 24u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIDIOUT: u32 = 25u32;
pub const MM_CRYSTAL_SOUND_FUSION_MIXER: u32 = 23u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEIN: u32 = 21u32;
pub const MM_CRYSTAL_SOUND_FUSION_WAVEOUT: u32 = 22u32;
pub const MM_CS: u32 = 242u32;
pub const MM_CYRIX: u32 = 6u32;
pub const MM_CYRIX_XAAUX: u32 = 6u32;
pub const MM_CYRIX_XAMIDIIN: u32 = 2u32;
pub const MM_CYRIX_XAMIDIOUT: u32 = 3u32;
pub const MM_CYRIX_XAMIXER: u32 = 7u32;
pub const MM_CYRIX_XASYNTH: u32 = 1u32;
pub const MM_CYRIX_XAWAVEIN: u32 = 4u32;
pub const MM_CYRIX_XAWAVEOUT: u32 = 5u32;
pub const MM_DATAFUSION: u32 = 196u32;
pub const MM_DATARAN: u32 = 232u32;
pub const MM_DDD: u32 = 151u32;
pub const MM_DDD_MIDILINK_MIDIIN: u32 = 1u32;
pub const MM_DDD_MIDILINK_MIDIOUT: u32 = 2u32;
pub const MM_DF_ACM_G726: u32 = 1u32;
pub const MM_DF_ACM_GSM610: u32 = 2u32;
pub const MM_DIACOUSTICS: u32 = 129u32;
pub const MM_DIACOUSTICS_DRUM_ACTION: u32 = 1u32;
pub const MM_DIALOGIC: u32 = 93u32;
pub const MM_DIAMONDMM: u32 = 163u32;
pub const MM_DICTAPHONE: u32 = 214u32;
pub const MM_DICTAPHONE_G726: u32 = 1u32;
pub const MM_DIGIGRAM: u32 = 227u32;
pub const MM_DIGITAL: u32 = 100u32;
pub const MM_DIGITAL_ACM_G723: u32 = 3u32;
pub const MM_DIGITAL_AUDIO_LABS: u32 = 136u32;
pub const MM_DIGITAL_AUDIO_LABS_CDLX: u32 = 19u32;
pub const MM_DIGITAL_AUDIO_LABS_CPRO: u32 = 17u32;
pub const MM_DIGITAL_AUDIO_LABS_CTDIF: u32 = 20u32;
pub const MM_DIGITAL_AUDIO_LABS_DOC: u32 = 2u32;
pub const MM_DIGITAL_AUDIO_LABS_TC: u32 = 1u32;
pub const MM_DIGITAL_AUDIO_LABS_V8: u32 = 16u32;
pub const MM_DIGITAL_AUDIO_LABS_VP: u32 = 18u32;
pub const MM_DIGITAL_AV320_WAVEIN: u32 = 1u32;
pub const MM_DIGITAL_AV320_WAVEOUT: u32 = 2u32;
pub const MM_DIGITAL_ICM_H261: u32 = 5u32;
pub const MM_DIGITAL_ICM_H263: u32 = 4u32;
pub const MM_DIMD_AUX_LINE: u32 = 9u32;
pub const MM_DIMD_DIRSOUND: u32 = 1u32;
pub const MM_DIMD_MIDIIN: u32 = 7u32;
pub const MM_DIMD_MIDIOUT: u32 = 8u32;
pub const MM_DIMD_MIXER: u32 = 10u32;
pub const MM_DIMD_PLATFORM: u32 = 0u32;
pub const MM_DIMD_VIRTJOY: u32 = 4u32;
pub const MM_DIMD_VIRTMPU: u32 = 2u32;
pub const MM_DIMD_VIRTSB: u32 = 3u32;
pub const MM_DIMD_WAVEIN: u32 = 5u32;
pub const MM_DIMD_WAVEOUT: u32 = 6u32;
pub const MM_DIMD_WSS_AUX: u32 = 21u32;
pub const MM_DIMD_WSS_MIXER: u32 = 17u32;
pub const MM_DIMD_WSS_SYNTH: u32 = 76u32;
pub const MM_DIMD_WSS_WAVEIN: u32 = 14u32;
pub const MM_DIMD_WSS_WAVEOUT: u32 = 15u32;
pub const MM_DOLBY: u32 = 78u32;
pub const MM_DPSINC: u32 = 191u32;
pub const MM_DRVM_CLOSE: u32 = 977u32;
pub const MM_DRVM_DATA: u32 = 978u32;
pub const MM_DRVM_ERROR: u32 = 979u32;
pub const MM_DRVM_OPEN: u32 = 976u32;
pub const MM_DSP_GROUP: u32 = 43u32;
pub const MM_DSP_GROUP_TRUESPEECH: u32 = 1u32;
pub const MM_DSP_SOLUTIONS: u32 = 25u32;
pub const MM_DSP_SOLUTIONS_AUX: u32 = 4u32;
pub const MM_DSP_SOLUTIONS_SYNTH: u32 = 3u32;
pub const MM_DSP_SOLUTIONS_WAVEIN: u32 = 2u32;
pub const MM_DSP_SOLUTIONS_WAVEOUT: u32 = 1u32;
pub const MM_DTS: u32 = 226u32;
pub const MM_DTS_DS: u32 = 1u32;
pub const MM_DUCK: u32 = 197u32;
pub const MM_DVISION: u32 = 165u32;
pub const MM_ECHO: u32 = 39u32;
pub const MM_ECHO_AUX: u32 = 6u32;
pub const MM_ECHO_MIDIIN: u32 = 5u32;
pub const MM_ECHO_MIDIOUT: u32 = 4u32;
pub const MM_ECHO_SYNTH: u32 = 1u32;
pub const MM_ECHO_WAVEIN: u32 = 3u32;
pub const MM_ECHO_WAVEOUT: u32 = 2u32;
pub const MM_ECS: u32 = 145u32;
pub const MM_ECS_AADF_MIDI_IN: u32 = 10u32;
pub const MM_ECS_AADF_MIDI_OUT: u32 = 11u32;
pub const MM_ECS_AADF_WAVE2MIDI_IN: u32 = 12u32;
pub const MM_EES: u32 = 219u32;
pub const MM_EES_PCMIDI14: u32 = 1u32;
pub const MM_EES_PCMIDI14_IN: u32 = 2u32;
pub const MM_EES_PCMIDI14_OUT1: u32 = 3u32;
pub const MM_EES_PCMIDI14_OUT2: u32 = 4u32;
pub const MM_EES_PCMIDI14_OUT3: u32 = 5u32;
pub const MM_EES_PCMIDI14_OUT4: u32 = 6u32;
pub const MM_EMAGIC: u32 = 208u32;
pub const MM_EMAGIC_UNITOR8: u32 = 1u32;
pub const MM_EMU: u32 = 19u32;
pub const MM_EMU_APSMIDIIN: u32 = 2u32;
pub const MM_EMU_APSMIDIOUT: u32 = 3u32;
pub const MM_EMU_APSSYNTH: u32 = 1u32;
pub const MM_EMU_APSWAVEIN: u32 = 4u32;
pub const MM_EMU_APSWAVEOUT: u32 = 5u32;
pub const MM_ENET: u32 = 206u32;
pub const MM_ENET_T2000_HANDSETIN: u32 = 3u32;
pub const MM_ENET_T2000_HANDSETOUT: u32 = 4u32;
pub const MM_ENET_T2000_LINEIN: u32 = 1u32;
pub const MM_ENET_T2000_LINEOUT: u32 = 2u32;
pub const MM_ENSONIQ: u32 = 125u32;
pub const MM_ENSONIQ_SOUNDSCAPE: u32 = 16u32;
pub const MM_EPSON: u32 = 50u32;
pub const MM_EPS_FMSND: u32 = 1u32;
pub const MM_ESS: u32 = 46u32;
pub const MM_ESS_AMAUX: u32 = 3u32;
pub const MM_ESS_AMMIDIIN: u32 = 6u32;
pub const MM_ESS_AMMIDIOUT: u32 = 5u32;
pub const MM_ESS_AMSYNTH: u32 = 4u32;
pub const MM_ESS_AMWAVEIN: u32 = 2u32;
pub const MM_ESS_AMWAVEOUT: u32 = 1u32;
pub const MM_ESS_AUX_CD: u32 = 8u32;
pub const MM_ESS_ES1488_MIXER: u32 = 24u32;
pub const MM_ESS_ES1488_WAVEIN: u32 = 23u32;
pub const MM_ESS_ES1488_WAVEOUT: u32 = 22u32;
pub const MM_ESS_ES1688_MIXER: u32 = 27u32;
pub const MM_ESS_ES1688_WAVEIN: u32 = 26u32;
pub const MM_ESS_ES1688_WAVEOUT: u32 = 25u32;
pub const MM_ESS_ES1788_MIXER: u32 = 30u32;
pub const MM_ESS_ES1788_WAVEIN: u32 = 29u32;
pub const MM_ESS_ES1788_WAVEOUT: u32 = 28u32;
pub const MM_ESS_ES1868_MIXER: u32 = 36u32;
pub const MM_ESS_ES1868_WAVEIN: u32 = 35u32;
pub const MM_ESS_ES1868_WAVEOUT: u32 = 34u32;
pub const MM_ESS_ES1878_MIXER: u32 = 39u32;
pub const MM_ESS_ES1878_WAVEIN: u32 = 38u32;
pub const MM_ESS_ES1878_WAVEOUT: u32 = 37u32;
pub const MM_ESS_ES1888_MIXER: u32 = 33u32;
pub const MM_ESS_ES1888_WAVEIN: u32 = 32u32;
pub const MM_ESS_ES1888_WAVEOUT: u32 = 31u32;
pub const MM_ESS_ES488_MIXER: u32 = 18u32;
pub const MM_ESS_ES488_WAVEIN: u32 = 17u32;
pub const MM_ESS_ES488_WAVEOUT: u32 = 16u32;
pub const MM_ESS_ES688_MIXER: u32 = 21u32;
pub const MM_ESS_ES688_WAVEIN: u32 = 20u32;
pub const MM_ESS_ES688_WAVEOUT: u32 = 19u32;
pub const MM_ESS_MIXER: u32 = 7u32;
pub const MM_ESS_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ESS_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ETEK: u32 = 241u32;
pub const MM_ETEK_KWIKMIDI_MIDIIN: u32 = 1u32;
pub const MM_ETEK_KWIKMIDI_MIDIOUT: u32 = 2u32;
pub const MM_EUPHONICS: u32 = 152u32;
pub const MM_EUPHONICS_AUX_CD: u32 = 1u32;
pub const MM_EUPHONICS_AUX_LINE: u32 = 2u32;
pub const MM_EUPHONICS_AUX_MASTER: u32 = 3u32;
pub const MM_EUPHONICS_AUX_MIC: u32 = 4u32;
pub const MM_EUPHONICS_AUX_MIDI: u32 = 5u32;
pub const MM_EUPHONICS_AUX_WAVE: u32 = 6u32;
pub const MM_EUPHONICS_EUSYNTH: u32 = 14u32;
pub const MM_EUPHONICS_FMSYNTH_MONO: u32 = 7u32;
pub const MM_EUPHONICS_FMSYNTH_STEREO: u32 = 8u32;
pub const MM_EUPHONICS_MIDIIN: u32 = 9u32;
pub const MM_EUPHONICS_MIDIOUT: u32 = 10u32;
pub const MM_EUPHONICS_MIXER: u32 = 11u32;
pub const MM_EUPHONICS_WAVEIN: u32 = 12u32;
pub const MM_EUPHONICS_WAVEOUT: u32 = 13u32;
pub const MM_EVEREX: u32 = 38u32;
pub const MM_EVEREX_CARRIER: u32 = 1u32;
pub const MM_EXAN: u32 = 63u32;
pub const MM_FAITH: u32 = 15u32;
pub const MM_FAST: u32 = 126u32;
pub const MM_FHGIIS_MPEGLAYER3: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCED: u32 = 12u32;
pub const MM_FHGIIS_MPEGLAYER3_ADVANCEDPLUS: u32 = 14u32;
pub const MM_FHGIIS_MPEGLAYER3_BASIC: u32 = 11u32;
pub const MM_FHGIIS_MPEGLAYER3_DECODE: u32 = 9u32;
pub const MM_FHGIIS_MPEGLAYER3_LITE: u32 = 10u32;
pub const MM_FHGIIS_MPEGLAYER3_PROFESSIONAL: u32 = 13u32;
pub const MM_FLEXION: u32 = 249u32;
pub const MM_FLEXION_X300_WAVEIN: u32 = 1u32;
pub const MM_FLEXION_X300_WAVEOUT: u32 = 2u32;
pub const MM_FORTEMEDIA: u32 = 229u32;
pub const MM_FORTEMEDIA_AUX: u32 = 5u32;
pub const MM_FORTEMEDIA_FMSYNC: u32 = 3u32;
pub const MM_FORTEMEDIA_MIXER: u32 = 4u32;
pub const MM_FORTEMEDIA_WAVEIN: u32 = 1u32;
pub const MM_FORTEMEDIA_WAVEOUT: u32 = 2u32;
pub const MM_FRAUNHOFER_IIS: u32 = 172u32;
pub const MM_FRONTIER: u32 = 160u32;
pub const MM_FRONTIER_WAVECENTER_MIDIIN: u32 = 1u32;
pub const MM_FRONTIER_WAVECENTER_MIDIOUT: u32 = 2u32;
pub const MM_FRONTIER_WAVECENTER_WAVEIN: u32 = 3u32;
pub const MM_FRONTIER_WAVECENTER_WAVEOUT: u32 = 4u32;
pub const MM_FTR: u32 = 198u32;
pub const MM_FTR_ACM: u32 = 2u32;
pub const MM_FTR_ENCODER_WAVEIN: u32 = 1u32;
pub const MM_FUJITSU: u32 = 4u32;
pub const MM_GADGETLABS: u32 = 159u32;
pub const MM_GADGETLABS_WAVE42_WAVEIN: u32 = 3u32;
pub const MM_GADGETLABS_WAVE42_WAVEOUT: u32 = 4u32;
pub const MM_GADGETLABS_WAVE44_WAVEIN: u32 = 1u32;
pub const MM_GADGETLABS_WAVE44_WAVEOUT: u32 = 2u32;
pub const MM_GADGETLABS_WAVE4_MIDIIN: u32 = 5u32;
pub const MM_GADGETLABS_WAVE4_MIDIOUT: u32 = 6u32;
pub const MM_GRANDE: u32 = 117u32;
pub const MM_GRAVIS: u32 = 34u32;
pub const MM_GUILLEMOT: u32 = 207u32;
pub const MM_GULBRANSEN: u32 = 130u32;
pub const MM_HAFTMANN: u32 = 220u32;
pub const MM_HAFTMANN_LPTDAC2: u32 = 1u32;
pub const MM_HEADSPACE: u32 = 222u32;
pub const MM_HEADSPACE_HAEMIXER: u32 = 4u32;
pub const MM_HEADSPACE_HAESYNTH: u32 = 1u32;
pub const MM_HEADSPACE_HAEWAVEIN: u32 = 3u32;
pub const MM_HEADSPACE_HAEWAVEOUT: u32 = 2u32;
pub const MM_HEWLETT_PACKARD: u32 = 13u32;
pub const MM_HEWLETT_PACKARD_CU_CODEC: u32 = 1u32;
pub const MM_HORIZONS: u32 = 107u32;
pub const MM_HP: u32 = 253u32;
pub const MM_HP_WAVEIN: u32 = 2u32;
pub const MM_HP_WAVEOUT: u32 = 1u32;
pub const MM_HYPERACTIVE: u32 = 246u32;
pub const MM_IBM: u32 = 22u32;
pub const MM_IBM_MWAVE_AUX: u32 = 23u32;
pub const MM_IBM_MWAVE_MIDIIN: u32 = 21u32;
pub const MM_IBM_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_IBM_MWAVE_MIXER: u32 = 20u32;
pub const MM_IBM_MWAVE_WAVEIN: u32 = 18u32;
pub const MM_IBM_MWAVE_WAVEOUT: u32 = 19u32;
pub const MM_IBM_PCMCIA_AUX: u32 = 16u32;
pub const MM_IBM_PCMCIA_MIDIIN: u32 = 14u32;
pub const MM_IBM_PCMCIA_MIDIOUT: u32 = 15u32;
pub const MM_IBM_PCMCIA_SYNTH: u32 = 13u32;
pub const MM_IBM_PCMCIA_WAVEIN: u32 = 11u32;
pub const MM_IBM_PCMCIA_WAVEOUT: u32 = 12u32;
pub const MM_IBM_THINKPAD200: u32 = 17u32;
pub const MM_IBM_WC_MIDIOUT: u32 = 30u32;
pub const MM_IBM_WC_MIXEROUT: u32 = 33u32;
pub const MM_IBM_WC_WAVEOUT: u32 = 31u32;
pub const MM_ICCC: u32 = 259u32;
pub const MM_ICCC_UNA3_AUX: u32 = 3u32;
pub const MM_ICCC_UNA3_MIXER: u32 = 4u32;
pub const MM_ICCC_UNA3_WAVEIN: u32 = 1u32;
pub const MM_ICCC_UNA3_WAVEOUT: u32 = 2u32;
pub const MM_ICE: u32 = 239u32;
pub const MM_ICE_AUX: u32 = 11u32;
pub const MM_ICE_MIDIIN1: u32 = 6u32;
pub const MM_ICE_MIDIIN2: u32 = 8u32;
pub const MM_ICE_MIDIOUT1: u32 = 5u32;
pub const MM_ICE_MIDIOUT2: u32 = 7u32;
pub const MM_ICE_MIXER: u32 = 10u32;
pub const MM_ICE_MTWAVEIN: u32 = 4u32;
pub const MM_ICE_MTWAVEOUT: u32 = 3u32;
pub const MM_ICE_SYNTH: u32 = 9u32;
pub const MM_ICE_WAVEIN: u32 = 2u32;
pub const MM_ICE_WAVEOUT: u32 = 1u32;
pub const MM_ICL_PS: u32 = 32u32;
pub const MM_ICOM_AUX: u32 = 6u32;
pub const MM_ICOM_LINE: u32 = 7u32;
pub const MM_ICOM_MIXER: u32 = 5u32;
pub const MM_ICOM_WAVEIN: u32 = 3u32;
pub const MM_ICOM_WAVEOUT: u32 = 4u32;
pub const MM_ICS: u32 = 57u32;
pub const MM_ICS_2115_LITE_MIDIOUT: u32 = 13u32;
pub const MM_ICS_2120_LITE_MIDIOUT: u32 = 14u32;
pub const MM_ICS_WAVEDECK_AUX: u32 = 4u32;
pub const MM_ICS_WAVEDECK_MIXER: u32 = 3u32;
pub const MM_ICS_WAVEDECK_SYNTH: u32 = 5u32;
pub const MM_ICS_WAVEDECK_WAVEIN: u32 = 2u32;
pub const MM_ICS_WAVEDECK_WAVEOUT: u32 = 1u32;
pub const MM_ICS_WAVEDEC_SB_AUX: u32 = 12u32;
pub const MM_ICS_WAVEDEC_SB_FM_MIDIOUT: u32 = 8u32;
pub const MM_ICS_WAVEDEC_SB_MIXER: u32 = 11u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIIN: u32 = 10u32;
pub const MM_ICS_WAVEDEC_SB_MPU401_MIDIOUT: u32 = 9u32;
pub const MM_ICS_WAVEDEC_SB_WAVEIN: u32 = 7u32;
pub const MM_ICS_WAVEDEC_SB_WAVEOUT: u32 = 6u32;
pub const MM_INSOFT: u32 = 94u32;
pub const MM_INTEL: u32 = 33u32;
pub const MM_INTELOPD_AUX: u32 = 401u32;
pub const MM_INTELOPD_WAVEIN: u32 = 1u32;
pub const MM_INTELOPD_WAVEOUT: u32 = 101u32;
pub const MM_INTEL_NSPMODEMLINEIN: u32 = 501u32;
pub const MM_INTEL_NSPMODEMLINEOUT: u32 = 502u32;
pub const MM_INTERACTIVE: u32 = 36u32;
pub const MM_INTERACTIVE_WAVEIN: u32 = 69u32;
pub const MM_INTERACTIVE_WAVEOUT: u32 = 69u32;
pub const MM_INTERNET: u32 = 244u32;
pub const MM_INTERNET_SSW_MIDIIN: u32 = 11u32;
pub const MM_INTERNET_SSW_MIDIOUT: u32 = 10u32;
pub const MM_INTERNET_SSW_WAVEIN: u32 = 13u32;
pub const MM_INTERNET_SSW_WAVEOUT: u32 = 12u32;
pub const MM_INVISION: u32 = 188u32;
pub const MM_IODD: u32 = 258u32;
pub const MM_IOMAGIC: u32 = 82u32;
pub const MM_IOMAGIC_TEMPO_AUXOUT: u32 = 6u32;
pub const MM_IOMAGIC_TEMPO_MIDIOUT: u32 = 4u32;
pub const MM_IOMAGIC_TEMPO_MXDOUT: u32 = 5u32;
pub const MM_IOMAGIC_TEMPO_SYNTH: u32 = 3u32;
pub const MM_IOMAGIC_TEMPO_WAVEIN: u32 = 2u32;
pub const MM_IOMAGIC_TEMPO_WAVEOUT: u32 = 1u32;
pub const MM_IPI: u32 = 238u32;
pub const MM_IPI_ACM_HSX: u32 = 1u32;
pub const MM_IPI_ACM_RPELP: u32 = 2u32;
pub const MM_IPI_AT_MIXER: u32 = 6u32;
pub const MM_IPI_AT_WAVEIN: u32 = 5u32;
pub const MM_IPI_AT_WAVEOUT: u32 = 4u32;
pub const MM_IPI_WF_ASSS: u32 = 3u32;
pub const MM_ISOLUTION: u32 = 106u32;
pub const MM_ISOLUTION_PASCAL: u32 = 1u32;
pub const MM_ITERATEDSYS: u32 = 58u32;
pub const MM_ITERATEDSYS_FUFCODEC: u32 = 1u32;
pub const MM_I_LINK: u32 = 233u32;
pub const MM_I_LINK_VOICE_CODER: u32 = 1u32;
pub const MM_JOY1BUTTONDOWN: u32 = 949u32;
pub const MM_JOY1BUTTONUP: u32 = 951u32;
pub const MM_JOY1MOVE: u32 = 928u32;
pub const MM_JOY1ZMOVE: u32 = 930u32;
pub const MM_JOY2BUTTONDOWN: u32 = 950u32;
pub const MM_JOY2BUTTONUP: u32 = 952u32;
pub const MM_JOY2MOVE: u32 = 929u32;
pub const MM_JOY2ZMOVE: u32 = 931u32;
pub const MM_KAY_ELEMETRICS: u32 = 131u32;
pub const MM_KAY_ELEMETRICS_CSL: u32 = 17152u32;
pub const MM_KAY_ELEMETRICS_CSL_4CHANNEL: u32 = 17161u32;
pub const MM_KAY_ELEMETRICS_CSL_DAT: u32 = 17160u32;
pub const MM_KORG: u32 = 55u32;
pub const MM_KORG_1212IO_MSWAVEIN: u32 = 3u32;
pub const MM_KORG_1212IO_MSWAVEOUT: u32 = 4u32;
pub const MM_KORG_PCIF_MIDIIN: u32 = 2u32;
pub const MM_KORG_PCIF_MIDIOUT: u32 = 1u32;
pub const MM_LERNOUT_ANDHAUSPIE_LHCODECACM: u32 = 1u32;
pub const MM_LERNOUT_AND_HAUSPIE: u32 = 97u32;
pub const MM_LEXICON: u32 = 236u32;
pub const MM_LEXICON_STUDIO_WAVE_IN: u32 = 2u32;
pub const MM_LEXICON_STUDIO_WAVE_OUT: u32 = 1u32;
pub const MM_LOGITECH: u32 = 60u32;
pub const MM_LUCENT: u32 = 184u32;
pub const MM_LUCENT_ACM_G723: u32 = 0u32;
pub const MM_LUCID: u32 = 221u32;
pub const MM_LUCID_PCI24WAVEIN: u32 = 1u32;
pub const MM_LUCID_PCI24WAVEOUT: u32 = 2u32;
pub const MM_LUMINOSITI: u32 = 224u32;
pub const MM_LUMINOSITI_SCWAVEIN: u32 = 1u32;
pub const MM_LUMINOSITI_SCWAVEMIX: u32 = 3u32;
pub const MM_LUMINOSITI_SCWAVEOUT: u32 = 2u32;
pub const MM_LYNX: u32 = 212u32;
pub const MM_LYRRUS: u32 = 88u32;
pub const MM_LYRRUS_BRIDGE_GUITAR: u32 = 1u32;
pub const MM_MALDEN: u32 = 261u32;
pub const MM_MARIAN: u32 = 190u32;
pub const MM_MARIAN_ARC44WAVEIN: u32 = 1u32;
pub const MM_MARIAN_ARC44WAVEOUT: u32 = 2u32;
pub const MM_MARIAN_ARC88WAVEIN: u32 = 5u32;
pub const MM_MARIAN_ARC88WAVEOUT: u32 = 6u32;
pub const MM_MARIAN_PRODIF24WAVEIN: u32 = 3u32;
pub const MM_MARIAN_PRODIF24WAVEOUT: u32 = 4u32;
pub const MM_MATROX_DIV: u32 = 254u32;
pub const MM_MATSUSHITA: u32 = 83u32;
pub const MM_MATSUSHITA_AUX: u32 = 5u32;
pub const MM_MATSUSHITA_FMSYNTH_STEREO: u32 = 3u32;
pub const MM_MATSUSHITA_MIXER: u32 = 4u32;
pub const MM_MATSUSHITA_WAVEIN: u32 = 1u32;
pub const MM_MATSUSHITA_WAVEOUT: u32 = 2u32;
pub const MM_MCINOTIFY: u32 = 953u32;
pub const MM_MCISIGNAL: u32 = 971u32;
pub const MM_MEDIASONIC: u32 = 71u32;
pub const MM_MEDIASONIC_ACM_G723: u32 = 1u32;
pub const MM_MEDIASONIC_ICOM: u32 = 2u32;
pub const MM_MEDIATRIX: u32 = 141u32;
pub const MM_MEDIAVISION: u32 = 3u32;
pub const MM_MEDIAVISION_CDPC: u32 = 112u32;
pub const MM_MEDIAVISION_OPUS1208: u32 = 128u32;
pub const MM_MEDIAVISION_OPUS1216: u32 = 144u32;
pub const MM_MEDIAVISION_PROAUDIO: u32 = 16u32;
pub const MM_MEDIAVISION_PROAUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_PROAUDIO_PLUS: u32 = 80u32;
pub const MM_MEDIAVISION_PROSTUDIO_16: u32 = 96u32;
pub const MM_MEDIAVISION_THUNDER: u32 = 32u32;
pub const MM_MEDIAVISION_TPORT: u32 = 64u32;
pub const MM_MELABS: u32 = 44u32;
pub const MM_MELABS_MIDI2GO: u32 = 1u32;
pub const MM_MERGING_MPEGL3: u32 = 1u32;
pub const MM_MERGING_TECHNOLOGIES: u32 = 177u32;
pub const MM_METHEUS: u32 = 59u32;
pub const MM_METHEUS_ZIPPER: u32 = 1u32;
pub const MM_MICRONAS: u32 = 251u32;
pub const MM_MICRONAS_CLP833: u32 = 2u32;
pub const MM_MICRONAS_SC4: u32 = 1u32;
pub const MM_MICROSOFT: u32 = 1u32;
pub const MM_MIDI_MAPPER: u32 = 1u32;
pub const MM_MIM_CLOSE: u32 = 962u32;
pub const MM_MIM_DATA: u32 = 963u32;
pub const MM_MIM_ERROR: u32 = 965u32;
pub const MM_MIM_LONGDATA: u32 = 964u32;
pub const MM_MIM_LONGERROR: u32 = 966u32;
pub const MM_MIM_MOREDATA: u32 = 972u32;
pub const MM_MIM_OPEN: u32 = 961u32;
pub const MM_MINDMAKER: u32 = 263u32;
pub const MM_MINDMAKER_GC_MIXER: u32 = 3u32;
pub const MM_MINDMAKER_GC_WAVEIN: u32 = 1u32;
pub const MM_MINDMAKER_GC_WAVEOUT: u32 = 2u32;
pub const MM_MIRO: u32 = 104u32;
pub const MM_MIRO_DC30_MIX: u32 = 7u32;
pub const MM_MIRO_DC30_WAVEIN: u32 = 6u32;
pub const MM_MIRO_DC30_WAVEOUT: u32 = 5u32;
pub const MM_MIRO_MOVIEPRO: u32 = 1u32;
pub const MM_MIRO_VIDEOD1: u32 = 2u32;
pub const MM_MIRO_VIDEODC1TV: u32 = 3u32;
pub const MM_MIRO_VIDEOTD: u32 = 4u32;
pub const MM_MITEL: u32 = 16u32;
pub const MM_MITEL_MEDIAPATH_WAVEIN: u32 = 301u32;
pub const MM_MITEL_MEDIAPATH_WAVEOUT: u32 = 300u32;
pub const MM_MITEL_MPA_HANDSET_WAVEIN: u32 = 201u32;
pub const MM_MITEL_MPA_HANDSET_WAVEOUT: u32 = 200u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEIN: u32 = 203u32;
pub const MM_MITEL_MPA_HANDSFREE_WAVEOUT: u32 = 202u32;
pub const MM_MITEL_MPA_LINE1_WAVEIN: u32 = 205u32;
pub const MM_MITEL_MPA_LINE1_WAVEOUT: u32 = 204u32;
pub const MM_MITEL_MPA_LINE2_WAVEIN: u32 = 207u32;
pub const MM_MITEL_MPA_LINE2_WAVEOUT: u32 = 206u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEIN: u32 = 105u32;
pub const MM_MITEL_TALKTO_BRIDGED_WAVEOUT: u32 = 104u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEIN: u32 = 103u32;
pub const MM_MITEL_TALKTO_HANDSET_WAVEOUT: u32 = 102u32;
pub const MM_MITEL_TALKTO_LINE_WAVEIN: u32 = 101u32;
pub const MM_MITEL_TALKTO_LINE_WAVEOUT: u32 = 100u32;
pub const MM_MIXM_CONTROL_CHANGE: u32 = 977u32;
pub const MM_MIXM_LINE_CHANGE: u32 = 976u32;
pub const MM_MMOTION_WAVEAUX: u32 = 1u32;
pub const MM_MMOTION_WAVEIN: u32 = 3u32;
pub const MM_MMOTION_WAVEOUT: u32 = 2u32;
pub const MM_MOM_CLOSE: u32 = 968u32;
pub const MM_MOM_DONE: u32 = 969u32;
pub const MM_MOM_OPEN: u32 = 967u32;
pub const MM_MOM_POSITIONCB: u32 = 970u32;
pub const MM_MOSCOM: u32 = 68u32;
pub const MM_MOSCOM_VPC2400_IN: u32 = 1u32;
pub const MM_MOSCOM_VPC2400_OUT: u32 = 2u32;
pub const MM_MOTIONPIXELS: u32 = 193u32;
pub const MM_MOTIONPIXELS_MVI2: u32 = 1u32;
pub const MM_MOTOROLA: u32 = 48u32;
pub const MM_MOTU: u32 = 101u32;
pub const MM_MOTU_DTX_MIDI_IN_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_IN_B: u32 = 802u32;
pub const MM_MOTU_DTX_MIDI_IN_SYNC: u32 = 800u32;
pub const MM_MOTU_DTX_MIDI_OUT_A: u32 = 801u32;
pub const MM_MOTU_DTX_MIDI_OUT_B: u32 = 802u32;
pub const MM_MOTU_FLYER_MIDI_IN_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_IN_B: u32 = 602u32;
pub const MM_MOTU_FLYER_MIDI_IN_SYNC: u32 = 600u32;
pub const MM_MOTU_FLYER_MIDI_OUT_A: u32 = 601u32;
pub const MM_MOTU_FLYER_MIDI_OUT_B: u32 = 602u32;
pub const MM_MOTU_MTPAV_MIDIIN_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIIN_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIIN_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIIN_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIIN_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIIN_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIIN_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIIN_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIIN_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIIN_SYNC: u32 = 900u32;
pub const MM_MOTU_MTPAV_MIDIOUT_1: u32 = 901u32;
pub const MM_MOTU_MTPAV_MIDIOUT_2: u32 = 902u32;
pub const MM_MOTU_MTPAV_MIDIOUT_3: u32 = 903u32;
pub const MM_MOTU_MTPAV_MIDIOUT_4: u32 = 904u32;
pub const MM_MOTU_MTPAV_MIDIOUT_5: u32 = 905u32;
pub const MM_MOTU_MTPAV_MIDIOUT_6: u32 = 906u32;
pub const MM_MOTU_MTPAV_MIDIOUT_7: u32 = 907u32;
pub const MM_MOTU_MTPAV_MIDIOUT_8: u32 = 908u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ADAT: u32 = 917u32;
pub const MM_MOTU_MTPAV_MIDIOUT_ALL: u32 = 900u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIIN_8: u32 = 916u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_1: u32 = 909u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_2: u32 = 910u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_3: u32 = 911u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_4: u32 = 912u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_5: u32 = 913u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_6: u32 = 914u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_7: u32 = 915u32;
pub const MM_MOTU_MTPAV_NET_MIDIOUT_8: u32 = 916u32;
pub const MM_MOTU_MTPII_MIDIIN_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIIN_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIIN_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIIN_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIIN_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIIN_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIIN_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIIN_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIIN_SYNC: u32 = 200u32;
pub const MM_MOTU_MTPII_MIDIOUT_1: u32 = 201u32;
pub const MM_MOTU_MTPII_MIDIOUT_2: u32 = 202u32;
pub const MM_MOTU_MTPII_MIDIOUT_3: u32 = 203u32;
pub const MM_MOTU_MTPII_MIDIOUT_4: u32 = 204u32;
pub const MM_MOTU_MTPII_MIDIOUT_5: u32 = 205u32;
pub const MM_MOTU_MTPII_MIDIOUT_6: u32 = 206u32;
pub const MM_MOTU_MTPII_MIDIOUT_7: u32 = 207u32;
pub const MM_MOTU_MTPII_MIDIOUT_8: u32 = 208u32;
pub const MM_MOTU_MTPII_MIDIOUT_ALL: u32 = 200u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIIN_8: u32 = 216u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_1: u32 = 209u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_2: u32 = 210u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_3: u32 = 211u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_4: u32 = 212u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_5: u32 = 213u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_6: u32 = 214u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_7: u32 = 215u32;
pub const MM_MOTU_MTPII_NET_MIDIOUT_8: u32 = 216u32;
pub const MM_MOTU_MTP_MIDIIN_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIIN_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIIN_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIIN_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIIN_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIIN_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIIN_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIIN_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_1: u32 = 101u32;
pub const MM_MOTU_MTP_MIDIOUT_2: u32 = 102u32;
pub const MM_MOTU_MTP_MIDIOUT_3: u32 = 103u32;
pub const MM_MOTU_MTP_MIDIOUT_4: u32 = 104u32;
pub const MM_MOTU_MTP_MIDIOUT_5: u32 = 105u32;
pub const MM_MOTU_MTP_MIDIOUT_6: u32 = 106u32;
pub const MM_MOTU_MTP_MIDIOUT_7: u32 = 107u32;
pub const MM_MOTU_MTP_MIDIOUT_8: u32 = 108u32;
pub const MM_MOTU_MTP_MIDIOUT_ALL: u32 = 100u32;
pub const MM_MOTU_MXN_MIDIIN_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIIN_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIIN_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIIN_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIIN_SYNC: u32 = 500u32;
pub const MM_MOTU_MXN_MIDIOUT_1: u32 = 501u32;
pub const MM_MOTU_MXN_MIDIOUT_2: u32 = 502u32;
pub const MM_MOTU_MXN_MIDIOUT_3: u32 = 503u32;
pub const MM_MOTU_MXN_MIDIOUT_4: u32 = 504u32;
pub const MM_MOTU_MXN_MIDIOUT_ALL: u32 = 500u32;
pub const MM_MOTU_MXPMPU_MIDIIN_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIIN_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIIN_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIIN_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIIN_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIIN_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIIN_SYNC: u32 = 400u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_1: u32 = 401u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_2: u32 = 402u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_3: u32 = 403u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_4: u32 = 404u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_5: u32 = 405u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_6: u32 = 406u32;
pub const MM_MOTU_MXPMPU_MIDIOUT_ALL: u32 = 400u32;
pub const MM_MOTU_MXPXT_MIDIIN_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIIN_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIIN_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIIN_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIIN_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIIN_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIIN_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIIN_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIIN_SYNC: u32 = 1000u32;
pub const MM_MOTU_MXPXT_MIDIOUT_1: u32 = 1001u32;
pub const MM_MOTU_MXPXT_MIDIOUT_2: u32 = 1002u32;
pub const MM_MOTU_MXPXT_MIDIOUT_3: u32 = 1003u32;
pub const MM_MOTU_MXPXT_MIDIOUT_4: u32 = 1004u32;
pub const MM_MOTU_MXPXT_MIDIOUT_5: u32 = 1005u32;
pub const MM_MOTU_MXPXT_MIDIOUT_6: u32 = 1006u32;
pub const MM_MOTU_MXPXT_MIDIOUT_7: u32 = 1007u32;
pub const MM_MOTU_MXPXT_MIDIOUT_8: u32 = 1008u32;
pub const MM_MOTU_MXPXT_MIDIOUT_ALL: u32 = 1000u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIIN_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_1: u32 = 301u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_2: u32 = 302u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_3: u32 = 303u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_4: u32 = 304u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_5: u32 = 305u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_6: u32 = 306u32;
pub const MM_MOTU_MXP_MIDIIN_MIDIOUT_ALL: u32 = 300u32;
pub const MM_MOTU_MXP_MIDIIN_SYNC: u32 = 300u32;
pub const MM_MOTU_PKX_MIDI_IN_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_IN_B: u32 = 702u32;
pub const MM_MOTU_PKX_MIDI_IN_SYNC: u32 = 700u32;
pub const MM_MOTU_PKX_MIDI_OUT_A: u32 = 701u32;
pub const MM_MOTU_PKX_MIDI_OUT_B: u32 = 702u32;
pub const MM_MPTUS: u32 = 95u32;
pub const MM_MPTUS_SPWAVEOUT: u32 = 1u32;
pub const MM_MPU401_MIDIIN: u32 = 11u32;
pub const MM_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_MSFT_ACM_G711: u32 = 37u32;
pub const MM_MSFT_ACM_GSM610: u32 = 36u32;
pub const MM_MSFT_ACM_IMAADPCM: u32 = 34u32;
pub const MM_MSFT_ACM_MSADPCM: u32 = 33u32;
pub const MM_MSFT_ACM_MSAUDIO1: u32 = 39u32;
pub const MM_MSFT_ACM_MSFILTER: u32 = 35u32;
pub const MM_MSFT_ACM_MSG723: u32 = 92u32;
pub const MM_MSFT_ACM_MSNAUDIO: u32 = 91u32;
pub const MM_MSFT_ACM_MSRT24: u32 = 93u32;
pub const MM_MSFT_ACM_PCM: u32 = 38u32;
pub const MM_MSFT_ACM_WMAUDIO: u32 = 39u32;
pub const MM_MSFT_ACM_WMAUDIO2: u32 = 101u32;
pub const MM_MSFT_GENERIC_AUX_CD: u32 = 30u32;
pub const MM_MSFT_GENERIC_AUX_LINE: u32 = 28u32;
pub const MM_MSFT_GENERIC_AUX_MIC: u32 = 29u32;
pub const MM_MSFT_GENERIC_MIDIIN: u32 = 25u32;
pub const MM_MSFT_GENERIC_MIDIOUT: u32 = 26u32;
pub const MM_MSFT_GENERIC_MIDISYNTH: u32 = 27u32;
pub const MM_MSFT_GENERIC_WAVEIN: u32 = 23u32;
pub const MM_MSFT_GENERIC_WAVEOUT: u32 = 24u32;
pub const MM_MSFT_MSACM: u32 = 32u32;
pub const MM_MSFT_MSOPL_SYNTH: u32 = 76u32;
pub const MM_MSFT_SB16_AUX_CD: u32 = 66u32;
pub const MM_MSFT_SB16_AUX_LINE: u32 = 65u32;
pub const MM_MSFT_SB16_MIDIIN: u32 = 62u32;
pub const MM_MSFT_SB16_MIDIOUT: u32 = 63u32;
pub const MM_MSFT_SB16_MIXER: u32 = 67u32;
pub const MM_MSFT_SB16_SYNTH: u32 = 64u32;
pub const MM_MSFT_SB16_WAVEIN: u32 = 60u32;
pub const MM_MSFT_SB16_WAVEOUT: u32 = 61u32;
pub const MM_MSFT_SBPRO_AUX_CD: u32 = 74u32;
pub const MM_MSFT_SBPRO_AUX_LINE: u32 = 73u32;
pub const MM_MSFT_SBPRO_MIDIIN: u32 = 70u32;
pub const MM_MSFT_SBPRO_MIDIOUT: u32 = 71u32;
pub const MM_MSFT_SBPRO_MIXER: u32 = 75u32;
pub const MM_MSFT_SBPRO_SYNTH: u32 = 72u32;
pub const MM_MSFT_SBPRO_WAVEIN: u32 = 68u32;
pub const MM_MSFT_SBPRO_WAVEOUT: u32 = 69u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEIN: u32 = 82u32;
pub const MM_MSFT_VMDMS_HANDSET_WAVEOUT: u32 = 83u32;
pub const MM_MSFT_VMDMS_LINE_WAVEIN: u32 = 80u32;
pub const MM_MSFT_VMDMS_LINE_WAVEOUT: u32 = 81u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEIN: u32 = 86u32;
pub const MM_MSFT_VMDMW_HANDSET_WAVEOUT: u32 = 87u32;
pub const MM_MSFT_VMDMW_LINE_WAVEIN: u32 = 84u32;
pub const MM_MSFT_VMDMW_LINE_WAVEOUT: u32 = 85u32;
pub const MM_MSFT_VMDMW_MIXER: u32 = 88u32;
pub const MM_MSFT_VMDM_GAME_WAVEIN: u32 = 90u32;
pub const MM_MSFT_VMDM_GAME_WAVEOUT: u32 = 89u32;
pub const MM_MSFT_WDMAUDIO_AUX: u32 = 105u32;
pub const MM_MSFT_WDMAUDIO_MIDIIN: u32 = 103u32;
pub const MM_MSFT_WDMAUDIO_MIDIOUT: u32 = 102u32;
pub const MM_MSFT_WDMAUDIO_MIXER: u32 = 104u32;
pub const MM_MSFT_WDMAUDIO_WAVEIN: u32 = 101u32;
pub const MM_MSFT_WDMAUDIO_WAVEOUT: u32 = 100u32;
pub const MM_MSFT_WSS_AUX: u32 = 21u32;
pub const MM_MSFT_WSS_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_MSFT_WSS_MIXER: u32 = 17u32;
pub const MM_MSFT_WSS_NT_AUX: u32 = 59u32;
pub const MM_MSFT_WSS_NT_FMSYNTH_STEREO: u32 = 57u32;
pub const MM_MSFT_WSS_NT_MIXER: u32 = 58u32;
pub const MM_MSFT_WSS_NT_WAVEIN: u32 = 55u32;
pub const MM_MSFT_WSS_NT_WAVEOUT: u32 = 56u32;
pub const MM_MSFT_WSS_OEM_AUX: u32 = 22u32;
pub const MM_MSFT_WSS_OEM_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_MSFT_WSS_OEM_MIXER: u32 = 31u32;
pub const MM_MSFT_WSS_OEM_WAVEIN: u32 = 18u32;
pub const MM_MSFT_WSS_OEM_WAVEOUT: u32 = 19u32;
pub const MM_MSFT_WSS_WAVEIN: u32 = 14u32;
pub const MM_MSFT_WSS_WAVEOUT: u32 = 15u32;
pub const MM_MWM: u32 = 209u32;
pub const MM_NCR: u32 = 62u32;
pub const MM_NCR_BA_AUX: u32 = 4u32;
pub const MM_NCR_BA_MIXER: u32 = 5u32;
pub const MM_NCR_BA_SYNTH: u32 = 3u32;
pub const MM_NCR_BA_WAVEIN: u32 = 1u32;
pub const MM_NCR_BA_WAVEOUT: u32 = 2u32;
pub const MM_NEC: u32 = 26u32;
pub const MM_NEC_26_SYNTH: u32 = 9u32;
pub const MM_NEC_73_86_SYNTH: u32 = 5u32;
pub const MM_NEC_73_86_WAVEIN: u32 = 7u32;
pub const MM_NEC_73_86_WAVEOUT: u32 = 6u32;
pub const MM_NEC_JOYSTICK: u32 = 12u32;
pub const MM_NEC_MPU401_MIDIIN: u32 = 11u32;
pub const MM_NEC_MPU401_MIDIOUT: u32 = 10u32;
pub const MM_NEOMAGIC: u32 = 176u32;
pub const MM_NEOMAGIC_AUX: u32 = 6u32;
pub const MM_NEOMAGIC_MIDIIN: u32 = 5u32;
pub const MM_NEOMAGIC_MIDIOUT: u32 = 4u32;
pub const MM_NEOMAGIC_MW3DX_AUX: u32 = 17u32;
pub const MM_NEOMAGIC_MW3DX_FMSYNTH: u32 = 14u32;
pub const MM_NEOMAGIC_MW3DX_GMSYNTH: u32 = 15u32;
pub const MM_NEOMAGIC_MW3DX_MIDIIN: u32 = 13u32;
pub const MM_NEOMAGIC_MW3DX_MIDIOUT: u32 = 12u32;
pub const MM_NEOMAGIC_MW3DX_MIXER: u32 = 16u32;
pub const MM_NEOMAGIC_MW3DX_WAVEIN: u32 = 11u32;
pub const MM_NEOMAGIC_MW3DX_WAVEOUT: u32 = 10u32;
pub const MM_NEOMAGIC_MWAVE_AUX: u32 = 25u32;
pub const MM_NEOMAGIC_MWAVE_MIDIIN: u32 = 23u32;
pub const MM_NEOMAGIC_MWAVE_MIDIOUT: u32 = 22u32;
pub const MM_NEOMAGIC_MWAVE_MIXER: u32 = 24u32;
pub const MM_NEOMAGIC_MWAVE_WAVEIN: u32 = 21u32;
pub const MM_NEOMAGIC_MWAVE_WAVEOUT: u32 = 20u32;
pub const MM_NEOMAGIC_SYNTH: u32 = 1u32;
pub const MM_NEOMAGIC_WAVEIN: u32 = 3u32;
pub const MM_NEOMAGIC_WAVEOUT: u32 = 2u32;
pub const MM_NETSCAPE: u32 = 166u32;
pub const MM_NETXL: u32 = 8u32;
pub const MM_NETXL_XLVIDEO: u32 = 1u32;
pub const MM_NEWMEDIA: u32 = 86u32;
pub const MM_NEWMEDIA_WAVJAMMER: u32 = 1u32;
pub const MM_NMP: u32 = 195u32;
pub const MM_NMP_ACM_AMR: u32 = 10u32;
pub const MM_NMP_CCP_WAVEIN: u32 = 1u32;
pub const MM_NMP_CCP_WAVEOUT: u32 = 2u32;
pub const MM_NMS: u32 = 87u32;
pub const MM_NOGATECH: u32 = 75u32;
pub const MM_NORRIS: u32 = 150u32;
pub const MM_NORRIS_VOICELINK: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEIN: u32 = 1u32;
pub const MM_NORTEL_MPXAC_WAVEOUT: u32 = 2u32;
pub const MM_NORTHERN_TELECOM: u32 = 115u32;
pub const MM_NVIDIA: u32 = 127u32;
pub const MM_NVIDIA_AUX: u32 = 7u32;
pub const MM_NVIDIA_GAMEPORT: u32 = 5u32;
pub const MM_NVIDIA_MIDIIN: u32 = 4u32;
pub const MM_NVIDIA_MIDIOUT: u32 = 3u32;
pub const MM_NVIDIA_MIXER: u32 = 6u32;
pub const MM_NVIDIA_WAVEIN: u32 = 2u32;
pub const MM_NVIDIA_WAVEOUT: u32 = 1u32;
pub const MM_OKI: u32 = 79u32;
pub const MM_OKSORI: u32 = 128u32;
pub const MM_OKSORI_BASE: u32 = 0u32;
pub const MM_OKSORI_EXT_MIC1: u32 = 15u32;
pub const MM_OKSORI_EXT_MIC2: u32 = 16u32;
pub const MM_OKSORI_FM_OPL4: u32 = 5u32;
pub const MM_OKSORI_MIDIIN: u32 = 18u32;
pub const MM_OKSORI_MIDIOUT: u32 = 17u32;
pub const MM_OKSORI_MIX_AUX1: u32 = 13u32;
pub const MM_OKSORI_MIX_CD: u32 = 10u32;
pub const MM_OKSORI_MIX_ECHO: u32 = 12u32;
pub const MM_OKSORI_MIX_FM: u32 = 8u32;
pub const MM_OKSORI_MIX_LINE: u32 = 9u32;
pub const MM_OKSORI_MIX_LINE1: u32 = 14u32;
pub const MM_OKSORI_MIX_MASTER: u32 = 6u32;
pub const MM_OKSORI_MIX_MIC: u32 = 11u32;
pub const MM_OKSORI_MIX_WAVE: u32 = 7u32;
pub const MM_OKSORI_MPEG_CDVISION: u32 = 19u32;
pub const MM_OKSORI_OSR16_WAVEIN: u32 = 4u32;
pub const MM_OKSORI_OSR16_WAVEOUT: u32 = 3u32;
pub const MM_OKSORI_OSR8_WAVEIN: u32 = 2u32;
pub const MM_OKSORI_OSR8_WAVEOUT: u32 = 1u32;
pub const MM_OLIVETTI: u32 = 81u32;
pub const MM_OLIVETTI_ACM_ADPCM: u32 = 10u32;
pub const MM_OLIVETTI_ACM_CELP: u32 = 11u32;
pub const MM_OLIVETTI_ACM_GSM: u32 = 9u32;
pub const MM_OLIVETTI_ACM_OPR: u32 = 13u32;
pub const MM_OLIVETTI_ACM_SBC: u32 = 12u32;
pub const MM_OLIVETTI_AUX: u32 = 4u32;
pub const MM_OLIVETTI_JOYSTICK: u32 = 8u32;
pub const MM_OLIVETTI_MIDIIN: u32 = 5u32;
pub const MM_OLIVETTI_MIDIOUT: u32 = 6u32;
pub const MM_OLIVETTI_MIXER: u32 = 3u32;
pub const MM_OLIVETTI_SYNTH: u32 = 7u32;
pub const MM_OLIVETTI_WAVEIN: u32 = 1u32;
pub const MM_OLIVETTI_WAVEOUT: u32 = 2u32;
pub const MM_ONLIVE: u32 = 200u32;
pub const MM_ONLIVE_MPCODEC: u32 = 1u32;
pub const MM_OPCODE: u32 = 113u32;
pub const MM_OPTI: u32 = 90u32;
pub const MM_OPTI_M16_AUX: u32 = 7u32;
pub const MM_OPTI_M16_FMSYNTH_STEREO: u32 = 1u32;
pub const MM_OPTI_M16_MIDIIN: u32 = 2u32;
pub const MM_OPTI_M16_MIDIOUT: u32 = 3u32;
pub const MM_OPTI_M16_MIXER: u32 = 6u32;
pub const MM_OPTI_M16_WAVEIN: u32 = 4u32;
pub const MM_OPTI_M16_WAVEOUT: u32 = 5u32;
pub const MM_OPTI_M32_AUX: u32 = 38u32;
pub const MM_OPTI_M32_MIDIIN: u32 = 34u32;
pub const MM_OPTI_M32_MIDIOUT: u32 = 35u32;
pub const MM_OPTI_M32_MIXER: u32 = 37u32;
pub const MM_OPTI_M32_SYNTH_STEREO: u32 = 36u32;
pub const MM_OPTI_M32_WAVEIN: u32 = 32u32;
pub const MM_OPTI_M32_WAVEOUT: u32 = 33u32;
pub const MM_OPTI_P16_AUX: u32 = 22u32;
pub const MM_OPTI_P16_FMSYNTH_STEREO: u32 = 16u32;
pub const MM_OPTI_P16_MIDIIN: u32 = 17u32;
pub const MM_OPTI_P16_MIDIOUT: u32 = 18u32;
pub const MM_OPTI_P16_MIXER: u32 = 21u32;
pub const MM_OPTI_P16_WAVEIN: u32 = 19u32;
pub const MM_OPTI_P16_WAVEOUT: u32 = 20u32;
pub const MM_OPUS1208_AUX: u32 = 135u32;
pub const MM_OPUS1208_MIXER: u32 = 134u32;
pub const MM_OPUS1208_SYNTH: u32 = 131u32;
pub const MM_OPUS1208_WAVEIN: u32 = 133u32;
pub const MM_OPUS1208_WAVEOUT: u32 = 132u32;
pub const MM_OPUS1216_AUX: u32 = 151u32;
pub const MM_OPUS1216_MIDIIN: u32 = 146u32;
pub const MM_OPUS1216_MIDIOUT: u32 = 145u32;
pub const MM_OPUS1216_MIXER: u32 = 150u32;
pub const MM_OPUS1216_SYNTH: u32 = 147u32;
pub const MM_OPUS1216_WAVEIN: u32 = 149u32;
pub const MM_OPUS1216_WAVEOUT: u32 = 148u32;
pub const MM_OPUS401_MIDIIN: u32 = 130u32;
pub const MM_OPUS401_MIDIOUT: u32 = 129u32;
pub const MM_OSITECH: u32 = 103u32;
pub const MM_OSITECH_TRUMPCARD: u32 = 1u32;
pub const MM_OSPREY: u32 = 140u32;
pub const MM_OSPREY_1000WAVEIN: u32 = 1u32;
pub const MM_OSPREY_1000WAVEOUT: u32 = 2u32;
pub const MM_OTI: u32 = 180u32;
pub const MM_OTI_611MIDIN: u32 = 18u32;
pub const MM_OTI_611MIDIOUT: u32 = 19u32;
pub const MM_OTI_611MIXER: u32 = 7u32;
pub const MM_OTI_611WAVEIN: u32 = 5u32;
pub const MM_OTI_611WAVEOUT: u32 = 6u32;
pub const MM_PACIFICRESEARCH: u32 = 210u32;
pub const MM_PCSPEAKER_WAVEOUT: u32 = 13u32;
pub const MM_PC_JOYSTICK: u32 = 12u32;
pub const MM_PHILIPS_ACM_LPCBB: u32 = 1u32;
pub const MM_PHILIPS_SPEECH_PROCESSING: u32 = 7u32;
pub const MM_PHONET: u32 = 203u32;
pub const MM_PHONET_PP_MIXER: u32 = 3u32;
pub const MM_PHONET_PP_WAVEIN: u32 = 2u32;
pub const MM_PHONET_PP_WAVEOUT: u32 = 1u32;
pub const MM_PICTURETEL: u32 = 138u32;
pub const MM_PID_UNMAPPED: u32 = 65535u32;
pub const MM_PINNACLE: u32 = 218u32;
pub const MM_PRAGMATRAX: u32 = 5u32;
pub const MM_PRECEPT: u32 = 153u32;
pub const MM_PROAUD_16_AUX: u32 = 103u32;
pub const MM_PROAUD_16_MIDIIN: u32 = 98u32;
pub const MM_PROAUD_16_MIDIOUT: u32 = 97u32;
pub const MM_PROAUD_16_MIXER: u32 = 102u32;
pub const MM_PROAUD_16_SYNTH: u32 = 99u32;
pub const MM_PROAUD_16_WAVEIN: u32 = 101u32;
pub const MM_PROAUD_16_WAVEOUT: u32 = 100u32;
pub const MM_PROAUD_AUX: u32 = 23u32;
pub const MM_PROAUD_MIDIIN: u32 = 18u32;
pub const MM_PROAUD_MIDIOUT: u32 = 17u32;
pub const MM_PROAUD_MIXER: u32 = 22u32;
pub const MM_PROAUD_PLUS_AUX: u32 = 87u32;
pub const MM_PROAUD_PLUS_MIDIIN: u32 = 82u32;
pub const MM_PROAUD_PLUS_MIDIOUT: u32 = 81u32;
pub const MM_PROAUD_PLUS_MIXER: u32 = 86u32;
pub const MM_PROAUD_PLUS_SYNTH: u32 = 83u32;
pub const MM_PROAUD_PLUS_WAVEIN: u32 = 85u32;
pub const MM_PROAUD_PLUS_WAVEOUT: u32 = 84u32;
pub const MM_PROAUD_SYNTH: u32 = 19u32;
pub const MM_PROAUD_WAVEIN: u32 = 21u32;
pub const MM_PROAUD_WAVEOUT: u32 = 20u32;
pub const MM_QCIAR: u32 = 98u32;
pub const MM_QDESIGN: u32 = 194u32;
pub const MM_QDESIGN_ACM_MPEG: u32 = 1u32;
pub const MM_QDESIGN_ACM_QDESIGN_MUSIC: u32 = 2u32;
pub const MM_QTEAM: u32 = 169u32;
pub const MM_QUALCOMM: u32 = 215u32;
pub const MM_QUANTUM3D: u32 = 17u32;
pub const MM_QUARTERDECK: u32 = 134u32;
pub const MM_QUARTERDECK_LHWAVEIN: u32 = 0u32;
pub const MM_QUARTERDECK_LHWAVEOUT: u32 = 1u32;
pub const MM_QUICKAUDIO: u32 = 255u32;
pub const MM_QUICKAUDIO_MAXIMIDI: u32 = 2u32;
pub const MM_QUICKAUDIO_MINIMIDI: u32 = 1u32;
pub const MM_QUICKNET: u32 = 173u32;
pub const MM_QUICKNET_PJWAVEIN: u32 = 1u32;
pub const MM_QUICKNET_PJWAVEOUT: u32 = 2u32;
pub const MM_RADIUS: u32 = 110u32;
pub const MM_RHETOREX: u32 = 120u32;
pub const MM_RHETOREX_WAVEIN: u32 = 1u32;
pub const MM_RHETOREX_WAVEOUT: u32 = 2u32;
pub const MM_RICHMOND: u32 = 257u32;
pub const MM_ROCKWELL: u32 = 111u32;
pub const MM_ROLAND: u32 = 24u32;
pub const MM_ROLAND_MPU401_MIDIIN: u32 = 16u32;
pub const MM_ROLAND_MPU401_MIDIOUT: u32 = 15u32;
pub const MM_ROLAND_RAP10_MIDIIN: u32 = 11u32;
pub const MM_ROLAND_RAP10_MIDIOUT: u32 = 10u32;
pub const MM_ROLAND_RAP10_SYNTH: u32 = 12u32;
pub const MM_ROLAND_RAP10_WAVEIN: u32 = 14u32;
pub const MM_ROLAND_RAP10_WAVEOUT: u32 = 13u32;
pub const MM_ROLAND_SC7_MIDIIN: u32 = 22u32;
pub const MM_ROLAND_SC7_MIDIOUT: u32 = 21u32;
pub const MM_ROLAND_SCP_AUX: u32 = 48u32;
pub const MM_ROLAND_SCP_MIDIIN: u32 = 39u32;
pub const MM_ROLAND_SCP_MIDIOUT: u32 = 38u32;
pub const MM_ROLAND_SCP_MIXER: u32 = 42u32;
pub const MM_ROLAND_SCP_WAVEIN: u32 = 41u32;
pub const MM_ROLAND_SCP_WAVEOUT: u32 = 40u32;
pub const MM_ROLAND_SERIAL_MIDIIN: u32 = 24u32;
pub const MM_ROLAND_SERIAL_MIDIOUT: u32 = 23u32;
pub const MM_ROLAND_SMPU_MIDIINA: u32 = 19u32;
pub const MM_ROLAND_SMPU_MIDIINB: u32 = 20u32;
pub const MM_ROLAND_SMPU_MIDIOUTA: u32 = 17u32;
pub const MM_ROLAND_SMPU_MIDIOUTB: u32 = 18u32;
pub const MM_RZS: u32 = 216u32;
pub const MM_RZS_ACM_TUBGSM: u32 = 1u32;
pub const MM_S3: u32 = 164u32;
pub const MM_S3_AUX: u32 = 7u32;
pub const MM_S3_FMSYNTH: u32 = 5u32;
pub const MM_S3_MIDIIN: u32 = 4u32;
pub const MM_S3_MIDIOUT: u32 = 3u32;
pub const MM_S3_MIXER: u32 = 6u32;
pub const MM_S3_WAVEIN: u32 = 2u32;
pub const MM_S3_WAVEOUT: u32 = 1u32;
pub const MM_SANYO: u32 = 72u32;
pub const MM_SANYO_ACM_LD_ADPCM: u32 = 1u32;
pub const MM_SCALACS: u32 = 54u32;
pub const MM_SEERSYS: u32 = 137u32;
pub const MM_SEERSYS_REALITY: u32 = 6u32;
pub const MM_SEERSYS_SEERMIX: u32 = 3u32;
pub const MM_SEERSYS_SEERSYNTH: u32 = 1u32;
pub const MM_SEERSYS_SEERWAVE: u32 = 2u32;
pub const MM_SEERSYS_WAVESYNTH: u32 = 4u32;
pub const MM_SEERSYS_WAVESYNTH_WG: u32 = 5u32;
pub const MM_SELSIUS_SYSTEMS: u32 = 234u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEIN: u32 = 2u32;
pub const MM_SELSIUS_SYSTEMS_RTPWAVEOUT: u32 = 1u32;
pub const MM_SGI: u32 = 237u32;
pub const MM_SGI_320_MIXER: u32 = 3u32;
pub const MM_SGI_320_WAVEIN: u32 = 1u32;
pub const MM_SGI_320_WAVEOUT: u32 = 2u32;
pub const MM_SGI_540_MIXER: u32 = 6u32;
pub const MM_SGI_540_WAVEIN: u32 = 4u32;
pub const MM_SGI_540_WAVEOUT: u32 = 5u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEIN: u32 = 19u32;
pub const MM_SGI_RAD_ADAT8CHAN_WAVEOUT: u32 = 32u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEIN: u32 = 7u32;
pub const MM_SGI_RAD_ADATMONO1_WAVEOUT: u32 = 20u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEIN: u32 = 8u32;
pub const MM_SGI_RAD_ADATMONO2_WAVEOUT: u32 = 21u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEIN: u32 = 9u32;
pub const MM_SGI_RAD_ADATMONO3_WAVEOUT: u32 = 22u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEIN: u32 = 10u32;
pub const MM_SGI_RAD_ADATMONO4_WAVEOUT: u32 = 23u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEIN: u32 = 11u32;
pub const MM_SGI_RAD_ADATMONO5_WAVEOUT: u32 = 24u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEIN: u32 = 12u32;
pub const MM_SGI_RAD_ADATMONO6_WAVEOUT: u32 = 25u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEIN: u32 = 13u32;
pub const MM_SGI_RAD_ADATMONO7_WAVEOUT: u32 = 26u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEIN: u32 = 14u32;
pub const MM_SGI_RAD_ADATMONO8_WAVEOUT: u32 = 27u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEIN: u32 = 15u32;
pub const MM_SGI_RAD_ADATSTEREO12_WAVEOUT: u32 = 28u32;
pub const MM_SGI_RAD_ADATSTEREO32_WAVEOUT: u32 = 29u32;
pub const MM_SGI_RAD_ADATSTEREO34_WAVEIN: u32 = 16u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEIN: u32 = 17u32;
pub const MM_SGI_RAD_ADATSTEREO56_WAVEOUT: u32 = 30u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEIN: u32 = 18u32;
pub const MM_SGI_RAD_ADATSTEREO78_WAVEOUT: u32 = 31u32;
pub const MM_SGI_RAD_AESMONO1_WAVEIN: u32 = 33u32;
pub const MM_SGI_RAD_AESMONO1_WAVEOUT: u32 = 36u32;
pub const MM_SGI_RAD_AESMONO2_WAVEIN: u32 = 34u32;
pub const MM_SGI_RAD_AESMONO2_WAVEOUT: u32 = 37u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEIN: u32 = 35u32;
pub const MM_SGI_RAD_AESSTEREO_WAVEOUT: u32 = 38u32;
pub const MM_SHARP: u32 = 183u32;
pub const MM_SHARP_MDC_AUX: u32 = 6u32;
pub const MM_SHARP_MDC_AUX_BASS: u32 = 101u32;
pub const MM_SHARP_MDC_AUX_CHR: u32 = 109u32;
pub const MM_SHARP_MDC_AUX_MASTER: u32 = 100u32;
pub const MM_SHARP_MDC_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_SHARP_MDC_AUX_RVB: u32 = 108u32;
pub const MM_SHARP_MDC_AUX_TREBLE: u32 = 102u32;
pub const MM_SHARP_MDC_AUX_VOL: u32 = 107u32;
pub const MM_SHARP_MDC_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_SHARP_MDC_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_SHARP_MDC_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_SHARP_MDC_MIDI_IN: u32 = 2u32;
pub const MM_SHARP_MDC_MIDI_OUT: u32 = 3u32;
pub const MM_SHARP_MDC_MIDI_SYNTH: u32 = 1u32;
pub const MM_SHARP_MDC_MIXER: u32 = 10u32;
pub const MM_SHARP_MDC_WAVE_IN: u32 = 4u32;
pub const MM_SHARP_MDC_WAVE_OUT: u32 = 5u32;
pub const MM_SICRESOURCE: u32 = 175u32;
pub const MM_SICRESOURCE_SSO3D: u32 = 2u32;
pub const MM_SICRESOURCE_SSOW3DI: u32 = 3u32;
pub const MM_SIEMENS_SBC: u32 = 201u32;
pub const MM_SIERRA: u32 = 40u32;
pub const MM_SIERRA_ARIA_AUX: u32 = 25u32;
pub const MM_SIERRA_ARIA_AUX2: u32 = 32u32;
pub const MM_SIERRA_ARIA_MIDIIN: u32 = 21u32;
pub const MM_SIERRA_ARIA_MIDIOUT: u32 = 20u32;
pub const MM_SIERRA_ARIA_SYNTH: u32 = 22u32;
pub const MM_SIERRA_ARIA_WAVEIN: u32 = 24u32;
pub const MM_SIERRA_ARIA_WAVEOUT: u32 = 23u32;
pub const MM_SIERRA_QUARTET_AUX_CD: u32 = 85u32;
pub const MM_SIERRA_QUARTET_AUX_LINE: u32 = 86u32;
pub const MM_SIERRA_QUARTET_AUX_MODEM: u32 = 87u32;
pub const MM_SIERRA_QUARTET_MIDIIN: u32 = 82u32;
pub const MM_SIERRA_QUARTET_MIDIOUT: u32 = 83u32;
pub const MM_SIERRA_QUARTET_MIXER: u32 = 88u32;
pub const MM_SIERRA_QUARTET_SYNTH: u32 = 84u32;
pub const MM_SIERRA_QUARTET_WAVEIN: u32 = 80u32;
pub const MM_SIERRA_QUARTET_WAVEOUT: u32 = 81u32;
pub const MM_SILICONSOFT: u32 = 69u32;
pub const MM_SILICONSOFT_SC1_WAVEIN: u32 = 1u32;
pub const MM_SILICONSOFT_SC1_WAVEOUT: u32 = 2u32;
pub const MM_SILICONSOFT_SC2_WAVEIN: u32 = 3u32;
pub const MM_SILICONSOFT_SC2_WAVEOUT: u32 = 4u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEIN: u32 = 6u32;
pub const MM_SILICONSOFT_SOUNDJR2PR_WAVEOUT: u32 = 7u32;
pub const MM_SILICONSOFT_SOUNDJR2_WAVEOUT: u32 = 5u32;
pub const MM_SILICONSOFT_SOUNDJR3_WAVEOUT: u32 = 8u32;
pub const MM_SIPROLAB: u32 = 211u32;
pub const MM_SIPROLAB_ACELPNET: u32 = 1u32;
pub const MM_SNDBLST_MIDIIN: u32 = 4u32;
pub const MM_SNDBLST_MIDIOUT: u32 = 3u32;
pub const MM_SNDBLST_SYNTH: u32 = 5u32;
pub const MM_SNDBLST_WAVEIN: u32 = 7u32;
pub const MM_SNDBLST_WAVEOUT: u32 = 6u32;
pub const MM_SNI: u32 = 18u32;
pub const MM_SNI_ACM_G721: u32 = 1u32;
pub const MM_SOFTLAB_NSK: u32 = 228u32;
pub const MM_SOFTLAB_NSK_FRW_AUX: u32 = 4u32;
pub const MM_SOFTLAB_NSK_FRW_MIXER: u32 = 3u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEIN: u32 = 1u32;
pub const MM_SOFTLAB_NSK_FRW_WAVEOUT: u32 = 2u32;
pub const MM_SOFTSOUND: u32 = 149u32;
pub const MM_SOFTSOUND_CODEC: u32 = 1u32;
pub const MM_SONICFOUNDRY: u32 = 66u32;
pub const MM_SONORUS: u32 = 230u32;
pub const MM_SONORUS_STUDIO: u32 = 1u32;
pub const MM_SONY: u32 = 245u32;
pub const MM_SONY_ACM_SCX: u32 = 1u32;
pub const MM_SORVIS: u32 = 187u32;
pub const MM_SOUNDESIGNS: u32 = 142u32;
pub const MM_SOUNDESIGNS_WAVEIN: u32 = 1u32;
pub const MM_SOUNDESIGNS_WAVEOUT: u32 = 2u32;
pub const MM_SOUNDSCAPE_AUX: u32 = 24u32;
pub const MM_SOUNDSCAPE_MIDIIN: u32 = 21u32;
pub const MM_SOUNDSCAPE_MIDIOUT: u32 = 20u32;
pub const MM_SOUNDSCAPE_MIXER: u32 = 23u32;
pub const MM_SOUNDSCAPE_SYNTH: u32 = 22u32;
pub const MM_SOUNDSCAPE_WAVEIN: u32 = 19u32;
pub const MM_SOUNDSCAPE_WAVEOUT: u32 = 17u32;
pub const MM_SOUNDSCAPE_WAVEOUT_AUX: u32 = 18u32;
pub const MM_SOUNDSPACE: u32 = 167u32;
pub const MM_SPECTRUM_PRODUCTIONS: u32 = 213u32;
pub const MM_SPECTRUM_SIGNAL_PROCESSING: u32 = 144u32;
pub const MM_SPEECHCOMP: u32 = 76u32;
pub const MM_SPLASH_STUDIOS: u32 = 133u32;
pub const MM_SSP_SNDFESAUX: u32 = 7u32;
pub const MM_SSP_SNDFESMIDIIN: u32 = 3u32;
pub const MM_SSP_SNDFESMIDIOUT: u32 = 4u32;
pub const MM_SSP_SNDFESMIX: u32 = 6u32;
pub const MM_SSP_SNDFESSYNTH: u32 = 5u32;
pub const MM_SSP_SNDFESWAVEIN: u32 = 1u32;
pub const MM_SSP_SNDFESWAVEOUT: u32 = 2u32;
pub const MM_STREAM_CLOSE: u32 = 981u32;
pub const MM_STREAM_DONE: u32 = 982u32;
pub const MM_STREAM_ERROR: u32 = 983u32;
pub const MM_STREAM_OPEN: u32 = 980u32;
pub const MM_STUDER: u32 = 171u32;
pub const MM_STUDIO_16_AUX: u32 = 103u32;
pub const MM_STUDIO_16_MIDIIN: u32 = 98u32;
pub const MM_STUDIO_16_MIDIOUT: u32 = 97u32;
pub const MM_STUDIO_16_MIXER: u32 = 102u32;
pub const MM_STUDIO_16_SYNTH: u32 = 99u32;
pub const MM_STUDIO_16_WAVEIN: u32 = 101u32;
pub const MM_STUDIO_16_WAVEOUT: u32 = 100u32;
pub const MM_ST_MICROELECTRONICS: u32 = 265u32;
pub const MM_SUNCOM: u32 = 186u32;
pub const MM_SUPERMAC: u32 = 73u32;
pub const MM_SYDEC_NV: u32 = 248u32;
pub const MM_SYDEC_NV_WAVEIN: u32 = 1u32;
pub const MM_SYDEC_NV_WAVEOUT: u32 = 2u32;
pub const MM_TANDY: u32 = 29u32;
pub const MM_TANDY_PSSJWAVEIN: u32 = 9u32;
pub const MM_TANDY_PSSJWAVEOUT: u32 = 10u32;
pub const MM_TANDY_SENS_MMAMIDIIN: u32 = 6u32;
pub const MM_TANDY_SENS_MMAMIDIOUT: u32 = 7u32;
pub const MM_TANDY_SENS_MMAWAVEIN: u32 = 4u32;
pub const MM_TANDY_SENS_MMAWAVEOUT: u32 = 5u32;
pub const MM_TANDY_SENS_VISWAVEOUT: u32 = 8u32;
pub const MM_TANDY_VISBIOSSYNTH: u32 = 3u32;
pub const MM_TANDY_VISWAVEIN: u32 = 1u32;
pub const MM_TANDY_VISWAVEOUT: u32 = 2u32;
pub const MM_TBS_TROPEZ_AUX1: u32 = 39u32;
pub const MM_TBS_TROPEZ_AUX2: u32 = 40u32;
pub const MM_TBS_TROPEZ_LINE: u32 = 41u32;
pub const MM_TBS_TROPEZ_WAVEIN: u32 = 37u32;
pub const MM_TBS_TROPEZ_WAVEOUT: u32 = 38u32;
pub const MM_TDK: u32 = 135u32;
pub const MM_TDK_MW_AUX: u32 = 6u32;
pub const MM_TDK_MW_AUX_BASS: u32 = 101u32;
pub const MM_TDK_MW_AUX_CHR: u32 = 109u32;
pub const MM_TDK_MW_AUX_MASTER: u32 = 100u32;
pub const MM_TDK_MW_AUX_MIDI_VOL: u32 = 103u32;
pub const MM_TDK_MW_AUX_RVB: u32 = 108u32;
pub const MM_TDK_MW_AUX_TREBLE: u32 = 102u32;
pub const MM_TDK_MW_AUX_VOL: u32 = 107u32;
pub const MM_TDK_MW_AUX_WAVE_CHR: u32 = 106u32;
pub const MM_TDK_MW_AUX_WAVE_RVB: u32 = 105u32;
pub const MM_TDK_MW_AUX_WAVE_VOL: u32 = 104u32;
pub const MM_TDK_MW_MIDI_IN: u32 = 2u32;
pub const MM_TDK_MW_MIDI_OUT: u32 = 3u32;
pub const MM_TDK_MW_MIDI_SYNTH: u32 = 1u32;
pub const MM_TDK_MW_MIXER: u32 = 10u32;
pub const MM_TDK_MW_WAVE_IN: u32 = 4u32;
pub const MM_TDK_MW_WAVE_OUT: u32 = 5u32;
pub const MM_TELEKOL: u32 = 264u32;
pub const MM_TELEKOL_WAVEIN: u32 = 2u32;
pub const MM_TELEKOL_WAVEOUT: u32 = 1u32;
pub const MM_TERALOGIC: u32 = 202u32;
pub const MM_TERRATEC: u32 = 70u32;
pub const MM_THUNDER_AUX: u32 = 39u32;
pub const MM_THUNDER_SYNTH: u32 = 35u32;
pub const MM_THUNDER_WAVEIN: u32 = 37u32;
pub const MM_THUNDER_WAVEOUT: u32 = 36u32;
pub const MM_TPORT_SYNTH: u32 = 67u32;
pub const MM_TPORT_WAVEIN: u32 = 66u32;
pub const MM_TPORT_WAVEOUT: u32 = 65u32;
pub const MM_TRUEVISION: u32 = 51u32;
pub const MM_TRUEVISION_WAVEIN1: u32 = 1u32;
pub const MM_TRUEVISION_WAVEOUT1: u32 = 2u32;
pub const MM_TTEWS_AUX: u32 = 9u32;
pub const MM_TTEWS_MIDIIN: u32 = 3u32;
pub const MM_TTEWS_MIDIMONITOR: u32 = 6u32;
pub const MM_TTEWS_MIDIOUT: u32 = 4u32;
pub const MM_TTEWS_MIDISYNTH: u32 = 5u32;
pub const MM_TTEWS_MIXER: u32 = 10u32;
pub const MM_TTEWS_VMIDIIN: u32 = 7u32;
pub const MM_TTEWS_VMIDIOUT: u32 = 8u32;
pub const MM_TTEWS_WAVEIN: u32 = 1u32;
pub const MM_TTEWS_WAVEOUT: u32 = 2u32;
pub const MM_TURTLE_BEACH: u32 = 21u32;
pub const MM_UHER_INFORMATIC: u32 = 247u32;
pub const MM_UH_ACM_ADPCM: u32 = 1u32;
pub const MM_UNISYS: u32 = 223u32;
pub const MM_UNISYS_ACM_NAP: u32 = 1u32;
pub const MM_UNMAPPED: u32 = 65535u32;
pub const MM_VAL: u32 = 35u32;
pub const MM_VAL_MICROKEY_AP_WAVEIN: u32 = 1u32;
pub const MM_VAL_MICROKEY_AP_WAVEOUT: u32 = 2u32;
pub const MM_VANKOEVERING: u32 = 168u32;
pub const MM_VIA: u32 = 250u32;
pub const MM_VIA_AUX: u32 = 4u32;
pub const MM_VIA_MIXER: u32 = 3u32;
pub const MM_VIA_MPU401_MIDIIN: u32 = 6u32;
pub const MM_VIA_MPU401_MIDIOUT: u32 = 5u32;
pub const MM_VIA_SWFM_SYNTH: u32 = 7u32;
pub const MM_VIA_WAVEIN: u32 = 2u32;
pub const MM_VIA_WAVEOUT: u32 = 1u32;
pub const MM_VIA_WDM_MIXER: u32 = 10u32;
pub const MM_VIA_WDM_MPU401_MIDIIN: u32 = 12u32;
pub const MM_VIA_WDM_MPU401_MIDIOUT: u32 = 11u32;
pub const MM_VIA_WDM_WAVEIN: u32 = 9u32;
pub const MM_VIA_WDM_WAVEOUT: u32 = 8u32;
pub const MM_VIDEOLOGIC: u32 = 53u32;
pub const MM_VIDEOLOGIC_MSWAVEIN: u32 = 1u32;
pub const MM_VIDEOLOGIC_MSWAVEOUT: u32 = 2u32;
pub const MM_VIENNASYS: u32 = 157u32;
pub const MM_VIENNASYS_TSP_WAVE_DRIVER: u32 = 1u32;
pub const MM_VIONA: u32 = 161u32;
pub const MM_VIONAQVINPCI_WAVEOUT: u32 = 3u32;
pub const MM_VIONA_BUSTER_MIXER: u32 = 4u32;
pub const MM_VIONA_CINEMASTER_MIXER: u32 = 5u32;
pub const MM_VIONA_CONCERTO_MIXER: u32 = 6u32;
pub const MM_VIONA_QVINPCI_MIXER: u32 = 1u32;
pub const MM_VIONA_QVINPCI_WAVEIN: u32 = 2u32;
pub const MM_VIRTUALMUSIC: u32 = 205u32;
pub const MM_VITEC: u32 = 67u32;
pub const MM_VITEC_VMAKER: u32 = 1u32;
pub const MM_VITEC_VMPRO: u32 = 2u32;
pub const MM_VIVO: u32 = 182u32;
pub const MM_VIVO_AUDIO_CODEC: u32 = 1u32;
pub const MM_VKC_MPU401_MIDIIN: u32 = 256u32;
pub const MM_VKC_MPU401_MIDIOUT: u32 = 512u32;
pub const MM_VKC_SERIAL_MIDIIN: u32 = 257u32;
pub const MM_VKC_SERIAL_MIDIOUT: u32 = 513u32;
pub const MM_VOCALTEC: u32 = 23u32;
pub const MM_VOCALTEC_WAVEIN: u32 = 2u32;
pub const MM_VOCALTEC_WAVEOUT: u32 = 1u32;
pub const MM_VOICEINFO: u32 = 156u32;
pub const MM_VOICEMIXER: u32 = 1u32;
pub const MM_VOXWARE: u32 = 114u32;
pub const MM_VOXWARE_CODEC: u32 = 1u32;
pub const MM_VOYETRA: u32 = 30u32;
pub const MM_VQST: u32 = 240u32;
pub const MM_VQST_VQC1: u32 = 1u32;
pub const MM_VQST_VQC2: u32 = 2u32;
pub const MM_VTG: u32 = 109u32;
pub const MM_WANGLABS: u32 = 28u32;
pub const MM_WANGLABS_WAVEIN1: u32 = 1u32;
pub const MM_WANGLABS_WAVEOUT1: u32 = 2u32;
pub const MM_WAVE_MAPPER: u32 = 2u32;
pub const MM_WEITEK: u32 = 96u32;
pub const MM_WILDCAT: u32 = 119u32;
pub const MM_WILDCAT_AUTOSCOREMIDIIN: u32 = 1u32;
pub const MM_WILLOPOND_SNDCOMM_WAVEIN: u32 = 108u32;
pub const MM_WILLOWPOND: u32 = 65u32;
pub const MM_WILLOWPOND_FMSYNTH_STEREO: u32 = 20u32;
pub const MM_WILLOWPOND_GENERIC_AUX: u32 = 115u32;
pub const MM_WILLOWPOND_GENERIC_MIXER: u32 = 114u32;
pub const MM_WILLOWPOND_GENERIC_WAVEIN: u32 = 112u32;
pub const MM_WILLOWPOND_GENERIC_WAVEOUT: u32 = 113u32;
pub const MM_WILLOWPOND_MPU401: u32 = 21u32;
pub const MM_WILLOWPOND_PH_AUX: u32 = 107u32;
pub const MM_WILLOWPOND_PH_MIXER: u32 = 106u32;
pub const MM_WILLOWPOND_PH_WAVEIN: u32 = 104u32;
pub const MM_WILLOWPOND_PH_WAVEOUT: u32 = 105u32;
pub const MM_WILLOWPOND_SNDCOMM_AUX: u32 = 111u32;
pub const MM_WILLOWPOND_SNDCOMM_MIXER: u32 = 110u32;
pub const MM_WILLOWPOND_SNDCOMM_WAVEOUT: u32 = 109u32;
pub const MM_WILLOWPOND_SNDPORT_AUX: u32 = 103u32;
pub const MM_WILLOWPOND_SNDPORT_MIXER: u32 = 102u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEIN: u32 = 100u32;
pub const MM_WILLOWPOND_SNDPORT_WAVEOUT: u32 = 101u32;
pub const MM_WIM_CLOSE: u32 = 959u32;
pub const MM_WIM_DATA: u32 = 960u32;
pub const MM_WIM_OPEN: u32 = 958u32;
pub const MM_WINBOND: u32 = 204u32;
pub const MM_WINNOV: u32 = 61u32;
pub const MM_WINNOV_CAVIAR_CHAMPAGNE: u32 = 4u32;
pub const MM_WINNOV_CAVIAR_VIDC: u32 = 3u32;
pub const MM_WINNOV_CAVIAR_WAVEIN: u32 = 1u32;
pub const MM_WINNOV_CAVIAR_WAVEOUT: u32 = 2u32;
pub const MM_WINNOV_CAVIAR_YUV8: u32 = 5u32;
pub const MM_WOM_CLOSE: u32 = 956u32;
pub const MM_WOM_DONE: u32 = 957u32;
pub const MM_WOM_OPEN: u32 = 955u32;
pub const MM_WORKBIT: u32 = 102u32;
pub const MM_WORKBIT_AUX: u32 = 7u32;
pub const MM_WORKBIT_FMSYNTH: u32 = 6u32;
pub const MM_WORKBIT_JOYSTICK: u32 = 8u32;
pub const MM_WORKBIT_MIDIIN: u32 = 4u32;
pub const MM_WORKBIT_MIDIOUT: u32 = 5u32;
pub const MM_WORKBIT_MIXER: u32 = 1u32;
pub const MM_WORKBIT_WAVEIN: u32 = 3u32;
pub const MM_WORKBIT_WAVEOUT: u32 = 2u32;
pub const MM_WSS_SB16_AUX_CD: u32 = 45u32;
pub const MM_WSS_SB16_AUX_LINE: u32 = 44u32;
pub const MM_WSS_SB16_MIDIIN: u32 = 41u32;
pub const MM_WSS_SB16_MIDIOUT: u32 = 42u32;
pub const MM_WSS_SB16_MIXER: u32 = 46u32;
pub const MM_WSS_SB16_SYNTH: u32 = 43u32;
pub const MM_WSS_SB16_WAVEIN: u32 = 39u32;
pub const MM_WSS_SB16_WAVEOUT: u32 = 40u32;
pub const MM_WSS_SBPRO_AUX_CD: u32 = 53u32;
pub const MM_WSS_SBPRO_AUX_LINE: u32 = 52u32;
pub const MM_WSS_SBPRO_MIDIIN: u32 = 49u32;
pub const MM_WSS_SBPRO_MIDIOUT: u32 = 50u32;
pub const MM_WSS_SBPRO_MIXER: u32 = 54u32;
pub const MM_WSS_SBPRO_SYNTH: u32 = 51u32;
pub const MM_WSS_SBPRO_WAVEIN: u32 = 47u32;
pub const MM_WSS_SBPRO_WAVEOUT: u32 = 48u32;
pub const MM_XEBEC: u32 = 85u32;
pub const MM_XIRLINK: u32 = 178u32;
pub const MM_XIRLINK_VISIONLINK: u32 = 1u32;
pub const MM_XYZ: u32 = 112u32;
pub const MM_YAMAHA: u32 = 37u32;
pub const MM_YAMAHA_ACXG_AUX: u32 = 41u32;
pub const MM_YAMAHA_ACXG_MIDIOUT: u32 = 39u32;
pub const MM_YAMAHA_ACXG_MIXER: u32 = 40u32;
pub const MM_YAMAHA_ACXG_WAVEIN: u32 = 37u32;
pub const MM_YAMAHA_ACXG_WAVEOUT: u32 = 38u32;
pub const MM_YAMAHA_GSS_AUX: u32 = 6u32;
pub const MM_YAMAHA_GSS_MIDIIN: u32 = 5u32;
pub const MM_YAMAHA_GSS_MIDIOUT: u32 = 4u32;
pub const MM_YAMAHA_GSS_SYNTH: u32 = 1u32;
pub const MM_YAMAHA_GSS_WAVEIN: u32 = 3u32;
pub const MM_YAMAHA_GSS_WAVEOUT: u32 = 2u32;
pub const MM_YAMAHA_OPL3SA_FMSYNTH: u32 = 18u32;
pub const MM_YAMAHA_OPL3SA_JOYSTICK: u32 = 24u32;
pub const MM_YAMAHA_OPL3SA_MIDIIN: u32 = 21u32;
pub const MM_YAMAHA_OPL3SA_MIDIOUT: u32 = 20u32;
pub const MM_YAMAHA_OPL3SA_MIXER: u32 = 23u32;
pub const MM_YAMAHA_OPL3SA_WAVEIN: u32 = 17u32;
pub const MM_YAMAHA_OPL3SA_WAVEOUT: u32 = 16u32;
pub const MM_YAMAHA_OPL3SA_YSYNTH: u32 = 19u32;
pub const MM_YAMAHA_SERIAL_MIDIIN: u32 = 8u32;
pub const MM_YAMAHA_SERIAL_MIDIOUT: u32 = 7u32;
pub const MM_YAMAHA_SXG_MIDIOUT: u32 = 34u32;
pub const MM_YAMAHA_SXG_MIXER: u32 = 36u32;
pub const MM_YAMAHA_SXG_WAVEOUT: u32 = 35u32;
pub const MM_YAMAHA_YMF724LEG_FMSYNTH: u32 = 32u32;
pub const MM_YAMAHA_YMF724LEG_MIDIIN: u32 = 26u32;
pub const MM_YAMAHA_YMF724LEG_MIDIOUT: u32 = 25u32;
pub const MM_YAMAHA_YMF724LEG_MIXER: u32 = 33u32;
pub const MM_YAMAHA_YMF724_AUX: u32 = 30u32;
pub const MM_YAMAHA_YMF724_MIDIOUT: u32 = 29u32;
pub const MM_YAMAHA_YMF724_MIXER: u32 = 31u32;
pub const MM_YAMAHA_YMF724_WAVEIN: u32 = 28u32;
pub const MM_YAMAHA_YMF724_WAVEOUT: u32 = 27u32;
pub const MM_YOUCOM: u32 = 256u32;
pub const MM_ZEFIRO: u32 = 170u32;
pub const MM_ZEFIRO_ZA2: u32 = 2u32;
pub const MM_ZYXEL: u32 = 9u32;
pub const MM_ZYXEL_ACM_ADPCM: u32 = 1u32;
pub const MODM_CACHEDRUMPATCHES: u32 = 13u32;
pub const MODM_CACHEPATCHES: u32 = 12u32;
pub const MODM_CLOSE: u32 = 4u32;
pub const MODM_DATA: u32 = 7u32;
pub const MODM_GETDEVCAPS: u32 = 2u32;
pub const MODM_GETNUMDEVS: u32 = 1u32;
pub const MODM_GETPOS: u32 = 17u32;
pub const MODM_GETVOLUME: u32 = 10u32;
pub const MODM_INIT: u32 = 100u32;
pub const MODM_INIT_EX: u32 = 104u32;
pub const MODM_LONGDATA: u32 = 8u32;
pub const MODM_MAPPER: u32 = 8192u32;
pub const MODM_OPEN: u32 = 3u32;
pub const MODM_PAUSE: u32 = 18u32;
pub const MODM_PREFERRED: u32 = 22u32;
pub const MODM_PREPARE: u32 = 5u32;
pub const MODM_PROPERTIES: u32 = 21u32;
pub const MODM_RECONFIGURE: u32 = 18280u32;
pub const MODM_RESET: u32 = 9u32;
pub const MODM_RESTART: u32 = 19u32;
pub const MODM_SETVOLUME: u32 = 11u32;
pub const MODM_STOP: u32 = 20u32;
pub const MODM_STRMDATA: u32 = 14u32;
pub const MODM_UNPREPARE: u32 = 6u32;
pub const MODM_USER: u32 = 16384u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub const MOM_CLOSE: u32 = 968u32;
pub const MOM_DONE: u32 = 969u32;
pub const MOM_OPEN: u32 = 967u32;
pub const MOM_POSITIONCB: u32 = 970u32;
pub const MPEGLAYER3_ID_CONSTANTFRAMESIZE: u32 = 2u32;
pub const MPEGLAYER3_ID_MPEG: u32 = 1u32;
pub const MPEGLAYER3_ID_UNKNOWN: u32 = 0u32;
pub const MPEGLAYER3_WFX_EXTRA_BYTES: u32 = 12u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MSAUDIO1WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
    pub wEncodeOptions: u16,
}
impl MSAUDIO1WAVEFORMAT {}
impl ::std::default::Default for MSAUDIO1WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MSAUDIO1WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MSAUDIO1WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for MSAUDIO1WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MSAUDIO1_BITS_PER_SAMPLE: u32 = 16u32;
pub const MSAUDIO1_MAX_CHANNELS: u32 = 2u32;
pub const MXDM_BASE: u32 = 1u32;
pub const MXDM_CLOSE: u32 = 4u32;
pub const MXDM_GETCONTROLDETAILS: u32 = 7u32;
pub const MXDM_GETDEVCAPS: u32 = 2u32;
pub const MXDM_GETLINECONTROLS: u32 = 6u32;
pub const MXDM_GETLINEINFO: u32 = 5u32;
pub const MXDM_GETNUMDEVS: u32 = 1u32;
pub const MXDM_INIT: u32 = 100u32;
pub const MXDM_INIT_EX: u32 = 104u32;
pub const MXDM_OPEN: u32 = 3u32;
pub const MXDM_SETCONTROLDETAILS: u32 = 8u32;
pub const MXDM_USER: u32 = 16384u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct NMS_VBXADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wSamplesPerBlock: u16,
}
impl NMS_VBXADPCMWAVEFORMAT {}
impl ::std::default::Default for NMS_VBXADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NMS_VBXADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NMS_VBXADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for NMS_VBXADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NS_DRM_E_MIGRATION_IMAGE_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879730i32 as _);
pub const NS_DRM_E_MIGRATION_SOURCE_MACHINE_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879732i32 as _);
pub const NS_DRM_E_MIGRATION_TARGET_MACHINE_LESS_THAN_LH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879731i32 as _);
pub const NS_DRM_E_MIGRATION_UPGRADE_WITH_DIFF_SID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879733i32 as _);
pub const NS_E_8BIT_WAVE_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886834i32 as _);
pub const NS_E_ACTIVE_SG_DEVICE_CONTROL_DISCONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882778i32 as _);
pub const NS_E_ACTIVE_SG_DEVICE_DISCONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882779i32 as _);
pub const NS_E_ADVANCEDEDIT_TOO_MANY_PICTURES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884886i32 as _);
pub const NS_E_ALLOCATE_FILE_FAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889759i32 as _);
pub const NS_E_ALL_PROTOCOLS_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877845i32 as _);
pub const NS_E_ALREADY_CONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889840i32 as _);
pub const NS_E_ANALOG_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879353i32 as _);
pub const NS_E_ARCHIVE_ABORT_DUE_TO_BCAST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884338i32 as _);
pub const NS_E_ARCHIVE_FILENAME_NOTSET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882823i32 as _);
pub const NS_E_ARCHIVE_GAP_DETECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884337i32 as _);
pub const NS_E_ARCHIVE_REACH_QUOTA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884339i32 as _);
pub const NS_E_ARCHIVE_SAME_AS_INPUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882812i32 as _);
pub const NS_E_ASSERT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889653i32 as _);
pub const NS_E_ASX_INVALIDFORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885655i32 as _);
pub const NS_E_ASX_INVALIDVERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885654i32 as _);
pub const NS_E_ASX_INVALID_REPEAT_BLOCK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885653i32 as _);
pub const NS_E_ASX_NOTHING_TO_WRITE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885652i32 as _);
pub const NS_E_ATTRIBUTE_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886825i32 as _);
pub const NS_E_ATTRIBUTE_READ_ONLY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886826i32 as _);
pub const NS_E_AUDIENCE_CONTENTTYPE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882791i32 as _);
pub const NS_E_AUDIENCE__LANGUAGE_CONTENTTYPE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882717i32 as _);
pub const NS_E_AUDIODEVICE_BADFORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882845i32 as _);
pub const NS_E_AUDIODEVICE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882847i32 as _);
pub const NS_E_AUDIODEVICE_UNEXPECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882846i32 as _);
pub const NS_E_AUDIO_BITRATE_STEPDOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882759i32 as _);
pub const NS_E_AUDIO_CODEC_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886845i32 as _);
pub const NS_E_AUDIO_CODEC_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886846i32 as _);
pub const NS_E_AUTHORIZATION_FILE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884336i32 as _);
pub const NS_E_BACKUP_RESTORE_BAD_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879803i32 as _);
pub const NS_E_BACKUP_RESTORE_BAD_REQUEST_ID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879826i32 as _);
pub const NS_E_BACKUP_RESTORE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879827i32 as _);
pub const NS_E_BACKUP_RESTORE_TOO_MANY_RESETS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879770i32 as _);
pub const NS_E_BAD_ADAPTER_ADDRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889799i32 as _);
pub const NS_E_BAD_ADAPTER_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889652i32 as _);
pub const NS_E_BAD_BLOCK0_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889757i32 as _);
pub const NS_E_BAD_CONTENTEDL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882774i32 as _);
pub const NS_E_BAD_CONTROL_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889806i32 as _);
pub const NS_E_BAD_CUB_UID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889454i32 as _);
pub const NS_E_BAD_DELIVERY_MODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889798i32 as _);
pub const NS_E_BAD_DISK_UID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889756i32 as _);
pub const NS_E_BAD_FSMAJOR_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889755i32 as _);
pub const NS_E_BAD_MARKIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882856i32 as _);
pub const NS_E_BAD_MARKOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882855i32 as _);
pub const NS_E_BAD_MULTICAST_ADDRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889800i32 as _);
pub const NS_E_BAD_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877853i32 as _);
pub const NS_E_BAD_STAMPNUMBER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889754i32 as _);
pub const NS_E_BAD_SYNTAX_IN_SERVER_RESPONSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877826i32 as _);
pub const NS_E_BKGDOWNLOAD_CALLFUNCENDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885145i32 as _);
pub const NS_E_BKGDOWNLOAD_CALLFUNCFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885147i32 as _);
pub const NS_E_BKGDOWNLOAD_CALLFUNCTIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885146i32 as _);
pub const NS_E_BKGDOWNLOAD_CANCELCOMPLETEDJOB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885153i32 as _);
pub const NS_E_BKGDOWNLOAD_COMPLETECANCELLEDJOB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885154i32 as _);
pub const NS_E_BKGDOWNLOAD_FAILEDINITIALIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885143i32 as _);
pub const NS_E_BKGDOWNLOAD_FAILED_TO_CREATE_TEMPFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885150i32 as _);
pub const NS_E_BKGDOWNLOAD_INVALIDJOBSIGNATURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885151i32 as _);
pub const NS_E_BKGDOWNLOAD_INVALID_FILE_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885141i32 as _);
pub const NS_E_BKGDOWNLOAD_NOJOBPOINTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885152i32 as _);
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDINITIALIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885149i32 as _);
pub const NS_E_BKGDOWNLOAD_PLUGIN_FAILEDTOMOVEFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885148i32 as _);
pub const NS_E_BKGDOWNLOAD_WMDUNPACKFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885144i32 as _);
pub const NS_E_BKGDOWNLOAD_WRONG_NO_FILES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885155i32 as _);
pub const NS_E_BUSY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1072889819i32 as _);
pub const NS_E_CACHE_ARCHIVE_CONFLICT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884756i32 as _);
pub const NS_E_CACHE_CANNOT_BE_CACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884752i32 as _);
pub const NS_E_CACHE_NOT_BROADCAST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884753i32 as _);
pub const NS_E_CACHE_NOT_MODIFIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884751i32 as _);
pub const NS_E_CACHE_ORIGIN_SERVER_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884755i32 as _);
pub const NS_E_CACHE_ORIGIN_SERVER_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884754i32 as _);
pub const NS_E_CANNOTCONNECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889850i32 as _);
pub const NS_E_CANNOTCONNECTEVENTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889745i32 as _);
pub const NS_E_CANNOTDESTROYTITLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889849i32 as _);
pub const NS_E_CANNOTOFFLINEDISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889847i32 as _);
pub const NS_E_CANNOTONLINEDISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889846i32 as _);
pub const NS_E_CANNOTRENAMETITLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889848i32 as _);
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884904i32 as _);
pub const NS_E_CANNOT_BUY_OR_DOWNLOAD_FROM_MULTIPLE_SERVICES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884905i32 as _);
pub const NS_E_CANNOT_CONNECT_TO_PROXY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877842i32 as _);
pub const NS_E_CANNOT_DELETE_ACTIVE_SOURCEGROUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882848i32 as _);
pub const NS_E_CANNOT_GENERATE_BROADCAST_INFO_FOR_QUALITYVBR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882721i32 as _);
pub const NS_E_CANNOT_PAUSE_LIVEBROADCAST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882802i32 as _);
pub const NS_E_CANNOT_READ_PLAYLIST_FROM_MEDIASERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877838i32 as _);
pub const NS_E_CANNOT_REMOVE_PLUGIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884655i32 as _);
pub const NS_E_CANNOT_REMOVE_PUBLISHING_POINT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884656i32 as _);
pub const NS_E_CANNOT_SYNC_DRM_TO_NON_JANUS_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885178i32 as _);
pub const NS_E_CANNOT_SYNC_PREVIOUS_SYNC_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885177i32 as _);
pub const NS_E_CANT_READ_DIGITAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885855i32 as _);
pub const NS_E_CCLINK_DOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889821i32 as _);
pub const NS_E_CD_COPYTO_CD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885842i32 as _);
pub const NS_E_CD_DRIVER_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885838i32 as _);
pub const NS_E_CD_EMPTY_TRACK_QUEUE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885255i32 as _);
pub const NS_E_CD_ISRC_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885253i32 as _);
pub const NS_E_CD_MEDIA_CATALOG_NUMBER_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885252i32 as _);
pub const NS_E_CD_NO_BUFFERS_READ: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885256i32 as _);
pub const NS_E_CD_NO_READER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885254i32 as _);
pub const NS_E_CD_QUEUEING_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885249i32 as _);
pub const NS_E_CD_READ_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885844i32 as _);
pub const NS_E_CD_READ_ERROR_NO_CORRECTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885845i32 as _);
pub const NS_E_CD_REFRESH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885839i32 as _);
pub const NS_E_CD_SLOW_COPY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885843i32 as _);
pub const NS_E_CD_SPEEDDETECT_NOT_ENOUGH_READS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885250i32 as _);
pub const NS_E_CHANGING_PROXYBYPASS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885565i32 as _);
pub const NS_E_CHANGING_PROXY_EXCEPTIONLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885566i32 as _);
pub const NS_E_CHANGING_PROXY_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885568i32 as _);
pub const NS_E_CHANGING_PROXY_PORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885567i32 as _);
pub const NS_E_CHANGING_PROXY_PROTOCOL_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885564i32 as _);
pub const NS_E_CLOSED_ON_SUSPEND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877839i32 as _);
pub const NS_E_CODEC_DMO_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886822i32 as _);
pub const NS_E_CODEC_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882813i32 as _);
pub const NS_E_COMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879352i32 as _);
pub const NS_E_COMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879355i32 as _);
pub const NS_E_CONNECTION_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889815i32 as _);
pub const NS_E_CONNECT_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877818i32 as _);
pub const NS_E_CONTENT_PARTNER_STILL_INITIALIZING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884894i32 as _);
pub const NS_E_CORECD_NOTAMEDIACD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885561i32 as _);
pub const NS_E_CRITICAL_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884452i32 as _);
pub const NS_E_CUB_FAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889773i32 as _);
pub const NS_E_CUB_FAIL_LINK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889456i32 as _);
pub const NS_E_CURLHELPER_NOTADIRECTORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884947i32 as _);
pub const NS_E_CURLHELPER_NOTAFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884946i32 as _);
pub const NS_E_CURLHELPER_NOTRELATIVE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884944i32 as _);
pub const NS_E_CURL_CANTDECODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884945i32 as _);
pub const NS_E_CURL_CANTWALK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884949i32 as _);
pub const NS_E_CURL_INVALIDBUFFERSIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884943i32 as _);
pub const NS_E_CURL_INVALIDCHAR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884955i32 as _);
pub const NS_E_CURL_INVALIDHOSTNAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884954i32 as _);
pub const NS_E_CURL_INVALIDPATH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884953i32 as _);
pub const NS_E_CURL_INVALIDPORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884948i32 as _);
pub const NS_E_CURL_INVALIDSCHEME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884952i32 as _);
pub const NS_E_CURL_INVALIDURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884951i32 as _);
pub const NS_E_CURL_NOTSAFE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884956i32 as _);
pub const NS_E_DAMAGED_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885813i32 as _);
pub const NS_E_DATAPATH_NO_SINK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884456i32 as _);
pub const NS_E_DATA_SOURCE_ENUMERATION_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884352i32 as _);
pub const NS_E_DATA_UNIT_EXTENSION_TOO_LARGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886823i32 as _);
pub const NS_E_DDRAW_GENERIC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885571i32 as _);
pub const NS_E_DEVCONTROL_FAILED_SEEK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882796i32 as _);
pub const NS_E_DEVICECONTROL_UNSTABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882719i32 as _);
pub const NS_E_DEVICE_DISCONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885854i32 as _);
pub const NS_E_DEVICE_IS_NOT_READY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885385i32 as _);
pub const NS_E_DEVICE_NOT_READY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885814i32 as _);
pub const NS_E_DEVICE_NOT_SUPPORT_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885853i32 as _);
pub const NS_E_DEVICE_NOT_WMDRM_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879749i32 as _);
pub const NS_E_DISK_FAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889771i32 as _);
pub const NS_E_DISK_READ: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889833i32 as _);
pub const NS_E_DISK_WRITE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889834i32 as _);
pub const NS_E_DISPLAY_MODE_CHANGE_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885570i32 as _);
pub const NS_E_DRMPROFILE_NOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882731i32 as _);
pub const NS_E_DRM_ACQUIRING_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879829i32 as _);
pub const NS_E_DRM_ACTION_NOT_QUERIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879830i32 as _);
pub const NS_E_DRM_ALREADY_INDIVIDUALIZED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879831i32 as _);
pub const NS_E_DRM_APPCERT_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879790i32 as _);
pub const NS_E_DRM_ATTRIBUTE_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879438i32 as _);
pub const NS_E_DRM_BACKUPRESTORE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879804i32 as _);
pub const NS_E_DRM_BACKUP_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879805i32 as _);
pub const NS_E_DRM_BACKUP_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879806i32 as _);
pub const NS_E_DRM_BAD_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879440i32 as _);
pub const NS_E_DRM_BB_UNABLE_TO_INITIALIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879744i32 as _);
pub const NS_E_DRM_BUFFER_TOO_SMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879780i32 as _);
pub const NS_E_DRM_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879551i32 as _);
pub const NS_E_DRM_CACHED_CONTENT_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879797i32 as _);
pub const NS_E_DRM_CERTIFICATE_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879455i32 as _);
pub const NS_E_DRM_CERTIFICATE_SECURITY_LEVEL_INADEQUATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879442i32 as _);
pub const NS_E_DRM_CHAIN_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879540i32 as _);
pub const NS_E_DRM_CHECKPOINT_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879721i32 as _);
pub const NS_E_DRM_CHECKPOINT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879745i32 as _);
pub const NS_E_DRM_CHECKPOINT_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879722i32 as _);
pub const NS_E_DRM_CLIENT_CODE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879545i32 as _);
pub const NS_E_DRM_DATASTORE_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879741i32 as _);
pub const NS_E_DRM_DEBUGGING_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879769i32 as _);
pub const NS_E_DRM_DECRYPT_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879837i32 as _);
pub const NS_E_DRM_DEVICE_ACTIVATION_CANCELED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879771i32 as _);
pub const NS_E_DRM_DEVICE_ALREADY_REGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879445i32 as _);
pub const NS_E_DRM_DEVICE_LIMIT_REACHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879453i32 as _);
pub const NS_E_DRM_DEVICE_NOT_OPEN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879446i32 as _);
pub const NS_E_DRM_DEVICE_NOT_REGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879646i32 as _);
pub const NS_E_DRM_DRIVER_AUTH_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879795i32 as _);
pub const NS_E_DRM_DRIVER_DIGIOUT_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879792i32 as _);
pub const NS_E_DRM_DRMV2CLT_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879434i32 as _);
pub const NS_E_DRM_ENCRYPT_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879838i32 as _);
pub const NS_E_DRM_ENUM_LICENSE_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879845i32 as _);
pub const NS_E_DRM_ERROR_BAD_NET_RESP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879778i32 as _);
pub const NS_E_DRM_EXPIRED_LICENSEBLOB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879437i32 as _);
pub const NS_E_DRM_GET_CONTENTSTRING_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879811i32 as _);
pub const NS_E_DRM_GET_LICENSESTRING_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879812i32 as _);
pub const NS_E_DRM_GET_LICENSE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879815i32 as _);
pub const NS_E_DRM_HARDWAREID_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879729i32 as _);
pub const NS_E_DRM_HARDWARE_INCONSISTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879788i32 as _);
pub const NS_E_DRM_INCLUSION_LIST_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879435i32 as _);
pub const NS_E_DRM_INDIVIDUALIZATION_INCOMPLETE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879796i32 as _);
pub const NS_E_DRM_INDIVIDUALIZE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879818i32 as _);
pub const NS_E_DRM_INDIVIDUALIZING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879828i32 as _);
pub const NS_E_DRM_INDIV_FRAUD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879549i32 as _);
pub const NS_E_DRM_INDIV_NO_CABS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879548i32 as _);
pub const NS_E_DRM_INDIV_SERVICE_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879547i32 as _);
pub const NS_E_DRM_INVALID_APPCERT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879748i32 as _);
pub const NS_E_DRM_INVALID_APPDATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879808i32 as _);
pub const NS_E_DRM_INVALID_APPDATA_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879807i32 as _);
pub const NS_E_DRM_INVALID_APPLICATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879855i32 as _);
pub const NS_E_DRM_INVALID_CERTIFICATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879456i32 as _);
pub const NS_E_DRM_INVALID_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879850i32 as _);
pub const NS_E_DRM_INVALID_CRL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879439i32 as _);
pub const NS_E_DRM_INVALID_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879775i32 as _);
pub const NS_E_DRM_INVALID_KID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879543i32 as _);
pub const NS_E_DRM_INVALID_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879848i32 as _);
pub const NS_E_DRM_INVALID_LICENSEBLOB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879436i32 as _);
pub const NS_E_DRM_INVALID_LICENSE_ACQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879841i32 as _);
pub const NS_E_DRM_INVALID_LICENSE_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879844i32 as _);
pub const NS_E_DRM_INVALID_MACHINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879847i32 as _);
pub const NS_E_DRM_INVALID_MIGRATION_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879736i32 as _);
pub const NS_E_DRM_INVALID_PROPERTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879799i32 as _);
pub const NS_E_DRM_INVALID_PROXIMITY_RESPONSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879448i32 as _);
pub const NS_E_DRM_INVALID_SECURESTORE_PASSWORD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879791i32 as _);
pub const NS_E_DRM_INVALID_SESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879447i32 as _);
pub const NS_E_DRM_KEY_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879839i32 as _);
pub const NS_E_DRM_LICENSE_APPSECLOW: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879654i32 as _);
pub const NS_E_DRM_LICENSE_APP_NOTALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879651i32 as _);
pub const NS_E_DRM_LICENSE_CERT_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879649i32 as _);
pub const NS_E_DRM_LICENSE_CLOSE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879816i32 as _);
pub const NS_E_DRM_LICENSE_CONTENT_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879647i32 as _);
pub const NS_E_DRM_LICENSE_DELETION_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879538i32 as _);
pub const NS_E_DRM_LICENSE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879656i32 as _);
pub const NS_E_DRM_LICENSE_INITIALIZATION_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879542i32 as _);
pub const NS_E_DRM_LICENSE_INVALID_XML: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879835i32 as _);
pub const NS_E_DRM_LICENSE_NOSAP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879606i32 as _);
pub const NS_E_DRM_LICENSE_NOSVP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879605i32 as _);
pub const NS_E_DRM_LICENSE_NOTACQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879783i32 as _);
pub const NS_E_DRM_LICENSE_NOTENABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879655i32 as _);
pub const NS_E_DRM_LICENSE_NOTRUSTEDCODEC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879603i32 as _);
pub const NS_E_DRM_LICENSE_NOWDM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879604i32 as _);
pub const NS_E_DRM_LICENSE_OPEN_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879817i32 as _);
pub const NS_E_DRM_LICENSE_SECLOW: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879648i32 as _);
pub const NS_E_DRM_LICENSE_SERVER_INFO_MISSING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879552i32 as _);
pub const NS_E_DRM_LICENSE_STORE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879854i32 as _);
pub const NS_E_DRM_LICENSE_STORE_SAVE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879852i32 as _);
pub const NS_E_DRM_LICENSE_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879454i32 as _);
pub const NS_E_DRM_LICENSE_UNUSABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879800i32 as _);
pub const NS_E_DRM_LIC_NEEDS_DEVICE_CLOCK_SET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879751i32 as _);
pub const NS_E_DRM_MALFORMED_CONTENT_HEADER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879716i32 as _);
pub const NS_E_DRM_MIGRATION_IMPORTER_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879734i32 as _);
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879727i32 as _);
pub const NS_E_DRM_MIGRATION_INVALID_LEGACYV2_SST_PASSWORD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879725i32 as _);
pub const NS_E_DRM_MIGRATION_LICENSE_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879726i32 as _);
pub const NS_E_DRM_MIGRATION_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879724i32 as _);
pub const NS_E_DRM_MIGRATION_OBJECT_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879717i32 as _);
pub const NS_E_DRM_MIGRATION_OPERATION_CANCELLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879718i32 as _);
pub const NS_E_DRM_MIGRATION_TARGET_NOT_ONLINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879737i32 as _);
pub const NS_E_DRM_MIGRATION_TARGET_STATES_CORRUPTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879735i32 as _);
pub const NS_E_DRM_MONITOR_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879810i32 as _);
pub const NS_E_DRM_MUST_APPROVE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879450i32 as _);
pub const NS_E_DRM_MUST_REGISTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879451i32 as _);
pub const NS_E_DRM_MUST_REVALIDATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879449i32 as _);
pub const NS_E_DRM_NEEDS_INDIVIDUALIZATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879832i32 as _);
pub const NS_E_DRM_NEEDS_UPGRADE_TEMPFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879555i32 as _);
pub const NS_E_DRM_NEED_UPGRADE_MSSAP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879794i32 as _);
pub const NS_E_DRM_NEED_UPGRADE_PD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879554i32 as _);
pub const NS_E_DRM_NOT_CONFIGURED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879772i32 as _);
pub const NS_E_DRM_NO_RIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879840i32 as _);
pub const NS_E_DRM_NO_UPLINK_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879544i32 as _);
pub const NS_E_DRM_OPERATION_CANCELED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879768i32 as _);
pub const NS_E_DRM_PARAMETERS_MISMATCHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879825i32 as _);
pub const NS_E_DRM_PASSWORD_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882797i32 as _);
pub const NS_E_DRM_PD_TOO_MANY_DEVICES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879550i32 as _);
pub const NS_E_DRM_POLICY_DISABLE_ONLINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879774i32 as _);
pub const NS_E_DRM_POLICY_METERING_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879754i32 as _);
pub const NS_E_DRM_PROFILE_NOT_SET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882801i32 as _);
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_CHALLENGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879746i32 as _);
pub const NS_E_DRM_PROTOCOL_FORCEFUL_TERMINATION_ON_PETITION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879747i32 as _);
pub const NS_E_DRM_QUERY_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879814i32 as _);
pub const NS_E_DRM_REOPEN_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879793i32 as _);
pub const NS_E_DRM_REPORT_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879813i32 as _);
pub const NS_E_DRM_RESTORE_FRAUD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879789i32 as _);
pub const NS_E_DRM_RESTORE_SERVICE_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879546i32 as _);
pub const NS_E_DRM_RESTRICTIONS_NOT_RETRIEVED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879767i32 as _);
pub const NS_E_DRM_RIV_TOO_SMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879433i32 as _);
pub const NS_E_DRM_SDK_VERSIONMISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879752i32 as _);
pub const NS_E_DRM_SDMI_NOMORECOPIES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879786i32 as _);
pub const NS_E_DRM_SDMI_TRIGGER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879787i32 as _);
pub const NS_E_DRM_SECURE_STORE_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879853i32 as _);
pub const NS_E_DRM_SECURE_STORE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879798i32 as _);
pub const NS_E_DRM_SECURE_STORE_UNLOCK_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879851i32 as _);
pub const NS_E_DRM_SECURITY_COMPONENT_SIGNATURE_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879776i32 as _);
pub const NS_E_DRM_SIGNATURE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879553i32 as _);
pub const NS_E_DRM_SOURCEID_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879602i32 as _);
pub const NS_E_DRM_STORE_NEEDINDI: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879653i32 as _);
pub const NS_E_DRM_STORE_NOTALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879652i32 as _);
pub const NS_E_DRM_STORE_NOTALLSTORED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879777i32 as _);
pub const NS_E_DRM_STUBLIB_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879739i32 as _);
pub const NS_E_DRM_TRACK_EXCEEDED_PLAYLIST_RESTICTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879760i32 as _);
pub const NS_E_DRM_TRACK_EXCEEDED_TRACKBURN_RESTRICTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879759i32 as _);
pub const NS_E_DRM_TRANSFER_CHAINED_LICENSES_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879753i32 as _);
pub const NS_E_DRM_UNABLE_TO_ACQUIRE_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879842i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_AUTHENTICATION_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879773i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_BACKUP_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879819i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_CERTIFICATE_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879738i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_CODING_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879782i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_DECRYPT_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879821i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_DEVICE_REGISTRATION_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879764i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_ENCRYPT_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879822i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_HEADER_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879785i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_INDI_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879823i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_INMEMORYSTORE_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879740i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_KEYS_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879784i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_LICENSE_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879824i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_METERING_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879763i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_MIGRATION_IMPORTER_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879723i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_BURN_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879765i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_PLAYLIST_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879766i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_PROPERTIES_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879820i32 as _);
pub const NS_E_DRM_UNABLE_TO_CREATE_STATE_DATA_OBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879781i32 as _);
pub const NS_E_DRM_UNABLE_TO_GET_DEVICE_CERT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879758i32 as _);
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879757i32 as _);
pub const NS_E_DRM_UNABLE_TO_GET_SECURE_CLOCK_FROM_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879755i32 as _);
pub const NS_E_DRM_UNABLE_TO_INITIALIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879843i32 as _);
pub const NS_E_DRM_UNABLE_TO_LOAD_HARDWARE_ID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879743i32 as _);
pub const NS_E_DRM_UNABLE_TO_OPEN_DATA_STORE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879742i32 as _);
pub const NS_E_DRM_UNABLE_TO_OPEN_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879849i32 as _);
pub const NS_E_DRM_UNABLE_TO_OPEN_PORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879441i32 as _);
pub const NS_E_DRM_UNABLE_TO_SET_PARAMETER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879809i32 as _);
pub const NS_E_DRM_UNABLE_TO_SET_SECURE_CLOCK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879756i32 as _);
pub const NS_E_DRM_UNABLE_TO_VERIFY_PROXIMITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879452i32 as _);
pub const NS_E_DRM_UNSUPPORTED_ACTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879443i32 as _);
pub const NS_E_DRM_UNSUPPORTED_ALGORITHM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879539i32 as _);
pub const NS_E_DRM_UNSUPPORTED_PROPERTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879779i32 as _);
pub const NS_E_DRM_UNSUPPORTED_PROTOCOL_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879444i32 as _);
pub const NS_E_DUPLICATE_ADDRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889801i32 as _);
pub const NS_E_DUPLICATE_DRMPROFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882800i32 as _);
pub const NS_E_DUPLICATE_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889802i32 as _);
pub const NS_E_DUPLICATE_PACKET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886829i32 as _);
pub const NS_E_DVD_AUTHORING_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885404i32 as _);
pub const NS_E_DVD_CANNOT_COPY_PROTECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885390i32 as _);
pub const NS_E_DVD_CANNOT_JUMP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885393i32 as _);
pub const NS_E_DVD_COMPATIBLE_VIDEO_CARD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885402i32 as _);
pub const NS_E_DVD_COPY_PROTECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885405i32 as _);
pub const NS_E_DVD_DEVICE_CONTENTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885392i32 as _);
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885407i32 as _);
pub const NS_E_DVD_DISC_COPY_PROTECT_OUTPUT_NS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885408i32 as _);
pub const NS_E_DVD_DISC_DECODER_REGION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885399i32 as _);
pub const NS_E_DVD_GRAPH_BUILDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885396i32 as _);
pub const NS_E_DVD_INVALID_DISC_REGION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885403i32 as _);
pub const NS_E_DVD_INVALID_TITLE_CHAPTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885388i32 as _);
pub const NS_E_DVD_MACROVISION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885401i32 as _);
pub const NS_E_DVD_NO_AUDIO_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885397i32 as _);
pub const NS_E_DVD_NO_DECODER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885395i32 as _);
pub const NS_E_DVD_NO_SUBPICTURE_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885406i32 as _);
pub const NS_E_DVD_NO_VIDEO_MEMORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885391i32 as _);
pub const NS_E_DVD_NO_VIDEO_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885398i32 as _);
pub const NS_E_DVD_PARENTAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885394i32 as _);
pub const NS_E_DVD_REQUIRED_PROPERTY_NOT_SET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885389i32 as _);
pub const NS_E_DVD_SYSTEM_DECODER_REGION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885400i32 as _);
pub const NS_E_EDL_REQUIRED_FOR_DEVICE_MULTIPASS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882713i32 as _);
pub const NS_E_EMPTY_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884555i32 as _);
pub const NS_E_EMPTY_PROGRAM_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889642i32 as _);
pub const NS_E_ENACTPLAN_GIVEUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889752i32 as _);
pub const NS_E_END_OF_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876856i32 as _);
pub const NS_E_END_OF_TAPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882770i32 as _);
pub const NS_E_ERROR_FROM_PROXY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877852i32 as _);
pub const NS_E_EXCEED_MAX_DRM_PROFILE_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882720i32 as _);
pub const NS_E_EXPECT_MONO_WAV_INPUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882783i32 as _);
pub const NS_E_FAILED_DOWNLOAD_ABORT_BURN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885540i32 as _);
pub const NS_E_FAIL_LAUNCH_ROXIO_PLUGIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885376i32 as _);
pub const NS_E_FEATURE_DISABLED_BY_GROUP_POLICY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886820i32 as _);
pub const NS_E_FEATURE_DISABLED_IN_SKU: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886819i32 as _);
pub const NS_E_FEATURE_REQUIRES_ENTERPRISE_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884349i32 as _);
pub const NS_E_FILE_ALLOCATION_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889826i32 as _);
pub const NS_E_FILE_BANDWIDTH_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889808i32 as _);
pub const NS_E_FILE_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889829i32 as _);
pub const NS_E_FILE_FAILED_CHECKS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885811i32 as _);
pub const NS_E_FILE_INIT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889825i32 as _);
pub const NS_E_FILE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889830i32 as _);
pub const NS_E_FILE_OPEN_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889827i32 as _);
pub const NS_E_FILE_PLAY_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889824i32 as _);
pub const NS_E_FILE_READ: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889831i32 as _);
pub const NS_E_FILE_WRITE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889832i32 as _);
pub const NS_E_FIREWALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877831i32 as _);
pub const NS_E_FLASH_PLAYBACK_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885553i32 as _);
pub const NS_E_GLITCH_MODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889451i32 as _);
pub const NS_E_GRAPH_NOAUDIOLANGUAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885563i32 as _);
pub const NS_E_GRAPH_NOAUDIOLANGUAGESELECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885562i32 as _);
pub const NS_E_HDS_KEY_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879719i32 as _);
pub const NS_E_HEADER_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884449i32 as _);
pub const NS_E_HTTP_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889645i32 as _);
pub const NS_E_HTTP_TEXT_DATACONTAINER_INVALID_SERVER_RESPONSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884340i32 as _);
pub const NS_E_HTTP_TEXT_DATACONTAINER_SIZE_LIMIT_EXCEEDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884343i32 as _);
pub const NS_E_ICMQUERYFORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882836i32 as _);
pub const NS_E_IE_DISALLOWS_ACTIVEX_CONTROLS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885554i32 as _);
pub const NS_E_IMAGE_DOWNLOAD_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885106i32 as _);
pub const NS_E_IMAPI_LOSSOFSTREAMING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885378i32 as _);
pub const NS_E_IMAPI_MEDIUM_INVALIDTYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885374i32 as _);
pub const NS_E_INCOMPATIBLE_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889791i32 as _);
pub const NS_E_INCOMPATIBLE_PUSH_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877812i32 as _);
pub const NS_E_INCOMPATIBLE_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877848i32 as _);
pub const NS_E_INCOMPATIBLE_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886841i32 as _);
pub const NS_E_INCOMPLETE_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885182i32 as _);
pub const NS_E_INCORRECTCLIPSETTINGS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882820i32 as _);
pub const NS_E_INDUCED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889822i32 as _);
pub const NS_E_INPUTSOURCE_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882806i32 as _);
pub const NS_E_INPUT_DOESNOT_SUPPORT_SMPTE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882776i32 as _);
pub const NS_E_INPUT_WAVFORMAT_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882782i32 as _);
pub const NS_E_INSUFFICIENT_BANDWIDTH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889812i32 as _);
pub const NS_E_INSUFFICIENT_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889654i32 as _);
pub const NS_E_INTERFACE_NOT_REGISTERED_IN_GIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885142i32 as _);
pub const NS_E_INTERLACEMODE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882773i32 as _);
pub const NS_E_INTERLACE_REQUIRE_SAMESIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882795i32 as _);
pub const NS_E_INTERNAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889820i32 as _);
pub const NS_E_INTERNAL_SERVER_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877854i32 as _);
pub const NS_E_INVALIDCALL_WHILE_ARCHIVAL_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882828i32 as _);
pub const NS_E_INVALIDCALL_WHILE_ENCODER_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882842i32 as _);
pub const NS_E_INVALIDCALL_WHILE_ENCODER_STOPPED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882817i32 as _);
pub const NS_E_INVALIDINPUTFPS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882815i32 as _);
pub const NS_E_INVALIDPACKETSIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882827i32 as _);
pub const NS_E_INVALIDPROFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886842i32 as _);
pub const NS_E_INVALID_ARCHIVE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889795i32 as _);
pub const NS_E_INVALID_AUDIO_BUFFERMAX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882756i32 as _);
pub const NS_E_INVALID_AUDIO_PEAKRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882758i32 as _);
pub const NS_E_INVALID_AUDIO_PEAKRATE_2: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882757i32 as _);
pub const NS_E_INVALID_BLACKHOLE_ADDRESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889792i32 as _);
pub const NS_E_INVALID_CHANNEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889797i32 as _);
pub const NS_E_INVALID_CLIENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889793i32 as _);
pub const NS_E_INVALID_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889809i32 as _);
pub const NS_E_INVALID_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882799i32 as _);
pub const NS_E_INVALID_DRMV2CLT_STUBLIB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879728i32 as _);
pub const NS_E_INVALID_EDL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886824i32 as _);
pub const NS_E_INVALID_FILE_BITRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882735i32 as _);
pub const NS_E_INVALID_FOLDDOWN_COEFFICIENTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882732i32 as _);
pub const NS_E_INVALID_INDEX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889839i32 as _);
pub const NS_E_INVALID_INDEX2: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889639i32 as _);
pub const NS_E_INVALID_INPUT_AUDIENCE_INDEX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882786i32 as _);
pub const NS_E_INVALID_INPUT_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886856i32 as _);
pub const NS_E_INVALID_INPUT_LANGUAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882785i32 as _);
pub const NS_E_INVALID_INPUT_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882784i32 as _);
pub const NS_E_INVALID_INTERLACEMODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882725i32 as _);
pub const NS_E_INVALID_INTERLACE_COMPAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882724i32 as _);
pub const NS_E_INVALID_KEY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889790i32 as _);
pub const NS_E_INVALID_LOG_URL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884347i32 as _);
pub const NS_E_INVALID_MTU_RANGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884346i32 as _);
pub const NS_E_INVALID_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889828i32 as _);
pub const NS_E_INVALID_NONSQUAREPIXEL_COMPAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882723i32 as _);
pub const NS_E_INVALID_NUM_PASSES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886827i32 as _);
pub const NS_E_INVALID_OPERATING_SYSTEM_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884647i32 as _);
pub const NS_E_INVALID_OUTPUT_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886853i32 as _);
pub const NS_E_INVALID_PIXEL_ASPECT_RATIO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882718i32 as _);
pub const NS_E_INVALID_PLAY_STATISTICS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884345i32 as _);
pub const NS_E_INVALID_PLUGIN_LOAD_TYPE_CONFIGURATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884652i32 as _);
pub const NS_E_INVALID_PORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889789i32 as _);
pub const NS_E_INVALID_PROFILE_CONTENTTYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882716i32 as _);
pub const NS_E_INVALID_PUBLISHING_POINT_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884651i32 as _);
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884453i32 as _);
pub const NS_E_INVALID_PUSH_PUBLISHING_POINT_START_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884645i32 as _);
pub const NS_E_INVALID_PUSH_TEMPLATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884454i32 as _);
pub const NS_E_INVALID_QUERY_OPERATOR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876849i32 as _);
pub const NS_E_INVALID_QUERY_PROPERTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876848i32 as _);
pub const NS_E_INVALID_REDIRECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877846i32 as _);
pub const NS_E_INVALID_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889813i32 as _);
pub const NS_E_INVALID_SAMPLING_RATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886832i32 as _);
pub const NS_E_INVALID_SCRIPT_BITRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882737i32 as _);
pub const NS_E_INVALID_SOURCE_WITH_DEVICE_CONTROL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882722i32 as _);
pub const NS_E_INVALID_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889796i32 as _);
pub const NS_E_INVALID_TIMECODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882730i32 as _);
pub const NS_E_INVALID_TTL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889788i32 as _);
pub const NS_E_INVALID_VBR_COMPAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882766i32 as _);
pub const NS_E_INVALID_VBR_WITH_UNCOMP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882764i32 as _);
pub const NS_E_INVALID_VIDEO_BITRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882753i32 as _);
pub const NS_E_INVALID_VIDEO_BUFFER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882743i32 as _);
pub const NS_E_INVALID_VIDEO_BUFFERMAX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882742i32 as _);
pub const NS_E_INVALID_VIDEO_BUFFERMAX_2: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882741i32 as _);
pub const NS_E_INVALID_VIDEO_CQUALITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882744i32 as _);
pub const NS_E_INVALID_VIDEO_FPS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882747i32 as _);
pub const NS_E_INVALID_VIDEO_HEIGHT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882748i32 as _);
pub const NS_E_INVALID_VIDEO_HEIGHT_ALIGN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882739i32 as _);
pub const NS_E_INVALID_VIDEO_IQUALITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882745i32 as _);
pub const NS_E_INVALID_VIDEO_KEYFRAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882746i32 as _);
pub const NS_E_INVALID_VIDEO_PEAKRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882751i32 as _);
pub const NS_E_INVALID_VIDEO_PEAKRATE_2: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882750i32 as _);
pub const NS_E_INVALID_VIDEO_WIDTH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882749i32 as _);
pub const NS_E_INVALID_VIDEO_WIDTH_ALIGN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882740i32 as _);
pub const NS_E_INVALID_VIDEO_WIDTH_FOR_INTERLACED_ENCODING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882712i32 as _);
pub const NS_E_LANGUAGE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882788i32 as _);
pub const NS_E_LATE_OPERATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889810i32 as _);
pub const NS_E_LATE_PACKET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886830i32 as _);
pub const NS_E_LICENSE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889644i32 as _);
pub const NS_E_LICENSE_HEADER_MISSING_URL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879750i32 as _);
pub const NS_E_LICENSE_INCORRECT_RIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886847i32 as _);
pub const NS_E_LICENSE_OUTOFDATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886848i32 as _);
pub const NS_E_LICENSE_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886850i32 as _);
pub const NS_E_LOGFILEPERIOD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889784i32 as _);
pub const NS_E_LOG_FILE_SIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889782i32 as _);
pub const NS_E_LOG_NEED_TO_BE_SKIPPED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884344i32 as _);
pub const NS_E_MARKIN_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882711i32 as _);
pub const NS_E_MAX_BITRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889785i32 as _);
pub const NS_E_MAX_CLIENTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889783i32 as _);
pub const NS_E_MAX_FILERATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889781i32 as _);
pub const NS_E_MAX_FUNNELS_ALERT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889760i32 as _);
pub const NS_E_MAX_PACKET_SIZE_TOO_SMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886831i32 as _);
pub const NS_E_MEDIACD_READ_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885555i32 as _);
pub const NS_E_MEDIA_LIBRARY_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885810i32 as _);
pub const NS_E_MEDIA_PARSER_INVALID_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884351i32 as _);
pub const NS_E_MEMSTORAGE_BAD_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885381i32 as _);
pub const NS_E_METADATA_CACHE_DATA_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876837i32 as _);
pub const NS_E_METADATA_CANNOT_RETRIEVE_FROM_OFFLINE_CACHE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876834i32 as _);
pub const NS_E_METADATA_CANNOT_SET_LOCALE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876841i32 as _);
pub const NS_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876843i32 as _);
pub const NS_E_METADATA_IDENTIFIER_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876835i32 as _);
pub const NS_E_METADATA_INVALID_DOCUMENT_TYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876836i32 as _);
pub const NS_E_METADATA_LANGUAGE_NOT_SUPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876840i32 as _);
pub const NS_E_METADATA_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876838i32 as _);
pub const NS_E_METADATA_NO_EDITING_CAPABILITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876842i32 as _);
pub const NS_E_METADATA_NO_RFC1766_NAME_FOR_LOCALE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876839i32 as _);
pub const NS_E_MISMATCHED_MEDIACONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882849i32 as _);
pub const NS_E_MISSING_AUDIENCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882792i32 as _);
pub const NS_E_MISSING_CHANNEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889641i32 as _);
pub const NS_E_MISSING_SOURCE_INDEX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882790i32 as _);
pub const NS_E_MIXER_INVALID_CONTROL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885850i32 as _);
pub const NS_E_MIXER_INVALID_LINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885851i32 as _);
pub const NS_E_MIXER_INVALID_VALUE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885849i32 as _);
pub const NS_E_MIXER_NODRIVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885841i32 as _);
pub const NS_E_MIXER_UNKNOWN_MMRESULT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885848i32 as _);
pub const NS_E_MLS_SMARTPLAYLIST_FILTER_NOT_REGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885643i32 as _);
pub const NS_E_MMSAUTOSERVER_CANTFINDWALKER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889786i32 as _);
pub const NS_E_MMS_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877830i32 as _);
pub const NS_E_MONITOR_GIVEUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889656i32 as _);
pub const NS_E_MP3_FORMAT_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885846i32 as _);
pub const NS_E_MPDB_GENERIC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885812i32 as _);
pub const NS_E_MSAUDIO_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886855i32 as _);
pub const NS_E_MSBD_NO_LONGER_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877844i32 as _);
pub const NS_E_MULTICAST_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877847i32 as _);
pub const NS_E_MULTICAST_PLUGIN_NOT_ENABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884648i32 as _);
pub const NS_E_MULTIPLE_AUDIO_CODECS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882761i32 as _);
pub const NS_E_MULTIPLE_AUDIO_FORMATS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882760i32 as _);
pub const NS_E_MULTIPLE_FILE_BITRATES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882736i32 as _);
pub const NS_E_MULTIPLE_SCRIPT_BITRATES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882738i32 as _);
pub const NS_E_MULTIPLE_VBR_AUDIENCES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882763i32 as _);
pub const NS_E_MULTIPLE_VIDEO_CODECS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882755i32 as _);
pub const NS_E_MULTIPLE_VIDEO_SIZES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882754i32 as _);
pub const NS_E_NAMESPACE_BAD_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884842i32 as _);
pub const NS_E_NAMESPACE_BUFFER_TOO_SMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884850i32 as _);
pub const NS_E_NAMESPACE_CALLBACK_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884847i32 as _);
pub const NS_E_NAMESPACE_DUPLICATE_CALLBACK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884848i32 as _);
pub const NS_E_NAMESPACE_DUPLICATE_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884845i32 as _);
pub const NS_E_NAMESPACE_EMPTY_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884844i32 as _);
pub const NS_E_NAMESPACE_INDEX_TOO_LARGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884843i32 as _);
pub const NS_E_NAMESPACE_NAME_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884846i32 as _);
pub const NS_E_NAMESPACE_NODE_CONFLICT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884852i32 as _);
pub const NS_E_NAMESPACE_NODE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884851i32 as _);
pub const NS_E_NAMESPACE_TOO_MANY_CALLBACKS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884849i32 as _);
pub const NS_E_NAMESPACE_WRONG_PERSIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884854i32 as _);
pub const NS_E_NAMESPACE_WRONG_SECURITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884841i32 as _);
pub const NS_E_NAMESPACE_WRONG_TYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884853i32 as _);
pub const NS_E_NEED_CORE_REFERENCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885556i32 as _);
pub const NS_E_NEED_TO_ASK_USER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885798i32 as _);
pub const NS_E_NETWORK_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889842i32 as _);
pub const NS_E_NETWORK_RESOURCE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889816i32 as _);
pub const NS_E_NETWORK_SERVICE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889817i32 as _);
pub const NS_E_NETWORK_SINK_WRITE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877832i32 as _);
pub const NS_E_NET_READ: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889835i32 as _);
pub const NS_E_NET_WRITE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889836i32 as _);
pub const NS_E_NOCONNECTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889851i32 as _);
pub const NS_E_NOFUNNEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889844i32 as _);
pub const NS_E_NOMATCHING_ELEMENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882850i32 as _);
pub const NS_E_NOMATCHING_MEDIASOURCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882854i32 as _);
pub const NS_E_NONSQUAREPIXELMODE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882772i32 as _);
pub const NS_E_NOREGISTEREDWALKER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889845i32 as _);
pub const NS_E_NOSOURCEGROUPS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882816i32 as _);
pub const NS_E_NOSTATSAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882819i32 as _);
pub const NS_E_NOTARCHIVING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882818i32 as _);
pub const NS_E_NOTHING_TO_DO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072887823i32 as _);
pub const NS_E_NOTITLES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889794i32 as _);
pub const NS_E_NOT_CONFIGURED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886852i32 as _);
pub const NS_E_NOT_CONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886837i32 as _);
pub const NS_E_NOT_CONTENT_PARTNER_TRACK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884902i32 as _);
pub const NS_E_NOT_LICENSED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889651i32 as _);
pub const NS_E_NOT_REBUILDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889811i32 as _);
pub const NS_E_NO_ACTIVE_SOURCEGROUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882830i32 as _);
pub const NS_E_NO_AUDIENCES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882768i32 as _);
pub const NS_E_NO_AUDIODATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882807i32 as _);
pub const NS_E_NO_AUDIO_COMPAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882767i32 as _);
pub const NS_E_NO_AUDIO_TIMECOMPRESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882729i32 as _);
pub const NS_E_NO_CD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885856i32 as _);
pub const NS_E_NO_CD_BURNER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885386i32 as _);
pub const NS_E_NO_CHANNELS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889640i32 as _);
pub const NS_E_NO_DATAVIEW_SUPPORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882814i32 as _);
pub const NS_E_NO_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889743i32 as _);
pub const NS_E_NO_ERROR_STRING_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885808i32 as _);
pub const NS_E_NO_EXISTING_PACKETIZER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877827i32 as _);
pub const NS_E_NO_FORMATS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889749i32 as _);
pub const NS_E_NO_FRAMES_SUBMITTED_TO_ANALYZER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882777i32 as _);
pub const NS_E_NO_LOCALPLAY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889843i32 as _);
pub const NS_E_NO_MBR_WITH_TIMECODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882726i32 as _);
pub const NS_E_NO_MEDIAFORMAT_IN_SOURCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882833i32 as _);
pub const NS_E_NO_MEDIA_IN_AUDIENCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882769i32 as _);
pub const NS_E_NO_MEDIA_PROTOCOL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889445i32 as _);
pub const NS_E_NO_MORE_SAMPLES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886833i32 as _);
pub const NS_E_NO_MULTICAST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072887822i32 as _);
pub const NS_E_NO_MULTIPASS_FOR_LIVEDEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882793i32 as _);
pub const NS_E_NO_NEW_CONNECTIONS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884451i32 as _);
pub const NS_E_NO_PAL_INVERSE_TELECINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882780i32 as _);
pub const NS_E_NO_PDA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885383i32 as _);
pub const NS_E_NO_PROFILE_IN_SOURCEGROUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882841i32 as _);
pub const NS_E_NO_PROFILE_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882765i32 as _);
pub const NS_E_NO_REALTIME_PREPROCESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882804i32 as _);
pub const NS_E_NO_REALTIME_TIMECOMPRESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882810i32 as _);
pub const NS_E_NO_REFERENCES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889748i32 as _);
pub const NS_E_NO_REPEAT_PREPROCESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882803i32 as _);
pub const NS_E_NO_SCRIPT_ENGINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884356i32 as _);
pub const NS_E_NO_SCRIPT_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882829i32 as _);
pub const NS_E_NO_SERVER_CONTACT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889650i32 as _);
pub const NS_E_NO_SMPTE_WITH_MULTIPLE_SOURCEGROUPS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882775i32 as _);
pub const NS_E_NO_SPECIFIED_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889742i32 as _);
pub const NS_E_NO_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889805i32 as _);
pub const NS_E_NO_TWOPASS_TIMECOMPRESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882728i32 as _);
pub const NS_E_NO_VALID_OUTPUT_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882832i32 as _);
pub const NS_E_NO_VALID_SOURCE_PLUGIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882831i32 as _);
pub const NS_E_NUM_LANGUAGE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882789i32 as _);
pub const NS_E_OFFLINE_MODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886838i32 as _);
pub const NS_E_OPEN_CONTAINING_FOLDER_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884893i32 as _);
pub const NS_E_OPEN_FILE_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889807i32 as _);
pub const NS_E_OUTPUT_PROTECTION_LEVEL_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879356i32 as _);
pub const NS_E_OUTPUT_PROTECTION_SCHEME_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879350i32 as _);
pub const NS_E_PACKETSINK_UNKNOWN_FEC_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877814i32 as _);
pub const NS_E_PAGING_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889758i32 as _);
pub const NS_E_PARTIALLY_REBUILT_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889753i32 as _);
pub const NS_E_PDA_CANNOT_CREATE_ADDITIONAL_SYNC_RELATIONSHIP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885371i32 as _);
pub const NS_E_PDA_CANNOT_SYNC_FROM_INTERNET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885196i32 as _);
pub const NS_E_PDA_CANNOT_SYNC_FROM_LOCATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885357i32 as _);
pub const NS_E_PDA_CANNOT_SYNC_INVALID_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885195i32 as _);
pub const NS_E_PDA_CANNOT_TRANSCODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885367i32 as _);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_AUDIO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885187i32 as _);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885185i32 as _);
pub const NS_E_PDA_CANNOT_TRANSCODE_TO_VIDEO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885186i32 as _);
pub const NS_E_PDA_CEWMDM_DRM_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885183i32 as _);
pub const NS_E_PDA_DELETE_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885192i32 as _);
pub const NS_E_PDA_DEVICESUPPORTDISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885360i32 as _);
pub const NS_E_PDA_DEVICE_FULL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885377i32 as _);
pub const NS_E_PDA_DEVICE_FULL_IN_SESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885375i32 as _);
pub const NS_E_PDA_DEVICE_NOT_RESPONDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885190i32 as _);
pub const NS_E_PDA_ENCODER_NOT_RESPONDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885358i32 as _);
pub const NS_E_PDA_FAILED_TO_BURN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885542i32 as _);
pub const NS_E_PDA_FAILED_TO_ENCRYPT_TRANSCODED_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885188i32 as _);
pub const NS_E_PDA_FAILED_TO_RETRIEVE_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885191i32 as _);
pub const NS_E_PDA_FAILED_TO_SYNCHRONIZE_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885194i32 as _);
pub const NS_E_PDA_FAILED_TO_TRANSCODE_PHOTO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885189i32 as _);
pub const NS_E_PDA_FAIL_READ_WAVE_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885379i32 as _);
pub const NS_E_PDA_FAIL_SELECT_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885380i32 as _);
pub const NS_E_PDA_INITIALIZINGDEVICES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885363i32 as _);
pub const NS_E_PDA_MANUALDEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885373i32 as _);
pub const NS_E_PDA_NO_LONGER_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885359i32 as _);
pub const NS_E_PDA_NO_TRANSCODE_OF_DRM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885370i32 as _);
pub const NS_E_PDA_OBSOLETE_SP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885362i32 as _);
pub const NS_E_PDA_PARTNERSHIPNOTEXIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885372i32 as _);
pub const NS_E_PDA_RETRIEVED_FILE_FILENAME_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885184i32 as _);
pub const NS_E_PDA_SYNC_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885193i32 as _);
pub const NS_E_PDA_SYNC_LOGIN_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885180i32 as _);
pub const NS_E_PDA_SYNC_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885181i32 as _);
pub const NS_E_PDA_TITLE_COLLISION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885361i32 as _);
pub const NS_E_PDA_TOO_MANY_FILES_IN_DIRECTORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885366i32 as _);
pub const NS_E_PDA_TOO_MANY_FILE_COLLISIONS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885368i32 as _);
pub const NS_E_PDA_TRANSCODECACHEFULL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885369i32 as _);
pub const NS_E_PDA_TRANSCODE_CODEC_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885179i32 as _);
pub const NS_E_PDA_TRANSCODE_NOT_PERMITTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885364i32 as _);
pub const NS_E_PDA_UNSPECIFIED_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885382i32 as _);
pub const NS_E_PDA_UNSUPPORTED_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885384i32 as _);
pub const NS_E_PLAYLIST_CONTAINS_ERRORS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885569i32 as _);
pub const NS_E_PLAYLIST_END_RECEDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884547i32 as _);
pub const NS_E_PLAYLIST_ENTRY_ALREADY_PLAYING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884556i32 as _);
pub const NS_E_PLAYLIST_ENTRY_HAS_CHANGED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877835i32 as _);
pub const NS_E_PLAYLIST_ENTRY_NOT_IN_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884552i32 as _);
pub const NS_E_PLAYLIST_ENTRY_SEEK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884551i32 as _);
pub const NS_E_PLAYLIST_PARSE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884554i32 as _);
pub const NS_E_PLAYLIST_PLUGIN_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884353i32 as _);
pub const NS_E_PLAYLIST_RECURSIVE_PLAYLISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884550i32 as _);
pub const NS_E_PLAYLIST_SHUTDOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884548i32 as _);
pub const NS_E_PLAYLIST_TOO_MANY_NESTED_PLAYLISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884549i32 as _);
pub const NS_E_PLAYLIST_UNSUPPORTED_ENTRY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884553i32 as _);
pub const NS_E_PLUGIN_CLSID_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882826i32 as _);
pub const NS_E_PLUGIN_ERROR_REPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884355i32 as _);
pub const NS_E_PLUGIN_NOTSHUTDOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885802i32 as _);
pub const NS_E_PORT_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884342i32 as _);
pub const NS_E_PORT_IN_USE_HTTP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884341i32 as _);
pub const NS_E_PROCESSINGSHOWSYNCWIZARD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885365i32 as _);
pub const NS_E_PROFILE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882821i32 as _);
pub const NS_E_PROPERTY_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876854i32 as _);
pub const NS_E_PROPERTY_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876846i32 as _);
pub const NS_E_PROPERTY_READ_ONLY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876852i32 as _);
pub const NS_E_PROTECTED_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886851i32 as _);
pub const NS_E_PROTOCOL_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889838i32 as _);
pub const NS_E_PROXY_ACCESSDENIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877834i32 as _);
pub const NS_E_PROXY_CONNECT_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877817i32 as _);
pub const NS_E_PROXY_DNS_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877840i32 as _);
pub const NS_E_PROXY_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877843i32 as _);
pub const NS_E_PROXY_SOURCE_ACCESSDENIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877833i32 as _);
pub const NS_E_PROXY_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877851i32 as _);
pub const NS_E_PUBLISHING_POINT_INVALID_REQUEST_WHILE_STARTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884649i32 as _);
pub const NS_E_PUBLISHING_POINT_REMOVED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884646i32 as _);
pub const NS_E_PUBLISHING_POINT_STOPPED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884642i32 as _);
pub const NS_E_PUSH_CANNOTCONNECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877813i32 as _);
pub const NS_E_PUSH_DUPLICATE_PUBLISHING_POINT_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884448i32 as _);
pub const NS_E_REBOOT_RECOMMENDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878854i32 as _);
pub const NS_E_REBOOT_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878853i32 as _);
pub const NS_E_RECORDQ_DISK_FULL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882781i32 as _);
pub const NS_E_REDBOOK_ENABLED_WHILE_COPYING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885840i32 as _);
pub const NS_E_REDIRECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884856i32 as _);
pub const NS_E_REDIRECT_TO_PROXY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877855i32 as _);
pub const NS_E_REFUSED_BY_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877849i32 as _);
pub const NS_E_REG_FLUSH_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072879720i32 as _);
pub const NS_E_REMIRRORED_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889655i32 as _);
pub const NS_E_REQUIRE_STREAMING_CLIENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877836i32 as _);
pub const NS_E_RESET_SOCKET_CONNECTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877824i32 as _);
pub const NS_E_RESOURCE_GONE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877828i32 as _);
pub const NS_E_SAME_AS_INPUT_COMBINATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882734i32 as _);
pub const NS_E_SCHEMA_CLASSIFY_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876844i32 as _);
pub const NS_E_SCRIPT_DEBUGGER_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884350i32 as _);
pub const NS_E_SDK_BUFFERTOOSMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886828i32 as _);
pub const NS_E_SERVER_ACCESSDENIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877829i32 as _);
pub const NS_E_SERVER_DNS_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877841i32 as _);
pub const NS_E_SERVER_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889803i32 as _);
pub const NS_E_SERVER_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877850i32 as _);
pub const NS_E_SESSION_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877816i32 as _);
pub const NS_E_SESSION_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877837i32 as _);
pub const NS_E_SETUP_BLOCKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878848i32 as _);
pub const NS_E_SETUP_DRM_MIGRATION_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878851i32 as _);
pub const NS_E_SETUP_DRM_MIGRATION_FAILED_AND_IGNORABLE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878849i32 as _);
pub const NS_E_SETUP_IGNORABLE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878850i32 as _);
pub const NS_E_SETUP_INCOMPLETE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072878852i32 as _);
pub const NS_E_SET_DISK_UID_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889823i32 as _);
pub const NS_E_SHARING_STATE_OUT_OF_SYNC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885772i32 as _);
pub const NS_E_SHARING_VIOLATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885809i32 as _);
pub const NS_E_SHUTDOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889814i32 as _);
pub const NS_E_SLOW_READ_DIGITAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885852i32 as _);
pub const NS_E_SLOW_READ_DIGITAL_WITH_ERRORCORRECTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885251i32 as _);
pub const NS_E_SMPTEMODE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882771i32 as _);
pub const NS_E_SOURCEGROUP_NOTPREPARED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882822i32 as _);
pub const NS_E_SOURCE_CANNOT_LOOP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882733i32 as _);
pub const NS_E_SOURCE_NOTSPECIFIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882811i32 as _);
pub const NS_E_SOURCE_PLUGIN_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884354i32 as _);
pub const NS_E_SPEECHEDL_ON_NON_MIXEDMODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882798i32 as _);
pub const NS_E_STALE_PRESENTATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884855i32 as _);
pub const NS_E_STREAM_END: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889804i32 as _);
pub const NS_E_STRIDE_REFUSED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889787i32 as _);
pub const NS_E_SUBSCRIPTIONSERVICE_DOWNLOAD_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884896i32 as _);
pub const NS_E_SUBSCRIPTIONSERVICE_LOGIN_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884897i32 as _);
pub const NS_E_SUBSCRIPTIONSERVICE_PLAYBACK_DISALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884906i32 as _);
pub const NS_E_SYNCWIZ_CANNOT_CHANGE_SETTINGS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885265i32 as _);
pub const NS_E_SYNCWIZ_DEVICE_FULL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885266i32 as _);
pub const NS_E_TABLE_KEY_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876851i32 as _);
pub const NS_E_TAMPERED_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886849i32 as _);
pub const NS_E_TCP_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889646i32 as _);
pub const NS_E_TIGER_FAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889776i32 as _);
pub const NS_E_TIMECODE_REQUIRES_VIDEOSTREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882727i32 as _);
pub const NS_E_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889837i32 as _);
pub const NS_E_TITLE_BITRATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889643i32 as _);
pub const NS_E_TITLE_SIZE_EXCEEDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889648i32 as _);
pub const NS_E_TOO_MANY_AUDIO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882852i32 as _);
pub const NS_E_TOO_MANY_DEVICECONTROL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882794i32 as _);
pub const NS_E_TOO_MANY_HOPS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877822i32 as _);
pub const NS_E_TOO_MANY_MULTICAST_SINKS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884650i32 as _);
pub const NS_E_TOO_MANY_SESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889841i32 as _);
pub const NS_E_TOO_MANY_TITLES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889649i32 as _);
pub const NS_E_TOO_MANY_VIDEO: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882851i32 as _);
pub const NS_E_TOO_MUCH_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886836i32 as _);
pub const NS_E_TOO_MUCH_DATA_FROM_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877819i32 as _);
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_ALBUM_PURCHASE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884901i32 as _);
pub const NS_E_TRACK_DOWNLOAD_REQUIRES_PURCHASE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884900i32 as _);
pub const NS_E_TRACK_PURCHASE_MAXIMUM_EXCEEDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884899i32 as _);
pub const NS_E_TRANSCODE_DELETECACHEERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885264i32 as _);
pub const NS_E_TRANSFORM_PLUGIN_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882714i32 as _);
pub const NS_E_TRANSFORM_PLUGIN_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882715i32 as _);
pub const NS_E_UDP_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889647i32 as _);
pub const NS_E_UNABLE_TO_CREATE_RIP_LOCATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885552i32 as _);
pub const NS_E_UNCOMPRESSED_DIGITAL_AUDIO_PROTECTION_LEVEL_UNSUPPORTED:
    ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1072879351i32 as _);
pub const NS_E_UNCOMPRESSED_DIGITAL_VIDEO_PROTECTION_LEVEL_UNSUPPORTED:
    ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-1072879354i32 as _);
pub const NS_E_UNCOMP_COMP_COMBINATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882762i32 as _);
pub const NS_E_UNEXPECTED_DISPLAY_SETTINGS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882808i32 as _);
pub const NS_E_UNEXPECTED_MSAUDIO_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886854i32 as _);
pub const NS_E_UNKNOWN_PROTOCOL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072877856i32 as _);
pub const NS_E_UNRECOGNIZED_STREAM_TYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889818i32 as _);
pub const NS_E_UNSUPPORTED_ARCHIVEOPERATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882824i32 as _);
pub const NS_E_UNSUPPORTED_ARCHIVETYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882825i32 as _);
pub const NS_E_UNSUPPORTED_ENCODER_DEVICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882809i32 as _);
pub const NS_E_UNSUPPORTED_LANGUAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884644i32 as _);
pub const NS_E_UNSUPPORTED_LOAD_TYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884653i32 as _);
pub const NS_E_UNSUPPORTED_PROPERTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886835i32 as _);
pub const NS_E_UNSUPPORTED_SOURCETYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882853i32 as _);
pub const NS_E_URLLIST_INVALIDFORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885651i32 as _);
pub const NS_E_USER_STOP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885847i32 as _);
pub const NS_E_USE_FILE_SOURCE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072876855i32 as _);
pub const NS_E_VBRMODE_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882787i32 as _);
pub const NS_E_VIDCAPCREATEWINDOW: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882835i32 as _);
pub const NS_E_VIDCAPDRVINUSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882834i32 as _);
pub const NS_E_VIDCAPSTARTFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882839i32 as _);
pub const NS_E_VIDEODEVICE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882844i32 as _);
pub const NS_E_VIDEODEVICE_UNEXPECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882843i32 as _);
pub const NS_E_VIDEODRIVER_UNSTABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882840i32 as _);
pub const NS_E_VIDEO_BITRATE_STEPDOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882752i32 as _);
pub const NS_E_VIDEO_CODEC_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886843i32 as _);
pub const NS_E_VIDEO_CODEC_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886844i32 as _);
pub const NS_E_VIDSOURCECOMPRESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882838i32 as _);
pub const NS_E_VIDSOURCESIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882837i32 as _);
pub const NS_E_WALKER_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889779i32 as _);
pub const NS_E_WALKER_UNKNOWN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889780i32 as _);
pub const NS_E_WALKER_USAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889778i32 as _);
pub const NS_E_WAVE_OPEN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072889747i32 as _);
pub const NS_E_WINSOCK_ERROR_STRING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885463i32 as _);
pub const NS_E_WIZARD_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884348i32 as _);
pub const NS_E_WMDM_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885572i32 as _);
pub const NS_E_WMDRM_DEPRECATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072886818i32 as _);
pub const NS_E_WME_VERSION_MISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072882805i32 as _);
pub const NS_E_WMG_CANNOTQUEUE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885684i32 as _);
pub const NS_E_WMG_COPP_SECURITY_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885678i32 as _);
pub const NS_E_WMG_COPP_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885677i32 as _);
pub const NS_E_WMG_FILETRANSFERNOTALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885672i32 as _);
pub const NS_E_WMG_INVALIDSTATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885676i32 as _);
pub const NS_E_WMG_INVALID_COPP_CERTIFICATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885679i32 as _);
pub const NS_E_WMG_LICENSE_TAMPERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885660i32 as _);
pub const NS_E_WMG_NOSDKINTERFACE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885674i32 as _);
pub const NS_E_WMG_NOTALLOUTPUTSRENDERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885673i32 as _);
pub const NS_E_WMG_PLUGINUNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885685i32 as _);
pub const NS_E_WMG_PREROLLLICENSEACQUISITIONNOTALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885683i32 as _);
pub const NS_E_WMG_RATEUNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885686i32 as _);
pub const NS_E_WMG_SINKALREADYEXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885675i32 as _);
pub const NS_E_WMG_UNEXPECTEDPREROLLSTATUS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885682i32 as _);
pub const NS_E_WMPBR_BACKUPCANCEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885455i32 as _);
pub const NS_E_WMPBR_BACKUPRESTOREFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885448i32 as _);
pub const NS_E_WMPBR_DRIVE_INVALID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885449i32 as _);
pub const NS_E_WMPBR_ERRORWITHURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885453i32 as _);
pub const NS_E_WMPBR_NAMECOLLISION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885452i32 as _);
pub const NS_E_WMPBR_NOLISTENER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885456i32 as _);
pub const NS_E_WMPBR_RESTORECANCEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885454i32 as _);
pub const NS_E_WMPCORE_BUFFERTOOSMALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885633i32 as _);
pub const NS_E_WMPCORE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885577i32 as _);
pub const NS_E_WMPCORE_COCREATEFAILEDFORGITOBJECT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885635i32 as _);
pub const NS_E_WMPCORE_CODEC_DOWNLOAD_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885604i32 as _);
pub const NS_E_WMPCORE_CODEC_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885605i32 as _);
pub const NS_E_WMPCORE_CODEC_NOT_TRUSTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885606i32 as _);
pub const NS_E_WMPCORE_CURRENT_MEDIA_NOT_ACTIVE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885591i32 as _);
pub const NS_E_WMPCORE_DEVICE_DRIVERS_MISSING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885539i32 as _);
pub const NS_E_WMPCORE_ERRORMANAGERNOTAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885619i32 as _);
pub const NS_E_WMPCORE_ERRORSINKNOTREGISTERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885620i32 as _);
pub const NS_E_WMPCORE_ERROR_DOWNLOADING_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885603i32 as _);
pub const NS_E_WMPCORE_FAILEDTOGETMARSHALLEDEVENTHANDLERINTERFACE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885634i32 as _);
pub const NS_E_WMPCORE_FAILED_TO_BUILD_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885602i32 as _);
pub const NS_E_WMPCORE_FILE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885574i32 as _);
pub const NS_E_WMPCORE_GRAPH_NOT_IN_LIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885622i32 as _);
pub const NS_E_WMPCORE_INVALIDPLAYLISTMODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885631i32 as _);
pub const NS_E_WMPCORE_INVALID_PLAYLIST_URL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885585i32 as _);
pub const NS_E_WMPCORE_ITEMNOTINPLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885626i32 as _);
pub const NS_E_WMPCORE_LIST_ENTRY_NO_REF: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885608i32 as _);
pub const NS_E_WMPCORE_MEDIA_ALTERNATE_REF_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885596i32 as _);
pub const NS_E_WMPCORE_MEDIA_CHILD_PLAYLIST_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885576i32 as _);
pub const NS_E_WMPCORE_MEDIA_ERROR_RESUME_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885617i32 as _);
pub const NS_E_WMPCORE_MEDIA_NO_CHILD_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885575i32 as _);
pub const NS_E_WMPCORE_MEDIA_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885581i32 as _);
pub const NS_E_WMPCORE_MEDIA_URL_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885560i32 as _);
pub const NS_E_WMPCORE_MISMATCHED_RUNTIME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885584i32 as _);
pub const NS_E_WMPCORE_MISNAMED_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885607i32 as _);
pub const NS_E_WMPCORE_NOBROWSER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885624i32 as _);
pub const NS_E_WMPCORE_NOSOURCEURLSTRING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885636i32 as _);
pub const NS_E_WMPCORE_NO_PLAYABLE_MEDIA_IN_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885579i32 as _);
pub const NS_E_WMPCORE_NO_REF_IN_ENTRY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885616i32 as _);
pub const NS_E_WMPCORE_PLAYLISTEMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885625i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_NESTED_PLAYLIST_SKIPPED_ITEMS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885578i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_EMPTY_OR_SINGLE_MEDIA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885621i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_EVENT_ATTRIBUTE_ABSENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885594i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_EVENT_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885593i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_IMPORT_FAILED_NO_ITEMS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885583i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_EXHAUSTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885600i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_INIT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885597i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_MORPH_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885598i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NAME_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885599i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_ITEM_ALTERNATE_NONE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885601i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_NO_EVENT_NAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885595i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885588i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_END_MEDIA_NONE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885586i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_REPEAT_START_MEDIA_NONE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885587i32 as _);
pub const NS_E_WMPCORE_PLAYLIST_STACK_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885592i32 as _);
pub const NS_E_WMPCORE_SOME_CODECS_MISSING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885551i32 as _);
pub const NS_E_WMPCORE_TEMP_FILE_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885573i32 as _);
pub const NS_E_WMPCORE_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885632i32 as _);
pub const NS_E_WMPCORE_UNRECOGNIZED_MEDIA_URL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885623i32 as _);
pub const NS_E_WMPCORE_USER_CANCEL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885589i32 as _);
pub const NS_E_WMPCORE_VIDEO_TRANSFORM_FILTER_INSERTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885582i32 as _);
pub const NS_E_WMPCORE_WEBHELPFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885618i32 as _);
pub const NS_E_WMPCORE_WMX_ENTRYREF_NO_REF: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885580i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885615i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_NAME_ILLEGAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885614i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885613i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ATTRIBUTE_VALUE_ILLEGAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885612i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885611i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_NAME_ILLEGAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885610i32 as _);
pub const NS_E_WMPCORE_WMX_LIST_ITEM_ATTRIBUTE_VALUE_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885609i32 as _);
pub const NS_E_WMPFLASH_CANT_FIND_COM_SERVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885559i32 as _);
pub const NS_E_WMPFLASH_INCOMPATIBLEVERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885558i32 as _);
pub const NS_E_WMPIM_DIALUPFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885464i32 as _);
pub const NS_E_WMPIM_USERCANCELED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885465i32 as _);
pub const NS_E_WMPIM_USEROFFLINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885466i32 as _);
pub const NS_E_WMPOCXGRAPH_IE_DISALLOWS_ACTIVEX_CONTROLS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885557i32 as _);
pub const NS_E_WMPOCX_ERRORMANAGERNOTAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885803i32 as _);
pub const NS_E_WMPOCX_NOT_RUNNING_REMOTELY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885805i32 as _);
pub const NS_E_WMPOCX_NO_ACTIVE_CORE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885806i32 as _);
pub const NS_E_WMPOCX_NO_REMOTE_CORE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885807i32 as _);
pub const NS_E_WMPOCX_NO_REMOTE_WINDOW: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885804i32 as _);
pub const NS_E_WMPOCX_PLAYER_NOT_DOCKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885797i32 as _);
pub const NS_E_WMPOCX_REMOTE_PLAYER_ALREADY_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885766i32 as _);
pub const NS_E_WMPOCX_UNABLE_TO_LOAD_SKIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885781i32 as _);
pub const NS_E_WMPXML_ATTRIBUTENOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885833i32 as _);
pub const NS_E_WMPXML_EMPTYDOC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885831i32 as _);
pub const NS_E_WMPXML_ENDOFDATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885835i32 as _);
pub const NS_E_WMPXML_NOERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885836i32 as _);
pub const NS_E_WMPXML_PARSEERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885834i32 as _);
pub const NS_E_WMPXML_PINOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885832i32 as _);
pub const NS_E_WMPZIP_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885735i32 as _);
pub const NS_E_WMPZIP_FILENOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885734i32 as _);
pub const NS_E_WMPZIP_NOTAZIPFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885736i32 as _);
pub const NS_E_WMP_ACCESS_DENIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885294i32 as _);
pub const NS_E_WMP_ADDTOLIBRARY_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885817i32 as _);
pub const NS_E_WMP_ALREADY_IN_USE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885346i32 as _);
pub const NS_E_WMP_AUDIO_CODEC_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885305i32 as _);
pub const NS_E_WMP_AUDIO_DEVICE_LOST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885275i32 as _);
pub const NS_E_WMP_AUDIO_HW_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885318i32 as _);
pub const NS_E_WMP_AUTOPLAY_INVALID_STATE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884996i32 as _);
pub const NS_E_WMP_BAD_DRIVER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885295i32 as _);
pub const NS_E_WMP_BMP_BITMAP_NOT_CREATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885712i32 as _);
pub const NS_E_WMP_BMP_COMPRESSION_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885711i32 as _);
pub const NS_E_WMP_BMP_INVALID_BITMASK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885714i32 as _);
pub const NS_E_WMP_BMP_INVALID_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885710i32 as _);
pub const NS_E_WMP_BMP_TOPDOWN_DIB_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885713i32 as _);
pub const NS_E_WMP_BSTR_TOO_LONG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885006i32 as _);
pub const NS_E_WMP_BURN_DISC_OVERFLOW: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885287i32 as _);
pub const NS_E_WMP_CANNOT_BURN_NON_LOCAL_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885546i32 as _);
pub const NS_E_WMP_CANNOT_FIND_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885353i32 as _);
pub const NS_E_WMP_CANNOT_FIND_FOLDER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885801i32 as _);
pub const NS_E_WMP_CANT_PLAY_PROTECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885773i32 as _);
pub const NS_E_WMP_CD_ANOTHER_USER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885297i32 as _);
pub const NS_E_WMP_CD_STASH_NO_SPACE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885291i32 as _);
pub const NS_E_WMP_CODEC_NEEDED_WITH_4CC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885343i32 as _);
pub const NS_E_WMP_CODEC_NEEDED_WITH_FORMATTAG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885342i32 as _);
pub const NS_E_WMP_COMPONENT_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884986i32 as _);
pub const NS_E_WMP_CONNECT_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885311i32 as _);
pub const NS_E_WMP_CONVERT_FILE_CORRUPT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885413i32 as _);
pub const NS_E_WMP_CONVERT_FILE_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885416i32 as _);
pub const NS_E_WMP_CONVERT_NO_RIGHTS_ERRORURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885415i32 as _);
pub const NS_E_WMP_CONVERT_NO_RIGHTS_NOERRORURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885414i32 as _);
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_ERRORURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885412i32 as _);
pub const NS_E_WMP_CONVERT_PLUGIN_UNAVAILABLE_NOERRORURL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885411i32 as _);
pub const NS_E_WMP_CONVERT_PLUGIN_UNKNOWN_FILE_OWNER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885410i32 as _);
pub const NS_E_WMP_CS_JPGPOSITIONIMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885746i32 as _);
pub const NS_E_WMP_CS_NOTEVENLYDIVISIBLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885745i32 as _);
pub const NS_E_WMP_DAI_SONGTOOSHORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885687i32 as _);
pub const NS_E_WMP_DRM_ACQUIRING_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885246i32 as _);
pub const NS_E_WMP_DRM_CANNOT_RESTORE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885288i32 as _);
pub const NS_E_WMP_DRM_COMPONENT_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885278i32 as _);
pub const NS_E_WMP_DRM_CORRUPT_BACKUP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885324i32 as _);
pub const NS_E_WMP_DRM_DRIVER_AUTH_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885302i32 as _);
pub const NS_E_WMP_DRM_GENERIC_LICENSE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885286i32 as _);
pub const NS_E_WMP_DRM_INDIV_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885283i32 as _);
pub const NS_E_WMP_DRM_INVALID_SIG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885289i32 as _);
pub const NS_E_WMP_DRM_LICENSE_CONTENT_REVOKED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885241i32 as _);
pub const NS_E_WMP_DRM_LICENSE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885245i32 as _);
pub const NS_E_WMP_DRM_LICENSE_NOSAP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885240i32 as _);
pub const NS_E_WMP_DRM_LICENSE_NOTACQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885244i32 as _);
pub const NS_E_WMP_DRM_LICENSE_NOTENABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885243i32 as _);
pub const NS_E_WMP_DRM_LICENSE_SERVER_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885323i32 as _);
pub const NS_E_WMP_DRM_LICENSE_UNUSABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885242i32 as _);
pub const NS_E_WMP_DRM_NEEDS_AUTHORIZATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885296i32 as _);
pub const NS_E_WMP_DRM_NEW_HARDWARE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885290i32 as _);
pub const NS_E_WMP_DRM_NOT_ACQUIRING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885055i32 as _);
pub const NS_E_WMP_DRM_NO_DEVICE_CERT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885277i32 as _);
pub const NS_E_WMP_DRM_NO_RIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885284i32 as _);
pub const NS_E_WMP_DRM_NO_SECURE_CLOCK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885285i32 as _);
pub const NS_E_WMP_DRM_UNABLE_TO_ACQUIRE_LICENSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885239i32 as _);
pub const NS_E_WMP_DSHOW_UNSUPPORTED_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885350i32 as _);
pub const NS_E_WMP_ERASE_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885548i32 as _);
pub const NS_E_WMP_EXTERNAL_NOTREADY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885796i32 as _);
pub const NS_E_WMP_FAILED_TO_OPEN_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885692i32 as _);
pub const NS_E_WMP_FAILED_TO_OPEN_WMD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885774i32 as _);
pub const NS_E_WMP_FAILED_TO_RIP_TRACK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885549i32 as _);
pub const NS_E_WMP_FAILED_TO_SAVE_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885777i32 as _);
pub const NS_E_WMP_FAILED_TO_SAVE_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885775i32 as _);
pub const NS_E_WMP_FILESCANALREADYSTARTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885826i32 as _);
pub const NS_E_WMP_FILE_DOES_NOT_FIT_ON_CD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885544i32 as _);
pub const NS_E_WMP_FILE_NO_DURATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885543i32 as _);
pub const NS_E_WMP_FILE_OPEN_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885327i32 as _);
pub const NS_E_WMP_FILE_TYPE_CANNOT_BURN_TO_AUDIO_CD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885545i32 as _);
pub const NS_E_WMP_FORMAT_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885547i32 as _);
pub const NS_E_WMP_GIF_BAD_VERSION_NUMBER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885722i32 as _);
pub const NS_E_WMP_GIF_INVALID_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885723i32 as _);
pub const NS_E_WMP_GIF_NO_IMAGE_IN_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885721i32 as _);
pub const NS_E_WMP_GIF_UNEXPECTED_ENDOFFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885724i32 as _);
pub const NS_E_WMP_GOFULLSCREEN_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885313i32 as _);
pub const NS_E_WMP_HME_INVALIDOBJECTID: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885825i32 as _);
pub const NS_E_WMP_HME_NOTSEARCHABLEFORITEMS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885823i32 as _);
pub const NS_E_WMP_HME_STALEREQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885822i32 as _);
pub const NS_E_WMP_HWND_NOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885156i32 as _);
pub const NS_E_WMP_IMAGE_FILETYPE_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885726i32 as _);
pub const NS_E_WMP_IMAGE_INVALID_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885725i32 as _);
pub const NS_E_WMP_IMAPI2_ERASE_DEVICE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885279i32 as _);
pub const NS_E_WMP_IMAPI2_ERASE_FAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885280i32 as _);
pub const NS_E_WMP_IMAPI_DEVICE_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885330i32 as _);
pub const NS_E_WMP_IMAPI_DEVICE_INVALIDTYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885303i32 as _);
pub const NS_E_WMP_IMAPI_DEVICE_NOTPRESENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885331i32 as _);
pub const NS_E_WMP_IMAPI_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885345i32 as _);
pub const NS_E_WMP_IMAPI_GENERIC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885333i32 as _);
pub const NS_E_WMP_IMAPI_LOSS_OF_STREAMING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885329i32 as _);
pub const NS_E_WMP_IMAPI_MEDIA_INCOMPATIBLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885274i32 as _);
pub const NS_E_WMP_INVALID_ASX: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885347i32 as _);
pub const NS_E_WMP_INVALID_KEY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885298i32 as _);
pub const NS_E_WMP_INVALID_LIBRARY_ADD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885316i32 as _);
pub const NS_E_WMP_INVALID_MAX_VAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885751i32 as _);
pub const NS_E_WMP_INVALID_MIN_VAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885750i32 as _);
pub const NS_E_WMP_INVALID_PROTOCOL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885317i32 as _);
pub const NS_E_WMP_INVALID_REQUEST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885292i32 as _);
pub const NS_E_WMP_INVALID_SKIN: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885780i32 as _);
pub const NS_E_WMP_JPGTRANSPARENCY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885755i32 as _);
pub const NS_E_WMP_JPG_BAD_DCTSIZE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885707i32 as _);
pub const NS_E_WMP_JPG_BAD_PRECISION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885705i32 as _);
pub const NS_E_WMP_JPG_BAD_VERSION_NUMBER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885706i32 as _);
pub const NS_E_WMP_JPG_CCIR601_NOTIMPL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885704i32 as _);
pub const NS_E_WMP_JPG_FRACT_SAMPLE_NOTIMPL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885701i32 as _);
pub const NS_E_WMP_JPG_IMAGE_TOO_BIG: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885700i32 as _);
pub const NS_E_WMP_JPG_INVALID_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885708i32 as _);
pub const NS_E_WMP_JPG_JERR_ARITHCODING_NOTIMPL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885709i32 as _);
pub const NS_E_WMP_JPG_NO_IMAGE_IN_FILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885703i32 as _);
pub const NS_E_WMP_JPG_READ_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885702i32 as _);
pub const NS_E_WMP_JPG_SOF_UNSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885698i32 as _);
pub const NS_E_WMP_JPG_UNEXPECTED_ENDOFFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885699i32 as _);
pub const NS_E_WMP_JPG_UNKNOWN_MARKER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885697i32 as _);
pub const NS_E_WMP_LICENSE_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885238i32 as _);
pub const NS_E_WMP_LICENSE_RESTRICTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885293i32 as _);
pub const NS_E_WMP_LOCKEDINSKINMODE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885778i32 as _);
pub const NS_E_WMP_LOGON_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885354i32 as _);
pub const NS_E_WMP_MF_CODE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885824i32 as _);
pub const NS_E_WMP_MLS_STALE_DATA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885795i32 as _);
pub const NS_E_WMP_MMS_NOT_SUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885315i32 as _);
pub const NS_E_WMP_MSSAP_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885341i32 as _);
pub const NS_E_WMP_MULTICAST_DISABLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885310i32 as _);
pub const NS_E_WMP_MULTIPLE_ERROR_IN_PLAYLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885281i32 as _);
pub const NS_E_WMP_NEED_UPGRADE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885319i32 as _);
pub const NS_E_WMP_NETWORK_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885312i32 as _);
pub const NS_E_WMP_NETWORK_FIREWALL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885322i32 as _);
pub const NS_E_WMP_NETWORK_RESOURCE_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885301i32 as _);
pub const NS_E_WMP_NONMEDIA_FILES: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885348i32 as _);
pub const NS_E_WMP_NO_DISK_SPACE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885355i32 as _);
pub const NS_E_WMP_NO_PROTOCOLS_SELECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885314i32 as _);
pub const NS_E_WMP_NO_REMOVABLE_MEDIA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885321i32 as _);
pub const NS_E_WMP_OUTOFMEMORY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885306i32 as _);
pub const NS_E_WMP_PATH_ALREADY_IN_LIBRARY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885830i32 as _);
pub const NS_E_WMP_PLAYLIST_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885349i32 as _);
pub const NS_E_WMP_PLUGINDLL_NOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885799i32 as _);
pub const NS_E_WMP_PNG_INVALIDFORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885720i32 as _);
pub const NS_E_WMP_PNG_UNSUPPORTED_BAD_CRC: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885715i32 as _);
pub const NS_E_WMP_PNG_UNSUPPORTED_BITDEPTH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885719i32 as _);
pub const NS_E_WMP_PNG_UNSUPPORTED_COMPRESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885718i32 as _);
pub const NS_E_WMP_PNG_UNSUPPORTED_FILTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885717i32 as _);
pub const NS_E_WMP_PNG_UNSUPPORTED_INTERLACE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885716i32 as _);
pub const NS_E_WMP_POLICY_VALUE_NOT_CONFIGURED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885206i32 as _);
pub const NS_E_WMP_PROTECTED_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885237i32 as _);
pub const NS_E_WMP_PROTOCOL_PROBLEM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885356i32 as _);
pub const NS_E_WMP_PROXY_CONNECT_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885320i32 as _);
pub const NS_E_WMP_PROXY_NOT_FOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885308i32 as _);
pub const NS_E_WMP_RBC_JPGMAPPINGIMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885756i32 as _);
pub const NS_E_WMP_RECORDING_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885815i32 as _);
pub const NS_E_WMP_RIP_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885550i32 as _);
pub const NS_E_WMP_SAVEAS_READONLY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885776i32 as _);
pub const NS_E_WMP_SENDMAILFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885779i32 as _);
pub const NS_E_WMP_SERVER_DNS_TIMEOUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885309i32 as _);
pub const NS_E_WMP_SERVER_INACCESSIBLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885352i32 as _);
pub const NS_E_WMP_SERVER_NONEWCONNECTIONS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885282i32 as _);
pub const NS_E_WMP_SERVER_NOT_RESPONDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885325i32 as _);
pub const NS_E_WMP_SERVER_SECURITY_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885276i32 as _);
pub const NS_E_WMP_SERVER_UNAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885328i32 as _);
pub const NS_E_WMP_STREAMING_RECORDING_NOT_ALLOWED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885800i32 as _);
pub const NS_E_WMP_TAMPERED_CONTENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885307i32 as _);
pub const NS_E_WMP_UDRM_NOUSERLIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885056i32 as _);
pub const NS_E_WMP_UI_NOSKININZIP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885785i32 as _);
pub const NS_E_WMP_UI_NOTATHEMEFILE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885792i32 as _);
pub const NS_E_WMP_UI_OBJECTNOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885787i32 as _);
pub const NS_E_WMP_UI_PASSTHROUGH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885788i32 as _);
pub const NS_E_WMP_UI_SECONDHANDLER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885786i32 as _);
pub const NS_E_WMP_UI_SUBCONTROLSNOTSUPPORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885794i32 as _);
pub const NS_E_WMP_UI_SUBELEMENTNOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885791i32 as _);
pub const NS_E_WMP_UI_VERSIONMISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885793i32 as _);
pub const NS_E_WMP_UI_VERSIONPARSE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885790i32 as _);
pub const NS_E_WMP_UI_VIEWIDNOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885789i32 as _);
pub const NS_E_WMP_UNKNOWN_ERROR: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885299i32 as _);
pub const NS_E_WMP_UNSUPPORTED_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885351i32 as _);
pub const NS_E_WMP_UPGRADE_APPLICATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885300i32 as _);
pub const NS_E_WMP_URLDOWNLOADFAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885782i32 as _);
pub const NS_E_WMP_VERIFY_ONLINE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885326i32 as _);
pub const NS_E_WMP_VIDEO_CODEC_NOT_INSTALLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885304i32 as _);
pub const NS_E_WMP_WINDOWSAPIFAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885816i32 as _);
pub const NS_E_WMP_WMDM_BUSY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885336i32 as _);
pub const NS_E_WMP_WMDM_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885344i32 as _);
pub const NS_E_WMP_WMDM_INCORRECT_RIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885334i32 as _);
pub const NS_E_WMP_WMDM_INTERFACEDEAD: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885340i32 as _);
pub const NS_E_WMP_WMDM_LICENSE_EXPIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885337i32 as _);
pub const NS_E_WMP_WMDM_LICENSE_NOTEXIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885338i32 as _);
pub const NS_E_WMP_WMDM_NORIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885335i32 as _);
pub const NS_E_WMP_WMDM_NOTCERTIFIED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885339i32 as _);
pub const NS_E_WMR_CANNOT_RENDER_BINARY_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885661i32 as _);
pub const NS_E_WMR_NOCALLBACKAVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885666i32 as _);
pub const NS_E_WMR_NOSOURCEFILTER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885668i32 as _);
pub const NS_E_WMR_PINNOTFOUND: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885670i32 as _);
pub const NS_E_WMR_PINTYPENOMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885667i32 as _);
pub const NS_E_WMR_SAMPLEPROPERTYNOTSET: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885662i32 as _);
pub const NS_E_WMR_UNSUPPORTEDSTREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885671i32 as _);
pub const NS_E_WMR_WAITINGONFORMATSWITCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885669i32 as _);
pub const NS_E_WMR_WILLNOT_RENDER_BINARY_STREAM: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885659i32 as _);
pub const NS_E_WMX_ATTRIBUTE_ALREADY_EXISTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885649i32 as _);
pub const NS_E_WMX_ATTRIBUTE_DOES_NOT_EXIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885650i32 as _);
pub const NS_E_WMX_ATTRIBUTE_UNRETRIEVABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885648i32 as _);
pub const NS_E_WMX_INVALID_FORMAT_OVER_NESTING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885642i32 as _);
pub const NS_E_WMX_ITEM_DOES_NOT_EXIST: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885647i32 as _);
pub const NS_E_WMX_ITEM_TYPE_ILLEGAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885646i32 as _);
pub const NS_E_WMX_ITEM_UNSETTABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885645i32 as _);
pub const NS_E_WMX_PLAYLIST_EMPTY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885644i32 as _);
pub const NS_E_WMX_UNRECOGNIZED_PLAYLIST_FORMAT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885656i32 as _);
pub const NS_E_WONT_DO_DIGITAL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072885837i32 as _);
pub const NS_E_WRONG_OS_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884643i32 as _);
pub const NS_E_WRONG_PUBLISHING_POINT_TYPE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884654i32 as _);
pub const NS_E_WSX_INVALID_VERSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-1072884450i32 as _);
pub const NS_I_CATATONIC_AUTO_UNFAIL: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2146631270i32 as _);
pub const NS_I_CATATONIC_FAILURE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2146631271i32 as _);
pub const NS_I_CUB_RUNNING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593874i32 as _);
pub const NS_I_CUB_START: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593873i32 as _);
pub const NS_I_CUB_UNFAIL_LINK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594193i32 as _);
pub const NS_I_DISK_REBUILD_ABORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593880i32 as _);
pub const NS_I_DISK_REBUILD_FINISHED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593879i32 as _);
pub const NS_I_DISK_REBUILD_STARTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593878i32 as _);
pub const NS_I_DISK_START: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593876i32 as _);
pub const NS_I_DISK_STOP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594200i32 as _);
pub const NS_I_EXISTING_PACKETIZER: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074605827i32 as _);
pub const NS_I_KILL_CONNECTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593886i32 as _);
pub const NS_I_KILL_USERSESSION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593885i32 as _);
pub const NS_I_LIMIT_BANDWIDTH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593904i32 as _);
pub const NS_I_LIMIT_FUNNELS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593881i32 as _);
pub const NS_I_LOGGING_FAILED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593902i32 as _);
pub const NS_I_MANUAL_PROXY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074605828i32 as _);
pub const NS_I_NOLOG_STOP: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074605825i32 as _);
pub const NS_I_PLAYLIST_CHANGE_RECEDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074599102i32 as _);
pub const NS_I_REBUILD_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593887i32 as _);
pub const NS_I_RECONNECTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074605823i32 as _);
pub const NS_I_RESTRIPE_CUB_OUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594199i32 as _);
pub const NS_I_RESTRIPE_DISK_OUT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594198i32 as _);
pub const NS_I_RESTRIPE_DONE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594196i32 as _);
pub const NS_I_RESTRIPE_START: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074594195i32 as _);
pub const NS_I_START_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593882i32 as _);
pub const NS_I_STOP_CUB: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593884i32 as _);
pub const NS_I_STOP_DISK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593883i32 as _);
pub const NS_I_TIGER_START: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(1074593871i32 as _);
pub const NS_S_CALLABORTED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(851969i32 as _);
pub const NS_S_CALLPENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(851968i32 as _);
pub const NS_S_CHANGENOTICE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(864013i32 as _);
pub const NS_S_DEGRADING_QUALITY: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(854985i32 as _);
pub const NS_S_DRM_ACQUIRE_CANCELLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862023i32 as _);
pub const NS_S_DRM_BURNABLE_TRACK: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862062i32 as _);
pub const NS_S_DRM_BURNABLE_TRACK_WITH_PLAYLIST_RESTRICTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862063i32 as _);
pub const NS_S_DRM_INDIVIDUALIZED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(861991i32 as _);
pub const NS_S_DRM_LICENSE_ACQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(861990i32 as _);
pub const NS_S_DRM_MONITOR_CANCELLED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862022i32 as _);
pub const NS_S_DRM_NEEDS_INDIVIDUALIZATION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862174i32 as _);
pub const NS_S_EOSRECEDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(864009i32 as _);
pub const NS_S_NAVIGATION_COMPLETE_WITH_ERRORS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856926i32 as _);
pub const NS_S_NEED_TO_BUY_BURN_RIGHTS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856283i32 as _);
pub const NS_S_OPERATION_PENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856398i32 as _);
pub const NS_S_PUBLISHING_POINT_STARTED_WITH_FAILED_SINKS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(857369i32 as _);
pub const NS_S_REBOOT_RECOMMENDED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862968i32 as _);
pub const NS_S_REBOOT_REQUIRED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(862969i32 as _);
pub const NS_S_REBUFFERING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(854984i32 as _);
pub const NS_S_STREAM_TRUNCATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(851970i32 as _);
pub const NS_S_TRACK_ALREADY_DOWNLOADED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856929i32 as _);
pub const NS_S_TRACK_BUY_REQUIRES_ALBUM_PURCHASE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856921i32 as _);
pub const NS_S_TRANSCRYPTOR_EOF: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(855003i32 as _);
pub const NS_S_WMG_ADVISE_DROP_FRAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856166i32 as _);
pub const NS_S_WMG_ADVISE_DROP_TO_KEYFRAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856167i32 as _);
pub const NS_S_WMG_FORCE_DROP_FRAME: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856143i32 as _);
pub const NS_S_WMPBR_PARTIALSUCCESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856374i32 as _);
pub const NS_S_WMPBR_SUCCESS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856373i32 as _);
pub const NS_S_WMPCORE_COMMAND_NOT_AVAILABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856325i32 as _);
pub const NS_S_WMPCORE_MEDIA_CHILD_PLAYLIST_OPEN_PENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856329i32 as _);
pub const NS_S_WMPCORE_MEDIA_VALIDATION_PENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856323i32 as _);
pub const NS_S_WMPCORE_MORE_NODES_AVAIABLE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856330i32 as _);
pub const NS_S_WMPCORE_PLAYLISTCLEARABORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856318i32 as _);
pub const NS_S_WMPCORE_PLAYLISTREMOVEITEMABORT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856319i32 as _);
pub const NS_S_WMPCORE_PLAYLIST_COLLAPSED_TO_SINGLE_MEDIA: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856328i32 as _);
pub const NS_S_WMPCORE_PLAYLIST_CREATION_PENDING: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856322i32 as _);
pub const NS_S_WMPCORE_PLAYLIST_IMPORT_MISSING_ITEMS: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856327i32 as _);
pub const NS_S_WMPCORE_PLAYLIST_NAME_AUTO_GENERATED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856326i32 as _);
pub const NS_S_WMPCORE_PLAYLIST_REPEAT_SECONDARY_SEGMENTS_IGNORED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856324i32 as _);
pub const NS_S_WMPEFFECT_OPAQUE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856389i32 as _);
pub const NS_S_WMPEFFECT_TRANSPARENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856388i32 as _);
pub const NS_S_WMP_EXCEPTION: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856041i32 as _);
pub const NS_S_WMP_LOADED_BMP_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856130i32 as _);
pub const NS_S_WMP_LOADED_GIF_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856128i32 as _);
pub const NS_S_WMP_LOADED_JPG_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856131i32 as _);
pub const NS_S_WMP_LOADED_PNG_IMAGE: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856129i32 as _);
pub const NS_S_WMP_UI_VERSIONMISMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856040i32 as _);
pub const NS_S_WMR_ALREADYRENDERED: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856159i32 as _);
pub const NS_S_WMR_PINTYPEFULLMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856161i32 as _);
pub const NS_S_WMR_PINTYPEPARTIALMATCH: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(856160i32 as _);
pub const NS_W_FILE_BANDWIDTH_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2146631676i32 as _);
pub const NS_W_SERVER_BANDWIDTH_LIMIT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2146631677i32 as _);
pub const NS_W_UNKNOWN_EVENT: ::windows::runtime::HRESULT =
    ::windows::runtime::HRESULT(-2146631584i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLIADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl OLIADPCMWAVEFORMAT {}
impl ::std::default::Default for OLIADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OLIADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OLIADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for OLIADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLICELPWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl OLICELPWAVEFORMAT {}
impl ::std::default::Default for OLICELPWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OLICELPWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OLICELPWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for OLICELPWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLIGSMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl OLIGSMWAVEFORMAT {}
impl ::std::default::Default for OLIGSMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OLIGSMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OLIGSMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for OLIGSMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLIOPRWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl OLIOPRWAVEFORMAT {}
impl ::std::default::Default for OLIOPRWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OLIOPRWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OLIOPRWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for OLIOPRWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OLISBCWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl OLISBCWAVEFORMAT {}
impl ::std::default::Default for OLISBCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OLISBCWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OLISBCWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for OLISBCWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenDriver<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    szdrivername: Param0,
    szsectionname: Param1,
    lparam2: Param2,
) -> HDRVR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenDriver(
                szdrivername: super::super::Foundation::PWSTR,
                szsectionname: super::super::Foundation::PWSTR,
                lparam2: super::super::Foundation::LPARAM,
            ) -> HDRVR;
        }
        ::std::mem::transmute(OpenDriver(
            szdrivername.into_param().abi(),
            szsectionname.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl PCMWAVEFORMAT {}
impl ::std::default::Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for PCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PD_CAN_DRAW_DIB: u32 = 1u32;
pub const PD_CAN_STRETCHDIB: u32 = 2u32;
pub const PD_STRETCHDIB_1_1_OK: u32 = 4u32;
pub const PD_STRETCHDIB_1_2_OK: u32 = 8u32;
pub const PD_STRETCHDIB_1_N_OK: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    pszsound: Param0,
    hmod: Param1,
    fdwsound: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundA(
                pszsound: super::super::Foundation::PSTR,
                hmod: super::super::Foundation::HINSTANCE,
                fdwsound: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PlaySoundA(
            pszsound.into_param().abi(),
            hmod.into_param().abi(),
            ::std::mem::transmute(fdwsound),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
>(
    pszsound: Param0,
    hmod: Param1,
    fdwsound: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundW(
                pszsound: super::super::Foundation::PWSTR,
                hmod: super::super::Foundation::HINSTANCE,
                fdwsound: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PlaySoundW(
            pszsound.into_param().abi(),
            hmod.into_param().abi(),
            ::std::mem::transmute(fdwsound),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ROCKWELL_WA1_MIXER: u32 = 103u32;
pub const ROCKWELL_WA1_MPU401_IN: u32 = 104u32;
pub const ROCKWELL_WA1_MPU401_OUT: u32 = 105u32;
pub const ROCKWELL_WA1_SYNTH: u32 = 102u32;
pub const ROCKWELL_WA1_WAVEIN: u32 = 100u32;
pub const ROCKWELL_WA1_WAVEOUT: u32 = 101u32;
pub const ROCKWELL_WA2_MIXER: u32 = 203u32;
pub const ROCKWELL_WA2_MPU401_IN: u32 = 204u32;
pub const ROCKWELL_WA2_MPU401_OUT: u32 = 205u32;
pub const ROCKWELL_WA2_SYNTH: u32 = 202u32;
pub const ROCKWELL_WA2_WAVEIN: u32 = 200u32;
pub const ROCKWELL_WA2_WAVEOUT: u32 = 201u32;
pub const SEARCH_ANY: i32 = 32i32;
pub const SEARCH_BACKWARD: i32 = 4i32;
pub const SEARCH_FORWARD: i32 = 1i32;
pub const SEARCH_KEY: i32 = 16i32;
pub const SEARCH_NEAREST: i32 = 4i32;
pub const SEEK_CUR: u32 = 1u32;
pub const SEEK_END: u32 = 2u32;
pub const SEEK_SET: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SIERRAADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
}
impl SIERRAADPCMWAVEFORMAT {}
impl ::std::default::Default for SIERRAADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SIERRAADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SIERRAADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for SIERRAADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SND_ALIAS: i32 = 65536i32;
pub const SND_ALIAS_ID: i32 = 1114112i32;
pub const SND_ALIAS_START: u32 = 0u32;
pub const SND_APPLICATION: u32 = 128u32;
pub const SND_ASYNC: u32 = 1u32;
pub const SND_FILENAME: i32 = 131072i32;
pub const SND_LOOP: u32 = 8u32;
pub const SND_MEMORY: u32 = 4u32;
pub const SND_NODEFAULT: u32 = 2u32;
pub const SND_NOSTOP: u32 = 16u32;
pub const SND_NOWAIT: i32 = 8192i32;
pub const SND_PURGE: u32 = 64u32;
pub const SND_RESOURCE: i32 = 262148i32;
pub const SND_RING: i32 = 1048576i32;
pub const SND_SENTRY: i32 = 524288i32;
pub const SND_SYNC: u32 = 0u32;
pub const SND_SYSTEM: i32 = 2097152i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SONARCWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wCompType: u16,
}
impl SONARCWAVEFORMAT {}
impl ::std::default::Default for SONARCWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SONARCWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SONARCWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for SONARCWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendDriverMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDRVR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hdriver: Param0,
    message: u32,
    lparam1: Param2,
    lparam2: Param3,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendDriverMessage(
                hdriver: HDRVR,
                message: u32,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(SendDriverMessage(
            hdriver.into_param().abi(),
            ::std::mem::transmute(message),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TASKERR_NOTASKSUPPORT: u32 = 1u32;
pub const TASKERR_OUTOFMEMORY: u32 = 2u32;
pub const TDD_BEGINMINPERIOD: u32 = 2064u32;
pub const TDD_ENDMINPERIOD: u32 = 2068u32;
pub const TDD_GETDEVCAPS: u32 = 2060u32;
pub const TDD_GETSYSTEMTIME: u32 = 2056u32;
pub const TDD_KILLTIMEREVENT: u32 = 2048u32;
pub const TDD_SETTIMEREVENT: u32 = 2052u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
impl TIMECAPS {}
impl ::std::default::Default for TIMECAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TIMECAPS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TIMECAPS {}
unsafe impl ::windows::runtime::Abi for TIMECAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::clone::Clone for TIMEREVENT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct TIMEREVENT {
    pub wDelay: u16,
    pub wResolution: u16,
    pub lpFunction: ::std::option::Option<super::super::System::SystemServices::LPTIMECALLBACK>,
    pub dwUser: u32,
    pub wFlags: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl TIMEREVENT {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for TIMEREVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for TIMEREVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for TIMEREVENT {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for TIMEREVENT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const TIMERR_BASE: u32 = 96u32;
pub const TIMERR_NOCANDO: u32 = 97u32;
pub const TIMERR_NOERROR: u32 = 0u32;
pub const TIMERR_STRUCT: u32 = 129u32;
pub const TIME_BYTES: u32 = 4u32;
pub const TIME_MIDI: u32 = 16u32;
pub const TIME_MS: u32 = 1u32;
pub const TIME_SAMPLES: u32 = 2u32;
pub const TIME_SMPTE: u32 = 8u32;
pub const TIME_TICKS: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct TRUESPEECHWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wRevision: u16,
    pub nSamplesPerBlock: u16,
    pub abReserved: [u8; 28],
}
impl TRUESPEECHWAVEFORMAT {}
impl ::std::default::Default for TRUESPEECHWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TRUESPEECHWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TRUESPEECHWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for TRUESPEECHWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VADMAD_Device_ID: u32 = 1092u32;
pub const VCAPS_CAN_SCALE: u32 = 8u32;
pub const VCAPS_DST_CAN_CLIP: u32 = 4u32;
pub const VCAPS_OVERLAY: u32 = 1u32;
pub const VCAPS_SRC_CAN_CLIP: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub type VFWWDMExtensionProc = unsafe extern "system" fn(
    pfndeviceiocontrol: *mut ::std::ffi::c_void,
    pfnaddpropertypage: ::windows::runtime::RawPtr,
    lparam: super::super::Foundation::LPARAM,
) -> u32;
pub const VFW_HIDE_CAMERACONTROL_PAGE: u32 = 4u32;
pub const VFW_HIDE_SETTINGS_PAGE: u32 = 1u32;
pub const VFW_HIDE_VIDEOSRC_PAGE: u32 = 2u32;
pub const VFW_OEM_ADD_PAGE: u32 = 2147483648u32;
pub const VFW_QUERY_DEV_CHANGED: u32 = 256u32;
pub const VFW_USE_DEVICE_HANDLE: u32 = 1u32;
pub const VFW_USE_STREAM_HANDLE: u32 = 2u32;
pub const VHDR_DONE: u32 = 1u32;
pub const VHDR_INQUEUE: u32 = 4u32;
pub const VHDR_KEYFRAME: u32 = 8u32;
pub const VHDR_PREPARED: u32 = 2u32;
pub const VHDR_VALID: u32 = 15u32;
pub const VIDCF_COMPRESSFRAMES: u32 = 8u32;
pub const VIDCF_CRUNCH: u32 = 2u32;
pub const VIDCF_DRAW: u32 = 16u32;
pub const VIDCF_FASTTEMPORALC: u32 = 32u32;
pub const VIDCF_FASTTEMPORALD: u32 = 128u32;
pub const VIDCF_QUALITY: u32 = 1u32;
pub const VIDCF_TEMPORAL: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIDEOHDR {
    pub lpData: *mut u8,
    pub dwBufferLength: u32,
    pub dwBytesUsed: u32,
    pub dwTimeCaptured: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwReserved: [usize; 4],
}
impl VIDEOHDR {}
impl ::std::default::Default for VIDEOHDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIDEOHDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIDEOHDR")
            .field("lpData", &self.lpData)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBytesUsed", &self.dwBytesUsed)
            .field("dwTimeCaptured", &self.dwTimeCaptured)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIDEOHDR {
    fn eq(&self, other: &Self) -> bool {
        self.lpData == other.lpData
            && self.dwBufferLength == other.dwBufferLength
            && self.dwBytesUsed == other.dwBytesUsed
            && self.dwTimeCaptured == other.dwTimeCaptured
            && self.dwUser == other.dwUser
            && self.dwFlags == other.dwFlags
            && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for VIDEOHDR {}
unsafe impl ::windows::runtime::Abi for VIDEOHDR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VIDEO_CONFIGURE_CURRENT: u32 = 16u32;
pub const VIDEO_CONFIGURE_GET: u32 = 8192u32;
pub const VIDEO_CONFIGURE_MAX: u32 = 128u32;
pub const VIDEO_CONFIGURE_MIN: u32 = 64u32;
pub const VIDEO_CONFIGURE_NOMINAL: u32 = 32u32;
pub const VIDEO_CONFIGURE_QUERY: u32 = 32768u32;
pub const VIDEO_CONFIGURE_QUERYSIZE: u32 = 1u32;
pub const VIDEO_CONFIGURE_SET: u32 = 4096u32;
pub const VIDEO_DLG_QUERY: u32 = 16u32;
pub const VIDEO_EXTERNALIN: u32 = 1u32;
pub const VIDEO_EXTERNALOUT: u32 = 2u32;
pub const VIDEO_IN: u32 = 4u32;
pub const VIDEO_OUT: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl VOLUMEWAVEFILTER {}
impl ::std::default::Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VOLUMEWAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VOLUMEWAVEFILTER {}
unsafe impl ::windows::runtime::Abi for VOLUMEWAVEFILTER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VP_COMMAND_GET: u32 = 1u32;
pub const VP_COMMAND_SET: u32 = 2u32;
pub const VP_CP_CMD_ACTIVATE: u32 = 1u32;
pub const VP_CP_CMD_CHANGE: u32 = 4u32;
pub const VP_CP_CMD_DEACTIVATE: u32 = 2u32;
pub const VP_CP_TYPE_APS_TRIGGER: u32 = 1u32;
pub const VP_CP_TYPE_MACROVISION: u32 = 2u32;
pub const VP_FLAGS_BRIGHTNESS: u32 = 64u32;
pub const VP_FLAGS_CONTRAST: u32 = 128u32;
pub const VP_FLAGS_COPYPROTECT: u32 = 256u32;
pub const VP_FLAGS_FLICKER: u32 = 4u32;
pub const VP_FLAGS_MAX_UNSCALED: u32 = 16u32;
pub const VP_FLAGS_OVERSCAN: u32 = 8u32;
pub const VP_FLAGS_POSITION: u32 = 32u32;
pub const VP_FLAGS_TV_MODE: u32 = 1u32;
pub const VP_FLAGS_TV_STANDARD: u32 = 2u32;
pub const VP_MODE_TV_PLAYBACK: u32 = 2u32;
pub const VP_MODE_WIN_GRAPHICS: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_433: u32 = 65536u32;
pub const VP_TV_STANDARD_NTSC_M: u32 = 1u32;
pub const VP_TV_STANDARD_NTSC_M_J: u32 = 2u32;
pub const VP_TV_STANDARD_PAL_60: u32 = 262144u32;
pub const VP_TV_STANDARD_PAL_B: u32 = 4u32;
pub const VP_TV_STANDARD_PAL_D: u32 = 8u32;
pub const VP_TV_STANDARD_PAL_G: u32 = 131072u32;
pub const VP_TV_STANDARD_PAL_H: u32 = 16u32;
pub const VP_TV_STANDARD_PAL_I: u32 = 32u32;
pub const VP_TV_STANDARD_PAL_M: u32 = 64u32;
pub const VP_TV_STANDARD_PAL_N: u32 = 128u32;
pub const VP_TV_STANDARD_SECAM_B: u32 = 256u32;
pub const VP_TV_STANDARD_SECAM_D: u32 = 512u32;
pub const VP_TV_STANDARD_SECAM_G: u32 = 1024u32;
pub const VP_TV_STANDARD_SECAM_H: u32 = 2048u32;
pub const VP_TV_STANDARD_SECAM_K: u32 = 4096u32;
pub const VP_TV_STANDARD_SECAM_K1: u32 = 8192u32;
pub const VP_TV_STANDARD_SECAM_L: u32 = 16384u32;
pub const VP_TV_STANDARD_SECAM_L1: u32 = 524288u32;
pub const VP_TV_STANDARD_WIN_VGA: u32 = 32768u32;
#[inline]
pub unsafe fn VideoForWindowsVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VideoForWindowsVersion() -> u32;
        }
        ::std::mem::transmute(VideoForWindowsVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl WAVEFILTER {}
impl ::std::default::Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEFILTER {}
unsafe impl ::windows::runtime::Abi for WAVEFILTER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl WAVEFORMAT {}
impl ::std::default::Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl WAVEFORMATEX {}
impl ::std::default::Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEFORMATEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEFORMATEX {}
unsafe impl ::windows::runtime::Abi for WAVEFORMATEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::runtime::GUID,
}
impl WAVEFORMATEXTENSIBLE {}
impl ::std::default::Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEFORMATEXTENSIBLE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEFORMATEXTENSIBLE {}
unsafe impl ::windows::runtime::Abi for WAVEFORMATEXTENSIBLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl WAVEFORMATEXTENSIBLE_0 {}
impl ::std::default::Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEFORMATEXTENSIBLE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEFORMATEXTENSIBLE_0 {}
unsafe impl ::windows::runtime::Abi for WAVEFORMATEXTENSIBLE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEHDR {
    pub lpData: super::super::Foundation::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut WAVEHDR,
    pub reserved: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WAVEHDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WAVEHDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WAVEHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WAVEHDR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WAVEINCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WAVEINCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl WAVEINCAPS2W {}
impl ::std::default::Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEINCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEINCAPS2W {}
unsafe impl ::windows::runtime::Abi for WAVEINCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WAVEINCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WAVEINCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl WAVEINCAPSW {}
impl ::std::default::Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEINCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEINCAPSW {}
unsafe impl ::windows::runtime::Abi for WAVEINCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEOPENDESC {
    pub hWave: HWAVE,
    pub lpFormat: *mut WAVEFORMAT,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub uMappedDeviceID: u32,
    pub dnDevNode: usize,
}
impl WAVEOPENDESC {}
impl ::std::default::Default for WAVEOPENDESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEOPENDESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEOPENDESC {}
unsafe impl ::windows::runtime::Abi for WAVEOPENDESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEOUTCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WAVEOUTCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPS2A {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEOUTCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::runtime::GUID,
    pub ProductGuid: ::windows::runtime::GUID,
    pub NameGuid: ::windows::runtime::GUID,
}
impl WAVEOUTCAPS2W {}
impl ::std::default::Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEOUTCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEOUTCAPS2W {}
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPS2W {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct WAVEOUTCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WAVEOUTCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEOUTCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl WAVEOUTCAPSW {}
impl ::std::default::Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WAVEOUTCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WAVEOUTCAPSW {}
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub const WAVERR_BADFORMAT: u32 = 32u32;
pub const WAVERR_BASE: u32 = 32u32;
pub const WAVERR_LASTERROR: u32 = 35u32;
pub const WAVERR_STILLPLAYING: u32 = 33u32;
pub const WAVERR_SYNC: u32 = 35u32;
pub const WAVERR_UNPREPARED: u32 = 34u32;
pub const WAVE_FILTER_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FILTER_ECHO: u32 = 2u32;
pub const WAVE_FILTER_UNKNOWN: u32 = 0u32;
pub const WAVE_FILTER_VOLUME: u32 = 1u32;
pub const WAVE_FORMAT_1M08: u32 = 1u32;
pub const WAVE_FORMAT_1M16: u32 = 4u32;
pub const WAVE_FORMAT_1S08: u32 = 2u32;
pub const WAVE_FORMAT_1S16: u32 = 8u32;
pub const WAVE_FORMAT_2M08: u32 = 16u32;
pub const WAVE_FORMAT_2M16: u32 = 64u32;
pub const WAVE_FORMAT_2S08: u32 = 32u32;
pub const WAVE_FORMAT_2S16: u32 = 128u32;
pub const WAVE_FORMAT_3COM_NBX: u32 = 28672u32;
pub const WAVE_FORMAT_44M08: u32 = 256u32;
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
pub const WAVE_FORMAT_44S08: u32 = 512u32;
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
pub const WAVE_FORMAT_4M08: u32 = 256u32;
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
pub const WAVE_FORMAT_4S08: u32 = 512u32;
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
pub const WAVE_FORMAT_ADPCM: u32 = 2u32;
pub const WAVE_FORMAT_ALAC: u32 = 27745u32;
pub const WAVE_FORMAT_ALAW: u32 = 6u32;
pub const WAVE_FORMAT_AMR_NB: u32 = 29537u32;
pub const WAVE_FORMAT_AMR_WB: u32 = 29538u32;
pub const WAVE_FORMAT_AMR_WP: u32 = 29539u32;
pub const WAVE_FORMAT_ANTEX_ADPCME: u32 = 51u32;
pub const WAVE_FORMAT_APTX: u32 = 37u32;
pub const WAVE_FORMAT_AUDIOFILE_AF10: u32 = 38u32;
pub const WAVE_FORMAT_AUDIOFILE_AF36: u32 = 36u32;
pub const WAVE_FORMAT_BTV_DIGITAL: u32 = 1024u32;
pub const WAVE_FORMAT_CANOPUS_ATRAC: u32 = 99u32;
pub const WAVE_FORMAT_CIRRUS: u32 = 96u32;
pub const WAVE_FORMAT_CODIAN: u32 = 41252u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_AVQSBC: u32 = 41217u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_G723_1: u32 = 41216u32;
pub const WAVE_FORMAT_COMVERSE_INFOSYS_SBC: u32 = 41218u32;
pub const WAVE_FORMAT_CONGRUENCY: u32 = 141u32;
pub const WAVE_FORMAT_CONTROL_RES_CR10: u32 = 55u32;
pub const WAVE_FORMAT_CONTROL_RES_VQLPC: u32 = 52u32;
pub const WAVE_FORMAT_CONVEDIA_G729: u32 = 140u32;
pub const WAVE_FORMAT_CREATIVE_ADPCM: u32 = 512u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH10: u32 = 515u32;
pub const WAVE_FORMAT_CREATIVE_FASTSPEECH8: u32 = 514u32;
pub const WAVE_FORMAT_CS2: u32 = 608u32;
pub const WAVE_FORMAT_CS_IMAADPCM: u32 = 57u32;
pub const WAVE_FORMAT_CUSEEME: u32 = 7939u32;
pub const WAVE_FORMAT_CU_CODEC: u32 = 25u32;
pub const WAVE_FORMAT_DEVELOPMENT: u32 = 65535u32;
pub const WAVE_FORMAT_DF_G726: u32 = 133u32;
pub const WAVE_FORMAT_DF_GSM610: u32 = 134u32;
pub const WAVE_FORMAT_DIALOGIC_OKI_ADPCM: u32 = 23u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP54: u32 = 322u32;
pub const WAVE_FORMAT_DICTAPHONE_CELP68: u32 = 321u32;
pub const WAVE_FORMAT_DIGIADPCM: u32 = 54u32;
pub const WAVE_FORMAT_DIGIFIX: u32 = 22u32;
pub const WAVE_FORMAT_DIGIREAL: u32 = 53u32;
pub const WAVE_FORMAT_DIGISTD: u32 = 21u32;
pub const WAVE_FORMAT_DIGITAL_G723: u32 = 291u32;
pub const WAVE_FORMAT_DIVIO_G726: u32 = 16963u32;
pub const WAVE_FORMAT_DIVIO_MPEG4_AAC: u32 = 16707u32;
pub const WAVE_FORMAT_DOLBY_AC2: u32 = 48u32;
pub const WAVE_FORMAT_DOLBY_AC3_SPDIF: u32 = 146u32;
pub const WAVE_FORMAT_DOLBY_AC4: u32 = 44096u32;
pub const WAVE_FORMAT_DRM: u32 = 9u32;
pub const WAVE_FORMAT_DSAT: u32 = 102u32;
pub const WAVE_FORMAT_DSAT_DISPLAY: u32 = 103u32;
pub const WAVE_FORMAT_DSPGROUP_TRUESPEECH: u32 = 34u32;
pub const WAVE_FORMAT_DTS: u32 = 8u32;
pub const WAVE_FORMAT_DTS2: u32 = 8193u32;
pub const WAVE_FORMAT_DTS_DS: u32 = 400u32;
pub const WAVE_FORMAT_DVI_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_DVM: u32 = 8192u32;
pub const WAVE_FORMAT_ECHOSC1: u32 = 35u32;
pub const WAVE_FORMAT_ECHOSC3: u32 = 58u32;
pub const WAVE_FORMAT_ENCORE_G726: u32 = 41223u32;
pub const WAVE_FORMAT_ESPCM: u32 = 97u32;
pub const WAVE_FORMAT_ESST_AC3: u32 = 577u32;
pub const WAVE_FORMAT_FAAD_AAC: u32 = 28781u32;
pub const WAVE_FORMAT_FLAC: u32 = 61868u32;
pub const WAVE_FORMAT_FM_TOWNS_SND: u32 = 768u32;
pub const WAVE_FORMAT_FRACE_TELECOM_G729: u32 = 41251u32;
pub const WAVE_FORMAT_FRAUNHOFER_IIS_MPEG2_AAC: u32 = 384u32;
pub const WAVE_FORMAT_G721_ADPCM: u32 = 64u32;
pub const WAVE_FORMAT_G722_ADPCM: u32 = 101u32;
pub const WAVE_FORMAT_G723_ADPCM: u32 = 20u32;
pub const WAVE_FORMAT_G726ADPCM: u32 = 320u32;
pub const WAVE_FORMAT_G726_ADPCM: u32 = 100u32;
pub const WAVE_FORMAT_G728_CELP: u32 = 65u32;
pub const WAVE_FORMAT_G729A: u32 = 131u32;
pub const WAVE_FORMAT_GENERIC_PASSTHRU: u32 = 585u32;
pub const WAVE_FORMAT_GLOBAL_IP_ILBC: u32 = 41238u32;
pub const WAVE_FORMAT_GSM610: u32 = 49u32;
pub const WAVE_FORMAT_GSM_610: u32 = 41229u32;
pub const WAVE_FORMAT_GSM_620: u32 = 41230u32;
pub const WAVE_FORMAT_GSM_660: u32 = 41231u32;
pub const WAVE_FORMAT_GSM_690: u32 = 41232u32;
pub const WAVE_FORMAT_GSM_ADAPTIVE_MULTIRATE_WB: u32 = 41233u32;
pub const WAVE_FORMAT_GSM_AMR_CBR: u32 = 31265u32;
pub const WAVE_FORMAT_GSM_AMR_VBR_SID: u32 = 31266u32;
pub const WAVE_FORMAT_HP_DYN_VOICE: u32 = 26u32;
pub const WAVE_FORMAT_IBM_CVSD: u32 = 5u32;
pub const WAVE_FORMAT_IEEE_FLOAT: u32 = 3u32;
pub const WAVE_FORMAT_ILINK_VC: u32 = 560u32;
pub const WAVE_FORMAT_IMA_ADPCM: u32 = 17u32;
pub const WAVE_FORMAT_INDEO_AUDIO: u32 = 1026u32;
pub const WAVE_FORMAT_INFOCOM_ITS_G721_ADPCM: u32 = 139u32;
pub const WAVE_FORMAT_INGENIENT_G726: u32 = 41221u32;
pub const WAVE_FORMAT_INNINGS_TELECOM_ADPCM: u32 = 6521u32;
pub const WAVE_FORMAT_INTEL_G723_1: u32 = 67u32;
pub const WAVE_FORMAT_INTEL_G729: u32 = 68u32;
pub const WAVE_FORMAT_INTEL_MUSIC_CODER: u32 = 1025u32;
pub const WAVE_FORMAT_IPI_HSX: u32 = 592u32;
pub const WAVE_FORMAT_IPI_RPELP: u32 = 593u32;
pub const WAVE_FORMAT_IRAT: u32 = 257u32;
pub const WAVE_FORMAT_ISIAUDIO: u32 = 136u32;
pub const WAVE_FORMAT_ISIAUDIO_2: u32 = 5121u32;
pub const WAVE_FORMAT_KNOWLEDGE_ADVENTURE_ADPCM: u32 = 376u32;
pub const WAVE_FORMAT_LEAD_SPEECH: u32 = 17228u32;
pub const WAVE_FORMAT_LEAD_VORBIS: u32 = 22092u32;
pub const WAVE_FORMAT_LH_CODEC: u32 = 4352u32;
pub const WAVE_FORMAT_LH_CODEC_CELP: u32 = 4353u32;
pub const WAVE_FORMAT_LH_CODEC_SBC12: u32 = 4355u32;
pub const WAVE_FORMAT_LH_CODEC_SBC16: u32 = 4356u32;
pub const WAVE_FORMAT_LH_CODEC_SBC8: u32 = 4354u32;
pub const WAVE_FORMAT_LIGHTWAVE_LOSSLESS: u32 = 2222u32;
pub const WAVE_FORMAT_LRC: u32 = 40u32;
pub const WAVE_FORMAT_LUCENT_G723: u32 = 89u32;
pub const WAVE_FORMAT_LUCENT_SX5363S: u32 = 7180u32;
pub const WAVE_FORMAT_LUCENT_SX8300P: u32 = 7175u32;
pub const WAVE_FORMAT_MAKEAVIS: u32 = 13075u32;
pub const WAVE_FORMAT_MALDEN_PHONYTALK: u32 = 160u32;
pub const WAVE_FORMAT_MEDIASONIC_G723: u32 = 147u32;
pub const WAVE_FORMAT_MEDIASPACE_ADPCM: u32 = 18u32;
pub const WAVE_FORMAT_MEDIAVISION_ADPCM: u32 = 24u32;
pub const WAVE_FORMAT_MICRONAS: u32 = 848u32;
pub const WAVE_FORMAT_MICRONAS_CELP833: u32 = 849u32;
pub const WAVE_FORMAT_MPEG: u32 = 80u32;
pub const WAVE_FORMAT_MPEG4_AAC: u32 = 41222u32;
pub const WAVE_FORMAT_MPEGLAYER3: u32 = 85u32;
pub const WAVE_FORMAT_MPEG_ADTS_AAC: u32 = 5632u32;
pub const WAVE_FORMAT_MPEG_HEAAC: u32 = 5648u32;
pub const WAVE_FORMAT_MPEG_LOAS: u32 = 5634u32;
pub const WAVE_FORMAT_MPEG_RAW_AAC: u32 = 5633u32;
pub const WAVE_FORMAT_MSAUDIO1: u32 = 352u32;
pub const WAVE_FORMAT_MSG723: u32 = 66u32;
pub const WAVE_FORMAT_MSNAUDIO: u32 = 50u32;
pub const WAVE_FORMAT_MSRT24: u32 = 130u32;
pub const WAVE_FORMAT_MULAW: u32 = 7u32;
pub const WAVE_FORMAT_MULTITUDE_FT_SX20: u32 = 138u32;
pub const WAVE_FORMAT_MVI_MVI2: u32 = 132u32;
pub const WAVE_FORMAT_NEC_AAC: u32 = 176u32;
pub const WAVE_FORMAT_NICE_ACA: u32 = 41240u32;
pub const WAVE_FORMAT_NICE_ADPCM: u32 = 41241u32;
pub const WAVE_FORMAT_NICE_G728: u32 = 41250u32;
pub const WAVE_FORMAT_NMS_VBXADPCM: u32 = 56u32;
pub const WAVE_FORMAT_NOKIA_ADAPTIVE_MULTIRATE: u32 = 16897u32;
pub const WAVE_FORMAT_NOKIA_MPEG_ADTS_AAC: u32 = 5640u32;
pub const WAVE_FORMAT_NOKIA_MPEG_RAW_AAC: u32 = 5641u32;
pub const WAVE_FORMAT_NORCOM_VOICE_SYSTEMS_ADPCM: u32 = 645u32;
pub const WAVE_FORMAT_NORRIS: u32 = 5120u32;
pub const WAVE_FORMAT_NTCSOFT_ALF2CM_ACM: u32 = 8132u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1: u32 = 26447u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_1_PLUS: u32 = 26479u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2: u32 = 26448u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_2_PLUS: u32 = 26480u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3: u32 = 26449u32;
pub const WAVE_FORMAT_OGG_VORBIS_MODE_3_PLUS: u32 = 26481u32;
pub const WAVE_FORMAT_OKI_ADPCM: u32 = 16u32;
pub const WAVE_FORMAT_OLIADPCM: u32 = 4097u32;
pub const WAVE_FORMAT_OLICELP: u32 = 4098u32;
pub const WAVE_FORMAT_OLIGSM: u32 = 4096u32;
pub const WAVE_FORMAT_OLIOPR: u32 = 4100u32;
pub const WAVE_FORMAT_OLISBC: u32 = 4099u32;
pub const WAVE_FORMAT_ON2_VP6_AUDIO: u32 = 1281u32;
pub const WAVE_FORMAT_ON2_VP7_AUDIO: u32 = 1280u32;
pub const WAVE_FORMAT_ONLIVE: u32 = 137u32;
pub const WAVE_FORMAT_OPUS: u32 = 28751u32;
pub const WAVE_FORMAT_PAC: u32 = 83u32;
pub const WAVE_FORMAT_PACKED: u32 = 153u32;
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_FORMAT_PCM_S: u32 = 1152u32;
pub const WAVE_FORMAT_PHILIPS_CELP: u32 = 288u32;
pub const WAVE_FORMAT_PHILIPS_GRUNDIG: u32 = 289u32;
pub const WAVE_FORMAT_PHILIPS_LPCBB: u32 = 152u32;
pub const WAVE_FORMAT_POLYCOM_G722: u32 = 41234u32;
pub const WAVE_FORMAT_POLYCOM_G728: u32 = 41235u32;
pub const WAVE_FORMAT_POLYCOM_G729_A: u32 = 41236u32;
pub const WAVE_FORMAT_POLYCOM_SIREN: u32 = 41237u32;
pub const WAVE_FORMAT_PROSODY_1612: u32 = 39u32;
pub const WAVE_FORMAT_PROSODY_8KBPS: u32 = 148u32;
pub const WAVE_FORMAT_QDESIGN_MUSIC: u32 = 1104u32;
pub const WAVE_FORMAT_QUALCOMM_HALFRATE: u32 = 337u32;
pub const WAVE_FORMAT_QUALCOMM_PUREVOICE: u32 = 336u32;
pub const WAVE_FORMAT_QUARTERDECK: u32 = 544u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G720_A: u32 = 162u32;
pub const WAVE_FORMAT_RACAL_RECORDER_G723_1: u32 = 163u32;
pub const WAVE_FORMAT_RACAL_RECORDER_GSM: u32 = 161u32;
pub const WAVE_FORMAT_RACAL_RECORDER_TETRA_ACELP: u32 = 164u32;
pub const WAVE_FORMAT_RADIOTIME_TIME_SHIFT_RADIO: u32 = 41239u32;
pub const WAVE_FORMAT_RAW_AAC1: u32 = 255u32;
pub const WAVE_FORMAT_RAW_SPORT: u32 = 576u32;
pub const WAVE_FORMAT_RHETOREX_ADPCM: u32 = 256u32;
pub const WAVE_FORMAT_ROCKWELL_ADPCM: u32 = 59u32;
pub const WAVE_FORMAT_ROCKWELL_DIGITALK: u32 = 60u32;
pub const WAVE_FORMAT_RT24: u32 = 82u32;
pub const WAVE_FORMAT_SANYO_LD_ADPCM: u32 = 293u32;
pub const WAVE_FORMAT_SBC24: u32 = 145u32;
pub const WAVE_FORMAT_SHARP_G726: u32 = 69u32;
pub const WAVE_FORMAT_SIERRA_ADPCM: u32 = 19u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP4800: u32 = 305u32;
pub const WAVE_FORMAT_SIPROLAB_ACELP8V3: u32 = 306u32;
pub const WAVE_FORMAT_SIPROLAB_ACEPLNET: u32 = 304u32;
pub const WAVE_FORMAT_SIPROLAB_G729: u32 = 307u32;
pub const WAVE_FORMAT_SIPROLAB_G729A: u32 = 308u32;
pub const WAVE_FORMAT_SIPROLAB_KELVIN: u32 = 309u32;
pub const WAVE_FORMAT_SOFTSOUND: u32 = 128u32;
pub const WAVE_FORMAT_SONARC: u32 = 33u32;
pub const WAVE_FORMAT_SONICFOUNDRY_LOSSLESS: u32 = 6513u32;
pub const WAVE_FORMAT_SONY_ATRAC3: u32 = 626u32;
pub const WAVE_FORMAT_SONY_SCX: u32 = 624u32;
pub const WAVE_FORMAT_SONY_SCY: u32 = 625u32;
pub const WAVE_FORMAT_SONY_SPC: u32 = 627u32;
pub const WAVE_FORMAT_SOUNDSPACE_MUSICOMPRESS: u32 = 5376u32;
pub const WAVE_FORMAT_SPEEX_VOICE: u32 = 41225u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC008: u32 = 372u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP54: u32 = 374u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_CELP68: u32 = 375u32;
pub const WAVE_FORMAT_SYCOM_ACM_SYC701_G726L: u32 = 373u32;
pub const WAVE_FORMAT_SYMBOL_G729_A: u32 = 41219u32;
pub const WAVE_FORMAT_TELUM_AUDIO: u32 = 640u32;
pub const WAVE_FORMAT_TELUM_IA_AUDIO: u32 = 641u32;
pub const WAVE_FORMAT_TPC: u32 = 1665u32;
pub const WAVE_FORMAT_TUBGSM: u32 = 341u32;
pub const WAVE_FORMAT_UHER_ADPCM: u32 = 528u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO: u32 = 533u32;
pub const WAVE_FORMAT_ULEAD_DV_AUDIO_1: u32 = 534u32;
pub const WAVE_FORMAT_UNISYS_NAP_16K: u32 = 371u32;
pub const WAVE_FORMAT_UNISYS_NAP_ADPCM: u32 = 368u32;
pub const WAVE_FORMAT_UNISYS_NAP_ALAW: u32 = 370u32;
pub const WAVE_FORMAT_UNISYS_NAP_ULAW: u32 = 369u32;
pub const WAVE_FORMAT_UNKNOWN: u32 = 0u32;
pub const WAVE_FORMAT_VIANIX_MASC: u32 = 41226u32;
pub const WAVE_FORMAT_VIVO_G723: u32 = 273u32;
pub const WAVE_FORMAT_VIVO_SIREN: u32 = 274u32;
pub const WAVE_FORMAT_VME_VMPCM: u32 = 1664u32;
pub const WAVE_FORMAT_VOCORD_G721: u32 = 41242u32;
pub const WAVE_FORMAT_VOCORD_G722_1: u32 = 41244u32;
pub const WAVE_FORMAT_VOCORD_G723_1: u32 = 41248u32;
pub const WAVE_FORMAT_VOCORD_G726: u32 = 41243u32;
pub const WAVE_FORMAT_VOCORD_G728: u32 = 41245u32;
pub const WAVE_FORMAT_VOCORD_G729: u32 = 41246u32;
pub const WAVE_FORMAT_VOCORD_G729_A: u32 = 41247u32;
pub const WAVE_FORMAT_VOCORD_LBC: u32 = 41249u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_ADTS_AAC: u32 = 5642u32;
pub const WAVE_FORMAT_VODAFONE_MPEG_RAW_AAC: u32 = 5643u32;
pub const WAVE_FORMAT_VOICEAGE_AMR: u32 = 310u32;
pub const WAVE_FORMAT_VOICEAGE_AMR_WB: u32 = 41220u32;
pub const WAVE_FORMAT_VOXWARE: u32 = 98u32;
pub const WAVE_FORMAT_VOXWARE_AC10: u32 = 113u32;
pub const WAVE_FORMAT_VOXWARE_AC16: u32 = 114u32;
pub const WAVE_FORMAT_VOXWARE_AC20: u32 = 115u32;
pub const WAVE_FORMAT_VOXWARE_AC8: u32 = 112u32;
pub const WAVE_FORMAT_VOXWARE_BYTE_ALIGNED: u32 = 105u32;
pub const WAVE_FORMAT_VOXWARE_RT24: u32 = 116u32;
pub const WAVE_FORMAT_VOXWARE_RT24_SPEECH: u32 = 6172u32;
pub const WAVE_FORMAT_VOXWARE_RT29: u32 = 117u32;
pub const WAVE_FORMAT_VOXWARE_RT29HW: u32 = 118u32;
pub const WAVE_FORMAT_VOXWARE_SC3: u32 = 122u32;
pub const WAVE_FORMAT_VOXWARE_SC3_1: u32 = 123u32;
pub const WAVE_FORMAT_VOXWARE_TQ40: u32 = 121u32;
pub const WAVE_FORMAT_VOXWARE_TQ60: u32 = 129u32;
pub const WAVE_FORMAT_VOXWARE_VR12: u32 = 119u32;
pub const WAVE_FORMAT_VOXWARE_VR18: u32 = 120u32;
pub const WAVE_FORMAT_VSELP: u32 = 4u32;
pub const WAVE_FORMAT_WAVPACK_AUDIO: u32 = 22358u32;
pub const WAVE_FORMAT_WM9_SPECTRUM_ANALYZER: u32 = 41227u32;
pub const WAVE_FORMAT_WMASPDIF: u32 = 356u32;
pub const WAVE_FORMAT_WMAUDIO2: u32 = 353u32;
pub const WAVE_FORMAT_WMAUDIO3: u32 = 354u32;
pub const WAVE_FORMAT_WMAUDIO_LOSSLESS: u32 = 355u32;
pub const WAVE_FORMAT_WMAVOICE10: u32 = 11u32;
pub const WAVE_FORMAT_WMAVOICE9: u32 = 10u32;
pub const WAVE_FORMAT_WMF_SPECTRUM_ANAYZER: u32 = 41228u32;
pub const WAVE_FORMAT_XEBEC: u32 = 61u32;
pub const WAVE_FORMAT_YAMAHA_ADPCM: u32 = 32u32;
pub const WAVE_FORMAT_ZOLL_ASAO: u32 = 41224u32;
pub const WAVE_FORMAT_ZYXEL_ADPCM: u32 = 151u32;
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WAVE_MAPPER_S: u32 = 1153u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_ADDBUFFER: u32 = 56u32;
pub const WIDM_CLOSE: u32 = 53u32;
pub const WIDM_GETDEVCAPS: u32 = 51u32;
pub const WIDM_GETNUMDEVS: u32 = 50u32;
pub const WIDM_GETPOS: u32 = 60u32;
pub const WIDM_INIT: u32 = 100u32;
pub const WIDM_INIT_EX: u32 = 104u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WIDM_OPEN: u32 = 52u32;
pub const WIDM_PREFERRED: u32 = 61u32;
pub const WIDM_PREPARE: u32 = 54u32;
pub const WIDM_RESET: u32 = 59u32;
pub const WIDM_START: u32 = 57u32;
pub const WIDM_STOP: u32 = 58u32;
pub const WIDM_UNPREPARE: u32 = 55u32;
pub const WIM_CLOSE: u32 = 959u32;
pub const WIM_DATA: u32 = 960u32;
pub const WIM_OPEN: u32 = 958u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WMAUDIO2WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub dwSamplesPerBlock: u32,
    pub wEncodeOptions: u16,
    pub dwSuperBlockAlign: u32,
}
impl WMAUDIO2WAVEFORMAT {}
impl ::std::default::Default for WMAUDIO2WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMAUDIO2WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMAUDIO2WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for WMAUDIO2WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WMAUDIO2_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO2_MAX_CHANNELS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WMAUDIO3WAVEFORMAT {
    pub wfx: WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub wEncodeOptions: u16,
    pub wReserved3: u16,
}
impl WMAUDIO3WAVEFORMAT {}
impl ::std::default::Default for WMAUDIO3WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for WMAUDIO3WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for WMAUDIO3WAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for WMAUDIO3WAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WMAUDIO_BITS_PER_SAMPLE: u32 = 16u32;
pub const WMAUDIO_MAX_CHANNELS: u32 = 2u32;
pub const WM_CAP_ABORT: u32 = 1093u32;
pub const WM_CAP_DLG_VIDEOCOMPRESSION: u32 = 1070u32;
pub const WM_CAP_DLG_VIDEODISPLAY: u32 = 1067u32;
pub const WM_CAP_DLG_VIDEOFORMAT: u32 = 1065u32;
pub const WM_CAP_DLG_VIDEOSOURCE: u32 = 1066u32;
pub const WM_CAP_DRIVER_CONNECT: u32 = 1034u32;
pub const WM_CAP_DRIVER_DISCONNECT: u32 = 1035u32;
pub const WM_CAP_DRIVER_GET_CAPS: u32 = 1038u32;
pub const WM_CAP_DRIVER_GET_NAME: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_NAMEA: u32 = 1036u32;
pub const WM_CAP_DRIVER_GET_NAMEW: u32 = 1136u32;
pub const WM_CAP_DRIVER_GET_VERSION: u32 = 1137u32;
pub const WM_CAP_DRIVER_GET_VERSIONA: u32 = 1037u32;
pub const WM_CAP_DRIVER_GET_VERSIONW: u32 = 1137u32;
pub const WM_CAP_EDIT_COPY: u32 = 1054u32;
pub const WM_CAP_END: u32 = 1205u32;
pub const WM_CAP_FILE_ALLOCATE: u32 = 1046u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILE: u32 = 1145u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEA: u32 = 1045u32;
pub const WM_CAP_FILE_GET_CAPTURE_FILEW: u32 = 1145u32;
pub const WM_CAP_FILE_SAVEAS: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEASA: u32 = 1047u32;
pub const WM_CAP_FILE_SAVEASW: u32 = 1147u32;
pub const WM_CAP_FILE_SAVEDIB: u32 = 1149u32;
pub const WM_CAP_FILE_SAVEDIBA: u32 = 1049u32;
pub const WM_CAP_FILE_SAVEDIBW: u32 = 1149u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILE: u32 = 1144u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEA: u32 = 1044u32;
pub const WM_CAP_FILE_SET_CAPTURE_FILEW: u32 = 1144u32;
pub const WM_CAP_FILE_SET_INFOCHUNK: u32 = 1048u32;
pub const WM_CAP_GET_AUDIOFORMAT: u32 = 1060u32;
pub const WM_CAP_GET_CAPSTREAMPTR: u32 = 1025u32;
pub const WM_CAP_GET_MCI_DEVICE: u32 = 1191u32;
pub const WM_CAP_GET_MCI_DEVICEA: u32 = 1091u32;
pub const WM_CAP_GET_MCI_DEVICEW: u32 = 1191u32;
pub const WM_CAP_GET_SEQUENCE_SETUP: u32 = 1089u32;
pub const WM_CAP_GET_STATUS: u32 = 1078u32;
pub const WM_CAP_GET_USER_DATA: u32 = 1032u32;
pub const WM_CAP_GET_VIDEOFORMAT: u32 = 1068u32;
pub const WM_CAP_GRAB_FRAME: u32 = 1084u32;
pub const WM_CAP_GRAB_FRAME_NOSTOP: u32 = 1085u32;
pub const WM_CAP_PAL_AUTOCREATE: u32 = 1107u32;
pub const WM_CAP_PAL_MANUALCREATE: u32 = 1108u32;
pub const WM_CAP_PAL_OPEN: u32 = 1204u32;
pub const WM_CAP_PAL_OPENA: u32 = 1104u32;
pub const WM_CAP_PAL_OPENW: u32 = 1204u32;
pub const WM_CAP_PAL_PASTE: u32 = 1106u32;
pub const WM_CAP_PAL_SAVE: u32 = 1205u32;
pub const WM_CAP_PAL_SAVEA: u32 = 1105u32;
pub const WM_CAP_PAL_SAVEW: u32 = 1205u32;
pub const WM_CAP_SEQUENCE: u32 = 1086u32;
pub const WM_CAP_SEQUENCE_NOFILE: u32 = 1087u32;
pub const WM_CAP_SET_AUDIOFORMAT: u32 = 1059u32;
pub const WM_CAP_SET_CALLBACK_CAPCONTROL: u32 = 1109u32;
pub const WM_CAP_SET_CALLBACK_ERROR: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_ERRORA: u32 = 1026u32;
pub const WM_CAP_SET_CALLBACK_ERRORW: u32 = 1126u32;
pub const WM_CAP_SET_CALLBACK_FRAME: u32 = 1029u32;
pub const WM_CAP_SET_CALLBACK_STATUS: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_STATUSA: u32 = 1027u32;
pub const WM_CAP_SET_CALLBACK_STATUSW: u32 = 1127u32;
pub const WM_CAP_SET_CALLBACK_VIDEOSTREAM: u32 = 1030u32;
pub const WM_CAP_SET_CALLBACK_WAVESTREAM: u32 = 1031u32;
pub const WM_CAP_SET_CALLBACK_YIELD: u32 = 1028u32;
pub const WM_CAP_SET_MCI_DEVICE: u32 = 1190u32;
pub const WM_CAP_SET_MCI_DEVICEA: u32 = 1090u32;
pub const WM_CAP_SET_MCI_DEVICEW: u32 = 1190u32;
pub const WM_CAP_SET_OVERLAY: u32 = 1075u32;
pub const WM_CAP_SET_PREVIEW: u32 = 1074u32;
pub const WM_CAP_SET_PREVIEWRATE: u32 = 1076u32;
pub const WM_CAP_SET_SCALE: u32 = 1077u32;
pub const WM_CAP_SET_SCROLL: u32 = 1079u32;
pub const WM_CAP_SET_SEQUENCE_SETUP: u32 = 1088u32;
pub const WM_CAP_SET_USER_DATA: u32 = 1033u32;
pub const WM_CAP_SET_VIDEOFORMAT: u32 = 1069u32;
pub const WM_CAP_SINGLE_FRAME: u32 = 1096u32;
pub const WM_CAP_SINGLE_FRAME_CLOSE: u32 = 1095u32;
pub const WM_CAP_SINGLE_FRAME_OPEN: u32 = 1094u32;
pub const WM_CAP_START: u32 = 1024u32;
pub const WM_CAP_STOP: u32 = 1092u32;
pub const WM_CAP_UNICODE_END: u32 = 1205u32;
pub const WM_CAP_UNICODE_START: u32 = 1124u32;
pub const WODM_BREAKLOOP: u32 = 20u32;
pub const WODM_BUSY: u32 = 21u32;
pub const WODM_CLOSE: u32 = 6u32;
pub const WODM_GETDEVCAPS: u32 = 4u32;
pub const WODM_GETNUMDEVS: u32 = 3u32;
pub const WODM_GETPITCH: u32 = 14u32;
pub const WODM_GETPLAYBACKRATE: u32 = 18u32;
pub const WODM_GETPOS: u32 = 13u32;
pub const WODM_GETVOLUME: u32 = 16u32;
pub const WODM_INIT: u32 = 100u32;
pub const WODM_INIT_EX: u32 = 104u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_OPEN: u32 = 5u32;
pub const WODM_PAUSE: u32 = 10u32;
pub const WODM_PREFERRED: u32 = 21u32;
pub const WODM_PREPARE: u32 = 7u32;
pub const WODM_RESET: u32 = 12u32;
pub const WODM_RESTART: u32 = 11u32;
pub const WODM_SETPITCH: u32 = 15u32;
pub const WODM_SETPLAYBACKRATE: u32 = 19u32;
pub const WODM_SETVOLUME: u32 = 17u32;
pub const WODM_UNPREPARE: u32 = 8u32;
pub const WODM_WRITE: u32 = 9u32;
pub const WOM_CLOSE: u32 = 956u32;
pub const WOM_DONE: u32 = 957u32;
pub const WOM_OPEN: u32 = 955u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct YAMAHA_ADPCMWAVEFORMAT {
    pub wfx: WAVEFORMATEX,
}
impl YAMAHA_ADPCMWAVEFORMAT {}
impl ::std::default::Default for YAMAHA_ADPCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for YAMAHA_ADPCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for YAMAHA_ADPCMWAVEFORMAT {}
unsafe impl ::windows::runtime::Abi for YAMAHA_ADPCMWAVEFORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    phadid: *mut isize,
    hinstmodule: Param1,
    lparam: Param2,
    dwpriority: u32,
    fdwadd: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverAddA(
                phadid: *mut isize,
                hinstmodule: super::super::Foundation::HINSTANCE,
                lparam: super::super::Foundation::LPARAM,
                dwpriority: u32,
                fdwadd: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmDriverAddA(
            ::std::mem::transmute(phadid),
            hinstmodule.into_param().abi(),
            lparam.into_param().abi(),
            ::std::mem::transmute(dwpriority),
            ::std::mem::transmute(fdwadd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    phadid: *mut isize,
    hinstmodule: Param1,
    lparam: Param2,
    dwpriority: u32,
    fdwadd: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverAddW(
                phadid: *mut isize,
                hinstmodule: super::super::Foundation::HINSTANCE,
                lparam: super::super::Foundation::LPARAM,
                dwpriority: u32,
                fdwadd: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmDriverAddW(
            ::std::mem::transmute(phadid),
            hinstmodule.into_param().abi(),
            lparam.into_param().abi(),
            ::std::mem::transmute(dwpriority),
            ::std::mem::transmute(fdwadd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverClose<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    fdwclose: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
        }
        ::std::mem::transmute(acmDriverClose(
            had.into_param().abi(),
            ::std::mem::transmute(fdwclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn acmDriverDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(
    hadid: Param0,
    padd: *mut ACMDRIVERDETAILSA,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverDetailsA(
                hadid: HACMDRIVERID,
                padd: *mut ACMDRIVERDETAILSA,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmDriverDetailsA(
            hadid.into_param().abi(),
            ::std::mem::transmute(padd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(
    hadid: Param0,
    padd: *mut ACMDRIVERDETAILSW,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverDetailsW(
                hadid: HACMDRIVERID,
                padd: *mut ACMDRIVERDETAILSW,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmDriverDetailsW(
            hadid.into_param().abi(),
            ::std::mem::transmute(padd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverEnum(
    fncallback: ::std::option::Option<ACMDRIVERENUMCB>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverEnum(
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmDriverEnum(
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverID<'a, Param0: ::windows::runtime::IntoParam<'a, HACMOBJ>>(
    hao: Param0,
    phadid: *mut isize,
    fdwdriverid: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
        }
        ::std::mem::transmute(acmDriverID(
            hao.into_param().abi(),
            ::std::mem::transmute(phadid),
            ::std::mem::transmute(fdwdriverid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    had: Param0,
    umsg: u32,
    lparam1: Param2,
    lparam2: Param3,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverMessage(
                had: HACMDRIVER,
                umsg: u32,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(acmDriverMessage(
            had.into_param().abi(),
            ::std::mem::transmute(umsg),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverOpen<'a, Param1: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(
    phad: *mut isize,
    hadid: Param1,
    fdwopen: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
        }
        ::std::mem::transmute(acmDriverOpen(
            ::std::mem::transmute(phad),
            hadid.into_param().abi(),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverPriority<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(
    hadid: Param0,
    dwpriority: u32,
    fdwpriority: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
        }
        ::std::mem::transmute(acmDriverPriority(
            hadid.into_param().abi(),
            ::std::mem::transmute(dwpriority),
            ::std::mem::transmute(fdwpriority),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverRemove<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(
    hadid: Param0,
    fdwremove: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
        }
        ::std::mem::transmute(acmDriverRemove(
            hadid.into_param().abi(),
            ::std::mem::transmute(fdwremove),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterChooseA(pafltrc: *mut ::std::mem::ManuallyDrop<ACMFILTERCHOOSEA>) -> u32;
        }
        ::std::mem::transmute(acmFilterChooseA(::std::mem::transmute(pafltrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterChooseW(pafltrc: *mut ::std::mem::ManuallyDrop<ACMFILTERCHOOSEW>) -> u32;
        }
        ::std::mem::transmute(acmFilterChooseW(::std::mem::transmute(pafltrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFILTERDETAILSA,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterDetailsA(
                had: HACMDRIVER,
                pafd: *mut ACMFILTERDETAILSA,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterDetailsA(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFilterDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFILTERDETAILSW,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterDetailsW(
                had: HACMDRIVER,
                pafd: *mut ACMFILTERDETAILSW,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterDetailsW(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFILTERDETAILSA,
    fncallback: ::std::option::Option<ACMFILTERENUMCBA>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumA(
                had: HACMDRIVER,
                pafd: *mut ACMFILTERDETAILSA,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterEnumA(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFILTERDETAILSW,
    fncallback: ::std::option::Option<ACMFILTERENUMCBW>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumW(
                had: HACMDRIVER,
                pafd: *mut ACMFILTERDETAILSW,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterEnumW(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFILTERTAGDETAILSA,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagDetailsA(
                had: HACMDRIVER,
                paftd: *mut ACMFILTERTAGDETAILSA,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterTagDetailsA(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFilterTagDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFILTERTAGDETAILSW,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagDetailsW(
                had: HACMDRIVER,
                paftd: *mut ACMFILTERTAGDETAILSW,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterTagDetailsW(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFILTERTAGDETAILSA,
    fncallback: ::std::option::Option<ACMFILTERTAGENUMCBA>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumA(
                had: HACMDRIVER,
                paftd: *mut ACMFILTERTAGDETAILSA,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterTagEnumA(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFILTERTAGDETAILSW,
    fncallback: ::std::option::Option<ACMFILTERTAGENUMCBW>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumW(
                had: HACMDRIVER,
                paftd: *mut ACMFILTERTAGDETAILSW,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFilterTagEnumW(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatChooseA(pafmtc: *mut ::std::mem::ManuallyDrop<ACMFORMATCHOOSEA>) -> u32;
        }
        ::std::mem::transmute(acmFormatChooseA(::std::mem::transmute(pafmtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatChooseW(pafmtc: *mut ::std::mem::ManuallyDrop<ACMFORMATCHOOSEW>) -> u32;
        }
        ::std::mem::transmute(acmFormatChooseW(::std::mem::transmute(pafmtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFORMATDETAILSA,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatDetailsA(
                had: HACMDRIVER,
                pafd: *mut ACMFORMATDETAILSA,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatDetailsA(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut tACMFORMATDETAILSW,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatDetailsW(
                had: HACMDRIVER,
                pafd: *mut tACMFORMATDETAILSW,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatDetailsW(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut ACMFORMATDETAILSA,
    fncallback: ::std::option::Option<ACMFORMATENUMCBA>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumA(
                had: HACMDRIVER,
                pafd: *mut ACMFORMATDETAILSA,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatEnumA(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pafd: *mut tACMFORMATDETAILSW,
    fncallback: ::std::option::Option<ACMFORMATENUMCBW>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumW(
                had: HACMDRIVER,
                pafd: *mut tACMFORMATDETAILSW,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatEnumW(
            had.into_param().abi(),
            ::std::mem::transmute(pafd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatSuggest<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    pwfxsrc: *mut WAVEFORMATEX,
    pwfxdst: *mut WAVEFORMATEX,
    cbwfxdst: u32,
    fdwsuggest: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatSuggest(
                had: HACMDRIVER,
                pwfxsrc: *mut WAVEFORMATEX,
                pwfxdst: *mut WAVEFORMATEX,
                cbwfxdst: u32,
                fdwsuggest: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatSuggest(
            had.into_param().abi(),
            ::std::mem::transmute(pwfxsrc),
            ::std::mem::transmute(pwfxdst),
            ::std::mem::transmute(cbwfxdst),
            ::std::mem::transmute(fdwsuggest),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFORMATTAGDETAILSA,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagDetailsA(
                had: HACMDRIVER,
                paftd: *mut ACMFORMATTAGDETAILSA,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatTagDetailsA(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatTagDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFORMATTAGDETAILSW,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagDetailsW(
                had: HACMDRIVER,
                paftd: *mut ACMFORMATTAGDETAILSW,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatTagDetailsW(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFORMATTAGDETAILSA,
    fncallback: ::std::option::Option<ACMFORMATTAGENUMCBA>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumA(
                had: HACMDRIVER,
                paftd: *mut ACMFORMATTAGDETAILSA,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatTagEnumA(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    had: Param0,
    paftd: *mut ACMFORMATTAGDETAILSW,
    fncallback: ::std::option::Option<ACMFORMATTAGENUMCBW>,
    dwinstance: usize,
    fdwenum: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumW(
                had: HACMDRIVER,
                paftd: *mut ACMFORMATTAGDETAILSW,
                fncallback: ::windows::runtime::RawPtr,
                dwinstance: usize,
                fdwenum: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmFormatTagEnumW(
            had.into_param().abi(),
            ::std::mem::transmute(paftd),
            ::std::mem::transmute(fncallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmGetVersion() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmGetVersion() -> u32;
        }
        ::std::mem::transmute(acmGetVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmMetrics<'a, Param0: ::windows::runtime::IntoParam<'a, HACMOBJ>>(
    hao: Param0,
    umetric: u32,
    pmetric: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(acmMetrics(
            hao.into_param().abi(),
            ::std::mem::transmute(umetric),
            ::std::mem::transmute(pmetric),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamClose<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(
    has: Param0,
    fdwclose: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
        }
        ::std::mem::transmute(acmStreamClose(
            has.into_param().abi(),
            ::std::mem::transmute(fdwclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamConvert<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(
    has: Param0,
    pash: *mut ACMSTREAMHEADER,
    fdwconvert: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamConvert(
                has: HACMSTREAM,
                pash: *mut ACMSTREAMHEADER,
                fdwconvert: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamConvert(
            has.into_param().abi(),
            ::std::mem::transmute(pash),
            ::std::mem::transmute(fdwconvert),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmStreamMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    has: Param0,
    umsg: u32,
    lparam1: Param2,
    lparam2: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamMessage(
                has: HACMSTREAM,
                umsg: u32,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamMessage(
            has.into_param().abi(),
            ::std::mem::transmute(umsg),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamOpen<'a, Param1: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(
    phas: *mut isize,
    had: Param1,
    pwfxsrc: *mut WAVEFORMATEX,
    pwfxdst: *mut WAVEFORMATEX,
    pwfltr: *mut WAVEFILTER,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamOpen(
                phas: *mut isize,
                had: HACMDRIVER,
                pwfxsrc: *mut WAVEFORMATEX,
                pwfxdst: *mut WAVEFORMATEX,
                pwfltr: *mut WAVEFILTER,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamOpen(
            ::std::mem::transmute(phas),
            had.into_param().abi(),
            ::std::mem::transmute(pwfxsrc),
            ::std::mem::transmute(pwfxdst),
            ::std::mem::transmute(pwfltr),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(
    has: Param0,
    pash: *mut ACMSTREAMHEADER,
    fdwprepare: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamPrepareHeader(
                has: HACMSTREAM,
                pash: *mut ACMSTREAMHEADER,
                fdwprepare: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamPrepareHeader(
            has.into_param().abi(),
            ::std::mem::transmute(pash),
            ::std::mem::transmute(fdwprepare),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamReset<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(
    has: Param0,
    fdwreset: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
        }
        ::std::mem::transmute(acmStreamReset(
            has.into_param().abi(),
            ::std::mem::transmute(fdwreset),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamSize<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(
    has: Param0,
    cbinput: u32,
    pdwoutputbytes: *mut u32,
    fdwsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamSize(
                has: HACMSTREAM,
                cbinput: u32,
                pdwoutputbytes: *mut u32,
                fdwsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamSize(
            has.into_param().abi(),
            ::std::mem::transmute(cbinput),
            ::std::mem::transmute(pdwoutputbytes),
            ::std::mem::transmute(fdwsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamUnprepareHeader<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>,
>(
    has: Param0,
    pash: *mut ACMSTREAMHEADER,
    fdwunprepare: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamUnprepareHeader(
                has: HACMSTREAM,
                pash: *mut ACMSTREAMHEADER,
                fdwunprepare: u32,
            ) -> u32;
        }
        ::std::mem::transmute(acmStreamUnprepareHeader(
            has.into_param().abi(),
            ::std::mem::transmute(pash),
            ::std::mem::transmute(fdwunprepare),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32;
        }
        ::std::mem::transmute(auxGetDevCapsA(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pac),
            ::std::mem::transmute(cbac),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32;
        }
        ::std::mem::transmute(auxGetDevCapsW(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pac),
            ::std::mem::transmute(cbac),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn auxGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxGetNumDevs() -> u32;
        }
        ::std::mem::transmute(auxGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32;
        }
        ::std::mem::transmute(auxGetVolume(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pdwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::std::mem::transmute(auxOutMessage(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32;
        }
        ::std::mem::transmute(auxSetVolume(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(dwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capCreateCaptureWindowA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    lpszwindowname: Param0,
    dwstyle: u32,
    x: i32,
    y: i32,
    nwidth: i32,
    nheight: i32,
    hwndparent: Param6,
    nid: i32,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capCreateCaptureWindowA(
                lpszwindowname: super::super::Foundation::PSTR,
                dwstyle: u32,
                x: i32,
                y: i32,
                nwidth: i32,
                nheight: i32,
                hwndparent: super::super::Foundation::HWND,
                nid: i32,
            ) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(capCreateCaptureWindowA(
            lpszwindowname.into_param().abi(),
            ::std::mem::transmute(dwstyle),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(nwidth),
            ::std::mem::transmute(nheight),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(nid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capCreateCaptureWindowW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    lpszwindowname: Param0,
    dwstyle: u32,
    x: i32,
    y: i32,
    nwidth: i32,
    nheight: i32,
    hwndparent: Param6,
    nid: i32,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capCreateCaptureWindowW(
                lpszwindowname: super::super::Foundation::PWSTR,
                dwstyle: u32,
                x: i32,
                y: i32,
                nwidth: i32,
                nheight: i32,
                hwndparent: super::super::Foundation::HWND,
                nid: i32,
            ) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(capCreateCaptureWindowW(
            lpszwindowname.into_param().abi(),
            ::std::mem::transmute(dwstyle),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(nwidth),
            ::std::mem::transmute(nheight),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(nid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capGetDriverDescriptionA(
    wdriverindex: u32,
    lpszname: super::super::Foundation::PSTR,
    cbname: i32,
    lpszver: super::super::Foundation::PSTR,
    cbver: i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capGetDriverDescriptionA(
                wdriverindex: u32,
                lpszname: super::super::Foundation::PSTR,
                cbname: i32,
                lpszver: super::super::Foundation::PSTR,
                cbver: i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(capGetDriverDescriptionA(
            ::std::mem::transmute(wdriverindex),
            ::std::mem::transmute(lpszname),
            ::std::mem::transmute(cbname),
            ::std::mem::transmute(lpszver),
            ::std::mem::transmute(cbver),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn capGetDriverDescriptionW(
    wdriverindex: u32,
    lpszname: super::super::Foundation::PWSTR,
    cbname: i32,
    lpszver: super::super::Foundation::PWSTR,
    cbver: i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn capGetDriverDescriptionW(
                wdriverindex: u32,
                lpszname: super::super::Foundation::PWSTR,
                cbname: i32,
                lpszver: super::super::Foundation::PWSTR,
                cbver: i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(capGetDriverDescriptionW(
            ::std::mem::transmute(wdriverindex),
            ::std::mem::transmute(lpszname),
            ::std::mem::transmute(cbname),
            ::std::mem::transmute(lpszver),
            ::std::mem::transmute(cbver),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetDevCapsA(ujoyid: usize, pjc: *mut JOYCAPSA, cbjc: u32) -> u32;
        }
        ::std::mem::transmute(joyGetDevCapsA(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(pjc),
            ::std::mem::transmute(cbjc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetDevCapsW(ujoyid: usize, pjc: *mut JOYCAPSW, cbjc: u32) -> u32;
        }
        ::std::mem::transmute(joyGetDevCapsW(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(pjc),
            ::std::mem::transmute(cbjc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetNumDevs() -> u32;
        }
        ::std::mem::transmute(joyGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetPos(ujoyid: u32, pji: *mut JOYINFO) -> u32;
        }
        ::std::mem::transmute(joyGetPos(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(pji),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetPosEx(ujoyid: u32, pji: *mut JOYINFOEX) -> u32;
        }
        ::std::mem::transmute(joyGetPosEx(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(pji),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyGetThreshold(ujoyid: u32, puthreshold: *mut u32) -> u32;
        }
        ::std::mem::transmute(joyGetThreshold(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(puthreshold),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joyReleaseCapture(ujoyid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joyReleaseCapture(ujoyid: u32) -> u32;
        }
        ::std::mem::transmute(joyReleaseCapture(::std::mem::transmute(ujoyid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn joySetCapture<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hwnd: Param0,
    ujoyid: u32,
    uperiod: u32,
    fchanged: Param3,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joySetCapture(
                hwnd: super::super::Foundation::HWND,
                ujoyid: u32,
                uperiod: u32,
                fchanged: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(joySetCapture(
            hwnd.into_param().abi(),
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(uperiod),
            fchanged.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn joySetThreshold(ujoyid: u32, uthreshold: u32) -> u32;
        }
        ::std::mem::transmute(joySetThreshold(
            ::std::mem::transmute(ujoyid),
            ::std::mem::transmute(uthreshold),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiConnect<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HMIDI>,
    Param1: ::windows::runtime::IntoParam<'a, HMIDIOUT>,
>(
    hmi: Param0,
    hmo: Param1,
    preserved: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(midiConnect(
            hmi.into_param().abi(),
            hmo.into_param().abi(),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiDisconnect<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HMIDI>,
    Param1: ::windows::runtime::IntoParam<'a, HMIDIOUT>,
>(
    hmi: Param0,
    hmo: Param1,
    preserved: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiDisconnect(
                hmi: HMIDI,
                hmo: HMIDIOUT,
                preserved: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(midiDisconnect(
            hmi.into_param().abi(),
            hmo.into_param().abi(),
            ::std::mem::transmute(preserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInAddBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiInAddBuffer(
            hmi.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInClose(hmi: HMIDIIN) -> u32;
        }
        ::std::mem::transmute(midiInClose(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32;
        }
        ::std::mem::transmute(midiInGetDevCapsA(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pmic),
            ::std::mem::transmute(cbmic),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32;
        }
        ::std::mem::transmute(midiInGetDevCapsW(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pmic),
            ::std::mem::transmute(cbmic),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInGetErrorTextA(
    mmrerror: u32,
    psztext: super::super::Foundation::PSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextA(
                mmrerror: u32,
                psztext: super::super::Foundation::PSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiInGetErrorTextA(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInGetErrorTextW(
    mmrerror: u32,
    psztext: super::super::Foundation::PWSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextW(
                mmrerror: u32,
                psztext: super::super::Foundation::PWSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiInGetErrorTextW(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
    pudeviceid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
        }
        ::std::mem::transmute(midiInGetID(
            hmi.into_param().abi(),
            ::std::mem::transmute(pudeviceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetNumDevs() -> u32;
        }
        ::std::mem::transmute(midiInGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
    umsg: u32,
    dw1: usize,
    dw2: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::std::mem::transmute(midiInMessage(
            hmi.into_param().abi(),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInOpen(
    phmi: *mut HMIDIIN,
    udeviceid: u32,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: MIDI_WAVE_OPEN_TYPE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInOpen(
                phmi: *mut HMIDIIN,
                udeviceid: u32,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: MIDI_WAVE_OPEN_TYPE,
            ) -> u32;
        }
        ::std::mem::transmute(midiInOpen(
            ::std::mem::transmute(phmi),
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiInPrepareHeader(
            hmi.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInReset<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInReset(hmi: HMIDIIN) -> u32;
        }
        ::std::mem::transmute(midiInReset(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInStart<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInStart(hmi: HMIDIIN) -> u32;
        }
        ::std::mem::transmute(midiInStart(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInStop<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInStop(hmi: HMIDIIN) -> u32;
        }
        ::std::mem::transmute(midiInStop(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(
    hmi: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiInUnprepareHeader(
            hmi.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutCacheDrumPatches<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    upatch: u32,
    pwkya: *const u16,
    fucache: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCacheDrumPatches(
                hmo: HMIDIOUT,
                upatch: u32,
                pwkya: *const u16,
                fucache: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiOutCacheDrumPatches(
            hmo.into_param().abi(),
            ::std::mem::transmute(upatch),
            ::std::mem::transmute(pwkya),
            ::std::mem::transmute(fucache),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutCachePatches<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    ubank: u32,
    pwpa: *const u16,
    fucache: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCachePatches(
                hmo: HMIDIOUT,
                ubank: u32,
                pwpa: *const u16,
                fucache: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiOutCachePatches(
            hmo.into_param().abi(),
            ::std::mem::transmute(ubank),
            ::std::mem::transmute(pwpa),
            ::std::mem::transmute(fucache),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutClose(hmo: HMIDIOUT) -> u32;
        }
        ::std::mem::transmute(midiOutClose(hmo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32;
        }
        ::std::mem::transmute(midiOutGetDevCapsA(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pmoc),
            ::std::mem::transmute(cbmoc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32;
        }
        ::std::mem::transmute(midiOutGetDevCapsW(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pmoc),
            ::std::mem::transmute(cbmoc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutGetErrorTextA(
    mmrerror: u32,
    psztext: super::super::Foundation::PSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextA(
                mmrerror: u32,
                psztext: super::super::Foundation::PSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiOutGetErrorTextA(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutGetErrorTextW(
    mmrerror: u32,
    psztext: super::super::Foundation::PWSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextW(
                mmrerror: u32,
                psztext: super::super::Foundation::PWSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiOutGetErrorTextW(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    pudeviceid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
        }
        ::std::mem::transmute(midiOutGetID(
            hmo.into_param().abi(),
            ::std::mem::transmute(pudeviceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetNumDevs() -> u32;
        }
        ::std::mem::transmute(midiOutGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    pdwvolume: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
        }
        ::std::mem::transmute(midiOutGetVolume(
            hmo.into_param().abi(),
            ::std::mem::transmute(pdwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutLongMsg<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    pmh: *const MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiOutLongMsg(
            hmo.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    umsg: u32,
    dw1: usize,
    dw2: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::std::mem::transmute(midiOutMessage(
            hmo.into_param().abi(),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutOpen(
    phmo: *mut HMIDIOUT,
    udeviceid: u32,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: MIDI_WAVE_OPEN_TYPE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutOpen(
                phmo: *mut HMIDIOUT,
                udeviceid: u32,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: MIDI_WAVE_OPEN_TYPE,
            ) -> u32;
        }
        ::std::mem::transmute(midiOutOpen(
            ::std::mem::transmute(phmo),
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiOutPrepareHeader(
            hmo.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutReset<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutReset(hmo: HMIDIOUT) -> u32;
        }
        ::std::mem::transmute(midiOutReset(hmo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutSetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    dwvolume: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
        }
        ::std::mem::transmute(midiOutSetVolume(
            hmo.into_param().abi(),
            ::std::mem::transmute(dwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutShortMsg<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    dwmsg: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
        }
        ::std::mem::transmute(midiOutShortMsg(
            hmo.into_param().abi(),
            ::std::mem::transmute(dwmsg),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(
    hmo: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiOutUnprepareHeader(
            hmo.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamClose(hms: HMIDISTRM) -> u32;
        }
        ::std::mem::transmute(midiStreamClose(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamOpen(
    phms: *mut HMIDISTRM,
    pudeviceid: *mut u32,
    cmidi: u32,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamOpen(
                phms: *mut HMIDISTRM,
                pudeviceid: *mut u32,
                cmidi: u32,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: u32,
            ) -> u32;
        }
        ::std::mem::transmute(midiStreamOpen(
            ::std::mem::transmute(phms),
            ::std::mem::transmute(pudeviceid),
            ::std::mem::transmute(cmidi),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiStreamOut<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
    pmh: *mut MIDIHDR,
    cbmh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::std::mem::transmute(midiStreamOut(
            hms.into_param().abi(),
            ::std::mem::transmute(pmh),
            ::std::mem::transmute(cbmh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamPause<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamPause(hms: HMIDISTRM) -> u32;
        }
        ::std::mem::transmute(midiStreamPause(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
    lpmmt: *mut MMTIME,
    cbmmt: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut MMTIME, cbmmt: u32) -> u32;
        }
        ::std::mem::transmute(midiStreamPosition(
            hms.into_param().abi(),
            ::std::mem::transmute(lpmmt),
            ::std::mem::transmute(cbmmt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamProperty<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
    lppropdata: *mut u8,
    dwproperty: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
        }
        ::std::mem::transmute(midiStreamProperty(
            hms.into_param().abi(),
            ::std::mem::transmute(lppropdata),
            ::std::mem::transmute(dwproperty),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamRestart<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamRestart(hms: HMIDISTRM) -> u32;
        }
        ::std::mem::transmute(midiStreamRestart(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamStop<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(
    hms: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamStop(hms: HMIDISTRM) -> u32;
        }
        ::std::mem::transmute(midiStreamStop(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXER>>(
    hmx: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerClose(hmx: HMIXER) -> u32;
        }
        ::std::mem::transmute(mixerClose(hmx.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxcd: *mut MIXERCONTROLDETAILS,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetControlDetailsA(
                hmxobj: HMIXEROBJ,
                pmxcd: *mut MIXERCONTROLDETAILS,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerGetControlDetailsA(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxcd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxcd: *mut MIXERCONTROLDETAILS,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetControlDetailsW(
                hmxobj: HMIXEROBJ,
                pmxcd: *mut MIXERCONTROLDETAILS,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerGetControlDetailsW(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxcd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32;
        }
        ::std::mem::transmute(mixerGetDevCapsA(
            ::std::mem::transmute(umxid),
            ::std::mem::transmute(pmxcaps),
            ::std::mem::transmute(cbmxcaps),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32;
        }
        ::std::mem::transmute(mixerGetDevCapsW(
            ::std::mem::transmute(umxid),
            ::std::mem::transmute(pmxcaps),
            ::std::mem::transmute(cbmxcaps),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pumxid: *mut u32,
    fdwid: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
        }
        ::std::mem::transmute(mixerGetID(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pumxid),
            ::std::mem::transmute(fdwid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineControlsA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxlc: *mut MIXERLINECONTROLSA,
    fdwcontrols: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineControlsA(
                hmxobj: HMIXEROBJ,
                pmxlc: *mut MIXERLINECONTROLSA,
                fdwcontrols: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerGetLineControlsA(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxlc),
            ::std::mem::transmute(fdwcontrols),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetLineControlsW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxlc: *mut MIXERLINECONTROLSW,
    fdwcontrols: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineControlsW(
                hmxobj: HMIXEROBJ,
                pmxlc: *mut MIXERLINECONTROLSW,
                fdwcontrols: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerGetLineControlsW(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxlc),
            ::std::mem::transmute(fdwcontrols),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxl: *mut MIXERLINEA,
    fdwinfo: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
        }
        ::std::mem::transmute(mixerGetLineInfoA(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxl),
            ::std::mem::transmute(fdwinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetLineInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxl: *mut MIXERLINEW,
    fdwinfo: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
        }
        ::std::mem::transmute(mixerGetLineInfoW(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxl),
            ::std::mem::transmute(fdwinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetNumDevs() -> u32;
        }
        ::std::mem::transmute(mixerGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXER>>(
    hmx: Param0,
    umsg: u32,
    dwparam1: usize,
    dwparam2: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
        }
        ::std::mem::transmute(mixerMessage(
            hmx.into_param().abi(),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dwparam1),
            ::std::mem::transmute(dwparam2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerOpen(
    phmx: *mut isize,
    umxid: u32,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerOpen(
                phmx: *mut isize,
                umxid: u32,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerOpen(
            ::std::mem::transmute(phmx),
            ::std::mem::transmute(umxid),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerSetControlDetails<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(
    hmxobj: Param0,
    pmxcd: *const MIXERCONTROLDETAILS,
    fdwdetails: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerSetControlDetails(
                hmxobj: HMIXEROBJ,
                pmxcd: *const MIXERCONTROLDETAILS,
                fdwdetails: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mixerSetControlDetails(
            hmxobj.into_param().abi(),
            ::std::mem::transmute(pmxcd),
            ::std::mem::transmute(fdwdetails),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmDrvInstall<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HDRVR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hdriver: Param0,
    wszdrventry: Param1,
    drvmessage: ::std::option::Option<DRIVERMSGPROC>,
    wflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmDrvInstall(
                hdriver: HDRVR,
                wszdrventry: super::super::Foundation::PWSTR,
                drvmessage: ::windows::runtime::RawPtr,
                wflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmDrvInstall(
            hdriver.into_param().abi(),
            wszdrventry.into_param().abi(),
            ::std::mem::transmute(drvmessage),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmGetCurrentTask() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmGetCurrentTask() -> u32;
        }
        ::std::mem::transmute(mmGetCurrentTask())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmTaskBlock(h: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskBlock(h: u32);
        }
        ::std::mem::transmute(mmTaskBlock(::std::mem::transmute(h)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmTaskCreate(
    lpfn: ::std::option::Option<LPTASKCALLBACK>,
    lph: *mut super::super::Foundation::HANDLE,
    dwinst: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskCreate(
                lpfn: ::windows::runtime::RawPtr,
                lph: *mut super::super::Foundation::HANDLE,
                dwinst: usize,
            ) -> u32;
        }
        ::std::mem::transmute(mmTaskCreate(
            ::std::mem::transmute(lpfn),
            ::std::mem::transmute(lph),
            ::std::mem::transmute(dwinst),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmTaskSignal(h: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskSignal(h: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(mmTaskSignal(::std::mem::transmute(h)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmTaskYield() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmTaskYield();
        }
        ::std::mem::transmute(mmTaskYield())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioAdvance<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmioinfo: *const MMIOINFO,
    fuadvance: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioAdvance(
                hmmio: HMMIO,
                pmmioinfo: *const ::std::mem::ManuallyDrop<MMIOINFO>,
                fuadvance: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioAdvance(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fuadvance),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioAscend<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmcki: *const MMCKINFO,
    fuascend: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioAscend(hmmio: HMMIO, pmmcki: *const MMCKINFO, fuascend: u32) -> u32;
        }
        ::std::mem::transmute(mmioAscend(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmcki),
            ::std::mem::transmute(fuascend),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    fuclose: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioClose(hmmio: HMMIO, fuclose: u32) -> u32;
        }
        ::std::mem::transmute(mmioClose(
            hmmio.into_param().abi(),
            ::std::mem::transmute(fuclose),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioCreateChunk<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmcki: *const MMCKINFO,
    fucreate: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioCreateChunk(hmmio: HMMIO, pmmcki: *const MMCKINFO, fucreate: u32) -> u32;
        }
        ::std::mem::transmute(mmioCreateChunk(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmcki),
            ::std::mem::transmute(fucreate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioDescend<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmcki: *mut MMCKINFO,
    pmmckiparent: *const MMCKINFO,
    fudescend: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioDescend(
                hmmio: HMMIO,
                pmmcki: *mut MMCKINFO,
                pmmckiparent: *const MMCKINFO,
                fudescend: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioDescend(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmcki),
            ::std::mem::transmute(pmmckiparent),
            ::std::mem::transmute(fudescend),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioFlush<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    fuflush: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioFlush(hmmio: HMMIO, fuflush: u32) -> u32;
        }
        ::std::mem::transmute(mmioFlush(
            hmmio.into_param().abi(),
            ::std::mem::transmute(fuflush),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmioinfo: *mut MMIOINFO,
    fuinfo: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioGetInfo(
                hmmio: HMMIO,
                pmmioinfo: *mut ::std::mem::ManuallyDrop<MMIOINFO>,
                fuinfo: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioGetInfo(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fuinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioInstallIOProcA(
    fccioproc: u32,
    pioproc: ::std::option::Option<LPMMIOPROC>,
    dwflags: u32,
) -> ::std::option::Option<LPMMIOPROC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioInstallIOProcA(
                fccioproc: u32,
                pioproc: ::windows::runtime::RawPtr,
                dwflags: u32,
            ) -> ::std::option::Option<LPMMIOPROC>;
        }
        ::std::mem::transmute(mmioInstallIOProcA(
            ::std::mem::transmute(fccioproc),
            ::std::mem::transmute(pioproc),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioInstallIOProcW(
    fccioproc: u32,
    pioproc: ::std::option::Option<LPMMIOPROC>,
    dwflags: u32,
) -> ::std::option::Option<LPMMIOPROC> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioInstallIOProcW(
                fccioproc: u32,
                pioproc: ::windows::runtime::RawPtr,
                dwflags: u32,
            ) -> ::std::option::Option<LPMMIOPROC>;
        }
        ::std::mem::transmute(mmioInstallIOProcW(
            ::std::mem::transmute(fccioproc),
            ::std::mem::transmute(pioproc),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioOpenA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pszfilename: Param0,
    pmmioinfo: *mut MMIOINFO,
    fdwopen: u32,
) -> HMMIO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioOpenA(
                pszfilename: super::super::Foundation::PSTR,
                pmmioinfo: *mut ::std::mem::ManuallyDrop<MMIOINFO>,
                fdwopen: u32,
            ) -> HMMIO;
        }
        ::std::mem::transmute(mmioOpenA(
            pszfilename.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioOpenW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszfilename: Param0,
    pmmioinfo: *mut MMIOINFO,
    fdwopen: u32,
) -> HMMIO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioOpenW(
                pszfilename: super::super::Foundation::PWSTR,
                pmmioinfo: *mut ::std::mem::ManuallyDrop<MMIOINFO>,
                fdwopen: u32,
            ) -> HMMIO;
        }
        ::std::mem::transmute(mmioOpenW(
            pszfilename.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioRead<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pch: *mut i8,
    cch: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRead(hmmio: HMMIO, pch: *mut i8, cch: i32) -> i32;
        }
        ::std::mem::transmute(mmioRead(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pch),
            ::std::mem::transmute(cch),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioRenameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pszfilename: Param0,
    psznewfilename: Param1,
    pmmioinfo: *const MMIOINFO,
    fdwrename: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRenameA(
                pszfilename: super::super::Foundation::PSTR,
                psznewfilename: super::super::Foundation::PSTR,
                pmmioinfo: *const ::std::mem::ManuallyDrop<MMIOINFO>,
                fdwrename: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioRenameA(
            pszfilename.into_param().abi(),
            psznewfilename.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fdwrename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioRenameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszfilename: Param0,
    psznewfilename: Param1,
    pmmioinfo: *const MMIOINFO,
    fdwrename: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioRenameW(
                pszfilename: super::super::Foundation::PWSTR,
                psznewfilename: super::super::Foundation::PWSTR,
                pmmioinfo: *const ::std::mem::ManuallyDrop<MMIOINFO>,
                fdwrename: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioRenameW(
            pszfilename.into_param().abi(),
            psznewfilename.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fdwrename),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mmioSeek<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    loffset: i32,
    iorigin: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSeek(hmmio: HMMIO, loffset: i32, iorigin: i32) -> i32;
        }
        ::std::mem::transmute(mmioSeek(
            hmmio.into_param().abi(),
            ::std::mem::transmute(loffset),
            ::std::mem::transmute(iorigin),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioSendMessage<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HMMIO>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>,
>(
    hmmio: Param0,
    umsg: u32,
    lparam1: Param2,
    lparam2: Param3,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSendMessage(
                hmmio: HMMIO,
                umsg: u32,
                lparam1: super::super::Foundation::LPARAM,
                lparam2: super::super::Foundation::LPARAM,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(mmioSendMessage(
            hmmio.into_param().abi(),
            ::std::mem::transmute(umsg),
            lparam1.into_param().abi(),
            lparam2.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioSetBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pchbuffer: super::super::Foundation::PSTR,
    cchbuffer: i32,
    fubuffer: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSetBuffer(
                hmmio: HMMIO,
                pchbuffer: super::super::Foundation::PSTR,
                cchbuffer: i32,
                fubuffer: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioSetBuffer(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pchbuffer),
            ::std::mem::transmute(cchbuffer),
            ::std::mem::transmute(fubuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_CoreAudio"))]
#[inline]
pub unsafe fn mmioSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HMMIO>>(
    hmmio: Param0,
    pmmioinfo: *const MMIOINFO,
    fuinfo: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioSetInfo(
                hmmio: HMMIO,
                pmmioinfo: *const ::std::mem::ManuallyDrop<MMIOINFO>,
                fuinfo: u32,
            ) -> u32;
        }
        ::std::mem::transmute(mmioSetInfo(
            hmmio.into_param().abi(),
            ::std::mem::transmute(pmmioinfo),
            ::std::mem::transmute(fuinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioStringToFOURCCA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    sz: Param0,
    uflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioStringToFOURCCA(sz: super::super::Foundation::PSTR, uflags: u32) -> u32;
        }
        ::std::mem::transmute(mmioStringToFOURCCA(
            sz.into_param().abi(),
            ::std::mem::transmute(uflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioStringToFOURCCW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    sz: Param0,
    uflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioStringToFOURCCW(sz: super::super::Foundation::PWSTR, uflags: u32) -> u32;
        }
        ::std::mem::transmute(mmioStringToFOURCCW(
            sz.into_param().abi(),
            ::std::mem::transmute(uflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mmioWrite<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, HMMIO>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hmmio: Param0,
    pch: Param1,
    cch: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mmioWrite(hmmio: HMMIO, pch: super::super::Foundation::PSTR, cch: i32) -> i32;
        }
        ::std::mem::transmute(mmioWrite(
            hmmio.into_param().abi(),
            pch.into_param().abi(),
            ::std::mem::transmute(cch),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct s_RIFFWAVE_inst {
    pub bUnshiftedNote: u8,
    pub chFineTune: super::super::Foundation::CHAR,
    pub chGain: super::super::Foundation::CHAR,
    pub bLowNote: u8,
    pub bHighNote: u8,
    pub bLowVelocity: u8,
    pub bHighVelocity: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl s_RIFFWAVE_inst {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for s_RIFFWAVE_inst {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for s_RIFFWAVE_inst {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("s_RIFFWAVE_inst")
            .field("bUnshiftedNote", &self.bUnshiftedNote)
            .field("chFineTune", &self.chFineTune)
            .field("chGain", &self.chGain)
            .field("bLowNote", &self.bLowNote)
            .field("bHighNote", &self.bHighNote)
            .field("bLowVelocity", &self.bLowVelocity)
            .field("bHighVelocity", &self.bHighVelocity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for s_RIFFWAVE_inst {
    fn eq(&self, other: &Self) -> bool {
        self.bUnshiftedNote == other.bUnshiftedNote
            && self.chFineTune == other.chFineTune
            && self.chGain == other.chGain
            && self.bLowNote == other.bLowNote
            && self.bHighNote == other.bHighNote
            && self.bLowVelocity == other.bLowVelocity
            && self.bHighVelocity == other.bHighVelocity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for s_RIFFWAVE_inst {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for s_RIFFWAVE_inst {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndOpenSound<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    eventname: Param0,
    appname: Param1,
    flags: i32,
    filehandle: *mut super::super::Foundation::HANDLE,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndOpenSound(
                eventname: super::super::Foundation::PWSTR,
                appname: super::super::Foundation::PWSTR,
                flags: i32,
                filehandle: *mut super::super::Foundation::HANDLE,
            ) -> i32;
        }
        ::std::mem::transmute(sndOpenSound(
            eventname.into_param().abi(),
            appname.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(filehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pszsound: Param0,
    fusound: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundA(
                pszsound: super::super::Foundation::PSTR,
                fusound: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(sndPlaySoundA(
            pszsound.into_param().abi(),
            ::std::mem::transmute(fusound),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszsound: Param0,
    fusound: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundW(
                pszsound: super::super::Foundation::PWSTR,
                fusound: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(sndPlaySoundW(
            pszsound.into_param().abi(),
            ::std::mem::transmute(fusound),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl tACMDRVFORMATSUGGEST {}
impl ::std::default::Default for tACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tACMDRVFORMATSUGGEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tACMDRVFORMATSUGGEST {}
unsafe impl ::windows::runtime::Abi for tACMDRVFORMATSUGGEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct tACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: super::super::Foundation::PSTR,
    pub pszAliasName: super::super::Foundation::PSTR,
    pub dnDevNode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl tACMDRVOPENDESCA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for tACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for tACMDRVOPENDESCA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for tACMDRVOPENDESCA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for tACMDRVOPENDESCA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct tACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: super::super::Foundation::PWSTR,
    pub pszAliasName: super::super::Foundation::PWSTR,
    pub dnDevNode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl tACMDRVOPENDESCW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for tACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for tACMDRVOPENDESCW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for tACMDRVOPENDESCW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for tACMDRVOPENDESCW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tACMDRVSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: *mut u8,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: *mut u8,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub fdwConvert: u32,
    pub padshNext: *mut tACMDRVSTREAMHEADER,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub fdwPrepared: u32,
    pub dwPrepared: usize,
    pub pbPreparedSrc: *mut u8,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: *mut u8,
    pub cbPreparedDstLength: u32,
}
impl tACMDRVSTREAMHEADER {}
impl ::std::default::Default for tACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tACMDRVSTREAMHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tACMDRVSTREAMHEADER {}
unsafe impl ::windows::runtime::Abi for tACMDRVSTREAMHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tACMDRVSTREAMINSTANCE {
    pub cbStruct: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub pwfltr: *mut WAVEFILTER,
    pub dwCallback: usize,
    pub dwInstance: usize,
    pub fdwOpen: u32,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub has: HACMSTREAM,
}
impl tACMDRVSTREAMINSTANCE {}
impl ::std::default::Default for tACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tACMDRVSTREAMINSTANCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tACMDRVSTREAMINSTANCE {}
unsafe impl ::windows::runtime::Abi for tACMDRVSTREAMINSTANCE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl tACMDRVSTREAMSIZE {}
impl ::std::default::Default for tACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tACMDRVSTREAMSIZE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tACMDRVSTREAMSIZE {}
unsafe impl ::windows::runtime::Abi for tACMDRVSTREAMSIZE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl tACMFORMATDETAILSW {}
impl ::std::default::Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tACMFORMATDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tACMFORMATDETAILSW {}
unsafe impl ::windows::runtime::Abi for tACMFORMATDETAILSW {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeBeginPeriod(uperiod: u32) -> u32;
        }
        ::std::mem::transmute(timeBeginPeriod(::std::mem::transmute(uperiod)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeEndPeriod(uperiod: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeEndPeriod(uperiod: u32) -> u32;
        }
        ::std::mem::transmute(timeEndPeriod(::std::mem::transmute(uperiod)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32;
        }
        ::std::mem::transmute(timeGetDevCaps(
            ::std::mem::transmute(ptc),
            ::std::mem::transmute(cbtc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32;
        }
        ::std::mem::transmute(timeGetSystemTime(
            ::std::mem::transmute(pmmt),
            ::std::mem::transmute(cbmmt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn timeGetTime() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn timeGetTime() -> u32;
        }
        ::std::mem::transmute(timeGetTime())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInAddBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveInAddBuffer(
            hwi.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInClose<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInClose(hwi: HWAVEIN) -> u32;
        }
        ::std::mem::transmute(waveInClose(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32;
        }
        ::std::mem::transmute(waveInGetDevCapsA(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwic),
            ::std::mem::transmute(cbwic),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32;
        }
        ::std::mem::transmute(waveInGetDevCapsW(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwic),
            ::std::mem::transmute(cbwic),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInGetErrorTextA(
    mmrerror: u32,
    psztext: super::super::Foundation::PSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextA(
                mmrerror: u32,
                psztext: super::super::Foundation::PSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(waveInGetErrorTextA(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInGetErrorTextW(
    mmrerror: u32,
    psztext: super::super::Foundation::PWSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextW(
                mmrerror: u32,
                psztext: super::super::Foundation::PWSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(waveInGetErrorTextW(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    pudeviceid: *const u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
        }
        ::std::mem::transmute(waveInGetID(
            hwi.into_param().abi(),
            ::std::mem::transmute(pudeviceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetNumDevs() -> u32;
        }
        ::std::mem::transmute(waveInGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    pmmt: *mut MMTIME,
    cbmmt: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut MMTIME, cbmmt: u32) -> u32;
        }
        ::std::mem::transmute(waveInGetPosition(
            hwi.into_param().abi(),
            ::std::mem::transmute(pmmt),
            ::std::mem::transmute(cbmmt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    umsg: u32,
    dw1: usize,
    dw2: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::std::mem::transmute(waveInMessage(
            hwi.into_param().abi(),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInOpen(
    phwi: *mut HWAVEIN,
    udeviceid: u32,
    pwfx: *const WAVEFORMATEX,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: MIDI_WAVE_OPEN_TYPE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInOpen(
                phwi: *mut HWAVEIN,
                udeviceid: u32,
                pwfx: *const WAVEFORMATEX,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: MIDI_WAVE_OPEN_TYPE,
            ) -> u32;
        }
        ::std::mem::transmute(waveInOpen(
            ::std::mem::transmute(phwi),
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwfx),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveInPrepareHeader(
            hwi.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInReset<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInReset(hwi: HWAVEIN) -> u32;
        }
        ::std::mem::transmute(waveInReset(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInStart<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInStart(hwi: HWAVEIN) -> u32;
        }
        ::std::mem::transmute(waveInStart(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInStop<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInStop(hwi: HWAVEIN) -> u32;
        }
        ::std::mem::transmute(waveInStop(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(
    hwi: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveInUnprepareHeader(
            hwi.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutBreakLoop<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
        }
        ::std::mem::transmute(waveOutBreakLoop(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutClose<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutClose(hwo: HWAVEOUT) -> u32;
        }
        ::std::mem::transmute(waveOutClose(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetDevCapsA(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwoc),
            ::std::mem::transmute(cbwoc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetDevCapsW(
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwoc),
            ::std::mem::transmute(cbwoc),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutGetErrorTextA(
    mmrerror: u32,
    psztext: super::super::Foundation::PSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextA(
                mmrerror: u32,
                psztext: super::super::Foundation::PSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(waveOutGetErrorTextA(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutGetErrorTextW(
    mmrerror: u32,
    psztext: super::super::Foundation::PWSTR,
    cchtext: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextW(
                mmrerror: u32,
                psztext: super::super::Foundation::PWSTR,
                cchtext: u32,
            ) -> u32;
        }
        ::std::mem::transmute(waveOutGetErrorTextW(
            ::std::mem::transmute(mmrerror),
            ::std::mem::transmute(psztext),
            ::std::mem::transmute(cchtext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pudeviceid: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetID(
            hwo.into_param().abi(),
            ::std::mem::transmute(pudeviceid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetNumDevs() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetNumDevs() -> u32;
        }
        ::std::mem::transmute(waveOutGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPitch<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pdwpitch: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetPitch(
            hwo.into_param().abi(),
            ::std::mem::transmute(pdwpitch),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPlaybackRate<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pdwrate: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetPlaybackRate(
            hwo.into_param().abi(),
            ::std::mem::transmute(pdwrate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pmmt: *mut MMTIME,
    cbmmt: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut MMTIME, cbmmt: u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetPosition(
            hwo.into_param().abi(),
            ::std::mem::transmute(pmmt),
            ::std::mem::transmute(cbmmt),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pdwvolume: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
        }
        ::std::mem::transmute(waveOutGetVolume(
            hwo.into_param().abi(),
            ::std::mem::transmute(pdwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    umsg: u32,
    dw1: usize,
    dw2: usize,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::std::mem::transmute(waveOutMessage(
            hwo.into_param().abi(),
            ::std::mem::transmute(umsg),
            ::std::mem::transmute(dw1),
            ::std::mem::transmute(dw2),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutOpen(
    phwo: *mut HWAVEOUT,
    udeviceid: u32,
    pwfx: *const WAVEFORMATEX,
    dwcallback: usize,
    dwinstance: usize,
    fdwopen: MIDI_WAVE_OPEN_TYPE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutOpen(
                phwo: *mut HWAVEOUT,
                udeviceid: u32,
                pwfx: *const WAVEFORMATEX,
                dwcallback: usize,
                dwinstance: usize,
                fdwopen: MIDI_WAVE_OPEN_TYPE,
            ) -> u32;
        }
        ::std::mem::transmute(waveOutOpen(
            ::std::mem::transmute(phwo),
            ::std::mem::transmute(udeviceid),
            ::std::mem::transmute(pwfx),
            ::std::mem::transmute(dwcallback),
            ::std::mem::transmute(dwinstance),
            ::std::mem::transmute(fdwopen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutPause<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutPause(hwo: HWAVEOUT) -> u32;
        }
        ::std::mem::transmute(waveOutPause(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveOutPrepareHeader(
            hwo.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutReset<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutReset(hwo: HWAVEOUT) -> u32;
        }
        ::std::mem::transmute(waveOutReset(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutRestart<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutRestart(hwo: HWAVEOUT) -> u32;
        }
        ::std::mem::transmute(waveOutRestart(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetPitch<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    dwpitch: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
        }
        ::std::mem::transmute(waveOutSetPitch(
            hwo.into_param().abi(),
            ::std::mem::transmute(dwpitch),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetPlaybackRate<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    dwrate: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
        }
        ::std::mem::transmute(waveOutSetPlaybackRate(
            hwo.into_param().abi(),
            ::std::mem::transmute(dwrate),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    dwvolume: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
        }
        ::std::mem::transmute(waveOutSetVolume(
            hwo.into_param().abi(),
            ::std::mem::transmute(dwvolume),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveOutUnprepareHeader(
            hwo.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutWrite<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(
    hwo: Param0,
    pwh: *mut WAVEHDR,
    cbwh: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::std::mem::transmute(waveOutWrite(
            hwo.into_param().abi(),
            ::std::mem::transmute(pwh),
            ::std::mem::transmute(cbwh),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
