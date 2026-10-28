#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn Icmp6CreateFile() -> super::HANDLE {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::HANDLE);
    unsafe { Icmp6CreateFile() }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn Icmp6ParseReplies(replybuffer: super::LPVOID, replysize: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : super::LPVOID, replysize : super::DWORD) -> super::DWORD);
    unsafe { Icmp6ParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "in6addr", feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn Icmp6SendEcho2(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<super::PVOID>, sourceaddress: *const super::SOCKADDR_IN6_LH, destinationaddress: *const super::SOCKADDR_IN6_LH, requestdata: super::LPVOID, requestsize: super::WORD, requestoptions: Option<super::PIP_OPTION_INFORMATION>, replybuffer: super::LPVOID, replysize: super::DWORD, timeout: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, sourceaddress : *const super::SOCKADDR_IN6_LH, destinationaddress : *const super::SOCKADDR_IN6_LH, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
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
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn IcmpParseReplies(replybuffer: super::LPVOID, replysize: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : super::LPVOID, replysize : super::DWORD) -> super::DWORD);
    unsafe { IcmpParseReplies(replybuffer as _, replysize) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho(icmphandle: super::HANDLE, destinationaddress: super::IPAddr, requestdata: super::LPVOID, requestsize: super::WORD, requestoptions: Option<super::PIP_OPTION_INFORMATION>, replybuffer: super::LPVOID, replysize: super::DWORD, timeout: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::HANDLE, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
    unsafe { IcmpSendEcho(icmphandle, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<super::PVOID>, destinationaddress: super::IPAddr, requestdata: super::LPVOID, requestsize: super::WORD, requestoptions: Option<super::PIP_OPTION_INFORMATION>, replybuffer: super::LPVOID, replysize: super::DWORD, timeout: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
    unsafe { IcmpSendEcho2(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn IcmpSendEcho2Ex(icmphandle: super::HANDLE, event: Option<super::HANDLE>, apcroutine: super::FARPROC, apccontext: Option<super::PVOID>, sourceaddress: super::IPAddr, destinationaddress: super::IPAddr, requestdata: super::LPVOID, requestsize: super::WORD, requestoptions: Option<super::PIP_OPTION_INFORMATION>, replybuffer: super::LPVOID, replysize: super::DWORD, timeout: super::DWORD) -> super::DWORD {
    windows_core::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, sourceaddress : super::IPAddr, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
    unsafe { IcmpSendEcho2Ex(icmphandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, sourceaddress, destinationaddress, requestdata, requestsize, requestoptions.unwrap_or(core::mem::zeroed()) as _, replybuffer as _, replysize, timeout) }
}
