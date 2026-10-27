#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EmptyWorkingSet(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EmptyWorkingSet" fn EmptyWorkingSet(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { EmptyWorkingSet(hprocess) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumDeviceDrivers(lpimagebase: *mut super::LPVOID, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumDeviceDrivers" fn EnumDeviceDrivers(lpimagebase : *mut super::LPVOID, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumPageFilesA" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : super::LPVOID) -> windows_core::BOOL);
    unsafe { EnumPageFilesA(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumPageFilesW" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : super::LPVOID) -> windows_core::BOOL);
    unsafe { EnumPageFilesW(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumProcessModules(hprocess: super::HANDLE, lphmodule: *mut super::HMODULE, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcessModules" fn EnumProcessModules(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumProcessModulesEx(hprocess: super::HANDLE, lphmodule: *mut super::HMODULE, cb: super::DWORD, lpcbneeded: super::LPDWORD, dwfilterflag: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcessModulesEx" fn EnumProcessModulesEx(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD, dwfilterflag : super::DWORD) -> windows_core::BOOL);
    unsafe { EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumProcesses(lpidprocess: *mut super::DWORD, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcesses" fn EnumProcesses(lpidprocess : *mut super::DWORD, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { EnumProcesses(lpidprocess as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetDeviceDriverBaseNameA(imagebase: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameA" fn GetDeviceDriverBaseNameA(imagebase : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetDeviceDriverBaseNameA(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetDeviceDriverBaseNameW(imagebase: super::LPVOID, lpbasename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameW" fn GetDeviceDriverBaseNameW(imagebase : super::LPVOID, lpbasename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetDeviceDriverBaseNameW(imagebase, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetDeviceDriverFileNameA(imagebase: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameA" fn GetDeviceDriverFileNameA(imagebase : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetDeviceDriverFileNameA(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetDeviceDriverFileNameW(imagebase: super::LPVOID, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameW" fn GetDeviceDriverFileNameW(imagebase : super::LPVOID, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetDeviceDriverFileNameW(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetMappedFileNameA(hprocess: super::HANDLE, lpv: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetMappedFileNameA" fn GetMappedFileNameA(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetMappedFileNameA(hprocess, lpv, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetMappedFileNameW(hprocess: super::HANDLE, lpv: super::LPVOID, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetMappedFileNameW" fn GetMappedFileNameW(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetMappedFileNameW(hprocess, lpv, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleBaseNameA(hprocess: super::HANDLE, hmodule: Option<super::HMODULE>, lpbasename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleBaseNameA" fn GetModuleBaseNameA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleBaseNameW(hprocess: super::HANDLE, hmodule: Option<super::HMODULE>, lpbasename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleBaseNameW" fn GetModuleBaseNameW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleFileNameExA(hprocess: Option<super::HANDLE>, hmodule: Option<super::HMODULE>, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleFileNameExA" fn GetModuleFileNameExA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleFileNameExW(hprocess: Option<super::HANDLE>, hmodule: Option<super::HMODULE>, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleFileNameExW" fn GetModuleFileNameExW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleInformation(hprocess: super::HANDLE, hmodule: super::HMODULE, lpmodinfo: LPMODULEINFO, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleInformation" fn GetModuleInformation(hprocess : super::HANDLE, hmodule : super::HMODULE, lpmodinfo : LPMODULEINFO, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPerformanceInfo(pperformanceinformation: PPERFORMANCE_INFORMATION, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetPerformanceInfo" fn GetPerformanceInfo(pperformanceinformation : PPERFORMANCE_INFORMATION, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { GetPerformanceInfo(pperformanceinformation as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessImageFileNameA(hprocess: super::HANDLE, lpimagefilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessImageFileNameA" fn GetProcessImageFileNameA(hprocess : super::HANDLE, lpimagefilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProcessImageFileNameA(hprocess, lpimagefilename, nsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessImageFileNameW(hprocess: super::HANDLE, lpimagefilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessImageFileNameW" fn GetProcessImageFileNameW(hprocess : super::HANDLE, lpimagefilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProcessImageFileNameW(hprocess, lpimagefilename, nsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessMemoryInfo(process: super::HANDLE, ppsmemcounters: PPROCESS_MEMORY_COUNTERS, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessMemoryInfo" fn GetProcessMemoryInfo(process : super::HANDLE, ppsmemcounters : PPROCESS_MEMORY_COUNTERS, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetWsChanges(hprocess: super::HANDLE, lpwatchinfo: PPSAPI_WS_WATCH_INFORMATION, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetWsChanges" fn GetWsChanges(hprocess : super::HANDLE, lpwatchinfo : PPSAPI_WS_WATCH_INFORMATION, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { GetWsChanges(hprocess, lpwatchinfo as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetWsChangesEx(hprocess: super::HANDLE, lpwatchinfoex: PPSAPI_WS_WATCH_INFORMATION_EX, cb: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetWsChangesEx" fn GetWsChangesEx(hprocess : super::HANDLE, lpwatchinfoex : PPSAPI_WS_WATCH_INFORMATION_EX, cb : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeProcessForWsWatch(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32InitializeProcessForWsWatch" fn InitializeProcessForWsWatch(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { InitializeProcessForWsWatch(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32EmptyWorkingSet(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EmptyWorkingSet(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { K32EmptyWorkingSet(hprocess) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut super::LPVOID, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumDeviceDrivers(lpimagebase : *mut super::LPVOID, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { K32EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : super::LPVOID) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesA(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : super::LPVOID) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesW(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32EnumProcessModules(hprocess: super::HANDLE, lphmodule: *mut super::HMODULE, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcessModules(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { K32EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32EnumProcessModulesEx(hprocess: super::HANDLE, lphmodule: *mut super::HMODULE, cb: super::DWORD, lpcbneeded: super::LPDWORD, dwfilterflag: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcessModulesEx(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD, dwfilterflag : super::DWORD) -> windows_core::BOOL);
    unsafe { K32EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut super::DWORD, cb: super::DWORD, lpcbneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess : *mut super::DWORD, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { K32EnumProcesses(lpidprocess as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameA(imagebase : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetDeviceDriverBaseNameA(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: super::LPVOID, lpbasename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameW(imagebase : super::LPVOID, lpbasename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetDeviceDriverBaseNameW(imagebase, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameA(imagebase : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetDeviceDriverFileNameA(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: super::LPVOID, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameW(imagebase : super::LPVOID, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetDeviceDriverFileNameW(imagebase, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetMappedFileNameA(hprocess: super::HANDLE, lpv: super::LPVOID, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetMappedFileNameA(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetMappedFileNameA(hprocess, lpv, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetMappedFileNameW(hprocess: super::HANDLE, lpv: super::LPVOID, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetMappedFileNameW(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetMappedFileNameW(hprocess, lpv, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleBaseNameA(hprocess: super::HANDLE, hmodule: Option<super::HMODULE>, lpbasename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleBaseNameA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleBaseNameW(hprocess: super::HANDLE, hmodule: Option<super::HMODULE>, lpbasename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleBaseNameW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, lpbasename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleFileNameExA(hprocess: Option<super::HANDLE>, hmodule: Option<super::HMODULE>, lpfilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleFileNameExA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleFileNameExW(hprocess: Option<super::HANDLE>, hmodule: Option<super::HMODULE>, lpfilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleFileNameExW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleInformation(hprocess: super::HANDLE, hmodule: super::HMODULE, lpmodinfo: LPMODULEINFO, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetModuleInformation(hprocess : super::HANDLE, hmodule : super::HMODULE, lpmodinfo : LPMODULEINFO, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: PPERFORMANCE_INFORMATION, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetPerformanceInfo(pperformanceinformation : PPERFORMANCE_INFORMATION, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32GetPerformanceInfo(pperformanceinformation as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetProcessImageFileNameA(hprocess: super::HANDLE, lpimagefilename: windows_core::PSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameA(hprocess : super::HANDLE, lpimagefilename : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetProcessImageFileNameA(hprocess, lpimagefilename, nsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetProcessImageFileNameW(hprocess: super::HANDLE, lpimagefilename: windows_core::PWSTR, nsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameW(hprocess : super::HANDLE, lpimagefilename : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { K32GetProcessImageFileNameW(hprocess, lpimagefilename, nsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetProcessMemoryInfo(process: super::HANDLE, ppsmemcounters: PPROCESS_MEMORY_COUNTERS, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetProcessMemoryInfo(process : super::HANDLE, ppsmemcounters : PPROCESS_MEMORY_COUNTERS, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetWsChanges(hprocess: super::HANDLE, lpwatchinfo: PPSAPI_WS_WATCH_INFORMATION, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetWsChanges(hprocess : super::HANDLE, lpwatchinfo : PPSAPI_WS_WATCH_INFORMATION, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32GetWsChanges(hprocess, lpwatchinfo as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetWsChangesEx(hprocess: super::HANDLE, lpwatchinfoex: PPSAPI_WS_WATCH_INFORMATION_EX, cb: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32GetWsChangesEx(hprocess : super::HANDLE, lpwatchinfoex : PPSAPI_WS_WATCH_INFORMATION_EX, cb : super::PDWORD) -> windows_core::BOOL);
    unsafe { K32GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch(hprocess: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32InitializeProcessForWsWatch(hprocess : super::HANDLE) -> windows_core::BOOL);
    unsafe { K32InitializeProcessForWsWatch(hprocess) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32QueryWorkingSet(hprocess: super::HANDLE, pv: super::PVOID, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32QueryWorkingSet(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSet(hprocess, pv as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn K32QueryWorkingSetEx(hprocess: super::HANDLE, pv: super::PVOID, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn K32QueryWorkingSetEx(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSetEx(hprocess, pv as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryWorkingSet(hprocess: super::HANDLE, pv: super::PVOID, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32QueryWorkingSet" fn QueryWorkingSet(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { QueryWorkingSet(hprocess, pv as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryWorkingSetEx(hprocess: super::HANDLE, pv: super::PVOID, cb: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32QueryWorkingSetEx" fn QueryWorkingSetEx(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { QueryWorkingSetEx(hprocess, pv as _, cb) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: super::DWORD,
    pub Reserved: super::DWORD,
    pub TotalSize: super::SIZE_T,
    pub TotalInUse: super::SIZE_T,
    pub PeakUsage: super::SIZE_T,
}
pub const LIST_MODULES_32BIT: i32 = 1;
pub const LIST_MODULES_64BIT: i32 = 2;
pub const LIST_MODULES_ALL: i32 = 3;
pub const LIST_MODULES_DEFAULT: i32 = 0;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMODULEINFO = *mut MODULEINFO;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODULEINFO {
    pub lpBaseOfDll: super::LPVOID,
    pub SizeOfImage: super::DWORD,
    pub EntryPoint: super::LPVOID,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: super::LPVOID, ppagefileinfo: PENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCSTR) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: super::LPVOID, ppagefileinfo: PENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCWSTR) -> windows_core::BOOL>;
#[cfg(feature = "shared")]
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;
#[cfg(feature = "shared")]
pub type PERFORMACE_INFORMATION = _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PERFORMANCE_INFORMATION = _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPERFORMACE_INFORMATION = *mut _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPERFORMANCE_INFORMATION = *mut _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS_EX2 = *mut PROCESS_MEMORY_COUNTERS_EX2;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_BLOCK = *mut PSAPI_WORKING_SET_BLOCK;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_EX_BLOCK = *mut PSAPI_WORKING_SET_EX_BLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPSAPI_WORKING_SET_EX_INFORMATION = *mut PSAPI_WORKING_SET_EX_INFORMATION;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_INFORMATION = *mut PSAPI_WORKING_SET_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PPSAPI_WS_WATCH_INFORMATION = *mut PSAPI_WS_WATCH_INFORMATION;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPSAPI_WS_WATCH_INFORMATION_EX = *mut PSAPI_WS_WATCH_INFORMATION_EX;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivateUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX2 {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivateUsage: super::SIZE_T,
    pub PrivateWorkingSetSize: super::SIZE_T,
    pub SharedCommitUsage: super::ULONG64,
}
pub const PSAPI_VERSION: i32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_BLOCK_0 {
    pub fn Protection(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 5, "bitfield exceeds target member width");
        ((self._bitfield) & 31) as usize
    }
    pub fn set_Protection(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 5, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !31) | ((value as u32) & 31);
    }
    pub fn ShareCount(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 5) & 7) as usize
    }
    pub fn set_ShareCount(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 5)) | (((value as u32) & 7) << 5);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 8) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 8)) | (((value as u32) & 1) << 8);
    }
    pub fn Reserved(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 9) & 7) as usize
    }
    pub fn set_Reserved(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 9)) | (((value as u32) & 7) << 9);
    }
    pub fn VirtualPage(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 20, "bitfield exceeds target member width");
        (self._bitfield >> 12) as usize
    }
    pub fn set_VirtualPage(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 20, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1048575 << 12)) | (((value as u32) & 1048575) << 12);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_BLOCK_0 {
    pub fn Protection(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 5, "bitfield exceeds target member width");
        ((self._bitfield) & 31) as usize
    }
    pub fn set_Protection(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 5, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !31) | ((value as u64) & 31);
    }
    pub fn ShareCount(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 5) & 7) as usize
    }
    pub fn set_ShareCount(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 5)) | (((value as u64) & 7) << 5);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 8) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 8)) | (((value as u64) & 1) << 8);
    }
    pub fn Reserved(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 9) & 7) as usize
    }
    pub fn set_Reserved(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 9)) | (((value as u64) & 7) << 9);
    }
    pub fn VirtualPage(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 52, "bitfield exceeds target member width");
        (self._bitfield >> 12) as usize
    }
    pub fn set_VirtualPage(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 52, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(4503599627370495 << 12)) | (((value as u64) & 4503599627370495) << 12);
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub fn Valid(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield) & 1) as usize
    }
    pub fn set_Valid(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !1) | ((value as u32) & 1);
    }
    pub fn ShareCount(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 1) & 7) as usize
    }
    pub fn set_ShareCount(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 1)) | (((value as u32) & 7) << 1);
    }
    pub fn Win32Protection(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 11, "bitfield exceeds target member width");
        ((self._bitfield >> 4) & 2047) as usize
    }
    pub fn set_Win32Protection(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 11, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(2047 << 4)) | (((value as u32) & 2047) << 4);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 15) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 15)) | (((value as u32) & 1) << 15);
    }
    pub fn Node(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 6, "bitfield exceeds target member width");
        ((self._bitfield >> 16) & 63) as usize
    }
    pub fn set_Node(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 6, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(63 << 16)) | (((value as u32) & 63) << 16);
    }
    pub fn Locked(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 22) & 1) as usize
    }
    pub fn set_Locked(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 22)) | (((value as u32) & 1) << 22);
    }
    pub fn LargePage(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 23) & 1) as usize
    }
    pub fn set_LargePage(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 23)) | (((value as u32) & 1) << 23);
    }
    pub fn Reserved(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 7, "bitfield exceeds target member width");
        ((self._bitfield >> 24) & 127) as usize
    }
    pub fn set_Reserved(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 7, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(127 << 24)) | (((value as u32) & 127) << 24);
    }
    pub fn Bad(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        (self._bitfield >> 31) as usize
    }
    pub fn set_Bad(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 31)) | (((value as u32) & 1) << 31);
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub fn Valid(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield) & 1) as usize
    }
    pub fn set_Valid(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !1) | ((value as u32) & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 14, "bitfield exceeds target member width");
        ((self._bitfield >> 1) & 16383) as usize
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 14, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(16383 << 1)) | (((value as u32) & 16383) << 1);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 15) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 15)) | (((value as u32) & 1) << 15);
    }
    pub fn Reserved1(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 15, "bitfield exceeds target member width");
        ((self._bitfield >> 16) & 32767) as usize
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 15, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(32767 << 16)) | (((value as u32) & 32767) << 16);
    }
    pub fn Bad(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        (self._bitfield >> 31) as usize
    }
    pub fn set_Bad(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 31)) | (((value as u32) & 1) << 31);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub fn Valid(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield) & 1) as usize
    }
    pub fn set_Valid(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !1) | ((value as u64) & 1);
    }
    pub fn ShareCount(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        ((self._bitfield >> 1) & 7) as usize
    }
    pub fn set_ShareCount(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 3, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(7 << 1)) | (((value as u64) & 7) << 1);
    }
    pub fn Win32Protection(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 11, "bitfield exceeds target member width");
        ((self._bitfield >> 4) & 2047) as usize
    }
    pub fn set_Win32Protection(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 11, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(2047 << 4)) | (((value as u64) & 2047) << 4);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 15) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 15)) | (((value as u64) & 1) << 15);
    }
    pub fn Node(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 6, "bitfield exceeds target member width");
        ((self._bitfield >> 16) & 63) as usize
    }
    pub fn set_Node(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 6, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(63 << 16)) | (((value as u64) & 63) << 16);
    }
    pub fn Locked(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 22) & 1) as usize
    }
    pub fn set_Locked(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 22)) | (((value as u64) & 1) << 22);
    }
    pub fn LargePage(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 23) & 1) as usize
    }
    pub fn set_LargePage(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 23)) | (((value as u64) & 1) << 23);
    }
    pub fn Reserved(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 7, "bitfield exceeds target member width");
        ((self._bitfield >> 24) & 127) as usize
    }
    pub fn set_Reserved(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 7, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(127 << 24)) | (((value as u64) & 127) << 24);
    }
    pub fn Bad(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 31) & 1) as usize
    }
    pub fn set_Bad(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 31)) | (((value as u64) & 1) << 31);
    }
    pub fn ReservedUlong(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target member width");
        (self._bitfield >> 32) as usize
    }
    pub fn set_ReservedUlong(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(4294967295 << 32)) | (((value as u64) & 4294967295) << 32);
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub fn Valid(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield) & 1) as usize
    }
    pub fn set_Valid(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !1) | ((value as u64) & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 14, "bitfield exceeds target member width");
        ((self._bitfield >> 1) & 16383) as usize
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 14, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(16383 << 1)) | (((value as u64) & 16383) << 1);
    }
    pub fn Shared(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 15) & 1) as usize
    }
    pub fn set_Shared(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 15)) | (((value as u64) & 1) << 15);
    }
    pub fn Reserved1(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 15, "bitfield exceeds target member width");
        ((self._bitfield >> 16) & 32767) as usize
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 15, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(32767 << 16)) | (((value as u64) & 32767) << 16);
    }
    pub fn Bad(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        ((self._bitfield >> 31) & 1) as usize
    }
    pub fn set_Bad(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 1, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(1 << 31)) | (((value as u64) & 1) << 31);
    }
    pub fn ReservedUlong(&self) -> super::ULONG_PTR {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target member width");
        (self._bitfield >> 32) as usize
    }
    pub fn set_ReservedUlong(&mut self, value: super::ULONG_PTR) {
        const _: () = assert!(usize::BITS >= 32, "bitfield exceeds target member width");
        self._bitfield = (self._bitfield & !(4294967295 << 32)) | (((value as u64) & 4294967295) << 32);
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: super::PVOID,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: super::ULONG_PTR,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: super::LPVOID,
    pub FaultingVa: super::LPVOID,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: super::ULONG_PTR,
    pub Flags: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _PERFORMANCE_INFORMATION {
    pub cb: super::DWORD,
    pub CommitTotal: super::SIZE_T,
    pub CommitLimit: super::SIZE_T,
    pub CommitPeak: super::SIZE_T,
    pub PhysicalTotal: super::SIZE_T,
    pub PhysicalAvailable: super::SIZE_T,
    pub SystemCache: super::SIZE_T,
    pub KernelTotal: super::SIZE_T,
    pub KernelPaged: super::SIZE_T,
    pub KernelNonpaged: super::SIZE_T,
    pub PageSize: super::SIZE_T,
    pub HandleCount: super::DWORD,
    pub ProcessCount: super::DWORD,
    pub ThreadCount: super::DWORD,
}
