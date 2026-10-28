#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : super::LPVOID, replysize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "in6addr", feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, sourceaddress : *const super::SOCKADDR_IN6_LH, destinationaddress : *const super::SOCKADDR_IN6_LH, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : super::LPVOID, replysize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::HANDLE, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : super::PVOID, sourceaddress : super::IPAddr, destinationaddress : super::IPAddr, requestdata : super::LPVOID, requestsize : super::WORD, requestoptions : super::PIP_OPTION_INFORMATION, replybuffer : super::LPVOID, replysize : super::DWORD, timeout : super::DWORD) -> super::DWORD);
