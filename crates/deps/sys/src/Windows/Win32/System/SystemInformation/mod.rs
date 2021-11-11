#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameExW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetIntegratedDisplaySize(sizeininches: *mut f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemCpuSetInformation(information: *mut SYSTEM_CPU_SET_INFORMATION, bufferlength: u32, returnedlength: *mut u32, process: super::super::Foundation::HANDLE, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: FIRMWARE_TABLE_ID, pfirmwaretablebuffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetTickCount() -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetTickCount64() -> u64;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetVersion() -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64GuestMachineSupported(wowguestmachine: u16, machineissupported: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: super::super::Foundation::PWSTR, deviceform: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: *mut u64, puldevicefamily: *mut DEVICEFAMILYINFOENUM, puldeviceform: *mut DEVICEFAMILYDEVICEFORM);
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameA(lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameEx2W(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameW(lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
}
