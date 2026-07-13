#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Icmp6CreateFile() -> super::winnt::HANDLE {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::winnt::HANDLE);
    unsafe { Icmp6CreateFile() }
}
#[inline]
pub unsafe fn Icmp6ParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { Icmp6ParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "in6addr", feature = "ipexport", feature = "minwindef", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn Icmp6SendEcho2(icmphandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::minwindef::FARPROC, apccontext: Option<*const core::ffi::c_void>, sourceaddress: *const super::ws2::SOCKADDR_IN6_LH, destinationaddress: *const super::ws2::SOCKADDR_IN6_LH, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::ipexport::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : *const super::ws2::SOCKADDR_IN6_LH, destinationaddress : *const super::ws2::SOCKADDR_IN6_LH, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { Icmp6SendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IcmpCloseHandle(icmphandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { IcmpCloseHandle(icmphandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IcmpCreateFile() -> super::winnt::HANDLE {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::winnt::HANDLE);
    unsafe { IcmpCreateFile() }
}
#[inline]
pub unsafe fn IcmpParseReplies(replybuffer: *mut core::ffi::c_void, replysize: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
    unsafe { IcmpParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho(icmphandle: super::winnt::HANDLE, destinationaddress: super::ntddndis::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::ipexport::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::winnt::HANDLE, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho(icmphandle, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2(icmphandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::minwindef::FARPROC, apccontext: Option<*const core::ffi::c_void>, destinationaddress: super::ntddndis::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::ipexport::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2Ex(icmphandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::minwindef::FARPROC, apccontext: Option<*const core::ffi::c_void>, sourceaddress: super::ntddndis::IPAddr, destinationaddress: super::ntddndis::IPAddr, requestdata: *const core::ffi::c_void, requestsize: u16, requestoptions: Option<*const super::ipexport::IP_OPTION_INFORMATION>, replybuffer: *mut core::ffi::c_void, replysize: u32, timeout: u32) -> u32 {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : super::ntddndis::IPAddr, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
    unsafe { IcmpSendEcho2Ex(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
