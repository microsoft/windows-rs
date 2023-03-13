#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameExW<P0>(hostname: P0, computername: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn DnsHostnameToComputerNameExW ( hostname : :: windows::core::PCWSTR , computername : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    DnsHostnameToComputerNameExW(hostname.into_param().abi(), ::core::mem::transmute(computername), nsize)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn EnumSystemFirmwareTables(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, pfirmwaretableenumbuffer: ::core::option::Option<*mut FIRMWARE_TABLE_ID>, buffersize: u32) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn EnumSystemFirmwareTables ( firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER , pfirmwaretableenumbuffer : *mut FIRMWARE_TABLE_ID , buffersize : u32 ) -> u32 );
    EnumSystemFirmwareTables(firmwaretableprovidersignature, ::core::mem::transmute(pfirmwaretableenumbuffer.unwrap_or(::std::ptr::null_mut())), buffersize)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExA(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows::core::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetComputerNameExA ( nametype : COMPUTER_NAME_FORMAT , lpbuffer : :: windows::core::PSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetComputerNameExA(nametype, ::core::mem::transmute(lpbuffer), nsize)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameExW(nametype: COMPUTER_NAME_FORMAT, lpbuffer: ::windows::core::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetComputerNameExW ( nametype : COMPUTER_NAME_FORMAT , lpbuffer : :: windows::core::PWSTR , nsize : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetComputerNameExW(nametype, ::core::mem::transmute(lpbuffer), nsize)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareType(firmwaretype: *mut FIRMWARE_TYPE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetFirmwareType ( firmwaretype : *mut FIRMWARE_TYPE ) -> super::super::Foundation:: BOOL );
    GetFirmwareType(firmwaretype)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetIntegratedDisplaySize() -> ::windows::core::Result<f64> {
    ::windows::imp::link ! ( "api-ms-win-core-sysinfo-l1-2-3.dll""system" fn GetIntegratedDisplaySize ( sizeininches : *mut f64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<f64>();
    GetIntegratedDisplaySize(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalTime() -> super::super::Foundation::SYSTEMTIME {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetLocalTime ( lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SYSTEMTIME>();
    GetLocalTime(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformation(buffer: ::core::option::Option<*mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION>, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetLogicalProcessorInformation ( buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION , returnedlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetLogicalProcessorInformation(::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), returnedlength)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLogicalProcessorInformationEx(relationshiptype: LOGICAL_PROCESSOR_RELATIONSHIP, buffer: ::core::option::Option<*mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX>, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetLogicalProcessorInformationEx ( relationshiptype : LOGICAL_PROCESSOR_RELATIONSHIP , buffer : *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX , returnedlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetLogicalProcessorInformationEx(relationshiptype, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), returnedlength)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetNativeSystemInfo ( lpsysteminfo : *mut SYSTEM_INFO ) -> ( ) );
    GetNativeSystemInfo(lpsysteminfo)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsManufacturingMode(pbenabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "api-ms-win-core-sysinfo-l1-2-3.dll""system" fn GetOsManufacturingMode ( pbenabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetOsManufacturingMode(pbenabled)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOsSafeBootMode(flags: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "api-ms-win-core-sysinfo-l1-2-0.dll""system" fn GetOsSafeBootMode ( flags : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetOsSafeBootMode(flags)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes: *mut u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetPhysicallyInstalledSystemMemory ( totalmemoryinkilobytes : *mut u64 ) -> super::super::Foundation:: BOOL );
    GetPhysicallyInstalledSystemMemory(totalmemoryinkilobytes)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessorSystemCycleTime(group: u16, buffer: ::core::option::Option<*mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION>, returnedlength: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProcessorSystemCycleTime ( group : u16 , buffer : *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION , returnedlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetProcessorSystemCycleTime(group, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), returnedlength)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProductInfo(dwosmajorversion: u32, dwosminorversion: u32, dwspmajorversion: u32, dwspminorversion: u32, pdwreturnedproducttype: *mut OS_PRODUCT_TYPE) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetProductInfo ( dwosmajorversion : u32 , dwosminorversion : u32 , dwspmajorversion : u32 , dwspminorversion : u32 , pdwreturnedproducttype : *mut OS_PRODUCT_TYPE ) -> super::super::Foundation:: BOOL );
    GetProductInfo(dwosmajorversion, dwosminorversion, dwspmajorversion, dwspminorversion, pdwreturnedproducttype)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemCpuSetInformation<P0>(information: ::core::option::Option<*mut SYSTEM_CPU_SET_INFORMATION>, bufferlength: u32, returnedlength: *mut u32, process: P0, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemCpuSetInformation ( information : *mut SYSTEM_CPU_SET_INFORMATION , bufferlength : u32 , returnedlength : *mut u32 , process : super::super::Foundation:: HANDLE , flags : u32 ) -> super::super::Foundation:: BOOL );
    GetSystemCpuSetInformation(::core::mem::transmute(information.unwrap_or(::std::ptr::null_mut())), bufferlength, returnedlength, process.into_param().abi(), flags)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemDEPPolicy ( ) -> DEP_SYSTEM_POLICY_TYPE );
    GetSystemDEPPolicy()
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemDirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemDirectoryA ( lpbuffer : :: windows::core::PSTR , usize : u32 ) -> u32 );
    GetSystemDirectoryA(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemDirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemDirectoryW ( lpbuffer : :: windows::core::PWSTR , usize : u32 ) -> u32 );
    GetSystemDirectoryW(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemFirmwareTable<P0>(firmwaretableprovidersignature: FIRMWARE_TABLE_PROVIDER, firmwaretableid: P0, pfirmwaretablebuffer: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32) -> u32
where
    P0: ::windows::core::IntoParam<FIRMWARE_TABLE_ID>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemFirmwareTable ( firmwaretableprovidersignature : FIRMWARE_TABLE_PROVIDER , firmwaretableid : FIRMWARE_TABLE_ID , pfirmwaretablebuffer : *mut ::core::ffi::c_void , buffersize : u32 ) -> u32 );
    GetSystemFirmwareTable(firmwaretableprovidersignature, firmwaretableid.into_param().abi(), ::core::mem::transmute(pfirmwaretablebuffer.unwrap_or(::std::ptr::null_mut())), buffersize)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[inline]
pub unsafe fn GetSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemInfo ( lpsysteminfo : *mut SYSTEM_INFO ) -> ( ) );
    GetSystemInfo(lpsysteminfo)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemLeapSecondInformation(enabled: *mut super::super::Foundation::BOOL, flags: *mut u32) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemLeapSecondInformation ( enabled : *mut super::super::Foundation:: BOOL , flags : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetSystemLeapSecondInformation(enabled, flags)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTime() -> super::super::Foundation::SYSTEMTIME {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemTime ( lpsystemtime : *mut super::super::Foundation:: SYSTEMTIME ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SYSTEMTIME>();
    GetSystemTime(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustment(lptimeadjustment: *mut u32, lptimeincrement: *mut u32, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemTimeAdjustment ( lptimeadjustment : *mut u32 , lptimeincrement : *mut u32 , lptimeadjustmentdisabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetSystemTimeAdjustment(lptimeadjustment, lptimeincrement, lptimeadjustmentdisabled)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAdjustmentPrecise(lptimeadjustment: *mut u64, lptimeincrement: *mut u64, lptimeadjustmentdisabled: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "api-ms-win-core-sysinfo-l1-2-4.dll""system" fn GetSystemTimeAdjustmentPrecise ( lptimeadjustment : *mut u64 , lptimeincrement : *mut u64 , lptimeadjustmentdisabled : *mut super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetSystemTimeAdjustmentPrecise(lptimeadjustment, lptimeincrement, lptimeadjustmentdisabled)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimeAsFileTime() -> super::super::Foundation::FILETIME {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemTimeAsFileTime ( lpsystemtimeasfiletime : *mut super::super::Foundation:: FILETIME ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    GetSystemTimeAsFileTime(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime() -> super::super::Foundation::FILETIME {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemTimePreciseAsFileTime ( lpsystemtimeasfiletime : *mut super::super::Foundation:: FILETIME ) -> ( ) );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
    GetSystemTimePreciseAsFileTime(&mut result__);
    ::std::mem::transmute(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemWindowsDirectoryA ( lpbuffer : :: windows::core::PSTR , usize : u32 ) -> u32 );
    GetSystemWindowsDirectoryA(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWindowsDirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemWindowsDirectoryW ( lpbuffer : :: windows::core::PWSTR , usize : u32 ) -> u32 );
    GetSystemWindowsDirectoryW(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWow64Directory2A(lpbuffer: ::core::option::Option<&mut [u8]>, imagefilemachinetype: IMAGE_FILE_MACHINE) -> u32 {
    ::windows::imp::link ! ( "api-ms-win-core-wow64-l1-1-1.dll""system" fn GetSystemWow64Directory2A ( lpbuffer : :: windows::core::PSTR , usize : u32 , imagefilemachinetype : IMAGE_FILE_MACHINE ) -> u32 );
    GetSystemWow64Directory2A(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), imagefilemachinetype)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWow64Directory2W(lpbuffer: ::core::option::Option<&mut [u16]>, imagefilemachinetype: IMAGE_FILE_MACHINE) -> u32 {
    ::windows::imp::link ! ( "api-ms-win-core-wow64-l1-1-1.dll""system" fn GetSystemWow64Directory2W ( lpbuffer : :: windows::core::PWSTR , usize : u32 , imagefilemachinetype : IMAGE_FILE_MACHINE ) -> u32 );
    GetSystemWow64Directory2W(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _), imagefilemachinetype)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWow64DirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemWow64DirectoryA ( lpbuffer : :: windows::core::PSTR , usize : u32 ) -> u32 );
    GetSystemWow64DirectoryA(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetSystemWow64DirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetSystemWow64DirectoryW ( lpbuffer : :: windows::core::PWSTR , usize : u32 ) -> u32 );
    GetSystemWow64DirectoryW(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetTickCount() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetTickCount ( ) -> u32 );
    GetTickCount()
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetTickCount64() -> u64 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetTickCount64 ( ) -> u64 );
    GetTickCount64()
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetVersion() -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetVersion ( ) -> u32 );
    GetVersion()
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExA(lpversioninformation: *mut OSVERSIONINFOA) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetVersionExA ( lpversioninformation : *mut OSVERSIONINFOA ) -> super::super::Foundation:: BOOL );
    GetVersionExA(lpversioninformation)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionExW(lpversioninformation: *mut OSVERSIONINFOW) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetVersionExW ( lpversioninformation : *mut OSVERSIONINFOW ) -> super::super::Foundation:: BOOL );
    GetVersionExW(lpversioninformation)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetWindowsDirectoryA(lpbuffer: ::core::option::Option<&mut [u8]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetWindowsDirectoryA ( lpbuffer : :: windows::core::PSTR , usize : u32 ) -> u32 );
    GetWindowsDirectoryA(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GetWindowsDirectoryW(lpbuffer: ::core::option::Option<&mut [u16]>) -> u32 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GetWindowsDirectoryW ( lpbuffer : :: windows::core::PWSTR , usize : u32 ) -> u32 );
    GetWindowsDirectoryW(::core::mem::transmute(lpbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpbuffer.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: *mut MEMORYSTATUS) {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GlobalMemoryStatus ( lpbuffer : *mut MEMORYSTATUS ) -> ( ) );
    GlobalMemoryStatus(lpbuffer)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalMemoryStatusEx(lpbuffer: *mut MEMORYSTATUSEX) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn GlobalMemoryStatusEx ( lpbuffer : *mut MEMORYSTATUSEX ) -> super::super::Foundation:: BOOL );
    GlobalMemoryStatusEx(lpbuffer)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsUserCetAvailableInEnvironment(usercetenvironment: USER_CET_ENVIRONMENT) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsUserCetAvailableInEnvironment ( usercetenvironment : USER_CET_ENVIRONMENT ) -> super::super::Foundation:: BOOL );
    IsUserCetAvailableInEnvironment(usercetenvironment)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64GuestMachineSupported(wowguestmachine: IMAGE_FILE_MACHINE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::imp::link ! ( "kernel32.dll""system" fn IsWow64GuestMachineSupported ( wowguestmachine : IMAGE_FILE_MACHINE , machineissupported : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
    IsWow64GuestMachineSupported(wowguestmachine, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: ::windows::core::PWSTR, deviceform: ::windows::core::PWSTR) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlConvertDeviceFamilyInfoToString ( puldevicefamilybuffersize : *mut u32 , puldeviceformbuffersize : *mut u32 , devicefamily : :: windows::core::PWSTR , deviceform : :: windows::core::PWSTR ) -> u32 );
    RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize, puldeviceformbuffersize, ::core::mem::transmute(devicefamily), ::core::mem::transmute(deviceform))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: ::core::option::Option<*mut u64>, puldevicefamily: ::core::option::Option<*mut DEVICEFAMILYINFOENUM>, puldeviceform: ::core::option::Option<*mut DEVICEFAMILYDEVICEFORM>) {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlGetDeviceFamilyInfoEnum ( pulluapinfo : *mut u64 , puldevicefamily : *mut DEVICEFAMILYINFOENUM , puldeviceform : *mut DEVICEFAMILYDEVICEFORM ) -> ( ) );
    RtlGetDeviceFamilyInfoEnum(::core::mem::transmute(pulluapinfo.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(puldevicefamily.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(puldeviceform.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> super::super::Foundation::BOOLEAN {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlGetProductInfo ( osmajorversion : u32 , osminorversion : u32 , spmajorversion : u32 , spminorversion : u32 , returnedproducttype : *mut u32 ) -> super::super::Foundation:: BOOLEAN );
    RtlGetProductInfo(osmajorversion, osminorversion, spmajorversion, spminorversion, returnedproducttype)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut ::core::ffi::c_void, size: u32) -> u32 {
    ::windows::imp::link ! ( "ntdllk.dll""system" fn RtlGetSystemGlobalData ( dataid : RTL_SYSTEM_GLOBAL_DATA_ID , buffer : *mut ::core::ffi::c_void , size : u32 ) -> u32 );
    RtlGetSystemGlobalData(dataid, buffer, size)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlOsDeploymentState ( flags : u32 ) -> OS_DEPLOYEMENT_STATE_VALUES );
    RtlOsDeploymentState(flags)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32 {
    ::windows::imp::link ! ( "ntdll.dll""system" fn RtlSwitchedVVI ( versioninfo : *const OSVERSIONINFOEXW , typemask : u32 , conditionmask : u64 ) -> u32 );
    RtlSwitchedVVI(versioninfo, typemask, conditionmask)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameA<P0>(lpcomputername: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetComputerNameA ( lpcomputername : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetComputerNameA(lpcomputername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameEx2W<P0>(nametype: COMPUTER_NAME_FORMAT, flags: u32, lpbuffer: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetComputerNameEx2W ( nametype : COMPUTER_NAME_FORMAT , flags : u32 , lpbuffer : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetComputerNameEx2W(nametype, flags, lpbuffer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExA<P0>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetComputerNameExA ( nametype : COMPUTER_NAME_FORMAT , lpbuffer : :: windows::core::PCSTR ) -> super::super::Foundation:: BOOL );
    SetComputerNameExA(nametype, lpbuffer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameExW<P0>(nametype: COMPUTER_NAME_FORMAT, lpbuffer: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetComputerNameExW ( nametype : COMPUTER_NAME_FORMAT , lpbuffer : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetComputerNameExW(nametype, lpbuffer.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetComputerNameW<P0>(lpcomputername: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetComputerNameW ( lpcomputername : :: windows::core::PCWSTR ) -> super::super::Foundation:: BOOL );
    SetComputerNameW(lpcomputername.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetLocalTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetLocalTime ( lpsystemtime : *const super::super::Foundation:: SYSTEMTIME ) -> super::super::Foundation:: BOOL );
    SetLocalTime(lpsystemtime)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTime(lpsystemtime: *const super::super::Foundation::SYSTEMTIME) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetSystemTime ( lpsystemtime : *const super::super::Foundation:: SYSTEMTIME ) -> super::super::Foundation:: BOOL );
    SetSystemTime(lpsystemtime)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustment<P0>(dwtimeadjustment: u32, btimeadjustmentdisabled: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "kernel32.dll""system" fn SetSystemTimeAdjustment ( dwtimeadjustment : u32 , btimeadjustmentdisabled : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetSystemTimeAdjustment(dwtimeadjustment, btimeadjustmentdisabled.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetSystemTimeAdjustmentPrecise<P0>(dwtimeadjustment: u64, btimeadjustmentdisabled: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "api-ms-win-core-sysinfo-l1-2-4.dll""system" fn SetSystemTimeAdjustmentPrecise ( dwtimeadjustment : u64 , btimeadjustmentdisabled : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    SetSystemTimeAdjustmentPrecise(dwtimeadjustment, btimeadjustmentdisabled.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: u64, typemask: VER_FLAGS, condition: u8) -> u64 {
    ::windows::imp::link ! ( "kernel32.dll""system" fn VerSetConditionMask ( conditionmask : u64 , typemask : VER_FLAGS , condition : u8 ) -> u64 );
    VerSetConditionMask(conditionmask, typemask, condition)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoA(lpversioninformation: *mut OSVERSIONINFOEXA, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn VerifyVersionInfoA ( lpversioninformation : *mut OSVERSIONINFOEXA , dwtypemask : VER_FLAGS , dwlconditionmask : u64 ) -> super::super::Foundation:: BOOL );
    VerifyVersionInfoA(lpversioninformation, dwtypemask, dwlconditionmask)
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyVersionInfoW(lpversioninformation: *mut OSVERSIONINFOEXW, dwtypemask: VER_FLAGS, dwlconditionmask: u64) -> super::super::Foundation::BOOL {
    ::windows::imp::link ! ( "kernel32.dll""system" fn VerifyVersionInfoW ( lpversioninformation : *mut OSVERSIONINFOEXW , dwtypemask : VER_FLAGS , dwlconditionmask : u64 ) -> super::super::Foundation:: BOOL );
    VerifyVersionInfoW(lpversioninformation, dwtypemask, dwlconditionmask)
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
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OSVERSION_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SCEX2_ALT_NETBIOS_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SPVERSION_MASK: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SUBVERSION_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8u32;
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
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPUTER_NAME_FORMAT(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(3i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalNetBIOS: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(4i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsHostname: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(5i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsDomain: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(6i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNamePhysicalDnsFullyQualified: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(7i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ComputerNameMax: COMPUTER_NAME_FORMAT = COMPUTER_NAME_FORMAT(8i32);
impl ::core::marker::Copy for COMPUTER_NAME_FORMAT {}
impl ::core::clone::Clone for COMPUTER_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPUTER_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMPUTER_NAME_FORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMPUTER_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPUTER_NAME_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CPU_SET_INFORMATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = CPU_SET_INFORMATION_TYPE(0i32);
impl ::core::marker::Copy for CPU_SET_INFORMATION_TYPE {}
impl ::core::clone::Clone for CPU_SET_INFORMATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CPU_SET_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CPU_SET_INFORMATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CPU_SET_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CPU_SET_INFORMATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEP_SYSTEM_POLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = DEP_SYSTEM_POLICY_TYPE(4i32);
impl ::core::marker::Copy for DEP_SYSTEM_POLICY_TYPE {}
impl ::core::clone::Clone for DEP_SYSTEM_POLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEP_SYSTEM_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEP_SYSTEM_POLICY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEP_SYSTEM_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEP_SYSTEM_POLICY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICEFAMILYDEVICEFORM(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(0u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PHONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(1u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(2u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(3u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(4u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(5u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(6u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(7u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_STICKPC: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(8u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PUCK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(9u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(10u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_HMD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(11u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(12u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(13u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_BANKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(14u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(15u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(16u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_GAMING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(17u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(18u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(19u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_KIOSK: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(20u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(21u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(22u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(23u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(24u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_PRINTING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(25u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(26u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_TOY: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(27u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_VENDING: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(28u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(29u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(30u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(31u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(32u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(33u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(34u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(35u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_00: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(36u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(37u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(38u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(39u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(40u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(41u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(42u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(43u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(44u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYDEVICEFORM_MAX: DEVICEFAMILYDEVICEFORM = DEVICEFAMILYDEVICEFORM(45u32);
impl ::core::marker::Copy for DEVICEFAMILYDEVICEFORM {}
impl ::core::clone::Clone for DEVICEFAMILYDEVICEFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICEFAMILYDEVICEFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEVICEFAMILYDEVICEFORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEVICEFAMILYDEVICEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYDEVICEFORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICEFAMILYINFOENUM(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_UAP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(0u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(1u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(2u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_DESKTOP: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(3u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_MOBILE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(4u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(5u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_TEAM: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(6u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_IOT: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(7u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(8u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_SERVER: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(9u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(10u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOXSRA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(11u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_XBOXERA: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(12u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(13u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_8828080: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(14u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_7067329: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(15u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(16u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const DEVICEFAMILYINFOENUM_MAX: DEVICEFAMILYINFOENUM = DEVICEFAMILYINFOENUM(17u32);
impl ::core::marker::Copy for DEVICEFAMILYINFOENUM {}
impl ::core::clone::Clone for DEVICEFAMILYINFOENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DEVICEFAMILYINFOENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DEVICEFAMILYINFOENUM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DEVICEFAMILYINFOENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYINFOENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FIRMWARE_TABLE_PROVIDER(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const ACPI: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1094930505u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FIRM: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1179210317u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RSMB: FIRMWARE_TABLE_PROVIDER = FIRMWARE_TABLE_PROVIDER(1381190978u32);
impl ::core::marker::Copy for FIRMWARE_TABLE_PROVIDER {}
impl ::core::clone::Clone for FIRMWARE_TABLE_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIRMWARE_TABLE_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FIRMWARE_TABLE_PROVIDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FIRMWARE_TABLE_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TABLE_PROVIDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FIRMWARE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = FIRMWARE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeBios: FIRMWARE_TYPE = FIRMWARE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeUefi: FIRMWARE_TYPE = FIRMWARE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const FirmwareTypeMax: FIRMWARE_TYPE = FIRMWARE_TYPE(3i32);
impl ::core::marker::Copy for FIRMWARE_TYPE {}
impl ::core::clone::Clone for FIRMWARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FIRMWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FIRMWARE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FIRMWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IMAGE_FILE_MACHINE(pub u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_AXP64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_I386: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(332u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_IA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(512u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_AMD64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(34404u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_UNKNOWN: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(0u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_TARGET_HOST: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_R3000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(354u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_R4000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(358u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_R10000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(360u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(361u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_ALPHA: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(388u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_SH3: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(418u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_SH3DSP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(419u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_SH3E: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(420u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_SH4: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(422u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_SH5: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(424u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_ARM: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(448u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_THUMB: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(450u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_ARMNT: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(452u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_AM33: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(467u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_POWERPC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(496u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_POWERPCFP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(497u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_MIPS16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(614u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_ALPHA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_MIPSFPU: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(870u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_MIPSFPU16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1126u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_TRICORE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1312u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_CEF: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3311u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_EBC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3772u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_M32R: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(36929u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_ARM64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(43620u16);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const IMAGE_FILE_MACHINE_CEE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(49390u16);
impl ::core::marker::Copy for IMAGE_FILE_MACHINE {}
impl ::core::clone::Clone for IMAGE_FILE_MACHINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IMAGE_FILE_MACHINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IMAGE_FILE_MACHINE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IMAGE_FILE_MACHINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_MACHINE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LOGICAL_PROCESSOR_RELATIONSHIP(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(3i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(4i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(5i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(6i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(7i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = LOGICAL_PROCESSOR_RELATIONSHIP(65535i32);
impl ::core::marker::Copy for LOGICAL_PROCESSOR_RELATIONSHIP {}
impl ::core::clone::Clone for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LOGICAL_PROCESSOR_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGICAL_PROCESSOR_RELATIONSHIP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OS_DEPLOYEMENT_STATE_VALUES(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = OS_DEPLOYEMENT_STATE_VALUES(2i32);
impl ::core::marker::Copy for OS_DEPLOYEMENT_STATE_VALUES {}
impl ::core::clone::Clone for OS_DEPLOYEMENT_STATE_VALUES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OS_DEPLOYEMENT_STATE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OS_DEPLOYEMENT_STATE_VALUES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OS_DEPLOYEMENT_STATE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_DEPLOYEMENT_STATE_VALUES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OS_PRODUCT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_BUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(6u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_BUSINESS_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(16u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CLUSTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(18u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CLUSTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(64u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(101u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_COUNTRYSPECIFIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(99u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(98u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_CORE_SINGLELANGUAGE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(100u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(80u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(145u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_A_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(146u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(8u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(12u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(39u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_DATACENTER_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(37u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_EDUCATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(121u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_EDUCATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(122u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(70u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(72u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(27u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(84u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(125u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(129u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(126u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(130u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(10u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(14u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(41u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_IA64: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(15u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ENTERPRISE_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(38u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(60u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(62u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(59u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(61u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(67u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_BASIC_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(5u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(68u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(26u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(34u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HOME_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(19u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_HYPERV: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(42u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_IOTUAP: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(123u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_IOTUAPCOMMERCIAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(131u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(30u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(32u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(31u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MOBILE_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(104u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MOBILE_ENTERPRISE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(133u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(77u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(76u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PRO_WORKSTATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(161u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PRO_WORKSTATION_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(162u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(48u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(69u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(49u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_PROFESSIONAL_WMC: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(103u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SB_SOLUTION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(50u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SB_SOLUTION_SERVER_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(54u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(51u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(55u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(24u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(35u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SERVER_FOUNDATION: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(33u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(9u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(25u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(63u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(56u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(79u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(7u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_CORE_: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(13u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_CORE_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(40u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_V: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(36u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(52u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(53u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(11u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(66u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STARTER_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(47u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(23u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(46u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_EXPRESS_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(20u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(43u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(96u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(21u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(44u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(95u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(22u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(45u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE_E: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(71u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_ULTIMATE_N: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(28u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_UNDEFINED: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_WEB_SERVER: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(17u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const PRODUCT_WEB_SERVER_CORE: OS_PRODUCT_TYPE = OS_PRODUCT_TYPE(29u32);
impl ::core::marker::Copy for OS_PRODUCT_TYPE {}
impl ::core::clone::Clone for OS_PRODUCT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OS_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OS_PRODUCT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OS_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_PRODUCT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROCESSOR_CACHE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheUnified: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheData: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const CacheTrace: PROCESSOR_CACHE_TYPE = PROCESSOR_CACHE_TYPE(3i32);
impl ::core::marker::Copy for PROCESSOR_CACHE_TYPE {}
impl ::core::clone::Clone for PROCESSOR_CACHE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROCESSOR_CACHE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROCESSOR_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_CACHE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTL_SYSTEM_GLOBAL_DATA_ID(pub i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(0i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(1i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(2i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(3i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(4i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(5i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(6i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(7i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(8i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(9i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(10i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(11i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(12i32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = RTL_SYSTEM_GLOBAL_DATA_ID(13i32);
impl ::core::marker::Copy for RTL_SYSTEM_GLOBAL_DATA_ID {}
impl ::core::clone::Clone for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for RTL_SYSTEM_GLOBAL_DATA_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_SYSTEM_GLOBAL_DATA_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct USER_CET_ENVIRONMENT(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_WIN32_PROCESS: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(0u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_SGX2_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(2u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_VBS_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(16u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const USER_CET_ENVIRONMENT_VBS_BASIC_ENCLAVE: USER_CET_ENVIRONMENT = USER_CET_ENVIRONMENT(17u32);
impl ::core::marker::Copy for USER_CET_ENVIRONMENT {}
impl ::core::clone::Clone for USER_CET_ENVIRONMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_CET_ENVIRONMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for USER_CET_ENVIRONMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for USER_CET_ENVIRONMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_CET_ENVIRONMENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_MINORVERSION: VER_FLAGS = VER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_MAJORVERSION: VER_FLAGS = VER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_BUILDNUMBER: VER_FLAGS = VER_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_PLATFORMID: VER_FLAGS = VER_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SERVICEPACKMINOR: VER_FLAGS = VER_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SERVICEPACKMAJOR: VER_FLAGS = VER_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_SUITENAME: VER_FLAGS = VER_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub const VER_PRODUCT_TYPE: VER_FLAGS = VER_FLAGS(128u32);
impl ::core::marker::Copy for VER_FLAGS {}
impl ::core::clone::Clone for VER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VER_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FLAGS").field(&self.0).finish()
    }
}
impl VER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
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
impl ::core::fmt::Debug for CACHE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACHE_DESCRIPTOR").field("Level", &self.Level).field("Associativity", &self.Associativity).field("LineSize", &self.LineSize).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::windows::core::TypeKind for CACHE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CACHE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Associativity == other.Associativity && self.LineSize == other.LineSize && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for CACHE_DESCRIPTOR {}
impl ::core::default::Default for CACHE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for CACHE_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for CACHE_RELATIONSHIP_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FIRMWARE_TABLE_ID(pub u32);
impl ::core::default::Default for FIRMWARE_TABLE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FIRMWARE_TABLE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FIRMWARE_TABLE_ID {}
impl ::core::fmt::Debug for FIRMWARE_TABLE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TABLE_ID").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for FIRMWARE_TABLE_ID {
    type TypeKind = ::windows::core::CopyType;
}
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
impl ::core::fmt::Debug for GROUP_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_AFFINITY").field("Mask", &self.Mask).field("Group", &self.Group).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_AFFINITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Group == other.Group && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for GROUP_AFFINITY {}
impl ::core::default::Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for GROUP_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_RELATIONSHIP").field("MaximumGroupCount", &self.MaximumGroupCount).field("ActiveGroupCount", &self.ActiveGroupCount).field("Reserved", &self.Reserved).field("GroupInfo", &self.GroupInfo).finish()
    }
}
impl ::windows::core::TypeKind for GROUP_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for GROUP_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumGroupCount == other.MaximumGroupCount && self.ActiveGroupCount == other.ActiveGroupCount && self.Reserved == other.Reserved && self.GroupInfo == other.GroupInfo
    }
}
impl ::core::cmp::Eq for GROUP_RELATIONSHIP {}
impl ::core::default::Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for MEMORYSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUS").field("dwLength", &self.dwLength).field("dwMemoryLoad", &self.dwMemoryLoad).field("dwTotalPhys", &self.dwTotalPhys).field("dwAvailPhys", &self.dwAvailPhys).field("dwTotalPageFile", &self.dwTotalPageFile).field("dwAvailPageFile", &self.dwAvailPageFile).field("dwTotalVirtual", &self.dwTotalVirtual).field("dwAvailVirtual", &self.dwAvailVirtual).finish()
    }
}
impl ::windows::core::TypeKind for MEMORYSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORYSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.dwTotalPhys == other.dwTotalPhys && self.dwAvailPhys == other.dwAvailPhys && self.dwTotalPageFile == other.dwTotalPageFile && self.dwAvailPageFile == other.dwAvailPageFile && self.dwTotalVirtual == other.dwTotalVirtual && self.dwAvailVirtual == other.dwAvailVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUS {}
impl ::core::default::Default for MEMORYSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for MEMORYSTATUSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUSEX").field("dwLength", &self.dwLength).field("dwMemoryLoad", &self.dwMemoryLoad).field("ullTotalPhys", &self.ullTotalPhys).field("ullAvailPhys", &self.ullAvailPhys).field("ullTotalPageFile", &self.ullTotalPageFile).field("ullAvailPageFile", &self.ullAvailPageFile).field("ullTotalVirtual", &self.ullTotalVirtual).field("ullAvailVirtual", &self.ullAvailVirtual).field("ullAvailExtendedVirtual", &self.ullAvailExtendedVirtual).finish()
    }
}
impl ::windows::core::TypeKind for MEMORYSTATUSEX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MEMORYSTATUSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.ullTotalPhys == other.ullTotalPhys && self.ullAvailPhys == other.ullAvailPhys && self.ullTotalPageFile == other.ullTotalPageFile && self.ullAvailPageFile == other.ullAvailPageFile && self.ullTotalVirtual == other.ullTotalVirtual && self.ullAvailVirtual == other.ullAvailVirtual && self.ullAvailExtendedVirtual == other.ullAvailExtendedVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUSEX {}
impl ::core::default::Default for MEMORYSTATUSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for NUMA_NODE_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for NUMA_NODE_RELATIONSHIP_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u8; 128],
}
impl ::core::marker::Copy for OSVERSIONINFOA {}
impl ::core::clone::Clone for OSVERSIONINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOA").field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("dwBuildNumber", &self.dwBuildNumber).field("dwPlatformId", &self.dwPlatformId).field("szCSDVersion", &self.szCSDVersion).finish()
    }
}
impl ::windows::core::TypeKind for OSVERSIONINFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OSVERSIONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOA {}
impl ::core::default::Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u8; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl ::core::marker::Copy for OSVERSIONINFOEXA {}
impl ::core::clone::Clone for OSVERSIONINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OSVERSIONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::windows::core::TypeKind for OSVERSIONINFOEXA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion && self.wServicePackMajor == other.wServicePackMajor && self.wServicePackMinor == other.wServicePackMinor && self.wSuiteMask == other.wSuiteMask && self.wProductType == other.wProductType && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXA {}
impl ::core::default::Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OSVERSIONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::windows::core::TypeKind for OSVERSIONINFOEXW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion && self.wServicePackMajor == other.wServicePackMajor && self.wServicePackMinor == other.wServicePackMinor && self.wSuiteMask == other.wSuiteMask && self.wProductType == other.wProductType && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXW {}
impl ::core::default::Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for OSVERSIONINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOW").field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("dwBuildNumber", &self.dwBuildNumber).field("dwPlatformId", &self.dwPlatformId).field("szCSDVersion", &self.szCSDVersion).finish()
    }
}
impl ::windows::core::TypeKind for OSVERSIONINFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OSVERSIONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOW {}
impl ::core::default::Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for PROCESSOR_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_GROUP_INFO").field("MaximumProcessorCount", &self.MaximumProcessorCount).field("ActiveProcessorCount", &self.ActiveProcessorCount).field("Reserved", &self.Reserved).field("ActiveProcessorMask", &self.ActiveProcessorMask).finish()
    }
}
impl ::windows::core::TypeKind for PROCESSOR_GROUP_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESSOR_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumProcessorCount == other.MaximumProcessorCount && self.ActiveProcessorCount == other.ActiveProcessorCount && self.Reserved == other.Reserved && self.ActiveProcessorMask == other.ActiveProcessorMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_GROUP_INFO {}
impl ::core::default::Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_RELATIONSHIP").field("Flags", &self.Flags).field("EfficiencyClass", &self.EfficiencyClass).field("Reserved", &self.Reserved).field("GroupCount", &self.GroupCount).field("GroupMask", &self.GroupMask).finish()
    }
}
impl ::windows::core::TypeKind for PROCESSOR_RELATIONSHIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESSOR_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EfficiencyClass == other.EfficiencyClass && self.Reserved == other.Reserved && self.GroupCount == other.GroupCount && self.GroupMask == other.GroupMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_RELATIONSHIP {}
impl ::core::default::Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_CPU_SET_INFORMATION_0_0_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::windows::core::TypeKind for SYSTEM_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::windows::core::TypeKind for SYSTEM_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::fmt::Debug for SYSTEM_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_INFO_0_0").field("wProcessorArchitecture", &self.wProcessorArchitecture).field("wReserved", &self.wReserved).finish()
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::windows::core::TypeKind for SYSTEM_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::PartialEq for SYSTEM_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wProcessorArchitecture == other.wProcessorArchitecture && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::cmp::Eq for SYSTEM_INFO_0_0 {}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0").field("NodeNumber", &self.NodeNumber).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeNumber == other.NodeNumber
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1").field("Flags", &self.Flags).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows::core::TypeKind for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POOL_ZEROING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POOL_ZEROING_INFORMATION").field("PoolZeroingSupportPresent", &self.PoolZeroingSupportPresent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SYSTEM_POOL_ZEROING_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_POOL_ZEROING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolZeroingSupportPresent == other.PoolZeroingSupportPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_POOL_ZEROING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION").field("CycleTime", &self.CycleTime).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CycleTime == other.CycleTime
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::core::default::Default for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::default::Default for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = ::core::option::Option<unsafe extern "system" fn(lpbuffer: ::windows::core::PSTR, usize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_System_SystemInformation\"`*"]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = ::core::option::Option<unsafe extern "system" fn(lpbuffer: ::windows::core::PWSTR, usize: u32) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
