#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::HANDLE);
windows_link::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
#[cfg(all(feature = "in6addr", feature = "ipexport", feature = "minwindef", feature = "winnt", feature = "ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : *const super::SOCKADDR_IN6_LH, destinationaddress : *const super::SOCKADDR_IN6_LH, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::HANDLE);
windows_link::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::HANDLE, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(all(feature = "ipexport", feature = "minwindef", feature = "ntddndis", feature = "winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::HANDLE, event : super::HANDLE, apcroutine : super::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : super::IPAddr, destinationaddress : super::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
