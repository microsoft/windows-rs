#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameExW(hostname: ::windows_sys::core::PCWSTR, computername: ::windows_sys::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows_sys::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows_sys::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetIntegratedDisplaySize(sizeininches: *mut f64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemCpuSetInformation(information: *mut SYSTEM_CPU_SET_INFORMATION, bufferlength: u32, returnedlength: *mut u32, process: super::super::Foundation::HANDLE, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemDirectoryA(lpbuffer: ::windows_sys::core::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemDirectoryW(lpbuffer: ::windows_sys::core::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: FIRMWARE_TABLE_ID, pfirmwaretablebuffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWindowsDirectoryA(lpbuffer: ::windows_sys::core::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWindowsDirectoryW(lpbuffer: ::windows_sys::core::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWow64Directory2A(lpbuffer: ::windows_sys::core::PSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWow64Directory2W(lpbuffer: ::windows_sys::core::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWow64DirectoryA(lpbuffer: ::windows_sys::core::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetSystemWow64DirectoryW(lpbuffer: ::windows_sys::core::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetTickCount() -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetTickCount64() -> u64;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetVersion() -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetWindowsDirectoryA(lpbuffer: ::windows_sys::core::PSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GetWindowsDirectoryW(lpbuffer: ::windows_sys::core::PWSTR, usize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64GuestMachineSupported(wowguestmachine: u16, machineissupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: ::windows_sys::core::PWSTR, deviceform: ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: *mut u64, puldevicefamily: *mut DEVICEFAMILYINFOENUM, puldeviceform: *mut DEVICEFAMILYDEVICEFORM);
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameA(lpcomputername: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameEx2W(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameW(lpcomputername: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
    pub fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
impl ::core::marker::Copy for CACHE_DESCRIPTOR {}
impl ::core::clone::Clone for CACHE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
impl ::core::marker::Copy for CACHE_RELATIONSHIP {}
impl ::core::clone::Clone for CACHE_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl ::core::marker::Copy for CACHE_RELATIONSHIP_0 {}
impl ::core::clone::Clone for CACHE_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type COMPUTER_NAME_FORMAT = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type CPU_SET_INFORMATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type DEP_SYSTEM_POLICY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type DEVICEFAMILYDEVICEFORM = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = 30u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_00: DEVICEFAMILYDEVICEFORM = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type DEVICEFAMILYINFOENUM = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = 17u32;
pub type FIRMWARE_TABLE_ID = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type FIRMWARE_TABLE_PROVIDER = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ACPI: FIRMWARE_TABLE_PROVIDER = 1094930505u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FIRM: FIRMWARE_TABLE_PROVIDER = 1179210317u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RSMB: FIRMWARE_TABLE_PROVIDER = 1381190978u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type FIRMWARE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeBios: FIRMWARE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeUefi: FIRMWARE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeMax: FIRMWARE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct GROUP_AFFINITY {
    pub Mask: usize,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl ::core::marker::Copy for GROUP_AFFINITY {}
impl ::core::clone::Clone for GROUP_AFFINITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl ::core::marker::Copy for GROUP_RELATIONSHIP {}
impl ::core::clone::Clone for GROUP_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type LOGICAL_PROCESSOR_RELATIONSHIP = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = 65535i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
impl ::core::marker::Copy for MEMORYSTATUS {}
impl ::core::clone::Clone for MEMORYSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: u64,
    pub ullAvailPhys: u64,
    pub ullTotalPageFile: u64,
    pub ullAvailPageFile: u64,
    pub ullTotalVirtual: u64,
    pub ullAvailVirtual: u64,
    pub ullAvailExtendedVirtual: u64,
}
impl ::core::marker::Copy for MEMORYSTATUSEX {}
impl ::core::clone::Clone for MEMORYSTATUSEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_LONGHORN: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VERSION: u32 = 167772171u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VISTA: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VISTASP1: u32 = 100663552u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VISTASP2: u32 = 100663808u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VISTASP3: u32 = 100664064u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_VISTASP4: u32 = 100664320u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10: u32 = 167772160u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN2K: u32 = 83886080u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN4: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN6: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN7: u32 = 100728832u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WIN8: u32 = 100794368u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINBLUE: u32 = 100859904u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINXP: u32 = 83951616u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS03: u32 = 84017152u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS03SP1: u32 = 84017408u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS03SP2: u32 = 84017664u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS03SP3: u32 = 84017920u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS03SP4: u32 = 84018176u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS08: u32 = 100663552u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS08SP2: u32 = 100663808u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS08SP3: u32 = 100664064u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const NTDDI_WS08SP4: u32 = 100664320u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl ::core::marker::Copy for NUMA_NODE_RELATIONSHIP {}
impl ::core::clone::Clone for NUMA_NODE_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl ::core::marker::Copy for NUMA_NODE_RELATIONSHIP_0 {}
impl ::core::clone::Clone for NUMA_NODE_RELATIONSHIP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OSVERSIONINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [super::super::Foundation::CHAR; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OSVERSIONINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl ::core::marker::Copy for OSVERSIONINFOEXW {}
impl ::core::clone::Clone for OSVERSIONINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl ::core::marker::Copy for OSVERSIONINFOW {}
impl ::core::clone::Clone for OSVERSIONINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type OS_DEPLOYEMENT_STATE_VALUES = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type OS_PRODUCT_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = 101u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = 99u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = 98u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = 100u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = 80u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = 145u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = 146u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = 121u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = 122u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = 70u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = 72u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = 84u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = 125u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = 129u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = 126u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = 130u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = 60u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = 62u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = 59u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = 61u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = 67u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = 68u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = 123u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = 131u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = 30u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = 104u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = 133u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = 77u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = 76u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = 161u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = 162u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = 48u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = 69u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = 49u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = 103u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = 50u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = 54u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = 51u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = 55u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = 63u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = 56u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = 79u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = 52u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = 53u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = 66u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = 47u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = 46u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = 96u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = 95u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = 71u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = ::core::option::Option<unsafe extern "system" fn(lpbuffer: ::windows_sys::core::PSTR, usize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = ::core::option::Option<unsafe extern "system" fn(lpbuffer: ::windows_sys::core::PWSTR, usize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type PROCESSOR_CACHE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheUnified: PROCESSOR_CACHE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheData: PROCESSOR_CACHE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheTrace: PROCESSOR_CACHE_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: usize,
}
impl ::core::marker::Copy for PROCESSOR_GROUP_INFO {}
impl ::core::clone::Clone for PROCESSOR_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl ::core::marker::Copy for PROCESSOR_RELATIONSHIP {}
impl ::core::clone::Clone for PROCESSOR_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type RTL_SYSTEM_GLOBAL_DATA_ID = i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = 13i32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SPVERSION_MASK: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SUBVERSION_MASK: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous1: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl ::core::marker::Copy for SYSTEM_CPU_SET_INFORMATION_0_0_1 {}
impl ::core::clone::Clone for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut ::core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut ::core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SYSTEM_INFO {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SYSTEM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SYSTEM_INFO_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SYSTEM_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::Diagnostics::Debug::PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SYSTEM_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub NodeNumber: u32,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub Flags: u8,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
}
impl ::core::marker::Copy for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {}
impl ::core::clone::Clone for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYSTEM_POOL_ZEROING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
impl ::core::marker::Copy for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type USER_CET_ENVIRONMENT = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type VER_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_MINORVERSION: VER_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_MAJORVERSION: VER_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_BUILDNUMBER: VER_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_PLATFORMID: VER_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SERVICEPACKMINOR: VER_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SUITENAME: VER_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_PRODUCT_TYPE: VER_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const WDK_NTDDI_VERSION: u32 = 167772171u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE100: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE110: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE20: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE30: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE302: u32 = 770u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE40: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE401: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE50: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE501: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE55: u32 = 1360u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE60: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE70: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE80: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_IE90: u32 = 2304u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN10: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN6: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN7: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN8: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN98: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WINME: u32 = 1360u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WS03: u32 = 1538u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_XP: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
