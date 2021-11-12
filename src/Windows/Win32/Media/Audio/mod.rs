#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub mod Apo;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub mod DirectSound;
#[cfg(feature = "Win32_Media_Audio_Endpoints")]
pub mod Endpoints;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for ACMDRIVERDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for ACMDRIVERDETAILSW {
    type Abi = Self;
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
pub type ACMDRIVERENUMCB = unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl ACMDRVFORMATSUGGEST {}
impl ::core::default::Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMDRVFORMATSUGGEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMDRVFORMATSUGGEST {}
unsafe impl ::windows::core::Abi for ACMDRVFORMATSUGGEST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMDRVSTREAMHEADER {
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
    pub padshNext: *mut ACMDRVSTREAMHEADER,
    pub fdwDriver: u32,
    pub dwDriver: usize,
    pub fdwPrepared: u32,
    pub dwPrepared: usize,
    pub pbPreparedSrc: *mut u8,
    pub cbPreparedSrcLength: u32,
    pub pbPreparedDst: *mut u8,
    pub cbPreparedDstLength: u32,
}
impl ACMDRVSTREAMHEADER {}
impl ::core::default::Default for ACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMHEADER {}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMDRVSTREAMINSTANCE {
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
impl ACMDRVSTREAMINSTANCE {}
impl ::core::default::Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMINSTANCE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMINSTANCE {}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMINSTANCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl ACMDRVSTREAMSIZE {}
impl ::core::default::Default for ACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMSIZE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMSIZE {}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMSIZE {
    type Abi = Self;
}
pub const ACMERR_BASE: u32 = 512u32;
pub const ACMERR_BUSY: u32 = 513u32;
pub const ACMERR_CANCELED: u32 = 515u32;
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
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
    pub pfnHook: ::core::option::Option<ACMFILTERCHOOSEHOOKPROCA>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERCHOOSEA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEW {
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
    pub pfnHook: ::core::option::Option<ACMFILTERCHOOSEHOOKPROCW>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERCHOOSEW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMFILTERDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMFILTERDETAILSW {}
unsafe impl ::windows::core::Abi for ACMFILTERDETAILSW {
    type Abi = Self;
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERTAGDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSW {}
unsafe impl ::windows::core::Abi for ACMFILTERTAGDETAILSW {
    type Abi = Self;
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEA {
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
    pub pfnHook: ::core::option::Option<ACMFORMATCHOOSEHOOKPROCA>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATCHOOSEA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEW {
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
    pub pfnHook: ::core::option::Option<ACMFORMATCHOOSEHOOKPROCW>,
}
#[cfg(feature = "Win32_Foundation")]
impl ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATCHOOSEW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATDETAILSA {
    type Abi = Self;
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATTAGDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSW {}
unsafe impl ::windows::core::Abi for ACMFORMATTAGDETAILSW {
    type Abi = Self;
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
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
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ACMSTREAMHEADER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
unsafe impl ::windows::core::Abi for ACMSTREAMHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
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
    pub dwReservedDriver: [u32; 10],
}
#[cfg(any(target_arch = "x86",))]
impl ACMSTREAMHEADER {}
#[cfg(any(target_arch = "x86",))]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(any(target_arch = "x86",))]
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "x86",))]
unsafe impl ::windows::core::Abi for ACMSTREAMHEADER {
    type Abi = Self;
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
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
impl ::core::convert::From<i32> for AMBISONICS_CHANNEL_ORDERING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_CHANNEL_ORDERING {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
impl ::core::convert::From<i32> for AMBISONICS_NORMALIZATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_NORMALIZATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
impl AMBISONICS_PARAMS {}
impl ::core::default::Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AMBISONICS_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AMBISONICS_PARAMS")
            .field("u32Size", &self.u32Size)
            .field("u32Version", &self.u32Version)
            .field("u32Type", &self.u32Type)
            .field("u32ChannelOrdering", &self.u32ChannelOrdering)
            .field("u32Normalization", &self.u32Normalization)
            .field("u32Order", &self.u32Order)
            .field("u32NumChannels", &self.u32NumChannels)
            .field("pu32ChannelMap", &self.pu32ChannelMap)
            .finish()
    }
}
impl ::core::cmp::PartialEq for AMBISONICS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.u32Size == other.u32Size && self.u32Version == other.u32Version && self.u32Type == other.u32Type && self.u32ChannelOrdering == other.u32ChannelOrdering && self.u32Normalization == other.u32Normalization && self.u32Order == other.u32Order && self.u32NumChannels == other.u32NumChannels && self.pu32ChannelMap == other.pu32ChannelMap
    }
}
impl ::core::cmp::Eq for AMBISONICS_PARAMS {}
unsafe impl ::windows::core::Abi for AMBISONICS_PARAMS {
    type Abi = Self;
}
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AMBISONICS_TYPE(pub i32);
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
impl ::core::convert::From<i32> for AMBISONICS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_TYPE {
    type Abi = Self;
}
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287486i32 as _);
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287469i32 as _);
pub const AUDCLNT_E_BUFFER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287464i32 as _);
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287477i32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287466i32 as _);
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287463i32 as _);
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287482i32 as _);
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287465i32 as _);
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287484i32 as _);
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287478i32 as _);
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287423i32 as _);
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287422i32 as _);
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287473i32 as _);
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287454i32 as _);
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287447i32 as _);
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287448i32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287471i32 as _);
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287468i32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287474i32 as _);
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287470i32 as _);
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287440i32 as _);
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287424i32 as _);
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287467i32 as _);
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287456i32 as _);
pub const AUDCLNT_E_INVALID_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287479i32 as _);
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287455i32 as _);
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287451i32 as _);
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287487i32 as _);
pub const AUDCLNT_E_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287483i32 as _);
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287452i32 as _);
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287453i32 as _);
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287481i32 as _);
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287449i32 as _);
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287450i32 as _);
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287472i32 as _);
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287476i32 as _);
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287480i32 as _);
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287485i32 as _);
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDCLNT_SHAREMODE(pub i32);
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
impl ::core::convert::From<i32> for AUDCLNT_SHAREMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDCLNT_SHAREMODE {
    type Abi = Self;
}
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDCLNT_STREAMOPTIONS(pub u32);
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0u32);
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1u32);
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2u32);
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4u32);
impl ::core::convert::From<u32> for AUDCLNT_STREAMOPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDCLNT_STREAMOPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(143196161i32 as _);
pub const AUDCLNT_S_POSITION_STALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196163i32 as _);
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196162i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS {}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_PARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
impl ::core::convert::From<i32> for AUDIOCLIENT_ACTIVATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::default::Default for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS").field("TargetProcessId", &self.TargetProcessId).field("ProcessLoopbackMode", &self.ProcessLoopbackMode).finish()
    }
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.TargetProcessId == other.TargetProcessId && self.ProcessLoopbackMode == other.ProcessLoopbackMode
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    type Abi = Self;
}
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_DUCKING_OPTIONS(pub u32);
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0u32);
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1u32);
impl ::core::convert::From<u32> for AUDIO_DUCKING_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_DUCKING_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_EFFECT {
    pub id: ::windows::core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_EFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_EFFECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_EFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_EFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.canSetState == other.canSetState && self.state == other.state
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_EFFECT {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_EFFECT_STATE(pub i32);
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
impl ::core::convert::From<i32> for AUDIO_EFFECT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_EFFECT_STATE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
impl ::core::convert::From<i32> for AUDIO_STREAM_CATEGORY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_STREAM_CATEGORY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: ::windows::core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA").field("guidEventContext", &self.guidEventContext).field("bMuted", &self.bMuted).field("fMasterVolume", &self.fMasterVolume).field("nChannels", &self.nChannels).field("afChannelVolumes", &self.afChannelVolumes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.guidEventContext == other.guidEventContext && self.bMuted == other.bMuted && self.fMasterVolume == other.fMasterVolume && self.nChannels == other.nChannels && self.afChannelVolumes == other.afChannelVolumes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_VOLUME_NOTIFICATION_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUXCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUXCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct AUXCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl AUXCAPS2W {}
impl ::core::default::Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUXCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for AUXCAPS2W {}
unsafe impl ::windows::core::Abi for AUXCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUXCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUXCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUXCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for AUXCAPSW {}
unsafe impl ::windows::core::Abi for AUXCAPSW {
    type Abi = Self;
}
pub const AUXCAPS_AUXIN: u32 = 2u32;
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IActivateAudioInterfaceCompletionHandler>>(deviceinterfacepath: Param0, riid: *const ::windows::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: Param3) -> ::windows::core::Result<IActivateAudioInterfaceAsyncOperation> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivateAudioInterfaceAsync(deviceinterfacepath: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, completionhandler: ::windows::core::RawPtr, activationoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IActivateAudioInterfaceAsyncOperation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        ActivateAudioInterfaceAsync(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(activationparams), completionhandler.into_param().abi(), &mut result__).from_abi::<IActivateAudioInterfaceAsyncOperation>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: ::windows::core::GUID,
}
impl AudioClient3ActivationParams {}
impl ::core::default::Default for AudioClient3ActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for AudioClient3ActivationParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AudioClient3ActivationParams").field("tracingContextId", &self.tracingContextId).finish()
    }
}
impl ::core::cmp::PartialEq for AudioClient3ActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId
    }
}
impl ::core::cmp::Eq for AudioClient3ActivationParams {}
unsafe impl ::windows::core::Abi for AudioClient3ActivationParams {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioClientProperties {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioClientProperties {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AudioClientProperties").field("cbSize", &self.cbSize).field("bIsOffload", &self.bIsOffload).field("eCategory", &self.eCategory).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioClientProperties {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.bIsOffload == other.bIsOffload && self.eCategory == other.eCategory && self.Options == other.Options
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AudioClientProperties {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: ::core::option::Option<IMMDevice>,
    pub pPnpInterface: ::core::option::Option<IMMDevice>,
    pub pPnpDevnode: ::core::option::Option<IMMDevice>,
}
#[cfg(feature = "Win32_Foundation")]
impl AudioExtensionParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioExtensionParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("AudioExtensionParams").field("AddPageParam", &self.AddPageParam).field("pEndpoint", &self.pEndpoint).field("pPnpInterface", &self.pPnpInterface).field("pPnpDevnode", &self.pPnpDevnode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioExtensionParams {
    fn eq(&self, other: &Self) -> bool {
        self.AddPageParam == other.AddPageParam && self.pEndpoint == other.pEndpoint && self.pPnpInterface == other.pPnpInterface && self.pPnpDevnode == other.pPnpDevnode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioExtensionParams {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AudioExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioObjectType(pub u32);
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0u32);
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1u32);
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2u32);
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4u32);
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8u32);
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16u32);
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32u32);
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64u32);
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128u32);
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256u32);
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512u32);
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024u32);
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048u32);
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096u32);
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192u32);
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384u32);
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768u32);
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536u32);
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072u32);
impl ::core::convert::From<u32> for AudioObjectType {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioObjectType {
    type Abi = Self;
}
impl ::core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioSessionDisconnectReason(pub i32);
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
impl ::core::convert::From<i32> for AudioSessionDisconnectReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioSessionDisconnectReason {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioSessionState(pub i32);
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
impl ::core::convert::From<i32> for AudioSessionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioSessionState {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AudioStateMonitorSoundLevel(pub i32);
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
impl ::core::convert::From<i32> for AudioStateMonitorSoundLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AudioStateMonitorSoundLevel {
    type Abi = Self;
}
#[inline]
pub unsafe fn CoRegisterMessageFilter<'a, Param0: ::windows::core::IntoParam<'a, IMessageFilter>>(lpmessagefilter: Param0) -> ::windows::core::Result<IMessageFilter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMessageFilter(lpmessagefilter: ::windows::core::RawPtr, lplpmessagefilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IMessageFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterMessageFilter(lpmessagefilter.into_param().abi(), &mut result__).from_abi::<IMessageFilter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: ConnectorType = ConnectorType(0i32);
    pub const Physical_Internal: ConnectorType = ConnectorType(1i32);
    pub const Physical_External: ConnectorType = ConnectorType(2i32);
    pub const Software_IO: ConnectorType = ConnectorType(3i32);
    pub const Software_Fixed: ConnectorType = ConnectorType(4i32);
    pub const Network: ConnectorType = ConnectorType(5i32);
}
impl ::core::convert::From<i32> for ConnectorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ConnectorType {
    type Abi = Self;
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitor(&mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategory(::core::mem::transmute(category), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitor(&mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategory(::core::mem::transmute(category), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: ::windows::core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::default::Default for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DIRECTX_AUDIO_ACTIVATION_PARAMS").field("cbDirectXAudioActivationParams", &self.cbDirectXAudioActivationParams).field("guidAudioSession", &self.guidAudioSession).field("dwAudioStreamFlags", &self.dwAudioStreamFlags).finish()
    }
}
impl ::core::cmp::PartialEq for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbDirectXAudioActivationParams == other.cbDirectXAudioActivationParams && self.guidAudioSession == other.guidAudioSession && self.dwAudioStreamFlags == other.dwAudioStreamFlags
    }
}
impl ::core::cmp::Eq for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
unsafe impl ::windows::core::Abi for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    type Abi = Self;
}
pub const DRVM_MAPPER: u32 = 8192u32;
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DataFlow(pub i32);
pub const In: DataFlow = DataFlow(0i32);
pub const Out: DataFlow = DataFlow(1i32);
impl ::core::convert::From<i32> for DataFlow {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DataFlow {
    type Abi = Self;
}
pub const DeviceTopology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ECHOWAVEFILTER {}
impl ::core::default::Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ECHOWAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for ECHOWAVEFILTER {}
unsafe impl ::windows::core::Abi for ECHOWAVEFILTER {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EDataFlow(pub i32);
pub const eRender: EDataFlow = EDataFlow(0i32);
pub const eCapture: EDataFlow = EDataFlow(1i32);
pub const eAll: EDataFlow = EDataFlow(2i32);
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
impl ::core::convert::From<i32> for EDataFlow {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EDataFlow {
    type Abi = Self;
}
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ERole(pub i32);
pub const eConsole: ERole = ERole(0i32);
pub const eMultimedia: ERole = ERole(1i32);
pub const eCommunications: ERole = ERole(2i32);
pub const ERole_enum_count: ERole = ERole(3i32);
impl ::core::convert::From<i32> for ERole {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ERole {
    type Abi = Self;
}
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct EndpointFormFactor(pub i32);
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
impl ::core::convert::From<i32> for EndpointFormFactor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for EndpointFormFactor {
    type Abi = Self;
}
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HACMDRIVER(pub isize);
impl ::core::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HACMDRIVER {}
unsafe impl ::windows::core::Abi for HACMDRIVER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HACMDRIVERID(pub isize);
impl ::core::default::Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HACMDRIVERID {}
unsafe impl ::windows::core::Abi for HACMDRIVERID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HACMOBJ(pub isize);
impl ::core::default::Default for HACMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HACMOBJ {}
unsafe impl ::windows::core::Abi for HACMOBJ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HACMSTREAM(pub isize);
impl ::core::default::Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HACMSTREAM {}
unsafe impl ::windows::core::Abi for HACMSTREAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIDI(pub isize);
impl ::core::default::Default for HMIDI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIDI {}
unsafe impl ::windows::core::Abi for HMIDI {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIDIIN(pub isize);
impl ::core::default::Default for HMIDIIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIDIIN {}
unsafe impl ::windows::core::Abi for HMIDIIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIDIOUT(pub isize);
impl ::core::default::Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIDIOUT {}
unsafe impl ::windows::core::Abi for HMIDIOUT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIDISTRM(pub isize);
impl ::core::default::Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIDISTRM {}
unsafe impl ::windows::core::Abi for HMIDISTRM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIXER(pub isize);
impl ::core::default::Default for HMIXER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIXER {}
unsafe impl ::windows::core::Abi for HMIXER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HMIXEROBJ(pub isize);
impl ::core::default::Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HMIXEROBJ {}
unsafe impl ::windows::core::Abi for HMIXEROBJ {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HWAVE(pub isize);
impl ::core::default::Default for HWAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HWAVE {}
unsafe impl ::windows::core::Abi for HWAVE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HWAVEIN(pub isize);
impl ::core::default::Default for HWAVEIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HWAVEIN {}
unsafe impl ::windows::core::Abi for HWAVEIN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HWAVEOUT(pub isize);
impl ::core::default::Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::core::Handle for HWAVEOUT {}
unsafe impl ::windows::core::Abi for HWAVEOUT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivateAudioInterfaceAsyncOperation(pub ::windows::core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    pub unsafe fn GetActivateResult(&self, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(activateresult), ::core::mem::transmute(activatedinterface)).ok()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceAsyncOperation {
    type Vtable = IActivateAudioInterfaceAsyncOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a22d78_cde4_431d_b8cc_843a71199b6d);
}
impl ::core::convert::From<IActivateAudioInterfaceAsyncOperation> for ::windows::core::IUnknown {
    fn from(value: IActivateAudioInterfaceAsyncOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceAsyncOperation> for ::windows::core::IUnknown {
    fn from(value: &IActivateAudioInterfaceAsyncOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivateAudioInterfaceCompletionHandler(pub ::windows::core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    pub unsafe fn ActivateCompleted<'a, Param0: ::windows::core::IntoParam<'a, IActivateAudioInterfaceAsyncOperation>>(&self, activateoperation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), activateoperation.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceCompletionHandler {
    type Vtable = IActivateAudioInterfaceCompletionHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d949ab_9862_444a_80f6_c261334da5eb);
}
impl ::core::convert::From<IActivateAudioInterfaceCompletionHandler> for ::windows::core::IUnknown {
    fn from(value: IActivateAudioInterfaceCompletionHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceCompletionHandler> for ::windows::core::IUnknown {
    fn from(value: &IActivateAudioInterfaceCompletionHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activateoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioAmbisonicsControl(pub ::windows::core::IUnknown);
impl IAudioAmbisonicsControl {
    pub unsafe fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pambisonicsparams), ::core::mem::transmute(cbambisonicsparams)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeadTracking<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benableheadtracking: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benableheadtracking.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHeadTracking(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioAmbisonicsControl {
    type Vtable = IAudioAmbisonicsControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28724c91_df35_4856_9f76_d6a26413f3df);
}
impl ::core::convert::From<IAudioAmbisonicsControl> for ::windows::core::IUnknown {
    fn from(value: IAudioAmbisonicsControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioAmbisonicsControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioAmbisonicsControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioAutoGainControl(pub ::windows::core::IUnknown);
impl IAudioAutoGainControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioAutoGainControl {
    type Vtable = IAudioAutoGainControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
}
impl ::core::convert::From<IAudioAutoGainControl> for ::windows::core::IUnknown {
    fn from(value: IAudioAutoGainControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioAutoGainControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioAutoGainControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioAutoGainControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioAutoGainControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioBass(pub ::windows::core::IUnknown);
impl IAudioBass {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioBass {
    type Vtable = IAudioBass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
}
impl ::core::convert::From<IAudioBass> for ::windows::core::IUnknown {
    fn from(value: IAudioBass) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioBass> for ::windows::core::IUnknown {
    fn from(value: &IAudioBass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioBass> for IPerChannelDbLevel {
    fn from(value: IAudioBass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioBass> for IPerChannelDbLevel {
    fn from(value: &IAudioBass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioCaptureClient(pub ::windows::core::IUnknown);
impl IAudioCaptureClient {
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdata), ::core::mem::transmute(pnumframestoread), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pu64deviceposition), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesread)).ok()
    }
    pub unsafe fn GetNextPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioCaptureClient {
    type Vtable = IAudioCaptureClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8adbd64_e71e_48a0_a4de_185c395cd317);
}
impl ::core::convert::From<IAudioCaptureClient> for ::windows::core::IUnknown {
    fn from(value: IAudioCaptureClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioCaptureClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioCaptureClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioCaptureClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioCaptureClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numframesread: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioChannelConfig(pub ::windows::core::IUnknown);
impl IAudioChannelConfig {
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconfig), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn GetChannelConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioChannelConfig {
    type Vtable = IAudioChannelConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb11c46f_ec28_493c_b88a_5db88062ce98);
}
impl ::core::convert::From<IAudioChannelConfig> for ::windows::core::IUnknown {
    fn from(value: IAudioChannelConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioChannelConfig> for ::windows::core::IUnknown {
    fn from(value: &IAudioChannelConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioChannelConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioChannelConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwconfig: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient(pub ::windows::core::IUnknown);
impl IAudioClient {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient {
    type Vtable = IAudioClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
}
impl ::core::convert::From<IAudioClient> for ::windows::core::IUnknown {
    fn from(value: IAudioClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnslatency: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient2(pub ::windows::core::IUnknown);
impl IAudioClient2 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient2 {
    type Vtable = IAudioClient2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x726778cd_f60a_4eda_82de_e47610cd78aa);
}
impl ::core::convert::From<IAudioClient2> for ::windows::core::IUnknown {
    fn from(value: IAudioClient2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient2> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioClient2> for IAudioClient {
    fn from(value: IAudioClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient2> for IAudioClient {
    fn from(value: &IAudioClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for &IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnslatency: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient3(pub ::windows::core::IUnknown);
impl IAudioClient3 {
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), ::core::mem::transmute(pdefaultperiodinframes), ::core::mem::transmute(pfundamentalperiodinframes), ::core::mem::transmute(pminperiodinframes), ::core::mem::transmute(pmaxperiodinframes)).ok()
    }
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformat), ::core::mem::transmute(pcurrentperiodinframes)).ok()
    }
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamflags), ::core::mem::transmute(periodinframes), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient3 {
    type Vtable = IAudioClient3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
}
impl ::core::convert::From<IAudioClient3> for ::windows::core::IUnknown {
    fn from(value: IAudioClient3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient3> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioClient3> for IAudioClient2 {
    fn from(value: IAudioClient3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient3> for IAudioClient2 {
    fn from(value: &IAudioClient3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient2> for IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient2> for &IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAudioClient3> for IAudioClient {
    fn from(value: IAudioClient3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient3> for IAudioClient {
    fn from(value: &IAudioClient3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for &IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnslatency: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClientDuckingControl(pub ::windows::core::IUnknown);
impl IAudioClientDuckingControl {
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClientDuckingControl {
    type Vtable = IAudioClientDuckingControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc789d381_a28c_4168_b28f_d3a837924dc3);
}
impl ::core::convert::From<IAudioClientDuckingControl> for ::windows::core::IUnknown {
    fn from(value: IAudioClientDuckingControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClientDuckingControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioClientDuckingControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClock(pub ::windows::core::IUnknown);
impl IAudioClock {
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u64> {
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pu64position), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioClock {
    type Vtable = IAudioClock_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd63314f_3fba_4a1b_812c_ef96358728e7);
}
impl ::core::convert::From<IAudioClock> for ::windows::core::IUnknown {
    fn from(value: IAudioClock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClock> for ::windows::core::IUnknown {
    fn from(value: &IAudioClock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pu64frequency: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClock2(pub ::windows::core::IUnknown);
impl IAudioClock2 {
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceposition), ::core::mem::transmute(qpcposition)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClock2 {
    type Vtable = IAudioClock2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f49ff73_6727_49ac_a008_d98cf5e70048);
}
impl ::core::convert::From<IAudioClock2> for ::windows::core::IUnknown {
    fn from(value: IAudioClock2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClock2> for ::windows::core::IUnknown {
    fn from(value: &IAudioClock2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClockAdjustment(pub ::windows::core::IUnknown);
impl IAudioClockAdjustment {
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flsamplerate)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioClockAdjustment {
    type Vtable = IAudioClockAdjustment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
}
impl ::core::convert::From<IAudioClockAdjustment> for ::windows::core::IUnknown {
    fn from(value: IAudioClockAdjustment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClockAdjustment> for ::windows::core::IUnknown {
    fn from(value: &IAudioClockAdjustment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClockAdjustment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClockAdjustment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flsamplerate: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEffectsChangedNotificationClient(pub ::windows::core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    pub unsafe fn OnAudioEffectsChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsChangedNotificationClient {
    type Vtable = IAudioEffectsChangedNotificationClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
}
impl ::core::convert::From<IAudioEffectsChangedNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IAudioEffectsChangedNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEffectsChangedNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioEffectsChangedNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEffectsManager(pub ::windows::core::IUnknown);
impl IAudioEffectsManager {
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects)).ok()
    }
    pub unsafe fn SetAudioEffectState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, effectid: Param0, state: AUDIO_EFFECT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsManager {
    type Vtable = IAudioEffectsManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4460b3ae_4b44_4527_8676_7548a8acd260);
}
impl ::core::convert::From<IAudioEffectsManager> for ::windows::core::IUnknown {
    fn from(value: IAudioEffectsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEffectsManager> for ::windows::core::IUnknown {
    fn from(value: &IAudioEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioFormatEnumerator(pub ::windows::core::IUnknown);
impl IAudioFormatEnumerator {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, index: u32) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioFormatEnumerator {
    type Vtable = IAudioFormatEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdaa858_895a_4a22_a5eb_67bda506096d);
}
impl ::core::convert::From<IAudioFormatEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAudioFormatEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioFormatEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAudioFormatEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioInputSelector(pub ::windows::core::IUnknown);
impl IAudioInputSelector {
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioInputSelector {
    type Vtable = IAudioInputSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f03dc02_5e6e_4653_8f72_a030c123d598);
}
impl ::core::convert::From<IAudioInputSelector> for ::windows::core::IUnknown {
    fn from(value: IAudioInputSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioInputSelector> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioLoudness(pub ::windows::core::IUnknown);
impl IAudioLoudness {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioLoudness {
    type Vtable = IAudioLoudness_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
}
impl ::core::convert::From<IAudioLoudness> for ::windows::core::IUnknown {
    fn from(value: IAudioLoudness) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioLoudness> for ::windows::core::IUnknown {
    fn from(value: &IAudioLoudness) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioLoudness {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioLoudness {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMidrange(pub ::windows::core::IUnknown);
impl IAudioMidrange {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioMidrange {
    type Vtable = IAudioMidrange_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
}
impl ::core::convert::From<IAudioMidrange> for ::windows::core::IUnknown {
    fn from(value: IAudioMidrange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMidrange> for ::windows::core::IUnknown {
    fn from(value: &IAudioMidrange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioMidrange> for IPerChannelDbLevel {
    fn from(value: IAudioMidrange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMidrange> for IPerChannelDbLevel {
    fn from(value: &IAudioMidrange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMute(pub ::windows::core::IUnknown);
impl IAudioMute {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmuted: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bmuted.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioMute {
    type Vtable = IAudioMute_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
}
impl ::core::convert::From<IAudioMute> for ::windows::core::IUnknown {
    fn from(value: IAudioMute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMute> for ::windows::core::IUnknown {
    fn from(value: &IAudioMute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioMute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioMute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioOutputSelector(pub ::windows::core::IUnknown);
impl IAudioOutputSelector {
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioOutputSelector {
    type Vtable = IAudioOutputSelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
}
impl ::core::convert::From<IAudioOutputSelector> for ::windows::core::IUnknown {
    fn from(value: IAudioOutputSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioOutputSelector> for ::windows::core::IUnknown {
    fn from(value: &IAudioOutputSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioOutputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioOutputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioPeakMeter(pub ::windows::core::IUnknown);
impl IAudioPeakMeter {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioPeakMeter {
    type Vtable = IAudioPeakMeter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
}
impl ::core::convert::From<IAudioPeakMeter> for ::windows::core::IUnknown {
    fn from(value: IAudioPeakMeter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioPeakMeter> for ::windows::core::IUnknown {
    fn from(value: &IAudioPeakMeter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioPeakMeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioPeakMeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioRenderClient(pub ::windows::core::IUnknown);
impl IAudioRenderClient {
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesrequested), &mut result__).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframeswritten), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioRenderClient {
    type Vtable = IAudioRenderClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf294acfc_3146_4483_a7bf_addca7c260e2);
}
impl ::core::convert::From<IAudioRenderClient> for ::windows::core::IUnknown {
    fn from(value: IAudioRenderClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioRenderClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioRenderClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioRenderClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioRenderClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionControl(pub ::windows::core::IUnknown);
impl IAudioSessionControl {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__: <AudioSessionState as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioSessionState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl {
    type Vtable = IAudioSessionControl_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
}
impl ::core::convert::From<IAudioSessionControl> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionControl2(pub ::windows::core::IUnknown);
impl IAudioSessionControl2 {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__: <AudioSessionState as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioSessionState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSessionIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSystemSoundsSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDuckingPreference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, optout: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), optout.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl2 {
    type Vtable = IAudioSessionControl2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
}
impl ::core::convert::From<IAudioSessionControl2> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionControl2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionControl2> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionControl2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioSessionControl2> for IAudioSessionControl {
    fn from(value: IAudioSessionControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionControl2> for IAudioSessionControl {
    fn from(value: &IAudioSessionControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionControl> for IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionControl> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionControl> for &IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionControl> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionEnumerator(pub ::windows::core::IUnknown);
impl IAudioSessionEnumerator {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn GetSession(&self, sessioncount: i32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessioncount), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEnumerator {
    type Vtable = IAudioSessionEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
}
impl ::core::convert::From<IAudioSessionEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessioncount: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessioncount: i32, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionEvents(pub ::windows::core::IUnknown);
impl IAudioSessionEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDisplayNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, newdisplayname: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newdisplayname.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnIconPathChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, newiconpath: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newiconpath.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSimpleVolumeChanged<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, newvolume: f32, newmute: Param1, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(newvolume), newmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(channelcount), ::core::mem::transmute(newchannelvolumearray), ::core::mem::transmute(changedchannel), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(newgroupingparam), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstate)).ok()
    }
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(disconnectreason)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEvents {
    type Vtable = IAudioSessionEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
}
impl ::core::convert::From<IAudioSessionEvents> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionEvents> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newdisplayname: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newiconpath: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newstate: AudioSessionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionManager(pub ::windows::core::IUnknown);
impl IAudioSessionManager {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__: <ISimpleAudioVolume as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<ISimpleAudioVolume>(result__)
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager {
    type Vtable = IAudioSessionManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
}
impl ::core::convert::From<IAudioSessionManager> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionManager> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionManager2(pub ::windows::core::IUnknown);
impl IAudioSessionManager2 {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__: <ISimpleAudioVolume as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<ISimpleAudioVolume>(result__)
    }
    pub unsafe fn GetSessionEnumerator(&self) -> ::windows::core::Result<IAudioSessionEnumerator> {
        let mut result__: <IAudioSessionEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioSessionEnumerator>(result__)
    }
    pub unsafe fn RegisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, sessionid: Param0, ducknotification: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ducknotification.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, ducknotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ducknotification.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager2 {
    type Vtable = IAudioSessionManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
}
impl ::core::convert::From<IAudioSessionManager2> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionManager2> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioSessionManager2> for IAudioSessionManager {
    fn from(value: IAudioSessionManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionManager2> for IAudioSessionManager {
    fn from(value: &IAudioSessionManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionManager> for IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionManager> for &IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionid: super::super::Foundation::PWSTR, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionNotification(pub ::windows::core::IUnknown);
impl IAudioSessionNotification {
    pub unsafe fn OnSessionCreated<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionControl>>(&self, newsession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newsession.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionNotification {
    type Vtable = IAudioSessionNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641dd20b_4d41_49cc_aba3_174b9477bb08);
}
impl ::core::convert::From<IAudioSessionNotification> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionNotification> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioStateMonitor(pub ::windows::core::IUnknown);
impl IAudioStateMonitor {
    pub unsafe fn RegisterCallback(&self, callback: ::core::option::Option<PAudioStateMonitorCallback>, context: *const ::core::ffi::c_void) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(callback), ::core::mem::transmute(context), &mut result__).from_abi::<i64>(result__)
    }
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(registration)))
    }
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
}
impl ::core::convert::From<IAudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: IAudioStateMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: &IAudioStateMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, registration: i64),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> AudioStateMonitorSoundLevel,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioStreamVolume(pub ::windows::core::IUnknown);
impl IAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel)).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
    pub unsafe fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioStreamVolume {
    type Vtable = IAudioStreamVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93014887_242d_4068_8a15_cf5e93b90fe3);
}
impl ::core::convert::From<IAudioStreamVolume> for ::windows::core::IUnknown {
    fn from(value: IAudioStreamVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioStreamVolume> for ::windows::core::IUnknown {
    fn from(value: &IAudioStreamVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioStreamVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioStreamVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(pub ::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyChanged<'a, Param1: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), key.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Vtable = IAudioSystemEffectsPropertyChangeNotificationClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20049d40_56d5_400e_a2ef_385599feed49);
}
impl ::core::convert::From<IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffectsPropertyStore(pub ::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    pub unsafe fn ResetUserPropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetVolatilePropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RegisterPropertyChangeNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterPropertyChangeNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyStore {
    type Vtable = IAudioSystemEffectsPropertyStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
}
impl ::core::convert::From<IAudioSystemEffectsPropertyStore> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioTreble(pub ::windows::core::IUnknown);
impl IAudioTreble {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioTreble {
    type Vtable = IAudioTreble_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a717812_694e_4907_b74b_bafa5cfdca7b);
}
impl ::core::convert::From<IAudioTreble> for ::windows::core::IUnknown {
    fn from(value: IAudioTreble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioTreble> for ::windows::core::IUnknown {
    fn from(value: &IAudioTreble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioTreble> for IPerChannelDbLevel {
    fn from(value: IAudioTreble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioTreble> for IPerChannelDbLevel {
    fn from(value: &IAudioTreble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioVolumeDuckNotification(pub ::windows::core::IUnknown);
impl IAudioVolumeDuckNotification {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVolumeDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: Param0, countcommunicationsessions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ::core::mem::transmute(countcommunicationsessions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVolumeUnduckNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), sessionid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeDuckNotification {
    type Vtable = IAudioVolumeDuckNotification_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
}
impl ::core::convert::From<IAudioVolumeDuckNotification> for ::windows::core::IUnknown {
    fn from(value: IAudioVolumeDuckNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioVolumeDuckNotification> for ::windows::core::IUnknown {
    fn from(value: &IAudioVolumeDuckNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionid: super::super::Foundation::PWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioVolumeLevel(pub ::windows::core::IUnknown);
impl IAudioVolumeLevel {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeLevel {
    type Vtable = IAudioVolumeLevel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
}
impl ::core::convert::From<IAudioVolumeLevel> for ::windows::core::IUnknown {
    fn from(value: IAudioVolumeLevel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioVolumeLevel> for ::windows::core::IUnknown {
    fn from(value: &IAudioVolumeLevel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IAudioVolumeLevel> for IPerChannelDbLevel {
    fn from(value: IAudioVolumeLevel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioVolumeLevel> for IPerChannelDbLevel {
    fn from(value: &IAudioVolumeLevel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IChannelAudioVolume(pub ::windows::core::IUnknown);
impl IChannelAudioVolume {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
}
unsafe impl ::windows::core::Interface for IChannelAudioVolume {
    type Vtable = IChannelAudioVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c158861_b533_4b30_b1cf_e853e51c59b8);
}
impl ::core::convert::From<IChannelAudioVolume> for ::windows::core::IUnknown {
    fn from(value: IChannelAudioVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IChannelAudioVolume> for ::windows::core::IUnknown {
    fn from(value: &IChannelAudioVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IChannelAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnector(pub ::windows::core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<ConnectorType> {
        let mut result__: <ConnectorType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ConnectorType>(result__)
    }
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<DataFlow> {
        let mut result__: <DataFlow as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataFlow>(result__)
    }
    pub unsafe fn ConnectTo<'a, Param0: ::windows::core::IntoParam<'a, IConnector>>(&self, pconnectto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pconnectto.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetConnectedTo(&self) -> ::windows::core::Result<IConnector> {
        let mut result__: <IConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IConnector>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IConnector {
    type Vtable = IConnector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2c4058_23f5_41de_877a_df3af236a09e);
}
impl ::core::convert::From<IConnector> for ::windows::core::IUnknown {
    fn from(value: IConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnector> for ::windows::core::IUnknown {
    fn from(value: &IConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ptype: *mut ConnectorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflow: *mut DataFlow) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pconnectto: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppconto: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrconnectorid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IControlChangeNotify(pub ::windows::core::IUnknown);
impl IControlChangeNotify {
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsenderprocessid), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IControlChangeNotify {
    type Vtable = IControlChangeNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
}
impl ::core::convert::From<IControlChangeNotify> for ::windows::core::IUnknown {
    fn from(value: IControlChangeNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IControlChangeNotify> for ::windows::core::IUnknown {
    fn from(value: &IControlChangeNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IControlChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IControlInterface(pub ::windows::core::IUnknown);
impl IControlInterface {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
}
unsafe impl ::windows::core::Interface for IControlInterface {
    type Vtable = IControlInterface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d37c3f_5140_444a_ae24_400789f3cbf3);
}
impl ::core::convert::From<IControlInterface> for ::windows::core::IUnknown {
    fn from(value: IControlInterface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IControlInterface> for ::windows::core::IUnknown {
    fn from(value: &IControlInterface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IControlInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceSpecificProperty(pub ::windows::core::IUnknown);
impl IDeviceSpecificProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn GetValue(&self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    pub unsafe fn SetValue(&self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(cbvalue), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plstepping)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDeviceSpecificProperty {
    type Vtable = IDeviceSpecificProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b22bcbf_2586_4af0_8583_205d391b807c);
}
impl ::core::convert::From<IDeviceSpecificProperty> for ::windows::core::IUnknown {
    fn from(value: IDeviceSpecificProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceSpecificProperty> for ::windows::core::IUnknown {
    fn from(value: &IDeviceSpecificProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvtype: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceTopology(pub ::windows::core::IUnknown);
impl IDeviceTopology {
    pub unsafe fn GetConnectorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnector(&self, nindex: u32) -> ::windows::core::Result<IConnector> {
        let mut result__: <IConnector as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IConnector>(result__)
    }
    pub unsafe fn GetSubunitCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSubunit(&self, nindex: u32) -> ::windows::core::Result<ISubunit> {
        let mut result__: <ISubunit as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<ISubunit>(result__)
    }
    pub unsafe fn GetPartById(&self, nid: u32) -> ::windows::core::Result<IPart> {
        let mut result__: <IPart as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(nid), &mut result__).from_abi::<IPart>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignalPath<'a, Param0: ::windows::core::IntoParam<'a, IPart>, Param1: ::windows::core::IntoParam<'a, IPart>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pipartfrom: Param0, pipartto: Param1, brejectmixedpaths: Param2) -> ::windows::core::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pipartfrom.into_param().abi(), pipartto.into_param().abi(), brejectmixedpaths.into_param().abi(), &mut result__).from_abi::<IPartsList>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDeviceTopology {
    type Vtable = IDeviceTopology_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a07407e_6497_4a18_9787_32f79bd0d98f);
}
impl ::core::convert::From<IDeviceTopology> for ::windows::core::IUnknown {
    fn from(value: IDeviceTopology) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceTopology> for ::windows::core::IUnknown {
    fn from(value: &IDeviceTopology) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceTopology {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceTopology {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nindex: u32, ppsubunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nid: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pipartfrom: ::windows::core::RawPtr, pipartto: ::windows::core::RawPtr, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDevice(pub ::windows::core::IUnknown);
impl IMMDevice {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate(&self, iid: *const ::windows::core::GUID, dwclsctx: u32, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMMDevice {
    type Vtable = IMMDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd666063f_1587_4e43_81f1_b948e807363f);
}
impl ::core::convert::From<IMMDevice> for ::windows::core::IUnknown {
    fn from(value: IMMDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDevice> for ::windows::core::IUnknown {
    fn from(value: &IMMDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, dwclsctx: u32, pactivationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stgmaccess: u32, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppstrid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwstate: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceActivator(pub ::windows::core::IUnknown);
impl IMMDeviceActivator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate<'a, Param1: ::windows::core::IntoParam<'a, IMMDevice>>(&self, iid: *const ::windows::core::GUID, pdevice: Param1, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), pdevice.into_param().abi(), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceActivator {
    type Vtable = IMMDeviceActivator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
}
impl ::core::convert::From<IMMDeviceActivator> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceActivator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceActivator> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceActivator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, pdevice: ::windows::core::RawPtr, pactivationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceCollection(pub ::windows::core::IUnknown);
impl IMMDeviceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Item(&self, ndevice: u32) -> ::windows::core::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ndevice), &mut result__).from_abi::<IMMDevice>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceCollection {
    type Vtable = IMMDeviceCollection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
}
impl ::core::convert::From<IMMDeviceCollection> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceCollection> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcdevices: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ndevice: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceEnumerator(pub ::windows::core::IUnknown);
impl IMMDeviceEnumerator {
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::core::Result<IMMDeviceCollection> {
        let mut result__: <IMMDeviceCollection as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(dwstatemask), &mut result__).from_abi::<IMMDeviceCollection>(result__)
    }
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> ::windows::core::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(role), &mut result__).from_abi::<IMMDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrid: Param0) -> ::windows::core::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwstrid.into_param().abi(), &mut result__).from_abi::<IMMDevice>(result__)
    }
    pub unsafe fn RegisterEndpointNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterEndpointNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceEnumerator {
    type Vtable = IMMDeviceEnumerator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
}
impl ::core::convert::From<IMMDeviceEnumerator> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dataflow: EDataFlow, role: ERole, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwstrid: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMEndpoint(pub ::windows::core::IUnknown);
impl IMMEndpoint {
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<EDataFlow> {
        let mut result__: <EDataFlow as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<EDataFlow>(result__)
    }
}
unsafe impl ::windows::core::Interface for IMMEndpoint {
    type Vtable = IMMEndpoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be09788_6894_4089_8586_9a2a6c265ac5);
}
impl ::core::convert::From<IMMEndpoint> for ::windows::core::IUnknown {
    fn from(value: IMMEndpoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMEndpoint> for ::windows::core::IUnknown {
    fn from(value: &IMMEndpoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMNotificationClient(pub ::windows::core::IUnknown);
impl IMMNotificationClient {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDeviceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0, dwnewstate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), ::core::mem::transmute(dwnewstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDeviceAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDeviceRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnDefaultDeviceChanged<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(flow), ::core::mem::transmute(role), pwstrdefaultdeviceid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OnPropertyValueChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, pwstrdeviceid: Param0, key: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), key.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IMMNotificationClient {
    type Vtable = IMMNotificationClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7991eec9_7e89_4d85_8390_6c703cec60c0);
}
impl ::core::convert::From<IMMNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IMMNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IMMNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR, dwnewstate: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMessageFilter(pub ::windows::core::IUnknown);
impl IMessageFilter {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleInComingCall<'a, Param1: ::windows::core::IntoParam<'a, super::HTASK>>(&self, dwcalltype: u32, htaskcaller: Param1, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcalltype), htaskcaller.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(lpinterfaceinfo)))
    }
    pub unsafe fn RetryRejectedCall<'a, Param0: ::windows::core::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwrejecttype: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwrejecttype)))
    }
    pub unsafe fn MessagePending<'a, Param0: ::windows::core::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwpendingtype: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwpendingtype)))
    }
}
unsafe impl ::windows::core::Interface for IMessageFilter {
    type Vtable = IMessageFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000016_0000_0000_c000_000000000046);
}
impl ::core::convert::From<IMessageFilter> for ::windows::core::IUnknown {
    fn from(value: IMessageFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMessageFilter> for ::windows::core::IUnknown {
    fn from(value: &IMessageFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMessageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMessageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::INTERFACEINFO>) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPart(pub ::windows::core::IUnknown);
impl IPart {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetLocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlobalId(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn GetPartType(&self) -> ::windows::core::Result<PartType> {
        let mut result__: <PartType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PartType>(result__)
    }
    pub unsafe fn GetSubType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn GetControlInterfaceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> ::windows::core::Result<IControlInterface> {
        let mut result__: <IControlInterface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IControlInterface>(result__)
    }
    pub unsafe fn EnumPartsIncoming(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPartsList>(result__)
    }
    pub unsafe fn EnumPartsOutgoing(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPartsList>(result__)
    }
    pub unsafe fn GetTopologyObject(&self) -> ::windows::core::Result<IDeviceTopology> {
        let mut result__: <IDeviceTopology as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDeviceTopology>(result__)
    }
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(refiid), ::core::mem::transmute(ppvobject)).ok()
    }
    pub unsafe fn RegisterControlChangeCallback<'a, Param1: ::windows::core::IntoParam<'a, IControlChangeNotify>>(&self, riid: *const ::windows::core::GUID, pnotify: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pnotify.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterControlChangeCallback<'a, Param0: ::windows::core::IntoParam<'a, IControlChangeNotify>>(&self, pnotify: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IPart {
    type Vtable = IPart_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
}
impl ::core::convert::From<IPart> for ::windows::core::IUnknown {
    fn from(value: IPart) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPart> for ::windows::core::IUnknown {
    fn from(value: &IPart) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppwstrglobalid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pparttype: *mut PartType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nindex: u32, ppinterfacedesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPartsList(pub ::windows::core::IUnknown);
impl IPartsList {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetPart(&self, nindex: u32) -> ::windows::core::Result<IPart> {
        let mut result__: <IPart as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IPart>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPartsList {
    type Vtable = IPartsList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
}
impl ::core::convert::From<IPartsList> for ::windows::core::IUnknown {
    fn from(value: IPartsList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPartsList> for ::windows::core::IUnknown {
    fn from(value: &IPartsList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPartsList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPartsList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nindex: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPerChannelDbLevel(pub ::windows::core::IUnknown);
impl IPerChannelDbLevel {
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPerChannelDbLevel {
    type Vtable = IPerChannelDbLevel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
}
impl ::core::convert::From<IPerChannelDbLevel> for ::windows::core::IUnknown {
    fn from(value: IPerChannelDbLevel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPerChannelDbLevel> for ::windows::core::IUnknown {
    fn from(value: &IPerChannelDbLevel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerChannelDbLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPerChannelDbLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimpleAudioVolume(pub ::windows::core::IUnknown);
impl ISimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    pub unsafe fn GetMasterVolume(&self) -> ::windows::core::Result<f32> {
        let mut result__: <f32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmute: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISimpleAudioVolume {
    type Vtable = ISimpleAudioVolume_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87ce5498_68d6_44e5_9215_6da47ef883d8);
}
impl ::core::convert::From<ISimpleAudioVolume> for ::windows::core::IUnknown {
    fn from(value: ISimpleAudioVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimpleAudioVolume> for ::windows::core::IUnknown {
    fn from(value: &ISimpleAudioVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimpleAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimpleAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pflevel: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioClient(pub ::windows::core::IUnknown);
impl ISpatialAudioClient {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__: <IAudioFormatEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioFormatEnumerator>(result__)
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::core::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient {
    type Vtable = ISpatialAudioClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
}
impl ::core::convert::From<ISpatialAudioClient> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioClient> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioClient2(pub ::windows::core::IUnknown);
impl ISpatialAudioClient2 {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__: <IAudioFormatEnumerator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioFormatEnumerator>(result__)
    }
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::core::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMaxFrameCountForCategory<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: Param1, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), offloadenabled.into_param().abi(), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient2 {
    type Vtable = ISpatialAudioClient2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaabe452_a66a_4bee_a93e_e320463f6a53);
}
impl ::core::convert::From<ISpatialAudioClient2> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioClient2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioClient2> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioClient2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioClient2> for ISpatialAudioClient {
    fn from(value: ISpatialAudioClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioClient2> for ISpatialAudioClient {
    fn from(value: &ISpatialAudioClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioClient> for ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioClient> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioClient> for &ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mask: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataClient(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataClient {
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), ::core::mem::transmute(framecount), ::core::mem::transmute(metadataitemsbuffer), ::core::mem::transmute(metadataitems)).ok()
    }
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::core::Result<ISpatialAudioMetadataWriter> {
        let mut result__: <ISpatialAudioMetadataWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(overflowmode), &mut result__).from_abi::<ISpatialAudioMetadataWriter>(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> ::windows::core::Result<ISpatialAudioMetadataCopier> {
        let mut result__: <ISpatialAudioMetadataCopier as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataCopier>(result__)
    }
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> ::windows::core::Result<ISpatialAudioMetadataReader> {
        let mut result__: <ISpatialAudioMetadataReader as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataReader>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataClient {
    type Vtable = ISpatialAudioMetadataClient_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
}
impl ::core::convert::From<ISpatialAudioMetadataClient> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataClient> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::windows::core::RawPtr, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadatacopier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataCopier(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataCopier {
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn CopyMetadataForFrames<'a, Param2: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: Param2) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(copyframecount), ::core::mem::transmute(copymode), dstmetadataitems.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataCopier {
    type Vtable = ISpatialAudioMetadataCopier_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
}
impl ::core::convert::From<ISpatialAudioMetadataCopier> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataCopier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataCopier> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataCopier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::windows::core::RawPtr, itemscopied: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataItems(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataItems {
    pub unsafe fn GetFrameCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: <u16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    pub unsafe fn GetMaxValueBufferLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<SpatialAudioMetadataItemsInfo> {
        let mut result__: <SpatialAudioMetadataItemsInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SpatialAudioMetadataItemsInfo>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItems {
    type Vtable = ISpatialAudioMetadataItems_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd7c78f_3098_4f22_b547_a2f25a381269);
}
impl ::core::convert::From<ISpatialAudioMetadataItems> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItems> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemcount: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxitemcount: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataItemsBuffer(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    pub unsafe fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn DetachBuffer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItemsBuffer {
    type Vtable = ISpatialAudioMetadataItemsBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
}
impl ::core::convert::From<ISpatialAudioMetadataItemsBuffer> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataItemsBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItemsBuffer> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataItemsBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataReader(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataReader {
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandcount), ::core::mem::transmute(frameoffset)).ok()
    }
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(maxvaluebufferlength), ::core::mem::transmute(valuebufferlength)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataReader {
    type Vtable = ISpatialAudioMetadataReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
}
impl ::core::convert::From<ISpatialAudioMetadataReader> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataReader> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataWriter(pub ::windows::core::IUnknown);
impl ISpatialAudioMetadataWriter {
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(frameoffset)).ok()
    }
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataWriter {
    type Vtable = ISpatialAudioMetadataWriter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b17ca01_2955_444d_a430_537dc589a844);
}
impl ::core::convert::From<ISpatialAudioMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameoffset: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObject(pub ::windows::core::IUnknown);
impl ISpatialAudioObject {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    pub unsafe fn SetVolume(&self, volume: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(volume)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObject {
    type Vtable = ISpatialAudioObject_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
}
impl ::core::convert::From<ISpatialAudioObject> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObject> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObject> for ISpatialAudioObjectBase {
    fn from(value: ISpatialAudioObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObject> for ISpatialAudioObjectBase {
    fn from(value: &ISpatialAudioObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, volume: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectBase(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectBase {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectBase {
    type Vtable = ISpatialAudioObjectBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
}
impl ::core::convert::From<ISpatialAudioObjectBase> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectBase> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForHrtf(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectForHrtf {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    pub unsafe fn SetGain(&self, gain: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(gain)).ok()
    }
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(orientation)).ok()
    }
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(environment)).ok()
    }
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(distancedecay)).ok()
    }
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(directivity)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForHrtf {
    type Vtable = ISpatialAudioObjectForHrtf_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7436ade_1978_4e14_aba0_555bd8eb83b4);
}
impl ::core::convert::From<ISpatialAudioObjectForHrtf> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForHrtf) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForHrtf> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForHrtf) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectForHrtf> for ISpatialAudioObjectBase {
    fn from(value: ISpatialAudioObjectForHrtf) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForHrtf> for ISpatialAudioObjectBase {
    fn from(value: &ISpatialAudioObjectForHrtf) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForHrtf_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gain: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, orientation: *const *const f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForMetadataCommands(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataCommands {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataCommands {
    type Vtable = ISpatialAudioObjectForMetadataCommands_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataCommands> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataCommands) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataCommands> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataCommands) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataCommands> for ISpatialAudioObjectBase {
    fn from(value: ISpatialAudioObjectForMetadataCommands) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataCommands> for ISpatialAudioObjectBase {
    fn from(value: &ISpatialAudioObjectForMetadataCommands) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForMetadataItems(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataItems {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> ::windows::core::Result<ISpatialAudioMetadataItems> {
        let mut result__: <ISpatialAudioMetadataItems as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataItems>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataItems {
    type Vtable = ISpatialAudioObjectForMetadataItems_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataItems> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataItems> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataItems> for ISpatialAudioObjectBase {
    fn from(value: ISpatialAudioObjectForMetadataItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataItems> for ISpatialAudioObjectBase {
    fn from(value: &ISpatialAudioObjectForMetadataItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStream(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStream {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObject> {
        let mut result__: <ISpatialAudioObject as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObject>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStream {
    type Vtable = ISpatialAudioObjectRenderStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab5f473_b423_477b_85f5_b5a332a04153);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStream> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStream> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStream> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: ISpatialAudioObjectRenderStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStream> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: &ISpatialAudioObjectRenderStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamBase(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamBase {
    type Vtable = ISpatialAudioObjectRenderStreamBase_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamBase> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamBase> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamForHrtf(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForHrtf {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForHrtf> {
        let mut result__: <ISpatialAudioObjectForHrtf as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForHrtf>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForHrtf {
    type Vtable = ISpatialAudioObjectRenderStreamForHrtf_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08deef9_5363_406e_9fdc_080ee247bbe0);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForHrtf> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForHrtf> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForHrtf> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForHrtf> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: &ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamForMetadata(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForMetadata {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataCommands> {
        let mut result__: <ISpatialAudioObjectForMetadataCommands as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForMetadataCommands>(result__)
    }
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataItems> {
        let mut result__: <ISpatialAudioObjectForMetadataItems as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForMetadataItems>(result__)
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForMetadata {
    type Vtable = ISpatialAudioObjectRenderStreamForMetadata_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForMetadata> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForMetadata> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForMetadata> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForMetadata> for ISpatialAudioObjectRenderStreamBase {
    fn from(value: &ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamNotify(pub ::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    pub unsafe fn OnAvailableDynamicObjectCountChange<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase>>(&self, sender: Param0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sender.into_param().abi(), ::core::mem::transmute(hnscompliancedeadlinetime), ::core::mem::transmute(availabledynamicobjectcountchange)).ok()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamNotify {
    type Vtable = ISpatialAudioObjectRenderStreamNotify_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamNotify> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamNotify> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISubunit(pub ::windows::core::IUnknown);
impl ISubunit {}
unsafe impl ::windows::core::Interface for ISubunit {
    type Vtable = ISubunit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
}
impl ::core::convert::From<ISubunit> for ::windows::core::IUnknown {
    fn from(value: ISubunit) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISubunit> for ::windows::core::IUnknown {
    fn from(value: &ISubunit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISubunit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISubunit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
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
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
pub const MIDIERR_LASTERROR: u32 = 71u32;
pub const MIDIERR_NODEVICE: u32 = 68u32;
pub const MIDIERR_NOMAP: u32 = 66u32;
pub const MIDIERR_NOTREADY: u32 = 67u32;
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl MIDIEVENT {}
impl ::core::default::Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIEVENT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIEVENT {}
unsafe impl ::windows::core::Abi for MIDIEVENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIDIHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIHDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIHDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIINCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIINCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl MIDIINCAPS2W {}
impl ::core::default::Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIINCAPS2W {}
unsafe impl ::windows::core::Abi for MIDIINCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIINCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIINCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl MIDIINCAPSW {}
impl ::core::default::Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIINCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIINCAPSW {}
unsafe impl ::windows::core::Abi for MIDIINCAPSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIOUTCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIOUTCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl MIDIOUTCAPS2W {}
impl ::core::default::Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPS2W {}
unsafe impl ::windows::core::Abi for MIDIOUTCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIOUTCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIOUTCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIOUTCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPSW {}
unsafe impl ::windows::core::Abi for MIDIOUTCAPSW {
    type Abi = Self;
}
pub const MIDIPATCHSIZE: u32 = 128u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl MIDIPROPTEMPO {}
impl ::core::default::Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIPROPTEMPO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIPROPTEMPO {}
unsafe impl ::windows::core::Abi for MIDIPROPTEMPO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl MIDIPROPTIMEDIV {}
impl ::core::default::Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDIPROPTIMEDIV {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDIPROPTIMEDIV {}
unsafe impl ::windows::core::Abi for MIDIPROPTIMEDIV {
    type Abi = Self;
}
pub const MIDIPROP_GET: i32 = 1073741824i32;
pub const MIDIPROP_SET: i32 = -2147483648i32;
pub const MIDIPROP_TEMPO: i32 = 2i32;
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl MIDISTRMBUFFVER {}
impl ::core::default::Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIDISTRMBUFFVER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIDISTRMBUFFVER {}
unsafe impl ::windows::core::Abi for MIDISTRMBUFFVER {
    type Abi = Self;
}
pub const MIDISTRM_ERROR: i32 = -2i32;
pub const MIDI_CACHE_ALL: u32 = 1u32;
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
pub const MIDI_CACHE_QUERY: u32 = 3u32;
pub const MIDI_UNCACHE: u32 = 4u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(16u32);
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
impl ::core::convert::From<u32> for MIDI_WAVE_OPEN_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MIDI_WAVE_OPEN_TYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCAPS2A {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl MIXERCAPS2W {}
impl ::core::default::Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCAPS2W {}
unsafe impl ::windows::core::Abi for MIXERCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCAPSW {}
unsafe impl ::windows::core::Abi for MIXERCAPSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS {
    pub cbStruct: u32,
    pub dwControlID: u32,
    pub cChannels: u32,
    pub Anonymous: MIXERCONTROLDETAILS_0,
    pub cbDetails: u32,
    pub paDetails: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::default::Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_BOOLEAN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_BOOLEAN {}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_BOOLEAN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_LISTTEXTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTW {}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_LISTTEXTW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl MIXERCONTROLDETAILS_SIGNED {}
impl ::core::default::Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_SIGNED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_SIGNED {}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_SIGNED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::default::Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_UNSIGNED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_UNSIGNED {}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_UNSIGNED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW {}
unsafe impl ::windows::core::Abi for MIXERCONTROLW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLW_0 {}
impl ::core::default::Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0 {}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl MIXERCONTROLW_0_0 {}
impl ::core::default::Default for MIXERCONTROLW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_0 {}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl MIXERCONTROLW_0_1 {}
impl ::core::default::Default for MIXERCONTROLW_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_1 {}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl MIXERCONTROLW_1 {}
impl ::core::default::Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_1 {}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_1 {
    type Abi = Self;
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
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINEA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINEA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINEA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MIXERLINECONTROLSA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINECONTROLSA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW {}
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl MIXERLINECONTROLSW_0 {}
impl ::core::default::Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW_0 {}
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERLINEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERLINEW {}
unsafe impl ::windows::core::Abi for MIXERLINEW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MIXERLINEW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for MIXERLINEW_0 {}
unsafe impl ::windows::core::Abi for MIXERLINEW_0 {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(1u32);
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(5u32);
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(3u32);
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4u32);
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(6u32);
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(0u32);
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(8u32);
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(7u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4106u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4105u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4101u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4097u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4098u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4099u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4103u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4100u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4102u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4096u32);
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4104u32);
impl ::core::convert::From<u32> for MIXERLINE_COMPONENTTYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MIXERLINE_COMPONENTTYPE {
    type Abi = Self;
}
impl ::core::ops::BitOr for MIXERLINE_COMPONENTTYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MIXERLINE_COMPONENTTYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MIXERLINE_COMPONENTTYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MIXERLINE_COMPONENTTYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MIXERLINE_COMPONENTTYPE {
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
pub const MMDeviceEnumerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
pub const MOD_FMSYNTH: u32 = 4u32;
pub const MOD_MAPPER: u32 = 5u32;
pub const MOD_MIDIPORT: u32 = 1u32;
pub const MOD_SQSYNTH: u32 = 3u32;
pub const MOD_SWSYNTH: u32 = 7u32;
pub const MOD_SYNTH: u32 = 2u32;
pub const MOD_WAVETABLE: u32 = 6u32;
pub type PAudioStateMonitorCallback = unsafe extern "system" fn(audiostatemonitor: ::windows::core::RawPtr, context: *const ::core::ffi::c_void);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl PCMWAVEFORMAT {}
impl ::core::default::Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PCMWAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for PCMWAVEFORMAT {}
unsafe impl ::windows::core::Abi for PCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3u32 };
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
impl ::core::convert::From<i32> for PROCESS_LOOPBACK_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_LOOPBACK_MODE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PartType(pub i32);
pub const Connector: PartType = PartType(0i32);
pub const Subunit: PartType = PartType(1i32);
impl ::core::convert::From<i32> for PartType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PartType {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundA(pszsound: super::super::Foundation::PSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PlaySoundA(pszsound.into_param().abi(), hmod.into_param().abi(), ::core::mem::transmute(fdwsound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundW(pszsound: super::super::Foundation::PWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PlaySoundW(pszsound.into_param().abi(), hmod.into_param().abi(), ::core::mem::transmute(fdwsound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
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
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(0u32);
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(1u32);
impl ::core::convert::From<u32> for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Abi = Self;
}
impl ::core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SPTLAUDCLNT_E_DESTROYED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287232i32 as _);
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287227i32 as _);
pub const SPTLAUDCLNT_E_INTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287219i32 as _);
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287224i32 as _);
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287226i32 as _);
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287229i32 as _);
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287220i32 as _);
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287231i32 as _);
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287228i32 as _);
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287230i32 as _);
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287221i32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287225i32 as _);
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287222i32 as _);
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286956i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286969i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286968i32 as _);
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286940i32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286942i32 as _);
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286976i32 as _);
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286955i32 as _);
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286941i32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286967i32 as _);
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286952i32 as _);
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286974i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286957i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286939i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286959i32 as _);
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286951i32 as _);
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286971i32 as _);
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286973i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286954i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286944i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286960i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286958i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286943i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286970i32 as _);
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286953i32 as _);
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286975i32 as _);
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286972i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: ::windows::core::GUID,
    pub appId: ::windows::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl SpatialAudioClientActivationParams {}
impl ::core::default::Default for SpatialAudioClientActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SpatialAudioClientActivationParams {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SpatialAudioClientActivationParams")
            .field("tracingContextId", &self.tracingContextId)
            .field("appId", &self.appId)
            .field("majorVersion", &self.majorVersion)
            .field("minorVersion1", &self.minorVersion1)
            .field("minorVersion2", &self.minorVersion2)
            .field("minorVersion3", &self.minorVersion3)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SpatialAudioClientActivationParams {
    fn eq(&self, other: &Self) -> bool {
        self.tracingContextId == other.tracingContextId && self.appId == other.appId && self.majorVersion == other.majorVersion && self.minorVersion1 == other.minorVersion1 && self.minorVersion2 == other.minorVersion2 && self.minorVersion3 == other.minorVersion3
    }
}
impl ::core::cmp::Eq for SpatialAudioClientActivationParams {}
unsafe impl ::windows::core::Abi for SpatialAudioClientActivationParams {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
}
#[cfg(feature = "Win32_Foundation")]
impl SpatialAudioHrtfActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioHrtfActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioHrtfActivationParams2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub DistanceDecay: *mut SpatialAudioHrtfDistanceDecay,
    pub Directivity: *mut SpatialAudioHrtfDirectivityUnion,
    pub Environment: *mut SpatialAudioHrtfEnvironmentType,
    pub Orientation: *mut f32,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl SpatialAudioHrtfActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioHrtfActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl SpatialAudioHrtfDirectivity {}
impl ::core::default::Default for SpatialAudioHrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivity {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivity {}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivity {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl SpatialAudioHrtfDirectivityCardioid {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCardioid {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCardioid {}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityCardioid {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl SpatialAudioHrtfDirectivityCone {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCone {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCone {}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityCone {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
impl ::core::convert::From<i32> for SpatialAudioHrtfDirectivityType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl SpatialAudioHrtfDirectivityUnion {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityUnion {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityUnion {}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityUnion {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl SpatialAudioHrtfDistanceDecay {}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDistanceDecay {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDistanceDecay {}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDistanceDecay {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
impl ::core::convert::From<i32> for SpatialAudioHrtfDistanceDecayType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDistanceDecayType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
impl ::core::convert::From<i32> for SpatialAudioHrtfEnvironmentType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfEnvironmentType {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
impl ::core::convert::From<i32> for SpatialAudioMetadataCopyMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataCopyMode {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl SpatialAudioMetadataItemsInfo {}
impl ::core::default::Default for SpatialAudioMetadataItemsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SpatialAudioMetadataItemsInfo {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for SpatialAudioMetadataItemsInfo {}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataItemsInfo {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
impl ::core::convert::From<i32> for SpatialAudioMetadataWriterOverflowMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataWriterOverflowMode {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
}
#[cfg(feature = "Win32_Foundation")]
impl SpatialAudioObjectRenderStreamActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamActivationParams2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl SpatialAudioObjectRenderStreamActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::clone::Clone for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl VOLUMEWAVEFILTER {}
impl ::core::default::Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUMEWAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for VOLUMEWAVEFILTER {}
unsafe impl ::windows::core::Abi for VOLUMEWAVEFILTER {
    type Abi = Self;
}
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
pub const WAVECAPS_PITCH: u32 = 1u32;
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
pub const WAVECAPS_SYNC: u32 = 16u32;
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl WAVEFILTER {}
impl ::core::default::Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEFILTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEFILTER {}
unsafe impl ::windows::core::Abi for WAVEFILTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl WAVEFORMAT {}
impl ::core::default::Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEFORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEFORMAT {}
unsafe impl ::windows::core::Abi for WAVEFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEFORMATEX {}
unsafe impl ::windows::core::Abi for WAVEFORMATEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::core::GUID,
}
impl WAVEFORMATEXTENSIBLE {}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE {}
unsafe impl ::windows::core::Abi for WAVEFORMATEXTENSIBLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl WAVEFORMATEXTENSIBLE_0 {}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE_0 {}
unsafe impl ::windows::core::Abi for WAVEFORMATEXTENSIBLE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEHDR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEHDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEINCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEINCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct WAVEINCAPS2W {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl WAVEINCAPS2W {}
impl ::core::default::Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEINCAPS2W {}
unsafe impl ::windows::core::Abi for WAVEINCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEINCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEINCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEINCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEINCAPSW {}
unsafe impl ::windows::core::Abi for WAVEINCAPSW {
    type Abi = Self;
}
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEOUTCAPS2A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEOUTCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
    pub ManufacturerGuid: ::windows::core::GUID,
    pub ProductGuid: ::windows::core::GUID,
    pub NameGuid: ::windows::core::GUID,
}
impl WAVEOUTCAPS2W {}
impl ::core::default::Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPS2W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPS2W {}
unsafe impl ::windows::core::Abi for WAVEOUTCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEOUTCAPSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEOUTCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WAVEOUTCAPSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPSW {}
unsafe impl ::windows::core::Abi for WAVEOUTCAPSW {
    type Abi = Self;
}
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
pub const WAVERR_BADFORMAT: u32 = 32u32;
pub const WAVERR_LASTERROR: u32 = 35u32;
pub const WAVERR_STILLPLAYING: u32 = 33u32;
pub const WAVERR_SYNC: u32 = 35u32;
pub const WAVERR_UNPREPARED: u32 = 34u32;
pub const WAVE_FORMAT_1M08: u32 = 1u32;
pub const WAVE_FORMAT_1M16: u32 = 4u32;
pub const WAVE_FORMAT_1S08: u32 = 2u32;
pub const WAVE_FORMAT_1S16: u32 = 8u32;
pub const WAVE_FORMAT_2M08: u32 = 16u32;
pub const WAVE_FORMAT_2M16: u32 = 64u32;
pub const WAVE_FORMAT_2S08: u32 = 32u32;
pub const WAVE_FORMAT_2S16: u32 = 128u32;
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
pub const WAVE_FORMAT_PCM: u32 = 1u32;
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
pub const WAVE_MAPPER: u32 = 4294967295u32;
pub const WHDR_BEGINLOOP: u32 = 4u32;
pub const WHDR_DONE: u32 = 1u32;
pub const WHDR_ENDLOOP: u32 = 8u32;
pub const WHDR_INQUEUE: u32 = 16u32;
pub const WHDR_PREPARED: u32 = 2u32;
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
impl ::core::convert::From<i32> for _AUDCLNT_BUFFERFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for _AUDCLNT_BUFFERFLAGS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(pub i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(0i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(1i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(2i32);
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(3i32);
impl ::core::convert::From<i32> for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(phadid: *mut isize, hinstmodule: Param1, lparam: Param2, dwpriority: u32, fdwadd: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverAddA(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverAddA(::core::mem::transmute(phadid), hinstmodule.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(dwpriority), ::core::mem::transmute(fdwadd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(phadid: *mut isize, hinstmodule: Param1, lparam: Param2, dwpriority: u32, fdwadd: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverAddW(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverAddW(::core::mem::transmute(phadid), hinstmodule.into_param().abi(), lparam.into_param().abi(), ::core::mem::transmute(dwpriority), ::core::mem::transmute(fdwadd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverClose<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, fdwclose: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverClose(had.into_param().abi(), ::core::mem::transmute(fdwclose)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn acmDriverDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverDetailsA(hadid.into_param().abi(), ::core::mem::transmute(padd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverDetailsW(hadid.into_param().abi(), ::core::mem::transmute(padd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ::core::option::Option<ACMDRIVERENUMCB>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverEnum(fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverEnum(::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverID<'a, Param0: ::windows::core::IntoParam<'a, HACMOBJ>>(hao: Param0, phadid: *mut isize, fdwdriverid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverID(hao.into_param().abi(), ::core::mem::transmute(phadid), ::core::mem::transmute(fdwdriverid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverMessage<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(had: Param0, umsg: u32, lparam1: Param2, lparam2: Param3) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::core::mem::transmute(acmDriverMessage(had.into_param().abi(), ::core::mem::transmute(umsg), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverOpen<'a, Param1: ::windows::core::IntoParam<'a, HACMDRIVERID>>(phad: *mut isize, hadid: Param1, fdwopen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverOpen(::core::mem::transmute(phad), hadid.into_param().abi(), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverPriority<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, dwpriority: u32, fdwpriority: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverPriority(hadid.into_param().abi(), ::core::mem::transmute(dwpriority), ::core::mem::transmute(fdwpriority)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmDriverRemove<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, fdwremove: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverRemove(hadid.into_param().abi(), ::core::mem::transmute(fdwremove)))
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
            fn acmFilterChooseA(pafltrc: *mut ::core::mem::ManuallyDrop<ACMFILTERCHOOSEA>) -> u32;
        }
        ::core::mem::transmute(acmFilterChooseA(::core::mem::transmute(pafltrc)))
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
            fn acmFilterChooseW(pafltrc: *mut ::core::mem::ManuallyDrop<ACMFILTERCHOOSEW>) -> u32;
        }
        ::core::mem::transmute(acmFilterChooseW(::core::mem::transmute(pafltrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterDetailsA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFilterDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterDetailsW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSA, fncallback: ::core::option::Option<ACMFILTERENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterEnumA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSW, fncallback: ::core::option::Option<ACMFILTERENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterEnumW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagDetailsA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFilterTagDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagDetailsW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ::core::option::Option<ACMFILTERTAGENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagEnumA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ::core::option::Option<ACMFILTERTAGENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagEnumW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
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
            fn acmFormatChooseA(pafmtc: *mut ::core::mem::ManuallyDrop<ACMFORMATCHOOSEA>) -> u32;
        }
        ::core::mem::transmute(acmFormatChooseA(::core::mem::transmute(pafmtc)))
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
            fn acmFormatChooseW(pafmtc: *mut ::core::mem::ManuallyDrop<ACMFORMATCHOOSEW>) -> u32;
        }
        ::core::mem::transmute(acmFormatChooseW(::core::mem::transmute(pafmtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatDetailsA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatDetailsW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFORMATDETAILSA, fncallback: ::core::option::Option<ACMFORMATENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatEnumA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut tACMFORMATDETAILSW, fncallback: ::core::option::Option<ACMFORMATENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatEnumW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatSuggest<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatSuggest(had.into_param().abi(), ::core::mem::transmute(pwfxsrc), ::core::mem::transmute(pwfxdst), ::core::mem::transmute(cbwfxdst), ::core::mem::transmute(fdwsuggest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagDetailsA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmFormatTagDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagDetailsW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ::core::option::Option<ACMFORMATTAGENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagEnumA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ::core::option::Option<ACMFORMATTAGENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ::windows::core::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagEnumW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
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
        ::core::mem::transmute(acmGetVersion())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmMetrics<'a, Param0: ::windows::core::IntoParam<'a, HACMOBJ>>(hao: Param0, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(acmMetrics(hao.into_param().abi(), ::core::mem::transmute(umetric), ::core::mem::transmute(pmetric)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamClose<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, fdwclose: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamClose(has.into_param().abi(), ::core::mem::transmute(fdwclose)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamConvert<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamConvert(has.into_param().abi(), ::core::mem::transmute(pash), ::core::mem::transmute(fdwconvert)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmStreamMessage<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::LPARAM>>(has: Param0, umsg: u32, lparam1: Param2, lparam2: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32;
        }
        ::core::mem::transmute(acmStreamMessage(has.into_param().abi(), ::core::mem::transmute(umsg), lparam1.into_param().abi(), lparam2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamOpen<'a, Param1: ::windows::core::IntoParam<'a, HACMDRIVER>>(phas: *mut isize, had: Param1, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamOpen(phas: *mut isize, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamOpen(::core::mem::transmute(phas), had.into_param().abi(), ::core::mem::transmute(pwfxsrc), ::core::mem::transmute(pwfxdst), ::core::mem::transmute(pwfltr), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamPrepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamPrepareHeader(has.into_param().abi(), ::core::mem::transmute(pash), ::core::mem::transmute(fdwprepare)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamReset<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, fdwreset: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamReset(has.into_param().abi(), ::core::mem::transmute(fdwreset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamSize<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamSize(has.into_param().abi(), ::core::mem::transmute(cbinput), ::core::mem::transmute(pdwoutputbytes), ::core::mem::transmute(fdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn acmStreamUnprepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32;
        }
        ::core::mem::transmute(acmStreamUnprepareHeader(has.into_param().abi(), ::core::mem::transmute(pash), ::core::mem::transmute(fdwunprepare)))
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
        ::core::mem::transmute(auxGetDevCapsA(::core::mem::transmute(udeviceid), ::core::mem::transmute(pac), ::core::mem::transmute(cbac)))
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
        ::core::mem::transmute(auxGetDevCapsW(::core::mem::transmute(udeviceid), ::core::mem::transmute(pac), ::core::mem::transmute(cbac)))
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
        ::core::mem::transmute(auxGetNumDevs())
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
        ::core::mem::transmute(auxGetVolume(::core::mem::transmute(udeviceid), ::core::mem::transmute(pdwvolume)))
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
        ::core::mem::transmute(auxOutMessage(::core::mem::transmute(udeviceid), ::core::mem::transmute(umsg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
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
        ::core::mem::transmute(auxSetVolume(::core::mem::transmute(udeviceid), ::core::mem::transmute(dwvolume)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiConnect<'a, Param0: ::windows::core::IntoParam<'a, HMIDI>, Param1: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmi: Param0, hmo: Param1, preserved: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(midiConnect(hmi.into_param().abi(), hmo.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiDisconnect<'a, Param0: ::windows::core::IntoParam<'a, HMIDI>, Param1: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmi: Param0, hmo: Param1, preserved: *const ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(midiDisconnect(hmi.into_param().abi(), hmo.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInAddBuffer<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiInAddBuffer(hmi.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInClose<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInClose(hmi: HMIDIIN) -> u32;
        }
        ::core::mem::transmute(midiInClose(hmi.into_param().abi()))
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
        ::core::mem::transmute(midiInGetDevCapsA(::core::mem::transmute(udeviceid), ::core::mem::transmute(pmic), ::core::mem::transmute(cbmic)))
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
        ::core::mem::transmute(midiInGetDevCapsW(::core::mem::transmute(udeviceid), ::core::mem::transmute(pmic), ::core::mem::transmute(cbmic)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiInGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiInGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInGetID<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0, pudeviceid: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
        }
        ::core::mem::transmute(midiInGetID(hmi.into_param().abi(), ::core::mem::transmute(pudeviceid)))
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
        ::core::mem::transmute(midiInGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInMessage<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::core::mem::transmute(midiInMessage(hmi.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
        }
        ::core::mem::transmute(midiInOpen(::core::mem::transmute(phmi), ::core::mem::transmute(udeviceid), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInPrepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiInPrepareHeader(hmi.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInReset<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInReset(hmi: HMIDIIN) -> u32;
        }
        ::core::mem::transmute(midiInReset(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInStart<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInStart(hmi: HMIDIIN) -> u32;
        }
        ::core::mem::transmute(midiInStart(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiInStop<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInStop(hmi: HMIDIIN) -> u32;
        }
        ::core::mem::transmute(midiInStop(hmi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInUnprepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiInUnprepareHeader(hmi.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutCacheDrumPatches<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, upatch: u32, pwkya: *const u16, fucache: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
        }
        ::core::mem::transmute(midiOutCacheDrumPatches(hmo.into_param().abi(), ::core::mem::transmute(upatch), ::core::mem::transmute(pwkya), ::core::mem::transmute(fucache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutCachePatches<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, ubank: u32, pwpa: *const u16, fucache: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
        }
        ::core::mem::transmute(midiOutCachePatches(hmo.into_param().abi(), ::core::mem::transmute(ubank), ::core::mem::transmute(pwpa), ::core::mem::transmute(fucache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutClose<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutClose(hmo: HMIDIOUT) -> u32;
        }
        ::core::mem::transmute(midiOutClose(hmo.into_param().abi()))
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
        ::core::mem::transmute(midiOutGetDevCapsA(::core::mem::transmute(udeviceid), ::core::mem::transmute(pmoc), ::core::mem::transmute(cbmoc)))
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
        ::core::mem::transmute(midiOutGetDevCapsW(::core::mem::transmute(udeviceid), ::core::mem::transmute(pmoc), ::core::mem::transmute(cbmoc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetID<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pudeviceid: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetID(hmo.into_param().abi(), ::core::mem::transmute(pudeviceid)))
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
        ::core::mem::transmute(midiOutGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutGetVolume<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pdwvolume: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetVolume(hmo.into_param().abi(), ::core::mem::transmute(pdwvolume)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutLongMsg<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *const MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiOutLongMsg(hmo.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutMessage<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::core::mem::transmute(midiOutMessage(hmo.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
        }
        ::core::mem::transmute(midiOutOpen(::core::mem::transmute(phmo), ::core::mem::transmute(udeviceid), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutPrepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiOutPrepareHeader(hmo.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutReset<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutReset(hmo: HMIDIOUT) -> u32;
        }
        ::core::mem::transmute(midiOutReset(hmo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutSetVolume<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, dwvolume: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
        }
        ::core::mem::transmute(midiOutSetVolume(hmo.into_param().abi(), ::core::mem::transmute(dwvolume)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiOutShortMsg<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, dwmsg: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
        }
        ::core::mem::transmute(midiOutShortMsg(hmo.into_param().abi(), ::core::mem::transmute(dwmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutUnprepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiOutUnprepareHeader(hmo.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamClose<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamClose(hms: HMIDISTRM) -> u32;
        }
        ::core::mem::transmute(midiStreamClose(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
        }
        ::core::mem::transmute(midiStreamOpen(::core::mem::transmute(phms), ::core::mem::transmute(pudeviceid), ::core::mem::transmute(cmidi), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiStreamOut<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
        }
        ::core::mem::transmute(midiStreamOut(hms.into_param().abi(), ::core::mem::transmute(pmh), ::core::mem::transmute(cbmh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamPause<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamPause(hms: HMIDISTRM) -> u32;
        }
        ::core::mem::transmute(midiStreamPause(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamPosition<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
        }
        ::core::mem::transmute(midiStreamPosition(hms.into_param().abi(), ::core::mem::transmute(lpmmt), ::core::mem::transmute(cbmmt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamProperty<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0, lppropdata: *mut u8, dwproperty: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
        }
        ::core::mem::transmute(midiStreamProperty(hms.into_param().abi(), ::core::mem::transmute(lppropdata), ::core::mem::transmute(dwproperty)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamRestart<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamRestart(hms: HMIDISTRM) -> u32;
        }
        ::core::mem::transmute(midiStreamRestart(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn midiStreamStop<'a, Param0: ::windows::core::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamStop(hms: HMIDISTRM) -> u32;
        }
        ::core::mem::transmute(midiStreamStop(hms.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerClose<'a, Param0: ::windows::core::IntoParam<'a, HMIXER>>(hmx: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerClose(hmx: HMIXER) -> u32;
        }
        ::core::mem::transmute(mixerClose(hmx.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsA<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetControlDetailsA(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetControlDetailsA(hmxobj.into_param().abi(), ::core::mem::transmute(pmxcd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsW<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetControlDetailsW(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetControlDetailsW(hmxobj.into_param().abi(), ::core::mem::transmute(pmxcd), ::core::mem::transmute(fdwdetails)))
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
        ::core::mem::transmute(mixerGetDevCapsA(::core::mem::transmute(umxid), ::core::mem::transmute(pmxcaps), ::core::mem::transmute(cbmxcaps)))
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
        ::core::mem::transmute(mixerGetDevCapsW(::core::mem::transmute(umxid), ::core::mem::transmute(pmxcaps), ::core::mem::transmute(cbmxcaps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetID<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pumxid: *mut u32, fdwid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetID(hmxobj.into_param().abi(), ::core::mem::transmute(pumxid), ::core::mem::transmute(fdwid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineControlsA<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineControlsA(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetLineControlsA(hmxobj.into_param().abi(), ::core::mem::transmute(pmxlc), ::core::mem::transmute(fdwcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetLineControlsW<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineControlsW(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetLineControlsW(hmxobj.into_param().abi(), ::core::mem::transmute(pmxlc), ::core::mem::transmute(fdwcontrols)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineInfoA<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetLineInfoA(hmxobj.into_param().abi(), ::core::mem::transmute(pmxl), ::core::mem::transmute(fdwinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerGetLineInfoW<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
        }
        ::core::mem::transmute(mixerGetLineInfoW(hmxobj.into_param().abi(), ::core::mem::transmute(pmxl), ::core::mem::transmute(fdwinfo)))
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
        ::core::mem::transmute(mixerGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerMessage<'a, Param0: ::windows::core::IntoParam<'a, HMIXER>>(hmx: Param0, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
        }
        ::core::mem::transmute(mixerMessage(hmx.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(dwparam1), ::core::mem::transmute(dwparam2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
        }
        ::core::mem::transmute(mixerOpen(::core::mem::transmute(phmx), ::core::mem::transmute(umxid), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerSetControlDetails<'a, Param0: ::windows::core::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mixerSetControlDetails(hmxobj: HMIXEROBJ, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
        }
        ::core::mem::transmute(mixerSetControlDetails(hmxobj.into_param().abi(), ::core::mem::transmute(pmxcd), ::core::mem::transmute(fdwdetails)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundA(pszsound: super::super::Foundation::PSTR, fusound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(sndPlaySoundA(pszsound.into_param().abi(), ::core::mem::transmute(fusound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundW(pszsound: super::super::Foundation::PWSTR, fusound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(sndPlaySoundW(pszsound.into_param().abi(), ::core::mem::transmute(fusound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for tACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for tACMDRVOPENDESCA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for tACMDRVOPENDESCA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for tACMDRVOPENDESCA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for tACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for tACMDRVOPENDESCW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for tACMDRVOPENDESCW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for tACMDRVOPENDESCW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
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
impl ::core::default::Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for tACMFORMATDETAILSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::core::cmp::Eq for tACMFORMATDETAILSW {}
unsafe impl ::windows::core::Abi for tACMFORMATDETAILSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInAddBuffer<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveInAddBuffer(hwi.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInClose<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInClose(hwi: HWAVEIN) -> u32;
        }
        ::core::mem::transmute(waveInClose(hwi.into_param().abi()))
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
        ::core::mem::transmute(waveInGetDevCapsA(::core::mem::transmute(udeviceid), ::core::mem::transmute(pwic), ::core::mem::transmute(cbwic)))
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
        ::core::mem::transmute(waveInGetDevCapsW(::core::mem::transmute(udeviceid), ::core::mem::transmute(pwic), ::core::mem::transmute(cbwic)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveInGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveInGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetID<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, pudeviceid: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
        }
        ::core::mem::transmute(waveInGetID(hwi.into_param().abi(), ::core::mem::transmute(pudeviceid)))
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
        ::core::mem::transmute(waveInGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInGetPosition<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
        }
        ::core::mem::transmute(waveInGetPosition(hwi.into_param().abi(), ::core::mem::transmute(pmmt), ::core::mem::transmute(cbmmt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInMessage<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::core::mem::transmute(waveInMessage(hwi.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
        }
        ::core::mem::transmute(waveInOpen(::core::mem::transmute(phwi), ::core::mem::transmute(udeviceid), ::core::mem::transmute(pwfx), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInPrepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveInPrepareHeader(hwi.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInReset<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInReset(hwi: HWAVEIN) -> u32;
        }
        ::core::mem::transmute(waveInReset(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInStart<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInStart(hwi: HWAVEIN) -> u32;
        }
        ::core::mem::transmute(waveInStart(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveInStop<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInStop(hwi: HWAVEIN) -> u32;
        }
        ::core::mem::transmute(waveInStop(hwi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInUnprepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveInUnprepareHeader(hwi.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutBreakLoop<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
        }
        ::core::mem::transmute(waveOutBreakLoop(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutClose<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutClose(hwo: HWAVEOUT) -> u32;
        }
        ::core::mem::transmute(waveOutClose(hwo.into_param().abi()))
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
        ::core::mem::transmute(waveOutGetDevCapsA(::core::mem::transmute(udeviceid), ::core::mem::transmute(pwoc), ::core::mem::transmute(cbwoc)))
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
        ::core::mem::transmute(waveOutGetDevCapsW(::core::mem::transmute(udeviceid), ::core::mem::transmute(pwoc), ::core::mem::transmute(cbwoc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(psztext), ::core::mem::transmute(cchtext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetID<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pudeviceid: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetID(hwo.into_param().abi(), ::core::mem::transmute(pudeviceid)))
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
        ::core::mem::transmute(waveOutGetNumDevs())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPitch<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwpitch: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetPitch(hwo.into_param().abi(), ::core::mem::transmute(pdwpitch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPlaybackRate<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwrate: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetPlaybackRate(hwo.into_param().abi(), ::core::mem::transmute(pdwrate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetPosition<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetPosition(hwo.into_param().abi(), ::core::mem::transmute(pmmt), ::core::mem::transmute(cbmmt)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutGetVolume<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwvolume: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetVolume(hwo.into_param().abi(), ::core::mem::transmute(pdwvolume)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutMessage<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
        }
        ::core::mem::transmute(waveOutMessage(hwo.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(dw1), ::core::mem::transmute(dw2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
        }
        ::core::mem::transmute(waveOutOpen(::core::mem::transmute(phwo), ::core::mem::transmute(udeviceid), ::core::mem::transmute(pwfx), ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutPause<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutPause(hwo: HWAVEOUT) -> u32;
        }
        ::core::mem::transmute(waveOutPause(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutPrepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveOutPrepareHeader(hwo.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutReset<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutReset(hwo: HWAVEOUT) -> u32;
        }
        ::core::mem::transmute(waveOutReset(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutRestart<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutRestart(hwo: HWAVEOUT) -> u32;
        }
        ::core::mem::transmute(waveOutRestart(hwo.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetPitch<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwpitch: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
        }
        ::core::mem::transmute(waveOutSetPitch(hwo.into_param().abi(), ::core::mem::transmute(dwpitch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetPlaybackRate<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwrate: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
        }
        ::core::mem::transmute(waveOutSetPlaybackRate(hwo.into_param().abi(), ::core::mem::transmute(dwrate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn waveOutSetVolume<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwvolume: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
        }
        ::core::mem::transmute(waveOutSetVolume(hwo.into_param().abi(), ::core::mem::transmute(dwvolume)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutUnprepareHeader<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveOutUnprepareHeader(hwo.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutWrite<'a, Param0: ::windows::core::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
        }
        ::core::mem::transmute(waveOutWrite(hwo.into_param().abi(), ::core::mem::transmute(pwh), ::core::mem::transmute(cbwh)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
