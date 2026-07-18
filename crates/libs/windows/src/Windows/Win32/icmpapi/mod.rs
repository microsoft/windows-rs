#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Icmp6CreateFile() -> super::HANDLE {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::HANDLE);
    unsafe { Icmp6CreateFile() }
}
#[inline]
pub unsafe fn Icmp6ParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { Icmp6ParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "in6addr", feature = "ipexport", feature = "minwindef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn Icmp6SendEcho2(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<*const core::ffi::c_void>, sourceaddress: *const super::SOCKADDR_IN6_LH, destinationaddress: *const super::SOCKADDR_IN6_LH, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : *const super::SOCKADDR_IN6_LH, destinationaddress : *const super::SOCKADDR_IN6_LH, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { Icmp6SendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IcmpCloseHandle(icmphandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { IcmpCloseHandle(icmphandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IcmpCreateFile() -> super::HANDLE {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::HANDLE);
    unsafe { IcmpCreateFile() }
}
#[inline]
pub unsafe fn IcmpParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { IcmpParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho(icmphandle: super::HANDLE, destinationaddress: super::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::HANDLE, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho(icmphandle, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<*const core::ffi::c_void>, destinationaddress: super::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2Ex(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<*const core::ffi::c_void>, sourceaddress: super::IPAddr, destinationaddress: super::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : super::IPAddr, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2Ex(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
