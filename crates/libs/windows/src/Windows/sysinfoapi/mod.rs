#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<P0>(hostname: P0, computername: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameExW(hostname : windows_core::PCWSTR, computername : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameExW(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: u32, pfirmwaretableenumbuffer: Option<*mut core::ffi::c_void>, buffersize: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn EnumSystemFirmwareTables(firmwaretableprovidersignature : u32, pfirmwaretableenumbuffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
    unsafe { EnumSystemFirmwareTables(firmwaretableprovidersignature, pfirmwaretableenumbuffer.unwrap_or(core::mem::zeroed()) as _, buffersize) }
}
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PSTR>, nsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExA(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetComputerNameExA(nametype, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: Option<windows_core::PWSTR>, nsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameExW(nametype : COMPUTER_NAME_FORMAT, lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
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
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetLocalTime() -> super::minwinbase::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime : *mut super::minwinbase::SYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetLocalTime(&mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: Option<*mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION>, returnedlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformation(buffer : *mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformation(buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: super::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, buffer: Option<*mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>, returnedlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalProcessorInformationEx(relationshiptype : super::winnt::LOGICAL_PROCESSOR_RELATIONSHIP, buffer : *mut super::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogicalProcessorInformationEx(relationshiptype, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
    unsafe { GetNativeSystemInfo(lpsysteminfo as _) }
}
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-3.dll" "system" fn GetOsManufacturingMode(pbenabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOsManufacturingMode(pbenabled as _) }
}
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-0.dll" "system" fn GetOsSafeBootMode(flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetOsSafeBootMode(flags as _) }
}
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes : *mut u64) -> windows_core::BOOL);
    unsafe { GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: u16, buffer: Option<*mut super::winnt::SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>, returnedlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessorSystemCycleTime(group : u16, buffer : *mut super::winnt::SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, returnedlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessorSystemCycleTime(group, buffer.unwrap_or(core::mem::zeroed()) as _, returnedlength as _) }
}
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProductInfo(dwosmajorversion : u32, dwosminorversion : u32, dwspmajorversion : u32, dwspminorversion : u32, pdwreturnedproducttype : *mut u32) -> windows_core::BOOL);
    unsafe { GetProductInfo(dwosmajorversion, dwosminorversion, dwspmajorversion, dwspminorversion, pdwreturnedproducttype as _) }
}
#[inline]
pub unsafe fn GetRuntimeAttestationReport(nonce: Option<*const u8>, packageversion: u16, reporttypesbitmap: u64, reportbuffer: Option<*mut core::ffi::c_void>, reportbuffersize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetRuntimeAttestationReport(nonce : *const u8, packageversion : u16, reporttypesbitmap : u64, reportbuffer : *mut core::ffi::c_void, reportbuffersize : *mut u32) -> windows_core::BOOL);
    unsafe { GetRuntimeAttestationReport(nonce.unwrap_or(core::mem::zeroed()) as _, packageversion, reporttypesbitmap, reportbuffer.unwrap_or(core::mem::zeroed()) as _, reportbuffersize as _) }
}
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemFirmwareTable(firmwaretableprovidersignature: u32, firmwaretableid: u32, pfirmwaretablebuffer: Option<*mut core::ffi::c_void>, buffersize: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFirmwareTable(firmwaretableprovidersignature : u32, firmwaretableid : u32, pfirmwaretablebuffer : *mut core::ffi::c_void, buffersize : u32) -> u32);
    unsafe { GetSystemFirmwareTable(firmwaretableprovidersignature, firmwaretableid, pfirmwaretablebuffer.unwrap_or(core::mem::zeroed()) as _, buffersize) }
}
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
    unsafe { GetSystemInfo(lpsysteminfo as _) }
}
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: *mut windows_core::BOOL, flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemLeapSecondInformation(enabled : *mut windows_core::BOOL, flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemLeapSecondInformation(enabled as _, flags as _) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn GetSystemTime() -> super::minwinbase::SYSTEMTIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTime(lpsystemtime : *mut super::minwinbase::SYSTEMTIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAdjustment(lptimeadjustment : *mut u32, lptimeincrement : *mut u32, lptimeadjustmentdisabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustment(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _) }
}
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn GetSystemTimeAdjustmentPrecise(lptimeadjustment : *mut u64, lptimeincrement : *mut u64, lptimeadjustmentdisabled : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetSystemTimeAdjustmentPrecise(lptimeadjustment as _, lptimeincrement as _, lptimeadjustmentdisabled as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSystemTimeAsFileTime() -> super::minwindef::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime : *mut super::minwindef::FILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimeAsFileTime(&mut result__);
        result__
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime() -> super::minwindef::FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : *mut super::minwindef::FILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimePreciseAsFileTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetSystemWindowsDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetSystemWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetSystemWindowsDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
    unsafe { GetTickCount() }
}
#[inline]
pub unsafe fn GetTickCount64() -> u64 {
    windows_core::link!("kernel32.dll" "system" fn GetTickCount64() -> u64);
    unsafe { GetTickCount64() }
}
#[inline]
pub unsafe fn GetVersion() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetVersion() -> u32);
    unsafe { GetVersion() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: *mut super::winnt::OSVERSIONINFOA) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExA(lpversioninformation : *mut super::winnt::OSVERSIONINFOA) -> windows_core::BOOL);
    unsafe { GetVersionExA(lpversioninformation as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: *mut super::winnt::OSVERSIONINFOW) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetVersionExW(lpversioninformation : *mut super::winnt::OSVERSIONINFOW) -> windows_core::BOOL);
    unsafe { GetVersionExW(lpversioninformation as _) }
}
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryA(lpbuffer : windows_core::PSTR, usize : u32) -> u32);
    unsafe { GetWindowsDirectoryA(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWindowsDirectoryW(lpbuffer : windows_core::PWSTR, usize : u32) -> u32);
    unsafe { GetWindowsDirectoryW(core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalMemoryStatusEx(lpbuffer : *mut MEMORYSTATUSEX) -> windows_core::BOOL);
    unsafe { GlobalMemoryStatusEx(lpbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InstallELAMCertificateInfo(elamfile: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InstallELAMCertificateInfo(elamfile : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { InstallELAMCertificateInfo(elamfile) }
}
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsUserCetAvailableInEnvironment(usercetenvironment : u32) -> windows_core::BOOL);
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
#[inline]
pub unsafe fn SetComputerNameEx2W<P2>(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetComputerNameEx2W(nametype : COMPUTER_NAME_FORMAT, flags : u32, lpbuffer : windows_core::PCWSTR) -> windows_core::BOOL);
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
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetLocalTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetLocalTime(lpsystemtime) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::minwinbase::SYSTEMTIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTime(lpsystemtime : *const super::minwinbase::SYSTEMTIME) -> windows_core::BOOL);
    unsafe { SetSystemTime(lpsystemtime) }
}
#[inline]
pub unsafe fn SetSystemTimeAdjustment(dwtimeadjustment: u32, btimeadjustmentdisabled: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemTimeAdjustment(dwtimeadjustment : u32, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemTimeAdjustment(dwtimeadjustment, btimeadjustmentdisabled.into()) }
}
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment: u64, btimeadjustmentdisabled: bool) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-sysinfo-l1-2-4.dll" "system" fn SetSystemTimeAdjustmentPrecise(dwtimeadjustment : u64, btimeadjustmentdisabled : windows_core::BOOL) -> windows_core::BOOL);
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
#[cfg(feature = "winnt")]
pub type LPMEMORYSTATUSEX = *mut MEMORYSTATUSEX;
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: u16,
    pub wReserved: u16,
}
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: u32 = 2;
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: u32 = 17;
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: u32 = 16;
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: u32 = 0;
