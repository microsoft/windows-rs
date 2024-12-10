#[inline]
pub unsafe fn EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    EmptyWorkingSet(hprocess).ok()
}
#[inline]
pub unsafe fn EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumDeviceDrivers(core::mem::transmute(lpimagebase), cb, core::mem::transmute(lpcbneeded)).ok()
}
#[inline]
pub unsafe fn EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    EnumPageFilesA(pcallbackroutine, core::mem::transmute(pcontext)).ok()
}
#[inline]
pub unsafe fn EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    EnumPageFilesW(pcallbackroutine, core::mem::transmute(pcontext)).ok()
}
#[inline]
pub unsafe fn EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumProcessModules(hprocess, core::mem::transmute(lphmodule), cb, core::mem::transmute(lpcbneeded)).ok()
}
#[inline]
pub unsafe fn EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation:: BOOL);
    EnumProcessModulesEx(hprocess, core::mem::transmute(lphmodule), cb, core::mem::transmute(lpcbneeded), dwfilterflag).ok()
}
#[inline]
pub unsafe fn EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    EnumProcesses(core::mem::transmute(lpidprocess), cb, core::mem::transmute(lpcbneeded)).ok()
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    GetModuleBaseNameA(hprocess, core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    GetModuleBaseNameW(hprocess, core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleFileNameExA(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    GetModuleFileNameExA(core::mem::transmute(hprocess.unwrap_or(core::mem::zeroed())), core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleFileNameExW(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetModuleFileNameExW(core::mem::transmute(hprocess.unwrap_or(core::mem::zeroed())), core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> super::super::Foundation:: BOOL);
    GetModuleInformation(hprocess, hmodule, core::mem::transmute(lpmodinfo), cb).ok()
}
#[inline]
pub unsafe fn GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    GetPerformanceInfo(core::mem::transmute(pperformanceinformation), cb).ok()
}
#[inline]
pub unsafe fn GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_targets::link!("psapi.dll" "system" fn GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> super::super::Foundation:: BOOL);
    GetProcessMemoryInfo(process, core::mem::transmute(ppsmemcounters), cb).ok()
}
#[inline]
pub unsafe fn GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    GetWsChanges(hprocess, core::mem::transmute(lpwatchinfo), cb).ok()
}
#[inline]
pub unsafe fn GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> super::super::Foundation:: BOOL);
    GetWsChangesEx(hprocess, core::mem::transmute(lpwatchinfoex), core::mem::transmute(cb)).ok()
}
#[inline]
pub unsafe fn InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    InitializeProcessForWsWatch(hprocess).ok()
}
#[inline]
pub unsafe fn K32EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EmptyWorkingSet(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    K32EmptyWorkingSet(hprocess)
}
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumDeviceDrivers(core::mem::transmute(lpimagebase), cb, core::mem::transmute(lpcbneeded))
}
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    K32EnumPageFilesA(pcallbackroutine, core::mem::transmute(pcontext))
}
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    K32EnumPageFilesW(pcallbackroutine, core::mem::transmute(pcontext))
}
#[inline]
pub unsafe fn K32EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcessModules(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumProcessModules(hprocess, core::mem::transmute(lphmodule), cb, core::mem::transmute(lpcbneeded))
}
#[inline]
pub unsafe fn K32EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcessModulesEx(hprocess : super::super::Foundation:: HANDLE, lphmodule : *mut super::super::Foundation:: HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> super::super::Foundation:: BOOL);
    K32EnumProcessModulesEx(hprocess, core::mem::transmute(lphmodule), cb, core::mem::transmute(lpcbneeded), dwfilterflag)
}
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> super::super::Foundation:: BOOL);
    K32EnumProcesses(core::mem::transmute(lpidprocess), cb, core::mem::transmute(lpcbneeded))
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetMappedFileNameA(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetMappedFileNameW(hprocess : super::super::Foundation:: HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleBaseNameA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetModuleBaseNameA(hprocess, core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: Option<super::super::Foundation::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleBaseNameW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetModuleBaseNameW(hprocess, core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleFileNameExA(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleFileNameExA(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetModuleFileNameExA(core::mem::transmute(hprocess.unwrap_or(core::mem::zeroed())), core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleFileNameExW(hprocess: Option<super::super::Foundation::HANDLE>, hmodule: Option<super::super::Foundation::HMODULE>, lpfilename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleFileNameExW(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetModuleFileNameExW(core::mem::transmute(hprocess.unwrap_or(core::mem::zeroed())), core::mem::transmute(hmodule.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetModuleInformation(hprocess : super::super::Foundation:: HANDLE, hmodule : super::super::Foundation:: HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetModuleInformation(hprocess, hmodule, core::mem::transmute(lpmodinfo), cb)
}
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetPerformanceInfo(core::mem::transmute(pperformanceinformation), cb)
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameA(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    K32GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameW(hprocess : super::super::Foundation:: HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    K32GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap())
}
#[inline]
pub unsafe fn K32GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetProcessMemoryInfo(process : super::super::Foundation:: HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetProcessMemoryInfo(process, core::mem::transmute(ppsmemcounters), cb)
}
#[inline]
pub unsafe fn K32GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetWsChanges(hprocess : super::super::Foundation:: HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> super::super::Foundation:: BOOL);
    K32GetWsChanges(hprocess, core::mem::transmute(lpwatchinfo), cb)
}
#[inline]
pub unsafe fn K32GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32GetWsChangesEx(hprocess : super::super::Foundation:: HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> super::super::Foundation:: BOOL);
    K32GetWsChangesEx(hprocess, core::mem::transmute(lpwatchinfoex), core::mem::transmute(cb))
}
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32InitializeProcessForWsWatch(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    K32InitializeProcessForWsWatch(hprocess)
}
#[inline]
pub unsafe fn K32QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    K32QueryWorkingSet(hprocess, core::mem::transmute(pv), cb)
}
#[inline]
pub unsafe fn K32QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn K32QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    K32QueryWorkingSetEx(hprocess, core::mem::transmute(pv), cb)
}
#[inline]
pub unsafe fn QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn QueryWorkingSet(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    QueryWorkingSet(hprocess, core::mem::transmute(pv), cb).ok()
}
#[inline]
pub unsafe fn QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::Result<()> {
    windows_targets::link!("psapi.dll" "system" fn QueryWorkingSetEx(hprocess : super::super::Foundation:: HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> super::super::Foundation:: BOOL);
    QueryWorkingSetEx(hprocess, core::mem::transmute(pv), cb).ok()
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
impl Default for ENUM_PAGE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENUM_PROCESS_MODULES_EX_FLAGS(pub u32);
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(1u32);
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(2u32);
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(3u32);
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = ENUM_PROCESS_MODULES_EX_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut core::ffi::c_void,
}
impl Default for MODULEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCSTR) -> super::super::Foundation::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCWSTR) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERFORMANCE_INFORMATION {
    pub cb: u32,
    pub CommitTotal: usize,
    pub CommitLimit: usize,
    pub CommitPeak: usize,
    pub PhysicalTotal: usize,
    pub PhysicalAvailable: usize,
    pub SystemCache: usize,
    pub KernelTotal: usize,
    pub KernelPaged: usize,
    pub KernelNonpaged: usize,
    pub PageSize: usize,
    pub HandleCount: u32,
    pub ProcessCount: u32,
    pub ThreadCount: u32,
}
impl Default for PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
impl Default for PROCESS_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
impl Default for PROCESS_MEMORY_COUNTERS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS_EX2 {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
    pub PrivateWorkingSetSize: usize,
    pub SharedCommitUsage: u64,
}
impl Default for PROCESS_MEMORY_COUNTERS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PSAPI_VERSION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
impl Default for PSAPI_WORKING_SET_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
impl Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
impl Default for PSAPI_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
