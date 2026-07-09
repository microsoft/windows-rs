#[cfg(feature = "Win32_winnt")]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6CreateFile() -> super::winnt::HANDLE);
windows_link::link!("iphlpapi.dll" "system" fn Icmp6ParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_ipexport", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_ws2"))]
windows_link::link!("iphlpapi.dll" "system" fn Icmp6SendEcho2(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : *const super::ws2::SOCKADDR_IN6_LH, destinationaddress : *const super::ws2::SOCKADDR_IN6_LH, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCloseHandle(icmphandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("iphlpapi.dll" "system" fn IcmpCreateFile() -> super::winnt::HANDLE);
windows_link::link!("iphlpapi.dll" "system" fn IcmpParseReplies(replybuffer : *mut core::ffi::c_void, replysize : u32) -> u32);
#[cfg(all(feature = "Win32_ipexport", feature = "Win32_minwindef", feature = "Win32_ntddndis", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho(icmphandle : super::winnt::HANDLE, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(all(feature = "Win32_ipexport", feature = "Win32_minwindef", feature = "Win32_ntddndis", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
#[cfg(all(feature = "Win32_ipexport", feature = "Win32_minwindef", feature = "Win32_ntddndis", feature = "Win32_winnt"))]
windows_link::link!("iphlpapi.dll" "system" fn IcmpSendEcho2Ex(icmphandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::minwindef::FARPROC, apccontext : *const core::ffi::c_void, sourceaddress : super::ntddndis::IPAddr, destinationaddress : super::ntddndis::IPAddr, requestdata : *const core::ffi::c_void, requestsize : u16, requestoptions : *const super::ipexport::IP_OPTION_INFORMATION, replybuffer : *mut core::ffi::c_void, replysize : u32, timeout : u32) -> u32);
