#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : *mut HACMDRIVERID, hinstmodule : super::HINSTANCE, lparam : super::LPARAM, dwpriority : u32, fdwadd : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : *mut HACMDRIVERID, hinstmodule : super::HINSTANCE, lparam : super::LPARAM, dwpriority : u32, fdwadd : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSA, fdwdetails : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : *mut ACMDRIVERDETAILSW, fdwdetails : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : *mut HACMDRIVERID, fdwdriverid : u32) -> super::MMRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : u32, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverOpen(phad : *mut HACMDRIVER, hadid : HACMDRIVERID, fdwopen : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : u32, fdwpriority : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : *mut ACMFILTERCHOOSEA) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : *mut ACMFILTERCHOOSEW) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fdwdetails : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fdwdetails : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : *mut ACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fdwdetails : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fdwdetails : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : *mut ACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : *mut ACMFORMATCHOOSEA) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : *mut ACMFORMATCHOOSEW) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fdwdetails : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSW, fdwdetails : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : *mut ACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : *mut super::WAVEFORMATEX, pwfxdst : *mut super::WAVEFORMATEX, cbwfxdst : u32, fdwsuggest : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fdwdetails : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fdwdetails : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : *mut ACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : usize, fdwenum : u32) -> super::MMRESULT);
windows_link::link!("msacm32.dll" "system" fn acmGetVersion() -> u32);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : u32, pmetric : *mut core::ffi::c_void) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwconvert : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : u32, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmreg", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamOpen(phas : *mut HACMSTREAM, had : HACMDRIVER, pwfxsrc : *mut super::WAVEFORMATEX, pwfxdst : *mut super::WAVEFORMATEX, pwfltr : *mut super::WAVEFILTER, dwcallback : usize, dwinstance : usize, fdwopen : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwprepare : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : u32) -> super::MMRESULT);
#[cfg(feature = "mmsyscom")]
windows_link::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : u32, pdwoutputbytes : *mut u32, fdwsize : u32) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : *mut ACMSTREAMHEADER, fdwunprepare : u32) -> super::MMRESULT);
pub const ACMDM_BASE: u32 = 24576;
pub const ACMDM_DRIVER_ABOUT: u32 = 24587;
pub const ACMDM_RESERVED_HIGH: u32 = 28671;
pub const ACMDM_RESERVED_LOW: u32 = 24576;
pub const ACMDM_USER: u32 = 16384;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: u32,
    pub fccType: super::FOURCC,
    pub fccComp: super::FOURCC,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::HICON,
    pub szShortName: [i8; 32],
    pub szLongName: [i8; 128],
    pub szCopyright: [i8; 80],
    pub szLicensing: [i8; 128],
    pub szFeatures: [i8; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: u32,
    pub fccType: super::FOURCC,
    pub fccComp: super::FOURCC,
    pub wMid: u16,
    pub wPid: u16,
    pub vdwACM: u32,
    pub vdwDriver: u32,
    pub fdwSupport: u32,
    pub cFormatTags: u32,
    pub cFilterTags: u32,
    pub hicon: super::HICON,
    pub szShortName: [u16; 32],
    pub szLongName: [u16; 128],
    pub szCopyright: [u16; 80],
    pub szLicensing: [u16; 128],
    pub szFeatures: [u16; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
impl Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80;
pub const ACMDRIVERDETAILS_FCCCOMP_UNDEFINED: u32 = 0;
pub const ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC: u32 = 1667528033;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: u32 = 16;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: u32 = 1;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: u32 = 2;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: u32 = 2147483648;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: u32 = 4;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: u32 = 8;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: u32 = 1073741824;
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "minwindef")]
pub type ACMDRIVERPROC = Option<unsafe extern "system" fn(param0: usize, param1: HACMDRIVERID, param2: u32, param3: super::LPARAM, param4: super::LPARAM) -> super::LRESULT>;
pub const ACMERR_BASE: u32 = 512;
pub const ACMERR_BUSY: u32 = 513;
pub const ACMERR_CANCELED: u32 = 515;
pub const ACMERR_NOTPOSSIBLE: u32 = 512;
pub const ACMERR_UNPREPARED: u32 = 514;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::HWND,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFilterTag: [i8; 48],
    pub szFilter: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: super::LPWAVEFILTER,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::HWND,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFilterTag: [u16; 48],
    pub szFilter: [u16; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfltrEnum: super::LPWAVEFILTER,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
impl Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: u32 = 128;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: u32 = 8;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: u32 = 16;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: u32 = 32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: u32 = 64;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: u32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "mmreg")]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [i8; 128],
}
#[cfg(feature = "mmreg")]
impl Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmreg")]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: u32,
    pub dwFilterIndex: u32,
    pub dwFilterTag: u32,
    pub fdwSupport: u32,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: u32,
    pub szFilter: [u16; 128],
}
#[cfg(feature = "mmreg")]
impl Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128;
#[cfg(feature = "mmreg")]
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "mmreg")]
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFILTERDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [i8; 48],
}
impl Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFilterTagIndex: u32,
    pub dwFilterTag: u32,
    pub cbFilterSize: u32,
    pub fdwSupport: u32,
    pub cStandardFilters: u32,
    pub szFilterTag: [u16; 48],
}
impl Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48;
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFILTERTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::HWND,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFormatTag: [i8; 48],
    pub szFormat: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: super::LPWAVEFORMATEX,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> u32>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: u32, wparam: super::WPARAM, lparam: super::LPARAM) -> u32>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: u32,
    pub fdwStyle: u32,
    pub hwndOwner: super::HWND,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFormatTag: [u16; 48],
    pub szFormat: [u16; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: u32,
    pub fdwEnum: u32,
    pub pwfxEnum: super::LPWAVEFORMATEX,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
impl Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: u32 = 128;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: u32 = 8;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: u32 = 16;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: u32 = 32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: u32 = 64;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: u32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [i8; 128],
}
#[cfg(feature = "mmeapi")]
impl Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "mmeapi")]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSW {
    pub cbStruct: u32,
    pub dwFormatIndex: u32,
    pub dwFormatTag: u32,
    pub fdwSupport: u32,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: u32,
    pub szFormat: [u16; 128],
}
#[cfg(feature = "mmeapi")]
impl Default for ACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128;
#[cfg(feature = "mmeapi")]
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[cfg(feature = "mmeapi")]
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: *mut ACMFORMATDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [i8; 48],
}
impl Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: u32,
    pub dwFormatTagIndex: u32,
    pub dwFormatTag: u32,
    pub cbFormatSize: u32,
    pub fdwSupport: u32,
    pub cStandardFormats: u32,
    pub szFormatTag: [u16; 48],
}
impl Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48;
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSA, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: *mut ACMFORMATTAGDETAILSW, dwinstance: usize, fdwsupport: u32) -> windows_sys::core::BOOL>;
pub const ACMHELPMSGCONTEXTHELPA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENUA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_help");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: super::LPBYTE,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: super::LPBYTE,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 10],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "minwindef")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: u32,
    pub fdwStatus: u32,
    pub dwUser: usize,
    pub pbSrc: super::LPBYTE,
    pub cbSrcLength: u32,
    pub cbSrcLengthUsed: u32,
    pub dwSrcUser: usize,
    pub pbDst: super::LPBYTE,
    pub cbDstLength: u32,
    pub cbDstLengthUsed: u32,
    pub dwDstUser: usize,
    pub dwReservedDriver: [u32; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "minwindef")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMSTREAMHEADER_STATUSF_DONE: u32 = 65536;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: u32 = 1048576;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: u32 = 131072;
pub const ACM_DRIVERADDF_FUNCTION: u32 = 3;
pub const ACM_DRIVERADDF_GLOBAL: u32 = 8;
pub const ACM_DRIVERADDF_LOCAL: u32 = 0;
pub const ACM_DRIVERADDF_NAME: u32 = 1;
pub const ACM_DRIVERADDF_NOTIFYHWND: u32 = 4;
pub const ACM_DRIVERADDF_TYPEMASK: u32 = 7;
pub const ACM_DRIVERENUMF_DISABLED: u32 = 2147483648;
pub const ACM_DRIVERENUMF_NOLOCAL: u32 = 1073741824;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: u32 = 3;
pub const ACM_DRIVERPRIORITYF_BEGIN: u32 = 65536;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: u32 = 196608;
pub const ACM_DRIVERPRIORITYF_DISABLE: u32 = 2;
pub const ACM_DRIVERPRIORITYF_ENABLE: u32 = 1;
pub const ACM_DRIVERPRIORITYF_END: u32 = 131072;
pub const ACM_FILTERDETAILSF_FILTER: u32 = 1;
pub const ACM_FILTERDETAILSF_INDEX: u32 = 0;
pub const ACM_FILTERDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FILTERENUMF_DWFILTERTAG: u32 = 65536;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: u32 = 1;
pub const ACM_FILTERTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: u32 = 2;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FORMATDETAILSF_FORMAT: u32 = 1;
pub const ACM_FORMATDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_FORMATENUMF_CONVERT: u32 = 1048576;
pub const ACM_FORMATENUMF_HARDWARE: u32 = 4194304;
pub const ACM_FORMATENUMF_INPUT: u32 = 8388608;
pub const ACM_FORMATENUMF_NCHANNELS: u32 = 131072;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: u32 = 262144;
pub const ACM_FORMATENUMF_OUTPUT: u32 = 16777216;
pub const ACM_FORMATENUMF_SUGGEST: u32 = 2097152;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: u32 = 524288;
pub const ACM_FORMATENUMF_WFORMATTAG: u32 = 65536;
pub const ACM_FORMATSUGGESTF_NCHANNELS: u32 = 131072;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: u32 = 262144;
pub const ACM_FORMATSUGGESTF_TYPEMASK: u32 = 16711680;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: u32 = 524288;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: u32 = 65536;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: u32 = 1;
pub const ACM_FORMATTAGDETAILSF_INDEX: u32 = 0;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: u32 = 2;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: u32 = 15;
pub const ACM_METRIC_COUNT_CODECS: u32 = 2;
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3;
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5;
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1;
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4;
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23;
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101;
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31;
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51;
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4;
pub const ACM_STREAMCONVERTF_END: u32 = 32;
pub const ACM_STREAMCONVERTF_START: u32 = 16;
pub const ACM_STREAMOPENF_ASYNC: u32 = 2;
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4;
pub const ACM_STREAMOPENF_QUERY: u32 = 1;
pub const ACM_STREAMSIZEF_DESTINATION: u32 = 1;
pub const ACM_STREAMSIZEF_QUERYMASK: u32 = 15;
pub const ACM_STREAMSIZEF_SOURCE: u32 = 0;
pub const DRVM_MAPPER: u32 = 8192;
pub const DRVM_MAPPER_STATUS: u32 = 8192;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386;
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0;
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1;
pub const FILTERCHOOSE_MESSAGE: u32 = 0;
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0;
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1;
pub const FORMATCHOOSE_MESSAGE: u32 = 0;
pub type HACMDRIVER = *mut core::ffi::c_void;
pub type HACMDRIVERID = *mut core::ffi::c_void;
pub type HACMOBJ = *mut core::ffi::c_void;
pub type HACMSTREAM = *mut core::ffi::c_void;
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
pub type LPACMDRIVERDETAILSA = *mut ACMDRIVERDETAILSA;
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
pub type LPACMDRIVERDETAILSW = *mut ACMDRIVERDETAILSW;
#[cfg(feature = "minwindef")]
pub type LPACMDRIVERPROC = *mut ACMDRIVERPROC;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
pub type LPACMFILTERCHOOSEA = *mut ACMFILTERCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
pub type LPACMFILTERCHOOSEW = *mut ACMFILTERCHOOSEW;
#[cfg(feature = "mmreg")]
pub type LPACMFILTERDETAILSA = *mut ACMFILTERDETAILSA;
#[cfg(feature = "mmreg")]
pub type LPACMFILTERDETAILSW = *mut ACMFILTERDETAILSW;
pub type LPACMFILTERTAGDETAILSA = *mut ACMFILTERTAGDETAILSA;
pub type LPACMFILTERTAGDETAILSW = *mut ACMFILTERTAGDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type LPACMFORMATCHOOSEA = *mut ACMFORMATCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type LPACMFORMATCHOOSEW = *mut ACMFORMATCHOOSEW;
#[cfg(feature = "mmeapi")]
pub type LPACMFORMATDETAILSA = *mut ACMFORMATDETAILSA;
#[cfg(feature = "mmeapi")]
pub type LPACMFORMATDETAILSW = *mut ACMFORMATDETAILSW;
pub type LPACMFORMATTAGDETAILSA = *mut ACMFORMATTAGDETAILSA;
pub type LPACMFORMATTAGDETAILSW = *mut ACMFORMATTAGDETAILSW;
#[cfg(feature = "minwindef")]
pub type LPACMSTREAMHEADER = *mut ACMSTREAMHEADER;
pub type LPHACMDRIVER = *mut HACMDRIVER;
pub type LPHACMDRIVERID = *mut HACMDRIVERID;
pub type LPHACMOBJ = *mut HACMOBJ;
pub type LPHACMSTREAM = *mut HACMSTREAM;
pub const MM_ACM_CLOSE: u32 = 981;
pub const MM_ACM_DONE: u32 = 982;
pub const MM_ACM_FILTERCHOOSE: u32 = 32768;
pub const MM_ACM_FORMATCHOOSE: u32 = 32768;
pub const MM_ACM_OPEN: u32 = 980;
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
pub type PACMDRIVERDETAILSA = *mut ACMDRIVERDETAILSA;
#[cfg(all(feature = "mmiscapi", feature = "windef"))]
pub type PACMDRIVERDETAILSW = *mut ACMDRIVERDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
pub type PACMFILTERCHOOSEA = *mut ACMFILTERCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "windef"))]
pub type PACMFILTERCHOOSEW = *mut ACMFILTERCHOOSEW;
#[cfg(feature = "mmreg")]
pub type PACMFILTERDETAILSA = *mut ACMFILTERDETAILSA;
#[cfg(feature = "mmreg")]
pub type PACMFILTERDETAILSW = *mut ACMFILTERDETAILSW;
pub type PACMFILTERTAGDETAILSA = *mut ACMFILTERTAGDETAILSA;
pub type PACMFILTERTAGDETAILSW = *mut ACMFILTERTAGDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type PACMFORMATCHOOSEA = *mut ACMFORMATCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "windef"))]
pub type PACMFORMATCHOOSEW = *mut ACMFORMATCHOOSEW;
#[cfg(feature = "mmeapi")]
pub type PACMFORMATDETAILSA = *mut ACMFORMATDETAILSA;
#[cfg(feature = "mmeapi")]
pub type PACMFORMATDETAILSW = *mut ACMFORMATDETAILSW;
pub type PACMFORMATTAGDETAILSA = *mut ACMFORMATTAGDETAILSA;
pub type PACMFORMATTAGDETAILSW = *mut ACMFORMATTAGDETAILSW;
#[cfg(feature = "minwindef")]
pub type PACMSTREAMHEADER = *mut ACMSTREAMHEADER;
pub type PHACMDRIVER = *mut HACMDRIVER;
pub type PHACMDRIVERID = *mut HACMDRIVERID;
pub type PHACMOBJ = *mut HACMOBJ;
pub type PHACMSTREAM = *mut HACMSTREAM;
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0;
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2;
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1;
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1;
pub const WIDM_MAPPER_STATUS: u32 = 8192;
pub const WODM_MAPPER_STATUS: u32 = 8192;
