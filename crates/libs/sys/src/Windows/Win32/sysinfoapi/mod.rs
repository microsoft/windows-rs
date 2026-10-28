#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_sys::core::PCWSTR, computername : windows_sys::core::PWSTR, nsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : super::DWORD, pfirmwaretableenumbuffer : super::PVOID, buffersize : super::DWORD) -> super::UINT);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PSTR, nsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PWSTR, nsize : super::LPDWORD) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-7.dll" "system" fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetIntegratedDisplaySize(sizeininches : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : super::LPSYSTEMTIME));
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : super::LOGICAL_PROCESSOR_RELATIONSHIP, buffer : super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : LPSYSTEM_INFO));
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : super::PULONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : super::USHORT, buffer : super::PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : super::DWORD, dwosminorversion : super::DWORD, dwspmajorversion : super::DWORD, dwspminorversion : super::DWORD, pdwreturnedproducttype : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernelbase.dll" "system" fn GetRuntimeAttestationReport(nonce : *const super::UCHAR, packageversion : super::UINT16, reporttypesbitmap : super::UINT64, reportbuffer : super::PVOID, reportbuffersize : super::PUINT32) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : super::UINT) -> super::UINT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : super::DWORD, firmwaretableid : super::DWORD, pfirmwaretablebuffer : super::PVOID, buffersize : super::DWORD) -> super::UINT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : LPSYSTEM_INFO));
#[cfg(feature = "minwindef")]
windows_link::link!("kernelbase.dll" "system" fn GetSystemLeapSecondInformation(enabled : super::PBOOL, flags : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : super::LPSYSTEMTIME));
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : super::PDWORD, lptimeincrement : super::PDWORD, lptimeadjustmentdisabled : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : super::PDWORD64, lptimeincrement : super::PDWORD64, lptimeadjustmentdisabled : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : super::LPFILETIME));
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : super::LPFILETIME));
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetTickCount64() -> super::ULONGLONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetVersion() -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : super::LPOSVERSIONINFOA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : super::LPOSVERSIONINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : LPMEMORYSTATUSEX) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InstallELAMCertificateInfo(elamfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : super::DWORD, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameW(lpcomputername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : super::DWORD, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : super::DWORD64, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type COMPUTER_NAME_FORMAT = i32;
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = 2;
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = 3;
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = 1;
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = 8;
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = 0;
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = 6;
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = 7;
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = 5;
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = 4;
pub type DEVELOPER_DRIVE_ENABLEMENT_STATE = i32;
pub const DeveloperDriveDisabledByGroupPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = 3;
pub const DeveloperDriveDisabledBySystemPolicy: DEVELOPER_DRIVE_ENABLEMENT_STATE = 2;
pub const DeveloperDriveEnabled: DEVELOPER_DRIVE_ENABLEMENT_STATE = 1;
pub const DeveloperDriveEnablementStateError: DEVELOPER_DRIVE_ENABLEMENT_STATE = 0;
#[cfg(feature = "shared")]
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEMORYSTATUSEX {
    pub dwLength: super::DWORD,
    pub dwMemoryLoad: super::DWORD,
    pub ullTotalPhys: super::DWORDLONG,
    pub ullAvailPhys: super::DWORDLONG,
    pub ullTotalPageFile: super::DWORDLONG,
    pub ullAvailPageFile: super::DWORDLONG,
    pub ullTotalVirtual: super::DWORDLONG,
    pub ullAvailVirtual: super::DWORDLONG,
    pub ullAvailExtendedVirtual: super::DWORDLONG,
}
pub const SCEX2_ALT_NETBIOS_NAME: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: super::DWORD,
    pub lpMinimumApplicationAddress: super::LPVOID,
    pub lpMaximumApplicationAddress: super::LPVOID,
    pub dwActiveProcessorMask: super::DWORD_PTR,
    pub dwNumberOfProcessors: super::DWORD,
    pub dwProcessorType: super::DWORD,
    pub dwAllocationGranularity: super::DWORD,
    pub wProcessorLevel: super::WORD,
    pub wProcessorRevision: super::WORD,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union SYSTEM_INFO_0 {
    pub dwOemId: super::DWORD,
    pub Anonymous: SYSTEM_INFO_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::WORD,
    pub wReserved: super::WORD,
}
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: i32 = 2;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: i32 = 17;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: i32 = 16;
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: i32 = 0;
