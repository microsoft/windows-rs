windows_link::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_sys::core::PCWSTR, computername : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer : *mut u8, buffersize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-6.dll" "system" fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE);
windows_link::link!("kernel32.dll" "system" fn GetFirmwareType(firmwaretype : *mut FIRMWARE_TYPE) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetIntegratedDisplaySize(sizeininches : *mut f64) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : *mut super::super::Foundation::SYSTEMTIME));
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : LOGICAL_PROCESSOR_RELATIONSHIP, buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : u16, buffer : *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : u32, dwosminorversion : u32, dwspmajorversion : u32, dwspminorversion : u32, pdwreturnedproducttype : *mut OS_PRODUCT_TYPE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetRuntimeAttestationReport(nonce : *const u8, packageversion : u16, reporttypesbitmap : u64, reportbuffer : *mut core::ffi::c_void, reportbuffersize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemCpuSetInformation(information : *mut SYSTEM_CPU_SET_INFORMATION, bufferlength : u32, returnedlength : *mut u32, process : super::super::Foundation::HANDLE, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE);
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER, firmwaretableid : u32, pfirmwaretablebuffer : *mut u8, buffersize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
windows_link::link!("kernel32.dll" "system" fn GetSystemLeapSecondInformation(enabled : *mut windows_sys::core::BOOL, flags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : *mut super::super::Foundation::SYSTEMTIME));
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : *mut u32, lptimeincrement : *mut u32, lptimeadjustmentdisabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : *mut u64, lptimeincrement : *mut u64, lptimeadjustmentdisabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : *mut super::super::Foundation::FILETIME));
windows_link::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : *mut super::super::Foundation::FILETIME));
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2A(lpbuffer : windows_sys::core::PSTR, usize : u32, imagefilemachinetype : IMAGE_FILE_MACHINE) -> u32);
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2W(lpbuffer : windows_sys::core::PWSTR, usize : u32, imagefilemachinetype : IMAGE_FILE_MACHINE) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTickCount64() -> u64);
windows_link::link!("kernel32.dll" "system" fn GetVersion() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : *mut OSVERSIONINFOA) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : *mut OSVERSIONINFOW) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : *mut MEMORYSTATUS));
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : *mut MEMORYSTATUSEX) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : USER_CET_ENVIRONMENT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsWow64GuestMachineSupported(wowguestmachine : IMAGE_FILE_MACHINE, machineissupported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("ntdll.dll" "system" fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize : *mut u32, puldeviceformbuffersize : *mut u32, devicefamily : windows_sys::core::PWSTR, deviceform : windows_sys::core::PWSTR) -> u32);
windows_link::link!("ntdll.dll" "system" fn RtlGetDeviceFamilyInfoEnum(pulluapinfo : *mut u64, puldevicefamily : *mut DEVICEFAMILYINFOENUM, puldeviceform : *mut DEVICEFAMILYDEVICEFORM));
windows_link::link!("ntdll.dll" "system" fn RtlGetProductInfo(osmajorversion : u32, osminorversion : u32, spmajorversion : u32, spminorversion : u32, returnedproducttype : *mut u32) -> bool);
windows_link::link!("ntdllk.dll" "system" fn RtlGetSystemGlobalData(dataid : RTL_SYSTEM_GLOBAL_DATA_ID, buffer : *mut core::ffi::c_void, size : u32) -> u32);
windows_link::link!("ntdll.dll" "system" fn RtlOsDeploymentState(flags : u32) -> OS_DEPLOYEMENT_STATE_VALUES);
windows_link::link!("ntdll.dll" "system" fn RtlSwitchedVVI(versioninfo : *const OSVERSIONINFOEXW, typemask : u32, conditionmask : u64) -> u32);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : u32, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameW(lpcomputername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::super::Foundation::SYSTEMTIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::super::Foundation::SYSTEMTIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : u32, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : u64, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn VerSetConditionMask(conditionmask : u64, typemask : VER_FLAGS, condition : u8) -> u64);
windows_link::link!("kernel32.dll" "system" fn VerifyVersionInfoA(lpversioninformation : *mut OSVERSIONINFOEXA, dwtypemask : VER_FLAGS, dwlconditionmask : u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn VerifyVersionInfoW(lpversioninformation : *mut OSVERSIONINFOEXW, dwtypemask : VER_FLAGS, dwlconditionmask : u64) -> windows_sys::core::BOOL);
pub const ACPI: FIRMWARE_TABLE_PROVIDER = 1094930505;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COMPUTER_NAME_FORMAT = i32;
pub type CPU_SET_INFORMATION_TYPE = i32;
pub const CacheData: PROCESSOR_CACHE_TYPE = 2;
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = 1;
pub const CacheTrace: PROCESSOR_CACHE_TYPE = 3;
pub const CacheUnified: PROCESSOR_CACHE_TYPE = 0;
pub const CacheUnknown: PROCESSOR_CACHE_TYPE = 4;
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = 2;
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = 3;
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = 1;
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = 8;
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = 0;
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = 6;
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = 7;
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = 5;
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = 4;
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = 0;
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = 0;
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = 1;
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = 2;
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = 3;
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = 4;
pub type DEP_SYSTEM_POLICY_TYPE = i32;
pub type DEVELOPER_DRIVE_ENABLEMENT_STATE = i32;
pub type DEVICEFAMILYDEVICEFORM = u32;
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = 7;
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = 14;
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM = 15;
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = 5;
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = 3;
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = 6;
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM = 16;
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = 17;
pub const DEVICEFAMILYDEVICEFORM_GAMING_CONSOLE: DEVICEFAMILYDEVICEFORM = 47;
pub const DEVICEFAMILYDEVICEFORM_GAMING_HANDHELD: DEVICEFAMILYDEVICEFORM = 46;
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = 11;
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM = 18;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM = 19;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM = 12;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM = 29;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM = 13;
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = 20;
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM = 10;
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM = 21;
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = 47;
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = 22;
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = 23;
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = 4;
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = 1;
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM = 24;
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = 25;
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = 9;
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = 8;
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = 2;
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM = 26;
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = 27;
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = 0;
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = 28;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = 30;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = 31;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = 32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM = 33;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM = 37;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM = 38;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM = 39;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM = 40;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM = 41;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM = 42;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM = 43;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM = 44;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM = 45;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_S: DEVICEFAMILYDEVICEFORM = 36;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM = 34;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM = 35;
pub type DEVICEFAMILYINFOENUM = u32;
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = 15;
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = 14;
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = 3;
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = 10;
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = 7;
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = 8;
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = 17;
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = 4;
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = 9;
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = 13;
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = 6;
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = 0;
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = 1;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = 16;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM = 17;
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = 2;
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = 5;
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = 12;
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = 11;
pub const DeveloperDriveDisabledByGroupPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = 3;
pub const DeveloperDriveDisabledBySystemPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = 2;
pub const DeveloperDriveEnabled: DEVELOPER_DRIVE_ENABLEMENT_STATE = 1;
pub const DeveloperDriveEnablementStateError: DEVELOPER_DRIVE_ENABLEMENT_STATE = 0;
pub const FIRM: FIRMWARE_TABLE_PROVIDER = 1179210317;
pub type FIRMWARE_TABLE_PROVIDER = u32;
pub type FIRMWARE_TYPE = i32;
pub const FirmwareTypeBios: FIRMWARE_TYPE = 1;
pub const FirmwareTypeMax: FIRMWARE_TYPE = 3;
pub const FirmwareTypeUefi: FIRMWARE_TYPE = 2;
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY {
    pub Mask: usize,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY32 {
    pub Mask: u32,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY64 {
    pub Mask: u64,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
impl Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GlobalDataIdConsoleSharedDataFlags: RTL_SYSTEM_GLOBAL_DATA_ID = 14;
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = 11;
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = 5;
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = 4;
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = 2;
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = 10;
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = 13;
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 7;
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 8;
pub const GlobalDataIdNtSystemRootDrive: RTL_SYSTEM_GLOBAL_DATA_ID = 15;
pub const GlobalDataIdQpcBias: RTL_SYSTEM_GLOBAL_DATA_ID = 18;
pub const GlobalDataIdQpcBypassEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = 16;
pub const GlobalDataIdQpcData: RTL_SYSTEM_GLOBAL_DATA_ID = 17;
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 1;
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = 12;
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = 9;
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = 3;
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = 6;
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = 0;
pub type IMAGE_FILE_MACHINE = u16;
pub const IMAGE_FILE_MACHINE_ALPHA: IMAGE_FILE_MACHINE = 388;
pub const IMAGE_FILE_MACHINE_ALPHA64: IMAGE_FILE_MACHINE = 644;
pub const IMAGE_FILE_MACHINE_AM33: IMAGE_FILE_MACHINE = 467;
pub const IMAGE_FILE_MACHINE_AMD64: IMAGE_FILE_MACHINE = 34404;
pub const IMAGE_FILE_MACHINE_ARM: IMAGE_FILE_MACHINE = 448;
pub const IMAGE_FILE_MACHINE_ARM64: IMAGE_FILE_MACHINE = 43620;
pub const IMAGE_FILE_MACHINE_ARMNT: IMAGE_FILE_MACHINE = 452;
pub const IMAGE_FILE_MACHINE_AXP64: IMAGE_FILE_MACHINE = 644;
pub const IMAGE_FILE_MACHINE_CEE: IMAGE_FILE_MACHINE = 49390;
pub const IMAGE_FILE_MACHINE_CEF: IMAGE_FILE_MACHINE = 3311;
pub const IMAGE_FILE_MACHINE_EBC: IMAGE_FILE_MACHINE = 3772;
pub const IMAGE_FILE_MACHINE_I386: IMAGE_FILE_MACHINE = 332;
pub const IMAGE_FILE_MACHINE_IA64: IMAGE_FILE_MACHINE = 512;
pub const IMAGE_FILE_MACHINE_M32R: IMAGE_FILE_MACHINE = 36929;
pub const IMAGE_FILE_MACHINE_MIPS16: IMAGE_FILE_MACHINE = 614;
pub const IMAGE_FILE_MACHINE_MIPSFPU: IMAGE_FILE_MACHINE = 870;
pub const IMAGE_FILE_MACHINE_MIPSFPU16: IMAGE_FILE_MACHINE = 1126;
pub const IMAGE_FILE_MACHINE_POWERPC: IMAGE_FILE_MACHINE = 496;
pub const IMAGE_FILE_MACHINE_POWERPCFP: IMAGE_FILE_MACHINE = 497;
pub const IMAGE_FILE_MACHINE_R10000: IMAGE_FILE_MACHINE = 360;
pub const IMAGE_FILE_MACHINE_R3000: IMAGE_FILE_MACHINE = 354;
pub const IMAGE_FILE_MACHINE_R4000: IMAGE_FILE_MACHINE = 358;
pub const IMAGE_FILE_MACHINE_SH3: IMAGE_FILE_MACHINE = 418;
pub const IMAGE_FILE_MACHINE_SH3DSP: IMAGE_FILE_MACHINE = 419;
pub const IMAGE_FILE_MACHINE_SH3E: IMAGE_FILE_MACHINE = 420;
pub const IMAGE_FILE_MACHINE_SH4: IMAGE_FILE_MACHINE = 422;
pub const IMAGE_FILE_MACHINE_SH5: IMAGE_FILE_MACHINE = 424;
pub const IMAGE_FILE_MACHINE_TARGET_HOST: IMAGE_FILE_MACHINE = 1;
pub const IMAGE_FILE_MACHINE_THUMB: IMAGE_FILE_MACHINE = 450;
pub const IMAGE_FILE_MACHINE_TRICORE: IMAGE_FILE_MACHINE = 1312;
pub const IMAGE_FILE_MACHINE_UNKNOWN: IMAGE_FILE_MACHINE = 0;
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: IMAGE_FILE_MACHINE = 361;
pub type LOGICAL_PROCESSOR_RELATIONSHIP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const NTDDI_LONGHORN: u32 = 100663296;
pub const NTDDI_VERSION: u32 = 167772176;
pub const NTDDI_VISTA: u32 = 100663296;
pub const NTDDI_VISTASP1: u32 = 100663552;
pub const NTDDI_VISTASP2: u32 = 100663808;
pub const NTDDI_VISTASP3: u32 = 100664064;
pub const NTDDI_VISTASP4: u32 = 100664320;
pub const NTDDI_WIN10: u32 = 167772160;
pub const NTDDI_WIN10_19H1: u32 = 167772167;
pub const NTDDI_WIN10_CO: u32 = 167772171;
pub const NTDDI_WIN10_CU: u32 = 167772173;
pub const NTDDI_WIN10_FE: u32 = 167772170;
pub const NTDDI_WIN10_MN: u32 = 167772169;
pub const NTDDI_WIN10_NI: u32 = 167772172;
pub const NTDDI_WIN10_RS1: u32 = 167772162;
pub const NTDDI_WIN10_RS2: u32 = 167772163;
pub const NTDDI_WIN10_RS3: u32 = 167772164;
pub const NTDDI_WIN10_RS4: u32 = 167772165;
pub const NTDDI_WIN10_RS5: u32 = 167772166;
pub const NTDDI_WIN10_TH2: u32 = 167772161;
pub const NTDDI_WIN10_VB: u32 = 167772168;
pub const NTDDI_WIN11_GA: u32 = 167772175;
pub const NTDDI_WIN11_GE: u32 = 167772176;
pub const NTDDI_WIN11_ZN: u32 = 167772174;
pub const NTDDI_WIN2K: u32 = 83886080;
pub const NTDDI_WIN2KSP1: u32 = 83886336;
pub const NTDDI_WIN2KSP2: u32 = 83886592;
pub const NTDDI_WIN2KSP3: u32 = 83886848;
pub const NTDDI_WIN2KSP4: u32 = 83887104;
pub const NTDDI_WIN4: u32 = 67108864;
pub const NTDDI_WIN6: u32 = 100663296;
pub const NTDDI_WIN6SP1: u32 = 100663552;
pub const NTDDI_WIN6SP2: u32 = 100663808;
pub const NTDDI_WIN6SP3: u32 = 100664064;
pub const NTDDI_WIN6SP4: u32 = 100664320;
pub const NTDDI_WIN7: u32 = 100728832;
pub const NTDDI_WIN8: u32 = 100794368;
pub const NTDDI_WINBLUE: u32 = 100859904;
pub const NTDDI_WINTHRESHOLD: u32 = 167772160;
pub const NTDDI_WINXP: u32 = 83951616;
pub const NTDDI_WINXPSP1: u32 = 83951872;
pub const NTDDI_WINXPSP2: u32 = 83952128;
pub const NTDDI_WINXPSP3: u32 = 83952384;
pub const NTDDI_WINXPSP4: u32 = 83952640;
pub const NTDDI_WS03: u32 = 84017152;
pub const NTDDI_WS03SP1: u32 = 84017408;
pub const NTDDI_WS03SP2: u32 = 84017664;
pub const NTDDI_WS03SP3: u32 = 84017920;
pub const NTDDI_WS03SP4: u32 = 84018176;
pub const NTDDI_WS08: u32 = 100663552;
pub const NTDDI_WS08SP2: u32 = 100663808;
pub const NTDDI_WS08SP3: u32 = 100664064;
pub const NTDDI_WS08SP4: u32 = 100664320;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
impl Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
impl Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
}
impl Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OSVERSION_MASK: u32 = 4294901760;
pub type OS_DEPLOYEMENT_STATE_VALUES = i32;
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = 2;
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = 1;
pub type OS_PRODUCT_TYPE = u32;
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = Option<unsafe extern "system" fn(lpbuffer: windows_sys::core::PSTR, usize: u32) -> u32>;
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = Option<unsafe extern "system" fn(lpbuffer: windows_sys::core::PWSTR, usize: u32) -> u32>;
pub type PROCESSOR_ARCHITECTURE = u16;
pub const PROCESSOR_ARCHITECTURE_ALPHA: PROCESSOR_ARCHITECTURE = 2;
pub const PROCESSOR_ARCHITECTURE_ALPHA64: PROCESSOR_ARCHITECTURE = 7;
pub const PROCESSOR_ARCHITECTURE_AMD64: PROCESSOR_ARCHITECTURE = 9;
pub const PROCESSOR_ARCHITECTURE_ARM: PROCESSOR_ARCHITECTURE = 5;
pub const PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64: PROCESSOR_ARCHITECTURE = 13;
pub const PROCESSOR_ARCHITECTURE_ARM64: PROCESSOR_ARCHITECTURE = 12;
pub const PROCESSOR_ARCHITECTURE_IA32_ON_ARM64: PROCESSOR_ARCHITECTURE = 14;
pub const PROCESSOR_ARCHITECTURE_IA32_ON_WIN64: PROCESSOR_ARCHITECTURE = 10;
pub const PROCESSOR_ARCHITECTURE_IA64: PROCESSOR_ARCHITECTURE = 6;
pub const PROCESSOR_ARCHITECTURE_INTEL: PROCESSOR_ARCHITECTURE = 0;
pub const PROCESSOR_ARCHITECTURE_MIPS: PROCESSOR_ARCHITECTURE = 1;
pub const PROCESSOR_ARCHITECTURE_MSIL: PROCESSOR_ARCHITECTURE = 8;
pub const PROCESSOR_ARCHITECTURE_NEUTRAL: PROCESSOR_ARCHITECTURE = 11;
pub const PROCESSOR_ARCHITECTURE_PPC: PROCESSOR_ARCHITECTURE = 3;
pub const PROCESSOR_ARCHITECTURE_SHX: PROCESSOR_ARCHITECTURE = 4;
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: PROCESSOR_ARCHITECTURE = 65535;
pub type PROCESSOR_CACHE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: usize,
}
impl Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
impl Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRODUCT_ARM64_SERVER: OS_PRODUCT_TYPE = 120;
pub const PRODUCT_AZURESTACKHCI_SERVER_CORE: OS_PRODUCT_TYPE = 406;
pub const PRODUCT_AZURE_NANO_SERVER: OS_PRODUCT_TYPE = 169;
pub const PRODUCT_AZURE_SERVER_AGENTBRIDGE: OS_PRODUCT_TYPE = 208;
pub const PRODUCT_AZURE_SERVER_CLOUDHOST: OS_PRODUCT_TYPE = 199;
pub const PRODUCT_AZURE_SERVER_CLOUDMOS: OS_PRODUCT_TYPE = 200;
pub const PRODUCT_AZURE_SERVER_CORE: OS_PRODUCT_TYPE = 168;
pub const PRODUCT_AZURE_SERVER_NANOHOST: OS_PRODUCT_TYPE = 209;
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = 6;
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = 16;
pub const PRODUCT_CLOUD: OS_PRODUCT_TYPE = 178;
pub const PRODUCT_CLOUDE: OS_PRODUCT_TYPE = 183;
pub const PRODUCT_CLOUDEDITION: OS_PRODUCT_TYPE = 203;
pub const PRODUCT_CLOUDEDITIONN: OS_PRODUCT_TYPE = 202;
pub const PRODUCT_CLOUDEN: OS_PRODUCT_TYPE = 186;
pub const PRODUCT_CLOUDN: OS_PRODUCT_TYPE = 179;
pub const PRODUCT_CLOUD_HOST_INFRASTRUCTURE_SERVER: OS_PRODUCT_TYPE = 124;
pub const PRODUCT_CLOUD_STORAGE_SERVER: OS_PRODUCT_TYPE = 110;
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = 18;
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = 64;
pub const PRODUCT_CONNECTED_CAR: OS_PRODUCT_TYPE = 117;
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = 101;
pub const PRODUCT_CORE_ARM: OS_PRODUCT_TYPE = 97;
pub const PRODUCT_CORE_CONNECTED: OS_PRODUCT_TYPE = 111;
pub const PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = 116;
pub const PRODUCT_CORE_CONNECTED_N: OS_PRODUCT_TYPE = 113;
pub const PRODUCT_CORE_CONNECTED_SINGLELANGUAGE: OS_PRODUCT_TYPE = 115;
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = 99;
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = 98;
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = 100;
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = 145;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = 80;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER_CORE: OS_PRODUCT_TYPE = 159;
pub const PRODUCT_DATACENTER_NANO_SERVER: OS_PRODUCT_TYPE = 143;
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = 8;
pub const PRODUCT_DATACENTER_SERVER_AZURE_EDITION: OS_PRODUCT_TYPE = 407;
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = 12;
pub const PRODUCT_DATACENTER_SERVER_CORE_AZURE_EDITION: OS_PRODUCT_TYPE = 408;
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = 39;
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = 37;
pub const PRODUCT_DATACENTER_WS_SERVER_CORE: OS_PRODUCT_TYPE = 147;
pub const PRODUCT_DATACENTER_WS_SERVER_CORE_AZURE_EDITION: OS_PRODUCT_TYPE = 409;
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = 121;
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = 122;
pub const PRODUCT_EMBEDDED: OS_PRODUCT_TYPE = 65;
pub const PRODUCT_EMBEDDED_A: OS_PRODUCT_TYPE = 88;
pub const PRODUCT_EMBEDDED_AUTOMOTIVE: OS_PRODUCT_TYPE = 85;
pub const PRODUCT_EMBEDDED_E: OS_PRODUCT_TYPE = 90;
pub const PRODUCT_EMBEDDED_EVAL: OS_PRODUCT_TYPE = 107;
pub const PRODUCT_EMBEDDED_E_EVAL: OS_PRODUCT_TYPE = 108;
pub const PRODUCT_EMBEDDED_INDUSTRY: OS_PRODUCT_TYPE = 89;
pub const PRODUCT_EMBEDDED_INDUSTRY_A: OS_PRODUCT_TYPE = 86;
pub const PRODUCT_EMBEDDED_INDUSTRY_A_E: OS_PRODUCT_TYPE = 92;
pub const PRODUCT_EMBEDDED_INDUSTRY_E: OS_PRODUCT_TYPE = 91;
pub const PRODUCT_EMBEDDED_INDUSTRY_EVAL: OS_PRODUCT_TYPE = 105;
pub const PRODUCT_EMBEDDED_INDUSTRY_E_EVAL: OS_PRODUCT_TYPE = 106;
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = 4;
pub const PRODUCT_ENTERPRISEG: OS_PRODUCT_TYPE = 171;
pub const PRODUCT_ENTERPRISEGN: OS_PRODUCT_TYPE = 172;
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = 70;
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = 72;
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = 27;
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = 84;
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = 125;
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = 10;
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = 14;
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = 41;
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = 15;
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = 38;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION: OS_PRODUCT_TYPE = 140;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION_N: OS_PRODUCT_TYPE = 141;
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = 129;
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = 126;
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = 130;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = 60;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = 62;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = 59;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = 61;
pub const PRODUCT_HOLOGRAPHIC: OS_PRODUCT_TYPE = 135;
pub const PRODUCT_HOLOGRAPHIC_BUSINESS: OS_PRODUCT_TYPE = 136;
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = 2;
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = 67;
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = 5;
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = 3;
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = 68;
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = 26;
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = 34;
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = 19;
pub const PRODUCT_HUBOS: OS_PRODUCT_TYPE = 180;
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = 42;
pub const PRODUCT_INDUSTRY_HANDHELD: OS_PRODUCT_TYPE = 118;
pub const PRODUCT_IOTEDGEOS: OS_PRODUCT_TYPE = 187;
pub const PRODUCT_IOTENTERPRISE: OS_PRODUCT_TYPE = 188;
pub const PRODUCT_IOTENTERPRISEK: OS_PRODUCT_TYPE = 206;
pub const PRODUCT_IOTENTERPRISES: OS_PRODUCT_TYPE = 191;
pub const PRODUCT_IOTENTERPRISESEVAL: OS_PRODUCT_TYPE = 207;
pub const PRODUCT_IOTENTERPRISESK: OS_PRODUCT_TYPE = 205;
pub const PRODUCT_IOTOS: OS_PRODUCT_TYPE = 185;
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = 123;
pub const PRODUCT_LITE: OS_PRODUCT_TYPE = 189;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = 30;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = 32;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = 31;
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = 77;
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = 76;
pub const PRODUCT_NANO_SERVER: OS_PRODUCT_TYPE = 109;
pub const PRODUCT_ONECOREUPDATEOS: OS_PRODUCT_TYPE = 182;
pub const PRODUCT_PPI_PRO: OS_PRODUCT_TYPE = 119;
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = 48;
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = 69;
pub const PRODUCT_PROFESSIONAL_EMBEDDED: OS_PRODUCT_TYPE = 58;
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = 49;
pub const PRODUCT_PROFESSIONAL_S: OS_PRODUCT_TYPE = 127;
pub const PRODUCT_PROFESSIONAL_STUDENT: OS_PRODUCT_TYPE = 112;
pub const PRODUCT_PROFESSIONAL_STUDENT_N: OS_PRODUCT_TYPE = 114;
pub const PRODUCT_PROFESSIONAL_S_N: OS_PRODUCT_TYPE = 128;
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = 103;
pub const PRODUCT_PRO_CHINA: OS_PRODUCT_TYPE = 139;
pub const PRODUCT_PRO_FOR_EDUCATION: OS_PRODUCT_TYPE = 164;
pub const PRODUCT_PRO_FOR_EDUCATION_N: OS_PRODUCT_TYPE = 165;
pub const PRODUCT_PRO_SINGLE_LANGUAGE: OS_PRODUCT_TYPE = 138;
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = 161;
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = 162;
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = 50;
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = 54;
pub const PRODUCT_SERVERRDSH: OS_PRODUCT_TYPE = 175;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = 51;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = 55;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = 24;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = 35;
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = 33;
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = 9;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = 25;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = 63;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = 56;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE: OS_PRODUCT_TYPE = 57;
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = 146;
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = 79;
pub const PRODUCT_STANDARD_EVALUATION_SERVER_CORE: OS_PRODUCT_TYPE = 160;
pub const PRODUCT_STANDARD_NANO_SERVER: OS_PRODUCT_TYPE = 144;
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = 7;
pub const PRODUCT_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = 13;
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = 40;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = 52;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = 53;
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = 36;
pub const PRODUCT_STANDARD_WS_SERVER_CORE: OS_PRODUCT_TYPE = 148;
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = 11;
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = 66;
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = 47;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = 23;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = 46;
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = 20;
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = 43;
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = 96;
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = 21;
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = 44;
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = 95;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = 22;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = 45;
pub const PRODUCT_THINPC: OS_PRODUCT_TYPE = 87;
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = 1;
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = 71;
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = 28;
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = 0;
pub const PRODUCT_UNLICENSED: OS_PRODUCT_TYPE = 2882382797;
pub const PRODUCT_UTILITY_VM: OS_PRODUCT_TYPE = 149;
pub const PRODUCT_VALIDATION: OS_PRODUCT_TYPE = 204;
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = 17;
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = 29;
pub const PRODUCT_WNC: OS_PRODUCT_TYPE = 210;
pub const PRODUCT_XBOX_DURANGOHOSTOS: OS_PRODUCT_TYPE = 196;
pub const PRODUCT_XBOX_ERAOS: OS_PRODUCT_TYPE = 195;
pub const PRODUCT_XBOX_GAMEOS: OS_PRODUCT_TYPE = 194;
pub const PRODUCT_XBOX_KEYSTONE: OS_PRODUCT_TYPE = 198;
pub const PRODUCT_XBOX_SCARLETTHOSTOS: OS_PRODUCT_TYPE = 197;
pub const PRODUCT_XBOX_SYSTEMOS: OS_PRODUCT_TYPE = 192;
pub const RSMB: FIRMWARE_TABLE_PROVIDER = 1381190978;
pub type RTL_SYSTEM_GLOBAL_DATA_ID = i32;
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = 65535;
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = 2;
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = 4;
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = 1;
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = 6;
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = 0;
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = 5;
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = 7;
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = 3;
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1;
pub const SPVERSION_MASK: u32 = 65280;
pub const SUBVERSION_MASK: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
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
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
impl Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
impl Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: PROCESSOR_ARCHITECTURE,
    pub wReserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub Flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub NodeNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
pub type USER_CET_ENVIRONMENT = u32;
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = 2;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = 17;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = 16;
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = 0;
pub const VER_BUILDNUMBER: VER_FLAGS = 4;
pub type VER_FLAGS = u32;
pub const VER_MAJORVERSION: VER_FLAGS = 2;
pub const VER_MINORVERSION: VER_FLAGS = 1;
pub const VER_PLATFORMID: VER_FLAGS = 8;
pub const VER_PRODUCT_TYPE: VER_FLAGS = 128;
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = 32;
pub const VER_SERVICEPACKMINOR: VER_FLAGS = 16;
pub const VER_SUITENAME: VER_FLAGS = 64;
pub const WDK_NTDDI_VERSION: u32 = 167772176;
pub const _WIN32_IE_IE100: u32 = 2560;
pub const _WIN32_IE_IE110: u32 = 2560;
pub const _WIN32_IE_IE20: u32 = 512;
pub const _WIN32_IE_IE30: u32 = 768;
pub const _WIN32_IE_IE302: u32 = 770;
pub const _WIN32_IE_IE40: u32 = 1024;
pub const _WIN32_IE_IE401: u32 = 1025;
pub const _WIN32_IE_IE50: u32 = 1280;
pub const _WIN32_IE_IE501: u32 = 1281;
pub const _WIN32_IE_IE55: u32 = 1360;
pub const _WIN32_IE_IE60: u32 = 1536;
pub const _WIN32_IE_IE60SP1: u32 = 1537;
pub const _WIN32_IE_IE60SP2: u32 = 1539;
pub const _WIN32_IE_IE70: u32 = 1792;
pub const _WIN32_IE_IE80: u32 = 2048;
pub const _WIN32_IE_IE90: u32 = 2304;
pub const _WIN32_IE_LONGHORN: u32 = 1792;
pub const _WIN32_IE_NT4: u32 = 512;
pub const _WIN32_IE_NT4SP1: u32 = 512;
pub const _WIN32_IE_NT4SP2: u32 = 512;
pub const _WIN32_IE_NT4SP3: u32 = 770;
pub const _WIN32_IE_NT4SP4: u32 = 1025;
pub const _WIN32_IE_NT4SP5: u32 = 1025;
pub const _WIN32_IE_NT4SP6: u32 = 1280;
pub const _WIN32_IE_WIN10: u32 = 2560;
pub const _WIN32_IE_WIN2K: u32 = 1281;
pub const _WIN32_IE_WIN2KSP1: u32 = 1281;
pub const _WIN32_IE_WIN2KSP2: u32 = 1281;
pub const _WIN32_IE_WIN2KSP3: u32 = 1281;
pub const _WIN32_IE_WIN2KSP4: u32 = 1281;
pub const _WIN32_IE_WIN6: u32 = 1792;
pub const _WIN32_IE_WIN7: u32 = 2048;
pub const _WIN32_IE_WIN8: u32 = 2560;
pub const _WIN32_IE_WIN98: u32 = 1025;
pub const _WIN32_IE_WIN98SE: u32 = 1280;
pub const _WIN32_IE_WINBLUE: u32 = 2560;
pub const _WIN32_IE_WINME: u32 = 1360;
pub const _WIN32_IE_WINTHRESHOLD: u32 = 2560;
pub const _WIN32_IE_WS03: u32 = 1538;
pub const _WIN32_IE_WS03SP1: u32 = 1539;
pub const _WIN32_IE_XP: u32 = 1536;
pub const _WIN32_IE_XPSP1: u32 = 1537;
pub const _WIN32_IE_XPSP2: u32 = 1539;
pub const _WIN32_WINNT_LONGHORN: u32 = 1536;
pub const _WIN32_WINNT_NT4: u32 = 1024;
pub const _WIN32_WINNT_VISTA: u32 = 1536;
pub const _WIN32_WINNT_WIN10: u32 = 2560;
pub const _WIN32_WINNT_WIN2K: u32 = 1280;
pub const _WIN32_WINNT_WIN6: u32 = 1536;
pub const _WIN32_WINNT_WIN7: u32 = 1537;
pub const _WIN32_WINNT_WIN8: u32 = 1538;
pub const _WIN32_WINNT_WINBLUE: u32 = 1539;
pub const _WIN32_WINNT_WINTHRESHOLD: u32 = 2560;
pub const _WIN32_WINNT_WINXP: u32 = 1281;
pub const _WIN32_WINNT_WS03: u32 = 1282;
pub const _WIN32_WINNT_WS08: u32 = 1536;
