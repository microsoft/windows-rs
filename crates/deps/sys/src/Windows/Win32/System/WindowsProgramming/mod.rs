#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryA(lpcszfilelist: super::super::Foundation::PSTR, lpcszbackupdir: super::super::Foundation::PSTR, lpcszbasename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryW(lpcszfilelist: super::super::Foundation::PWSTR, lpcszbackupdir: super::super::Foundation::PWSTR, lpcszbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileA(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PSTR, lpszsourcefile: super::super::Foundation::PSTR, lpszdestdir: super::super::Foundation::PSTR, lpszdestfile: super::super::Foundation::PSTR, dwflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileW(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, lpszdestdir: super::super::Foundation::PWSTR, lpszdestfile: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApphelpCheckShellObject(objectclsid: *const ::windows::runtime::GUID, bshimifnecessary: super::super::Foundation::BOOL, pullflags: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDeviceWakeupRequest(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn CloseINFEngine(hinf: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICloseProvider(hdc: super::super::Graphics::Gdi::HDC);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOffscreen(hdc: super::super::Graphics::Gdi::HDC, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOverlay(hdc: super::super::Graphics::Gdi::HDC, lpoffscreensurf: *mut ::core::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreatePrimary(hdc: super::super::Graphics::Gdi::HDC, lplpsurface: *mut *mut DCISURFACEINFO) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIDestroy(pdci: *mut DCISURFACEINFO);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIEndAccess(pdci: *mut DCISURFACEINFO);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCIEnum(hdc: super::super::Graphics::Gdi::HDC, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeA(pszfileordirname: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeRunDLL32W(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeW(pszfileordirname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameA(hostname: super::super::Foundation::PSTR, computername: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabA(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOA, preserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabW(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOW, preserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesA(pszcabname: super::super::Foundation::PSTR, pszexpanddir: super::super::Foundation::PSTR, dwflags: u32, pszfilelist: super::super::Foundation::PSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesW(pszcabname: super::super::Foundation::PWSTR, pszexpanddir: super::super::Foundation::PWSTR, dwflags: u32, pszfilelist: super::super::Foundation::PWSTR, lpreserved: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistA(lpfilelist: super::super::Foundation::PSTR, lpdir: super::super::Foundation::PSTR, lpbasename: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistW(lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, pszbackupdir: super::super::Foundation::PSTR, pszbasebackupfile: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, pszbackupdir: super::super::Foundation::PWSTR, pszbasebackupfile: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreW(hdlg: super::super::Foundation::HWND, lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GdiEntry13() -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameA(lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameW(lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDCRegionData(hdc: super::super::Graphics::Gdi::HDC, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::core::ffi::c_void, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesA(lpszreturnbuffer: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesW(lpszreturnbuffer: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *mut ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetThreadEnabledXStateFeatures() -> u64;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameA(lpbuffer: super::super::Foundation::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameW(lpbuffer: super::super::Foundation::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetWindowRegionData(hwnd: super::super::Foundation::HWND, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalCompact(dwminfree: u32) -> usize;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalFix(hmem: isize);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalUnfix(hmem: isize);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalWire(hmem: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsApiSetImplemented(contract: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenUntrusted(tokenhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionExW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionW(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparams: super::super::Foundation::PWSTR, nshow: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn LocalCompact(uminfree: u32) -> usize;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn NeedRebootInit() -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtClose(handle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtDeviceIoControlFile(filehandle: super::super::Foundation::HANDLE, event: super::super::Foundation::HANDLE, apcroutine: ::windows::runtime::RawPtr, apccontext: *mut ::core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::core::ffi::c_void, outputbufferlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtNotifyChangeMultipleKeys(
        masterkeyhandle: super::super::Foundation::HANDLE,
        count: u32,
        subordinateobjects: *const OBJECT_ATTRIBUTES,
        event: super::super::Foundation::HANDLE,
        apcroutine: ::windows::runtime::RawPtr,
        apccontext: *const ::core::ffi::c_void,
        iostatusblock: *mut IO_STATUS_BLOCK,
        completionfilter: u32,
        watchtree: super::super::Foundation::BOOLEAN,
        buffer: *mut ::core::ffi::c_void,
        buffersize: u32,
        asynchronous: super::super::Foundation::BOOLEAN,
    ) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryMultipleValueKey(keyhandle: super::super::Foundation::HANDLE, valueentries: *mut KEY_VALUE_ENTRY, entrycount: u32, valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryObject(handle: super::super::Foundation::HANDLE, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::core::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemTime(systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtRenameKey(keyhandle: super::super::Foundation::HANDLE, newname: *const super::super::Foundation::UNICODE_STRING) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationKey(keyhandle: super::super::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtWaitForSingleObject(handle: super::super::Foundation::HANDLE, alertable: super::super::Foundation::BOOLEAN, timeout: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, dwflags: u32, phinf: *mut *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryInterruptTime(lpinterrupttime: *mut u64);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessCycleTime(processhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadCycleTime(threadhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallA(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PSTR, pszsec: super::super::Foundation::PSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallW(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PWSTR, pszsec: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureUsage(featureid: u32, kind: u32, addend: u32, originname: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallA(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PSTR, psttable: *const STRTABLEA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallW(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PWSTR, psttable: *const STRTABLEW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PSTR, pcszsubkey: super::super::Foundation::PSTR, pcszvaluename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PWSTR, pcszsubkey: super::super::Foundation::PWSTR, pcszvaluename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplacePartitionUnit(targetpartition: super::super::Foundation::PWSTR, sparepartition: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RequestDeviceWakeup(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlAnsiStringToUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn RtlGetReturnAddressHijackTarget() -> usize;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInitUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToOemString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnicodeToMultiByteSize(bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn RtlUniform(seed: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandA(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PSTR, szinfsection: super::super::Foundation::PSTR, szdir: super::super::Foundation::PSTR, lpsztitle: super::super::Foundation::PSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandW(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PWSTR, szinfsection: super::super::Foundation::PWSTR, szdir: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentStringsA(newenvironment: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::core::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn SetHandleCount(unumber: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageWaitingIndicator(hmsgindicator: super::super::Foundation::HANDLE, ulmsgcount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignalObjectAndWait(hobjecttosignal: super::super::Foundation::HANDLE, hobjecttowaiton: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExA(hinf: *mut ::core::ffi::c_void, pszinffilename: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExW(hinf: *mut ::core::ffi::c_void, pszinffilename: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn UnsubscribeFeatureStateChangeNotification(subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSEnableIME(param0: super::super::Foundation::HWND, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetEnableStatus(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetIMEHotkey(param0: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn WinWatchClose(hww: HWINWATCH);
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchDidStatusChange(hww: HWINWATCH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn WinWatchGetClipList(hww: HWINWATCH, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchNotify(hww: HWINWATCH, notifycallback: ::windows::runtime::RawPtr, notifyparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchOpen(hwnd: super::super::Foundation::HWND) -> HWINWATCH;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsClassInApprovedList(classid: *const ::windows::runtime::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsDynamicCodePolicyEnabled(isenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDeviceSecurityInformation(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::core::ffi::c_void, imagesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpSetDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *const ::core::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _hread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, lbytes: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _hwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, lbytes: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _lclose(hfile: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lcreat(lppathname: super::super::Foundation::PSTR, iattribute: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lopen(lppathname: super::super::Foundation::PSTR, ireadwrite: i32) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _lread(hfile: i32, lpbuffer: *mut ::core::ffi::c_void, ubytes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, ubytes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrlenW(string: *const u16) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcslen(string: *const u16) -> usize;
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16;
}
