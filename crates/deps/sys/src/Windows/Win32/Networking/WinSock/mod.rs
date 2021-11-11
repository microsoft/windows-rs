#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AcceptEx(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn EnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn EnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoEx(paddrinfoex: *const addrinfoexA);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoExW(paddrinfoex: *const addrinfoexW);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoW(paddrinfo: *const addrinfoW);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptExSockaddrs(lpoutputbuffer: *const ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, hints: *const addrinfoexA, ppresult: *mut *mut addrinfoexA, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoExCancel(lphandle: *const super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, hints: *const addrinfoexW, ppresult: *mut *mut addrinfoexW, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lphandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoW(pnodename: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, phints: *const addrinfoW, ppresult: *mut *mut addrinfoW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameA(dwnamespace: u32, lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameW(dwnamespace: u32, lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PWSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetHostNameW(name: super::super::Foundation::PWSTR, namelen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeA(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, dwnamelength: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeW(lpservicetype: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, dwnamelength: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PWSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PWSTR, servicebuffersize: u32, flags: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceA(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceW(dwnamespace: u32, lpguid: *const ::windows::runtime::GUID, lpservicename: super::super::Foundation::PWSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameA(lpservicename: super::super::Foundation::PSTR, lpservicetype: *mut ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameW(lpservicename: super::super::Foundation::PWSTR, lpservicetype: *mut ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetNtopW(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: super::super::Foundation::PWSTR, stringbufsize: usize) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetPtonW(family: i32, pszaddrstring: super::super::Foundation::PWSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ProcessSocketNotifications(completionport: super::super::Foundation::HANDLE, registrationcount: u32, registrationinfos: *mut SOCK_NOTIFY_REGISTRATION, timeoutms: u32, completioncount: u32, completionportentries: *mut super::super::System::IO::OVERLAPPED_ENTRY, receivedentrycount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringA(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringW(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_WindowsFilteringPlatform`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringA(addr: *const IN_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExA(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExW(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringW(addr: *const IN_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressA(s: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN_ADDR) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExA(addressstring: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExW(addressstring: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressW(s: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN_ADDR) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringA(addr: *const IN6_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExA(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExW(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringW(addr: *const IN6_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN6_ADDR) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExA(addressstring: super::super::Foundation::PSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExW(addressstring: super::super::Foundation::PWSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN6_ADDR) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows::runtime::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceA(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOA, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpdwstatusflags: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceW(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOW, lpserviceasyncinfo: *const ::core::mem::ManuallyDrop<SERVICE_ASYNC_INFO>, lpdwstatusflags: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSocketMediaStreamingMode(value: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TransmitFile(hsocket: SOCKET, hfile: super::super::Foundation::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WPUCompleteOverlappedRequest(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAAccept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: ::windows::runtime::RawPtr, dwcallbackdata: usize) -> SOCKET;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringA(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpszaddressstring: super::super::Foundation::PSTR, lpdwaddressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringW(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAAdvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID, pnspv2routine: *const ::core::mem::ManuallyDrop<NSPV2_ROUTINE>) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByAddr(hwnd: super::super::Foundation::HWND, wmsg: u32, addr: super::super::Foundation::PSTR, len: i32, r#type: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByNumber(hwnd: super::super::Foundation::HWND, wmsg: u32, number: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByPort(hwnd: super::super::Foundation::HWND, wmsg: u32, port: i32, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncSelect(s: SOCKET, hwnd: super::super::Foundation::HWND, wmsg: u32, levent: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACancelAsyncRequest(hasynctaskhandle: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSACancelBlockingCall() -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSACleanup() -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACloseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAConnect(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByList(s: SOCKET, socketaddress: *const SOCKET_ADDRESS_LIST, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameA(s: SOCKET, nodename: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameW(s: SOCKET, nodename: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACreateEvent() -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSADeleteSocketPeerTargetName(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSADuplicateSocketA(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSADuplicateSocketW(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNetworkEvents(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOA, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAEnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEventSelect(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lnetworkevents: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAGetLastError() -> WSA_ERROR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: super::super::Foundation::BOOL, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAGetQOSByName(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoA(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoW(lpproviderid: *const ::windows::runtime::GUID, lpserviceclassid: *const ::windows::runtime::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const ::windows::runtime::GUID, lpszserviceclassname: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAHtonl(s: SOCKET, hostlong: u32, lpnetlong: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAHtons(s: SOCKET, hostshort: u16, lpnetshort: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAImpersonateSocketPeer(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAIoctl(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAIsBlocking() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_NetworkManagement_QoS`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAJoinLeaf(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32) -> SOCKET;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSALookupServiceEnd(hlookup: super::super::Foundation::HANDLE) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextA(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextW(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSANSPIoctl(hlookup: super::super::Foundation::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const ::core::mem::ManuallyDrop<WSACOMPLETION>) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSANtohl(s: SOCKET, netlong: u32, lphostlong: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSANtohs(s: SOCKET, netshort: u16, lphostshort: *mut u16) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAProviderCompleteAsyncCall(hasynccall: super::super::Foundation::HANDLE, iretcode: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAProviderConfigChange(lpnotificationhandle: *mut super::super::Foundation::HANDLE, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAQuerySocketSecurity(socket: SOCKET, securityquerytemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE, securityquerytemplatelen: u32, securityqueryinfo: *mut SOCKET_SECURITY_QUERY_INFO, securityqueryinfolen: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecv(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: *const WSABUF) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvEx(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecvFrom(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSARemoveServiceClass(lpserviceclassid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSARevertImpersonation() -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASend(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendMsg(handle: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendTo(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetBlockingHook(lpblockfunc: ::windows::runtime::RawPtr) -> ::core::option::Option<super::super::Foundation::FARPROC>;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSASetLastError(ierror: i32);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketPeerTargetName(socket: SOCKET, peertargetname: *const SOCKET_PEER_TARGET_NAME, peertargetnamelen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_IO`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketSecurity(socket: SOCKET, securitysettings: *const SOCKET_SECURITY_SETTINGS, securitysettingslen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: ::windows::runtime::RawPtr) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, g: u32, dwflags: u32) -> SOCKET;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32) -> SOCKET;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSAData) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressA(addressstring: super::super::Foundation::PSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressW(addressstring: super::super::Foundation::PWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAUnadvertiseProvider(puuidproviderid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSAUnhookBlockingHook() -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAWaitForMultipleEvents(cevents: u32, lphevents: *const super::super::Foundation::HANDLE, fwaitall: super::super::Foundation::BOOL, dwtimeout: u32, falertable: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCDeinstallProvider(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCDeinstallProvider32(lpproviderid: *const ::windows::runtime::GUID, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider(lpproviderid: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider32(lpproviderid: *const ::windows::runtime::GUID, fenable: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnumNameSpaceProviders32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCEnumNameSpaceProvidersEx32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCEnumProtocols(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCEnumProtocols32(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, ppermittedlspcategories: *mut u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCGetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCGetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows::runtime::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider64_32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProviderAndChains64_32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpszproviderdllpath32: super::super::Foundation::PWSTR, lpszlspname: super::super::Foundation::PWSTR, dwserviceflags: u32, lpprotocolinfolist: *mut WSAPROTOCOL_INFOW, dwnumberofentries: u32, lpdwcatalogentryid: *mut u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCSetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, permittedlspcategories: u32, pprevpermlspcat: *mut u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCSetProviderInfo(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCSetProviderInfo32(lpproviderid: *const ::windows::runtime::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCUnInstallNameSpace(lpproviderid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCUnInstallNameSpace32(lpproviderid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider32(lpproviderid: *const ::windows::runtime::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCWriteNameSpaceOrder(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteNameSpaceOrder32(lpproviderid: *mut ::windows::runtime::GUID, dwnumberofentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn WSCWriteProviderOrder(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteProviderOrder32(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn accept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn closesocket(s: SOCKET) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn freeaddrinfo(paddrinfo: *const ADDRINFOA);
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getaddrinfo(pnodename: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, phints: *const ADDRINFOA, ppresult: *mut *mut ADDRINFOA) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyaddr(addr: super::super::Foundation::PSTR, len: i32, r#type: i32) -> *mut hostent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyname(name: super::super::Foundation::PSTR) -> *mut hostent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostname(name: super::super::Foundation::PSTR, namelen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PSTR, servicebuffersize: u32, flags: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getpeername(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobyname(name: super::super::Foundation::PSTR) -> *mut protoent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobynumber(number: i32) -> *mut protoent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyname(name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR) -> *mut servent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyport(port: i32, proto: super::super::Foundation::PSTR) -> *mut servent;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockname(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn htonl(hostlong: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn htons(hostshort: u16) -> u16;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_addr(cp: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntoa(r#in: IN_ADDR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntop(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: super::super::Foundation::PSTR, stringbufsize: usize) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_pton(family: i32, pszaddrstring: super::super::Foundation::PSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn listen(s: SOCKET, backlog: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ntohl(netlong: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn ntohs(netshort: u16) -> u16;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn recv(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn recvfrom(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, from: *mut SOCKADDR, fromlen: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn select(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn send(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: SEND_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sendto(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, to: *const SOCKADDR, tolen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn shutdown(s: SOCKET, how: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_WinSock`*"]
    pub fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET;
}
