#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDM_USER: u32 = 16384u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
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
impl ::core::marker::Copy for ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::clone::Clone for ACMDRIVERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::core::Abi for ACMDRIVERDETAILSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRIVERDETAILSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::cmp::Eq for ACMDRIVERDETAILSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
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
impl ::core::marker::Copy for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::clone::Clone for ACMDRIVERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
unsafe impl ::windows::core::Abi for ACMDRIVERDETAILSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::PartialEq for ACMDRIVERDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRIVERDETAILSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::cmp::Eq for ACMDRIVERDETAILSW {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::core::default::Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMDRIVERENUMCB = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVFORMATSUGGEST {
    pub cbStruct: u32,
    pub fdwSuggest: u32,
    pub pwfxSrc: *mut WAVEFORMATEX,
    pub cbwfxSrc: u32,
    pub pwfxDst: *mut WAVEFORMATEX,
    pub cbwfxDst: u32,
}
impl ::core::marker::Copy for ACMDRVFORMATSUGGEST {}
impl ::core::clone::Clone for ACMDRVFORMATSUGGEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMDRVFORMATSUGGEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMDRVFORMATSUGGEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRVFORMATSUGGEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMDRVFORMATSUGGEST {}
impl ::core::default::Default for ACMDRVFORMATSUGGEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for ACMDRVSTREAMHEADER {}
impl ::core::clone::Clone for ACMDRVSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMHEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRVSTREAMHEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMHEADER {}
impl ::core::default::Default for ACMDRVSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for ACMDRVSTREAMINSTANCE {}
impl ::core::clone::Clone for ACMDRVSTREAMINSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMINSTANCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMINSTANCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRVSTREAMINSTANCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMINSTANCE {}
impl ::core::default::Default for ACMDRVSTREAMINSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMDRVSTREAMSIZE {
    pub cbStruct: u32,
    pub fdwSize: u32,
    pub cbSrcLength: u32,
    pub cbDstLength: u32,
}
impl ::core::marker::Copy for ACMDRVSTREAMSIZE {}
impl ::core::clone::Clone for ACMDRVSTREAMSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMDRVSTREAMSIZE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMDRVSTREAMSIZE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMDRVSTREAMSIZE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMDRVSTREAMSIZE {}
impl ::core::default::Default for ACMDRVSTREAMSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BASE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_BUSY: u32 = 513u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_CANCELED: u32 = 515u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows::core::PCSTR,
    pub szFilterTag: [super::super::Foundation::CHAR; 48],
    pub szFilter: [super::super::Foundation::CHAR; 128],
    pub pszName: ::windows::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERCHOOSEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERCHOOSEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: ::windows::core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: ::windows::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: *mut WAVEFILTER,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERCHOOSEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERCHOOSEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERCHOOSEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERDETAILSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERDETAILSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: *mut WAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
impl ::core::marker::Copy for ACMFILTERDETAILSW {}
impl ::core::clone::Clone for ACMFILTERDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMFILTERDETAILSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMFILTERDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERDETAILSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMFILTERDETAILSW {}
impl ::core::default::Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFILTERTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFILTERTAGDETAILSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERTAGDETAILSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFILTERTAGDETAILSW {}
impl ::core::clone::Clone for ACMFILTERTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMFILTERTAGDETAILSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMFILTERTAGDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFILTERTAGDETAILSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMFILTERTAGDETAILSW {}
impl ::core::default::Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFILTERTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows::core::PCSTR,
    pub szFormatTag: [super::super::Foundation::CHAR; 48],
    pub szFormat: [super::super::Foundation::CHAR; 128],
    pub pszName: ::windows::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows::core::PCSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATCHOOSEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFORMATCHOOSEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATCHOOSEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCA = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATCHOOSEHOOKPROCW = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: ::windows::core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: ::windows::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: *mut WAVEFORMATEX,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub pszTemplateName: ::windows::core::PCWSTR,
    pub lCustData: super::super::Foundation::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATCHOOSEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATCHOOSEW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATCHOOSEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFORMATCHOOSEW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATCHOOSEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATDETAILSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFORMATDETAILSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut tACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACMFORMATTAGDETAILSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ACMFORMATTAGDETAILSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFORMATTAGDETAILSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl ::core::marker::Copy for ACMFORMATTAGDETAILSW {}
impl ::core::clone::Clone for ACMFORMATTAGDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ACMFORMATTAGDETAILSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ACMFORMATTAGDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMFORMATTAGDETAILSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ACMFORMATTAGDETAILSW {}
impl ::core::default::Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBA = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ACMFORMATTAGENUMCBW = ::core::option::Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELP: &'static str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPA: &'static str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTHELPW: &'static str = "acmchoose_contexthelp";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENU: &'static str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUA: &'static str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGCONTEXTMENUW: &'static str = "acmchoose_contextmenu";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRING: &'static str = "acmchoose_help";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGA: &'static str = "acmchoose_help";
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMHELPMSGSTRINGW: &'static str = "acmchoose_help";
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for ACMSTREAMHEADER {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMSTREAMHEADER>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[cfg(target_arch = "x86")]
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
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for ACMSTREAMHEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for ACMSTREAMHEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
unsafe impl ::windows::core::Abi for ACMSTREAMHEADER {
    type Abi = Self;
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for ACMSTREAMHEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ACMSTREAMHEADER>()) == 0 }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for ACMSTREAMHEADER {}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AMBISONICS_CHANNEL_ORDERING(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = AMBISONICS_CHANNEL_ORDERING(0i32);
impl ::core::marker::Copy for AMBISONICS_CHANNEL_ORDERING {}
impl ::core::clone::Clone for AMBISONICS_CHANNEL_ORDERING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_CHANNEL_ORDERING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_CHANNEL_ORDERING {
    type Abi = Self;
}
impl ::core::fmt::Debug for AMBISONICS_CHANNEL_ORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_CHANNEL_ORDERING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AMBISONICS_NORMALIZATION(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = AMBISONICS_NORMALIZATION(1i32);
impl ::core::marker::Copy for AMBISONICS_NORMALIZATION {}
impl ::core::clone::Clone for AMBISONICS_NORMALIZATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_NORMALIZATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_NORMALIZATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for AMBISONICS_NORMALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_NORMALIZATION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for AMBISONICS_PARAMS {}
impl ::core::clone::Clone for AMBISONICS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AMBISONICS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AMBISONICS_PARAMS").field("u32Size", &self.u32Size).field("u32Version", &self.u32Version).field("u32Type", &self.u32Type).field("u32ChannelOrdering", &self.u32ChannelOrdering).field("u32Normalization", &self.u32Normalization).field("u32Order", &self.u32Order).field("u32NumChannels", &self.u32NumChannels).field("pu32ChannelMap", &self.pu32ChannelMap).finish()
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AMBISONICS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AMBISONICS_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AMBISONICS_PARAMS {}
impl ::core::default::Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AMBISONICS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = AMBISONICS_TYPE(0i32);
impl ::core::marker::Copy for AMBISONICS_TYPE {}
impl ::core::clone::Clone for AMBISONICS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AMBISONICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AMBISONICS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AMBISONICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AMBISONICS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287486i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287469i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287464i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287477i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287466i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287463i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287482i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287465i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287484i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287478i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287423i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287422i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287473i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287454i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287447i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287448i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287471i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287468i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287474i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287470i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287440i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287424i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287467i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287456i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287479i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287455i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287451i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287487i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287483i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287452i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287453i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287481i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287449i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287450i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287472i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287476i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287480i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287485i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDCLNT_SHAREMODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_SHAREMODE_EXCLUSIVE: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(1i32);
impl ::core::marker::Copy for AUDCLNT_SHAREMODE {}
impl ::core::clone::Clone for AUDCLNT_SHAREMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_SHAREMODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDCLNT_SHAREMODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDCLNT_SHAREMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_SHAREMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDCLNT_STREAMOPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = AUDCLNT_STREAMOPTIONS(4u32);
impl ::core::marker::Copy for AUDCLNT_STREAMOPTIONS {}
impl ::core::clone::Clone for AUDCLNT_STREAMOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDCLNT_STREAMOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDCLNT_STREAMOPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDCLNT_STREAMOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDCLNT_STREAMOPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDCLNT_STREAMOPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDCLNT_STREAMOPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDCLNT_STREAMOPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(143196161i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_POSITION_STALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196163i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(143196162i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_ACTIVATION_PARAMS {
    pub ActivationType: AUDIOCLIENT_ACTIVATION_TYPE,
    pub Anonymous: AUDIOCLIENT_ACTIVATION_PARAMS_0,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIOCLIENT_ACTIVATION_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS {}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    pub ProcessLoopbackParams: AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS,
}
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIOCLIENT_ACTIVATION_PARAMS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_ACTIVATION_PARAMS_0 {}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_PARAMS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIOCLIENT_ACTIVATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_DEFAULT: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLIENT_ACTIVATION_TYPE_PROCESS_LOOPBACK: AUDIOCLIENT_ACTIVATION_TYPE = AUDIOCLIENT_ACTIVATION_TYPE(1i32);
impl ::core::marker::Copy for AUDIOCLIENT_ACTIVATION_TYPE {}
impl ::core::clone::Clone for AUDIOCLIENT_ACTIVATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIOCLIENT_ACTIVATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_ACTIVATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIOCLIENT_ACTIVATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIOCLIENT_ACTIVATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    pub TargetProcessId: u32,
    pub ProcessLoopbackMode: PROCESS_LOOPBACK_MODE,
}
impl ::core::marker::Copy for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::clone::Clone for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS").field("TargetProcessId", &self.TargetProcessId).field("ProcessLoopbackMode", &self.ProcessLoopbackMode).finish()
    }
}
unsafe impl ::windows::core::Abi for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {}
impl ::core::default::Default for AUDIOCLIENT_PROCESS_LOOPBACK_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_DUCKING_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = AUDIO_DUCKING_OPTIONS(1u32);
impl ::core::marker::Copy for AUDIO_DUCKING_OPTIONS {}
impl ::core::clone::Clone for AUDIO_DUCKING_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_DUCKING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_DUCKING_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_DUCKING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_DUCKING_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AUDIO_DUCKING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AUDIO_DUCKING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AUDIO_DUCKING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_EFFECT {
    pub id: ::windows::core::GUID,
    pub canSetState: super::super::Foundation::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_EFFECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_EFFECT").field("id", &self.id).field("canSetState", &self.canSetState).field("state", &self.state).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_EFFECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_EFFECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_EFFECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_EFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_EFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_EFFECT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = AUDIO_EFFECT_STATE(1i32);
impl ::core::marker::Copy for AUDIO_EFFECT_STATE {}
impl ::core::clone::Clone for AUDIO_EFFECT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_EFFECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_EFFECT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_EFFECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_EFFECT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUDIO_STREAM_CATEGORY(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Other: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_ForegroundOnlyMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Communications: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Alerts: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_SoundEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameEffects: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(6i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameMedia: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(7i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_GameChat: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Speech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(9i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Movie: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(10i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_Media: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(11i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_FarFieldSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(12i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_UniformSpeech: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(13i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioCategory_VoiceTyping: AUDIO_STREAM_CATEGORY = AUDIO_STREAM_CATEGORY(14i32);
impl ::core::marker::Copy for AUDIO_STREAM_CATEGORY {}
impl ::core::clone::Clone for AUDIO_STREAM_CATEGORY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUDIO_STREAM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUDIO_STREAM_CATEGORY {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUDIO_STREAM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUDIO_STREAM_CATEGORY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: ::windows::core::GUID,
    pub bMuted: super::super::Foundation::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIO_VOLUME_NOTIFICATION_DATA").field("guidEventContext", &self.guidEventContext).field("bMuted", &self.bMuted).field("fMasterVolume", &self.fMasterVolume).field("nChannels", &self.nChannels).field("afChannelVolumes", &self.afChannelVolumes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUDIO_VOLUME_NOTIFICATION_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUDIO_VOLUME_NOTIFICATION_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIO_VOLUME_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUXCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUXCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUXCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUXCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUXCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUXCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for AUXCAPS2W {}
impl ::core::clone::Clone for AUXCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUXCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUXCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUXCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUXCAPS2W {}
impl ::core::default::Default for AUXCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUXCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUXCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUXCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUXCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUXCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUXCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AUXCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub wTechnology: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
impl ::core::marker::Copy for AUXCAPSW {}
impl ::core::clone::Clone for AUXCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AUXCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AUXCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUXCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for AUXCAPSW {}
impl ::core::default::Default for AUXCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_AUXIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, IActivateAudioInterfaceCompletionHandler>>(deviceinterfacepath: Param0, riid: *const ::windows::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: Param3) -> ::windows::core::Result<IActivateAudioInterfaceAsyncOperation> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivateAudioInterfaceAsync(deviceinterfacepath: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: ::windows::core::RawPtr, activationoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        ActivateAudioInterfaceAsync(deviceinterfacepath.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(activationparams), completionhandler.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IActivateAudioInterfaceAsyncOperation>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: ::windows::core::GUID,
}
impl ::core::marker::Copy for AudioClient3ActivationParams {}
impl ::core::clone::Clone for AudioClient3ActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AudioClient3ActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClient3ActivationParams").field("tracingContextId", &self.tracingContextId).finish()
    }
}
unsafe impl ::windows::core::Abi for AudioClient3ActivationParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AudioClient3ActivationParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AudioClient3ActivationParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for AudioClient3ActivationParams {}
impl ::core::default::Default for AudioClient3ActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: super::super::Foundation::BOOL,
    pub eCategory: AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioClientProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioClientProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioClientProperties").field("cbSize", &self.cbSize).field("bIsOffload", &self.bIsOffload).field("eCategory", &self.eCategory).field("Options", &self.Options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AudioClientProperties {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AudioClientProperties {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AudioClientProperties>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AudioClientProperties {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AudioClientProperties {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AudioExtensionParams {
    pub AddPageParam: super::super::Foundation::LPARAM,
    pub pEndpoint: ::core::option::Option<IMMDevice>,
    pub pPnpInterface: ::core::option::Option<IMMDevice>,
    pub pPnpDevnode: ::core::option::Option<IMMDevice>,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AudioExtensionParams {
    fn clone(&self) -> Self {
        Self {
            AddPageParam: self.AddPageParam,
            pEndpoint: self.pEndpoint.clone(),
            pPnpInterface: self.pPnpInterface.clone(),
            pPnpDevnode: self.pPnpDevnode.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AudioExtensionParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AudioExtensionParams").field("AddPageParam", &self.AddPageParam).field("pEndpoint", &self.pEndpoint).field("pPnpInterface", &self.pPnpInterface).field("pPnpDevnode", &self.pPnpDevnode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AudioExtensionParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
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
impl ::core::default::Default for AudioExtensionParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioObjectType(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_None: AudioObjectType = AudioObjectType(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_Dynamic: AudioObjectType = AudioObjectType(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontLeft: AudioObjectType = AudioObjectType(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontRight: AudioObjectType = AudioObjectType(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_FrontCenter: AudioObjectType = AudioObjectType(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_LowFrequency: AudioObjectType = AudioObjectType(16u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideLeft: AudioObjectType = AudioObjectType(32u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_SideRight: AudioObjectType = AudioObjectType(64u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackLeft: AudioObjectType = AudioObjectType(128u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackRight: AudioObjectType = AudioObjectType(256u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontLeft: AudioObjectType = AudioObjectType(512u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopFrontRight: AudioObjectType = AudioObjectType(1024u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackLeft: AudioObjectType = AudioObjectType(2048u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_TopBackRight: AudioObjectType = AudioObjectType(4096u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = AudioObjectType(8192u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomFrontRight: AudioObjectType = AudioObjectType(16384u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackLeft: AudioObjectType = AudioObjectType(32768u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BottomBackRight: AudioObjectType = AudioObjectType(65536u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioObjectType_BackCenter: AudioObjectType = AudioObjectType(131072u32);
impl ::core::marker::Copy for AudioObjectType {}
impl ::core::clone::Clone for AudioObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioObjectType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioObjectType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioObjectType").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AudioObjectType {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AudioObjectType {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AudioObjectType {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AudioObjectType {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AudioObjectType {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioSessionDisconnectReason(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = AudioSessionDisconnectReason(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = AudioSessionDisconnectReason(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = AudioSessionDisconnectReason(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = AudioSessionDisconnectReason(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = AudioSessionDisconnectReason(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = AudioSessionDisconnectReason(5i32);
impl ::core::marker::Copy for AudioSessionDisconnectReason {}
impl ::core::clone::Clone for AudioSessionDisconnectReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionDisconnectReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioSessionDisconnectReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioSessionDisconnectReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionDisconnectReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioSessionState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateInactive: AudioSessionState = AudioSessionState(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateActive: AudioSessionState = AudioSessionState(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AudioSessionStateExpired: AudioSessionState = AudioSessionState(2i32);
impl ::core::marker::Copy for AudioSessionState {}
impl ::core::clone::Clone for AudioSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioSessionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioSessionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioSessionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AudioStateMonitorSoundLevel(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Muted: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Low: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Full: AudioStateMonitorSoundLevel = AudioStateMonitorSoundLevel(2i32);
impl ::core::marker::Copy for AudioStateMonitorSoundLevel {}
impl ::core::clone::Clone for AudioStateMonitorSoundLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioStateMonitorSoundLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AudioStateMonitorSoundLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AudioStateMonitorSoundLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStateMonitorSoundLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CoRegisterMessageFilter<'a, Param0: ::windows::core::IntoParam<'a, IMessageFilter>>(lpmessagefilter: Param0) -> ::windows::core::Result<IMessageFilter> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoRegisterMessageFilter(lpmessagefilter: ::windows::core::RawPtr, lplpmessagefilter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CoRegisterMessageFilter(lpmessagefilter.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMessageFilter>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConnectorType(pub i32);
impl ConnectorType {
    pub const Unknown_Connector: Self = Self(0i32);
    pub const Physical_Internal: Self = Self(1i32);
    pub const Physical_External: Self = Self(2i32);
    pub const Software_IO: Self = Self(3i32);
    pub const Software_Fixed: Self = Self(4i32);
    pub const Network: Self = Self(5i32);
}
impl ::core::marker::Copy for ConnectorType {}
impl ::core::clone::Clone for ConnectorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConnectorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConnectorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitor(::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategory(::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: ::windows::core::PCWSTR, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitor() -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateRenderAudioStateMonitor(::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategory(::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceId<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(category: AUDIO_STREAM_CATEGORY, deviceid: Param1) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: ::windows::core::PCWSTR, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceId(::core::mem::transmute(category), deviceid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole) -> ::windows::core::Result<IAudioStateMonitor> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        CreateRenderAudioStateMonitorForCategoryAndDeviceRole(::core::mem::transmute(category), ::core::mem::transmute(role), ::core::mem::transmute(&mut result__)).from_abi::<IAudioStateMonitor>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: ::windows::core::GUID,
    pub dwAudioStreamFlags: u32,
}
impl ::core::marker::Copy for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::clone::Clone for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRECTX_AUDIO_ACTIVATION_PARAMS").field("cbDirectXAudioActivationParams", &self.cbDirectXAudioActivationParams).field("guidAudioSession", &self.guidAudioSession).field("dwAudioStreamFlags", &self.dwAudioStreamFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DIRECTX_AUDIO_ACTIVATION_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DIRECTX_AUDIO_ACTIVATION_PARAMS {}
impl ::core::default::Default for DIRECTX_AUDIO_ACTIVATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DataFlow(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const In: DataFlow = DataFlow(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Out: DataFlow = DataFlow(1i32);
impl ::core::marker::Copy for DataFlow {}
impl ::core::clone::Clone for DataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataFlow {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DataFlow {
    type Abi = Self;
}
impl ::core::fmt::Debug for DataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataFlow").field(&self.0).finish()
    }
}
pub const DeviceTopology: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct ECHOWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
    pub dwDelay: u32,
}
impl ::core::marker::Copy for ECHOWAVEFILTER {}
impl ::core::clone::Clone for ECHOWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ECHOWAVEFILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ECHOWAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ECHOWAVEFILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ECHOWAVEFILTER {}
impl ::core::default::Default for ECHOWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EDataFlow(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eRender: EDataFlow = EDataFlow(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCapture: EDataFlow = EDataFlow(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eAll: EDataFlow = EDataFlow(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EDataFlow_enum_count: EDataFlow = EDataFlow(3i32);
impl ::core::marker::Copy for EDataFlow {}
impl ::core::clone::Clone for EDataFlow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDataFlow {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EDataFlow {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDataFlow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDataFlow").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ERole(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eConsole: ERole = ERole(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eMultimedia: ERole = ERole(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const eCommunications: ERole = ERole(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const ERole_enum_count: ERole = ERole(3i32);
impl ::core::marker::Copy for ERole {}
impl ::core::clone::Clone for ERole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ERole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ERole {
    type Abi = Self;
}
impl ::core::fmt::Debug for ERole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ERole").field(&self.0).finish()
    }
}
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EndpointFormFactor(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const RemoteNetworkDevice: EndpointFormFactor = EndpointFormFactor(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Speakers: EndpointFormFactor = EndpointFormFactor(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const LineLevel: EndpointFormFactor = EndpointFormFactor(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headphones: EndpointFormFactor = EndpointFormFactor(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Microphone: EndpointFormFactor = EndpointFormFactor(4i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Headset: EndpointFormFactor = EndpointFormFactor(5i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Handset: EndpointFormFactor = EndpointFormFactor(6i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownDigitalPassthrough: EndpointFormFactor = EndpointFormFactor(7i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPDIF: EndpointFormFactor = EndpointFormFactor(8i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const DigitalAudioDisplayDevice: EndpointFormFactor = EndpointFormFactor(9i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const UnknownFormFactor: EndpointFormFactor = EndpointFormFactor(10i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const EndpointFormFactor_enum_count: EndpointFormFactor = EndpointFormFactor(11i32);
impl ::core::marker::Copy for EndpointFormFactor {}
impl ::core::clone::Clone for EndpointFormFactor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EndpointFormFactor {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EndpointFormFactor {
    type Abi = Self;
}
impl ::core::fmt::Debug for EndpointFormFactor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointFormFactor").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACMDRIVER(pub isize);
impl HACMDRIVER {
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
impl ::core::default::Default for HACMDRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVER {}
impl ::core::fmt::Debug for HACMDRIVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVER").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HACMDRIVER {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACMDRIVERID(pub isize);
impl HACMDRIVERID {
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
impl ::core::default::Default for HACMDRIVERID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMDRIVERID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMDRIVERID {}
impl ::core::fmt::Debug for HACMDRIVERID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMDRIVERID").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HACMDRIVERID {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACMOBJ(pub isize);
impl HACMOBJ {
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
impl ::core::default::Default for HACMOBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMOBJ {}
impl ::core::fmt::Debug for HACMOBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMOBJ").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HACMOBJ {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HACMSTREAM(pub isize);
impl HACMSTREAM {
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
impl ::core::default::Default for HACMSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HACMSTREAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HACMSTREAM {}
impl ::core::fmt::Debug for HACMSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HACMSTREAM").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HACMSTREAM {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIDI(pub isize);
impl HMIDI {
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
impl ::core::default::Default for HMIDI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDI {}
impl ::core::fmt::Debug for HMIDI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIDI {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIDIIN(pub isize);
impl HMIDIIN {
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
impl ::core::default::Default for HMIDIIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIIN {}
impl ::core::fmt::Debug for HMIDIIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIIN").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIDIIN {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIDIOUT(pub isize);
impl HMIDIOUT {
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
impl ::core::default::Default for HMIDIOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDIOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDIOUT {}
impl ::core::fmt::Debug for HMIDIOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDIOUT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIDIOUT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIDISTRM(pub isize);
impl HMIDISTRM {
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
impl ::core::default::Default for HMIDISTRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIDISTRM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIDISTRM {}
impl ::core::fmt::Debug for HMIDISTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIDISTRM").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIDISTRM {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIXER(pub isize);
impl HMIXER {
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
impl ::core::default::Default for HMIXER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXER {}
impl ::core::fmt::Debug for HMIXER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXER").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIXER {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HMIXEROBJ(pub isize);
impl HMIXEROBJ {
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
impl ::core::default::Default for HMIXEROBJ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HMIXEROBJ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HMIXEROBJ {}
impl ::core::fmt::Debug for HMIXEROBJ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HMIXEROBJ").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HMIXEROBJ {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWAVE(pub isize);
impl HWAVE {
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
impl ::core::default::Default for HWAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVE {}
impl ::core::fmt::Debug for HWAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HWAVE {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWAVEIN(pub isize);
impl HWAVEIN {
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
impl ::core::default::Default for HWAVEIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEIN {}
impl ::core::fmt::Debug for HWAVEIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEIN").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HWAVEIN {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HWAVEOUT(pub isize);
impl HWAVEOUT {
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
impl ::core::default::Default for HWAVEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HWAVEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HWAVEOUT {}
impl ::core::fmt::Debug for HWAVEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HWAVEOUT").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HWAVEOUT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IActivateAudioInterfaceAsyncOperation(::windows::core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetActivateResult(&self, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActivateResult)(::core::mem::transmute_copy(self), ::core::mem::transmute(activateresult), ::core::mem::transmute(activatedinterface)).ok()
    }
}
impl ::core::convert::From<IActivateAudioInterfaceAsyncOperation> for ::windows::core::IUnknown {
    fn from(value: IActivateAudioInterfaceAsyncOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceAsyncOperation> for ::windows::core::IUnknown {
    fn from(value: &IActivateAudioInterfaceAsyncOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivateAudioInterfaceAsyncOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivateAudioInterfaceAsyncOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceAsyncOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceAsyncOperation {}
impl ::core::fmt::Debug for IActivateAudioInterfaceAsyncOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceAsyncOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceAsyncOperation {
    type Vtable = IActivateAudioInterfaceAsyncOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a22d78_cde4_431d_b8cc_843a71199b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetActivateResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IActivateAudioInterfaceCompletionHandler(::windows::core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateCompleted<'a, Param0: ::windows::core::IntoParam<'a, IActivateAudioInterfaceAsyncOperation>>(&self, activateoperation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateCompleted)(::core::mem::transmute_copy(self), activateoperation.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IActivateAudioInterfaceCompletionHandler> for ::windows::core::IUnknown {
    fn from(value: IActivateAudioInterfaceCompletionHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivateAudioInterfaceCompletionHandler> for ::windows::core::IUnknown {
    fn from(value: &IActivateAudioInterfaceCompletionHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivateAudioInterfaceCompletionHandler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IActivateAudioInterfaceCompletionHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivateAudioInterfaceCompletionHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivateAudioInterfaceCompletionHandler {}
impl ::core::fmt::Debug for IActivateAudioInterfaceCompletionHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivateAudioInterfaceCompletionHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IActivateAudioInterfaceCompletionHandler {
    type Vtable = IActivateAudioInterfaceCompletionHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d949ab_9862_444a_80f6_c261334da5eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ActivateCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activateoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioAmbisonicsControl(::windows::core::IUnknown);
impl IAudioAmbisonicsControl {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetData(&self, pambisonicsparams: &[AMBISONICS_PARAMS]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetData)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(pambisonicsparams)), pambisonicsparams.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHeadTracking<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benableheadtracking: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHeadTracking)(::core::mem::transmute_copy(self), benableheadtracking.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHeadTracking(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHeadTracking)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z), ::core::mem::transmute(w)).ok()
    }
}
impl ::core::convert::From<IAudioAmbisonicsControl> for ::windows::core::IUnknown {
    fn from(value: IAudioAmbisonicsControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioAmbisonicsControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioAmbisonicsControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioAmbisonicsControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioAmbisonicsControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioAmbisonicsControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAmbisonicsControl {}
impl ::core::fmt::Debug for IAudioAmbisonicsControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAmbisonicsControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioAmbisonicsControl {
    type Vtable = IAudioAmbisonicsControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28724c91_df35_4856_9f76_d6a26413f3df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHeadTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHeadTracking: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHeadTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHeadTracking: usize,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioAutoGainControl(::windows::core::IUnknown);
impl IAudioAutoGainControl {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioAutoGainControl> for ::windows::core::IUnknown {
    fn from(value: IAudioAutoGainControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioAutoGainControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioAutoGainControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioAutoGainControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioAutoGainControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioAutoGainControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioAutoGainControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioAutoGainControl {}
impl ::core::fmt::Debug for IAudioAutoGainControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioAutoGainControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioAutoGainControl {
    type Vtable = IAudioAutoGainControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioBass(::windows::core::IUnknown);
impl IAudioBass {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLevelRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelUniform)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelAllChannels)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(alevelsdb)), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioBass> for ::windows::core::IUnknown {
    fn from(value: IAudioBass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioBass> for ::windows::core::IUnknown {
    fn from(value: &IAudioBass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &'a IAudioBass {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioBass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioBass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioBass {}
impl ::core::fmt::Debug for IAudioBass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioBass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioBass {
    type Vtable = IAudioBass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_Vtbl {
    pub base: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioCaptureClient(::windows::core::IUnknown);
impl IAudioCaptureClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppdata), ::core::mem::transmute(pnumframestoread), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pu64deviceposition), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesread)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetNextPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextPacketSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioCaptureClient> for ::windows::core::IUnknown {
    fn from(value: IAudioCaptureClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioCaptureClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioCaptureClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioCaptureClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioCaptureClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioCaptureClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioCaptureClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioCaptureClient {}
impl ::core::fmt::Debug for IAudioCaptureClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioCaptureClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioCaptureClient {
    type Vtable = IAudioCaptureClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8adbd64_e71e_48a0_a4de_185c395cd317);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows::core::HRESULT,
    pub GetNextPacketSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioChannelConfig(::windows::core::IUnknown);
impl IAudioChannelConfig {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelConfig)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwconfig), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelConfig(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelConfig)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioChannelConfig> for ::windows::core::IUnknown {
    fn from(value: IAudioChannelConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioChannelConfig> for ::windows::core::IUnknown {
    fn from(value: &IAudioChannelConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioChannelConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioChannelConfig {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioChannelConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioChannelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioChannelConfig {}
impl ::core::fmt::Debug for IAudioChannelConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioChannelConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioChannelConfig {
    type Vtable = IAudioChannelConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb11c46f_ec28_493c_b88a_5db88062ce98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetChannelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChannelConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient(::windows::core::IUnknown);
impl IAudioClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBufferSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetStreamLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentPadding)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsFormatSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMixFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDevicePeriod)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventHandle)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetService)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
}
impl ::core::convert::From<IAudioClient> for ::windows::core::IUnknown {
    fn from(value: IAudioClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient {}
impl ::core::fmt::Debug for IAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient {
    type Vtable = IAudioClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT,
    pub GetStreamLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows::core::HRESULT,
    pub GetCurrentPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT,
    pub IsFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetMixFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
    pub GetDevicePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventHandle: usize,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient2(::windows::core::IUnknown);
impl IAudioClient2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetBufferSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetStreamLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetCurrentPadding)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsFormatSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMixFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetDevicePeriod)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEventHandle)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetService)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsOffloadCapable)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferSizeLimits)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
}
impl ::core::convert::From<IAudioClient2> for ::windows::core::IUnknown {
    fn from(value: IAudioClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient2> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for &'a IAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient2 {}
impl ::core::fmt::Debug for IAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient2 {
    type Vtable = IAudioClient2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x726778cd_f60a_4eda_82de_e47610cd78aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_Vtbl {
    pub base: IAudioClient_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOffloadCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOffloadCapable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBufferSizeLimits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBufferSizeLimits: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClient3(::windows::core::IUnknown);
impl IAudioClient3 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Initialize)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(streamflags), ::core::mem::transmute(hnsbufferduration), ::core::mem::transmute(hnsperiodicity), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBufferSize(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetBufferSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetStreamLatency(&self) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetStreamLatency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCurrentPadding(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetCurrentPadding)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.IsFormatSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharemode), ::core::mem::transmute(pformat), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.GetMixFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetDevicePeriod)(::core::mem::transmute_copy(self), ::core::mem::transmute(phnsdefaultdeviceperiod), ::core::mem::transmute(phnsminimumdeviceperiod)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, eventhandle: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetEventHandle)(::core::mem::transmute_copy(self), eventhandle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.GetService)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsOffloadCapable)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClientProperties)(::core::mem::transmute_copy(self), ::core::mem::transmute(pproperties)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBufferSizeLimits<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pformat: *const WAVEFORMATEX, beventdriven: Param1, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBufferSizeLimits)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), beventdriven.into_param().abi(), ::core::mem::transmute(phnsminbufferduration), ::core::mem::transmute(phnsmaxbufferduration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSharedModeEnginePeriod)(::core::mem::transmute_copy(self), ::core::mem::transmute(pformat), ::core::mem::transmute(pdefaultperiodinframes), ::core::mem::transmute(pfundamentalperiodinframes), ::core::mem::transmute(pminperiodinframes), ::core::mem::transmute(pmaxperiodinframes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentSharedModeEnginePeriod)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppformat), ::core::mem::transmute(pcurrentperiodinframes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeSharedAudioStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamflags), ::core::mem::transmute(periodinframes), ::core::mem::transmute(pformat), ::core::mem::transmute(audiosessionguid)).ok()
    }
}
impl ::core::convert::From<IAudioClient3> for ::windows::core::IUnknown {
    fn from(value: IAudioClient3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClient3> for ::windows::core::IUnknown {
    fn from(value: &IAudioClient3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
impl<'a> ::windows::core::IntoParam<'a, IAudioClient> for &'a IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioClient2> for &'a IAudioClient3 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioClient2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClient3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClient3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClient3 {}
impl ::core::fmt::Debug for IAudioClient3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClient3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClient3 {
    type Vtable = IAudioClient3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_Vtbl {
    pub base: IAudioClient2_Vtbl,
    pub GetSharedModeEnginePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT,
    pub InitializeSharedAudioStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClientDuckingControl(::windows::core::IUnknown);
impl IAudioClientDuckingControl {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuckingOptionsForCurrentStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(options)).ok()
    }
}
impl ::core::convert::From<IAudioClientDuckingControl> for ::windows::core::IUnknown {
    fn from(value: IAudioClientDuckingControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClientDuckingControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioClientDuckingControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClientDuckingControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClientDuckingControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClientDuckingControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClientDuckingControl {}
impl ::core::fmt::Debug for IAudioClientDuckingControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClientDuckingControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClientDuckingControl {
    type Vtable = IAudioClientDuckingControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc789d381_a28c_4168_b28f_d3a837924dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetDuckingOptionsForCurrentStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClock(::windows::core::IUnknown);
impl IAudioClock {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetFrequency(&self) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrequency)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(pu64position), ::core::mem::transmute(pu64qpcposition)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCharacteristics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IAudioClock> for ::windows::core::IUnknown {
    fn from(value: IAudioClock) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClock> for ::windows::core::IUnknown {
    fn from(value: &IAudioClock) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClock {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock {}
impl ::core::fmt::Debug for IAudioClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClock {
    type Vtable = IAudioClock_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd63314f_3fba_4a1b_812c_ef96358728e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows::core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClock2(::windows::core::IUnknown);
impl IAudioClock2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDevicePosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(deviceposition), ::core::mem::transmute(qpcposition)).ok()
    }
}
impl ::core::convert::From<IAudioClock2> for ::windows::core::IUnknown {
    fn from(value: IAudioClock2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClock2> for ::windows::core::IUnknown {
    fn from(value: &IAudioClock2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClock2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClock2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClock2 {}
impl ::core::fmt::Debug for IAudioClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClock2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClock2 {
    type Vtable = IAudioClock2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f49ff73_6727_49ac_a008_d98cf5e70048);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDevicePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioClockAdjustment(::windows::core::IUnknown);
impl IAudioClockAdjustment {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSampleRate)(::core::mem::transmute_copy(self), ::core::mem::transmute(flsamplerate)).ok()
    }
}
impl ::core::convert::From<IAudioClockAdjustment> for ::windows::core::IUnknown {
    fn from(value: IAudioClockAdjustment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioClockAdjustment> for ::windows::core::IUnknown {
    fn from(value: &IAudioClockAdjustment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioClockAdjustment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioClockAdjustment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioClockAdjustment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioClockAdjustment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioClockAdjustment {}
impl ::core::fmt::Debug for IAudioClockAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioClockAdjustment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioClockAdjustment {
    type Vtable = IAudioClockAdjustment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioEffectsChangedNotificationClient(::windows::core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnAudioEffectsChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAudioEffectsChanged)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IAudioEffectsChangedNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IAudioEffectsChangedNotificationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEffectsChangedNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioEffectsChangedNotificationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEffectsChangedNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEffectsChangedNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEffectsChangedNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsChangedNotificationClient {}
impl ::core::fmt::Debug for IAudioEffectsChangedNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsChangedNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsChangedNotificationClient {
    type Vtable = IAudioEffectsChangedNotificationClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnAudioEffectsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioEffectsManager(::windows::core::IUnknown);
impl IAudioEffectsManager {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterAudioEffectsChangedNotificationCallback)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IAudioEffectsChangedNotificationClient>>(&self, client: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterAudioEffectsChangedNotificationCallback)(::core::mem::transmute_copy(self), client.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAudioEffects)(::core::mem::transmute_copy(self), ::core::mem::transmute(effects), ::core::mem::transmute(numeffects)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetAudioEffectState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, effectid: Param0, state: AUDIO_EFFECT_STATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioEffectState)(::core::mem::transmute_copy(self), effectid.into_param().abi(), ::core::mem::transmute(state)).ok()
    }
}
impl ::core::convert::From<IAudioEffectsManager> for ::windows::core::IUnknown {
    fn from(value: IAudioEffectsManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioEffectsManager> for ::windows::core::IUnknown {
    fn from(value: &IAudioEffectsManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioEffectsManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioEffectsManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioEffectsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEffectsManager {}
impl ::core::fmt::Debug for IAudioEffectsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEffectsManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioEffectsManager {
    type Vtable = IAudioEffectsManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4460b3ae_4b44_4527_8676_7548a8acd260);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAudioEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAudioEffects: usize,
    pub SetAudioEffectState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioFormatEnumerator(::windows::core::IUnknown);
impl IAudioFormatEnumerator {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetFormat(&self, index: u32) -> ::windows::core::Result<*mut WAVEFORMATEX> {
        let mut result__: *mut WAVEFORMATEX = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFormat)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<*mut WAVEFORMATEX>(result__)
    }
}
impl ::core::convert::From<IAudioFormatEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAudioFormatEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioFormatEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAudioFormatEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioFormatEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioFormatEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioFormatEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioFormatEnumerator {}
impl ::core::fmt::Debug for IAudioFormatEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioFormatEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioFormatEnumerator {
    type Vtable = IAudioFormatEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcdaa858_895a_4a22_a5eb_67bda506096d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioInputSelector(::windows::core::IUnknown);
impl IAudioInputSelector {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioInputSelector> for ::windows::core::IUnknown {
    fn from(value: IAudioInputSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioInputSelector> for ::windows::core::IUnknown {
    fn from(value: &IAudioInputSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioInputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioInputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioInputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioInputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputSelector {}
impl ::core::fmt::Debug for IAudioInputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioInputSelector {
    type Vtable = IAudioInputSelector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f03dc02_5e6e_4653_8f72_a030c123d598);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioLoudness(::windows::core::IUnknown);
impl IAudioLoudness {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetEnabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benable: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), benable.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioLoudness> for ::windows::core::IUnknown {
    fn from(value: IAudioLoudness) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioLoudness> for ::windows::core::IUnknown {
    fn from(value: &IAudioLoudness) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioLoudness {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioLoudness {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioLoudness {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioLoudness {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioLoudness {}
impl ::core::fmt::Debug for IAudioLoudness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioLoudness").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioLoudness {
    type Vtable = IAudioLoudness_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioMidrange(::windows::core::IUnknown);
impl IAudioMidrange {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLevelRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelUniform)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelAllChannels)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(alevelsdb)), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioMidrange> for ::windows::core::IUnknown {
    fn from(value: IAudioMidrange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMidrange> for ::windows::core::IUnknown {
    fn from(value: &IAudioMidrange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &'a IAudioMidrange {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioMidrange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMidrange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMidrange {}
impl ::core::fmt::Debug for IAudioMidrange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMidrange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMidrange {
    type Vtable = IAudioMidrange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_Vtbl {
    pub base: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioMute(::windows::core::IUnknown);
impl IAudioMute {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmuted: Param0, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMute)(::core::mem::transmute_copy(self), bmuted.into_param().abi(), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMute)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IAudioMute> for ::windows::core::IUnknown {
    fn from(value: IAudioMute) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioMute> for ::windows::core::IUnknown {
    fn from(value: &IAudioMute) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioMute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioMute {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioMute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioMute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioMute {}
impl ::core::fmt::Debug for IAudioMute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioMute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioMute {
    type Vtable = IAudioMute_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioOutputSelector(::windows::core::IUnknown);
impl IAudioOutputSelector {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::core::mem::transmute_copy(self), ::core::mem::transmute(nidselect), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioOutputSelector> for ::windows::core::IUnknown {
    fn from(value: IAudioOutputSelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioOutputSelector> for ::windows::core::IUnknown {
    fn from(value: &IAudioOutputSelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioOutputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioOutputSelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioOutputSelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioOutputSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputSelector {}
impl ::core::fmt::Debug for IAudioOutputSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputSelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioOutputSelector {
    type Vtable = IAudioOutputSelector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioPeakMeter(::windows::core::IUnknown);
impl IAudioPeakMeter {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
}
impl ::core::convert::From<IAudioPeakMeter> for ::windows::core::IUnknown {
    fn from(value: IAudioPeakMeter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioPeakMeter> for ::windows::core::IUnknown {
    fn from(value: &IAudioPeakMeter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioPeakMeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioPeakMeter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioPeakMeter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioPeakMeter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioPeakMeter {}
impl ::core::fmt::Debug for IAudioPeakMeter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioPeakMeter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioPeakMeter {
    type Vtable = IAudioPeakMeter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioRenderClient(::windows::core::IUnknown);
impl IAudioRenderClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__: *mut u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframesrequested), ::core::mem::transmute(&mut result__)).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(numframeswritten), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IAudioRenderClient> for ::windows::core::IUnknown {
    fn from(value: IAudioRenderClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioRenderClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioRenderClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioRenderClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioRenderClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioRenderClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioRenderClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioRenderClient {}
impl ::core::fmt::Debug for IAudioRenderClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioRenderClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioRenderClient {
    type Vtable = IAudioRenderClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf294acfc_3146_4483_a7bf_addca7c260e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionControl(::windows::core::IUnknown);
impl IAudioSessionControl {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__: AudioSessionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioSessionState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIconPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIconPath)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGroupingParam)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGroupingParam)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterAudioSessionNotification)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterAudioSessionNotification)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSessionControl> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionControl> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl {}
impl ::core::fmt::Debug for IAudioSessionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl {
    type Vtable = IAudioSessionControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIconPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetGroupingParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetGroupingParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RegisterAudioSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterAudioSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionControl2(::windows::core::IUnknown);
impl IAudioSessionControl2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<AudioSessionState> {
        let mut result__: AudioSessionState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioSessionState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDisplayName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDisplayName)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetIconPath)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetIconPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, value: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetIconPath)(::core::mem::transmute_copy(self), value.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetGroupingParam)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetGroupingParam)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#override), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RegisterAudioSessionNotification)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterAudioSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionEvents>>(&self, newnotifications: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.UnregisterAudioSessionNotification)(::core::mem::transmute_copy(self), newnotifications.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSessionIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSessionIdentifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSessionInstanceIdentifier)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProcessId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsSystemSoundsSession(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSystemSoundsSession)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDuckingPreference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, optout: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDuckingPreference)(::core::mem::transmute_copy(self), optout.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSessionControl2> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionControl2> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionControl> for &'a IAudioSessionControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionControl> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionControl2 {}
impl ::core::fmt::Debug for IAudioSessionControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionControl2 {
    type Vtable = IAudioSessionControl2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_Vtbl {
    pub base: IAudioSessionControl_Vtbl,
    pub GetSessionIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSessionInstanceIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT,
    pub IsSystemSoundsSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDuckingPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDuckingPreference: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionEnumerator(::windows::core::IUnknown);
impl IAudioSessionEnumerator {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSession(&self, sessioncount: i32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSession)(::core::mem::transmute_copy(self), ::core::mem::transmute(sessioncount), ::core::mem::transmute(&mut result__)).from_abi::<IAudioSessionControl>(result__)
    }
}
impl ::core::convert::From<IAudioSessionEnumerator> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEnumerator {}
impl ::core::fmt::Debug for IAudioSessionEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEnumerator {
    type Vtable = IAudioSessionEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows::core::HRESULT,
    pub GetSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionEvents(::windows::core::IUnknown);
impl IAudioSessionEvents {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnDisplayNameChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, newdisplayname: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisplayNameChanged)(::core::mem::transmute_copy(self), newdisplayname.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnIconPathChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, newiconpath: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnIconPathChanged)(::core::mem::transmute_copy(self), newiconpath.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSimpleVolumeChanged<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, newvolume: f32, newmute: Param1, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSimpleVolumeChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newvolume), newmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnChannelVolumeChanged(&self, newchannelvolumearray: &[f32], changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChannelVolumeChanged)(::core::mem::transmute_copy(self), newchannelvolumearray.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(newchannelvolumearray)), ::core::mem::transmute(changedchannel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnGroupingParamChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newgroupingparam), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnStateChanged(&self, newstate: AudioSessionState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStateChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(newstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSessionDisconnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(disconnectreason)).ok()
    }
}
impl ::core::convert::From<IAudioSessionEvents> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionEvents> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionEvents {}
impl ::core::fmt::Debug for IAudioSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionEvents {
    type Vtable = IAudioSessionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnDisplayNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newdisplayname: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnIconPathChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newiconpath: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSimpleVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSimpleVolumeChanged: usize,
    pub OnChannelVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnGroupingParamChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows::core::HRESULT,
    pub OnSessionDisconnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionManager(::windows::core::IUnknown);
impl IAudioSessionManager {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAudioSessionControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), ::core::mem::transmute(&mut result__)).from_abi::<IAudioSessionControl>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSimpleAudioVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), ::core::mem::transmute(&mut result__)).from_abi::<ISimpleAudioVolume>(result__)
    }
}
impl ::core::convert::From<IAudioSessionManager> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionManager> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager {}
impl ::core::fmt::Debug for IAudioSessionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager {
    type Vtable = IAudioSessionManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAudioSessionControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSimpleAudioVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionManager2(::windows::core::IUnknown);
impl IAudioSessionManager2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAudioSessionControl)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), ::core::mem::transmute(&mut result__)).from_abi::<IAudioSessionControl>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSimpleAudioVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(audiosessionguid), ::core::mem::transmute(streamflags), ::core::mem::transmute(&mut result__)).from_abi::<ISimpleAudioVolume>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSessionEnumerator(&self) -> ::windows::core::Result<IAudioSessionEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSessionEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAudioSessionEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterSessionNotification)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterSessionNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionNotification>>(&self, sessionnotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterSessionNotification)(::core::mem::transmute_copy(self), sessionnotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, sessionid: Param0, ducknotification: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterDuckNotification)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ducknotification.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioVolumeDuckNotification>>(&self, ducknotification: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterDuckNotification)(::core::mem::transmute_copy(self), ducknotification.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSessionManager2> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionManager2> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IAudioSessionManager> for &'a IAudioSessionManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, IAudioSessionManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionManager2 {}
impl ::core::fmt::Debug for IAudioSessionManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionManager2 {
    type Vtable = IAudioSessionManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_Vtbl {
    pub base: IAudioSessionManager_Vtbl,
    pub GetSessionEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegisterSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterSessionNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegisterDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSessionNotification(::windows::core::IUnknown);
impl IAudioSessionNotification {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnSessionCreated<'a, Param0: ::windows::core::IntoParam<'a, IAudioSessionControl>>(&self, newsession: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSessionCreated)(::core::mem::transmute_copy(self), newsession.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSessionNotification> for ::windows::core::IUnknown {
    fn from(value: IAudioSessionNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSessionNotification> for ::windows::core::IUnknown {
    fn from(value: &IAudioSessionNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSessionNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSessionNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSessionNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSessionNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSessionNotification {}
impl ::core::fmt::Debug for IAudioSessionNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSessionNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSessionNotification {
    type Vtable = IAudioSessionNotification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641dd20b_4d41_49cc_aba3_174b9477bb08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnSessionCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioStateMonitor(::windows::core::IUnknown);
impl IAudioStateMonitor {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterCallback(&self, callback: PAudioStateMonitorCallback, context: *const ::core::ffi::c_void) -> ::windows::core::Result<i64> {
        let mut result__: i64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).RegisterCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(&mut result__)).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterCallback(&self, registration: i64) {
        (::windows::core::Interface::vtable(self).UnregisterCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(registration))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetSoundLevel)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IAudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: IAudioStateMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioStateMonitor> for ::windows::core::IUnknown {
    fn from(value: &IAudioStateMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioStateMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioStateMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStateMonitor {}
impl ::core::fmt::Debug for IAudioStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStateMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioStateMonitor {
    type Vtable = IAudioStateMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bd8738_e30d_4c77_bf5c_834e87c657e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStateMonitor_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT,
    pub UnregisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, registration: i64),
    pub GetSoundLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioStreamVolume(::windows::core::IUnknown);
impl IAudioStreamVolume {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllVolumes)(::core::mem::transmute_copy(self), pfvolumes.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfvolumes))).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllVolumes)(::core::mem::transmute_copy(self), pfvolumes.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pfvolumes))).ok()
    }
}
impl ::core::convert::From<IAudioStreamVolume> for ::windows::core::IUnknown {
    fn from(value: IAudioStreamVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioStreamVolume> for ::windows::core::IUnknown {
    fn from(value: &IAudioStreamVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioStreamVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioStreamVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioStreamVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioStreamVolume {}
impl ::core::fmt::Debug for IAudioStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioStreamVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioStreamVolume {
    type Vtable = IAudioStreamVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93014887_242d_4068_8a15_cf5e93b90fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient(::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyChanged<'a, Param1: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPropertyChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), key.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyChangeNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyChangeNotificationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffectsPropertyChangeNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyChangeNotificationClient {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyChangeNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyChangeNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyChangeNotificationClient {
    type Vtable = IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20049d40_56d5_400e_a2ef_385599feed49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyChanged: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioSystemEffectsPropertyStore(::windows::core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenDefaultPropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenUserPropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenVolatilePropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ResetUserPropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetUserPropertyStore)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ResetVolatilePropertyStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetVolatilePropertyStore)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterPropertyChangeNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterPropertyChangeNotification)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterPropertyChangeNotification<'a, Param0: ::windows::core::IntoParam<'a, IAudioSystemEffectsPropertyChangeNotificationClient>>(&self, callback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterPropertyChangeNotification)(::core::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioSystemEffectsPropertyStore> for ::windows::core::IUnknown {
    fn from(value: IAudioSystemEffectsPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioSystemEffectsPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &IAudioSystemEffectsPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioSystemEffectsPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioSystemEffectsPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioSystemEffectsPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSystemEffectsPropertyStore {}
impl ::core::fmt::Debug for IAudioSystemEffectsPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSystemEffectsPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioSystemEffectsPropertyStore {
    type Vtable = IAudioSystemEffectsPropertyStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenDefaultPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenDefaultPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenUserPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenUserPropertyStore: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OpenVolatilePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OpenVolatilePropertyStore: usize,
    pub ResetUserPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResetVolatilePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterPropertyChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterPropertyChangeNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioTreble(::windows::core::IUnknown);
impl IAudioTreble {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLevelRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelUniform)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelAllChannels)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(alevelsdb)), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioTreble> for ::windows::core::IUnknown {
    fn from(value: IAudioTreble) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioTreble> for ::windows::core::IUnknown {
    fn from(value: &IAudioTreble) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &'a IAudioTreble {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioTreble {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioTreble {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioTreble {}
impl ::core::fmt::Debug for IAudioTreble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioTreble").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioTreble {
    type Vtable = IAudioTreble_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a717812_694e_4907_b74b_bafa5cfdca7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_Vtbl {
    pub base: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioVolumeDuckNotification(::windows::core::IUnknown);
impl IAudioVolumeDuckNotification {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnVolumeDuckNotification<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, sessionid: Param0, countcommunicationsessions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnVolumeDuckNotification)(::core::mem::transmute_copy(self), sessionid.into_param().abi(), ::core::mem::transmute(countcommunicationsessions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnVolumeUnduckNotification<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, sessionid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnVolumeUnduckNotification)(::core::mem::transmute_copy(self), sessionid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IAudioVolumeDuckNotification> for ::windows::core::IUnknown {
    fn from(value: IAudioVolumeDuckNotification) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioVolumeDuckNotification> for ::windows::core::IUnknown {
    fn from(value: &IAudioVolumeDuckNotification) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioVolumeDuckNotification {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioVolumeDuckNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioVolumeDuckNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeDuckNotification {}
impl ::core::fmt::Debug for IAudioVolumeDuckNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeDuckNotification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeDuckNotification {
    type Vtable = IAudioVolumeDuckNotification_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnVolumeDuckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT,
    pub OnVolumeUnduckNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IAudioVolumeLevel(::windows::core::IUnknown);
impl IAudioVolumeLevel {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetLevelRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelUniform)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetLevelAllChannels)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(alevelsdb)), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IAudioVolumeLevel> for ::windows::core::IUnknown {
    fn from(value: IAudioVolumeLevel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAudioVolumeLevel> for ::windows::core::IUnknown {
    fn from(value: &IAudioVolumeLevel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPerChannelDbLevel> for &'a IAudioVolumeLevel {
    fn into_param(self) -> ::windows::core::Param<'a, IPerChannelDbLevel> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAudioVolumeLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAudioVolumeLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioVolumeLevel {}
impl ::core::fmt::Debug for IAudioVolumeLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioVolumeLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAudioVolumeLevel {
    type Vtable = IAudioVolumeLevel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_Vtbl {
    pub base: IPerChannelDbLevel_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IChannelAudioVolume(::windows::core::IUnknown);
impl IChannelAudioVolume {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannelVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32], eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllVolumes)(::core::mem::transmute_copy(self), pfvolumes.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pfvolumes)), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllVolumes)(::core::mem::transmute_copy(self), pfvolumes.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pfvolumes))).ok()
    }
}
impl ::core::convert::From<IChannelAudioVolume> for ::windows::core::IUnknown {
    fn from(value: IChannelAudioVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelAudioVolume> for ::windows::core::IUnknown {
    fn from(value: &IChannelAudioVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IChannelAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IChannelAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelAudioVolume {}
impl ::core::fmt::Debug for IChannelAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelAudioVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChannelAudioVolume {
    type Vtable = IChannelAudioVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c158861_b533_4b30_b1cf_e853e51c59b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IConnector(::windows::core::IUnknown);
impl IConnector {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<ConnectorType> {
        let mut result__: ConnectorType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ConnectorType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<DataFlow> {
        let mut result__: DataFlow = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataFlow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DataFlow>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ConnectTo<'a, Param0: ::windows::core::IntoParam<'a, IConnector>>(&self, pconnectto: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConnectTo)(::core::mem::transmute_copy(self), pconnectto.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsConnected)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetConnectedTo(&self) -> ::windows::core::Result<IConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectedTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectorIdConnectedTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceIdConnectedTo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IConnector> for ::windows::core::IUnknown {
    fn from(value: IConnector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConnector> for ::windows::core::IUnknown {
    fn from(value: &IConnector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IConnector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnector {}
impl ::core::fmt::Debug for IConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConnector {
    type Vtable = IConnector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2c4058_23f5_41de_877a_df3af236a09e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows::core::HRESULT,
    pub GetDataFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows::core::HRESULT,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnectto: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconto: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetConnectorIdConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDeviceIdConnectedTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IControlChangeNotify(::windows::core::IUnknown);
impl IControlChangeNotify {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNotify)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwsenderprocessid), ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IControlChangeNotify> for ::windows::core::IUnknown {
    fn from(value: IControlChangeNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlChangeNotify> for ::windows::core::IUnknown {
    fn from(value: &IControlChangeNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IControlChangeNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IControlChangeNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IControlChangeNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlChangeNotify {}
impl ::core::fmt::Debug for IControlChangeNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlChangeNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlChangeNotify {
    type Vtable = IControlChangeNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IControlInterface(::windows::core::IUnknown);
impl IControlInterface {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetIID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<IControlInterface> for ::windows::core::IUnknown {
    fn from(value: IControlInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IControlInterface> for ::windows::core::IUnknown {
    fn from(value: &IControlInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IControlInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IControlInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IControlInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IControlInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlInterface {}
impl ::core::fmt::Debug for IControlInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlInterface {
    type Vtable = IControlInterface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d37c3f_5140_444a_ae24_400789f3cbf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetIID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IDeviceSpecificProperty(::windows::core::IUnknown);
impl IDeviceSpecificProperty {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetType(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetValue(&self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetValue(&self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvvalue), ::core::mem::transmute(cbvalue), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Get4BRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmin), ::core::mem::transmute(plmax), ::core::mem::transmute(plstepping)).ok()
    }
}
impl ::core::convert::From<IDeviceSpecificProperty> for ::windows::core::IUnknown {
    fn from(value: IDeviceSpecificProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceSpecificProperty> for ::windows::core::IUnknown {
    fn from(value: &IDeviceSpecificProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceSpecificProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceSpecificProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceSpecificProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceSpecificProperty {}
impl ::core::fmt::Debug for IDeviceSpecificProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceSpecificProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeviceSpecificProperty {
    type Vtable = IDeviceSpecificProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b22bcbf_2586_4af0_8583_205d391b807c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Get4BRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IDeviceTopology(::windows::core::IUnknown);
impl IDeviceTopology {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetConnectorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnectorCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetConnector(&self, nindex: u32) -> ::windows::core::Result<IConnector> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetConnector)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IConnector>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSubunitCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubunitCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSubunit(&self, nindex: u32) -> ::windows::core::Result<ISubunit> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubunit)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<ISubunit>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetPartById(&self, nid: u32) -> ::windows::core::Result<IPart> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartById)(::core::mem::transmute_copy(self), ::core::mem::transmute(nid), ::core::mem::transmute(&mut result__)).from_abi::<IPart>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDeviceId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSignalPath<'a, Param0: ::windows::core::IntoParam<'a, IPart>, Param1: ::windows::core::IntoParam<'a, IPart>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pipartfrom: Param0, pipartto: Param1, brejectmixedpaths: Param2) -> ::windows::core::Result<IPartsList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSignalPath)(::core::mem::transmute_copy(self), pipartfrom.into_param().abi(), pipartto.into_param().abi(), brejectmixedpaths.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPartsList>(result__)
    }
}
impl ::core::convert::From<IDeviceTopology> for ::windows::core::IUnknown {
    fn from(value: IDeviceTopology) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDeviceTopology> for ::windows::core::IUnknown {
    fn from(value: &IDeviceTopology) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceTopology {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceTopology {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDeviceTopology {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDeviceTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceTopology {}
impl ::core::fmt::Debug for IDeviceTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceTopology").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDeviceTopology {
    type Vtable = IDeviceTopology_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a07407e_6497_4a18_9787_32f79bd0d98f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetConnectorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSubunitCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSubunit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetPartById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSignalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipartfrom: ::windows::core::RawPtr, pipartto: ::windows::core::RawPtr, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSignalPath: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDevice(::windows::core::IUnknown);
impl IMMDevice {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate(&self, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), ::core::mem::transmute(dwclsctx), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::StructuredStorage::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OpenPropertyStore)(::core::mem::transmute_copy(self), ::core::mem::transmute(stgmaccess), ::core::mem::transmute(&mut result__)).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetState(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IMMDevice> for ::windows::core::IUnknown {
    fn from(value: IMMDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMDevice> for ::windows::core::IUnknown {
    fn from(value: &IMMDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDevice {}
impl ::core::fmt::Debug for IMMDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDevice {
    type Vtable = IMMDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd666063f_1587_4e43_81f1_b948e807363f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Activate: usize,
    #[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub OpenPropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::StructuredStorage::STGM, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))]
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstrid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceActivator(::windows::core::IUnknown);
impl IMMDeviceActivator {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Activate<'a, Param1: ::windows::core::IntoParam<'a, IMMDevice>>(&self, iid: *const ::windows::core::GUID, pdevice: Param1, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), pdevice.into_param().abi(), ::core::mem::transmute(pactivationparams), ::core::mem::transmute(ppinterface)).ok()
    }
}
impl ::core::convert::From<IMMDeviceActivator> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceActivator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMDeviceActivator> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceActivator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceActivator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMDeviceActivator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMDeviceActivator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceActivator {}
impl ::core::fmt::Debug for IMMDeviceActivator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceActivator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceActivator {
    type Vtable = IMMDeviceActivator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, pdevice: ::windows::core::RawPtr, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Activate: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceCollection(::windows::core::IUnknown);
impl IMMDeviceCollection {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Item(&self, ndevice: u32) -> ::windows::core::Result<IMMDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(ndevice), ::core::mem::transmute(&mut result__)).from_abi::<IMMDevice>(result__)
    }
}
impl ::core::convert::From<IMMDeviceCollection> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMDeviceCollection> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMDeviceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMDeviceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceCollection {}
impl ::core::fmt::Debug for IMMDeviceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceCollection {
    type Vtable = IMMDeviceCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMDeviceEnumerator(::windows::core::IUnknown);
impl IMMDeviceEnumerator {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::core::Result<IMMDeviceCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumAudioEndpoints)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(dwstatemask), ::core::mem::transmute(&mut result__)).from_abi::<IMMDeviceCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> ::windows::core::Result<IMMDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultAudioEndpoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(dataflow), ::core::mem::transmute(role), ::core::mem::transmute(&mut result__)).from_abi::<IMMDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrid: Param0) -> ::windows::core::Result<IMMDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDevice)(::core::mem::transmute_copy(self), pwstrid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IMMDevice>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterEndpointNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterEndpointNotificationCallback)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterEndpointNotificationCallback<'a, Param0: ::windows::core::IntoParam<'a, IMMNotificationClient>>(&self, pclient: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterEndpointNotificationCallback)(::core::mem::transmute_copy(self), pclient.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMMDeviceEnumerator> for ::windows::core::IUnknown {
    fn from(value: IMMDeviceEnumerator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMDeviceEnumerator> for ::windows::core::IUnknown {
    fn from(value: &IMMDeviceEnumerator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMDeviceEnumerator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMDeviceEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMDeviceEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMDeviceEnumerator {}
impl ::core::fmt::Debug for IMMDeviceEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMDeviceEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMDeviceEnumerator {
    type Vtable = IMMDeviceEnumerator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95664d2_9614_4f35_a746_de8db63617e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnumAudioEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PCWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMEndpoint(::windows::core::IUnknown);
impl IMMEndpoint {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetDataFlow(&self) -> ::windows::core::Result<EDataFlow> {
        let mut result__: EDataFlow = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDataFlow)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<EDataFlow>(result__)
    }
}
impl ::core::convert::From<IMMEndpoint> for ::windows::core::IUnknown {
    fn from(value: IMMEndpoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMEndpoint> for ::windows::core::IUnknown {
    fn from(value: &IMMEndpoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMEndpoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMEndpoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMEndpoint {}
impl ::core::fmt::Debug for IMMEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMEndpoint {
    type Vtable = IMMEndpoint_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be09788_6894_4089_8586_9a2a6c265ac5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetDataFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMMNotificationClient(::windows::core::IUnknown);
impl IMMNotificationClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnDeviceStateChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrdeviceid: Param0, dwnewstate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDeviceStateChanged)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), ::core::mem::transmute(dwnewstate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnDeviceAdded<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDeviceAdded)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnDeviceRemoved<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, pwstrdeviceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDeviceRemoved)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnDefaultDeviceChanged<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDefaultDeviceChanged)(::core::mem::transmute_copy(self), ::core::mem::transmute(flow), ::core::mem::transmute(role), pwstrdefaultdeviceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn OnPropertyValueChanged<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>>(&self, pwstrdeviceid: Param0, key: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnPropertyValueChanged)(::core::mem::transmute_copy(self), pwstrdeviceid.into_param().abi(), key.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMMNotificationClient> for ::windows::core::IUnknown {
    fn from(value: IMMNotificationClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMMNotificationClient> for ::windows::core::IUnknown {
    fn from(value: &IMMNotificationClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMMNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMMNotificationClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMMNotificationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMMNotificationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMNotificationClient {}
impl ::core::fmt::Debug for IMMNotificationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMNotificationClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMNotificationClient {
    type Vtable = IMMNotificationClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7991eec9_7e89_4d85_8390_6c703cec60c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnDeviceStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, dwnewstate: u32) -> ::windows::core::HRESULT,
    pub OnDeviceAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnDeviceRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub OnDefaultDeviceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub OnPropertyValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    OnPropertyValueChanged: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IMessageFilter(::windows::core::IUnknown);
impl IMessageFilter {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HandleInComingCall<'a, Param1: ::windows::core::IntoParam<'a, super::HTASK>>(&self, dwcalltype: u32, htaskcaller: Param1, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).HandleInComingCall)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcalltype), htaskcaller.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(lpinterfaceinfo)))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RetryRejectedCall<'a, Param0: ::windows::core::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwrejecttype: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).RetryRejectedCall)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwrejecttype)))
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn MessagePending<'a, Param0: ::windows::core::IntoParam<'a, super::HTASK>>(&self, htaskcallee: Param0, dwtickcount: u32, dwpendingtype: u32) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).MessagePending)(::core::mem::transmute_copy(self), htaskcallee.into_param().abi(), ::core::mem::transmute(dwtickcount), ::core::mem::transmute(dwpendingtype)))
    }
}
impl ::core::convert::From<IMessageFilter> for ::windows::core::IUnknown {
    fn from(value: IMessageFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMessageFilter> for ::windows::core::IUnknown {
    fn from(value: &IMessageFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMessageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMessageFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMessageFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMessageFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageFilter {}
impl ::core::fmt::Debug for IMessageFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessageFilter {
    type Vtable = IMessageFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000016_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub HandleInComingCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32,
    #[cfg(not(feature = "Win32_System_Com"))]
    HandleInComingCall: usize,
    pub RetryRejectedCall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32,
    pub MessagePending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPart(::windows::core::IUnknown);
impl IPart {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLocalId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLocalId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetGlobalId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetGlobalId)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetPartType(&self) -> ::windows::core::Result<PartType> {
        let mut result__: PartType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPartType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<PartType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSubType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetControlInterfaceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetControlInterfaceCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> ::windows::core::Result<IControlInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetControlInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IControlInterface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EnumPartsIncoming(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumPartsIncoming)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPartsList>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EnumPartsOutgoing(&self) -> ::windows::core::Result<IPartsList> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EnumPartsOutgoing)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPartsList>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetTopologyObject(&self) -> ::windows::core::Result<IDeviceTopology> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTopologyObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDeviceTopology>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwclscontext), ::core::mem::transmute(refiid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn RegisterControlChangeCallback<'a, Param1: ::windows::core::IntoParam<'a, IControlChangeNotify>>(&self, riid: *const ::windows::core::GUID, pnotify: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterControlChangeCallback)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), pnotify.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn UnregisterControlChangeCallback<'a, Param0: ::windows::core::IntoParam<'a, IControlChangeNotify>>(&self, pnotify: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterControlChangeCallback)(::core::mem::transmute_copy(self), pnotify.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPart> for ::windows::core::IUnknown {
    fn from(value: IPart) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPart> for ::windows::core::IUnknown {
    fn from(value: &IPart) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPart {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPart {}
impl ::core::fmt::Debug for IPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPart").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPart {
    type Vtable = IPart_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlobalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPartType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows::core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetControlInterfaceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetControlInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumPartsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub EnumPartsOutgoing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetTopologyObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RegisterControlChangeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UnregisterControlChangeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPartsList(::windows::core::IUnknown);
impl IPartsList {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetPart(&self, nindex: u32) -> ::windows::core::Result<IPart> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPart)(::core::mem::transmute_copy(self), ::core::mem::transmute(nindex), ::core::mem::transmute(&mut result__)).from_abi::<IPart>(result__)
    }
}
impl ::core::convert::From<IPartsList> for ::windows::core::IUnknown {
    fn from(value: IPartsList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPartsList> for ::windows::core::IUnknown {
    fn from(value: &IPartsList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPartsList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPartsList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPartsList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPartsList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPartsList {}
impl ::core::fmt::Debug for IPartsList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPartsList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPartsList {
    type Vtable = IPartsList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct IPerChannelDbLevel(::windows::core::IUnknown);
impl IPerChannelDbLevel {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLevelRange)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(pfminleveldb), ::core::mem::transmute(pfmaxleveldb), ::core::mem::transmute(pfstepping)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevel)(::core::mem::transmute_copy(self), ::core::mem::transmute(nchannel), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevelUniform)(::core::mem::transmute_copy(self), ::core::mem::transmute(fleveldb), ::core::mem::transmute(pguideventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLevelAllChannels)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(alevelsdb)), alevelsdb.len() as _, ::core::mem::transmute(pguideventcontext)).ok()
    }
}
impl ::core::convert::From<IPerChannelDbLevel> for ::windows::core::IUnknown {
    fn from(value: IPerChannelDbLevel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPerChannelDbLevel> for ::windows::core::IUnknown {
    fn from(value: &IPerChannelDbLevel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPerChannelDbLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPerChannelDbLevel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPerChannelDbLevel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPerChannelDbLevel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPerChannelDbLevel {}
impl ::core::fmt::Debug for IPerChannelDbLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPerChannelDbLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPerChannelDbLevel {
    type Vtable = IPerChannelDbLevel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT,
    pub GetLevelRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetLevelUniform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetLevelAllChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISimpleAudioVolume(::windows::core::IUnknown);
impl ISimpleAudioVolume {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMasterVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(flevel), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMasterVolume(&self) -> ::windows::core::Result<f32> {
        let mut result__: f32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMasterVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bmute: Param0, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMute)(::core::mem::transmute_copy(self), bmute.into_param().abi(), ::core::mem::transmute(eventcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMute)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ISimpleAudioVolume> for ::windows::core::IUnknown {
    fn from(value: ISimpleAudioVolume) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISimpleAudioVolume> for ::windows::core::IUnknown {
    fn from(value: &ISimpleAudioVolume) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISimpleAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISimpleAudioVolume {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISimpleAudioVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISimpleAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleAudioVolume {}
impl ::core::fmt::Debug for ISimpleAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleAudioVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISimpleAudioVolume {
    type Vtable = ISimpleAudioVolume_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87ce5498_68d6_44e5_9215_6da47ef883d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetMasterVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMute: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioClient(::windows::core::IUnknown);
impl ISpatialAudioClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStaticObjectPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNativeStaticObjectTypeMask)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSupportedAudioObjectFormatEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAudioFormatEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxFrameCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsAudioObjectFormatSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSpatialAudioStreamAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::core::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ISpatialAudioClient> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioClient> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient {}
impl ::core::fmt::Debug for ISpatialAudioClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient {
    type Vtable = ISpatialAudioClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetStaticObjectPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT,
    pub GetNativeStaticObjectTypeMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows::core::HRESULT,
    pub GetMaxDynamicObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetSupportedAudioObjectFormatEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetMaxFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub IsAudioObjectFormatSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub IsSpatialAudioStreamAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    IsSpatialAudioStreamAvailable: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub ActivateSpatialAudioStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    ActivateSpatialAudioStream: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioClient2(::windows::core::IUnknown);
impl ISpatialAudioClient2 {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetStaticObjectPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetNativeStaticObjectTypeMask)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSupportedAudioObjectFormatEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IAudioFormatEnumerator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetMaxFrameCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsAudioObjectFormatSupported)(::core::mem::transmute_copy(self), ::core::mem::transmute(objectformat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.IsSpatialAudioStreamAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(streamuuid), ::core::mem::transmute(auxiliaryinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn ActivateSpatialAudioStream<T: ::windows::core::Interface>(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.ActivateSpatialAudioStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(activationparams), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsOffloadCapable)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMaxFrameCountForCategory<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: Param1, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxFrameCountForCategory)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), offloadenabled.into_param().abi(), ::core::mem::transmute(objectformat), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioClient2> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioClient2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioClient2> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioClient2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioClient> for &'a ISpatialAudioClient2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioClient> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioClient2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioClient2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioClient2 {}
impl ::core::fmt::Debug for ISpatialAudioClient2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioClient2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioClient2 {
    type Vtable = ISpatialAudioClient2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaabe452_a66a_4bee_a93e_e320463f6a53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_Vtbl {
    pub base: ISpatialAudioClient_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsOffloadCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsOffloadCapable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMaxFrameCountForCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMaxFrameCountForCategory: usize,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataClient(::windows::core::IUnknown);
impl ISpatialAudioMetadataClient {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), ::core::mem::transmute(framecount), ::core::mem::transmute(metadataitemsbuffer), ::core::mem::transmute(metadataitems)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSpatialAudioMetadataItemsBufferLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxitemcount), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::core::Result<ISpatialAudioMetadataWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataWriter)(::core::mem::transmute_copy(self), ::core::mem::transmute(overflowmode), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioMetadataWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioMetadataCopier(&self) -> ::windows::core::Result<ISpatialAudioMetadataCopier> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataCopier)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioMetadataCopier>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioMetadataReader(&self) -> ::windows::core::Result<ISpatialAudioMetadataReader> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioMetadataReader)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioMetadataReader>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioMetadataClient> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataClient) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataClient> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataClient) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataClient {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataClient {}
impl ::core::fmt::Debug for ISpatialAudioMetadataClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataClient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataClient {
    type Vtable = ISpatialAudioMetadataClient_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x777d4a3b_f6ff_4a26_85dc_68d7cdeda1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataClient_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ActivateSpatialAudioMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::windows::core::RawPtr, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSpatialAudioMetadataItemsBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataCopier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadatacopier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioMetadataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataCopier(::windows::core::IUnknown);
impl ISpatialAudioMetadataCopier {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn CopyMetadataForFrames<'a, Param2: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: Param2) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CopyMetadataForFrames)(::core::mem::transmute_copy(self), ::core::mem::transmute(copyframecount), ::core::mem::transmute(copymode), dstmetadataitems.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioMetadataCopier> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataCopier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataCopier> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataCopier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataCopier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataCopier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataCopier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataCopier {}
impl ::core::fmt::Debug for ISpatialAudioMetadataCopier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataCopier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataCopier {
    type Vtable = ISpatialAudioMetadataCopier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd224b233_e251_4fd0_9ca2_d5ecf9a68404);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataCopier_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CopyMetadataForFrames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::windows::core::RawPtr, itemscopied: *mut u16) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataItems(::windows::core::IUnknown);
impl ISpatialAudioMetadataItems {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetFrameCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetItemCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxItemCount(&self) -> ::windows::core::Result<u16> {
        let mut result__: u16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxItemCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetMaxValueBufferLength(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxValueBufferLength)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::core::Result<SpatialAudioMetadataItemsInfo> {
        let mut result__: SpatialAudioMetadataItemsInfo = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<SpatialAudioMetadataItemsInfo>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioMetadataItems> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItems> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItems {
    type Vtable = ISpatialAudioMetadataItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcd7c78f_3098_4f22_b547_a2f25a381269);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItems_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows::core::HRESULT,
    pub GetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetMaxValueBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataItemsBuffer(::windows::core::IUnknown);
impl ISpatialAudioMetadataItemsBuffer {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachToBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AttachToPopulatedBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn DetachBuffer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DetachBuffer)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioMetadataItemsBuffer> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataItemsBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataItemsBuffer> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataItemsBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataItemsBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataItemsBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataItemsBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataItemsBuffer {}
impl ::core::fmt::Debug for ISpatialAudioMetadataItemsBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataItemsBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataItemsBuffer {
    type Vtable = ISpatialAudioMetadataItemsBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42640a16_e1bd_42d9_9ff6_031ab71a2dba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AttachToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub AttachToPopulatedBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataReader(::windows::core::IUnknown);
impl ISpatialAudioMetadataReader {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadNextItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandcount), ::core::mem::transmute(frameoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReadNextItemCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(maxvaluebufferlength), ::core::mem::transmute(valuebufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioMetadataReader> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataReader> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataReader {}
impl ::core::fmt::Debug for ISpatialAudioMetadataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataReader {
    type Vtable = ISpatialAudioMetadataReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb78e86a2_31d9_4c32_94d2_7df40fc7ebec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataReader_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ReadNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT,
    pub ReadNextItemCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioMetadataWriter(::windows::core::IUnknown);
impl ISpatialAudioMetadataWriter {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioMetadataItems>>(&self, metadataitems: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::core::mem::transmute_copy(self), metadataitems.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn WriteNextItem(&self, frameoffset: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextItem)(::core::mem::transmute_copy(self), ::core::mem::transmute(frameoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextItemCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioMetadataWriter {}
impl ::core::fmt::Debug for ISpatialAudioMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioMetadataWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioMetadataWriter {
    type Vtable = ISpatialAudioMetadataWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b17ca01_2955_444d_a430_537dc589a844);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioMetadataWriter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub WriteNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows::core::HRESULT,
    pub WriteNextItemCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObject(::windows::core::IUnknown);
impl ISpatialAudioObject {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEndOfStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsActive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAudioObjectType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetVolume(&self, volume: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::core::mem::transmute_copy(self), ::core::mem::transmute(volume)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioObject> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObject> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &'a ISpatialAudioObject {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObject {}
impl ::core::fmt::Debug for ISpatialAudioObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObject {
    type Vtable = ISpatialAudioObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_Vtbl {
    pub base: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectBase(::windows::core::IUnknown);
impl ISpatialAudioObjectBase {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEndOfStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsActive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAudioObjectType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioObjectBase> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectBase> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectBase {
    type Vtable = ISpatialAudioObjectBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsActive: usize,
    pub GetAudioObjectType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForHrtf(::windows::core::IUnknown);
impl ISpatialAudioObjectForHrtf {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEndOfStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsActive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAudioObjectType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPosition)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(z)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetGain(&self, gain: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGain)(::core::mem::transmute_copy(self), ::core::mem::transmute(gain)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetOrientation(&self, orientation: *const *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOrientation)(::core::mem::transmute_copy(self), ::core::mem::transmute(orientation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnvironment)(::core::mem::transmute_copy(self), ::core::mem::transmute(environment)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDistanceDecay)(::core::mem::transmute_copy(self), ::core::mem::transmute(distancedecay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDirectivity)(::core::mem::transmute_copy(self), ::core::mem::transmute(directivity)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioObjectForHrtf> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForHrtf) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForHrtf> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForHrtf) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &'a ISpatialAudioObjectForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForHrtf").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForHrtf {
    type Vtable = ISpatialAudioObjectForHrtf_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7436ade_1978_4e14_aba0_555bd8eb83b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForHrtf_Vtbl {
    pub base: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT,
    pub SetGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows::core::HRESULT,
    pub SetEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT,
    pub SetDistanceDecay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT,
    pub SetDirectivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataCommands(::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataCommands {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEndOfStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsActive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAudioObjectType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteNextMetadataCommand)(::core::mem::transmute_copy(self), ::core::mem::transmute(commandid), ::core::mem::transmute(valuebuffer), ::core::mem::transmute(valuebufferlength)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataCommands> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataCommands) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataCommands> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataCommands) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &'a ISpatialAudioObjectForMetadataCommands {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataCommands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataCommands {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataCommands").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataCommands {
    type Vtable = ISpatialAudioObjectForMetadataCommands_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0df2c94b_f5f9_472d_af6b_c46e0ac9cd05);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub base: ISpatialAudioObjectBase_Vtbl,
    pub WriteNextMetadataCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectForMetadataItems(::windows::core::IUnknown);
impl ISpatialAudioObjectForMetadataItems {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.GetBuffer)(::core::mem::transmute_copy(self), ::core::mem::transmute(buffer), ::core::mem::transmute(bufferlength)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEndOfStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(framecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsActive)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType> {
        let mut result__: AudioObjectType = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAudioObjectType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<AudioObjectType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetSpatialAudioMetadataItems(&self) -> ::windows::core::Result<ISpatialAudioMetadataItems> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSpatialAudioMetadataItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioMetadataItems>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioObjectForMetadataItems> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectForMetadataItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectForMetadataItems> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectForMetadataItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectBase> for &'a ISpatialAudioObjectForMetadataItems {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectForMetadataItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectForMetadataItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectForMetadataItems {}
impl ::core::fmt::Debug for ISpatialAudioObjectForMetadataItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectForMetadataItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectForMetadataItems {
    type Vtable = ISpatialAudioObjectForMetadataItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddea49ff_3bc0_4377_8aad_9fbcfd808566);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectForMetadataItems_Vtbl {
    pub base: ISpatialAudioObjectBase_Vtbl,
    pub GetSpatialAudioMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStream(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStream {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAvailableDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetService)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginUpdatingAudioObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EndUpdatingAudioObjects)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObject)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioObject>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStream> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStream> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &'a ISpatialAudioObjectRenderStream {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStream {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStream {
    type Vtable = ISpatialAudioObjectRenderStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab5f473_b423_477b_85f5_b5a332a04153);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_Vtbl {
    pub base: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamBase(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailableDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetService)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUpdatingAudioObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndUpdatingAudioObjects)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamBase> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamBase> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamBase {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamBase {
    type Vtable = ISpatialAudioObjectRenderStreamBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetAvailableDynamicObjectCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT,
    pub GetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BeginUpdatingAudioObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT,
    pub EndUpdatingAudioObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForHrtf(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForHrtf {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAvailableDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetService)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginUpdatingAudioObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EndUpdatingAudioObjects)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForHrtf> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForHrtf)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioObjectForHrtf>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForHrtf> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForHrtf> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForHrtf) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &'a ISpatialAudioObjectRenderStreamForHrtf {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForHrtf {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForHrtf {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForHrtf {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForHrtf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForHrtf").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForHrtf {
    type Vtable = ISpatialAudioObjectRenderStreamForHrtf_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe08deef9_5363_406e_9fdc_080ee247bbe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub base: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForHrtf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamForMetadata(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamForMetadata {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAvailableDynamicObjectCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn GetService<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base.GetService)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Start)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Stop)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginUpdatingAudioObjects)(::core::mem::transmute_copy(self), ::core::mem::transmute(availabledynamicobjectcount), ::core::mem::transmute(framecountperbuffer)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EndUpdatingAudioObjects)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataCommands> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataCommands)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioObjectForMetadataCommands>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataItems> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).ActivateSpatialAudioObjectForMetadataItems)(::core::mem::transmute_copy(self), ::core::mem::transmute(r#type), ::core::mem::transmute(&mut result__)).from_abi::<ISpatialAudioObjectForMetadataItems>(result__)
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamForMetadata> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamForMetadata> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamForMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase> for &'a ISpatialAudioObjectRenderStreamForMetadata {
    fn into_param(self) -> ::windows::core::Param<'a, ISpatialAudioObjectRenderStreamBase> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamForMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamForMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamForMetadata {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamForMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamForMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamForMetadata {
    type Vtable = ISpatialAudioObjectRenderStreamForMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbc9c907_48d5_4a2e_a0c7_f7f0d67c1fb1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub base: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObjectForMetadataCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ActivateSpatialAudioObjectForMetadataItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISpatialAudioObjectRenderStreamNotify(::windows::core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    pub unsafe fn OnAvailableDynamicObjectCountChange<'a, Param0: ::windows::core::IntoParam<'a, ISpatialAudioObjectRenderStreamBase>>(&self, sender: Param0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAvailableDynamicObjectCountChange)(::core::mem::transmute_copy(self), sender.into_param().abi(), ::core::mem::transmute(hnscompliancedeadlinetime), ::core::mem::transmute(availabledynamicobjectcountchange)).ok()
    }
}
impl ::core::convert::From<ISpatialAudioObjectRenderStreamNotify> for ::windows::core::IUnknown {
    fn from(value: ISpatialAudioObjectRenderStreamNotify) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpatialAudioObjectRenderStreamNotify> for ::windows::core::IUnknown {
    fn from(value: &ISpatialAudioObjectRenderStreamNotify) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpatialAudioObjectRenderStreamNotify {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpatialAudioObjectRenderStreamNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpatialAudioObjectRenderStreamNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpatialAudioObjectRenderStreamNotify {}
impl ::core::fmt::Debug for ISpatialAudioObjectRenderStreamNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpatialAudioObjectRenderStreamNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpatialAudioObjectRenderStreamNotify {
    type Vtable = ISpatialAudioObjectRenderStreamNotify_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub OnAvailableDynamicObjectCountChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
pub struct ISubunit(::windows::core::IUnknown);
impl ISubunit {}
impl ::core::convert::From<ISubunit> for ::windows::core::IUnknown {
    fn from(value: ISubunit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISubunit> for ::windows::core::IUnknown {
    fn from(value: &ISubunit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISubunit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISubunit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISubunit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISubunit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubunit {}
impl ::core::fmt::Debug for ISubunit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubunit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISubunit {
    type Vtable = ISubunit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPACMDRIVERPROC = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::super::Foundation::LPARAM, param4: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPMIDICALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPWAVECALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: super::Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_LONG: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MEVT_F_SHORT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_ISSTRM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_CACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_STREAM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDICAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_LASTERROR: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NODEVICE: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOMAP: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_NOTREADY: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIEVENT {
    pub dwDeltaTime: u32,
    pub dwStreamID: u32,
    pub dwEvent: u32,
    pub dwParms: [u32; 1],
}
impl ::core::marker::Copy for MIDIEVENT {}
impl ::core::clone::Clone for MIDIEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIEVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIEVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIEVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIEVENT {}
impl ::core::default::Default for MIDIEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIHDR {
    pub lpData: ::windows::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub lpNext: *mut MIDIHDR,
    pub reserved: usize,
    pub dwOffset: u32,
    pub dwReserved: [usize; 8],
}
impl ::core::marker::Copy for MIDIHDR {}
impl ::core::clone::Clone for MIDIHDR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIHDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIHDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIHDR {}
impl ::core::default::Default for MIDIHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIINCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIINCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIINCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIDIINCAPS2W {}
impl ::core::clone::Clone for MIDIINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIINCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIINCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIINCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIINCAPS2W {}
impl ::core::default::Default for MIDIINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIDIINCAPSA {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [super::super::Foundation::CHAR; 32],
    pub dwSupport: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIINCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIINCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIINCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwSupport: u32,
}
impl ::core::marker::Copy for MIDIINCAPSW {}
impl ::core::clone::Clone for MIDIINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIINCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIINCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIINCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIINCAPSW {}
impl ::core::default::Default for MIDIINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIOUTCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIOUTCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOUTCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIDIOUTCAPS2W {}
impl ::core::clone::Clone for MIDIOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIOUTCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIOUTCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOUTCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPS2W {}
impl ::core::default::Default for MIDIOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIDIOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIDIOUTCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIDIOUTCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOUTCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIDIOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIDIOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIDIOUTCAPSW {}
impl ::core::clone::Clone for MIDIOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIOUTCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIOUTCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIOUTCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIOUTCAPSW {}
impl ::core::default::Default for MIDIOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPATCHSIZE: u32 = 128u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTEMPO {
    pub cbStruct: u32,
    pub dwTempo: u32,
}
impl ::core::marker::Copy for MIDIPROPTEMPO {}
impl ::core::clone::Clone for MIDIPROPTEMPO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIPROPTEMPO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIPROPTEMPO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIPROPTEMPO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIPROPTEMPO {}
impl ::core::default::Default for MIDIPROPTEMPO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDIPROPTIMEDIV {
    pub cbStruct: u32,
    pub dwTimeDiv: u32,
}
impl ::core::marker::Copy for MIDIPROPTIMEDIV {}
impl ::core::clone::Clone for MIDIPROPTIMEDIV {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDIPROPTIMEDIV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDIPROPTIMEDIV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDIPROPTIMEDIV>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDIPROPTIMEDIV {}
impl ::core::default::Default for MIDIPROPTIMEDIV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_GET: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_SET: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TEMPO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIDISTRMBUFFVER {
    pub dwVersion: u32,
    pub dwMid: u32,
    pub dwOEMVersion: u32,
}
impl ::core::marker::Copy for MIDISTRMBUFFVER {}
impl ::core::clone::Clone for MIDISTRMBUFFVER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIDISTRMBUFFVER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDISTRMBUFFVER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDISTRMBUFFVER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDISTRMBUFFVER {}
impl ::core::default::Default for MIDISTRMBUFFVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDISTRM_ERROR: i32 = -2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_CACHE_QUERY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_UNCACHE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIDI_WAVE_OPEN_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TYPEMASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(458752u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_NULL: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_WINDOW: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(65536u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_TASK: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_FUNCTION: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(196608u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_THREAD: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(131072u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const CALLBACK_EVENT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(327680u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_ALLOWSYNC: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_DIRECT_QUERY: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(9u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPED_DEFAULT_COMMUNICATION_DEVICE: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(16u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIDI_IO_STATUS: MIDI_WAVE_OPEN_TYPE = MIDI_WAVE_OPEN_TYPE(32u32);
impl ::core::marker::Copy for MIDI_WAVE_OPEN_TYPE {}
impl ::core::clone::Clone for MIDI_WAVE_OPEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDI_WAVE_OPEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIDI_WAVE_OPEN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIDI_WAVE_OPEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDI_WAVE_OPEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MIDI_WAVE_OPEN_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MIDI_WAVE_OPEN_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIXERCAPS2W {}
impl ::core::clone::Clone for MIXERCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCAPS2W {}
impl ::core::default::Default for MIXERCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub fdwSupport: u32,
    pub cDestinations: u32,
}
impl ::core::marker::Copy for MIXERCAPSW {}
impl ::core::clone::Clone for MIXERCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCAPSW {}
impl ::core::default::Default for MIXERCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_0 {
    pub Anonymous1: MIXERCONTROLA_0_0,
    pub Anonymous2: MIXERCONTROLA_0_1,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLA_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLA_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLA_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLA_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLA_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLA_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLA_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLA_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERCONTROLDETAILS_0 {
    pub hwndOwner: super::super::Foundation::HWND,
    pub cMultipleItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_BOOLEAN {
    pub fValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_BOOLEAN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_BOOLEAN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_BOOLEAN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_BOOLEAN>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_BOOLEAN {}
impl ::core::default::Default for MIXERCONTROLDETAILS_BOOLEAN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MIXERCONTROLDETAILS_LISTTEXTA {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_LISTTEXTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_LISTTEXTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_LISTTEXTW {
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub szName: [u16; 64],
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_LISTTEXTW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_LISTTEXTW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_LISTTEXTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_LISTTEXTW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_LISTTEXTW {}
impl ::core::default::Default for MIXERCONTROLDETAILS_LISTTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_SIGNED {
    pub lValue: i32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_SIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_SIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_SIGNED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_SIGNED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_SIGNED>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_SIGNED {}
impl ::core::default::Default for MIXERCONTROLDETAILS_SIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLDETAILS_UNSIGNED {
    pub dwValue: u32,
}
impl ::core::marker::Copy for MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::clone::Clone for MIXERCONTROLDETAILS_UNSIGNED {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLDETAILS_UNSIGNED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLDETAILS_UNSIGNED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLDETAILS_UNSIGNED>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLDETAILS_UNSIGNED {}
impl ::core::default::Default for MIXERCONTROLDETAILS_UNSIGNED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIXERCONTROLW {}
impl ::core::clone::Clone for MIXERCONTROLW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW {}
impl ::core::default::Default for MIXERCONTROLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_0 {
    pub Anonymous1: MIXERCONTROLW_0_0,
    pub Anonymous2: MIXERCONTROLW_0_1,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0 {}
impl ::core::default::Default for MIXERCONTROLW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_0 {
    pub lMinimum: i32,
    pub lMaximum: i32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_0 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLW_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_0 {}
impl ::core::default::Default for MIXERCONTROLW_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERCONTROLW_0_1 {
    pub dwMinimum: u32,
    pub dwMaximum: u32,
}
impl ::core::marker::Copy for MIXERCONTROLW_0_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLW_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_0_1 {}
impl ::core::default::Default for MIXERCONTROLW_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERCONTROLW_1 {
    pub cSteps: u32,
    pub cbCustomData: u32,
    pub dwReserved: [u32; 6],
}
impl ::core::marker::Copy for MIXERCONTROLW_1 {}
impl ::core::clone::Clone for MIXERCONTROLW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERCONTROLW_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERCONTROLW_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERCONTROLW_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERCONTROLW_1 {}
impl ::core::default::Default for MIXERCONTROLW_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINEA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINEA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINEA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINEA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINEA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINEA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINECONTROLSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINECONTROLSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINECONTROLSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINECONTROLSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MIXERLINECONTROLSA_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MIXERLINECONTROLSA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MIXERLINECONTROLSA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSA_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MIXERLINECONTROLSA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINECONTROLSA_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MIXERLINECONTROLSA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MIXERLINECONTROLSA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINECONTROLSW {
    pub cbStruct: u32,
    pub dwLineID: u32,
    pub Anonymous: MIXERLINECONTROLSW_0,
    pub cControls: u32,
    pub cbmxctrl: u32,
    pub pamxctrl: *mut MIXERCONTROLW,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW {}
impl ::core::clone::Clone for MIXERLINECONTROLSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINECONTROLSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW {}
impl ::core::default::Default for MIXERLINECONTROLSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union MIXERLINECONTROLSW_0 {
    pub dwControlID: u32,
    pub dwControlType: u32,
}
impl ::core::marker::Copy for MIXERLINECONTROLSW_0 {}
impl ::core::clone::Clone for MIXERLINECONTROLSW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERLINECONTROLSW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERLINECONTROLSW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINECONTROLSW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERLINECONTROLSW_0 {}
impl ::core::default::Default for MIXERLINECONTROLSW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for MIXERLINEW {}
impl ::core::clone::Clone for MIXERLINEW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERLINEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERLINEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERLINEW {}
impl ::core::default::Default for MIXERLINEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct MIXERLINEW_0 {
    pub dwType: u32,
    pub dwDeviceID: u32,
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
}
impl ::core::marker::Copy for MIXERLINEW_0 {}
impl ::core::clone::Clone for MIXERLINEW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MIXERLINEW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIXERLINEW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIXERLINEW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIXERLINEW_0 {}
impl ::core::default::Default for MIXERLINEW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIXERLINE_COMPONENTTYPE(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(1u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_HEADPHONES: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(5u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(2u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_MONITOR: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(3u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_SPEAKERS: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(6u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_VOICEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(8u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_WAVEIN: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(7u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_ANALOG: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4106u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_AUXILIARY: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4105u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_COMPACTDISC: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4101u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_DIGITAL: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4097u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LINE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4098u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_MICROPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4099u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_PCSPEAKER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4103u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_SYNTHESIZER: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4100u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_TELEPHONE: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4102u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_UNDEFINED: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4096u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_WAVEOUT: MIXERLINE_COMPONENTTYPE = MIXERLINE_COMPONENTTYPE(4104u32);
impl ::core::marker::Copy for MIXERLINE_COMPONENTTYPE {}
impl ::core::clone::Clone for MIXERLINE_COMPONENTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIXERLINE_COMPONENTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIXERLINE_COMPONENTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIXERLINE_COMPONENTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIXERLINE_COMPONENTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALLINE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_INVALVALUE: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXERR_LASTERROR: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
pub const MMDeviceEnumerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MAPPER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_MIDIPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SQSYNTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SWSYNTH: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_SYNTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const MOD_WAVETABLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub type PAudioStateMonitorCallback = ::core::option::Option<unsafe extern "system" fn(audiostatemonitor: ::core::option::Option<IAudioStateMonitor>, context: *const ::core::ffi::c_void)>;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct PCMWAVEFORMAT {
    pub wf: WAVEFORMAT,
    pub wBitsPerSample: u16,
}
impl ::core::marker::Copy for PCMWAVEFORMAT {}
impl ::core::clone::Clone for PCMWAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PCMWAVEFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PCMWAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PCMWAVEFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for PCMWAVEFORMAT {}
impl ::core::default::Default for PCMWAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3u32 };
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_LOOPBACK_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_INCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const PROCESS_LOOPBACK_MODE_EXCLUDE_TARGET_PROCESS_TREE: PROCESS_LOOPBACK_MODE = PROCESS_LOOPBACK_MODE(1i32);
impl ::core::marker::Copy for PROCESS_LOOPBACK_MODE {}
impl ::core::clone::Clone for PROCESS_LOOPBACK_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_LOOPBACK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_LOOPBACK_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_LOOPBACK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_LOOPBACK_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PartType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Connector: PartType = PartType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const Subunit: PartType = PartType(1i32);
impl ::core::marker::Copy for PartType {}
impl ::core::clone::Clone for PartType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PartType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PartType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PartType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PartType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundA(pszsound: ::windows::core::PCSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PlaySoundA(pszsound.into_param().abi(), hmod.into_param().abi(), ::core::mem::transmute(fdwsound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PlaySoundW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pszsound: Param0, hmod: Param1, fdwsound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PlaySoundW(pszsound: ::windows::core::PCWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PlaySoundW(pszsound.into_param().abi(), hmod.into_param().abi(), ::core::mem::transmute(fdwsound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_ID: i32 = 1114112i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ALIAS_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_APPLICATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_ASYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_FILENAME: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_LOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_MEMORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NODEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOSTOP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_NOWAIT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_PURGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RESOURCE: i32 = 262148i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_RING: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SENTRY: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYNC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SND_SYSTEM: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SPATIAL_AUDIO_STREAM_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = SPATIAL_AUDIO_STREAM_OPTIONS(1u32);
impl ::core::marker::Copy for SPATIAL_AUDIO_STREAM_OPTIONS {}
impl ::core::clone::Clone for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPATIAL_AUDIO_STREAM_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SPATIAL_AUDIO_STREAM_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SPATIAL_AUDIO_STREAM_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_DESTROYED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287232i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287227i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287219i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287224i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287226i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287229i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287220i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287231i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287228i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287230i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287221i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287225i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004287222i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286956i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286969i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286968i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286940i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286942i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286976i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286955i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286941i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286967i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286952i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286974i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286957i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286939i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286959i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286951i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286971i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286973i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286954i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286944i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286960i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286958i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286943i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286970i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286953i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286975i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2004286972i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: ::windows::core::GUID,
    pub appId: ::windows::core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
impl ::core::marker::Copy for SpatialAudioClientActivationParams {}
impl ::core::clone::Clone for SpatialAudioClientActivationParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SpatialAudioClientActivationParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SpatialAudioClientActivationParams").field("tracingContextId", &self.tracingContextId).field("appId", &self.appId).field("majorVersion", &self.majorVersion).field("minorVersion1", &self.minorVersion1).field("minorVersion2", &self.minorVersion2).field("minorVersion3", &self.minorVersion3).finish()
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioClientActivationParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioClientActivationParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioClientActivationParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioClientActivationParams {}
impl ::core::default::Default for SpatialAudioClientActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
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
unsafe impl ::windows::core::Abi for SpatialAudioHrtfActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfActivationParams>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioHrtfActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
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
unsafe impl ::windows::core::Abi for SpatialAudioHrtfActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioHrtfActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfActivationParams2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioHrtfActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioHrtfActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivity {
    pub Type: SpatialAudioHrtfDirectivityType,
    pub Scaling: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivity {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivity {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivity {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivity {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfDirectivity>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivity {}
impl ::core::default::Default for SpatialAudioHrtfDirectivity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCardioid {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub Order: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCardioid {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCardioid {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityCardioid {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCardioid {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfDirectivityCardioid>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCardioid {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCardioid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDirectivityCone {
    pub directivity: SpatialAudioHrtfDirectivity,
    pub InnerAngle: f32,
    pub OuterAngle: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityCone {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityCone {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityCone {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityCone {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfDirectivityCone>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityCone {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityCone {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioHrtfDirectivityType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_OmniDirectional: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cardioid: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDirectivity_Cone: SpatialAudioHrtfDirectivityType = SpatialAudioHrtfDirectivityType(2i32);
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityType {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDirectivityType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioHrtfDirectivityType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDirectivityType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union SpatialAudioHrtfDirectivityUnion {
    pub Cone: SpatialAudioHrtfDirectivityCone,
    pub Cardiod: SpatialAudioHrtfDirectivityCardioid,
    pub Omni: SpatialAudioHrtfDirectivity,
}
impl ::core::marker::Copy for SpatialAudioHrtfDirectivityUnion {}
impl ::core::clone::Clone for SpatialAudioHrtfDirectivityUnion {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDirectivityUnion {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDirectivityUnion {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfDirectivityUnion>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDirectivityUnion {}
impl ::core::default::Default for SpatialAudioHrtfDirectivityUnion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioHrtfDistanceDecay {
    pub Type: SpatialAudioHrtfDistanceDecayType,
    pub MaxGain: f32,
    pub MinGain: f32,
    pub UnityGainDistance: f32,
    pub CutoffDistance: f32,
}
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecay {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecay {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDistanceDecay {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioHrtfDistanceDecay {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioHrtfDistanceDecay>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioHrtfDistanceDecay {}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecay {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioHrtfDistanceDecayType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_NaturalDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfDistanceDecay_CustomDecay: SpatialAudioHrtfDistanceDecayType = SpatialAudioHrtfDistanceDecayType(1i32);
impl ::core::marker::Copy for SpatialAudioHrtfDistanceDecayType {}
impl ::core::clone::Clone for SpatialAudioHrtfDistanceDecayType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfDistanceDecayType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfDistanceDecayType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioHrtfDistanceDecayType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfDistanceDecayType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioHrtfEnvironmentType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Small: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Medium: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Large: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Outdoors: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(3i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioHrtfEnvironment_Average: SpatialAudioHrtfEnvironmentType = SpatialAudioHrtfEnvironmentType(4i32);
impl ::core::marker::Copy for SpatialAudioHrtfEnvironmentType {}
impl ::core::clone::Clone for SpatialAudioHrtfEnvironmentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioHrtfEnvironmentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioHrtfEnvironmentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioHrtfEnvironmentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioHrtfEnvironmentType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioMetadataCopyMode(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Overwrite: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_Append: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithLast: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataCopy_AppendMergeWithFirst: SpatialAudioMetadataCopyMode = SpatialAudioMetadataCopyMode(3i32);
impl ::core::marker::Copy for SpatialAudioMetadataCopyMode {}
impl ::core::clone::Clone for SpatialAudioMetadataCopyMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataCopyMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataCopyMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioMetadataCopyMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataCopyMode").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct SpatialAudioMetadataItemsInfo {
    pub FrameCount: u16,
    pub ItemCount: u16,
    pub MaxItemCount: u16,
    pub MaxValueBufferLength: u32,
}
impl ::core::marker::Copy for SpatialAudioMetadataItemsInfo {}
impl ::core::clone::Clone for SpatialAudioMetadataItemsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataItemsInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SpatialAudioMetadataItemsInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioMetadataItemsInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for SpatialAudioMetadataItemsInfo {}
impl ::core::default::Default for SpatialAudioMetadataItemsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SpatialAudioMetadataWriterOverflowMode(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_Fail: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithNew: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const SpatialAudioMetadataWriterOverflow_MergeWithLast: SpatialAudioMetadataWriterOverflowMode = SpatialAudioMetadataWriterOverflowMode(2i32);
impl ::core::marker::Copy for SpatialAudioMetadataWriterOverflowMode {}
impl ::core::clone::Clone for SpatialAudioMetadataWriterOverflowMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpatialAudioMetadataWriterOverflowMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SpatialAudioMetadataWriterOverflowMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SpatialAudioMetadataWriterOverflowMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialAudioMetadataWriterOverflowMode").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioObjectRenderStreamActivationParams>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioObjectRenderStreamActivationParams2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamActivationParams2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u16,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioObjectRenderStreamForMetadataActivationParams>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub struct SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    pub ObjectFormat: *const WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub MetadataFormatId: ::windows::core::GUID,
    pub MaxMetadataItemCount: u32,
    pub MetadataActivationParams: *const super::super::System::Com::StructuredStorage::PROPVARIANT,
    pub NotifyObject: ::core::option::Option<ISpatialAudioObjectRenderStreamNotify>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
unsafe impl ::windows::core::Abi for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SpatialAudioObjectRenderStreamForMetadataActivationParams2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SpatialAudioObjectRenderStreamForMetadataActivationParams2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const VIRTUAL_AUDIO_DEVICE_PROCESS_LOOPBACK: &'static str = "VAD\\Process_Loopback";
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct VOLUMEWAVEFILTER {
    pub wfltr: WAVEFILTER,
    pub dwVolume: u32,
}
impl ::core::marker::Copy for VOLUMEWAVEFILTER {}
impl ::core::clone::Clone for VOLUMEWAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VOLUMEWAVEFILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VOLUMEWAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VOLUMEWAVEFILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for VOLUMEWAVEFILTER {}
impl ::core::default::Default for VOLUMEWAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PITCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_SYNC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFILTER {
    pub cbStruct: u32,
    pub dwFilterTag: u32,
    pub fdwFilter: u32,
    pub dwReserved: [u32; 5],
}
impl ::core::marker::Copy for WAVEFILTER {}
impl ::core::clone::Clone for WAVEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEFILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEFILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEFILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEFILTER {}
impl ::core::default::Default for WAVEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMAT {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
}
impl ::core::marker::Copy for WAVEFORMAT {}
impl ::core::clone::Clone for WAVEFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEFORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEFORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEFORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEFORMAT {}
impl ::core::default::Default for WAVEFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
impl ::core::marker::Copy for WAVEFORMATEX {}
impl ::core::clone::Clone for WAVEFORMATEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEFORMATEX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEFORMATEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEFORMATEX>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEFORMATEX {}
impl ::core::default::Default for WAVEFORMATEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub Samples: WAVEFORMATEXTENSIBLE_0,
    pub dwChannelMask: u32,
    pub SubFormat: ::windows::core::GUID,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEFORMATEXTENSIBLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEFORMATEXTENSIBLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE {}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub union WAVEFORMATEXTENSIBLE_0 {
    pub wValidBitsPerSample: u16,
    pub wSamplesPerBlock: u16,
    pub wReserved: u16,
}
impl ::core::marker::Copy for WAVEFORMATEXTENSIBLE_0 {}
impl ::core::clone::Clone for WAVEFORMATEXTENSIBLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEFORMATEXTENSIBLE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEFORMATEXTENSIBLE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEFORMATEXTENSIBLE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEFORMATEXTENSIBLE_0 {}
impl ::core::default::Default for WAVEFORMATEXTENSIBLE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEHDR {
    pub lpData: ::windows::core::PSTR,
    pub dwBufferLength: u32,
    pub dwBytesRecorded: u32,
    pub dwUser: usize,
    pub dwFlags: u32,
    pub dwLoops: u32,
    pub lpNext: *mut WAVEHDR,
    pub reserved: usize,
}
impl ::core::marker::Copy for WAVEHDR {}
impl ::core::clone::Clone for WAVEHDR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEHDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEHDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEHDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEHDR {}
impl ::core::default::Default for WAVEHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEINCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEINCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEINCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEINCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEINCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEINCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for WAVEINCAPS2W {}
impl ::core::clone::Clone for WAVEINCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEINCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEINCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEINCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEINCAPS2W {}
impl ::core::default::Default for WAVEINCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEINCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEINCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEINCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEINCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEINCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEINCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct WAVEINCAPSW {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    pub szPname: [u16; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
}
impl ::core::marker::Copy for WAVEINCAPSW {}
impl ::core::clone::Clone for WAVEINCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEINCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEINCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEINCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEINCAPSW {}
impl ::core::default::Default for WAVEINCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEOUTCAPS2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEOUTCAPS2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEOUTCAPS2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEOUTCAPS2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEOUTCAPS2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEOUTCAPS2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for WAVEOUTCAPS2W {}
impl ::core::clone::Clone for WAVEOUTCAPS2W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEOUTCAPS2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEOUTCAPS2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEOUTCAPS2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPS2W {}
impl ::core::default::Default for WAVEOUTCAPS2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WAVEOUTCAPSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WAVEOUTCAPSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WAVEOUTCAPSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEOUTCAPSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WAVEOUTCAPSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WAVEOUTCAPSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
impl ::core::marker::Copy for WAVEOUTCAPSW {}
impl ::core::clone::Clone for WAVEOUTCAPSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WAVEOUTCAPSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WAVEOUTCAPSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WAVEOUTCAPSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for WAVEOUTCAPSW {}
impl ::core::default::Default for WAVEOUTCAPSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_BADFORMAT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_LASTERROR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_STILLPLAYING: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_SYNC: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVERR_UNPREPARED: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M08: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1M16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S08: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_1S16: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M08: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2M16: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S08: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_2S16: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M08: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S08: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_FORMAT_PCM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WAVE_MAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_BEGINLOOP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_ENDLOOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_INQUEUE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(4i32);
impl ::core::marker::Copy for _AUDCLNT_BUFFERFLAGS {}
impl ::core::clone::Clone for _AUDCLNT_BUFFERFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _AUDCLNT_BUFFERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for _AUDCLNT_BUFFERFLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for _AUDCLNT_BUFFERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AUDCLNT_BUFFERFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(pub i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(0i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(1i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(2i32);
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 = __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002(3i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    type Abi = Self;
}
impl ::core::fmt::Debug for __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32;
        }
        ::core::mem::transmute(acmFilterChooseA(::core::mem::transmute(pafltrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32;
        }
        ::core::mem::transmute(acmFilterChooseW(::core::mem::transmute(pafltrc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFilterTagEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32;
        }
        ::core::mem::transmute(acmFormatChooseA(::core::mem::transmute(pafmtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32;
        }
        ::core::mem::transmute(acmFormatChooseW(::core::mem::transmute(pafmtc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumA<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn acmFormatTagEnumW<'a, Param0: ::windows::core::IntoParam<'a, HACMDRIVER>>(had: Param0, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32 {
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextA(mmrerror: u32, psztext: ::windows::core::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiInGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiInGetErrorTextW(mmrerror: u32, psztext: ::windows::core::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiInGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutCacheDrumPatches<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, upatch: u32, pwkya: &[u16; 128], fucache: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
        }
        ::core::mem::transmute(midiOutCacheDrumPatches(hmo.into_param().abi(), ::core::mem::transmute(upatch), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwkya)), ::core::mem::transmute(fucache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutCachePatches<'a, Param0: ::windows::core::IntoParam<'a, HMIDIOUT>>(hmo: Param0, ubank: u32, pwpa: &[u16; 128], fucache: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
        }
        ::core::mem::transmute(midiOutCachePatches(hmo.into_param().abi(), ::core::mem::transmute(ubank), ::core::mem::transmute(::windows::core::as_ptr_or_null(pwpa)), ::core::mem::transmute(fucache)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextA(mmrerror: u32, psztext: ::windows::core::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiOutGetErrorTextW(mmrerror: u32, psztext: ::windows::core::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(midiOutGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: &mut [u32], dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
        }
        ::core::mem::transmute(midiStreamOpen(::core::mem::transmute(phms), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pudeviceid)), pudeviceid.len() as _, ::core::mem::transmute(dwcallback), ::core::mem::transmute(dwinstance), ::core::mem::transmute(fdwopen)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundA(pszsound: ::windows::core::PCSTR, fusound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(sndPlaySoundA(pszsound.into_param().abi(), ::core::mem::transmute(fusound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn sndPlaySoundW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszsound: Param0, fusound: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn sndPlaySoundW(pszsound: ::windows::core::PCWSTR, fusound: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(sndPlaySoundW(pszsound.into_param().abi(), ::core::mem::transmute(fusound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMDRVOPENDESCA {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows::core::PCSTR,
    pub pszAliasName: ::windows::core::PCSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCA {}
impl ::core::clone::Clone for tACMDRVOPENDESCA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tACMDRVOPENDESCA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tACMDRVOPENDESCA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tACMDRVOPENDESCA>()) == 0 }
    }
}
impl ::core::cmp::Eq for tACMDRVOPENDESCA {}
impl ::core::default::Default for tACMDRVOPENDESCA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMDRVOPENDESCW {
    pub cbStruct: u32,
    pub fccType: u32,
    pub fccComp: u32,
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub dwError: u32,
    pub pszSectionName: ::windows::core::PCWSTR,
    pub pszAliasName: ::windows::core::PCWSTR,
    pub dnDevNode: u32,
}
impl ::core::marker::Copy for tACMDRVOPENDESCW {}
impl ::core::clone::Clone for tACMDRVOPENDESCW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tACMDRVOPENDESCW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tACMDRVOPENDESCW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tACMDRVOPENDESCW>()) == 0 }
    }
}
impl ::core::cmp::Eq for tACMDRVOPENDESCW {}
impl ::core::default::Default for tACMDRVOPENDESCW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
pub struct tACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: *mut WAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
impl ::core::marker::Copy for tACMFORMATDETAILSW {}
impl ::core::clone::Clone for tACMFORMATDETAILSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tACMFORMATDETAILSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tACMFORMATDETAILSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tACMFORMATDETAILSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for tACMFORMATDETAILSW {}
impl ::core::default::Default for tACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextA(mmrerror: u32, psztext: ::windows::core::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveInGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveInGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveInGetErrorTextW(mmrerror: u32, psztext: ::windows::core::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveInGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetErrorTextA(mmrerror: u32, psztext: &mut [u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextA(mmrerror: u32, psztext: ::windows::core::PSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetErrorTextA(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
#[inline]
pub unsafe fn waveOutGetErrorTextW(mmrerror: u32, psztext: &mut [u16]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn waveOutGetErrorTextW(mmrerror: u32, psztext: ::windows::core::PWSTR, cchtext: u32) -> u32;
        }
        ::core::mem::transmute(waveOutGetErrorTextW(::core::mem::transmute(mmrerror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(psztext)), psztext.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
