windows_link::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_sys::core::PCWSTR, computername : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : u32, pfirmwaretableenumbuffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-7.dll" "system" fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetIntegratedDisplaySize(sizeininches : *mut f64) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : *mut super::minwinbase::SYSTEMTIME));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : *mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : super::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, buffer : *mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
windows_link::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : u16, buffer : *mut super::winnt::SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : u32, dwosminorversion : u32, dwspmajorversion : u32, dwspminorversion : u32, pdwreturnedproducttype : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetRuntimeAttestationReport(nonce : *const u8, packageversion : u16, reporttypesbitmap : u64, reportbuffer : *mut core::ffi::c_void, reportbuffersize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : u32, firmwaretableid : u32, pfirmwaretablebuffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
windows_link::link!("kernel32.dll" "system" fn GetSystemLeapSecondInformation(enabled : *mut windows_sys::core::BOOL, flags : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : *mut super::minwinbase::SYSTEMTIME));
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : *mut u32, lptimeincrement : *mut u32, lptimeadjustmentdisabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : *mut u64, lptimeincrement : *mut u64, lptimeadjustmentdisabled : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : *mut super::minwindef::FILETIME));
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : *mut super::minwindef::FILETIME));
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
windows_link::link!("kernel32.dll" "system" fn GetTickCount64() -> u64);
windows_link::link!("kernel32.dll" "system" fn GetVersion() -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : *mut super::winnt::OSVERSIONINFOA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : *mut super::winnt::OSVERSIONINFOW) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : *mut MEMORYSTATUSEX) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn InstallELAMCertificateInfo(elamfile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : u32, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetComputerNameW(lpcomputername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_minwinbase")]
windows_link::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : u32, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : u64, btimeadjustmentdisabled : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
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
#[cfg(feature = "Win32_winnt")]
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEMORYSTATUSEX {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub ullTotalPhys: super::winnt::DWORDLONG,
    pub ullAvailPhys: super::winnt::DWORDLONG,
    pub ullTotalPageFile: super::winnt::DWORDLONG,
    pub ullAvailPageFile: super::winnt::DWORDLONG,
    pub ullTotalVirtual: super::winnt::DWORDLONG,
    pub ullAvailVirtual: super::winnt::DWORDLONG,
    pub ullAvailExtendedVirtual: super::winnt::DWORDLONG,
}
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1;
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
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: u16,
    pub wReserved: u16,
}
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: u32 = 2;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: u32 = 17;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: u32 = 16;
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: u32 = 0;
