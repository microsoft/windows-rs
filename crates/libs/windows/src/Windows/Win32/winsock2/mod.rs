#[inline]
pub unsafe fn GetHostNameW(name: windows_core::PWSTR, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetHostNameW(name : windows_core::PWSTR, namelen : i32) -> i32);
    unsafe { GetHostNameW(name, namelen) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ProcessSocketNotifications(completionport: super::HANDLE, registrationinfos: Option<&mut [SOCK_NOTIFY_REGISTRATION]>, timeoutms: super::UINT32, completioncount: super::ULONG, completionportentries: Option<*mut super::OVERLAPPED_ENTRY>, receivedentrycount: Option<*mut super::UINT32>) -> super::DWORD {
    windows_core::link!("ws2_32.dll" "system" fn ProcessSocketNotifications(completionport : super::HANDLE, registrationcount : super::UINT32, registrationinfos : *mut SOCK_NOTIFY_REGISTRATION, timeoutms : super::UINT32, completioncount : super::ULONG, completionportentries : *mut super::OVERLAPPED_ENTRY, receivedentrycount : *mut super::UINT32) -> super::DWORD);
    unsafe { ProcessSocketNotifications(completionport, registrationinfos.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), registrationinfos.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), timeoutms, completioncount, completionportentries.unwrap_or(core::mem::zeroed()) as _, receivedentrycount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "qos", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAAccept(s: SOCKET, addr: Option<*mut super::SOCKADDR>, addrlen: Option<super::LPINT>, lpfncondition: LPCONDITIONPROC, dwcallbackdata: Option<super::DWORD_PTR>) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSAAccept(s : SOCKET, addr : *mut super::SOCKADDR, addrlen : super::LPINT, lpfncondition : LPCONDITIONPROC, dwcallbackdata : super::DWORD_PTR) -> SOCKET);
    unsafe { WSAAccept(s, addr.unwrap_or(core::mem::zeroed()) as _, addrlen.unwrap_or(core::mem::zeroed()) as _, lpfncondition, dwcallbackdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAAddressToStringA(lpsaaddress: super::LPSOCKADDR, dwaddresslength: super::DWORD, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOA>, lpszaddressstring: windows_core::PSTR, lpdwaddressstringlength: super::LPDWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAAddressToStringA(lpsaaddress : super::LPSOCKADDR, dwaddresslength : super::DWORD, lpprotocolinfo : LPWSAPROTOCOL_INFOA, lpszaddressstring : windows_core::PSTR, lpdwaddressstringlength : super::LPDWORD) -> super::INT);
    unsafe { WSAAddressToStringA(lpsaaddress, dwaddresslength, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpszaddressstring, lpdwaddressstringlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAAddressToStringW(lpsaaddress: super::LPSOCKADDR, dwaddresslength: super::DWORD, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOW>, lpszaddressstring: windows_core::PWSTR, lpdwaddressstringlength: super::LPDWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAAddressToStringW(lpsaaddress : super::LPSOCKADDR, dwaddresslength : super::DWORD, lpprotocolinfo : LPWSAPROTOCOL_INFOW, lpszaddressstring : windows_core::PWSTR, lpdwaddressstringlength : super::LPDWORD) -> super::INT);
    unsafe { WSAAddressToStringW(lpsaaddress, dwaddresslength, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpszaddressstring, lpdwaddressstringlength as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetHostByAddr(hwnd: super::HWND, wmsg: u_int, addr: &[u8], r#type: i32, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetHostByAddr(hwnd : super::HWND, wmsg : u_int, addr : *const i8, len : i32, r#type : i32, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetHostByAddr(hwnd, wmsg, core::mem::transmute(addr.as_ptr()), addr.len().try_into().unwrap(), r#type, buf as _, buflen) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetHostByName(hwnd: super::HWND, wmsg: u_int, name: *const i8, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetHostByName(hwnd : super::HWND, wmsg : u_int, name : *const i8, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetHostByName(hwnd, wmsg, name, buf as _, buflen) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetProtoByName(hwnd: super::HWND, wmsg: u_int, name: *const i8, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetProtoByName(hwnd : super::HWND, wmsg : u_int, name : *const i8, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetProtoByName(hwnd, wmsg, name, buf as _, buflen) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetProtoByNumber(hwnd: super::HWND, wmsg: u_int, number: i32, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetProtoByNumber(hwnd : super::HWND, wmsg : u_int, number : i32, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetProtoByNumber(hwnd, wmsg, number, buf as _, buflen) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetServByName(hwnd: super::HWND, wmsg: u_int, name: *const i8, proto: *const i8, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetServByName(hwnd : super::HWND, wmsg : u_int, name : *const i8, proto : *const i8, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetServByName(hwnd, wmsg, name, proto, buf as _, buflen) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetServByPort(hwnd: super::HWND, wmsg: u_int, port: i32, proto: *const i8, buf: *mut i8, buflen: i32) -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetServByPort(hwnd : super::HWND, wmsg : u_int, port : i32, proto : *const i8, buf : *mut i8, buflen : i32) -> super::HANDLE);
    unsafe { WSAAsyncGetServByPort(hwnd, wmsg, port, proto, buf as _, buflen) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WSAAsyncSelect(s: SOCKET, hwnd: super::HWND, wmsg: u_int, levent: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncSelect(s : SOCKET, hwnd : super::HWND, wmsg : u_int, levent : i32) -> i32);
    unsafe { WSAAsyncSelect(s, hwnd, wmsg, levent) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSACancelAsyncRequest(hasynctaskhandle: super::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSACancelAsyncRequest(hasynctaskhandle : super::HANDLE) -> i32);
    unsafe { WSACancelAsyncRequest(hasynctaskhandle) }
}
#[inline]
pub unsafe fn WSACancelBlockingCall() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSACancelBlockingCall() -> i32);
    unsafe { WSACancelBlockingCall() }
}
#[inline]
pub unsafe fn WSACleanup() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSACleanup() -> i32);
    unsafe { WSACleanup() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSACloseEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSACloseEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { WSACloseEvent(hevent) }
}
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnect(s: SOCKET, name: *const super::SOCKADDR, namelen: i32, lpcallerdata: Option<super::LPWSABUF>, lpcalleedata: Option<super::LPWSABUF>, lpsqos: Option<LPQOS>, lpgqos: Option<LPQOS>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAConnect(s : SOCKET, name : *const super::SOCKADDR, namelen : i32, lpcallerdata : super::LPWSABUF, lpcalleedata : super::LPWSABUF, lpsqos : LPQOS, lpgqos : LPQOS) -> i32);
    unsafe { WSAConnect(s, name, namelen, lpcallerdata.unwrap_or(core::mem::zeroed()) as _, lpcalleedata.unwrap_or(core::mem::zeroed()) as _, lpsqos.unwrap_or(core::mem::zeroed()) as _, lpgqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByList(s: SOCKET, socketaddress: super::PSOCKET_ADDRESS_LIST, localaddresslength: Option<super::LPDWORD>, localaddress: Option<super::LPSOCKADDR>, remoteaddresslength: Option<super::LPDWORD>, remoteaddress: Option<super::LPSOCKADDR>, timeout: Option<*const timeval>, reserved: LPWSAOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByList(s : SOCKET, socketaddress : super::PSOCKET_ADDRESS_LIST, localaddresslength : super::LPDWORD, localaddress : super::LPSOCKADDR, remoteaddresslength : super::LPDWORD, remoteaddress : super::LPSOCKADDR, timeout : *const timeval, reserved : LPWSAOVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByList(s, socketaddress, localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByNameA<P1, P2>(s: SOCKET, nodename: P1, servicename: P2, localaddresslength: Option<super::LPDWORD>, localaddress: Option<super::LPSOCKADDR>, remoteaddresslength: Option<super::LPDWORD>, remoteaddress: Option<super::LPSOCKADDR>, timeout: Option<*const timeval>, reserved: LPWSAOVERLAPPED) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByNameA(s : SOCKET, nodename : windows_core::PCSTR, servicename : windows_core::PCSTR, localaddresslength : super::LPDWORD, localaddress : super::LPSOCKADDR, remoteaddresslength : super::LPDWORD, remoteaddress : super::LPSOCKADDR, timeout : *const timeval, reserved : LPWSAOVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByNameA(s, nodename.param().abi(), servicename.param().abi(), localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByNameW<P1, P2>(s: SOCKET, nodename: P1, servicename: P2, localaddresslength: Option<super::LPDWORD>, localaddress: Option<super::LPSOCKADDR>, remoteaddresslength: Option<super::LPDWORD>, remoteaddress: Option<super::LPSOCKADDR>, timeout: Option<*const timeval>, reserved: LPWSAOVERLAPPED) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByNameW(s : SOCKET, nodename : windows_core::PCWSTR, servicename : windows_core::PCWSTR, localaddresslength : super::LPDWORD, localaddress : super::LPSOCKADDR, remoteaddresslength : super::LPDWORD, remoteaddress : super::LPSOCKADDR, timeout : *const timeval, reserved : LPWSAOVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByNameW(s, nodename.param().abi(), servicename.param().abi(), localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSACreateEvent() -> super::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSACreateEvent() -> super::HANDLE);
    unsafe { WSACreateEvent() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSADuplicateSocketA(s: SOCKET, dwprocessid: super::DWORD, lpprotocolinfo: LPWSAPROTOCOL_INFOA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSADuplicateSocketA(s : SOCKET, dwprocessid : super::DWORD, lpprotocolinfo : LPWSAPROTOCOL_INFOA) -> i32);
    unsafe { WSADuplicateSocketA(s, dwprocessid, lpprotocolinfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSADuplicateSocketW(s: SOCKET, dwprocessid: super::DWORD, lpprotocolinfo: LPWSAPROTOCOL_INFOW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSADuplicateSocketW(s : SOCKET, dwprocessid : super::DWORD, lpprotocolinfo : LPWSAPROTOCOL_INFOW) -> i32);
    unsafe { WSADuplicateSocketW(s, dwprocessid, lpprotocolinfo as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: super::LPDWORD, lpnspbuffer: LPWSANAMESPACE_INFOA) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersA(lpdwbufferlength : super::LPDWORD, lpnspbuffer : LPWSANAMESPACE_INFOA) -> super::INT);
    unsafe { WSAEnumNameSpaceProvidersA(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: super::LPDWORD, lpnspbuffer: LPWSANAMESPACE_INFOEXA) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength : super::LPDWORD, lpnspbuffer : LPWSANAMESPACE_INFOEXA) -> super::INT);
    unsafe { WSAEnumNameSpaceProvidersExA(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: super::LPDWORD, lpnspbuffer: LPWSANAMESPACE_INFOEXW) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength : super::LPDWORD, lpnspbuffer : LPWSANAMESPACE_INFOEXW) -> super::INT);
    unsafe { WSAEnumNameSpaceProvidersExW(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: super::LPDWORD, lpnspbuffer: LPWSANAMESPACE_INFOW) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersW(lpdwbufferlength : super::LPDWORD, lpnspbuffer : LPWSANAMESPACE_INFOW) -> super::INT);
    unsafe { WSAEnumNameSpaceProvidersW(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAEnumNetworkEvents(s: SOCKET, heventobject: super::HANDLE, lpnetworkevents: LPWSANETWORKEVENTS) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNetworkEvents(s : SOCKET, heventobject : super::HANDLE, lpnetworkevents : LPWSANETWORKEVENTS) -> i32);
    unsafe { WSAEnumNetworkEvents(s, heventobject, lpnetworkevents as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAEnumProtocolsA(lpiprotocols: Option<super::LPINT>, lpprotocolbuffer: Option<LPWSAPROTOCOL_INFOA>, lpdwbufferlength: super::LPDWORD) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumProtocolsA(lpiprotocols : super::LPINT, lpprotocolbuffer : LPWSAPROTOCOL_INFOA, lpdwbufferlength : super::LPDWORD) -> i32);
    unsafe { WSAEnumProtocolsA(lpiprotocols.unwrap_or(core::mem::zeroed()) as _, lpprotocolbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSAEnumProtocolsW(lpiprotocols: Option<super::LPINT>, lpprotocolbuffer: Option<LPWSAPROTOCOL_INFOW>, lpdwbufferlength: super::LPDWORD) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumProtocolsW(lpiprotocols : super::LPINT, lpprotocolbuffer : LPWSAPROTOCOL_INFOW, lpdwbufferlength : super::LPDWORD) -> i32);
    unsafe { WSAEnumProtocolsW(lpiprotocols.unwrap_or(core::mem::zeroed()) as _, lpprotocolbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAEventSelect(s: SOCKET, heventobject: Option<super::HANDLE>, lnetworkevents: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEventSelect(s : SOCKET, heventobject : super::HANDLE, lnetworkevents : i32) -> i32);
    unsafe { WSAEventSelect(s, heventobject.unwrap_or(core::mem::zeroed()) as _, lnetworkevents) }
}
#[inline]
pub unsafe fn WSAGetLastError() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetLastError() -> i32);
    unsafe { WSAGetLastError() }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: LPWSAOVERLAPPED, lpcbtransfer: super::LPDWORD, fwait: bool, lpdwflags: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetOverlappedResult(s : SOCKET, lpoverlapped : LPWSAOVERLAPPED, lpcbtransfer : super::LPDWORD, fwait : windows_core::BOOL, lpdwflags : super::LPDWORD) -> windows_core::BOOL);
    unsafe { WSAGetOverlappedResult(s, lpoverlapped, lpcbtransfer as _, fwait.into(), lpdwflags as _) }
}
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAGetQOSByName(s: SOCKET, lpqosname: super::LPWSABUF, lpqos: LPQOS) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetQOSByName(s : SOCKET, lpqosname : super::LPWSABUF, lpqos : LPQOS) -> windows_core::BOOL);
    unsafe { WSAGetQOSByName(s, lpqosname, lpqos as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAGetServiceClassInfoA(lpproviderid: super::LPGUID, lpserviceclassid: super::LPGUID, lpdwbufsize: super::LPDWORD, lpserviceclassinfo: LPWSASERVICECLASSINFOA) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassInfoA(lpproviderid : super::LPGUID, lpserviceclassid : super::LPGUID, lpdwbufsize : super::LPDWORD, lpserviceclassinfo : LPWSASERVICECLASSINFOA) -> super::INT);
    unsafe { WSAGetServiceClassInfoA(lpproviderid, lpserviceclassid, lpdwbufsize as _, lpserviceclassinfo as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAGetServiceClassInfoW(lpproviderid: super::LPGUID, lpserviceclassid: super::LPGUID, lpdwbufsize: super::LPDWORD, lpserviceclassinfo: LPWSASERVICECLASSINFOW) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassInfoW(lpproviderid : super::LPGUID, lpserviceclassid : super::LPGUID, lpdwbufsize : super::LPDWORD, lpserviceclassinfo : LPWSASERVICECLASSINFOW) -> super::INT);
    unsafe { WSAGetServiceClassInfoW(lpproviderid, lpserviceclassid, lpdwbufsize as _, lpserviceclassinfo as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: super::LPGUID, lpszserviceclassname: windows_core::PSTR, lpdwbufferlength: super::LPDWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassNameByClassIdA(lpserviceclassid : super::LPGUID, lpszserviceclassname : windows_core::PSTR, lpdwbufferlength : super::LPDWORD) -> super::INT);
    unsafe { WSAGetServiceClassNameByClassIdA(lpserviceclassid, lpszserviceclassname, lpdwbufferlength as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: super::LPGUID, lpszserviceclassname: windows_core::PWSTR, lpdwbufferlength: super::LPDWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassNameByClassIdW(lpserviceclassid : super::LPGUID, lpszserviceclassname : windows_core::PWSTR, lpdwbufferlength : super::LPDWORD) -> super::INT);
    unsafe { WSAGetServiceClassNameByClassIdW(lpserviceclassid, lpszserviceclassname, lpdwbufferlength as _) }
}
#[inline]
pub unsafe fn WSAHtonl(s: SOCKET, hostlong: u_long, lpnetlong: *mut u_long) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAHtonl(s : SOCKET, hostlong : u_long, lpnetlong : *mut u_long) -> i32);
    unsafe { WSAHtonl(s, hostlong, lpnetlong as _) }
}
#[inline]
pub unsafe fn WSAHtons(s: SOCKET, hostshort: u_short, lpnetshort: *mut u_short) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAHtons(s : SOCKET, hostshort : u_short, lpnetshort : *mut u_short) -> i32);
    unsafe { WSAHtons(s, hostshort, lpnetshort as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAInstallServiceClassA(lpserviceclassinfo: LPWSASERVICECLASSINFOA) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAInstallServiceClassA(lpserviceclassinfo : LPWSASERVICECLASSINFOA) -> super::INT);
    unsafe { WSAInstallServiceClassA(lpserviceclassinfo) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSAInstallServiceClassW(lpserviceclassinfo: LPWSASERVICECLASSINFOW) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAInstallServiceClassW(lpserviceclassinfo : LPWSASERVICECLASSINFOW) -> super::INT);
    unsafe { WSAInstallServiceClassW(lpserviceclassinfo) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSAIoctl(s: SOCKET, dwiocontrolcode: super::DWORD, lpvinbuffer: Option<super::LPVOID>, cbinbuffer: super::DWORD, lpvoutbuffer: Option<super::LPVOID>, cboutbuffer: super::DWORD, lpcbbytesreturned: super::LPDWORD, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAIoctl(s : SOCKET, dwiocontrolcode : super::DWORD, lpvinbuffer : super::LPVOID, cbinbuffer : super::DWORD, lpvoutbuffer : super::LPVOID, cboutbuffer : super::DWORD, lpcbbytesreturned : super::LPDWORD, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSAIoctl(s, dwiocontrolcode, lpvinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffer, lpvoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffer, lpcbbytesreturned as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[inline]
pub unsafe fn WSAIsBlocking() -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAIsBlocking() -> windows_core::BOOL);
    unsafe { WSAIsBlocking() }
}
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAJoinLeaf(s: SOCKET, name: *const super::SOCKADDR, namelen: i32, lpcallerdata: Option<super::LPWSABUF>, lpcalleedata: Option<super::LPWSABUF>, lpsqos: Option<LPQOS>, lpgqos: Option<LPQOS>, dwflags: super::DWORD) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSAJoinLeaf(s : SOCKET, name : *const super::SOCKADDR, namelen : i32, lpcallerdata : super::LPWSABUF, lpcalleedata : super::LPWSABUF, lpsqos : LPQOS, lpgqos : LPQOS, dwflags : super::DWORD) -> SOCKET);
    unsafe { WSAJoinLeaf(s, name, namelen, lpcallerdata.unwrap_or(core::mem::zeroed()) as _, lpcalleedata.unwrap_or(core::mem::zeroed()) as _, lpsqos.unwrap_or(core::mem::zeroed()) as _, lpgqos.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceBeginA(lpqsrestrictions: LPWSAQUERYSETA, dwcontrolflags: super::DWORD, lphlookup: super::LPHANDLE) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceBeginA(lpqsrestrictions : LPWSAQUERYSETA, dwcontrolflags : super::DWORD, lphlookup : super::LPHANDLE) -> super::INT);
    unsafe { WSALookupServiceBeginA(lpqsrestrictions, dwcontrolflags, lphlookup as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceBeginW(lpqsrestrictions: LPWSAQUERYSETW, dwcontrolflags: super::DWORD, lphlookup: super::LPHANDLE) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceBeginW(lpqsrestrictions : LPWSAQUERYSETW, dwcontrolflags : super::DWORD, lphlookup : super::LPHANDLE) -> super::INT);
    unsafe { WSALookupServiceBeginW(lpqsrestrictions, dwcontrolflags, lphlookup as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSALookupServiceEnd(hlookup: super::HANDLE) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceEnd(hlookup : super::HANDLE) -> super::INT);
    unsafe { WSALookupServiceEnd(hlookup) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceNextA(hlookup: super::HANDLE, dwcontrolflags: super::DWORD, lpdwbufferlength: super::LPDWORD, lpqsresults: LPWSAQUERYSETA) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceNextA(hlookup : super::HANDLE, dwcontrolflags : super::DWORD, lpdwbufferlength : super::LPDWORD, lpqsresults : LPWSAQUERYSETA) -> super::INT);
    unsafe { WSALookupServiceNextA(hlookup, dwcontrolflags, lpdwbufferlength as _, lpqsresults as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceNextW(hlookup: super::HANDLE, dwcontrolflags: super::DWORD, lpdwbufferlength: super::LPDWORD, lpqsresults: Option<LPWSAQUERYSETW>) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceNextW(hlookup : super::HANDLE, dwcontrolflags : super::DWORD, lpdwbufferlength : super::LPDWORD, lpqsresults : LPWSAQUERYSETW) -> super::INT);
    unsafe { WSALookupServiceNextW(hlookup, dwcontrolflags, lpdwbufferlength as _, lpqsresults.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSANSPIoctl(hlookup: super::HANDLE, dwcontrolcode: super::DWORD, lpvinbuffer: Option<super::LPVOID>, cbinbuffer: super::DWORD, lpvoutbuffer: Option<super::LPVOID>, cboutbuffer: super::DWORD, lpcbbytesreturned: super::LPDWORD, lpcompletion: Option<LPWSACOMPLETION>) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSANSPIoctl(hlookup : super::HANDLE, dwcontrolcode : super::DWORD, lpvinbuffer : super::LPVOID, cbinbuffer : super::DWORD, lpvoutbuffer : super::LPVOID, cboutbuffer : super::DWORD, lpcbbytesreturned : super::LPDWORD, lpcompletion : LPWSACOMPLETION) -> super::INT);
    unsafe { WSANSPIoctl(hlookup, dwcontrolcode, lpvinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffer, lpvoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffer, lpcbbytesreturned as _, lpcompletion.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WSANtohl(s: SOCKET, netlong: u_long, lphostlong: *mut u_long) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSANtohl(s : SOCKET, netlong : u_long, lphostlong : *mut u_long) -> i32);
    unsafe { WSANtohl(s, netlong, lphostlong as _) }
}
#[inline]
pub unsafe fn WSANtohs(s: SOCKET, netshort: u_short, lphostshort: *mut u_short) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSANtohs(s : SOCKET, netshort : u_short, lphostshort : *mut u_short) -> i32);
    unsafe { WSANtohs(s, netshort, lphostshort as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSAPoll(fdarray: LPWSAPOLLFD, fds: super::ULONG, timeout: super::INT) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAPoll(fdarray : LPWSAPOLLFD, fds : super::ULONG, timeout : super::INT) -> i32);
    unsafe { WSAPoll(fdarray as _, fds, timeout) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSAProviderConfigChange(lpnotificationhandle: super::LPHANDLE, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSAProviderConfigChange(lpnotificationhandle : super::LPHANDLE, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> super::INT);
    unsafe { WSAProviderConfigChange(lpnotificationhandle as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSARecv(s: SOCKET, lpbuffers: super::LPWSABUF, dwbuffercount: super::DWORD, lpnumberofbytesrecvd: Option<super::LPDWORD>, lpflags: super::LPDWORD, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecv(s : SOCKET, lpbuffers : super::LPWSABUF, dwbuffercount : super::DWORD, lpnumberofbytesrecvd : super::LPDWORD, lpflags : super::LPDWORD, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSARecv(s, lpbuffers, dwbuffercount, lpnumberofbytesrecvd.unwrap_or(core::mem::zeroed()) as _, lpflags as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: Option<super::LPWSABUF>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecvDisconnect(s : SOCKET, lpinbounddisconnectdata : super::LPWSABUF) -> i32);
    unsafe { WSARecvDisconnect(s, lpinbounddisconnectdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSARecvFrom(s: SOCKET, lpbuffers: super::LPWSABUF, dwbuffercount: super::DWORD, lpnumberofbytesrecvd: Option<super::LPDWORD>, lpflags: super::LPDWORD, lpfrom: Option<*mut super::SOCKADDR>, lpfromlen: Option<super::LPINT>, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecvFrom(s : SOCKET, lpbuffers : super::LPWSABUF, dwbuffercount : super::DWORD, lpnumberofbytesrecvd : super::LPDWORD, lpflags : super::LPDWORD, lpfrom : *mut super::SOCKADDR, lpfromlen : super::LPINT, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSARecvFrom(s, lpbuffers, dwbuffercount, lpnumberofbytesrecvd.unwrap_or(core::mem::zeroed()) as _, lpflags as _, lpfrom.unwrap_or(core::mem::zeroed()) as _, lpfromlen.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn WSARemoveServiceClass(lpserviceclassid: super::LPGUID) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSARemoveServiceClass(lpserviceclassid : super::LPGUID) -> super::INT);
    unsafe { WSARemoveServiceClass(lpserviceclassid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAResetEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAResetEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { WSAResetEvent(hevent) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASend(s: SOCKET, lpbuffers: &[super::WSABUF], lpnumberofbytessent: Option<super::LPDWORD>, dwflags: super::DWORD, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASend(s : SOCKET, lpbuffers : super::LPWSABUF, dwbuffercount : super::DWORD, lpnumberofbytessent : super::LPDWORD, dwflags : super::DWORD, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASend(s, core::mem::transmute(lpbuffers.as_ptr()), lpbuffers.len().try_into().unwrap(), lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, dwflags, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: Option<super::LPWSABUF>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendDisconnect(s : SOCKET, lpoutbounddisconnectdata : super::LPWSABUF) -> i32);
    unsafe { WSASendDisconnect(s, lpoutbounddisconnectdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASendMsg(handle: SOCKET, lpmsg: super::LPWSAMSG, dwflags: super::DWORD, lpnumberofbytessent: Option<super::LPDWORD>, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendMsg(handle : SOCKET, lpmsg : super::LPWSAMSG, dwflags : super::DWORD, lpnumberofbytessent : super::LPDWORD, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASendMsg(handle, lpmsg, dwflags, lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASendTo(s: SOCKET, lpbuffers: &[super::WSABUF], lpnumberofbytessent: Option<super::LPDWORD>, dwflags: super::DWORD, lpto: Option<*const super::SOCKADDR>, itolen: i32, lpoverlapped: Option<LPWSAOVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendTo(s : SOCKET, lpbuffers : super::LPWSABUF, dwbuffercount : super::DWORD, lpnumberofbytessent : super::LPDWORD, dwflags : super::DWORD, lpto : *const super::SOCKADDR, itolen : i32, lpoverlapped : LPWSAOVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASendTo(s, core::mem::transmute(lpbuffers.as_ptr()), lpbuffers.len().try_into().unwrap(), lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, dwflags, lpto.unwrap_or(core::mem::zeroed()) as _, itolen, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSASetBlockingHook(lpblockfunc: super::FARPROC) -> super::FARPROC {
    windows_core::link!("ws2_32.dll" "system" fn WSASetBlockingHook(lpblockfunc : super::FARPROC) -> super::FARPROC);
    unsafe { WSASetBlockingHook(lpblockfunc) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSASetEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSASetEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { WSASetEvent(hevent) }
}
#[inline]
pub unsafe fn WSASetLastError(ierror: i32) {
    windows_core::link!("ws2_32.dll" "system" fn WSASetLastError(ierror : i32));
    unsafe { WSASetLastError(ierror) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSASetServiceA(lpqsreginfo: LPWSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: super::DWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSASetServiceA(lpqsreginfo : LPWSAQUERYSETA, essoperation : WSAESETSERVICEOP, dwcontrolflags : super::DWORD) -> super::INT);
    unsafe { WSASetServiceA(lpqsreginfo, essoperation, dwcontrolflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSASetServiceW(lpqsreginfo: LPWSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: super::DWORD) -> super::INT {
    windows_core::link!("ws2_32.dll" "system" fn WSASetServiceW(lpqsreginfo : LPWSAQUERYSETW, essoperation : WSAESETSERVICEOP, dwcontrolflags : super::DWORD) -> super::INT);
    unsafe { WSASetServiceW(lpqsreginfo, essoperation, dwcontrolflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOA>, g: GROUP, dwflags: super::DWORD) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSASocketA(af : i32, r#type : i32, protocol : i32, lpprotocolinfo : LPWSAPROTOCOL_INFOA, g : GROUP, dwflags : super::DWORD) -> SOCKET);
    unsafe { WSASocketA(af, r#type, protocol, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, g, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOW>, g: GROUP, dwflags: super::DWORD) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSASocketW(af : i32, r#type : i32, protocol : i32, lpprotocolinfo : LPWSAPROTOCOL_INFOW, g : GROUP, dwflags : super::DWORD) -> SOCKET);
    unsafe { WSASocketW(af, r#type, protocol, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, g, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSAStartup(wversionrequested: super::WORD, lpwsadata: LPWSADATA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAStartup(wversionrequested : super::WORD, lpwsadata : LPWSADATA) -> i32);
    unsafe { WSAStartup(wversionrequested, lpwsadata as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn WSAStringToAddressA<P0>(addressstring: P0, addressfamily: super::INT, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOA>, lpaddress: super::LPSOCKADDR, lpaddresslength: super::LPINT) -> super::INT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAStringToAddressA(addressstring : windows_core::PCSTR, addressfamily : super::INT, lpprotocolinfo : LPWSAPROTOCOL_INFOA, lpaddress : super::LPSOCKADDR, lpaddresslength : super::LPINT) -> super::INT);
    unsafe { WSAStringToAddressA(addressstring.param().abi(), addressfamily, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpaddress as _, lpaddresslength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAStringToAddressW<P0>(addressstring: P0, addressfamily: super::INT, lpprotocolinfo: Option<LPWSAPROTOCOL_INFOW>, lpaddress: super::LPSOCKADDR, lpaddresslength: super::LPINT) -> super::INT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAStringToAddressW(addressstring : windows_core::PCWSTR, addressfamily : super::INT, lpprotocolinfo : LPWSAPROTOCOL_INFOW, lpaddress : super::LPSOCKADDR, lpaddresslength : super::LPINT) -> super::INT);
    unsafe { WSAStringToAddressW(addressstring.param().abi(), addressfamily, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpaddress as _, lpaddresslength as _) }
}
#[inline]
pub unsafe fn WSAUnhookBlockingHook() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAUnhookBlockingHook() -> i32);
    unsafe { WSAUnhookBlockingHook() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSAWaitForMultipleEvents(lphevents: &[super::HANDLE], fwaitall: bool, dwtimeout: super::DWORD, falertable: bool) -> super::DWORD {
    windows_core::link!("ws2_32.dll" "system" fn WSAWaitForMultipleEvents(cevents : super::DWORD, lphevents : *const super::HANDLE, fwaitall : windows_core::BOOL, dwtimeout : super::DWORD, falertable : windows_core::BOOL) -> super::DWORD);
    unsafe { WSAWaitForMultipleEvents(lphevents.len().try_into().unwrap(), lphevents.as_ptr(), fwaitall.into(), dwtimeout, falertable.into()) }
}
#[inline]
pub unsafe fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn __WSAFDIsSet(fd : SOCKET, param1 : *mut fd_set) -> i32);
    unsafe { __WSAFDIsSet(fd, param1 as _) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn accept(s: SOCKET, addr: Option<*mut super::SOCKADDR>, addrlen: Option<*mut i32>) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn accept(s : SOCKET, addr : *mut super::SOCKADDR, addrlen : *mut i32) -> SOCKET);
    unsafe { accept(s, addr.unwrap_or(core::mem::zeroed()) as _, addrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn bind(s: SOCKET, name: *const super::SOCKADDR, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn bind(s : SOCKET, name : *const super::SOCKADDR, namelen : i32) -> i32);
    unsafe { bind(s, name, namelen) }
}
#[inline]
pub unsafe fn closesocket(s: SOCKET) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn closesocket(s : SOCKET) -> i32);
    unsafe { closesocket(s) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn connect(s: SOCKET, name: *const super::SOCKADDR, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn connect(s : SOCKET, name : *const super::SOCKADDR, namelen : i32) -> i32);
    unsafe { connect(s, name, namelen) }
}
#[inline]
pub unsafe fn gethostbyaddr(addr: &[u8], r#type: i32) -> *mut hostent {
    windows_core::link!("ws2_32.dll" "system" fn gethostbyaddr(addr : *const i8, len : i32, r#type : i32) -> *mut hostent);
    unsafe { gethostbyaddr(core::mem::transmute(addr.as_ptr()), addr.len().try_into().unwrap(), r#type) }
}
#[inline]
pub unsafe fn gethostbyname(name: *const i8) -> *mut hostent {
    windows_core::link!("ws2_32.dll" "system" fn gethostbyname(name : *const i8) -> *mut hostent);
    unsafe { gethostbyname(name) }
}
#[inline]
pub unsafe fn gethostname(name: *mut i8, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn gethostname(name : *mut i8, namelen : i32) -> i32);
    unsafe { gethostname(name as _, namelen) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn getpeername(s: SOCKET, name: *mut super::SOCKADDR, namelen: *mut i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getpeername(s : SOCKET, name : *mut super::SOCKADDR, namelen : *mut i32) -> i32);
    unsafe { getpeername(s, name as _, namelen as _) }
}
#[inline]
pub unsafe fn getprotobyname(name: *const i8) -> *mut protoent {
    windows_core::link!("ws2_32.dll" "system" fn getprotobyname(name : *const i8) -> *mut protoent);
    unsafe { getprotobyname(name) }
}
#[inline]
pub unsafe fn getprotobynumber(number: i32) -> *mut protoent {
    windows_core::link!("ws2_32.dll" "system" fn getprotobynumber(number : i32) -> *mut protoent);
    unsafe { getprotobynumber(number) }
}
#[inline]
pub unsafe fn getservbyname(name: *const i8, proto: Option<*const i8>) -> *mut servent {
    windows_core::link!("ws2_32.dll" "system" fn getservbyname(name : *const i8, proto : *const i8) -> *mut servent);
    unsafe { getservbyname(name, proto.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn getservbyport(port: i32, proto: Option<*const i8>) -> *mut servent {
    windows_core::link!("ws2_32.dll" "system" fn getservbyport(port : i32, proto : *const i8) -> *mut servent);
    unsafe { getservbyport(port, proto.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn getsockname(s: SOCKET, name: *mut super::SOCKADDR, namelen: *mut i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getsockname(s : SOCKET, name : *mut super::SOCKADDR, namelen : *mut i32) -> i32);
    unsafe { getsockname(s, name as _, namelen as _) }
}
#[inline]
pub unsafe fn getsockopt(s: SOCKET, level: i32, optname: i32, optval: *mut i8, optlen: *mut i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getsockopt(s : SOCKET, level : i32, optname : i32, optval : *mut i8, optlen : *mut i32) -> i32);
    unsafe { getsockopt(s, level, optname, optval as _, optlen as _) }
}
#[inline]
pub unsafe fn htonl(hostlong: u_long) -> u_long {
    windows_core::link!("ws2_32.dll" "system" fn htonl(hostlong : u_long) -> u_long);
    unsafe { htonl(hostlong) }
}
#[inline]
pub unsafe fn htons(hostshort: u_short) -> u_short {
    windows_core::link!("ws2_32.dll" "system" fn htons(hostshort : u_short) -> u_short);
    unsafe { htons(hostshort) }
}
#[inline]
pub unsafe fn inet_addr(cp: *const i8) -> u32 {
    windows_core::link!("ws2_32.dll" "system" fn inet_addr(cp : *const i8) -> u32);
    unsafe { inet_addr(cp) }
}
#[cfg(all(feature = "inaddr", feature = "shared"))]
#[inline]
pub unsafe fn inet_ntoa(r#in: super::IN_ADDR) -> *mut i8 {
    windows_core::link!("ws2_32.dll" "system" fn inet_ntoa(r#in : super::IN_ADDR) -> *mut i8);
    unsafe { inet_ntoa(r#in) }
}
#[inline]
pub unsafe fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u_long) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn ioctlsocket(s : SOCKET, cmd : i32, argp : *mut u_long) -> i32);
    unsafe { ioctlsocket(s, cmd, argp as _) }
}
#[inline]
pub unsafe fn listen(s: SOCKET, backlog: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn listen(s : SOCKET, backlog : i32) -> i32);
    unsafe { listen(s, backlog) }
}
#[inline]
pub unsafe fn ntohl(netlong: u_long) -> u_long {
    windows_core::link!("ws2_32.dll" "system" fn ntohl(netlong : u_long) -> u_long);
    unsafe { ntohl(netlong) }
}
#[inline]
pub unsafe fn ntohs(netshort: u_short) -> u_short {
    windows_core::link!("ws2_32.dll" "system" fn ntohs(netshort : u_short) -> u_short);
    unsafe { ntohs(netshort) }
}
#[inline]
pub unsafe fn recv(s: SOCKET, buf: *mut i8, len: i32, flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn recv(s : SOCKET, buf : *mut i8, len : i32, flags : i32) -> i32);
    unsafe { recv(s, buf as _, len, flags) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn recvfrom(s: SOCKET, buf: *mut i8, len: i32, flags: i32, from: Option<*mut super::SOCKADDR>, fromlen: Option<*mut i32>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn recvfrom(s : SOCKET, buf : *mut i8, len : i32, flags : i32, from : *mut super::SOCKADDR, fromlen : *mut i32) -> i32);
    unsafe { recvfrom(s, buf as _, len, flags, from.unwrap_or(core::mem::zeroed()) as _, fromlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn select(nfds: i32, readfds: Option<*mut fd_set>, writefds: Option<*mut fd_set>, exceptfds: Option<*mut fd_set>, timeout: Option<*const timeval>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn select(nfds : i32, readfds : *mut fd_set, writefds : *mut fd_set, exceptfds : *mut fd_set, timeout : *const timeval) -> i32);
    unsafe { select(nfds, readfds.unwrap_or(core::mem::zeroed()) as _, writefds.unwrap_or(core::mem::zeroed()) as _, exceptfds.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn send(s: SOCKET, buf: &[u8], flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn send(s : SOCKET, buf : *const i8, len : i32, flags : i32) -> i32);
    unsafe { send(s, core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), flags) }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
#[inline]
pub unsafe fn sendto(s: SOCKET, buf: &[u8], flags: i32, to: *const super::SOCKADDR, tolen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn sendto(s : SOCKET, buf : *const i8, len : i32, flags : i32, to : *const super::SOCKADDR, tolen : i32) -> i32);
    unsafe { sendto(s, core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), flags, to, tolen) }
}
#[inline]
pub unsafe fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: Option<*const i8>, optlen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn setsockopt(s : SOCKET, level : i32, optname : i32, optval : *const i8, optlen : i32) -> i32);
    unsafe { setsockopt(s, level, optname, optval.unwrap_or(core::mem::zeroed()) as _, optlen) }
}
#[inline]
pub unsafe fn shutdown(s: SOCKET, how: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn shutdown(s : SOCKET, how : i32) -> i32);
    unsafe { shutdown(s, how) }
}
#[inline]
pub unsafe fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn socket(af : i32, r#type : i32, protocol : i32) -> SOCKET);
    unsafe { socket(af, r#type, protocol) }
}
#[cfg(feature = "shared")]
pub const ADDR_ANY: super::ULONG = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AFPROTOCOLS {
    pub iAddressFamily: super::INT,
    pub iProtocol: super::INT,
}
pub const BASE_PROTOCOL: i32 = 1;
pub const BIGENDIAN: i32 = 0;
pub const CF_ACCEPT: i32 = 0;
pub const CF_DEFER: i32 = 2;
pub const CF_REJECT: i32 = 1;
pub const COMP_EQUAL: WSAECOMPARATOR = 0;
pub const COMP_NOTLESS: WSAECOMPARATOR = 1;
pub const FD_ACCEPT: i32 = 8;
pub const FD_ACCEPT_BIT: i32 = 3;
pub const FD_ADDRESS_LIST_CHANGE: i32 = 512;
pub const FD_ADDRESS_LIST_CHANGE_BIT: i32 = 9;
pub const FD_ALL_EVENTS: i32 = 1023;
pub const FD_CLOSE: i32 = 32;
pub const FD_CLOSE_BIT: i32 = 5;
pub const FD_CONNECT: i32 = 16;
pub const FD_CONNECT_BIT: i32 = 4;
pub const FD_GROUP_QOS: i32 = 128;
pub const FD_GROUP_QOS_BIT: i32 = 7;
pub const FD_MAX_EVENTS: i32 = 10;
pub const FD_OOB: i32 = 4;
pub const FD_OOB_BIT: i32 = 2;
pub const FD_QOS: i32 = 64;
pub const FD_QOS_BIT: i32 = 6;
pub const FD_READ: i32 = 1;
pub const FD_READ_BIT: i32 = 0;
pub const FD_ROUTING_INTERFACE_CHANGE: i32 = 256;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: i32 = 8;
pub type FD_SET = fd_set;
pub const FD_SETSIZE: i32 = 64;
pub const FD_WRITE: i32 = 2;
pub const FD_WRITE_BIT: i32 = 1;
pub const FIOASYNC: u32 = 2147772029;
pub const FIONBIO: u32 = 2147772030;
pub const FIONREAD: i32 = 1074030207;
pub const FROM_PROTOCOL_INFO: i32 = -1;
pub type GROUP = u32;
pub type HOSTENT = hostent;
pub const HOST_NOT_FOUND: i32 = 11001;
pub const IMPLINK_HIGHEXPER: i32 = 158;
pub const IMPLINK_IP: i32 = 155;
pub const IMPLINK_LOWEXPER: i32 = 156;
pub const INCL_WINSOCK_API_PROTOTYPES: i32 = 1;
pub const INCL_WINSOCK_API_TYPEDEFS: i32 = 0;
#[cfg(target_arch = "x86")]
pub const INVALID_SOCKET: SOCKET = 4294967295u64 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INVALID_SOCKET: SOCKET = 18446744073709551615u64 as _;
pub const IOCPARM_MASK: i32 = 127;
pub const IOC_IN: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOC_OUT: i32 = 1073741824;
pub const IOC_VOID: i32 = 536870912;
pub const IPPORT_BIFFUDP: i32 = 512;
pub const IPPORT_CMDSERVER: i32 = 514;
pub const IPPORT_DAYTIME: i32 = 13;
pub const IPPORT_DISCARD: i32 = 9;
pub const IPPORT_ECHO: i32 = 7;
pub const IPPORT_EFSSERVER: i32 = 520;
pub const IPPORT_EXECSERVER: i32 = 512;
pub const IPPORT_FINGER: i32 = 79;
pub const IPPORT_FTP: i32 = 21;
pub const IPPORT_LOGINSERVER: i32 = 513;
pub const IPPORT_MTP: i32 = 57;
pub const IPPORT_NAMESERVER: i32 = 42;
pub const IPPORT_NETSTAT: i32 = 15;
pub const IPPORT_RESERVED: i32 = 1024;
pub const IPPORT_RJE: i32 = 77;
pub const IPPORT_ROUTESERVER: i32 = 520;
pub const IPPORT_SMTP: i32 = 25;
pub const IPPORT_SUPDUP: i32 = 95;
pub const IPPORT_SYSTAT: i32 = 11;
pub const IPPORT_TELNET: i32 = 23;
pub const IPPORT_TFTP: i32 = 69;
pub const IPPORT_TIMESERVER: i32 = 37;
pub const IPPORT_TTYLINK: i32 = 87;
pub const IPPORT_WHOIS: i32 = 43;
pub const IPPORT_WHOSERVER: i32 = 513;
pub const JL_BOTH: i32 = 4;
pub const JL_RECEIVER_ONLY: i32 = 2;
pub const JL_SENDER_ONLY: i32 = 1;
pub const LAYERED_PROTOCOL: i32 = 0;
pub type LINGER = linger;
pub const LITTLEENDIAN: i32 = 1;
#[cfg(feature = "shared")]
pub type LPAFPROTOCOLS = *mut AFPROTOCOLS;
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
pub type LPCONDITIONPROC = Option<unsafe extern "system" fn(lpcallerid: super::LPWSABUF, lpcallerdata: super::LPWSABUF, lpsqos: LPQOS, lpgqos: LPQOS, lpcalleeid: super::LPWSABUF, lpcalleedata: super::LPWSABUF, g: *mut GROUP, dwcallbackdata: super::DWORD_PTR) -> i32>;
pub type LPFD_SET = *mut fd_set;
pub type LPHOSTENT = *mut hostent;
pub type LPLINGER = *mut linger;
pub type LPPROTOENT = *mut protoent;
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
pub type LPQOS = *mut QOS;
pub type LPSERVENT = *mut servent;
#[cfg(all(feature = "inaddr", feature = "shared", feature = "ws2"))]
pub type LPSOCKADDR_IN = *mut super::SOCKADDR_IN;
pub type LPTIMEVAL = *mut timeval;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type LPWSACOMPLETION = *mut WSACOMPLETION;
pub type LPWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
#[cfg(feature = "shared")]
pub type LPWSADATA = *mut WSADATA;
pub type LPWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type LPWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
#[cfg(feature = "shared")]
pub type LPWSANAMESPACE_INFO = *mut WSANAMESPACE_INFOA;
#[cfg(feature = "shared")]
pub type LPWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPWSANAMESPACE_INFOEX = *mut WSANAMESPACE_INFOEXA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type LPWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
#[cfg(feature = "shared")]
pub type LPWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
pub type LPWSANETWORKEVENTS = *mut WSANETWORKEVENTS;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPWSANSCLASSINFO = *mut WSANSCLASSINFOA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type LPWSAOVERLAPPED = *mut super::OVERLAPPED;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: super::DWORD, cbtransferred: super::DWORD, lpoverlapped: LPWSAOVERLAPPED, dwflags: super::DWORD)>;
#[cfg(feature = "shared")]
pub type LPWSAPOLLFD = *mut WSAPOLLFD;
#[cfg(feature = "shared")]
pub type LPWSAPROTOCOLCHAIN = *mut WSAPROTOCOLCHAIN;
#[cfg(feature = "shared")]
pub type LPWSAPROTOCOL_INFO = *mut WSAPROTOCOL_INFOA;
#[cfg(feature = "shared")]
pub type LPWSAPROTOCOL_INFOA = *mut WSAPROTOCOL_INFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPWSAPROTOCOL_INFOW = *mut WSAPROTOCOL_INFOW;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2 = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2A = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2W = *mut WSAQUERYSET2W;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSETA = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSETW = *mut WSAQUERYSETW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type LPWSASERVICECLASSINFO = *mut WSASERVICECLASSINFOA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type LPWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type LPWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
#[cfg(feature = "shared")]
pub type LPWSAVERSION = *mut WSAVERSION;
pub const LUP_ADDRCONFIG: i32 = 1048576;
pub const LUP_API_ANSI: i32 = 16777216;
pub const LUP_CONTAINERS: i32 = 2;
pub const LUP_DEEP: i32 = 1;
pub const LUP_DISABLE_IDN_ENCODING: i32 = 8388608;
pub const LUP_DNS_ONLY: i32 = 131072;
pub const LUP_DUAL_ADDR: i32 = 2097152;
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: i32 = 134217728;
pub const LUP_EXTENDED_QUERYSET: i32 = 33554432;
pub const LUP_FILESERVER: i32 = 4194304;
pub const LUP_FLUSHCACHE: i32 = 4096;
pub const LUP_FLUSHPREVIOUS: i32 = 8192;
pub const LUP_FORCE_CLEAR_TEXT: i32 = 1073741824;
pub const LUP_NEAREST: i32 = 8;
pub const LUP_NOCONTAINERS: i32 = 4;
pub const LUP_NON_AUTHORITATIVE: i32 = 16384;
pub const LUP_REQUIRE_SECURE: i32 = 268435456;
pub const LUP_RESERVED_UNUSED: i32 = 524288;
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648;
pub const LUP_RES_SERVICE: i32 = 32768;
pub const LUP_RETURN_ADDR: i32 = 256;
pub const LUP_RETURN_ALIASES: i32 = 1024;
pub const LUP_RETURN_ALL: i32 = 4080;
pub const LUP_RETURN_BLOB: i32 = 512;
pub const LUP_RETURN_COMMENT: i32 = 128;
pub const LUP_RETURN_NAME: i32 = 16;
pub const LUP_RETURN_PREFERRED_NAMES: i32 = 65536;
pub const LUP_RETURN_QUERY_STRING: i32 = 2048;
pub const LUP_RETURN_RESPONSE_FLAGS: i32 = 262144;
pub const LUP_RETURN_TTL: i32 = 536870912;
pub const LUP_RETURN_TYPE: i32 = 32;
pub const LUP_RETURN_VERSION: i32 = 64;
pub const LUP_SECURE: i32 = 32768;
pub const LUP_SECURE_WITH_FALLBACK: i32 = 67108864;
pub const MAXGETHOSTSTRUCT: i32 = 1024;
pub const MAX_PROTOCOL_CHAIN: i32 = 7;
pub const MSG_DONTROUTE: i32 = 4;
pub const MSG_INTERRUPT: i32 = 16;
pub const MSG_MAXIOVLEN: i32 = 16;
pub const MSG_OOB: i32 = 1;
pub const MSG_PARTIAL: i32 = 32768;
pub const MSG_PEEK: i32 = 2;
pub const MSG_PUSH_IMMEDIATE: i32 = 32;
pub const MSG_WAITALL: i32 = 8;
pub const NO_ADDRESS: i32 = 11004;
pub const NO_DATA: i32 = 11004;
pub const NO_RECOVERY: i32 = 11003;
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = 4;
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = 2;
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = 1;
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = 0;
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = 3;
pub const NS_ALL: i32 = 0;
pub const NS_BTH: i32 = 16;
pub const NS_DHCP: i32 = 6;
pub const NS_DNS: i32 = 12;
pub const NS_EMAIL: i32 = 37;
pub const NS_LOCALNAME: i32 = 19;
pub const NS_MS: i32 = 30;
pub const NS_NBP: i32 = 20;
pub const NS_NDS: i32 = 2;
pub const NS_NETBT: i32 = 13;
pub const NS_NETDES: i32 = 60;
pub const NS_NIS: i32 = 41;
pub const NS_NISPLUS: i32 = 42;
pub const NS_NLA: i32 = 15;
pub const NS_NTDS: i32 = 32;
pub const NS_PEER_BROWSE: i32 = 3;
pub const NS_PNRPCLOUD: i32 = 39;
pub const NS_PNRPNAME: i32 = 38;
pub const NS_SAP: i32 = 1;
pub const NS_SLP: i32 = 5;
pub const NS_STDA: i32 = 31;
pub const NS_TCPIP_HOSTS: i32 = 11;
pub const NS_TCPIP_LOCAL: i32 = 10;
pub const NS_WINS: i32 = 14;
pub const NS_WRQ: i32 = 50;
pub const NS_X500: i32 = 40;
#[cfg(feature = "shared")]
pub type PAFPROTOCOLS = *mut AFPROTOCOLS;
pub type PFD_SET = *mut fd_set;
pub const PFL_HIDDEN: i32 = 4;
pub const PFL_MATCHES_PROTOCOL_ZERO: i32 = 8;
pub const PFL_MULTIPLE_PROTO_ENTRIES: i32 = 1;
pub const PFL_NETWORKDIRECT_PROVIDER: i32 = 16;
pub const PFL_RECOMMENDED_PROTO_ENTRY: i32 = 2;
pub const PF_APPLETALK: i32 = 16;
pub const PF_ATM: i32 = 22;
pub const PF_BAN: i32 = 21;
pub const PF_BTH: i32 = 32;
pub const PF_CCITT: i32 = 10;
pub const PF_CHAOS: i32 = 5;
pub const PF_DATAKIT: i32 = 9;
pub const PF_DECnet: i32 = 12;
pub const PF_DLI: i32 = 13;
pub const PF_ECMA: i32 = 8;
pub const PF_FIREFOX: i32 = 19;
pub const PF_HYLINK: i32 = 15;
pub const PF_IMPLINK: i32 = 3;
pub const PF_INET: i32 = 2;
pub const PF_INET6: i32 = 23;
pub const PF_IPX: i32 = 6;
pub const PF_ISO: i32 = 7;
pub const PF_LAT: i32 = 14;
pub const PF_MAX: i32 = 35;
pub const PF_NS: i32 = 6;
pub const PF_OSI: i32 = 7;
pub const PF_PUP: i32 = 4;
pub const PF_SNA: i32 = 11;
pub const PF_UNIX: i32 = 1;
pub const PF_UNKNOWN1: i32 = 20;
pub const PF_UNSPEC: i32 = 0;
pub const PF_VOICEVIEW: i32 = 18;
pub type PHOSTENT = *mut hostent;
pub type PLINGER = *mut linger;
pub const POLLERR: i32 = 1;
pub const POLLHUP: i32 = 2;
pub const POLLIN: i32 = 768;
pub const POLLNVAL: i32 = 4;
pub const POLLOUT: i32 = 16;
pub const POLLPRI: i32 = 1024;
pub const POLLRDBAND: i32 = 512;
pub const POLLRDNORM: i32 = 256;
pub const POLLWRBAND: i32 = 32;
pub const POLLWRNORM: i32 = 16;
pub type PPROTOENT = *mut protoent;
pub type PROTOENT = protoent;
pub type PSERVENT = *mut servent;
pub type PTIMEVAL = *mut timeval;
pub const PVD_CONFIG: i32 = 12289;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
pub type PWSACOMPLETION = *mut WSACOMPLETION;
pub type PWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
pub type PWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type PWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
#[cfg(feature = "shared")]
pub type PWSANAMESPACE_INFO = *mut WSANAMESPACE_INFOA;
#[cfg(feature = "shared")]
pub type PWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type PWSANAMESPACE_INFOEX = *mut WSANAMESPACE_INFOEXA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type PWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type PWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
#[cfg(feature = "shared")]
pub type PWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWSANSCLASSINFO = *mut WSANSCLASSINFOA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
#[cfg(feature = "shared")]
pub type PWSAPOLLFD = *mut WSAPOLLFD;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2 = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2A = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2W = *mut WSAQUERYSET2W;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSETA = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSETW = *mut WSAQUERYSETW;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type PWSASERVICECLASSINFO = *mut WSASERVICECLASSINFOA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type PWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type PWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
#[cfg(feature = "shared")]
pub type PWSAVERSION = *mut WSAVERSION;
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS {
    pub SendingFlowspec: super::FLOWSPEC,
    pub ReceivingFlowspec: super::FLOWSPEC,
    pub ProviderSpecific: super::WSABUF,
}
pub const RESULT_IS_ADDED: i32 = 16;
pub const RESULT_IS_ALIAS: i32 = 1;
pub const RESULT_IS_CHANGED: i32 = 32;
pub const RESULT_IS_DELETED: i32 = 64;
pub const RES_FLUSH_CACHE: i32 = 2;
pub const RES_SERVICE: i32 = 4;
pub const RES_UNUSED_1: i32 = 1;
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = 2;
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = 1;
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = 0;
pub const SD_BOTH: i32 = 2;
pub const SD_RECEIVE: i32 = 0;
pub const SD_SEND: i32 = 1;
pub const SECURITY_PROTOCOL_NONE: i32 = 0;
pub type SERVENT = servent;
pub const SERVICE_MULTIPLE: i32 = 1;
pub const SERVICE_TYPE_VALUE_IPXPORTA: windows_core::PCSTR = windows_core::s!("IpxSocket");
pub const SERVICE_TYPE_VALUE_IPXPORTW: windows_core::PCWSTR = windows_core::w!("IpxSocket");
pub const SERVICE_TYPE_VALUE_OBJECTID: windows_core::PCSTR = windows_core::s!("ObjectId");
pub const SERVICE_TYPE_VALUE_OBJECTIDA: windows_core::PCSTR = windows_core::s!("ObjectId");
pub const SERVICE_TYPE_VALUE_OBJECTIDW: windows_core::PCWSTR = windows_core::w!("ObjectId");
pub const SERVICE_TYPE_VALUE_SAPID: windows_core::PCSTR = windows_core::s!("SapId");
pub const SERVICE_TYPE_VALUE_SAPIDA: windows_core::PCSTR = windows_core::s!("SapId");
pub const SERVICE_TYPE_VALUE_SAPIDW: windows_core::PCWSTR = windows_core::w!("SapId");
pub const SERVICE_TYPE_VALUE_TCPPORT: windows_core::PCSTR = windows_core::s!("TcpPort");
pub const SERVICE_TYPE_VALUE_TCPPORTA: windows_core::PCSTR = windows_core::s!("TcpPort");
pub const SERVICE_TYPE_VALUE_TCPPORTW: windows_core::PCWSTR = windows_core::w!("TcpPort");
pub const SERVICE_TYPE_VALUE_UDPPORT: windows_core::PCSTR = windows_core::s!("UdpPort");
pub const SERVICE_TYPE_VALUE_UDPPORTA: windows_core::PCSTR = windows_core::s!("UdpPort");
pub const SERVICE_TYPE_VALUE_UDPPORTW: windows_core::PCWSTR = windows_core::w!("UdpPort");
pub const SG_CONSTRAINED_GROUP: i32 = 2;
pub const SG_UNCONSTRAINED_GROUP: i32 = 1;
pub const SIOCATMARK: i32 = 1074033415;
pub const SIOCGHIWAT: i32 = 1074033409;
pub const SIOCGLOWAT: i32 = 1074033411;
pub const SIOCSHIWAT: u32 = 2147775232;
pub const SIOCSLOWAT: u32 = 2147775234;
pub const SIO_NSP_NOTIFY_CHANGE: u32 = 2281701401;
#[cfg(target_arch = "x86")]
pub type SOCKET = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SOCKET = u64;
pub const SOCKET_ERROR: i32 = -1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_NOTIFY_EVENTS_ALL: i32 = 199;
pub const SOCK_NOTIFY_EVENT_ERR: i32 = 64;
pub const SOCK_NOTIFY_EVENT_HANGUP: i32 = 4;
pub const SOCK_NOTIFY_EVENT_IN: i32 = 1;
pub const SOCK_NOTIFY_EVENT_OUT: i32 = 2;
pub const SOCK_NOTIFY_EVENT_REMOVE: i32 = 128;
pub const SOCK_NOTIFY_OP_DISABLE: i32 = 2;
pub const SOCK_NOTIFY_OP_ENABLE: i32 = 1;
pub const SOCK_NOTIFY_OP_NONE: i32 = 0;
pub const SOCK_NOTIFY_OP_REMOVE: i32 = 4;
pub const SOCK_NOTIFY_REGISTER_EVENTS_ALL: i32 = 7;
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: i32 = 4;
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: i32 = 1;
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: i32 = 0;
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCK_NOTIFY_REGISTRATION {
    pub socket: SOCKET,
    pub completionKey: super::PVOID,
    pub eventFilter: super::UINT16,
    pub operation: super::UINT8,
    pub triggerFlags: super::UINT8,
    pub registrationResult: super::DWORD,
}
pub const SOCK_NOTIFY_TRIGGER_ALL: i32 = 15;
pub const SOCK_NOTIFY_TRIGGER_EDGE: i32 = 8;
pub const SOCK_NOTIFY_TRIGGER_LEVEL: i32 = 4;
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: i32 = 1;
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_RDM: i32 = 4;
pub const SOCK_SEQPACKET: i32 = 5;
pub const SOCK_STREAM: i32 = 1;
pub const SOL_SOCKET: i32 = 65535;
pub const SOMAXCONN: i32 = 2147483647;
pub const SO_ACCEPTCONN: i32 = 2;
pub const SO_BROADCAST: i32 = 32;
pub const SO_CONDITIONAL_ACCEPT: i32 = 12290;
pub const SO_DEBUG: i32 = 1;
pub const SO_DONTLINGER: i32 = -129;
pub const SO_DONTROUTE: i32 = 16;
pub const SO_ERROR: i32 = 4103;
pub const SO_EXCLUSIVEADDRUSE: i32 = -5;
pub const SO_GROUP_ID: i32 = 8193;
pub const SO_GROUP_PRIORITY: i32 = 8194;
pub const SO_KEEPALIVE: i32 = 8;
pub const SO_LINGER: i32 = 128;
pub const SO_MAX_MSG_SIZE: i32 = 8195;
pub const SO_OOBINLINE: i32 = 256;
pub const SO_PROTOCOL_INFO: i32 = 8196;
pub const SO_PROTOCOL_INFOA: i32 = 8196;
pub const SO_PROTOCOL_INFOW: i32 = 8197;
pub const SO_RCVBUF: i32 = 4098;
pub const SO_RCVLOWAT: i32 = 4100;
pub const SO_RCVTIMEO: i32 = 4102;
pub const SO_REUSEADDR: i32 = 4;
pub const SO_SNDBUF: i32 = 4097;
pub const SO_SNDLOWAT: i32 = 4099;
pub const SO_SNDTIMEO: i32 = 4101;
pub const SO_TYPE: i32 = 4104;
pub const SO_USELOOPBACK: i32 = 64;
pub const TH_NETDEV: i32 = 1;
pub const TH_TAPI: i32 = 2;
pub type TIMEVAL = timeval;
pub const TRY_AGAIN: i32 = 11002;
#[cfg(feature = "shared")]
pub const WINSOCK_VERSION: super::WORD = 514;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    pub Parameters: WSACOMPLETION_0,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for WSACOMPLETION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WSACOMPLETION_0 {
    pub WindowMessage: WSACOMPLETION_0_0,
    pub Event: WSACOMPLETION_0_1,
    pub Apc: WSACOMPLETION_0_2,
    pub Port: WSACOMPLETION_0_3,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
impl Default for WSACOMPLETION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_0 {
    pub hWnd: super::HWND,
    pub uMsg: super::UINT,
    pub context: super::WPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_1 {
    pub lpOverlapped: LPWSAOVERLAPPED,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WSACOMPLETION_0_2 {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_3 {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub hPort: super::HANDLE,
    pub Key: super::ULONG_PTR,
}
pub type WSACOMPLETIONTYPE = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSADATA {
    pub wVersion: super::WORD,
    pub wHighVersion: super::WORD,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut i8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for WSADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSADATA {
    pub wVersion: super::WORD,
    pub wHighVersion: super::WORD,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut i8,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for WSADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSADESCRIPTION_LEN: i32 = 256;
pub type WSAECOMPARATOR = i32;
pub type WSAESETSERVICEOP = i32;
#[cfg(feature = "shared")]
pub type WSANAMESPACE_INFO = WSANAMESPACE_INFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: super::DWORD,
    pub fActive: windows_core::BOOL,
    pub dwVersion: super::DWORD,
    pub lpszIdentifier: windows_core::PSTR,
}
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
pub type WSANAMESPACE_INFOEX = WSANAMESPACE_INFOEXA;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: super::DWORD,
    pub fActive: windows_core::BOOL,
    pub dwVersion: super::DWORD,
    pub lpszIdentifier: windows_core::PSTR,
    pub ProviderSpecific: super::BLOB,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: super::DWORD,
    pub fActive: windows_core::BOOL,
    pub dwVersion: super::DWORD,
    pub lpszIdentifier: windows_core::PWSTR,
    pub ProviderSpecific: super::BLOB,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: super::DWORD,
    pub fActive: windows_core::BOOL,
    pub dwVersion: super::DWORD,
    pub lpszIdentifier: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSANETWORKEVENTS {
    pub lNetworkEvents: i32,
    pub iErrorCode: [i32; 10],
}
impl Default for WSANETWORKEVENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSANO_ADDRESS: i32 = 11004;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type WSANSCLASSINFO = WSANSCLASSINFOA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANSCLASSINFOA {
    pub lpszName: windows_core::PSTR,
    pub dwNameSpace: super::DWORD,
    pub dwValueType: super::DWORD,
    pub dwValueSize: super::DWORD,
    pub lpValue: super::LPVOID,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANSCLASSINFOW {
    pub lpszName: windows_core::PWSTR,
    pub dwNameSpace: super::DWORD,
    pub dwValueType: super::DWORD,
    pub dwValueSize: super::DWORD,
    pub lpValue: super::LPVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAPOLLFD {
    pub fd: SOCKET,
    pub events: super::SHORT,
    pub revents: super::SHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: i32,
    pub ChainEntries: [super::DWORD; 7],
}
#[cfg(feature = "shared")]
impl Default for WSAPROTOCOLCHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type WSAPROTOCOL_INFO = WSAPROTOCOL_INFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: super::DWORD,
    pub dwServiceFlags2: super::DWORD,
    pub dwServiceFlags3: super::DWORD,
    pub dwServiceFlags4: super::DWORD,
    pub dwProviderFlags: super::DWORD,
    pub ProviderId: windows_core::GUID,
    pub dwCatalogEntryId: super::DWORD,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: super::DWORD,
    pub dwProviderReserved: super::DWORD,
    pub szProtocol: [super::CHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for WSAPROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: super::DWORD,
    pub dwServiceFlags2: super::DWORD,
    pub dwServiceFlags3: super::DWORD,
    pub dwServiceFlags4: super::DWORD,
    pub dwProviderFlags: super::DWORD,
    pub ProviderId: windows_core::GUID,
    pub dwCatalogEntryId: super::DWORD,
    pub ProtocolChain: WSAPROTOCOLCHAIN,
    pub iVersion: i32,
    pub iAddressFamily: i32,
    pub iMaxSockAddr: i32,
    pub iMinSockAddr: i32,
    pub iSocketType: i32,
    pub iProtocol: i32,
    pub iProtocolMaxOffset: i32,
    pub iNetworkByteOrder: i32,
    pub iSecurityScheme: i32,
    pub dwMessageSize: super::DWORD,
    pub dwProviderReserved: super::DWORD,
    pub szProtocol: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WSAPROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSAPROTOCOL_LEN: i32 = 255;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type WSAQUERYSET = WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
pub type WSAQUERYSET2 = WSAQUERYSET2A;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSET2A {
    pub dwSize: super::DWORD,
    pub lpszServiceInstanceName: windows_core::PSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PSTR,
    pub dwNameSpace: super::DWORD,
    pub lpNSProviderId: super::LPGUID,
    pub lpszContext: windows_core::PSTR,
    pub dwNumberOfProtocols: super::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PSTR,
    pub dwNumberOfCsAddrs: super::DWORD,
    pub lpcsaBuffer: super::LPCSADDR_INFO,
    pub dwOutputFlags: super::DWORD,
    pub lpBlob: super::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSET2W {
    pub dwSize: super::DWORD,
    pub lpszServiceInstanceName: windows_core::PWSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PWSTR,
    pub dwNameSpace: super::DWORD,
    pub lpNSProviderId: super::LPGUID,
    pub lpszContext: windows_core::PWSTR,
    pub dwNumberOfProtocols: super::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PWSTR,
    pub dwNumberOfCsAddrs: super::DWORD,
    pub lpcsaBuffer: super::LPCSADDR_INFO,
    pub dwOutputFlags: super::DWORD,
    pub lpBlob: super::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSETA {
    pub dwSize: super::DWORD,
    pub lpszServiceInstanceName: windows_core::PSTR,
    pub lpServiceClassId: super::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PSTR,
    pub dwNameSpace: super::DWORD,
    pub lpNSProviderId: super::LPGUID,
    pub lpszContext: windows_core::PSTR,
    pub dwNumberOfProtocols: super::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PSTR,
    pub dwNumberOfCsAddrs: super::DWORD,
    pub lpcsaBuffer: super::LPCSADDR_INFO,
    pub dwOutputFlags: super::DWORD,
    pub lpBlob: super::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSETW {
    pub dwSize: super::DWORD,
    pub lpszServiceInstanceName: windows_core::PWSTR,
    pub lpServiceClassId: super::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PWSTR,
    pub dwNameSpace: super::DWORD,
    pub lpNSProviderId: super::LPGUID,
    pub lpszContext: windows_core::PWSTR,
    pub dwNumberOfProtocols: super::DWORD,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PWSTR,
    pub dwNumberOfCsAddrs: super::DWORD,
    pub lpcsaBuffer: super::LPCSADDR_INFO,
    pub dwOutputFlags: super::DWORD,
    pub lpBlob: super::LPBLOB,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type WSASERVICECLASSINFO = WSASERVICECLASSINFOA;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: super::LPGUID,
    pub lpszServiceClassName: windows_core::PSTR,
    pub dwCount: super::DWORD,
    pub lpClassInfos: LPWSANSCLASSINFOA,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: super::LPGUID,
    pub lpszServiceClassName: windows_core::PWSTR,
    pub dwCount: super::DWORD,
    pub lpClassInfos: LPWSANSCLASSINFOW,
}
pub const WSASYS_STATUS_LEN: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAVERSION {
    pub dwVersion: super::DWORD,
    pub ecHow: WSAECOMPARATOR,
}
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: i32 = 64;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: i32 = 4;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: i32 = 2;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: i32 = 16;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: i32 = 8;
pub const WSA_FLAG_NO_HANDLE_INHERIT: i32 = 128;
pub const WSA_FLAG_OVERLAPPED: i32 = 1;
pub const WSA_FLAG_REGISTERED_IO: i32 = 256;
pub const WSA_INFINITE: u32 = 4294967295;
#[cfg(feature = "winnt")]
pub const WSA_INVALID_EVENT: super::HANDLE = 0 as _;
pub const WSA_INVALID_HANDLE: i32 = 6;
pub const WSA_INVALID_PARAMETER: i32 = 87;
pub const WSA_IO_INCOMPLETE: i32 = 996;
pub const WSA_IO_PENDING: i32 = 997;
pub const WSA_MAXIMUM_WAIT_EVENTS: i32 = 64;
pub const WSA_NOT_ENOUGH_MEMORY: i32 = 8;
pub const WSA_OPERATION_ABORTED: i32 = 995;
pub const WSA_WAIT_EVENT_0: windows_core::NTSTATUS = windows_core::NTSTATUS(0x0_u32 as _);
#[cfg(feature = "shared")]
pub const WSA_WAIT_FAILED: super::DWORD = 4294967295;
pub const WSA_WAIT_IO_COMPLETION: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0_u32 as _);
pub const WSA_WAIT_TIMEOUT: i32 = 258;
pub const XP1_CONNECTIONLESS: i32 = 1;
pub const XP1_CONNECT_DATA: i32 = 128;
pub const XP1_DISCONNECT_DATA: i32 = 256;
pub const XP1_EXPEDITED_DATA: i32 = 64;
pub const XP1_GRACEFUL_CLOSE: i32 = 32;
pub const XP1_GUARANTEED_DELIVERY: i32 = 2;
pub const XP1_GUARANTEED_ORDER: i32 = 4;
pub const XP1_IFS_HANDLES: i32 = 131072;
pub const XP1_INTERRUPT: i32 = 16384;
pub const XP1_MESSAGE_ORIENTED: i32 = 8;
pub const XP1_MULTIPOINT_CONTROL_PLANE: i32 = 2048;
pub const XP1_MULTIPOINT_DATA_PLANE: i32 = 4096;
pub const XP1_PARTIAL_MESSAGE: i32 = 262144;
pub const XP1_PSEUDO_STREAM: i32 = 16;
pub const XP1_QOS_SUPPORTED: i32 = 8192;
pub const XP1_SAN_SUPPORT_SDP: i32 = 524288;
pub const XP1_SUPPORT_BROADCAST: i32 = 512;
pub const XP1_SUPPORT_MULTIPOINT: i32 = 1024;
pub const XP1_UNI_RECV: i32 = 65536;
pub const XP1_UNI_SEND: i32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct fd_set {
    pub fd_count: u_int,
    pub fd_array: [SOCKET; 64],
}
impl Default for fd_set {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct hostent {
    pub h_name: *mut i8,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i16,
    pub h_length: i16,
    pub h_addr_list: *mut *mut i8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct linger {
    pub l_onoff: u_short,
    pub l_linger: u_short,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct netent {
    pub n_name: *mut i8,
    pub n_aliases: *mut *mut i8,
    pub n_addrtype: i16,
    pub n_net: u_long,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct protoent {
    pub p_name: *mut i8,
    pub p_aliases: *mut *mut i8,
    pub p_proto: i16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct servent {
    pub s_name: *mut i8,
    pub s_aliases: *mut *mut i8,
    pub s_port: i16,
    pub s_proto: *mut i8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct servent {
    pub s_name: *mut i8,
    pub s_aliases: *mut *mut i8,
    pub s_proto: *mut i8,
    pub s_port: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct sockproto {
    pub sp_family: u_short,
    pub sp_protocol: u_short,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
pub type u_char = u8;
pub type u_int = u32;
pub type u_int64 = u64;
pub type u_long = u32;
pub type u_short = u16;
