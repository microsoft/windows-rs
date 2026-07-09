#[inline]
pub unsafe fn AbortSystemShutdownA<P0>(lpmachinename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AbortSystemShutdownA(lpmachinename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { AbortSystemShutdownA(lpmachinename.param().abi()) }
}
#[inline]
pub unsafe fn AbortSystemShutdownW<P0>(lpmachinename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AbortSystemShutdownW(lpmachinename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AbortSystemShutdownW(lpmachinename.param().abi()) }
}
#[inline]
pub unsafe fn CheckForHiberboot(phiberboot: *mut bool, bclearflag: bool) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn CheckForHiberboot(phiberboot : *mut bool, bclearflag : bool) -> u32);
    unsafe { CheckForHiberboot(phiberboot as _, bclearflag) }
}
#[inline]
pub unsafe fn InitiateShutdownA<P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: u32, dwreason: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateShutdownA(lpmachinename : windows_core::PCSTR, lpmessage : windows_core::PCSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
    unsafe { InitiateShutdownA(lpmachinename.param().abi(), lpmessage.param().abi(), dwgraceperiod, dwshutdownflags, dwreason) }
}
#[inline]
pub unsafe fn InitiateShutdownW<P0, P1>(lpmachinename: P0, lpmessage: P1, dwgraceperiod: u32, dwshutdownflags: u32, dwreason: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateShutdownW(lpmachinename : windows_core::PCWSTR, lpmessage : windows_core::PCWSTR, dwgraceperiod : u32, dwshutdownflags : u32, dwreason : u32) -> u32);
    unsafe { InitiateShutdownW(lpmachinename.param().abi(), lpmessage.param().abi(), dwgraceperiod, dwshutdownflags, dwreason) }
}
#[inline]
pub unsafe fn InitiateSystemShutdownA<P0, P1>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: bool, brebootaftershutdown: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateSystemShutdownA(lpmachinename : windows_core::PCSTR, lpmessage : windows_core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_core::BOOL, brebootaftershutdown : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InitiateSystemShutdownA(lpmachinename.param().abi(), lpmessage.param().abi(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into()) }
}
#[inline]
pub unsafe fn InitiateSystemShutdownExA<P0, P1>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: bool, brebootaftershutdown: bool, dwreason: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateSystemShutdownExA(lpmachinename : windows_core::PCSTR, lpmessage : windows_core::PCSTR, dwtimeout : u32, bforceappsclosed : windows_core::BOOL, brebootaftershutdown : windows_core::BOOL, dwreason : u32) -> windows_core::BOOL);
    unsafe { InitiateSystemShutdownExA(lpmachinename.param().abi(), lpmessage.param().abi(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into(), dwreason) }
}
#[inline]
pub unsafe fn InitiateSystemShutdownExW<P0, P1>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: bool, brebootaftershutdown: bool, dwreason: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateSystemShutdownExW(lpmachinename : windows_core::PCWSTR, lpmessage : windows_core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_core::BOOL, brebootaftershutdown : windows_core::BOOL, dwreason : u32) -> windows_core::BOOL);
    unsafe { InitiateSystemShutdownExW(lpmachinename.param().abi(), lpmessage.param().abi(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into(), dwreason) }
}
#[inline]
pub unsafe fn InitiateSystemShutdownW<P0, P1>(lpmachinename: P0, lpmessage: P1, dwtimeout: u32, bforceappsclosed: bool, brebootaftershutdown: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn InitiateSystemShutdownW(lpmachinename : windows_core::PCWSTR, lpmessage : windows_core::PCWSTR, dwtimeout : u32, bforceappsclosed : windows_core::BOOL, brebootaftershutdown : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { InitiateSystemShutdownW(lpmachinename.param().abi(), lpmessage.param().abi(), dwtimeout, bforceappsclosed.into(), brebootaftershutdown.into()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegCloseKey(hkey: super::minwindef::HKEY) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegCloseKey(hkey : super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegCloseKey(hkey) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegConnectRegistryA<P0>(lpmachinename: P0, hkey: super::minwindef::HKEY, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegConnectRegistryA(lpmachinename : windows_core::PCSTR, hkey : super::minwindef::HKEY, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegConnectRegistryA(lpmachinename.param().abi(), hkey, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegConnectRegistryExA<P0>(lpmachinename: P0, hkey: super::minwindef::HKEY, flags: u32, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegConnectRegistryExA(lpmachinename : windows_core::PCSTR, hkey : super::minwindef::HKEY, flags : u32, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegConnectRegistryExA(lpmachinename.param().abi(), hkey, flags, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegConnectRegistryExW<P0>(lpmachinename: P0, hkey: super::minwindef::HKEY, flags: u32, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegConnectRegistryExW(lpmachinename : windows_core::PCWSTR, hkey : super::minwindef::HKEY, flags : u32, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegConnectRegistryExW(lpmachinename.param().abi(), hkey, flags, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegConnectRegistryW<P0>(lpmachinename: P0, hkey: super::minwindef::HKEY, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegConnectRegistryW(lpmachinename : windows_core::PCWSTR, hkey : super::minwindef::HKEY, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegConnectRegistryW(lpmachinename.param().abi(), hkey, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegCopyTreeA<P1>(hkeysrc: super::minwindef::HKEY, lpsubkey: P1, hkeydest: super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCopyTreeA(hkeysrc : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, hkeydest : super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegCopyTreeA(hkeysrc, lpsubkey.param().abi(), hkeydest) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegCopyTreeW<P1>(hkeysrc: super::minwindef::HKEY, lpsubkey: P1, hkeydest: super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCopyTreeW(hkeysrc : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, hkeydest : super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegCopyTreeW(hkeysrc, lpsubkey.param().abi(), hkeydest) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegCreateKeyA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegCreateKeyA(hkey, lpsubkey.param().abi(), phkresult as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegCreateKeyExA<P1, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, reserved: Option<u32>, lpclass: P3, dwoptions: u32, samdesired: REGSAM, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: Option<*mut u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, reserved : u32, lpclass : windows_core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32) -> WIN32_ERROR);
    unsafe { RegCreateKeyExA(hkey, lpsubkey.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, lpclass.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegCreateKeyExW<P1, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, reserved: Option<u32>, lpclass: P3, dwoptions: u32, samdesired: REGSAM, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: Option<*mut u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32) -> WIN32_ERROR);
    unsafe { RegCreateKeyExW(hkey, lpsubkey.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, lpclass.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedA<P1, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, reserved: Option<u32>, lpclass: P3, dwoptions: u32, samdesired: REGSAM, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: Option<*mut u32>, htransaction: super::winnt::HANDLE, pextendedparemeter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, reserved : u32, lpclass : windows_core::PCSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegCreateKeyTransactedA(hkey, lpsubkey.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, lpclass.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _, htransaction, pextendedparemeter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegCreateKeyTransactedW<P1, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, reserved: Option<u32>, lpclass: P3, dwoptions: u32, samdesired: REGSAM, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: Option<*mut u32>, htransaction: super::winnt::HANDLE, pextendedparemeter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegCreateKeyTransactedW(hkey, lpsubkey.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, lpclass.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _, htransaction, pextendedparemeter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegCreateKeyW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegCreateKeyW(hkey, lpsubkey.param().abi(), phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteKeyA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegDeleteKeyA(hkey, lpsubkey.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegDeleteKeyExA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, samdesired: REGSAM, reserved: Option<u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, samdesired : REGSAM, reserved : u32) -> WIN32_ERROR);
    unsafe { RegDeleteKeyExA(hkey, lpsubkey.param().abi(), samdesired, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegDeleteKeyExW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, samdesired: REGSAM, reserved: Option<u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, samdesired : REGSAM, reserved : u32) -> WIN32_ERROR);
    unsafe { RegDeleteKeyExW(hkey, lpsubkey.param().abi(), samdesired, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegDeleteKeyTransactedA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, samdesired: REGSAM, reserved: Option<u32>, htransaction: super::winnt::HANDLE, pextendedparameter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, samdesired : REGSAM, reserved : u32, htransaction : super::winnt::HANDLE, pextendedparameter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegDeleteKeyTransactedA(hkey, lpsubkey.param().abi(), samdesired, reserved.unwrap_or(core::mem::zeroed()) as _, htransaction, pextendedparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegDeleteKeyTransactedW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, samdesired: REGSAM, reserved: Option<u32>, htransaction: super::winnt::HANDLE, pextendedparameter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, samdesired : REGSAM, reserved : u32, htransaction : super::winnt::HANDLE, pextendedparameter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegDeleteKeyTransactedW(hkey, lpsubkey.param().abi(), samdesired, reserved.unwrap_or(core::mem::zeroed()) as _, htransaction, pextendedparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteKeyValueA<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvaluename: P2) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpvaluename : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegDeleteKeyValueA(hkey, lpsubkey.param().abi(), lpvaluename.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteKeyValueW<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvaluename: P2) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegDeleteKeyValueW(hkey, lpsubkey.param().abi(), lpvaluename.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteKeyW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegDeleteKeyW(hkey, lpsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteTreeA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteTreeA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegDeleteTreeA(hkey, lpsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteTreeW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegDeleteTreeW(hkey, lpsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteValueA<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteValueA(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegDeleteValueA(hkey, lpvaluename.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDeleteValueW<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegDeleteValueW(hkey, lpvaluename.param().abi()) }
}
#[inline]
pub unsafe fn RegDisablePredefinedCache() -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegDisablePredefinedCache() -> WIN32_ERROR);
    unsafe { RegDisablePredefinedCache() }
}
#[inline]
pub unsafe fn RegDisablePredefinedCacheEx() -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegDisablePredefinedCacheEx() -> WIN32_ERROR);
    unsafe { RegDisablePredefinedCacheEx() }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegDisableReflectionKey(hbase: super::minwindef::HKEY) -> i32 {
    windows_core::link!("advapi32.dll" "system" fn RegDisableReflectionKey(hbase : super::minwindef::HKEY) -> i32);
    unsafe { RegDisableReflectionKey(hbase) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnableReflectionKey(hbase: super::minwindef::HKEY) -> i32 {
    windows_core::link!("advapi32.dll" "system" fn RegEnableReflectionKey(hbase : super::minwindef::HKEY) -> i32);
    unsafe { RegEnableReflectionKey(hbase) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumKeyA(hkey: super::minwindef::HKEY, dwindex: u32, lpname: Option<&mut [u8]>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumKeyA(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_core::PSTR, cchname : u32) -> WIN32_ERROR);
    unsafe { RegEnumKeyA(hkey, dwindex, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumKeyExA(hkey: super::minwindef::HKEY, dwindex: u32, lpname: Option<windows_core::PSTR>, lpcchname: *mut u32, lpreserved: Option<*const u32>, lpclass: Option<windows_core::PSTR>, lpcchclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::minwindef::FILETIME>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumKeyExA(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_core::PSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_core::PSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
    unsafe { RegEnumKeyExA(hkey, dwindex, lpname.unwrap_or(core::mem::zeroed()) as _, lpcchname as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lpclass.unwrap_or(core::mem::zeroed()) as _, lpcchclass.unwrap_or(core::mem::zeroed()) as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumKeyExW(hkey: super::minwindef::HKEY, dwindex: u32, lpname: Option<windows_core::PWSTR>, lpcchname: *mut u32, lpreserved: Option<*const u32>, lpclass: Option<windows_core::PWSTR>, lpcchclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::minwindef::FILETIME>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumKeyExW(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_core::PWSTR, lpcchname : *mut u32, lpreserved : *const u32, lpclass : windows_core::PWSTR, lpcchclass : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
    unsafe { RegEnumKeyExW(hkey, dwindex, lpname.unwrap_or(core::mem::zeroed()) as _, lpcchname as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lpclass.unwrap_or(core::mem::zeroed()) as _, lpcchclass.unwrap_or(core::mem::zeroed()) as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumKeyW(hkey: super::minwindef::HKEY, dwindex: u32, lpname: Option<&mut [u16]>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumKeyW(hkey : super::minwindef::HKEY, dwindex : u32, lpname : windows_core::PWSTR, cchname : u32) -> WIN32_ERROR);
    unsafe { RegEnumKeyW(hkey, dwindex, core::mem::transmute(lpname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumValueA(hkey: super::minwindef::HKEY, dwindex: u32, lpvaluename: Option<windows_core::PSTR>, lpcchvaluename: *mut u32, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumValueA(hkey : super::minwindef::HKEY, dwindex : u32, lpvaluename : windows_core::PSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegEnumValueA(hkey, dwindex, lpvaluename.unwrap_or(core::mem::zeroed()) as _, lpcchvaluename as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegEnumValueW(hkey: super::minwindef::HKEY, dwindex: u32, lpvaluename: Option<windows_core::PWSTR>, lpcchvaluename: *mut u32, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegEnumValueW(hkey : super::minwindef::HKEY, dwindex : u32, lpvaluename : windows_core::PWSTR, lpcchvaluename : *mut u32, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegEnumValueW(hkey, dwindex, lpvaluename.unwrap_or(core::mem::zeroed()) as _, lpcchvaluename as _, lpreserved.unwrap_or(core::mem::zeroed()) as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegFlushKey(hkey: super::minwindef::HKEY) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegFlushKey(hkey : super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegFlushKey(hkey) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegGetKeySecurity(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, lpcbsecuritydescriptor: *mut u32) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegGetKeySecurity(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> WIN32_ERROR);
    unsafe { RegGetKeySecurity(hkey, securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, lpcbsecuritydescriptor as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegGetValueA<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvalue: P2, dwflags: u32, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegGetValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpvalue : windows_core::PCSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegGetValueA(hkey, lpsubkey.param().abi(), lpvalue.param().abi(), dwflags, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegGetValueW<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvalue: P2, dwflags: u32, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegGetValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegGetValueW(hkey, lpsubkey.param().abi(), lpvalue.param().abi(), dwflags, pdwtype.unwrap_or(core::mem::zeroed()) as _, pvdata.unwrap_or(core::mem::zeroed()) as _, pcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegLoadAppKeyA<P0>(lpfile: P0, phkresult: *mut super::minwindef::HKEY, samdesired: REGSAM, dwoptions: u32, reserved: Option<u32>) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadAppKeyA(lpfile : windows_core::PCSTR, phkresult : *mut super::minwindef::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> WIN32_ERROR);
    unsafe { RegLoadAppKeyA(lpfile.param().abi(), phkresult as _, samdesired, dwoptions, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegLoadAppKeyW<P0>(lpfile: P0, phkresult: *mut super::minwindef::HKEY, samdesired: REGSAM, dwoptions: u32, reserved: Option<u32>) -> WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadAppKeyW(lpfile : windows_core::PCWSTR, phkresult : *mut super::minwindef::HKEY, samdesired : REGSAM, dwoptions : u32, reserved : u32) -> WIN32_ERROR);
    unsafe { RegLoadAppKeyW(lpfile.param().abi(), phkresult as _, samdesired, dwoptions, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegLoadKeyA<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpfile: P2) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpfile : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegLoadKeyA(hkey, lpsubkey.param().abi(), lpfile.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegLoadKeyW<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpfile: P2) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpfile : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegLoadKeyW(hkey, lpsubkey.param().abi(), lpfile.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegLoadMUIStringA<P1, P6>(hkey: super::minwindef::HKEY, pszvalue: P1, pszoutbuf: Option<&mut [u8]>, pcbdata: Option<*mut u32>, flags: u32, pszdirectory: P6) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadMUIStringA(hkey : super::minwindef::HKEY, pszvalue : windows_core::PCSTR, pszoutbuf : windows_core::PSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegLoadMUIStringA(hkey, pszvalue.param().abi(), core::mem::transmute(pszoutbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszoutbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbdata.unwrap_or(core::mem::zeroed()) as _, flags, pszdirectory.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegLoadMUIStringW<P1, P6>(hkey: super::minwindef::HKEY, pszvalue: P1, pszoutbuf: Option<windows_core::PWSTR>, cboutbuf: u32, pcbdata: Option<*mut u32>, flags: u32, pszdirectory: P6) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegLoadMUIStringW(hkey : super::minwindef::HKEY, pszvalue : windows_core::PCWSTR, pszoutbuf : windows_core::PWSTR, cboutbuf : u32, pcbdata : *mut u32, flags : u32, pszdirectory : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegLoadMUIStringW(hkey, pszvalue.param().abi(), pszoutbuf.unwrap_or(core::mem::zeroed()) as _, cboutbuf, pcbdata.unwrap_or(core::mem::zeroed()) as _, flags, pszdirectory.param().abi()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegNotifyChangeKeyValue(hkey: super::minwindef::HKEY, bwatchsubtree: bool, dwnotifyfilter: u32, hevent: Option<super::winnt::HANDLE>, fasynchronous: bool) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegNotifyChangeKeyValue(hkey : super::minwindef::HKEY, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : u32, hevent : super::winnt::HANDLE, fasynchronous : windows_core::BOOL) -> WIN32_ERROR);
    unsafe { RegNotifyChangeKeyValue(hkey, bwatchsubtree.into(), dwnotifyfilter, hevent.unwrap_or(core::mem::zeroed()) as _, fasynchronous.into()) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenCurrentUser(samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegOpenCurrentUser(samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenCurrentUser(samdesired, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegOpenKeyA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenKeyA(hkey, lpsubkey.param().abi(), phkresult as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenKeyExA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, uloptions: Option<u32>, samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyExA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenKeyExA(hkey, lpsubkey.param().abi(), uloptions.unwrap_or(core::mem::zeroed()) as _, samdesired, phkresult as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenKeyExW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, uloptions: Option<u32>, samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenKeyExW(hkey, lpsubkey.param().abi(), uloptions.unwrap_or(core::mem::zeroed()) as _, samdesired, phkresult as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenKeyTransactedA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, uloptions: Option<u32>, samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY, htransaction: super::winnt::HANDLE, pextendedparemeter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyTransactedA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegOpenKeyTransactedA(hkey, lpsubkey.param().abi(), uloptions.unwrap_or(core::mem::zeroed()) as _, samdesired, phkresult as _, htransaction, pextendedparemeter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenKeyTransactedW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, uloptions: Option<u32>, samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY, htransaction: super::winnt::HANDLE, pextendedparemeter: Option<*const core::ffi::c_void>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyTransactedW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY, htransaction : super::winnt::HANDLE, pextendedparemeter : *const core::ffi::c_void) -> WIN32_ERROR);
    unsafe { RegOpenKeyTransactedW(hkey, lpsubkey.param().abi(), uloptions.unwrap_or(core::mem::zeroed()) as _, samdesired, phkresult as _, htransaction, pextendedparemeter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegOpenKeyW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenKeyW(hkey, lpsubkey.param().abi(), phkresult as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegOpenUserClassesRoot(htoken: super::winnt::HANDLE, dwoptions: Option<u32>, samdesired: REGSAM, phkresult: *mut super::minwindef::HKEY) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegOpenUserClassesRoot(htoken : super::winnt::HANDLE, dwoptions : u32, samdesired : REGSAM, phkresult : *mut super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOpenUserClassesRoot(htoken, dwoptions.unwrap_or(core::mem::zeroed()) as _, samdesired, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegOverridePredefKey(hkey: super::minwindef::HKEY, hnewhkey: Option<super::minwindef::HKEY>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegOverridePredefKey(hkey : super::minwindef::HKEY, hnewhkey : super::minwindef::HKEY) -> WIN32_ERROR);
    unsafe { RegOverridePredefKey(hkey, hnewhkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryInfoKeyA(hkey: super::minwindef::HKEY, lpclass: Option<windows_core::PSTR>, lpcchclass: Option<*mut u32>, lpreserved: Option<*const u32>, lpcsubkeys: Option<*mut u32>, lpcbmaxsubkeylen: Option<*mut u32>, lpcbmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcbmaxvaluenamelen: Option<*mut u32>, lpcbmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::minwindef::FILETIME>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegQueryInfoKeyA(hkey : super::minwindef::HKEY, lpclass : windows_core::PSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
    unsafe {
        RegQueryInfoKeyA(
            hkey,
            lpclass.unwrap_or(core::mem::zeroed()) as _,
            lpcchclass.unwrap_or(core::mem::zeroed()) as _,
            lpreserved.unwrap_or(core::mem::zeroed()) as _,
            lpcsubkeys.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxclasslen.unwrap_or(core::mem::zeroed()) as _,
            lpcvalues.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxvaluelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _,
            lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryInfoKeyW(hkey: super::minwindef::HKEY, lpclass: Option<windows_core::PWSTR>, lpcchclass: Option<*mut u32>, lpreserved: Option<*const u32>, lpcsubkeys: Option<*mut u32>, lpcbmaxsubkeylen: Option<*mut u32>, lpcbmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcbmaxvaluenamelen: Option<*mut u32>, lpcbmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::minwindef::FILETIME>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegQueryInfoKeyW(hkey : super::minwindef::HKEY, lpclass : windows_core::PWSTR, lpcchclass : *mut u32, lpreserved : *const u32, lpcsubkeys : *mut u32, lpcbmaxsubkeylen : *mut u32, lpcbmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcbmaxvaluenamelen : *mut u32, lpcbmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> WIN32_ERROR);
    unsafe {
        RegQueryInfoKeyW(
            hkey,
            lpclass.unwrap_or(core::mem::zeroed()) as _,
            lpcchclass.unwrap_or(core::mem::zeroed()) as _,
            lpreserved.unwrap_or(core::mem::zeroed()) as _,
            lpcsubkeys.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxsubkeylen.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxclasslen.unwrap_or(core::mem::zeroed()) as _,
            lpcvalues.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxvaluenamelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbmaxvaluelen.unwrap_or(core::mem::zeroed()) as _,
            lpcbsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _,
            lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryMultipleValuesA(hkey: super::minwindef::HKEY, val_list: &mut [VALENTA], lpvaluebuf: Option<windows_core::PSTR>, ldwtotsize: Option<*mut u32>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegQueryMultipleValuesA(hkey : super::minwindef::HKEY, val_list : *mut VALENTA, num_vals : u32, lpvaluebuf : windows_core::PSTR, ldwtotsize : *mut u32) -> WIN32_ERROR);
    unsafe { RegQueryMultipleValuesA(hkey, core::mem::transmute(val_list.as_ptr()), val_list.len().try_into().unwrap(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, ldwtotsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryMultipleValuesW(hkey: super::minwindef::HKEY, val_list: &mut [VALENTW], lpvaluebuf: Option<windows_core::PWSTR>, ldwtotsize: Option<*mut u32>) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegQueryMultipleValuesW(hkey : super::minwindef::HKEY, val_list : *mut VALENTW, num_vals : u32, lpvaluebuf : windows_core::PWSTR, ldwtotsize : *mut u32) -> WIN32_ERROR);
    unsafe { RegQueryMultipleValuesW(hkey, core::mem::transmute(val_list.as_ptr()), val_list.len().try_into().unwrap(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, ldwtotsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryReflectionKey(hbase: super::minwindef::HKEY, bisreflectiondisabled: *mut windows_core::BOOL) -> i32 {
    windows_core::link!("advapi32.dll" "system" fn RegQueryReflectionKey(hbase : super::minwindef::HKEY, bisreflectiondisabled : *mut windows_core::BOOL) -> i32);
    unsafe { RegQueryReflectionKey(hbase, bisreflectiondisabled as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryValueA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpdata: Option<windows_core::PSTR>, lpcbdata: Option<*mut i32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegQueryValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpdata : windows_core::PSTR, lpcbdata : *mut i32) -> WIN32_ERROR);
    unsafe { RegQueryValueA(hkey, lpsubkey.param().abi(), lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryValueExA<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: *mut u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegQueryValueExA(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegQueryValueExA(hkey, lpvaluename.param().abi(), lpreserved.unwrap_or(core::mem::zeroed()) as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryValueExW<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1, lpreserved: Option<*const u32>, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: *mut u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegQueryValueExW(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCWSTR, lpreserved : *const u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> WIN32_ERROR);
    unsafe { RegQueryValueExW(hkey, lpvaluename.param().abi(), lpreserved.unwrap_or(core::mem::zeroed()) as _, lptype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegQueryValueW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpdata: Option<windows_core::PWSTR>, lpcbdata: Option<*mut i32>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegQueryValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpdata : windows_core::PWSTR, lpcbdata : *mut i32) -> WIN32_ERROR);
    unsafe { RegQueryValueW(hkey, lpsubkey.param().abi(), lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegRenameKey<P1, P2>(hkey: super::minwindef::HKEY, lpsubkeyname: P1, lpnewkeyname: P2) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegRenameKey(hkey : super::minwindef::HKEY, lpsubkeyname : windows_core::PCWSTR, lpnewkeyname : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegRenameKey(hkey, lpsubkeyname.param().abi(), lpnewkeyname.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegReplaceKeyA<P1, P2, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegReplaceKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpnewfile : windows_core::PCSTR, lpoldfile : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegReplaceKeyA(hkey, lpsubkey.param().abi(), lpnewfile.param().abi(), lpoldfile.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegReplaceKeyW<P1, P2, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpnewfile: P2, lpoldfile: P3) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegReplaceKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpnewfile : windows_core::PCWSTR, lpoldfile : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegReplaceKeyW(hkey, lpsubkey.param().abi(), lpnewfile.param().abi(), lpoldfile.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegRestoreKeyA<P1>(hkey: super::minwindef::HKEY, lpfile: P1, dwflags: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegRestoreKeyA(hkey : super::minwindef::HKEY, lpfile : windows_core::PCSTR, dwflags : u32) -> WIN32_ERROR);
    unsafe { RegRestoreKeyA(hkey, lpfile.param().abi(), dwflags) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegRestoreKeyW<P1>(hkey: super::minwindef::HKEY, lpfile: P1, dwflags: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegRestoreKeyW(hkey : super::minwindef::HKEY, lpfile : windows_core::PCWSTR, dwflags : u32) -> WIN32_ERROR);
    unsafe { RegRestoreKeyW(hkey, lpfile.param().abi(), dwflags) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn RegSaveKeyA<P1>(hkey: super::minwindef::HKEY, lpfile: P1, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSaveKeyA(hkey : super::minwindef::HKEY, lpfile : windows_core::PCSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> WIN32_ERROR);
    unsafe { RegSaveKeyA(hkey, lpfile.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn RegSaveKeyExA<P1>(hkey: super::minwindef::HKEY, lpfile: P1, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, flags: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSaveKeyExA(hkey : super::minwindef::HKEY, lpfile : windows_core::PCSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flags : u32) -> WIN32_ERROR);
    unsafe { RegSaveKeyExA(hkey, lpfile.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn RegSaveKeyExW<P1>(hkey: super::minwindef::HKEY, lpfile: P1, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, flags: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSaveKeyExW(hkey : super::minwindef::HKEY, lpfile : windows_core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flags : u32) -> WIN32_ERROR);
    unsafe { RegSaveKeyExW(hkey, lpfile.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn RegSaveKeyW<P1>(hkey: super::minwindef::HKEY, lpfile: P1, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSaveKeyW(hkey : super::minwindef::HKEY, lpfile : windows_core::PCWSTR, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> WIN32_ERROR);
    unsafe { RegSaveKeyW(hkey, lpfile.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn RegSetKeySecurity(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> WIN32_ERROR {
    windows_core::link!("advapi32.dll" "system" fn RegSetKeySecurity(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> WIN32_ERROR);
    unsafe { RegSetKeySecurity(hkey, securityinformation, psecuritydescriptor) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetKeyValueA<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetKeyValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, lpvaluename : windows_core::PCSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetKeyValueA(hkey, lpsubkey.param().abi(), lpvaluename.param().abi(), dwtype, lpdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetKeyValueW<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpvaluename: P2, dwtype: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetKeyValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetKeyValueW(hkey, lpsubkey.param().abi(), lpvaluename.param().abi(), dwtype, lpdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetValueA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1, dwtype: u32, lpdata: Option<&[u8]>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetValueA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR, dwtype : u32, lpdata : windows_core::PCSTR, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetValueA(hkey, lpsubkey.param().abi(), dwtype, core::mem::transmute(lpdata.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetValueExA<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1, reserved: Option<u32>, dwtype: u32, lpdata: Option<&[u8]>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetValueExA(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetValueExA(hkey, lpvaluename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, dwtype, core::mem::transmute(lpdata.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetValueExW<P1>(hkey: super::minwindef::HKEY, lpvaluename: P1, reserved: Option<u32>, dwtype: u32, lpdata: Option<&[u8]>) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : super::minwindef::HKEY, lpvaluename : windows_core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetValueExW(hkey, lpvaluename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, dwtype, core::mem::transmute(lpdata.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegSetValueW<P1, P3>(hkey: super::minwindef::HKEY, lpsubkey: P1, dwtype: u32, lpdata: P3, cbdata: u32) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetValueW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, dwtype : u32, lpdata : windows_core::PCWSTR, cbdata : u32) -> WIN32_ERROR);
    unsafe { RegSetValueW(hkey, lpsubkey.param().abi(), dwtype, lpdata.param().abi(), cbdata) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegUnLoadKeyA<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegUnLoadKeyA(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCSTR) -> WIN32_ERROR);
    unsafe { RegUnLoadKeyA(hkey, lpsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RegUnLoadKeyW<P1>(hkey: super::minwindef::HKEY, lpsubkey: P1) -> WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegUnLoadKeyW(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR) -> WIN32_ERROR);
    unsafe { RegUnLoadKeyW(hkey, lpsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CLASSES_ROOT: super::minwindef::HKEY = super::minwindef::HKEY(-2147483648 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_CONFIG: super::minwindef::HKEY = super::minwindef::HKEY(-2147483643 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_USER: super::minwindef::HKEY = super::minwindef::HKEY(-2147483647 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_CURRENT_USER_LOCAL_SETTINGS: super::minwindef::HKEY = super::minwindef::HKEY(-2147483641 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_DYN_DATA: super::minwindef::HKEY = super::minwindef::HKEY(-2147483642 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_LOCAL_MACHINE: super::minwindef::HKEY = super::minwindef::HKEY(-2147483646 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_DATA: super::minwindef::HKEY = super::minwindef::HKEY(-2147483644 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_NLSTEXT: super::minwindef::HKEY = super::minwindef::HKEY(-2147483552 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_PERFORMANCE_TEXT: super::minwindef::HKEY = super::minwindef::HKEY(-2147483568 as _);
#[cfg(feature = "Win32_minwindef")]
pub const HKEY_USERS: super::minwindef::HKEY = super::minwindef::HKEY(-2147483645 as _);
pub const MAX_SHUTDOWN_TIMEOUT: u32 = 315360000;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROVIDER(pub *mut REG_PROVIDER);
impl PPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PPVALUE(pub PPVALUEA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPVALUEA(pub *mut PVALUEA);
impl PPVALUEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPVALUEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPVALUEW(pub *mut PVALUEW);
impl PPVALUEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPVALUEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PQUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const PROVIDER_KEEPS_VALUE_LENGTH: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVALCONTEXT(pub *mut val_context);
impl PVALCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PVALCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PVALENT(pub PVALENTA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVALENTA(pub *mut VALENTA);
impl PVALENTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PVALENTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVALENTW(pub *mut VALENTW);
impl PVALENTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PVALENTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PVALUE = PVALUEA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PVALUEA {
    pub pv_valuename: windows_core::PSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl Default for PVALUEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PVALUEW {
    pub pv_valuename: windows_core::PWSTR,
    pub pv_valuelen: i32,
    pub pv_value_context: *mut core::ffi::c_void,
    pub pv_type: u32,
}
impl Default for PVALUEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type QUERYHANDLER = Option<unsafe extern "C" fn(keycontext: *mut core::ffi::c_void, val_list: *mut val_context, num_vals: u32, outputbuffer: *mut core::ffi::c_void, total_outlen: *mut u32, input_blen: u32) -> u32>;
pub const REASON_HWINSTALL: u32 = 65538;
pub const REASON_LEGACY_API: i32 = -2147024896;
pub const REASON_OTHER: u32 = 0;
pub const REASON_PLANNED_FLAG: i32 = -2147483648;
pub const REASON_SERVICEHANG: u32 = 196613;
pub const REASON_SWHWRECONF: u32 = 196612;
pub const REASON_SWINSTALL: u32 = 196610;
pub const REASON_UNKNOWN: u32 = 255;
pub const REASON_UNSTABLE: u32 = 327686;
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct REGSAM(pub super::winnt::ACCESS_MASK);
pub const REG_ALLOW_TRANSPORT_FALLBACK: u32 = 2;
pub const REG_ALLOW_UNSECURE_CONNECTION: u32 = 4;
pub const REG_MUI_STRING_TRUNCATE: u32 = 1;
pub const REG_PROCESS_APPKEY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct REG_PROVIDER {
    pub pi_R0_1val: PQUERYHANDLER,
    pub pi_R0_allvals: PQUERYHANDLER,
    pub pi_R3_1val: PQUERYHANDLER,
    pub pi_R3_allvals: PQUERYHANDLER,
    pub pi_flags: u32,
    pub pi_key_context: *mut core::ffi::c_void,
}
impl Default for REG_PROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REG_SECURE_CONNECTION: u32 = 1;
pub const REG_USE_CURRENT_SECURITY_CONTEXT: u32 = 2;
pub const RRF_NOEXPAND: u32 = 268435456;
pub const RRF_RT_ANY: u32 = 65535;
pub const RRF_RT_DWORD: u32 = 24;
pub const RRF_RT_QWORD: u32 = 72;
pub const RRF_RT_REG_BINARY: u32 = 8;
pub const RRF_RT_REG_DWORD: u32 = 16;
pub const RRF_RT_REG_EXPAND_SZ: u32 = 4;
pub const RRF_RT_REG_MULTI_SZ: u32 = 32;
pub const RRF_RT_REG_NONE: u32 = 1;
pub const RRF_RT_REG_QWORD: u32 = 64;
pub const RRF_RT_REG_SZ: u32 = 2;
pub const RRF_SUBKEY_WOW6432KEY: u32 = 131072;
pub const RRF_SUBKEY_WOW6464KEY: u32 = 65536;
pub const RRF_WOW64_MASK: u32 = 196608;
pub const RRF_ZEROONFAILURE: u32 = 536870912;
pub const SHUTDOWN_ARSO: u32 = 8192;
pub const SHUTDOWN_CHECK_SAFE_FOR_SERVER: u32 = 16384;
pub const SHUTDOWN_FORCE_OTHERS: u32 = 1;
pub const SHUTDOWN_FORCE_SELF: u32 = 2;
pub const SHUTDOWN_GRACE_OVERRIDE: u32 = 32;
pub const SHUTDOWN_HYBRID: u32 = 512;
pub const SHUTDOWN_INSTALL_UPDATES: u32 = 64;
pub const SHUTDOWN_MOBILE_UI: u32 = 4096;
pub const SHUTDOWN_NOREBOOT: u32 = 16;
pub const SHUTDOWN_POWEROFF: u32 = 8;
pub const SHUTDOWN_RESTART: u32 = 4;
pub const SHUTDOWN_RESTARTAPPS: u32 = 128;
pub const SHUTDOWN_RESTART_BOOTOPTIONS: u32 = 1024;
pub const SHUTDOWN_SKIP_SVC_PRESHUTDOWN: u32 = 256;
pub const SHUTDOWN_SOFT_REBOOT: u32 = 2048;
pub const SHUTDOWN_SYSTEM_INITIATED: u32 = 65536;
pub const SHUTDOWN_UPDATE_POWEROFF: u32 = 131072;
pub const SHUTDOWN_VAIL_CONTAINER: u32 = 32768;
pub type VALENT = VALENTA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VALENTA {
    pub ve_valuename: windows_core::PSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VALENTW {
    pub ve_valuename: windows_core::PWSTR,
    pub ve_valuelen: u32,
    pub ve_valueptr: usize,
    pub ve_type: u32,
}
pub const WIN31_CLASS: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WIN32_ERROR(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct val_context {
    pub valuelen: i32,
    pub value_context: *mut core::ffi::c_void,
    pub val_buff_ptr: *mut core::ffi::c_void,
}
impl Default for val_context {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
