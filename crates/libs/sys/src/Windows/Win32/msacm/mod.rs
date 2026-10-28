#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverAddA(phadid : LPHACMDRIVERID, hinstmodule : super::HINSTANCE, lparam : super::LPARAM, dwpriority : super::DWORD, fdwadd : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverAddW(phadid : LPHACMDRIVERID, hinstmodule : super::HINSTANCE, lparam : super::LPARAM, dwpriority : super::DWORD, fdwadd : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverClose(had : HACMDRIVER, fdwclose : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "shared", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverDetailsA(hadid : HACMDRIVERID, padd : LPACMDRIVERDETAILSA, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmiscapi", feature = "mmsyscom", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverDetailsW(hadid : HACMDRIVERID, padd : LPACMDRIVERDETAILSW, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverEnum(fncallback : ACMDRIVERENUMCB, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverID(hao : HACMOBJ, phadid : LPHACMDRIVERID, fdwdriverid : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverMessage(had : HACMDRIVER, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::LRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverOpen(phad : LPHACMDRIVER, hadid : HACMDRIVERID, fdwopen : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverPriority(hadid : HACMDRIVERID, dwpriority : super::DWORD, fdwpriority : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmDriverRemove(hadid : HACMDRIVERID, fdwremove : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "shared", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterChooseA(pafltrc : LPACMFILTERCHOOSEA) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "mmsyscom", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterChooseW(pafltrc : LPACMFILTERCHOOSEW) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterDetailsA(had : HACMDRIVER, pafd : LPACMFILTERDETAILSA, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterDetailsW(had : HACMDRIVER, pafd : LPACMFILTERDETAILSW, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterEnumA(had : HACMDRIVER, pafd : LPACMFILTERDETAILSA, fncallback : ACMFILTERENUMCBA, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmreg", feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterEnumW(had : HACMDRIVER, pafd : LPACMFILTERDETAILSW, fncallback : ACMFILTERENUMCBW, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagDetailsA(had : HACMDRIVER, paftd : LPACMFILTERTAGDETAILSA, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagDetailsW(had : HACMDRIVER, paftd : LPACMFILTERTAGDETAILSW, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagEnumA(had : HACMDRIVER, paftd : LPACMFILTERTAGDETAILSA, fncallback : ACMFILTERTAGENUMCBA, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFilterTagEnumW(had : HACMDRIVER, paftd : LPACMFILTERTAGDETAILSW, fncallback : ACMFILTERTAGENUMCBW, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "shared", feature = "windef"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatChooseA(pafmtc : LPACMFORMATCHOOSEA) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "mmsyscom", feature = "shared", feature = "windef", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatChooseW(pafmtc : LPACMFORMATCHOOSEW) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatDetailsA(had : HACMDRIVER, pafd : LPACMFORMATDETAILSA, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatDetailsW(had : HACMDRIVER, pafd : LPACMFORMATDETAILSW, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatEnumA(had : HACMDRIVER, pafd : LPACMFORMATDETAILSA, fncallback : ACMFORMATENUMCBA, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatEnumW(had : HACMDRIVER, pafd : LPACMFORMATDETAILSW, fncallback : ACMFORMATENUMCBW, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatSuggest(had : HACMDRIVER, pwfxsrc : super::LPWAVEFORMATEX, pwfxdst : super::LPWAVEFORMATEX, cbwfxdst : super::DWORD, fdwsuggest : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagDetailsA(had : HACMDRIVER, paftd : LPACMFORMATTAGDETAILSA, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagDetailsW(had : HACMDRIVER, paftd : LPACMFORMATTAGDETAILSW, fdwdetails : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagEnumA(had : HACMDRIVER, paftd : LPACMFORMATTAGDETAILSA, fncallback : ACMFORMATTAGENUMCBA, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared", feature = "winnt"))]
windows_link::link!("msacm32.dll" "system" fn acmFormatTagEnumW(had : HACMDRIVER, paftd : LPACMFORMATTAGDETAILSW, fncallback : ACMFORMATTAGENUMCBW, dwinstance : super::DWORD_PTR, fdwenum : super::DWORD) -> super::MMRESULT);
#[cfg(feature = "shared")]
windows_link::link!("msacm32.dll" "system" fn acmGetVersion() -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmMetrics(hao : HACMOBJ, umetric : super::UINT, pmetric : super::LPVOID) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamClose(has : HACMSTREAM, fdwclose : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamConvert(has : HACMSTREAM, pash : LPACMSTREAMHEADER, fdwconvert : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamMessage(has : HACMSTREAM, umsg : super::UINT, lparam1 : super::LPARAM, lparam2 : super::LPARAM) -> super::MMRESULT);
#[cfg(all(feature = "mmeapi", feature = "mmreg", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamOpen(phas : LPHACMSTREAM, had : HACMDRIVER, pwfxsrc : super::LPWAVEFORMATEX, pwfxdst : super::LPWAVEFORMATEX, pwfltr : super::LPWAVEFILTER, dwcallback : super::DWORD_PTR, dwinstance : super::DWORD_PTR, fdwopen : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamPrepareHeader(has : HACMSTREAM, pash : LPACMSTREAMHEADER, fdwprepare : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamReset(has : HACMSTREAM, fdwreset : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "minwindef", feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamSize(has : HACMSTREAM, cbinput : super::DWORD, pdwoutputbytes : super::LPDWORD, fdwsize : super::DWORD) -> super::MMRESULT);
#[cfg(all(feature = "mmsyscom", feature = "shared"))]
windows_link::link!("msacm32.dll" "system" fn acmStreamUnprepareHeader(has : HACMSTREAM, pash : LPACMSTREAMHEADER, fdwunprepare : super::DWORD) -> super::MMRESULT);
pub const ACMDM_BASE: i32 = 24576;
pub const ACMDM_DRIVER_ABOUT: i32 = 24587;
pub const ACMDM_RESERVED_HIGH: i32 = 28671;
pub const ACMDM_RESERVED_LOW: i32 = 24576;
pub const ACMDM_USER: i32 = 16384;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSA {
    pub cbStruct: super::DWORD,
    pub fccType: super::FOURCC,
    pub fccComp: super::FOURCC,
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vdwACM: super::DWORD,
    pub vdwDriver: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cFormatTags: super::DWORD,
    pub cFilterTags: super::DWORD,
    pub hicon: super::HICON,
    pub szShortName: [i8; 32],
    pub szLongName: [i8; 128],
    pub szCopyright: [i8; 80],
    pub szLicensing: [i8; 128],
    pub szFeatures: [i8; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef"))]
impl Default for ACMDRIVERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMDRIVERDETAILSW {
    pub cbStruct: super::DWORD,
    pub fccType: super::FOURCC,
    pub fccComp: super::FOURCC,
    pub wMid: super::WORD,
    pub wPid: super::WORD,
    pub vdwACM: super::DWORD,
    pub vdwDriver: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cFormatTags: super::DWORD,
    pub cFilterTags: super::DWORD,
    pub hicon: super::HICON,
    pub szShortName: [super::WCHAR; 32],
    pub szLongName: [super::WCHAR; 128],
    pub szCopyright: [super::WCHAR; 80],
    pub szLicensing: [super::WCHAR; 128],
    pub szFeatures: [super::WCHAR; 512],
}
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for ACMDRIVERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: i32 = 80;
#[cfg(feature = "shared")]
pub const ACMDRIVERDETAILS_FCCCOMP_UNDEFINED: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const ACMDRIVERDETAILS_FCCTYPE_AUDIOCODEC: super::DWORD = 1667528033;
pub const ACMDRIVERDETAILS_FEATURES_CHARS: i32 = 512;
pub const ACMDRIVERDETAILS_LICENSING_CHARS: i32 = 128;
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: i32 = 128;
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: i32 = 32;
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16;
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1;
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2;
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: u32 = 2147483648;
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4;
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8;
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824;
#[cfg(feature = "shared")]
pub type ACMDRIVERENUMCB = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ACMDRIVERPROC = Option<unsafe extern "system" fn(param0: super::DWORD_PTR, param1: HACMDRIVERID, param2: super::UINT, param3: super::LPARAM, param4: super::LPARAM) -> super::LRESULT>;
pub const ACMERR_BASE: i32 = 512;
pub const ACMERR_BUSY: i32 = 513;
pub const ACMERR_CANCELED: i32 = 515;
pub const ACMERR_NOTPOSSIBLE: i32 = 512;
pub const ACMERR_UNPREPARED: i32 = 514;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEA {
    pub cbStruct: super::DWORD,
    pub fdwStyle: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: super::DWORD,
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFilterTag: [i8; 48],
    pub szFilter: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: super::DWORD,
    pub fdwEnum: super::DWORD,
    pub pwfltrEnum: super::LPWAVEFILTER,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef"))]
impl Default for ACMFILTERCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::UINT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ACMFILTERCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::UINT>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERCHOOSEW {
    pub cbStruct: super::DWORD,
    pub fdwStyle: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: super::DWORD,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFilterTag: [super::WCHAR; 48],
    pub szFilter: [super::WCHAR; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: super::DWORD,
    pub fdwEnum: super::DWORD,
    pub pwfltrEnum: super::LPWAVEFILTER,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFILTERCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for ACMFILTERCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128;
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16;
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32;
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64;
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmreg", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSA {
    pub cbStruct: super::DWORD,
    pub dwFilterIndex: super::DWORD,
    pub dwFilterTag: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: super::DWORD,
    pub szFilter: [i8; 128],
}
#[cfg(all(feature = "mmreg", feature = "shared"))]
impl Default for ACMFILTERDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmreg", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERDETAILSW {
    pub cbStruct: super::DWORD,
    pub dwFilterIndex: super::DWORD,
    pub dwFilterTag: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub pwfltr: super::LPWAVEFILTER,
    pub cbwfltr: super::DWORD,
    pub szFilter: [super::WCHAR; 128],
}
#[cfg(all(feature = "mmreg", feature = "shared", feature = "winnt"))]
impl Default for ACMFILTERDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERDETAILS_FILTER_CHARS: i32 = 128;
#[cfg(all(feature = "mmreg", feature = "shared"))]
pub type ACMFILTERENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: LPACMFILTERDETAILSA, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mmreg", feature = "shared", feature = "winnt"))]
pub type ACMFILTERENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: LPACMFILTERDETAILSW, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSA {
    pub cbStruct: super::DWORD,
    pub dwFilterTagIndex: super::DWORD,
    pub dwFilterTag: super::DWORD,
    pub cbFilterSize: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cStandardFilters: super::DWORD,
    pub szFilterTag: [i8; 48],
}
#[cfg(feature = "shared")]
impl Default for ACMFILTERTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFILTERTAGDETAILSW {
    pub cbStruct: super::DWORD,
    pub dwFilterTagIndex: super::DWORD,
    pub dwFilterTag: super::DWORD,
    pub cbFilterSize: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cStandardFilters: super::DWORD,
    pub szFilterTag: [super::WCHAR; 48],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ACMFILTERTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: i32 = 48;
#[cfg(feature = "shared")]
pub type ACMFILTERTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: LPACMFILTERTAGDETAILSA, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type ACMFILTERTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: LPACMFILTERTAGDETAILSW, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEA {
    pub cbStruct: super::DWORD,
    pub fdwStyle: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: super::DWORD,
    pub pszTitle: windows_sys::core::PCSTR,
    pub szFormatTag: [i8; 48],
    pub szFormat: [i8; 128],
    pub pszName: windows_sys::core::PSTR,
    pub cchName: super::DWORD,
    pub fdwEnum: super::DWORD,
    pub pwfxEnum: super::LPWAVEFORMATEX,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCA,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef"))]
impl Default for ACMFORMATCHOOSEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCA = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::UINT>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type ACMFORMATCHOOSEHOOKPROCW = Option<unsafe extern "system" fn(hwnd: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM) -> super::UINT>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATCHOOSEW {
    pub cbStruct: super::DWORD,
    pub fdwStyle: super::DWORD,
    pub hwndOwner: super::HWND,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: super::DWORD,
    pub pszTitle: windows_sys::core::PCWSTR,
    pub szFormatTag: [super::WCHAR; 48],
    pub szFormat: [super::WCHAR; 128],
    pub pszName: windows_sys::core::PWSTR,
    pub cchName: super::DWORD,
    pub fdwEnum: super::DWORD,
    pub pwfxEnum: super::LPWAVEFORMATEX,
    pub hInstance: super::HINSTANCE,
    pub pszTemplateName: windows_sys::core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub pfnHook: ACMFORMATCHOOSEHOOKPROCW,
}
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for ACMFORMATCHOOSEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128;
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16;
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32;
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64;
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4;
#[repr(C, packed(1))]
#[cfg(all(feature = "mmeapi", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSA {
    pub cbStruct: super::DWORD,
    pub dwFormatIndex: super::DWORD,
    pub dwFormatTag: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: super::DWORD,
    pub szFormat: [i8; 128],
}
#[cfg(all(feature = "mmeapi", feature = "shared"))]
impl Default for ACMFORMATDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "mmeapi", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATDETAILSW {
    pub cbStruct: super::DWORD,
    pub dwFormatIndex: super::DWORD,
    pub dwFormatTag: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub pwfx: super::LPWAVEFORMATEX,
    pub cbwfx: super::DWORD,
    pub szFormat: [super::WCHAR; 128],
}
#[cfg(all(feature = "mmeapi", feature = "shared", feature = "winnt"))]
impl Default for ACMFORMATDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATDETAILS_FORMAT_CHARS: i32 = 128;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type ACMFORMATENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: LPACMFORMATDETAILSA, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "mmeapi", feature = "shared", feature = "winnt"))]
pub type ACMFORMATENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, pafd: LPACMFORMATDETAILSW, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSA {
    pub cbStruct: super::DWORD,
    pub dwFormatTagIndex: super::DWORD,
    pub dwFormatTag: super::DWORD,
    pub cbFormatSize: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cStandardFormats: super::DWORD,
    pub szFormatTag: [i8; 48],
}
#[cfg(feature = "shared")]
impl Default for ACMFORMATTAGDETAILSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACMFORMATTAGDETAILSW {
    pub cbStruct: super::DWORD,
    pub dwFormatTagIndex: super::DWORD,
    pub dwFormatTag: super::DWORD,
    pub cbFormatSize: super::DWORD,
    pub fdwSupport: super::DWORD,
    pub cStandardFormats: super::DWORD,
    pub szFormatTag: [super::WCHAR; 48],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ACMFORMATTAGDETAILSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: i32 = 48;
#[cfg(feature = "shared")]
pub type ACMFORMATTAGENUMCBA = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: LPACMFORMATTAGDETAILSA, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type ACMFORMATTAGENUMCBW = Option<unsafe extern "system" fn(hadid: HACMDRIVERID, paftd: LPACMFORMATTAGDETAILSW, dwinstance: super::DWORD_PTR, fdwsupport: super::DWORD) -> windows_sys::core::BOOL>;
pub const ACMHELPMSGCONTEXTHELP: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTHELPW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contexthelp");
pub const ACMHELPMSGCONTEXTMENU: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_contextmenu");
pub const ACMHELPMSGCONTEXTMENUW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_contextmenu");
pub const ACMHELPMSGSTRING: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("acmchoose_help");
pub const ACMHELPMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("acmchoose_help");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: super::DWORD,
    pub fdwStatus: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub pbSrc: super::LPBYTE,
    pub cbSrcLength: super::DWORD,
    pub cbSrcLengthUsed: super::DWORD,
    pub dwSrcUser: super::DWORD_PTR,
    pub pbDst: super::LPBYTE,
    pub cbDstLength: super::DWORD,
    pub cbDstLengthUsed: super::DWORD,
    pub dwDstUser: super::DWORD_PTR,
    pub dwReservedDriver: [super::DWORD; 10],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ACMSTREAMHEADER {
    pub cbStruct: super::DWORD,
    pub fdwStatus: super::DWORD,
    pub dwUser: super::DWORD_PTR,
    pub pbSrc: super::LPBYTE,
    pub cbSrcLength: super::DWORD,
    pub cbSrcLengthUsed: super::DWORD,
    pub dwSrcUser: super::DWORD_PTR,
    pub pbDst: super::LPBYTE,
    pub cbDstLength: super::DWORD,
    pub cbDstLengthUsed: super::DWORD,
    pub dwDstUser: super::DWORD_PTR,
    pub dwReservedDriver: [super::DWORD; 15],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for ACMSTREAMHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536;
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576;
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072;
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3;
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8;
pub const ACM_DRIVERADDF_LOCAL: i32 = 0;
pub const ACM_DRIVERADDF_NAME: i32 = 1;
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4;
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7;
pub const ACM_DRIVERENUMF_DISABLED: u32 = 2147483648;
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824;
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3;
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536;
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608;
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2;
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1;
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072;
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1;
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0;
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15;
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536;
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1;
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0;
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2;
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15;
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1;
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0;
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15;
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576;
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304;
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608;
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072;
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144;
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216;
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152;
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288;
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536;
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072;
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144;
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680;
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288;
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536;
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1;
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0;
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2;
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15;
pub const ACM_METRIC_COUNT_CODECS: i32 = 2;
pub const ACM_METRIC_COUNT_CONVERTERS: i32 = 3;
pub const ACM_METRIC_COUNT_DISABLED: i32 = 5;
pub const ACM_METRIC_COUNT_DRIVERS: i32 = 1;
pub const ACM_METRIC_COUNT_FILTERS: i32 = 4;
pub const ACM_METRIC_COUNT_HARDWARE: i32 = 6;
pub const ACM_METRIC_COUNT_LOCAL_CODECS: i32 = 21;
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: i32 = 22;
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: i32 = 24;
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: i32 = 20;
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: i32 = 23;
pub const ACM_METRIC_DRIVER_PRIORITY: i32 = 101;
pub const ACM_METRIC_DRIVER_SUPPORT: i32 = 100;
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: i32 = 30;
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: i32 = 31;
pub const ACM_METRIC_MAX_SIZE_FILTER: i32 = 51;
pub const ACM_METRIC_MAX_SIZE_FORMAT: i32 = 50;
pub const ACM_STREAMCONVERTF_BLOCKALIGN: i32 = 4;
pub const ACM_STREAMCONVERTF_END: i32 = 32;
pub const ACM_STREAMCONVERTF_START: i32 = 16;
pub const ACM_STREAMOPENF_ASYNC: i32 = 2;
pub const ACM_STREAMOPENF_NONREALTIME: i32 = 4;
pub const ACM_STREAMOPENF_QUERY: i32 = 1;
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1;
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15;
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0;
pub const DRVM_MAPPER: i32 = 8192;
pub const DRVM_MAPPER_STATUS: i32 = 8192;
pub const DRV_MAPPER_PREFERRED_INPUT_GET: i32 = 16384;
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: i32 = 16386;
pub const FILTERCHOOSE_CUSTOM_VERIFY: i32 = 2;
pub const FILTERCHOOSE_FILTERTAG_VERIFY: i32 = 0;
pub const FILTERCHOOSE_FILTER_VERIFY: i32 = 1;
pub const FILTERCHOOSE_MESSAGE: i32 = 0;
pub const FORMATCHOOSE_CUSTOM_VERIFY: i32 = 2;
pub const FORMATCHOOSE_FORMATTAG_VERIFY: i32 = 0;
pub const FORMATCHOOSE_FORMAT_VERIFY: i32 = 1;
pub const FORMATCHOOSE_MESSAGE: i32 = 0;
pub type HACMDRIVER = *mut HACMDRIVER__;
pub type HACMDRIVERID = *mut HACMDRIVERID__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HACMDRIVERID__ {
    pub unused: i32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HACMDRIVER__ {
    pub unused: i32,
}
pub type HACMOBJ = *mut HACMOBJ__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HACMOBJ__ {
    pub unused: i32,
}
pub type HACMSTREAM = *mut HACMSTREAM__;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct HACMSTREAM__ {
    pub unused: i32,
}
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef"))]
pub type LPACMDRIVERDETAILSA = *mut ACMDRIVERDETAILSA;
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPACMDRIVERDETAILSW = *mut ACMDRIVERDETAILSW;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPACMDRIVERPROC = *mut ACMDRIVERPROC;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef"))]
pub type LPACMFILTERCHOOSEA = *mut ACMFILTERCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPACMFILTERCHOOSEW = *mut ACMFILTERCHOOSEW;
#[cfg(all(feature = "mmreg", feature = "shared"))]
pub type LPACMFILTERDETAILSA = *mut ACMFILTERDETAILSA;
#[cfg(all(feature = "mmreg", feature = "shared", feature = "winnt"))]
pub type LPACMFILTERDETAILSW = *mut ACMFILTERDETAILSW;
#[cfg(feature = "shared")]
pub type LPACMFILTERTAGDETAILSA = *mut ACMFILTERTAGDETAILSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPACMFILTERTAGDETAILSW = *mut ACMFILTERTAGDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef"))]
pub type LPACMFORMATCHOOSEA = *mut ACMFORMATCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPACMFORMATCHOOSEW = *mut ACMFORMATCHOOSEW;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type LPACMFORMATDETAILSA = *mut ACMFORMATDETAILSA;
#[cfg(all(feature = "mmeapi", feature = "shared", feature = "winnt"))]
pub type LPACMFORMATDETAILSW = *mut ACMFORMATDETAILSW;
#[cfg(feature = "shared")]
pub type LPACMFORMATTAGDETAILSA = *mut ACMFORMATTAGDETAILSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPACMFORMATTAGDETAILSW = *mut ACMFORMATTAGDETAILSW;
#[cfg(feature = "shared")]
pub type LPACMSTREAMHEADER = *mut ACMSTREAMHEADER;
pub type LPHACMDRIVER = *mut *mut HACMDRIVER__;
pub type LPHACMDRIVERID = *mut *mut HACMDRIVERID__;
pub type LPHACMOBJ = *mut *mut HACMOBJ__;
pub type LPHACMSTREAM = *mut *mut HACMSTREAM__;
pub const MM_ACM_CLOSE: i32 = 981;
pub const MM_ACM_DONE: i32 = 982;
pub const MM_ACM_FILTERCHOOSE: i32 = 32768;
pub const MM_ACM_FORMATCHOOSE: i32 = 32768;
pub const MM_ACM_OPEN: i32 = 980;
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef"))]
pub type PACMDRIVERDETAILSA = *mut ACMDRIVERDETAILSA;
#[cfg(all(feature = "mmiscapi", feature = "shared", feature = "windef", feature = "winnt"))]
pub type PACMDRIVERDETAILSW = *mut ACMDRIVERDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef"))]
pub type PACMFILTERCHOOSEA = *mut ACMFILTERCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmreg", feature = "shared", feature = "windef", feature = "winnt"))]
pub type PACMFILTERCHOOSEW = *mut ACMFILTERCHOOSEW;
#[cfg(all(feature = "mmreg", feature = "shared"))]
pub type PACMFILTERDETAILSA = *mut ACMFILTERDETAILSA;
#[cfg(all(feature = "mmreg", feature = "shared", feature = "winnt"))]
pub type PACMFILTERDETAILSW = *mut ACMFILTERDETAILSW;
#[cfg(feature = "shared")]
pub type PACMFILTERTAGDETAILSA = *mut ACMFILTERTAGDETAILSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACMFILTERTAGDETAILSW = *mut ACMFILTERTAGDETAILSW;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef"))]
pub type PACMFORMATCHOOSEA = *mut ACMFORMATCHOOSEA;
#[cfg(all(feature = "minwindef", feature = "mmeapi", feature = "shared", feature = "windef", feature = "winnt"))]
pub type PACMFORMATCHOOSEW = *mut ACMFORMATCHOOSEW;
#[cfg(all(feature = "mmeapi", feature = "shared"))]
pub type PACMFORMATDETAILSA = *mut ACMFORMATDETAILSA;
#[cfg(all(feature = "mmeapi", feature = "shared", feature = "winnt"))]
pub type PACMFORMATDETAILSW = *mut ACMFORMATDETAILSW;
#[cfg(feature = "shared")]
pub type PACMFORMATTAGDETAILSA = *mut ACMFORMATTAGDETAILSA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACMFORMATTAGDETAILSW = *mut ACMFORMATTAGDETAILSW;
#[cfg(feature = "shared")]
pub type PACMSTREAMHEADER = *mut ACMSTREAMHEADER;
pub type PHACMDRIVER = *mut *mut HACMDRIVER__;
pub type PHACMDRIVERID = *mut *mut HACMDRIVERID__;
pub type PHACMOBJ = *mut *mut HACMOBJ__;
pub type PHACMSTREAM = *mut *mut HACMSTREAM__;
pub const WAVEIN_MAPPER_STATUS_DEVICE: i32 = 0;
pub const WAVEIN_MAPPER_STATUS_FORMAT: i32 = 2;
pub const WAVEIN_MAPPER_STATUS_MAPPED: i32 = 1;
pub const WAVEOUT_MAPPER_STATUS_DEVICE: i32 = 0;
pub const WAVEOUT_MAPPER_STATUS_FORMAT: i32 = 2;
pub const WAVEOUT_MAPPER_STATUS_MAPPED: i32 = 1;
pub const WIDM_MAPPER_STATUS: i32 = 8192;
pub const WODM_MAPPER_STATUS: i32 = 8192;
#[cfg(target_arch = "x86")]
pub const _DRVRESERVED: i32 = 10;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const _DRVRESERVED: i32 = 15;
