#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameExW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    pub fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: *mut FIRMWARE_TABLE_ID, buffersize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL;
    pub fn GetIntegratedDisplaySize(sizeininches: *mut f64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformation(buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessorSystemCycleTime(group: u16, buffer: *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemCpuSetInformation(information: *mut SYSTEM_CPU_SET_INFORMATION, bufferlength: u32, returnedlength: *mut u32, process: super::super::Foundation::HANDLE, flags: u32) -> super::super::Foundation::BOOL;
    pub fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    pub fn GetSystemFirmwareTable(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: FIRMWARE_TABLE_ID, pfirmwaretablebuffer: *mut ::core::ffi::c_void, buffersize: u32) -> u32;
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTime(lpsystemtime: *mut super::super::Foundation::SYSTEMTIME);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime: *mut super::super::Foundation::FILETIME);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2A(lpbuffer: super::super::Foundation::PSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2W(lpbuffer: super::super::Foundation::PWSTR, usize: u32, imagefilemachinetype: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    pub fn GetTickCount() -> u32;
    pub fn GetTickCount64() -> u64;
    pub fn GetVersion() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryA(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryW(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
    pub fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64GuestMachineSupported(wowguestmachine: u16, machineissupported: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: super::super::Foundation::PWSTR, deviceform: super::super::Foundation::PWSTR) -> u32;
    pub fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: *mut u64, puldevicefamily: *mut DEVICEFAMILYINFOENUM, puldeviceform: *mut DEVICEFAMILYDEVICEFORM);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN;
    pub fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32;
    pub fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES;
    pub fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameA(lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameEx2W(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameW(lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    pub fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL;
}
#[repr(C)]
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
pub const ComputerNameNetBIOS: i32 = 0i32;
pub const ComputerNameDnsHostname: i32 = 1i32;
pub const ComputerNameDnsDomain: i32 = 2i32;
pub const ComputerNameDnsFullyQualified: i32 = 3i32;
pub const ComputerNamePhysicalNetBIOS: i32 = 4i32;
pub const ComputerNamePhysicalDnsHostname: i32 = 5i32;
pub const ComputerNamePhysicalDnsDomain: i32 = 6i32;
pub const ComputerNamePhysicalDnsFullyQualified: i32 = 7i32;
pub const ComputerNameMax: i32 = 8i32;
pub const CpuSetInformation: i32 = 0i32;
pub const DEPPolicyAlwaysOff: i32 = 0i32;
pub const DEPPolicyAlwaysOn: i32 = 1i32;
pub const DEPPolicyOptIn: i32 = 2i32;
pub const DEPPolicyOptOut: i32 = 3i32;
pub const DEPTotalPolicyCount: i32 = 4i32;
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: u32 = 0u32;
pub const DEVICEFAMILYDEVICEFORM_PHONE: u32 = 1u32;
pub const DEVICEFAMILYDEVICEFORM_TABLET: u32 = 2u32;
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: u32 = 3u32;
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: u32 = 4u32;
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: u32 = 5u32;
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: u32 = 6u32;
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: u32 = 7u32;
pub const DEVICEFAMILYDEVICEFORM_STICKPC: u32 = 8u32;
pub const DEVICEFAMILYDEVICEFORM_PUCK: u32 = 9u32;
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: u32 = 10u32;
pub const DEVICEFAMILYDEVICEFORM_HMD: u32 = 11u32;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: u32 = 12u32;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: u32 = 13u32;
pub const DEVICEFAMILYDEVICEFORM_BANKING: u32 = 14u32;
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: u32 = 15u32;
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: u32 = 16u32;
pub const DEVICEFAMILYDEVICEFORM_GAMING: u32 = 17u32;
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: u32 = 18u32;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: u32 = 19u32;
pub const DEVICEFAMILYDEVICEFORM_KIOSK: u32 = 20u32;
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: u32 = 21u32;
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: u32 = 22u32;
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: u32 = 23u32;
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: u32 = 24u32;
pub const DEVICEFAMILYDEVICEFORM_PRINTING: u32 = 25u32;
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: u32 = 26u32;
pub const DEVICEFAMILYDEVICEFORM_TOY: u32 = 27u32;
pub const DEVICEFAMILYDEVICEFORM_VENDING: u32 = 28u32;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: u32 = 29u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: u32 = 30u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: u32 = 31u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: u32 = 32u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: u32 = 33u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: u32 = 34u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: u32 = 35u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_00: u32 = 36u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: u32 = 37u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: u32 = 38u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: u32 = 39u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: u32 = 40u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: u32 = 41u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: u32 = 42u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: u32 = 43u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: u32 = 44u32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: u32 = 45u32;
pub const DEVICEFAMILYDEVICEFORM_MAX: u32 = 45u32;
pub const DEVICEFAMILYINFOENUM_UAP: u32 = 0u32;
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: u32 = 1u32;
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: u32 = 2u32;
pub const DEVICEFAMILYINFOENUM_DESKTOP: u32 = 3u32;
pub const DEVICEFAMILYINFOENUM_MOBILE: u32 = 4u32;
pub const DEVICEFAMILYINFOENUM_XBOX: u32 = 5u32;
pub const DEVICEFAMILYINFOENUM_TEAM: u32 = 6u32;
pub const DEVICEFAMILYINFOENUM_IOT: u32 = 7u32;
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: u32 = 8u32;
pub const DEVICEFAMILYINFOENUM_SERVER: u32 = 9u32;
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: u32 = 10u32;
pub const DEVICEFAMILYINFOENUM_XBOXSRA: u32 = 11u32;
pub const DEVICEFAMILYINFOENUM_XBOXERA: u32 = 12u32;
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: u32 = 13u32;
pub const DEVICEFAMILYINFOENUM_8828080: u32 = 14u32;
pub const DEVICEFAMILYINFOENUM_7067329: u32 = 15u32;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: u32 = 16u32;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: u32 = 17u32;
pub const DEVICEFAMILYINFOENUM_MAX: u32 = 17u32;
pub type FIRMWARE_TABLE_ID = u32;
pub const ACPI: u32 = 1094930505u32;
pub const FIRM: u32 = 1179210317u32;
pub const RSMB: u32 = 1381190978u32;
pub const FirmwareTypeUnknown: i32 = 0i32;
pub const FirmwareTypeBios: i32 = 1i32;
pub const FirmwareTypeUefi: i32 = 2i32;
pub const FirmwareTypeMax: i32 = 3i32;
#[repr(C)]
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
pub const RelationProcessorCore: i32 = 0i32;
pub const RelationNumaNode: i32 = 1i32;
pub const RelationCache: i32 = 2i32;
pub const RelationProcessorPackage: i32 = 3i32;
pub const RelationGroup: i32 = 4i32;
pub const RelationProcessorDie: i32 = 5i32;
pub const RelationNumaNodeEx: i32 = 6i32;
pub const RelationProcessorModule: i32 = 7i32;
pub const RelationAll: i32 = 65535i32;
#[repr(C)]
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
pub const NTDDI_LONGHORN: u32 = 100663296u32;
pub const NTDDI_VERSION: u32 = 167772171u32;
pub const NTDDI_VISTA: u32 = 100663296u32;
pub const NTDDI_VISTASP1: u32 = 100663552u32;
pub const NTDDI_VISTASP2: u32 = 100663808u32;
pub const NTDDI_VISTASP3: u32 = 100664064u32;
pub const NTDDI_VISTASP4: u32 = 100664320u32;
pub const NTDDI_WIN10: u32 = 167772160u32;
pub const NTDDI_WIN10_19H1: u32 = 167772167u32;
pub const NTDDI_WIN10_CO: u32 = 167772171u32;
pub const NTDDI_WIN10_FE: u32 = 167772170u32;
pub const NTDDI_WIN10_MN: u32 = 167772169u32;
pub const NTDDI_WIN10_RS1: u32 = 167772162u32;
pub const NTDDI_WIN10_RS2: u32 = 167772163u32;
pub const NTDDI_WIN10_RS3: u32 = 167772164u32;
pub const NTDDI_WIN10_RS4: u32 = 167772165u32;
pub const NTDDI_WIN10_RS5: u32 = 167772166u32;
pub const NTDDI_WIN10_TH2: u32 = 167772161u32;
pub const NTDDI_WIN10_VB: u32 = 167772168u32;
pub const NTDDI_WIN2K: u32 = 83886080u32;
pub const NTDDI_WIN2KSP1: u32 = 83886336u32;
pub const NTDDI_WIN2KSP2: u32 = 83886592u32;
pub const NTDDI_WIN2KSP3: u32 = 83886848u32;
pub const NTDDI_WIN2KSP4: u32 = 83887104u32;
pub const NTDDI_WIN4: u32 = 67108864u32;
pub const NTDDI_WIN6: u32 = 100663296u32;
pub const NTDDI_WIN6SP1: u32 = 100663552u32;
pub const NTDDI_WIN6SP2: u32 = 100663808u32;
pub const NTDDI_WIN6SP3: u32 = 100664064u32;
pub const NTDDI_WIN6SP4: u32 = 100664320u32;
pub const NTDDI_WIN7: u32 = 100728832u32;
pub const NTDDI_WIN8: u32 = 100794368u32;
pub const NTDDI_WINBLUE: u32 = 100859904u32;
pub const NTDDI_WINTHRESHOLD: u32 = 167772160u32;
pub const NTDDI_WINXP: u32 = 83951616u32;
pub const NTDDI_WINXPSP1: u32 = 83951872u32;
pub const NTDDI_WINXPSP2: u32 = 83952128u32;
pub const NTDDI_WINXPSP3: u32 = 83952384u32;
pub const NTDDI_WINXPSP4: u32 = 83952640u32;
pub const NTDDI_WS03: u32 = 84017152u32;
pub const NTDDI_WS03SP1: u32 = 84017408u32;
pub const NTDDI_WS03SP2: u32 = 84017664u32;
pub const NTDDI_WS03SP3: u32 = 84017920u32;
pub const NTDDI_WS03SP4: u32 = 84018176u32;
pub const NTDDI_WS08: u32 = 100663552u32;
pub const NTDDI_WS08SP2: u32 = 100663808u32;
pub const NTDDI_WS08SP3: u32 = 100664064u32;
pub const NTDDI_WS08SP4: u32 = 100664320u32;
#[repr(C)]
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
pub const OSVERSION_MASK: u32 = 4294901760u32;
pub const OS_DEPLOYMENT_STANDARD: i32 = 1i32;
pub const OS_DEPLOYMENT_COMPACT: i32 = 2i32;
pub const PRODUCT_BUSINESS: u32 = 6u32;
pub const PRODUCT_BUSINESS_N: u32 = 16u32;
pub const PRODUCT_CLUSTER_SERVER: u32 = 18u32;
pub const PRODUCT_CLUSTER_SERVER_V: u32 = 64u32;
pub const PRODUCT_CORE: u32 = 101u32;
pub const PRODUCT_CORE_COUNTRYSPECIFIC: u32 = 99u32;
pub const PRODUCT_CORE_N: u32 = 98u32;
pub const PRODUCT_CORE_SINGLELANGUAGE: u32 = 100u32;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: u32 = 80u32;
pub const PRODUCT_DATACENTER_A_SERVER_CORE: u32 = 145u32;
pub const PRODUCT_STANDARD_A_SERVER_CORE: u32 = 146u32;
pub const PRODUCT_DATACENTER_SERVER: u32 = 8u32;
pub const PRODUCT_DATACENTER_SERVER_CORE: u32 = 12u32;
pub const PRODUCT_DATACENTER_SERVER_CORE_V: u32 = 39u32;
pub const PRODUCT_DATACENTER_SERVER_V: u32 = 37u32;
pub const PRODUCT_EDUCATION: u32 = 121u32;
pub const PRODUCT_EDUCATION_N: u32 = 122u32;
pub const PRODUCT_ENTERPRISE: u32 = 4u32;
pub const PRODUCT_ENTERPRISE_E: u32 = 70u32;
pub const PRODUCT_ENTERPRISE_EVALUATION: u32 = 72u32;
pub const PRODUCT_ENTERPRISE_N: u32 = 27u32;
pub const PRODUCT_ENTERPRISE_N_EVALUATION: u32 = 84u32;
pub const PRODUCT_ENTERPRISE_S: u32 = 125u32;
pub const PRODUCT_ENTERPRISE_S_EVALUATION: u32 = 129u32;
pub const PRODUCT_ENTERPRISE_S_N: u32 = 126u32;
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: u32 = 130u32;
pub const PRODUCT_ENTERPRISE_SERVER: u32 = 10u32;
pub const PRODUCT_ENTERPRISE_SERVER_CORE: u32 = 14u32;
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: u32 = 41u32;
pub const PRODUCT_ENTERPRISE_SERVER_IA64: u32 = 15u32;
pub const PRODUCT_ENTERPRISE_SERVER_V: u32 = 38u32;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: u32 = 60u32;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: u32 = 62u32;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: u32 = 59u32;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: u32 = 61u32;
pub const PRODUCT_HOME_BASIC: u32 = 2u32;
pub const PRODUCT_HOME_BASIC_E: u32 = 67u32;
pub const PRODUCT_HOME_BASIC_N: u32 = 5u32;
pub const PRODUCT_HOME_PREMIUM: u32 = 3u32;
pub const PRODUCT_HOME_PREMIUM_E: u32 = 68u32;
pub const PRODUCT_HOME_PREMIUM_N: u32 = 26u32;
pub const PRODUCT_HOME_PREMIUM_SERVER: u32 = 34u32;
pub const PRODUCT_HOME_SERVER: u32 = 19u32;
pub const PRODUCT_HYPERV: u32 = 42u32;
pub const PRODUCT_IOTUAP: u32 = 123u32;
pub const PRODUCT_IOTUAPCOMMERCIAL: u32 = 131u32;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: u32 = 30u32;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: u32 = 32u32;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: u32 = 31u32;
pub const PRODUCT_MOBILE_CORE: u32 = 104u32;
pub const PRODUCT_MOBILE_ENTERPRISE: u32 = 133u32;
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: u32 = 77u32;
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: u32 = 76u32;
pub const PRODUCT_PRO_WORKSTATION: u32 = 161u32;
pub const PRODUCT_PRO_WORKSTATION_N: u32 = 162u32;
pub const PRODUCT_PROFESSIONAL: u32 = 48u32;
pub const PRODUCT_PROFESSIONAL_E: u32 = 69u32;
pub const PRODUCT_PROFESSIONAL_N: u32 = 49u32;
pub const PRODUCT_PROFESSIONAL_WMC: u32 = 103u32;
pub const PRODUCT_SB_SOLUTION_SERVER: u32 = 50u32;
pub const PRODUCT_SB_SOLUTION_SERVER_EM: u32 = 54u32;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: u32 = 51u32;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: u32 = 55u32;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: u32 = 24u32;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: u32 = 35u32;
pub const PRODUCT_SERVER_FOUNDATION: u32 = 33u32;
pub const PRODUCT_SMALLBUSINESS_SERVER: u32 = 9u32;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: u32 = 25u32;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: u32 = 63u32;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: u32 = 56u32;
pub const PRODUCT_STANDARD_EVALUATION_SERVER: u32 = 79u32;
pub const PRODUCT_STANDARD_SERVER: u32 = 7u32;
pub const PRODUCT_STANDARD_SERVER_CORE_: u32 = 13u32;
pub const PRODUCT_STANDARD_SERVER_CORE_V: u32 = 40u32;
pub const PRODUCT_STANDARD_SERVER_V: u32 = 36u32;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: u32 = 52u32;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: u32 = 53u32;
pub const PRODUCT_STARTER: u32 = 11u32;
pub const PRODUCT_STARTER_E: u32 = 66u32;
pub const PRODUCT_STARTER_N: u32 = 47u32;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: u32 = 23u32;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: u32 = 46u32;
pub const PRODUCT_STORAGE_EXPRESS_SERVER: u32 = 20u32;
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: u32 = 43u32;
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: u32 = 96u32;
pub const PRODUCT_STORAGE_STANDARD_SERVER: u32 = 21u32;
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: u32 = 44u32;
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: u32 = 95u32;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: u32 = 22u32;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: u32 = 45u32;
pub const PRODUCT_ULTIMATE: u32 = 1u32;
pub const PRODUCT_ULTIMATE_E: u32 = 71u32;
pub const PRODUCT_ULTIMATE_N: u32 = 28u32;
pub const PRODUCT_UNDEFINED: u32 = 0u32;
pub const PRODUCT_WEB_SERVER: u32 = 17u32;
pub const PRODUCT_WEB_SERVER_CORE: u32 = 29u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PSTR, usize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = unsafe extern "system" fn(lpbuffer: super::super::Foundation::PWSTR, usize: u32) -> u32;
pub const CacheUnified: i32 = 0i32;
pub const CacheInstruction: i32 = 1i32;
pub const CacheData: i32 = 2i32;
pub const CacheTrace: i32 = 3i32;
#[repr(C)]
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
pub const GlobalDataIdUnknown: i32 = 0i32;
pub const GlobalDataIdRngSeedVersion: i32 = 1i32;
pub const GlobalDataIdInterruptTime: i32 = 2i32;
pub const GlobalDataIdTimeZoneBias: i32 = 3i32;
pub const GlobalDataIdImageNumberLow: i32 = 4i32;
pub const GlobalDataIdImageNumberHigh: i32 = 5i32;
pub const GlobalDataIdTimeZoneId: i32 = 6i32;
pub const GlobalDataIdNtMajorVersion: i32 = 7i32;
pub const GlobalDataIdNtMinorVersion: i32 = 8i32;
pub const GlobalDataIdSystemExpirationDate: i32 = 9i32;
pub const GlobalDataIdKdDebuggerEnabled: i32 = 10i32;
pub const GlobalDataIdCyclesPerYield: i32 = 11i32;
pub const GlobalDataIdSafeBootMode: i32 = 12i32;
pub const GlobalDataIdLastSystemRITEventTickCount: i32 = 13i32;
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
pub const SPVERSION_MASK: u32 = 65280u32;
pub const SUBVERSION_MASK: u32 = 255u32;
#[repr(C)]
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
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
#[repr(C)]
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
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::clone::Clone for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: u32 = 0u32;
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: u32 = 2u32;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: u32 = 16u32;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: u32 = 17u32;
pub const VER_MINORVERSION: u32 = 1u32;
pub const VER_MAJORVERSION: u32 = 2u32;
pub const VER_BUILDNUMBER: u32 = 4u32;
pub const VER_PLATFORMID: u32 = 8u32;
pub const VER_SERVICEPACKMINOR: u32 = 16u32;
pub const VER_SERVICEPACKMAJOR: u32 = 32u32;
pub const VER_SUITENAME: u32 = 64u32;
pub const VER_PRODUCT_TYPE: u32 = 128u32;
pub const WDK_NTDDI_VERSION: u32 = 167772171u32;
pub const _WIN32_IE_IE100: u32 = 2560u32;
pub const _WIN32_IE_IE110: u32 = 2560u32;
pub const _WIN32_IE_IE20: u32 = 512u32;
pub const _WIN32_IE_IE30: u32 = 768u32;
pub const _WIN32_IE_IE302: u32 = 770u32;
pub const _WIN32_IE_IE40: u32 = 1024u32;
pub const _WIN32_IE_IE401: u32 = 1025u32;
pub const _WIN32_IE_IE50: u32 = 1280u32;
pub const _WIN32_IE_IE501: u32 = 1281u32;
pub const _WIN32_IE_IE55: u32 = 1360u32;
pub const _WIN32_IE_IE60: u32 = 1536u32;
pub const _WIN32_IE_IE60SP1: u32 = 1537u32;
pub const _WIN32_IE_IE60SP2: u32 = 1539u32;
pub const _WIN32_IE_IE70: u32 = 1792u32;
pub const _WIN32_IE_IE80: u32 = 2048u32;
pub const _WIN32_IE_IE90: u32 = 2304u32;
pub const _WIN32_IE_LONGHORN: u32 = 1792u32;
pub const _WIN32_IE_NT4: u32 = 512u32;
pub const _WIN32_IE_NT4SP1: u32 = 512u32;
pub const _WIN32_IE_NT4SP2: u32 = 512u32;
pub const _WIN32_IE_NT4SP3: u32 = 770u32;
pub const _WIN32_IE_NT4SP4: u32 = 1025u32;
pub const _WIN32_IE_NT4SP5: u32 = 1025u32;
pub const _WIN32_IE_NT4SP6: u32 = 1280u32;
pub const _WIN32_IE_WIN10: u32 = 2560u32;
pub const _WIN32_IE_WIN2K: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP1: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP2: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP3: u32 = 1281u32;
pub const _WIN32_IE_WIN2KSP4: u32 = 1281u32;
pub const _WIN32_IE_WIN6: u32 = 1792u32;
pub const _WIN32_IE_WIN7: u32 = 2048u32;
pub const _WIN32_IE_WIN8: u32 = 2560u32;
pub const _WIN32_IE_WIN98: u32 = 1025u32;
pub const _WIN32_IE_WIN98SE: u32 = 1280u32;
pub const _WIN32_IE_WINBLUE: u32 = 2560u32;
pub const _WIN32_IE_WINME: u32 = 1360u32;
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_IE_WS03: u32 = 1538u32;
pub const _WIN32_IE_WS03SP1: u32 = 1539u32;
pub const _WIN32_IE_XP: u32 = 1536u32;
pub const _WIN32_IE_XPSP1: u32 = 1537u32;
pub const _WIN32_IE_XPSP2: u32 = 1539u32;
pub const _WIN32_WINNT_LONGHORN: u32 = 1536u32;
pub const _WIN32_WINNT_NT4: u32 = 1024u32;
pub const _WIN32_WINNT_VISTA: u32 = 1536u32;
pub const _WIN32_WINNT_WIN10: u32 = 2560u32;
pub const _WIN32_WINNT_WIN2K: u32 = 1280u32;
pub const _WIN32_WINNT_WIN6: u32 = 1536u32;
pub const _WIN32_WINNT_WIN7: u32 = 1537u32;
pub const _WIN32_WINNT_WIN8: u32 = 1538u32;
pub const _WIN32_WINNT_WINBLUE: u32 = 1539u32;
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560u32;
pub const _WIN32_WINNT_WINXP: u32 = 1281u32;
pub const _WIN32_WINNT_WS03: u32 = 1282u32;
pub const _WIN32_WINNT_WS08: u32 = 1536u32;
