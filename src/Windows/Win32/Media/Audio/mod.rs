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
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_USER: u32 = 16384u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRIVERDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_WindowsAndMessaging`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRIVERDETAILSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRVFORMATSUGGEST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRVSTREAMHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRVSTREAMINSTANCE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMDRVSTREAMSIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_BASE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_BUSY: u32 = 513u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_CANCELED: u32 = 515u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERCHOOSEA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERCHOOSEW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERDETAILSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERTAGDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFILTERTAGDETAILSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFORMATCHOOSEA {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFORMATCHOOSEW {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFORMATDETAILSA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFORMATTAGDETAILSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMFORMATTAGDETAILSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMSTREAMHEADER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ACMSTREAMHEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
impl ::core::convert::From<i32> for AMBISONICS_CHANNEL_ORDERING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AMBISONICS_CHANNEL_ORDERING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AMBISONICS_NORMALIZATION {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AMBISONICS_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AMBISONICS_TYPE(pub i32);
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
impl ::core::convert::From<i32> for AMBISONICS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AMBISONICS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287486i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287469i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287464i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287477i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287466i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287463i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287482i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287465i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287484i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287478i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287423i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287422i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287473i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287454i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287447i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287448i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287471i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287468i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287474i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287470i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287440i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287424i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287467i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287456i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287479i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287455i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287451i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287487i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NOT_STOPPED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287483i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287452i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287453i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287481i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287449i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287450i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287472i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287476i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287480i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287485i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDCLNT_SHAREMODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDCLNT_STREAMOPTIONS {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(143196161i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_POSITION_STALLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(143196163i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(143196162i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIOCLIENT_ACTIVATION_PARAMS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIOCLIENT_ACTIVATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIO_DUCKING_OPTIONS {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
pub struct AUDIO_EFFECT {
    pub id: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for AUDIO_EFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIO_EFFECT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUDIO_STREAM_CATEGORY {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for AUDIO_VOLUME_NOTIFICATION_DATA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for AUXCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUXCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for AUXCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AUXCAPSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_AUXIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IActivateAudioInterfaceCompletionHandler>>(deviceinterfacepath: Param0, riid: *const ::windows::runtime::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: Param3) -> ::windows::runtime::Result<IActivateAudioInterfaceAsyncOperation> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivateAudioInterfaceAsync(deviceinterfacepath: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, completionhandler: ::windows::runtime::RawPtr, activationoperation: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IActivateAudioInterfaceAsyncOperation as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        ActivateAudioInterfaceAsync(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(activationparams), completionhandler.into_param().abi(), &mut result__).from_abi::<IActivateAudioInterfaceAsyncOperation>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for AudioClient3ActivationParams {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for AudioClientProperties {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for AudioExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AudioObjectType {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AudioSessionDisconnectReason {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AudioSessionState {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for AudioStateMonitorSoundLevel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CoRegisterMessageFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IMessageFilter>>(lpmessagefilter: Param0) -> ::windows::runtime::Result<IMessageFilter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMessageFilter(lpmessagefilter: ::windows::runtime::RawPtr, lplpmessagefilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMessageFilter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CoRegisterMessageFilter(lpmessagefilter.into_param().abi(), &mut result__).from_abi::<IMessageFilter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ConnectorType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitor(&mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategory(::core::mem::transmute(category), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitor(&mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategory(::core::mem::transmute(category), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::runtime::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAudioStateMonitor as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), &mut result__).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(787448254, 13306, 18432, [150, 112, 28, 212, 116, 151, 44, 63]);
pub const DEVINTERFACE_AUDIO_RENDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3862068397, 56556, 18761, [174, 138, 153, 30, 151, 106, 121, 210]);
pub const DEVINTERFACE_MIDI_INPUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1347150636, 52470, 19756, [183, 63, 111, 139, 55, 71, 226, 43]);
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1841443616, 43827, 19684, [128, 212, 187, 179, 235, 191, 40, 20]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRVM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for DataFlow {
    type Abi = Self;
}
pub const DeviceTopology: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(502675920, 24257, 18346, [147, 121, 130, 141, 193, 170, 140, 89]);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ECHOWAVEFILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for EDataFlow {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for ERole {
    type Abi = Self;
}
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3804424670, 2481, 19204, [132, 229, 7, 147, 18, 37, 238, 4]);
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for EndpointFormFactor {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
#[repr(transparent)]
pub struct HACMDRIVER(pub isize);
impl ::core::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HACMDRIVER {}
unsafe impl ::windows::runtime::Abi for HACMDRIVER {
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
unsafe impl ::windows::runtime::Handle for HACMDRIVERID {}
unsafe impl ::windows::runtime::Abi for HACMDRIVERID {
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
unsafe impl ::windows::runtime::Handle for HACMOBJ {}
unsafe impl ::windows::runtime::Abi for HACMOBJ {
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
unsafe impl ::windows::runtime::Handle for HACMSTREAM {}
unsafe impl ::windows::runtime::Abi for HACMSTREAM {
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
unsafe impl ::windows::runtime::Handle for HMIDI {}
unsafe impl ::windows::runtime::Abi for HMIDI {
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
unsafe impl ::windows::runtime::Handle for HMIDIIN {}
unsafe impl ::windows::runtime::Abi for HMIDIIN {
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
unsafe impl ::windows::runtime::Handle for HMIDIOUT {}
unsafe impl ::windows::runtime::Abi for HMIDIOUT {
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
unsafe impl ::windows::runtime::Handle for HMIDISTRM {}
unsafe impl ::windows::runtime::Abi for HMIDISTRM {
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
unsafe impl ::windows::runtime::Handle for HMIXER {}
unsafe impl ::windows::runtime::Abi for HMIXER {
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
unsafe impl ::windows::runtime::Handle for HMIXEROBJ {}
unsafe impl ::windows::runtime::Abi for HMIXEROBJ {
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
unsafe impl ::windows::runtime::Handle for HWAVE {}
unsafe impl ::windows::runtime::Abi for HWAVE {
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
unsafe impl ::windows::runtime::Handle for HWAVEIN {}
unsafe impl ::windows::runtime::Abi for HWAVEIN {
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
unsafe impl ::windows::runtime::Handle for HWAVEOUT {}
unsafe impl ::windows::runtime::Abi for HWAVEOUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivateAudioInterfaceAsyncOperation(pub ::windows::runtime::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetActivateResult(&self, activateresult: *mut ::windows::runtime::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(activateresult), ::core::mem::transmute(activatedinterface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IActivateAudioInterfaceAsyncOperation {
    type Vtable = IActivateAudioInterfaceAsyncOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1923231096, 52708, 17181, [184, 204, 132, 58, 113, 25, 155, 109]);
}
impl ::core::convert::From<IActivateAudioInterfaceAsyncOperation> for ::windows::runtime::IUnknown {
    fn from(value: IActivateAudioInterfaceAsyncOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceAsyncOperation> for ::windows::runtime::IUnknown {
    fn from(value: &IActivateAudioInterfaceAsyncOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activateresult: *mut ::windows::runtime::HRESULT, activatedinterface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IActivateAudioInterfaceCompletionHandler(pub ::windows::runtime::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, IActivateAudioInterfaceAsyncOperation>>(&self, activateoperation: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), activateoperation.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IActivateAudioInterfaceCompletionHandler {
    type Vtable = IActivateAudioInterfaceCompletionHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1104759211, 39010, 17482, [128, 246, 194, 97, 51, 77, 165, 235]);
}
impl ::core::convert::From<IActivateAudioInterfaceCompletionHandler> for ::windows::runtime::IUnknown {
    fn from(value: IActivateAudioInterfaceCompletionHandler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceCompletionHandler> for ::windows::runtime::IUnknown {
    fn from(value: &IActivateAudioInterfaceCompletionHandler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activateoperation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioAmbisonicsControl(pub ::windows::runtime::IUnknown);
impl IAudioAmbisonicsControl {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pambisonicsparams), ::core::mem::transmute(cbambisonicsparams)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetHeadTracking<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benableheadtracking: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benableheadtracking.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetHeadTracking(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioAmbisonicsControl {
    type Vtable = IAudioAmbisonicsControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(678579345, 57141, 18518, [159, 118, 214, 162, 100, 19, 243, 223]);
}
impl ::core::convert::From<IAudioAmbisonicsControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioAmbisonicsControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioAmbisonicsControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioAmbisonicsControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32, w: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioAutoGainControl(pub ::windows::runtime::IUnknown);
impl IAudioAutoGainControl {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioAutoGainControl {
    type Vtable = IAudioAutoGainControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2235572180, 28132, 19357, [152, 105, 45, 103, 83, 168, 47, 60]);
}
impl ::core::convert::From<IAudioAutoGainControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioAutoGainControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioAutoGainControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioAutoGainControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioAutoGainControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioAutoGainControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioBass(pub ::windows::runtime::IUnknown);
impl IAudioBass {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioBass {
    type Vtable = IAudioBass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2729550297, 19891, 16989, [162, 178, 189, 51, 92, 179, 226, 229]);
}
impl ::core::convert::From<IAudioBass> for ::windows::runtime::IUnknown {
    fn from(value: IAudioBass) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioBass> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioBass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioBass {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioBass {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for IAudioBass {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for &IAudioBass {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioCaptureClient(pub ::windows::runtime::IUnknown);
impl IAudioCaptureClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdata), ::core::mem::transmute(pnumframestoread), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pu64deviceposition), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesread)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetNextPacketSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioCaptureClient {
    type Vtable = IAudioCaptureClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3366829412, 59166, 18592, [164, 222, 24, 92, 57, 92, 211, 23]);
}
impl ::core::convert::From<IAudioCaptureClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioCaptureClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioCaptureClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioCaptureClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioCaptureClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioCaptureClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numframesread: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumframesinnextpacket: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioChannelConfig(pub ::windows::runtime::IUnknown);
impl IAudioChannelConfig {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconfig), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelConfig(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioChannelConfig {
    type Vtable = IAudioChannelConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3138503791, 60456, 18748, [184, 138, 93, 184, 128, 98, 206, 152]);
}
impl ::core::convert::From<IAudioChannelConfig> for ::windows::runtime::IUnknown {
    fn from(value: IAudioChannelConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioChannelConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioChannelConfig) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioChannelConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioChannelConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwconfig: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwconfig: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient(pub ::windows::runtime::IUnknown);
impl IAudioClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClient {
    type Vtable = IAudioClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(481930572, 56314, 19506, [177, 120, 194, 245, 104, 167, 3, 178]);
}
impl ::core::convert::From<IAudioClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumbufferframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnslatency: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient2(pub ::windows::runtime::IUnknown);
impl IAudioClient2 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClient2 {
    type Vtable = IAudioClient2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1919383757, 62986, 20186, [130, 222, 228, 118, 16, 205, 120, 170]);
}
impl ::core::convert::From<IAudioClient2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClient2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClient2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient> for IAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient> for &IAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumbufferframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnslatency: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproperties: *const AudioClientProperties) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClient3(pub ::windows::runtime::IUnknown);
impl IAudioClient3 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), ::core::mem::transmute(pdefaultperiodinframes), ::core::mem::transmute(pfundamentalperiodinframes), ::core::mem::transmute(pminperiodinframes), ::core::mem::transmute(pmaxperiodinframes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformat), ::core::mem::transmute(pcurrentperiodinframes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamflags), ::core::mem::transmute(periodinframes), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClient3 {
    type Vtable = IAudioClient3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2127883783, 36455, 19668, [140, 26, 43, 122, 89, 135, 173, 66]);
}
impl ::core::convert::From<IAudioClient3> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClient3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClient3> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClient3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient2> for IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient2> for &IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient> for IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioClient> for &IAudioClient3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioClient> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumbufferframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnslatency: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnumpaddingframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproperties: *const AudioClientProperties) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClientDuckingControl(pub ::windows::runtime::IUnknown);
impl IAudioClientDuckingControl {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClientDuckingControl {
    type Vtable = IAudioClientDuckingControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3347698561, 41612, 16744, [178, 143, 211, 168, 55, 146, 77, 195]);
}
impl ::core::convert::From<IAudioClientDuckingControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClientDuckingControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClientDuckingControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClientDuckingControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: AUDIO_DUCKING_OPTIONS) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClock(pub ::windows::runtime::IUnknown);
impl IAudioClock {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pu64position), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCharacteristics(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClock {
    type Vtable = IAudioClock_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3445829967, 16314, 18971, [129, 44, 239, 150, 53, 135, 40, 231]);
}
impl ::core::convert::From<IAudioClock> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClock) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClock> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClock) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClock {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pu64frequency: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcharacteristics: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClock2(pub ::windows::runtime::IUnknown);
impl IAudioClock2 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceposition), ::core::mem::transmute(qpcposition)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClock2 {
    type Vtable = IAudioClock2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1867120499, 26407, 18860, [160, 8, 217, 140, 245, 231, 0, 72]);
}
impl ::core::convert::From<IAudioClock2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClock2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClock2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClock2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClock2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClock2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioClockAdjustment(pub ::windows::runtime::IUnknown);
impl IAudioClockAdjustment {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flsamplerate)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioClockAdjustment {
    type Vtable = IAudioClockAdjustment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4142186656, 18137, 20408, [190, 33, 87, 163, 239, 43, 98, 108]);
}
impl ::core::convert::From<IAudioClockAdjustment> for ::windows::runtime::IUnknown {
    fn from(value: IAudioClockAdjustment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioClockAdjustment> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioClockAdjustment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioClockAdjustment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioClockAdjustment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flsamplerate: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEffectsChangedNotificationClient(pub ::windows::runtime::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnAudioEffectsChanged(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEffectsChangedNotificationClient {
    type Vtable = IAudioEffectsChangedNotificationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2782843983, 15453, 19243, [189, 30, 93, 193, 238, 32, 187, 246]);
}
impl ::core::convert::From<IAudioEffectsChangedNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEffectsChangedNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEffectsChangedNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEffectsChangedNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioEffectsManager(pub ::windows::runtime::IUnknown);
impl IAudioEffectsManager {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetAudioEffectState<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, effectid: Param0, state: AUDIO_EFFECT_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioEffectsManager {
    type Vtable = IAudioEffectsManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1147188142, 19268, 17703, [134, 118, 117, 72, 168, 172, 210, 96]);
}
impl ::core::convert::From<IAudioEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: IAudioEffectsManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioEffectsManager> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioEffectsManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioEffectsManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, client: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, client: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, effectid: ::windows::runtime::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioFormatEnumerator(pub ::windows::runtime::IUnknown);
impl IAudioFormatEnumerator {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetFormat(&self, index: u32) -> ::windows::runtime::Result<*mut WAVEFORMATEX> {
        let mut result__: <*mut WAVEFORMATEX as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<*mut WAVEFORMATEX>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioFormatEnumerator {
    type Vtable = IAudioFormatEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3705317464, 35162, 18978, [165, 235, 103, 189, 165, 6, 9, 109]);
}
impl ::core::convert::From<IAudioFormatEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAudioFormatEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioFormatEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioFormatEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioInputSelector(pub ::windows::runtime::IUnknown);
impl IAudioInputSelector {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioInputSelector {
    type Vtable = IAudioInputSelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1325652994, 24174, 18003, [143, 114, 160, 48, 193, 35, 213, 152]);
}
impl ::core::convert::From<IAudioInputSelector> for ::windows::runtime::IUnknown {
    fn from(value: IAudioInputSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioInputSelector> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioInputSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioInputSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioInputSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnidselected: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nidselect: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioLoudness(pub ::windows::runtime::IUnknown);
impl IAudioLoudness {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioLoudness {
    type Vtable = IAudioLoudness_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2106266679, 56659, 17232, [156, 27, 30, 226, 137, 11, 217, 56]);
}
impl ::core::convert::From<IAudioLoudness> for ::windows::runtime::IUnknown {
    fn from(value: IAudioLoudness) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioLoudness> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioLoudness) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioLoudness {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioLoudness {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMidrange(pub ::windows::runtime::IUnknown);
impl IAudioMidrange {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioMidrange {
    type Vtable = IAudioMidrange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1582610135, 46155, 16601, [154, 158, 230, 145, 217, 206, 110, 223]);
}
impl ::core::convert::From<IAudioMidrange> for ::windows::runtime::IUnknown {
    fn from(value: IAudioMidrange) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMidrange> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioMidrange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioMidrange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioMidrange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for IAudioMidrange {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for &IAudioMidrange {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioMute(pub ::windows::runtime::IUnknown);
impl IAudioMute {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetMute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmuted: Param0, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bmuted.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetMute(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioMute {
    type Vtable = IAudioMute_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3745885930, 46922, 19307, [175, 173, 35, 102, 182, 170, 1, 46]);
}
impl ::core::convert::From<IAudioMute> for ::windows::runtime::IUnknown {
    fn from(value: IAudioMute) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioMute> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioMute) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioMute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioMute {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioOutputSelector(pub ::windows::runtime::IUnknown);
impl IAudioOutputSelector {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioOutputSelector {
    type Vtable = IAudioOutputSelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3142672233, 38055, 17054, [139, 156, 39, 27, 63, 17, 163, 171]);
}
impl ::core::convert::From<IAudioOutputSelector> for ::windows::runtime::IUnknown {
    fn from(value: IAudioOutputSelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioOutputSelector> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioOutputSelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioOutputSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioOutputSelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnidselected: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nidselect: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioPeakMeter(pub ::windows::runtime::IUnknown);
impl IAudioPeakMeter {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioPeakMeter {
    type Vtable = IAudioPeakMeter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3715732028, 1433, 17888, [184, 182, 200, 223, 125, 182, 231, 150]);
}
impl ::core::convert::From<IAudioPeakMeter> for ::windows::runtime::IUnknown {
    fn from(value: IAudioPeakMeter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioPeakMeter> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioPeakMeter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioPeakMeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioPeakMeter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioRenderClient(pub ::windows::runtime::IUnknown);
impl IAudioRenderClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesrequested), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframeswritten), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioRenderClient {
    type Vtable = IAudioRenderClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4069829884, 12614, 17539, [167, 191, 173, 220, 167, 194, 96, 226]);
}
impl ::core::convert::From<IAudioRenderClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioRenderClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioRenderClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioRenderClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioRenderClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioRenderClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, numframeswritten: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionControl(pub ::windows::runtime::IUnknown);
impl IAudioSessionControl {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<AudioSessionState> {
        let mut result__: <AudioSessionState as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioSessionState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetIconPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetGroupingParam(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionControl {
    type Vtable = IAudioSessionControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4105282969, 29286, 17177, [168, 202, 231, 10, 203, 17, 232, 205]);
}
impl ::core::convert::From<IAudioSessionControl> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionControl> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut AudioSessionState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#override: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newnotifications: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newnotifications: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionControl2(pub ::windows::runtime::IUnknown);
impl IAudioSessionControl2 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<AudioSessionState> {
        let mut result__: <AudioSessionState as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioSessionState>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetIconPath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, value: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetGroupingParam(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetSessionIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsSystemSoundsSession(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetDuckingPreference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, optout: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), optout.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionControl2 {
    type Vtable = IAudioSessionControl2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3216506760, 29241, 20425, [143, 162, 7, 201, 80, 190, 156, 109]);
}
impl ::core::convert::From<IAudioSessionControl2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionControl2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionControl2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionControl2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSessionControl> for IAudioSessionControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSessionControl> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSessionControl> for &IAudioSessionControl2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSessionControl> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut AudioSessionState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#override: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newnotifications: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newnotifications: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pretval: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, optout: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionEnumerator(pub ::windows::runtime::IUnknown);
impl IAudioSessionEnumerator {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSession(&self, sessioncount: i32) -> ::windows::runtime::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessioncount), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionEnumerator {
    type Vtable = IAudioSessionEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3807755025, 1392, 16586, [172, 221, 58, 160, 18, 119, 222, 232]);
}
impl ::core::convert::From<IAudioSessionEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessioncount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessioncount: i32, session: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionEvents(pub ::windows::runtime::IUnknown);
impl IAudioSessionEvents {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnDisplayNameChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, newdisplayname: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newdisplayname.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnIconPathChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, newiconpath: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), newiconpath.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnSimpleVolumeChanged<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, newvolume: f32, newmute: Param1, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(newvolume), newmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(channelcount), ::core::mem::transmute(newchannelvolumearray), ::core::mem::transmute(changedchannel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(newgroupingparam), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstate)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(disconnectreason)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionEvents {
    type Vtable = IAudioSessionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(613518028, 25779, 14273, [140, 169, 116, 166, 110, 153, 87, 168]);
}
impl ::core::convert::From<IAudioSessionEvents> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newdisplayname: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newiconpath: super::super::Foundation::PWSTR, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newgroupingparam: *const ::windows::runtime::GUID, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newstate: AudioSessionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disconnectreason: AudioSessionDisconnectReason) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionManager(pub ::windows::runtime::IUnknown);
impl IAudioSessionManager {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32) -> ::windows::runtime::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32) -> ::windows::runtime::Result<ISimpleAudioVolume> {
        let mut result__: <ISimpleAudioVolume as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<ISimpleAudioVolume>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionManager {
    type Vtable = IAudioSessionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3215553009, 19806, 16571, [147, 94, 150, 112, 57, 191, 190, 228]);
}
impl ::core::convert::From<IAudioSessionManager> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionManager> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32, sessioncontrol: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32, audiovolume: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionManager2(pub ::windows::runtime::IUnknown);
impl IAudioSessionManager2 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32) -> ::windows::runtime::Result<IAudioSessionControl> {
        let mut result__: <IAudioSessionControl as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<IAudioSessionControl>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32) -> ::windows::runtime::Result<ISimpleAudioVolume> {
        let mut result__: <ISimpleAudioVolume as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), &mut result__).from_abi::<ISimpleAudioVolume>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSessionEnumerator(&self) -> ::windows::runtime::Result<IAudioSessionEnumerator> {
        let mut result__: <IAudioSessionEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioSessionEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterSessionNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn RegisterDuckNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, sessionid: Param0, ducknotification: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ducknotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterDuckNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, ducknotification: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ducknotification.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionManager2 {
    type Vtable = IAudioSessionManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2007669152, 7126, 18511, [139, 199, 44, 101, 76, 154, 155, 111]);
}
impl ::core::convert::From<IAudioSessionManager2> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSessionManager> for IAudioSessionManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSessionManager> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAudioSessionManager> for &IAudioSessionManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAudioSessionManager> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32, sessioncontrol: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audiosessionguid: *const ::windows::runtime::GUID, streamflags: u32, audiovolume: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionnotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionnotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: super::super::Foundation::PWSTR, ducknotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ducknotification: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSessionNotification(pub ::windows::runtime::IUnknown);
impl IAudioSessionNotification {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnSessionCreated<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSessionControl>>(&self, newsession: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), newsession.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSessionNotification {
    type Vtable = IAudioSessionNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1679675915, 19777, 18892, [171, 163, 23, 75, 148, 119, 187, 8]);
}
impl ::core::convert::From<IAudioSessionNotification> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSessionNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSessionNotification> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSessionNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSessionNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSessionNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newsession: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioStateMonitor(pub ::windows::runtime::IUnknown);
impl IAudioStateMonitor {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterCallback(&self, callback: ::core::option::Option<PAudioStateMonitorCallback>, context: *const ::core::ffi::c_void) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(callback), ::core::mem::transmute(context), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(registration)))
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1673365304, 58125, 19575, [191, 92, 131, 78, 135, 198, 87, 226]);
}
impl ::core::convert::From<IAudioStateMonitor> for ::windows::runtime::IUnknown {
    fn from(value: IAudioStateMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioStateMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioStateMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioStateMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioStateMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, registration: i64),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> AudioStateMonitorSoundLevel,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioStreamVolume(pub ::windows::runtime::IUnknown);
impl IAudioStreamVolume {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioStreamVolume {
    type Vtable = IAudioStreamVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2466334855, 9261, 16488, [138, 21, 207, 94, 147, 185, 15, 227]);
}
impl ::core::convert::From<IAudioStreamVolume> for ::windows::runtime::IUnknown {
    fn from(value: IAudioStreamVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioStreamVolume> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioStreamVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioStreamVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioStreamVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, flevel: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcount: u32, pfvolumes: *const f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcount: u32, pfvolumes: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OnPropertyChanged<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), key.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Vtable = IAudioSystemEffectsPropertyChangeNotificationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(537173312, 22229, 16398, [162, 239, 56, 85, 153, 254, 237, 73]);
}
impl ::core::convert::From<IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioSystemEffectsPropertyStore(pub ::windows::runtime::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ResetUserPropertyStore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ResetVolatilePropertyStore(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterPropertyChangeNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterPropertyChangeNotification<'a, Param0: ::windows::runtime::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioSystemEffectsPropertyStore {
    type Vtable = IAudioSystemEffectsPropertyStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(808118265, 55264, 17380, [151, 27, 31, 130, 147, 97, 61, 42]);
}
impl ::core::convert::From<IAudioSystemEffectsPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stgmaccess: u32, propstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stgmaccess: u32, propstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stgmaccess: u32, propstore: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioTreble(pub ::windows::runtime::IUnknown);
impl IAudioTreble {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioTreble {
    type Vtable = IAudioTreble_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(175208466, 26958, 18695, [183, 75, 186, 250, 92, 253, 202, 123]);
}
impl ::core::convert::From<IAudioTreble> for ::windows::runtime::IUnknown {
    fn from(value: IAudioTreble) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioTreble> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioTreble) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioTreble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioTreble {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for IAudioTreble {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for &IAudioTreble {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioVolumeDuckNotification(pub ::windows::runtime::IUnknown);
impl IAudioVolumeDuckNotification {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnVolumeDuckNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: Param0, countcommunicationsessions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ::core::mem::transmute(countcommunicationsessions)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnVolumeUnduckNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, sessionid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), sessionid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioVolumeDuckNotification {
    type Vtable = IAudioVolumeDuckNotification_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3283256532, 27961, 17241, [179, 207, 181, 109, 219, 59, 179, 156]);
}
impl ::core::convert::From<IAudioVolumeDuckNotification> for ::windows::runtime::IUnknown {
    fn from(value: IAudioVolumeDuckNotification) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioVolumeDuckNotification> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioVolumeDuckNotification) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: super::super::Foundation::PWSTR, countcommunicationsessions: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IAudioVolumeLevel(pub ::windows::runtime::IUnknown);
impl IAudioVolumeLevel {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAudioVolumeLevel {
    type Vtable = IAudioVolumeLevel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2142745743, 21277, 17570, [188, 179, 90, 213, 161, 52, 179, 220]);
}
impl ::core::convert::From<IAudioVolumeLevel> for ::windows::runtime::IUnknown {
    fn from(value: IAudioVolumeLevel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAudioVolumeLevel> for ::windows::runtime::IUnknown {
    fn from(value: &IAudioVolumeLevel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAudioVolumeLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAudioVolumeLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for IAudioVolumeLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPerChannelDbLevel> for &IAudioVolumeLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPerChannelDbLevel> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IChannelAudioVolume(pub ::windows::runtime::IUnknown);
impl IChannelAudioVolume {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcount), ::core::mem::transmute(pfvolumes)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IChannelAudioVolume {
    type Vtable = IChannelAudioVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(471173217, 46387, 19248, [177, 207, 232, 83, 229, 28, 89, 184]);
}
impl ::core::convert::From<IChannelAudioVolume> for ::windows::runtime::IUnknown {
    fn from(value: IChannelAudioVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IChannelAudioVolume> for ::windows::runtime::IUnknown {
    fn from(value: &IChannelAudioVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChannelAudioVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IChannelAudioVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, flevel: f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcount: u32, pfvolumes: *mut f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnector(pub ::windows::runtime::IUnknown);
impl IConnector {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<ConnectorType> {
        let mut result__: <ConnectorType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ConnectorType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDataFlow(&self) -> ::windows::runtime::Result<DataFlow> {
        let mut result__: <DataFlow as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DataFlow>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ConnectTo<'a, Param0: ::windows::runtime::IntoParam<'a, IConnector>>(&self, pconnectto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pconnectto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetConnectedTo(&self) -> ::windows::runtime::Result<IConnector> {
        let mut result__: <IConnector as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IConnector>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IConnector {
    type Vtable = IConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2620145752, 9205, 16862, [135, 122, 223, 58, 242, 54, 160, 158]);
}
impl ::core::convert::From<IConnector> for ::windows::runtime::IUnknown {
    fn from(value: IConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut ConnectorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflow: *mut DataFlow) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnectto: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconto: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrconnectorid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IControlChangeNotify(pub ::windows::runtime::IUnknown);
impl IControlChangeNotify {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsenderprocessid), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IControlChangeNotify {
    type Vtable = IControlChangeNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2694124525, 50953, 19745, [189, 123, 95, 52, 196, 127, 57, 71]);
}
impl ::core::convert::From<IControlChangeNotify> for ::windows::runtime::IUnknown {
    fn from(value: IControlChangeNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IControlChangeNotify> for ::windows::runtime::IUnknown {
    fn from(value: &IControlChangeNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IControlChangeNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IControlChangeNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwsenderprocessid: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IControlInterface(pub ::windows::runtime::IUnknown);
impl IControlInterface {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetIID(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IControlInterface {
    type Vtable = IControlInterface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1171487807, 20800, 17482, [174, 36, 64, 7, 137, 243, 203, 243]);
}
impl ::core::convert::From<IControlInterface> for ::windows::runtime::IUnknown {
    fn from(value: IControlInterface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IControlInterface> for ::windows::runtime::IUnknown {
    fn from(value: &IControlInterface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IControlInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IControlInterface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceSpecificProperty(pub ::windows::runtime::IUnknown);
impl IDeviceSpecificProperty {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetValue(&self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetValue(&self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(cbvalue), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plstepping)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDeviceSpecificProperty {
    type Vtable = IDeviceSpecificProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(992132287, 9606, 19184, [133, 131, 32, 93, 57, 27, 128, 124]);
}
impl ::core::convert::From<IDeviceSpecificProperty> for ::windows::runtime::IUnknown {
    fn from(value: IDeviceSpecificProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceSpecificProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IDeviceSpecificProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvtype: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDeviceTopology(pub ::windows::runtime::IUnknown);
impl IDeviceTopology {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetConnectorCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetConnector(&self, nindex: u32) -> ::windows::runtime::Result<IConnector> {
        let mut result__: <IConnector as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IConnector>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSubunitCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSubunit(&self, nindex: u32) -> ::windows::runtime::Result<ISubunit> {
        let mut result__: <ISubunit as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<ISubunit>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetPartById(&self, nid: u32) -> ::windows::runtime::Result<IPart> {
        let mut result__: <IPart as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(nid), &mut result__).from_abi::<IPart>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetSignalPath<'a, Param0: ::windows::runtime::IntoParam<'a, IPart>, Param1: ::windows::runtime::IntoParam<'a, IPart>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pipartfrom: Param0, pipartto: Param1, brejectmixedpaths: Param2) -> ::windows::runtime::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pipartfrom.into_param().abi(), pipartto.into_param().abi(), brejectmixedpaths.into_param().abi(), &mut result__).from_abi::<IPartsList>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDeviceTopology {
    type Vtable = IDeviceTopology_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(705118334, 25751, 18968, [151, 135, 50, 247, 155, 208, 217, 143]);
}
impl ::core::convert::From<IDeviceTopology> for ::windows::runtime::IUnknown {
    fn from(value: IDeviceTopology) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceTopology> for ::windows::runtime::IUnknown {
    fn from(value: &IDeviceTopology) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDeviceTopology {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDeviceTopology {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, ppsubunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nid: u32, pppart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipartfrom: ::windows::runtime::RawPtr, pipartto: ::windows::runtime::RawPtr, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDevice(pub ::windows::runtime::IUnknown);
impl IMMDevice {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Activate(&self, iid: *const ::windows::runtime::GUID, dwclsctx: u32, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: u32) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetState(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMMDevice {
    type Vtable = IMMDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3597010495, 5511, 20035, [129, 241, 185, 72, 232, 7, 54, 63]);
}
impl ::core::convert::From<IMMDevice> for ::windows::runtime::IUnknown {
    fn from(value: IMMDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IMMDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, dwclsctx: u32, pactivationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stgmaccess: u32, ppproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstrid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwstate: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceActivator(pub ::windows::runtime::IUnknown);
impl IMMDeviceActivator {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Activate<'a, Param1: ::windows::runtime::IntoParam<'a, IMMDevice>>(&self, iid: *const ::windows::runtime::GUID, pdevice: Param1, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), pdevice.into_param().abi(), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMMDeviceActivator {
    type Vtable = IMMDeviceActivator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(990711460, 53417, 19214, [147, 91, 9, 81, 103, 70, 250, 192]);
}
impl ::core::convert::From<IMMDeviceActivator> for ::windows::runtime::IUnknown {
    fn from(value: IMMDeviceActivator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceActivator> for ::windows::runtime::IUnknown {
    fn from(value: &IMMDeviceActivator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMDeviceActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMDeviceActivator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, pdevice: ::windows::runtime::RawPtr, pactivationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceCollection(pub ::windows::runtime::IUnknown);
impl IMMDeviceCollection {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Item(&self, ndevice: u32) -> ::windows::runtime::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(ndevice), &mut result__).from_abi::<IMMDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMMDeviceCollection {
    type Vtable = IMMDeviceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(198681022, 31258, 17627, [131, 151, 204, 83, 146, 56, 123, 94]);
}
impl ::core::convert::From<IMMDeviceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IMMDeviceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IMMDeviceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMDeviceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMDeviceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcdevices: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ndevice: u32, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMDeviceEnumerator(pub ::windows::runtime::IUnknown);
impl IMMDeviceEnumerator {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::runtime::Result<IMMDeviceCollection> {
        let mut result__: <IMMDeviceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(dwstatemask), &mut result__).from_abi::<IMMDeviceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> ::windows::runtime::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(role), &mut result__).from_abi::<IMMDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrid: Param0) -> ::windows::runtime::Result<IMMDevice> {
        let mut result__: <IMMDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwstrid.into_param().abi(), &mut result__).from_abi::<IMMDevice>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterEndpointNotificationCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterEndpointNotificationCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMMDeviceEnumerator {
    type Vtable = IMMDeviceEnumerator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2841011410, 38420, 20277, [167, 70, 222, 141, 182, 54, 23, 230]);
}
impl ::core::convert::From<IMMDeviceEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: IMMDeviceEnumerator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMDeviceEnumerator> for ::windows::runtime::IUnknown {
    fn from(value: &IMMDeviceEnumerator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataflow: EDataFlow, role: ERole, ppendpoint: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrid: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclient: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclient: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMEndpoint(pub ::windows::runtime::IUnknown);
impl IMMEndpoint {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetDataFlow(&self) -> ::windows::runtime::Result<EDataFlow> {
        let mut result__: <EDataFlow as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<EDataFlow>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMMEndpoint {
    type Vtable = IMMEndpoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(467703688, 26772, 16521, [133, 134, 154, 42, 108, 38, 90, 197]);
}
impl ::core::convert::From<IMMEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: IMMEndpoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMEndpoint> for ::windows::runtime::IUnknown {
    fn from(value: &IMMEndpoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMEndpoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataflow: *mut EDataFlow) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMMNotificationClient(pub ::windows::runtime::IUnknown);
impl IMMNotificationClient {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnDeviceStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0, dwnewstate: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), ::core::mem::transmute(dwnewstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnDeviceAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnDeviceRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn OnDefaultDeviceChanged<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(flow), ::core::mem::transmute(role), pwstrdefaultdeviceid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn OnPropertyValueChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, pwstrdeviceid: Param0, key: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), key.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMMNotificationClient {
    type Vtable = IMMNotificationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2039606985, 32393, 19845, [131, 144, 108, 112, 60, 236, 96, 192]);
}
impl ::core::convert::From<IMMNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: IMMNotificationClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMMNotificationClient> for ::windows::runtime::IUnknown {
    fn from(value: &IMMNotificationClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMMNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMMNotificationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR, dwnewstate: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwstrdeviceid: super::super::Foundation::PWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMessageFilter(pub ::windows::runtime::IUnknown);
impl IMessageFilter {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_System_Com`*"]
    pub unsafe fn HandleInComingCall<'a, Param1: ::windows::runtime::IntoParam<'a, super::HTASK>>(&self, dwcalltype: u32, htaskcaller: Param1, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcalltype), htaskcaller.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(lpinterfaceinfo)))
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RetryRejectedCall<'a, Param0: ::windows::runtime::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwrejecttype: u32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwrejecttype)))
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn MessagePending<'a, Param0: ::windows::runtime::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwpendingtype: u32) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwpendingtype)))
    }
}
unsafe impl ::windows::runtime::Interface for IMessageFilter {
    type Vtable = IMessageFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(22, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::core::convert::From<IMessageFilter> for ::windows::runtime::IUnknown {
    fn from(value: IMessageFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMessageFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IMessageFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMessageFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMessageFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::INTERFACEINFO>) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPart(pub ::windows::runtime::IUnknown);
impl IPart {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLocalId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetGlobalId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetPartType(&self) -> ::windows::runtime::Result<PartType> {
        let mut result__: <PartType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<PartType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSubType(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetControlInterfaceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> ::windows::runtime::Result<IControlInterface> {
        let mut result__: <IControlInterface as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IControlInterface>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EnumPartsIncoming(&self) -> ::windows::runtime::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPartsList>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EnumPartsOutgoing(&self) -> ::windows::runtime::Result<IPartsList> {
        let mut result__: <IPartsList as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPartsList>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetTopologyObject(&self) -> ::windows::runtime::Result<IDeviceTopology> {
        let mut result__: <IDeviceTopology as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDeviceTopology>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(refiid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn RegisterControlChangeCallback<'a, Param1: ::windows::runtime::IntoParam<'a, IControlChangeNotify>>(&self, riid: *const ::windows::runtime::GUID, pnotify: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn UnregisterControlChangeCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IControlChangeNotify>>(&self, pnotify: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPart {
    type Vtable = IPart_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2922242276, 23498, 20269, [170, 70, 93, 19, 248, 253, 179, 169]);
}
impl ::core::convert::From<IPart> for ::windows::runtime::IUnknown {
    fn from(value: IPart) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPart> for ::windows::runtime::IUnknown {
    fn from(value: &IPart) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPart {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwstrglobalid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparttype: *mut PartType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psubtype: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, ppinterfacedesc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppparts: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppparts: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptopology: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclscontext: u32, refiid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnotify: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPartsList(pub ::windows::runtime::IUnknown);
impl IPartsList {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetPart(&self, nindex: u32) -> ::windows::runtime::Result<IPart> {
        let mut result__: <IPart as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), &mut result__).from_abi::<IPart>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPartsList {
    type Vtable = IPartsList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1839891596, 24240, 17868, [174, 165, 153, 138, 44, 218, 31, 251]);
}
impl ::core::convert::From<IPartsList> for ::windows::runtime::IUnknown {
    fn from(value: IPartsList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPartsList> for ::windows::runtime::IUnknown {
    fn from(value: &IPartsList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPartsList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPartsList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nindex: u32, pppart: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPerChannelDbLevel(pub ::windows::runtime::IUnknown);
impl IPerChannelDbLevel {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(alevelsdb), ::core::mem::transmute(cchannels), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPerChannelDbLevel {
    type Vtable = IPerChannelDbLevel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3271090177, 61957, 19401, [153, 188, 193, 59, 30, 4, 140, 203]);
}
impl ::core::convert::From<IPerChannelDbLevel> for ::windows::runtime::IUnknown {
    fn from(value: IPerChannelDbLevel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPerChannelDbLevel> for ::windows::runtime::IUnknown {
    fn from(value: &IPerChannelDbLevel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPerChannelDbLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPerChannelDbLevel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcchannels: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, pfleveldb: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fleveldb: f32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISimpleAudioVolume(pub ::windows::runtime::IUnknown);
impl ISimpleAudioVolume {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMasterVolume(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn SetMute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmute: Param0, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetMute(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISimpleAudioVolume {
    type Vtable = ISimpleAudioVolume_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2278446232, 26838, 17637, [146, 21, 109, 164, 126, 248, 131, 216]);
}
impl ::core::convert::From<ISimpleAudioVolume> for ::windows::runtime::IUnknown {
    fn from(value: ISimpleAudioVolume) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISimpleAudioVolume> for ::windows::runtime::IUnknown {
    fn from(value: &ISimpleAudioVolume) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISimpleAudioVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISimpleAudioVolume {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flevel: f32, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflevel: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioClient(pub ::windows::runtime::IUnknown);
impl ISpatialAudioClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::runtime::Result<IAudioFormatEnumerator> {
        let mut result__: <IAudioFormatEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioFormatEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::runtime::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::runtime::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioClient {
    type Vtable = ISpatialAudioClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3153649766, 43690, 18878, [154, 77, 253, 42, 133, 142, 162, 127]);
}
impl ::core::convert::From<ISpatialAudioClient> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioClient> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamuuid: *const ::windows::runtime::GUID, auxiliaryinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioClient2(pub ::windows::runtime::IUnknown);
impl ISpatialAudioClient2 {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::runtime::Result<IAudioFormatEnumerator> {
        let mut result__: <IAudioFormatEnumerator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IAudioFormatEnumerator>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::runtime::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::runtime::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn GetMaxFrameCountForCategory<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: Param1, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), offloadenabled.into_param().abi(), ::core::mem::transmute(objectformat), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioClient2 {
    type Vtable = ISpatialAudioClient2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3400262738, 42602, 19438, [169, 62, 227, 32, 70, 63, 106, 83]);
}
impl ::core::convert::From<ISpatialAudioClient2> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioClient2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioClient2> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioClient2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioClient> for ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioClient> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioClient> for &ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioClient> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mask: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enumerator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objectformat: *const WAVEFORMATEX) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, streamuuid: *const ::windows::runtime::GUID, auxiliaryinfo: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationparams: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>, riid: *const ::windows::runtime::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataClient(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataClient {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), ::core::mem::transmute(framecount), ::core::mem::transmute(metadataitemsbuffer), ::core::mem::transmute(metadataitems)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::runtime::Result<ISpatialAudioMetadataWriter> {
        let mut result__: <ISpatialAudioMetadataWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(overflowmode), &mut result__).from_abi::<ISpatialAudioMetadataWriter>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> ::windows::runtime::Result<ISpatialAudioMetadataCopier> {
        let mut result__: <ISpatialAudioMetadataCopier as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataCopier>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> ::windows::runtime::Result<ISpatialAudioMetadataReader> {
        let mut result__: <ISpatialAudioMetadataReader as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataReader>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataClient {
    type Vtable = ISpatialAudioMetadataClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2004699707, 63231, 18982, [133, 220, 104, 215, 205, 237, 161, 212]);
}
impl ::core::convert::From<ISpatialAudioMetadataClient> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataClient) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataClient> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::windows::runtime::RawPtr, metadataitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadatacopier: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadatareader: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataCopier(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataCopier {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn CopyMetadataForFrames<'a, Param2: ::windows::runtime::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: Param2) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(copyframecount), ::core::mem::transmute(copymode), dstmetadataitems.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataCopier {
    type Vtable = ISpatialAudioMetadataCopier_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3525620275, 57937, 20432, [156, 162, 213, 236, 249, 166, 132, 4]);
}
impl ::core::convert::From<ISpatialAudioMetadataCopier> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataCopier) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataCopier> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataCopier) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataitems: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::windows::runtime::RawPtr, itemscopied: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataItems(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataItems {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetFrameCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxItemCount(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetMaxValueBufferLength(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<SpatialAudioMetadataItemsInfo> {
        let mut result__: <SpatialAudioMetadataItemsInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SpatialAudioMetadataItemsInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataItems {
    type Vtable = ISpatialAudioMetadataItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3168257935, 12440, 20258, [181, 71, 162, 242, 90, 56, 18, 105]);
}
impl ::core::convert::From<ISpatialAudioMetadataItems> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItems> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itemcount: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxitemcount: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxvaluebufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataItemsBuffer(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn DetachBuffer(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataItemsBuffer {
    type Vtable = ISpatialAudioMetadataItemsBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1113852438, 57789, 17113, [159, 246, 3, 26, 183, 26, 45, 186]);
}
impl ::core::convert::From<ISpatialAudioMetadataItemsBuffer> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataItemsBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItemsBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataItemsBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut u8, bufferlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut u8, bufferlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataReader(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataReader {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandcount), ::core::mem::transmute(frameoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(maxvaluebufferlength), ::core::mem::transmute(valuebufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataReader {
    type Vtable = ISpatialAudioMetadataReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3079571106, 12761, 19506, [148, 210, 125, 244, 15, 199, 235, 236]);
}
impl ::core::convert::From<ISpatialAudioMetadataReader> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataReader> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataitems: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioMetadataWriter(pub ::windows::runtime::IUnknown);
impl ISpatialAudioMetadataWriter {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(frameoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioMetadataWriter {
    type Vtable = ISpatialAudioMetadataWriter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(454543873, 10581, 17485, [164, 48, 83, 125, 197, 137, 168, 68]);
}
impl ::core::convert::From<ISpatialAudioMetadataWriter> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioMetadataWriter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataWriter> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioMetadataWriter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataitems: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, frameoffset: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObject(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObject {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsActive(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetVolume(&self, volume: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(volume)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObject {
    type Vtable = ISpatialAudioObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3722611047, 21019, 18149, [143, 0, 189, 111, 43, 200, 171, 29]);
}
impl ::core::convert::From<ISpatialAudioObject> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObject) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObject> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObject) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volume: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectBase(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectBase {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsActive(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectBase {
    type Vtable = ISpatialAudioObjectBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437279474, 36173, 20219, [168, 207, 61, 110, 207, 28, 48, 224]);
}
impl ::core::convert::From<ISpatialAudioObjectBase> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectBase> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForHrtf(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectForHrtf {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsActive(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetGain(&self, gain: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(gain)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(orientation)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(environment)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(distancedecay)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(directivity)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectForHrtf {
    type Vtable = ISpatialAudioObjectForHrtf_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3611519710, 6520, 19988, [171, 160, 85, 91, 216, 235, 131, 180]);
}
impl ::core::convert::From<ISpatialAudioObjectForHrtf> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectForHrtf) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForHrtf> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectForHrtf) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForHrtf_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: f32, y: f32, z: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gain: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, orientation: *const *const f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForMetadataCommands(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectForMetadataCommands {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsActive(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectForMetadataCommands {
    type Vtable = ISpatialAudioObjectForMetadataCommands_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(234015051, 62969, 18221, [175, 107, 196, 110, 10, 201, 205, 5]);
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataCommands> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataCommands) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataCommands> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataCommands) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectForMetadataItems(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectForMetadataItems {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    pub unsafe fn IsActive(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::runtime::Result<AudioObjectType> {
        let mut result__: <AudioObjectType as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> ::windows::runtime::Result<ISpatialAudioMetadataItems> {
        let mut result__: <ISpatialAudioMetadataItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ISpatialAudioMetadataItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectForMetadataItems {
    type Vtable = ISpatialAudioObjectForMetadataItems_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3723119103, 15296, 17271, [138, 173, 159, 188, 253, 128, 133, 102]);
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataItems> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataItems) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataItems> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataItems) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectBase> for &ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, framecount: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isactive: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, audioobjecttype: *mut AudioObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, metadataitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStream(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectRenderStream {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> ::windows::runtime::Result<ISpatialAudioObject> {
        let mut result__: <ISpatialAudioObject as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectRenderStream {
    type Vtable = ISpatialAudioObjectRenderStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3132486771, 46115, 18299, [133, 245, 181, 163, 50, 160, 65, 83]);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStream> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStream> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamBase(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectRenderStreamBase {
    type Vtable = ISpatialAudioObjectRenderStreamBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4272616451, 49624, 17677, [170, 5, 224, 204, 238, 117, 2, 168]);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamBase> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamBase) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamBase> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamForHrtf(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectRenderStreamForHrtf {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> ::windows::runtime::Result<ISpatialAudioObjectForHrtf> {
        let mut result__: <ISpatialAudioObjectForHrtf as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForHrtf>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectRenderStreamForHrtf {
    type Vtable = ISpatialAudioObjectRenderStreamForHrtf_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3767398137, 21347, 16494, [159, 220, 8, 14, 226, 71, 187, 224]);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForHrtf> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForHrtf> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamForMetadata(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectRenderStreamForMetadata {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn GetService<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Start(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> ::windows::runtime::Result<ISpatialAudioObjectForMetadataCommands> {
        let mut result__: <ISpatialAudioObjectForMetadataCommands as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForMetadataCommands>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> ::windows::runtime::Result<ISpatialAudioObjectForMetadataItems> {
        let mut result__: <ISpatialAudioObjectForMetadataItems as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), &mut result__).from_abi::<ISpatialAudioObjectForMetadataItems>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectRenderStreamForMetadata {
    type Vtable = ISpatialAudioObjectRenderStreamForMetadata_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3150563591, 18645, 18990, [160, 199, 247, 240, 214, 124, 31, 177]);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForMetadata> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForMetadata> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: AudioObjectType, audioobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISpatialAudioObjectRenderStreamNotify(pub ::windows::runtime::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub unsafe fn OnAvailableDynamicObjectCountChange<'a, Param0: ::windows::runtime::IntoParam<'a, ISpatialAudioObjectRenderStreamBase>>(&self, sender: Param0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), sender.into_param().abi(), ::core::mem::transmute(hnscompliancedeadlinetime), ::core::mem::transmute(availabledynamicobjectcountchange)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialAudioObjectRenderStreamNotify {
    type Vtable = ISpatialAudioObjectRenderStreamNotify_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3722413030, 26839, 19568, [136, 63, 161, 131, 106, 251, 74, 80]);
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamNotify> for ::windows::runtime::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamNotify) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamNotify> for ::windows::runtime::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamNotify) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISubunit(pub ::windows::runtime::IUnknown);
impl ISubunit {}
unsafe impl ::windows::runtime::Interface for ISubunit {
    type Vtable = ISubunit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2182388357, 56230, 17543, [134, 187, 234, 143, 127, 239, 204, 113]);
}
impl ::core::convert::From<ISubunit> for ::windows::runtime::IUnknown {
    fn from(value: ISubunit) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISubunit> for ::windows::runtime::IUnknown {
    fn from(value: &ISubunit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISubunit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISubunit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Media_Multimedia`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Media_Multimedia`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_LONG: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_SHORT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_ISSTRM: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_STREAM: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_LASTERROR: u32 = 71u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NODEVICE: u32 = 68u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NOMAP: u32 = 66u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NOTREADY: u32 = 67u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIEVENT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIHDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIINCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIINCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIINCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIINCAPSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIOUTCAPSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPATCHSIZE: u32 = 128u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIPROPTEMPO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDIPROPTIMEDIV {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_GET: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_SET: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_TEMPO: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDISTRMBUFFVER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDISTRM_ERROR: i32 = -2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_QUERY: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_UNCACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIDI_WAVE_OPEN_TYPE {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCAPSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA {
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0 {
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0_0 {
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_0_1 {
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLA_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS {
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_BOOLEAN {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_LISTTEXTA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_LISTTEXTW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_SIGNED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLDETAILS_UNSIGNED {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_0_1 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERCONTROLW_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINEA {
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
unsafe impl ::windows::runtime::Abi for MIXERLINEA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSA {
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
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSA_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINECONTROLSW_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINEW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINEW_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for MIXERLINE_COMPONENTTYPE {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALLINE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALVALUE: u32 = 1026u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_LASTERROR: u32 = 1026u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
pub const MMDeviceEnumerator: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3168666517, 58671, 18044, [142, 61, 196, 87, 146, 145, 105, 46]);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_MAPPER: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_MIDIPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SQSYNTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SWSYNTH: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SYNTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_WAVETABLE: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub type PAudioStateMonitorCallback = unsafe extern "system" fn(audiostatemonitor: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for PCMWAVEFORMAT {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4054546445, 8208, 20179, [163, 166, 139, 135, 240, 240, 196, 118]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4054546445, 8208, 20179, [163, 166, 139, 135, 240, 240, 196, 118]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(337911810, 800, 19940, [149, 85, 167, 216, 43, 115, 194, 134]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(337911810, 800, 19940, [149, 85, 167, 216, 43, 115, 194, 134]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(497408003, 54418, 20189, [140, 35, 224, 192, 255, 238, 127, 14]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4053730893, 2092, 20007, [188, 115, 104, 130, 161, 187, 142, 76]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3834056230, 15557, 19666, [186, 70, 202, 10, 154, 112, 237, 4]),
    pid: 3u32,
};
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for PROCESS_LOOPBACK_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for PartType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS_ID: i32 = 1114112i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS_START: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_APPLICATION: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ASYNC: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_FILENAME: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_LOOP: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_MEMORY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NODEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NOSTOP: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NOWAIT: i32 = 8192i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_PURGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_RESOURCE: i32 = 262148i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_RING: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SENTRY: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SYNC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SYSTEM: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SPATIAL_AUDIO_STREAM_OPTIONS {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_DESTROYED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287232i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287227i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_INTERNAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287219i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287224i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287226i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287229i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287220i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287231i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287228i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287230i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287221i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287225i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004287222i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286956i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286969i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286968i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286940i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286942i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286976i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286955i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286941i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286967i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286952i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286974i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286957i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286939i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286959i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286951i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286971i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286973i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286954i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286944i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286960i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286958i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286943i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286970i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286953i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286975i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2004286972i32 as _);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: ::windows::runtime::GUID,
    pub appId: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for SpatialAudioClientActivationParams {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfActivationParams {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDirectivity {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDirectivityCardioid {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDirectivityCone {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDirectivityType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDirectivityUnion {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDistanceDecay {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfDistanceDecayType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioHrtfEnvironmentType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioMetadataCopyMode {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioMetadataItemsInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioMetadataWriterOverflowMode {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioObjectRenderStreamActivationParams {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for SpatialAudioObjectRenderStreamActivationParams2 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *mut WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for VOLUMEWAVEFILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_PITCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_SYNC: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEFILTER {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEFORMAT {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEFORMATEX {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::runtime::GUID,
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
unsafe impl ::windows::runtime::Abi for WAVEFORMATEXTENSIBLE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEFORMATEXTENSIBLE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEHDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEINCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEINCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEINCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEINCAPSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPS2A {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPS2W {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPSA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for WAVEOUTCAPSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_BADFORMAT: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_LASTERROR: u32 = 35u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_STILLPLAYING: u32 = 33u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_SYNC: u32 = 35u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_UNPREPARED: u32 = 34u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1M08: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1M16: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1S08: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1S16: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2M08: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2M16: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2S08: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2S16: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44M08: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44S08: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4M08: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4S08: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_PCM: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_MAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_BEGINLOOP: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_ENDLOOP: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_INQUEUE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for _AUDCLNT_BUFFERFLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(phadid: *mut isize, hinstmodule: Param1, lparam: Param2, dwpriority: u32, fdwadd: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverAddW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(phadid: *mut isize, hinstmodule: Param1, lparam: Param2, dwpriority: u32, fdwadd: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmDriverClose<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, fdwclose: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn acmDriverDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn acmDriverDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ::core::option::Option<ACMDRIVERENUMCB>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmDriverEnum(fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmDriverEnum(::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmDriverID<'a, Param0: ::windows::runtime::IntoParam<'a, HACMOBJ>>(hao: Param0, phadid: *mut isize, fdwdriverid: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(had: Param0, umsg: u32, lparam1: Param2, lparam2: Param3) -> super::super::Foundation::LRESULT {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmDriverOpen<'a, Param1: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(phad: *mut isize, hadid: Param1, fdwopen: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmDriverPriority<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, dwpriority: u32, fdwpriority: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmDriverRemove<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVERID>>(hadid: Param0, fdwremove: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmFilterDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSA, fncallback: ::core::option::Option<ACMFILTERENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterEnumA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSW, fncallback: ::core::option::Option<ACMFILTERENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterEnumW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmFilterTagDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ::core::option::Option<ACMFILTERTAGENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagEnumA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ::core::option::Option<ACMFILTERTAGENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFilterTagEnumW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmFormatDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFORMATDETAILSA, fncallback: ::core::option::Option<ACMFORMATENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatEnumA(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut tACMFORMATDETAILSW, fncallback: ::core::option::Option<ACMFORMATENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatEnumW(had.into_param().abi(), ::core::mem::transmute(pafd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmFormatSuggest<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmFormatTagDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumA<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ::core::option::Option<ACMFORMATTAGENUMCBA>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagEnumA(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumW<'a, Param0: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ::core::option::Option<ACMFORMATTAGENUMCBW>, dwinstance: usize, fdwenum: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ::windows::runtime::RawPtr, dwinstance: usize, fdwenum: u32) -> u32;
        }
        ::core::mem::transmute(acmFormatTagEnumW(had.into_param().abi(), ::core::mem::transmute(paftd), ::core::mem::transmute(fncallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwenum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmMetrics<'a, Param0: ::windows::runtime::IntoParam<'a, HACMOBJ>>(hao: Param0, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamClose<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, fdwclose: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamConvert<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmStreamMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(has: Param0, umsg: u32, lparam1: Param2, lparam2: Param3) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamOpen<'a, Param1: ::windows::runtime::IntoParam<'a, HACMDRIVER>>(phas: *mut isize, had: Param1, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamReset<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, fdwreset: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamSize<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn acmStreamUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HACMSTREAM>>(has: Param0, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiConnect<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDI>, Param1: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmi: Param0, hmo: Param1, preserved: *const ::core::ffi::c_void) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiDisconnect<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDI>, Param1: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmi: Param0, hmo: Param1, preserved: *const ::core::ffi::c_void) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInAddBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0, pudeviceid: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInReset<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInStart<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiInStop<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiInUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIIN>>(hmi: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutCacheDrumPatches<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, upatch: u32, pwkya: *const u16, fucache: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutCachePatches<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, ubank: u32, pwpa: *const u16, fucache: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pudeviceid: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutGetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pdwvolume: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutLongMsg<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *const MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutReset<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutSetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, dwvolume: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiOutShortMsg<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, dwmsg: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiOutUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDIOUT>>(hmo: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn midiStreamOut<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0, pmh: *mut MIDIHDR, cbmh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamPause<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamProperty<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0, lppropdata: *mut u8, dwproperty: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamRestart<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn midiStreamStop<'a, Param0: ::windows::runtime::IntoParam<'a, HMIDISTRM>>(hms: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn mixerClose<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXER>>(hmx: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetControlDetailsW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn mixerGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pumxid: *mut u32, fdwid: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineControlsA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn mixerGetLineControlsW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerGetLineInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn mixerGetLineInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn mixerMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXER>>(hmx: Param0, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn mixerSetControlDetails<'a, Param0: ::windows::runtime::IntoParam<'a, HMIXEROBJ>>(hmxobj: Param0, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for tACMDRVOPENDESCA {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
unsafe impl ::windows::runtime::Abi for tACMDRVOPENDESCW {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
unsafe impl ::windows::runtime::Abi for tACMFORMATDETAILSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInAddBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInClose<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, pudeviceid: *const u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInReset<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInStart<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveInStop<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveInUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEIN>>(hwi: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutBreakLoop<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutClose<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutGetID<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pudeviceid: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutGetPitch<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwpitch: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutGetPlaybackRate<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwrate: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutGetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutGetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pdwvolume: *mut u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutMessage<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, umsg: u32, dw1: usize, dw2: usize) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutPause<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutPrepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutReset<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutRestart<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutSetPitch<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwpitch: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutSetPlaybackRate<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwrate: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`*"]
#[inline]
pub unsafe fn waveOutSetVolume<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, dwvolume: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutUnprepareHeader<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn waveOutWrite<'a, Param0: ::windows::runtime::IntoParam<'a, HWAVEOUT>>(hwo: Param0, pwh: *mut WAVEHDR, cbwh: u32) -> u32 {
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
