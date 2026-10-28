#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<P0>(hostname: P0, computername: Option<windows_core::PWSTR>, nsize: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_core::PCWSTR, computername : windows_core::PWSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameExW(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: super::DWORD, pfirmwaretableenumbuffer: Option<super::PVOID>, buffersize: super::DWORD) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : super::DWORD, pfirmwaretableenumbuffer : super::PVOID, buffersize : super::DWORD) -> super::UINT);
    unsafe { EnumSystemFirmwareTables(firmwaretableprovidersignature, pfirmwaretableenumbuffer.unwrap_or(core::mem::zeroed()) as _, buffersize) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PSTR>, nsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetComputerNameExA(nametype, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PWSTR>, nsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PWSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetComputerNameExW(nametype, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-7.dll" "system" fn GetDeveloperDriveEnablementState() -> DEVELOPER_DRIVE_ENABLEMENT_STATE);
    unsafe { GetDeveloperDriveEnablementState() }
}
#[inline]
pub unsafe fn GetIntegratedDisplaySize() -> windows_core::Result<f64> {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetIntegratedDisplaySize(sizeininches : *mut f64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetIntegratedDisplaySize(&mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetLocalTime() -> super::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : super::LPSYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetLocalTime(&mut result__);
        result__
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: Option<super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION>, returnedlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformation(buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: super::LOGICAL_PROCESSOR_RELATIONSHIP, buffer: Option<super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>, returnedlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : super::LOGICAL_PROCESSOR_RELATIONSHIP, buffer : super::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformationEx(relationshiptype, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: LPSYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : LPSYSTEM_INFO));
    unsafe { GetNativeSystemInfo(lpsysteminfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetOsManufacturingMode(pbenabled as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetOsSafeBootMode(flags as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: super::PULONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : super::PULONGLONG) -> windows_core::BOOL);
    unsafe { GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: super::USHORT, buffer: Option<super::PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>, returnedlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : super::USHORT, buffer : super::PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetProcessorSystemCycleTime(group, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: super::DWORD, dwosminorversion: super::DWORD, dwspmajorversion: super::DWORD, dwspminorversion: super::DWORD, pdwreturnedproducttype: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : super::DWORD, dwosminorversion : super::DWORD, dwspmajorversion : super::DWORD, dwspminorversion : super::DWORD, pdwreturnedproducttype : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetProductInfo(dwosmajorversion, dwosminorversion, dwspmajorversion, dwspminorversion, pdwreturnedproducttype as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetRuntimeAttestationReport(nonce: Option<*const super::UCHAR>, packageversion: super::UINT16, reporttypesbitmap: super::UINT64, reportbuffer: Option<super::PVOID>, reportbuffersize: super::PUINT32) -> windows_core::BOOL {
    windows_core::link!("kernelbase.dll" "system" fn GetRuntimeAttestationReport(nonce : *const super::UCHAR, packageversion : super::UINT16, reporttypesbitmap : super::UINT64, reportbuffer : super::PVOID, reportbuffersize : super::PUINT32) -> windows_core::BOOL);
    unsafe { GetRuntimeAttestationReport(nonce.unwrap_or(core::mem::zeroed()) as _, packageversion, reporttypesbitmap, reportbuffer.unwrap_or(core::mem::zeroed()) as _, reportbuffersize as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: Option<windows_core::PSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_core::PSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetSystemDirectoryA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: Option<windows_core::PWSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_core::PWSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetSystemDirectoryW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetSystemFirmwareTable(firmwaretableprovidersignature: super::DWORD, firmwaretableid: super::DWORD, pfirmwaretablebuffer: Option<super::PVOID>, buffersize: super::DWORD) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : super::DWORD, firmwaretableid : super::DWORD, pfirmwaretablebuffer : super::PVOID, buffersize : super::DWORD) -> super::UINT);
    unsafe { GetSystemFirmwareTable(firmwaretableprovidersignature, firmwaretableid, pfirmwaretablebuffer.unwrap_or(core::mem::zeroed()) as _, buffersize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: LPSYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : LPSYSTEM_INFO));
    unsafe { GetSystemInfo(lpsysteminfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: super::PBOOL, flags: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernelbase.dll" "system" fn GetSystemLeapSecondInformation(enabled : super::PBOOL, flags : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetSystemLeapSecondInformation(enabled as _, flags as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemTime() -> super::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : super::LPSYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTime(&mut result__);
        result__
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: super::PDWORD, lptimeincrement: super::PDWORD, lptimeadjustmentdisabled: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : super::PDWORD, lptimeincrement : super::PDWORD, lptimeadjustmentdisabled : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustment(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: super::PDWORD64, lptimeincrement: super::PDWORD64, lptimeadjustmentdisabled: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : super::PDWORD64, lptimeincrement : super::PDWORD64, lptimeadjustmentdisabled : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustmentPrecise(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemTimeAsFileTime() -> super::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : super::LPFILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimeAsFileTime(&mut result__);
        result__
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime() -> super::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : super::LPFILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimePreciseAsFileTime(&mut result__);
        result__
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: Option<windows_core::PSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetSystemWindowsDirectoryA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: Option<windows_core::PWSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetSystemWindowsDirectoryW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTickCount() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount() -> super::DWORD);
    unsafe { GetTickCount() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetTickCount64() -> super::ULONGLONG {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount64() -> super::ULONGLONG);
    unsafe { GetTickCount64() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetVersion() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetVersion() -> super::DWORD);
    unsafe { GetVersion() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: super::LPOSVERSIONINFOA) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : super::LPOSVERSIONINFOA) -> windows_core::BOOL);
    unsafe { GetVersionExA(lpversioninformation as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: super::LPOSVERSIONINFOW) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : super::LPOSVERSIONINFOW) -> windows_core::BOOL);
    unsafe { GetVersionExW(lpversioninformation as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: Option<windows_core::PSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetWindowsDirectoryA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: Option<windows_core::PWSTR>, usize: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : super::UINT) -> super::UINT);
    unsafe { GetWindowsDirectoryW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, usize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: LPMEMORYSTATUSEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : LPMEMORYSTATUSEX) -> windows_core::BOOL);
    unsafe { GlobalMemoryStatusEx(lpbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InstallELAMCertificateInfo(elamfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InstallELAMCertificateInfo(elamfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { InstallELAMCertificateInfo(elamfile) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : super::DWORD) -> windows_core::BOOL);
    unsafe { IsUserCetAvailableInEnvironment(usercetenvironment) }
}
#[inline]
pub unsafe fn SetComputerNameA<P0>(lpcomputername: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameA(lpcomputername.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetComputerNameEx2W<P2>(nametype: COMPUTER_NAME_FORMAT, flags: super::DWORD, lpbuffer: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : super::DWORD, lpbuffer : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameEx2W(nametype, flags, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn SetComputerNameExA<P1>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameExA(nametype, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn SetComputerNameExW<P1>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameExW(nametype, lpbuffer.param().abi()) }
}
#[inline]
pub unsafe fn SetComputerNameW<P0>(lpcomputername: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameW(lpcomputername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetComputerNameW(lpcomputername.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetLocalTime(lpsystemtime) }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetSystemTime(lpsystemtime) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetSystemTimeAdjustment(dwtimeadjustment: super::DWORD, btimeadjustmentdisabled: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : super::DWORD, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemTimeAdjustment(dwtimeadjustment, btimeadjustmentdisabled.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: super::DWORD64, btimeadjustmentdisabled: bool) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : super::DWORD64, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemTimeAdjustmentPrecise(dwtimeadjustment, btimeadjustmentdisabled.into()) }
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: super::WORD,
    pub wReserved: super::WORD,
}
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: i32 = 2;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: i32 = 17;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: i32 = 16;
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: i32 = 0;
