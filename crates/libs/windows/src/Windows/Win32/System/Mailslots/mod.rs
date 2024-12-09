#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotA<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_core::PCSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMailslotA(lpname.param().abi(), core::mem::transmute(nmaxmessagesize), core::mem::transmute(lreadtimeout), core::mem::transmute(lpsecurityattributes.unwrap_or(core::mem::zeroed())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMailslotW<P0>(lpname: P0, nmaxmessagesize: u32, lreadtimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_core::PCWSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMailslotW(lpname.param().abi(), core::mem::transmute(nmaxmessagesize), core::mem::transmute(lreadtimeout), core::mem::transmute(lpsecurityattributes.unwrap_or(core::mem::zeroed())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lpmaxmessagesize: Option<*mut u32>, lpnextsize: Option<*mut u32>, lpmessagecount: Option<*mut u32>, lpreadtimeout: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lpmaxmessagesize : *mut u32, lpnextsize : *mut u32, lpmessagecount : *mut u32, lpreadtimeout : *mut u32) -> super::super::Foundation:: BOOL);
    GetMailslotInfo(core::mem::transmute(hmailslot), core::mem::transmute(lpmaxmessagesize.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpnextsize.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpmessagecount.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpreadtimeout.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn SetMailslotInfo(hmailslot: super::super::Foundation::HANDLE, lreadtimeout: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::super::Foundation:: HANDLE, lreadtimeout : u32) -> super::super::Foundation:: BOOL);
    SetMailslotInfo(core::mem::transmute(hmailslot), core::mem::transmute(lreadtimeout)).ok()
}
