#[inline]
pub unsafe fn GetHostNameW(name: &mut [u16]) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetHostNameW(name : windows_core::PWSTR, namelen : i32) -> i32);
    unsafe { GetHostNameW(core::mem::transmute(name.as_mut_ptr()), name.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ProcessSocketNotifications(completionport: super::winnt::HANDLE, registrationinfos: Option<&mut [SOCK_NOTIFY_REGISTRATION]>, timeoutms: u32, completionportentries: Option<&mut [super::minwinbase::OVERLAPPED_ENTRY]>, receivedentrycount: Option<*mut u32>) -> u32 {
    windows_core::link!("ws2_32.dll" "system" fn ProcessSocketNotifications(completionport : super::winnt::HANDLE, registrationcount : u32, registrationinfos : *mut SOCK_NOTIFY_REGISTRATION, timeoutms : u32, completioncount : u32, completionportentries : *mut super::minwinbase::OVERLAPPED_ENTRY, receivedentrycount : *mut u32) -> u32);
    unsafe { ProcessSocketNotifications(completionport, registrationinfos.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), registrationinfos.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), timeoutms, completionportentries.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), completionportentries.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), receivedentrycount.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "qos", feature = "ws2"))]
#[inline]
pub unsafe fn WSAAccept(s: SOCKET, addr: Option<*mut super::ws2::SOCKADDR>, addrlen: Option<*mut i32>, lpfncondition: LPCONDITIONPROC, dwcallbackdata: Option<usize>) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSAAccept(s : SOCKET, addr : *mut super::ws2::SOCKADDR, addrlen : *mut i32, lpfncondition : LPCONDITIONPROC, dwcallbackdata : usize) -> SOCKET);
    unsafe { WSAAccept(s, addr.unwrap_or(core::mem::zeroed()) as _, addrlen.unwrap_or(core::mem::zeroed()) as _, lpfncondition, dwcallbackdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSAAddressToStringA(lpsaaddress: *const super::ws2::SOCKADDR, dwaddresslength: u32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOA>, lpszaddressstring: windows_core::PSTR, lpdwaddressstringlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAAddressToStringA(lpsaaddress : *const super::ws2::SOCKADDR, dwaddresslength : u32, lpprotocolinfo : *const WSAPROTOCOL_INFOA, lpszaddressstring : windows_core::PSTR, lpdwaddressstringlength : *mut u32) -> i32);
    unsafe { WSAAddressToStringA(lpsaaddress, dwaddresslength, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpszaddressstring, lpdwaddressstringlength as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSAAddressToStringW(lpsaaddress: *const super::ws2::SOCKADDR, dwaddresslength: u32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOW>, lpszaddressstring: windows_core::PWSTR, lpdwaddressstringlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAAddressToStringW(lpsaaddress : *const super::ws2::SOCKADDR, dwaddresslength : u32, lpprotocolinfo : *const WSAPROTOCOL_INFOW, lpszaddressstring : windows_core::PWSTR, lpdwaddressstringlength : *mut u32) -> i32);
    unsafe { WSAAddressToStringW(lpsaaddress, dwaddresslength, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpszaddressstring, lpdwaddressstringlength as _) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetHostByAddr(hwnd: super::windef::HWND, wmsg: u_int, addr: &[u8], r#type: i32, buf: &mut [u8]) -> super::winnt::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetHostByAddr(hwnd : super::windef::HWND, wmsg : u_int, addr : *const i8, len : i32, r#type : i32, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetHostByAddr(hwnd, wmsg, core::mem::transmute(addr.as_ptr()), addr.len().try_into().unwrap(), r#type, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetHostByName<P2>(hwnd: super::windef::HWND, wmsg: u_int, name: P2, buf: &mut [u8]) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetHostByName(hwnd : super::windef::HWND, wmsg : u_int, name : windows_core::PCSTR, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetHostByName(hwnd, wmsg, name.param().abi(), core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetProtoByName<P2>(hwnd: super::windef::HWND, wmsg: u_int, name: P2, buf: &mut [u8]) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetProtoByName(hwnd : super::windef::HWND, wmsg : u_int, name : windows_core::PCSTR, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetProtoByName(hwnd, wmsg, name.param().abi(), core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetProtoByNumber(hwnd: super::windef::HWND, wmsg: u_int, number: i32, buf: &mut [u8]) -> super::winnt::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetProtoByNumber(hwnd : super::windef::HWND, wmsg : u_int, number : i32, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetProtoByNumber(hwnd, wmsg, number, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetServByName<P2, P3>(hwnd: super::windef::HWND, wmsg: u_int, name: P2, proto: P3, buf: &mut [u8]) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetServByName(hwnd : super::windef::HWND, wmsg : u_int, name : windows_core::PCSTR, proto : windows_core::PCSTR, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetServByName(hwnd, wmsg, name.param().abi(), proto.param().abi(), core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSAAsyncGetServByPort(hwnd: super::windef::HWND, wmsg: u_int, port: i32, proto: *const i8, buf: &mut [u8]) -> super::winnt::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncGetServByPort(hwnd : super::windef::HWND, wmsg : u_int, port : i32, proto : *const i8, buf : *mut i8, buflen : i32) -> super::winnt::HANDLE);
    unsafe { WSAAsyncGetServByPort(hwnd, wmsg, port, proto, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn WSAAsyncSelect(s: SOCKET, hwnd: super::windef::HWND, wmsg: u_int, levent: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAAsyncSelect(s : SOCKET, hwnd : super::windef::HWND, wmsg : u_int, levent : i32) -> i32);
    unsafe { WSAAsyncSelect(s, hwnd, wmsg, levent) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSACancelAsyncRequest(hasynctaskhandle: super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSACancelAsyncRequest(hasynctaskhandle : super::winnt::HANDLE) -> i32);
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
pub unsafe fn WSACloseEvent(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSACloseEvent(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WSACloseEvent(hevent) }
}
#[cfg(all(feature = "qos", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnect(s: SOCKET, name: *const super::ws2::SOCKADDR, namelen: i32, lpcallerdata: Option<*const super::ws2::WSABUF>, lpcalleedata: Option<*mut super::ws2::WSABUF>, lpsqos: Option<*const QOS>, lpgqos: Option<*const QOS>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAConnect(s : SOCKET, name : *const super::ws2::SOCKADDR, namelen : i32, lpcallerdata : *const super::ws2::WSABUF, lpcalleedata : *mut super::ws2::WSABUF, lpsqos : *const QOS, lpgqos : *const QOS) -> i32);
    unsafe { WSAConnect(s, name, namelen, lpcallerdata.unwrap_or(core::mem::zeroed()) as _, lpcalleedata.unwrap_or(core::mem::zeroed()) as _, lpsqos.unwrap_or(core::mem::zeroed()) as _, lpgqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByList(s: SOCKET, socketaddress: *const super::ws2::SOCKET_ADDRESS_LIST, localaddresslength: Option<*mut u32>, localaddress: Option<*mut super::ws2::SOCKADDR>, remoteaddresslength: Option<*mut u32>, remoteaddress: Option<*mut super::ws2::SOCKADDR>, timeout: Option<*const timeval>, reserved: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByList(s : SOCKET, socketaddress : *const super::ws2::SOCKET_ADDRESS_LIST, localaddresslength : *mut u32, localaddress : *mut super::ws2::SOCKADDR, remoteaddresslength : *mut u32, remoteaddress : *mut super::ws2::SOCKADDR, timeout : *const timeval, reserved : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByList(s, socketaddress, localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByNameA<P1, P2>(s: SOCKET, nodename: P1, servicename: P2, localaddresslength: Option<*mut u32>, localaddress: Option<*mut super::ws2::SOCKADDR>, remoteaddresslength: Option<*mut u32>, remoteaddress: Option<*mut super::ws2::SOCKADDR>, timeout: Option<*const timeval>, reserved: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByNameA(s : SOCKET, nodename : windows_core::PCSTR, servicename : windows_core::PCSTR, localaddresslength : *mut u32, localaddress : *mut super::ws2::SOCKADDR, remoteaddresslength : *mut u32, remoteaddress : *mut super::ws2::SOCKADDR, timeout : *const timeval, reserved : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByNameA(s, nodename.param().abi(), servicename.param().abi(), localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSAConnectByNameW<P1, P2>(s: SOCKET, nodename: P1, servicename: P2, localaddresslength: Option<*mut u32>, localaddress: Option<*mut super::ws2::SOCKADDR>, remoteaddresslength: Option<*mut u32>, remoteaddress: Option<*mut super::ws2::SOCKADDR>, timeout: Option<*const timeval>, reserved: Option<*const super::minwinbase::OVERLAPPED>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAConnectByNameW(s : SOCKET, nodename : windows_core::PCWSTR, servicename : windows_core::PCWSTR, localaddresslength : *mut u32, localaddress : *mut super::ws2::SOCKADDR, remoteaddresslength : *mut u32, remoteaddress : *mut super::ws2::SOCKADDR, timeout : *const timeval, reserved : *const super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WSAConnectByNameW(s, nodename.param().abi(), servicename.param().abi(), localaddresslength.unwrap_or(core::mem::zeroed()) as _, localaddress.unwrap_or(core::mem::zeroed()) as _, remoteaddresslength.unwrap_or(core::mem::zeroed()) as _, remoteaddress.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSACreateEvent() -> super::winnt::HANDLE {
    windows_core::link!("ws2_32.dll" "system" fn WSACreateEvent() -> super::winnt::HANDLE);
    unsafe { WSACreateEvent() }
}
#[inline]
pub unsafe fn WSADuplicateSocketA(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSADuplicateSocketA(s : SOCKET, dwprocessid : u32, lpprotocolinfo : *mut WSAPROTOCOL_INFOA) -> i32);
    unsafe { WSADuplicateSocketA(s, dwprocessid, lpprotocolinfo as _) }
}
#[inline]
pub unsafe fn WSADuplicateSocketW(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSADuplicateSocketW(s : SOCKET, dwprocessid : u32, lpprotocolinfo : *mut WSAPROTOCOL_INFOW) -> i32);
    unsafe { WSADuplicateSocketW(s, dwprocessid, lpprotocolinfo as _) }
}
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersA(lpdwbufferlength : *mut u32, lpnspbuffer : *mut WSANAMESPACE_INFOA) -> i32);
    unsafe { WSAEnumNameSpaceProvidersA(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength : *mut u32, lpnspbuffer : *mut WSANAMESPACE_INFOEXA) -> i32);
    unsafe { WSAEnumNameSpaceProvidersExA(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(feature = "wtypesbase")]
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength : *mut u32, lpnspbuffer : *mut WSANAMESPACE_INFOEXW) -> i32);
    unsafe { WSAEnumNameSpaceProvidersExW(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[inline]
pub unsafe fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNameSpaceProvidersW(lpdwbufferlength : *mut u32, lpnspbuffer : *mut WSANAMESPACE_INFOW) -> i32);
    unsafe { WSAEnumNameSpaceProvidersW(lpdwbufferlength as _, lpnspbuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAEnumNetworkEvents(s: SOCKET, heventobject: super::winnt::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumNetworkEvents(s : SOCKET, heventobject : super::winnt::HANDLE, lpnetworkevents : *mut WSANETWORKEVENTS) -> i32);
    unsafe { WSAEnumNetworkEvents(s, heventobject, lpnetworkevents as _) }
}
#[inline]
pub unsafe fn WSAEnumProtocolsA(lpiprotocols: Option<*const i32>, lpprotocolbuffer: Option<*mut WSAPROTOCOL_INFOA>, lpdwbufferlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumProtocolsA(lpiprotocols : *const i32, lpprotocolbuffer : *mut WSAPROTOCOL_INFOA, lpdwbufferlength : *mut u32) -> i32);
    unsafe { WSAEnumProtocolsA(lpiprotocols.unwrap_or(core::mem::zeroed()) as _, lpprotocolbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[inline]
pub unsafe fn WSAEnumProtocolsW(lpiprotocols: Option<*const i32>, lpprotocolbuffer: Option<*mut WSAPROTOCOL_INFOW>, lpdwbufferlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEnumProtocolsW(lpiprotocols : *const i32, lpprotocolbuffer : *mut WSAPROTOCOL_INFOW, lpdwbufferlength : *mut u32) -> i32);
    unsafe { WSAEnumProtocolsW(lpiprotocols.unwrap_or(core::mem::zeroed()) as _, lpprotocolbuffer.unwrap_or(core::mem::zeroed()) as _, lpdwbufferlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAEventSelect(s: SOCKET, heventobject: Option<super::winnt::HANDLE>, lnetworkevents: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAEventSelect(s : SOCKET, heventobject : super::winnt::HANDLE, lnetworkevents : i32) -> i32);
    unsafe { WSAEventSelect(s, heventobject.unwrap_or(core::mem::zeroed()) as _, lnetworkevents) }
}
#[inline]
pub unsafe fn WSAGetLastError() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetLastError() -> i32);
    unsafe { WSAGetLastError() }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: *const super::minwinbase::OVERLAPPED, lpcbtransfer: *mut u32, fwait: bool, lpdwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetOverlappedResult(s : SOCKET, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcbtransfer : *mut u32, fwait : windows_core::BOOL, lpdwflags : *mut u32) -> windows_core::BOOL);
    unsafe { WSAGetOverlappedResult(s, lpoverlapped, lpcbtransfer as _, fwait.into(), lpdwflags as _) }
}
#[cfg(all(feature = "qos", feature = "ws2"))]
#[inline]
pub unsafe fn WSAGetQOSByName(s: SOCKET, lpqosname: *const super::ws2::WSABUF, lpqos: *mut QOS) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetQOSByName(s : SOCKET, lpqosname : *const super::ws2::WSABUF, lpqos : *mut QOS) -> windows_core::BOOL);
    unsafe { WSAGetQOSByName(s, lpqosname, lpqos as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn WSAGetServiceClassInfoA(lpproviderid: *const windows_core::GUID, lpserviceclassid: *const windows_core::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassInfoA(lpproviderid : *const windows_core::GUID, lpserviceclassid : *const windows_core::GUID, lpdwbufsize : *mut u32, lpserviceclassinfo : *mut WSASERVICECLASSINFOA) -> i32);
    unsafe { WSAGetServiceClassInfoA(lpproviderid, lpserviceclassid, lpdwbufsize as _, lpserviceclassinfo as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn WSAGetServiceClassInfoW(lpproviderid: *const windows_core::GUID, lpserviceclassid: *const windows_core::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassInfoW(lpproviderid : *const windows_core::GUID, lpserviceclassid : *const windows_core::GUID, lpdwbufsize : *mut u32, lpserviceclassinfo : *mut WSASERVICECLASSINFOW) -> i32);
    unsafe { WSAGetServiceClassInfoW(lpproviderid, lpserviceclassid, lpdwbufsize as _, lpserviceclassinfo as _) }
}
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const windows_core::GUID, lpszserviceclassname: windows_core::PSTR, lpdwbufferlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassNameByClassIdA(lpserviceclassid : *const windows_core::GUID, lpszserviceclassname : windows_core::PSTR, lpdwbufferlength : *mut u32) -> i32);
    unsafe { WSAGetServiceClassNameByClassIdA(lpserviceclassid, lpszserviceclassname, lpdwbufferlength as _) }
}
#[inline]
pub unsafe fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const windows_core::GUID, lpszserviceclassname: windows_core::PWSTR, lpdwbufferlength: *mut u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAGetServiceClassNameByClassIdW(lpserviceclassid : *const windows_core::GUID, lpszserviceclassname : windows_core::PWSTR, lpdwbufferlength : *mut u32) -> i32);
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
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAInstallServiceClassA(lpserviceclassinfo : *const WSASERVICECLASSINFOA) -> i32);
    unsafe { WSAInstallServiceClassA(lpserviceclassinfo) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAInstallServiceClassW(lpserviceclassinfo : *const WSASERVICECLASSINFOW) -> i32);
    unsafe { WSAInstallServiceClassW(lpserviceclassinfo) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WSAIoctl(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: Option<*const core::ffi::c_void>, cbinbuffer: u32, lpvoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAIoctl(s : SOCKET, dwiocontrolcode : u32, lpvinbuffer : *const core::ffi::c_void, cbinbuffer : u32, lpvoutbuffer : *mut core::ffi::c_void, cboutbuffer : u32, lpcbbytesreturned : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSAIoctl(s, dwiocontrolcode, lpvinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffer, lpvoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffer, lpcbbytesreturned as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[inline]
pub unsafe fn WSAIsBlocking() -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAIsBlocking() -> windows_core::BOOL);
    unsafe { WSAIsBlocking() }
}
#[cfg(all(feature = "qos", feature = "ws2"))]
#[inline]
pub unsafe fn WSAJoinLeaf(s: SOCKET, name: *const super::ws2::SOCKADDR, namelen: i32, lpcallerdata: Option<*const super::ws2::WSABUF>, lpcalleedata: Option<*mut super::ws2::WSABUF>, lpsqos: Option<*const QOS>, lpgqos: Option<*const QOS>, dwflags: u32) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSAJoinLeaf(s : SOCKET, name : *const super::ws2::SOCKADDR, namelen : i32, lpcallerdata : *const super::ws2::WSABUF, lpcalleedata : *mut super::ws2::WSABUF, lpsqos : *const QOS, lpgqos : *const QOS, dwflags : u32) -> SOCKET);
    unsafe { WSAJoinLeaf(s, name, namelen, lpcallerdata.unwrap_or(core::mem::zeroed()) as _, lpcalleedata.unwrap_or(core::mem::zeroed()) as _, lpsqos.unwrap_or(core::mem::zeroed()) as _, lpgqos.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceBeginA(lpqsrestrictions : *const WSAQUERYSETA, dwcontrolflags : u32, lphlookup : *mut super::winnt::HANDLE) -> i32);
    unsafe { WSALookupServiceBeginA(lpqsrestrictions, dwcontrolflags, lphlookup as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceBeginW(lpqsrestrictions : *const WSAQUERYSETW, dwcontrolflags : u32, lphlookup : *mut super::winnt::HANDLE) -> i32);
    unsafe { WSALookupServiceBeginW(lpqsrestrictions, dwcontrolflags, lphlookup as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSALookupServiceEnd(hlookup: super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceEnd(hlookup : super::winnt::HANDLE) -> i32);
    unsafe { WSALookupServiceEnd(hlookup) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceNextA(hlookup: super::winnt::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceNextA(hlookup : super::winnt::HANDLE, dwcontrolflags : u32, lpdwbufferlength : *mut u32, lpqsresults : *mut WSAQUERYSETA) -> i32);
    unsafe { WSALookupServiceNextA(hlookup, dwcontrolflags, lpdwbufferlength as _, lpqsresults as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSALookupServiceNextW(hlookup: super::winnt::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: Option<*mut WSAQUERYSETW>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSALookupServiceNextW(hlookup : super::winnt::HANDLE, dwcontrolflags : u32, lpdwbufferlength : *mut u32, lpqsresults : *mut WSAQUERYSETW) -> i32);
    unsafe { WSALookupServiceNextW(hlookup, dwcontrolflags, lpdwbufferlength as _, lpqsresults.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn WSANSPIoctl(hlookup: super::winnt::HANDLE, dwcontrolcode: u32, lpvinbuffer: Option<*const core::ffi::c_void>, cbinbuffer: u32, lpvoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: Option<*const WSACOMPLETION>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSANSPIoctl(hlookup : super::winnt::HANDLE, dwcontrolcode : u32, lpvinbuffer : *const core::ffi::c_void, cbinbuffer : u32, lpvoutbuffer : *mut core::ffi::c_void, cboutbuffer : u32, lpcbbytesreturned : *mut u32, lpcompletion : *const WSACOMPLETION) -> i32);
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
#[inline]
pub unsafe fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAPoll(fdarray : *mut WSAPOLLFD, fds : u32, timeout : i32) -> i32);
    unsafe { WSAPoll(fdarray as _, fds, timeout) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WSAProviderConfigChange(lpnotificationhandle: *mut super::winnt::HANDLE, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAProviderConfigChange(lpnotificationhandle : *mut super::winnt::HANDLE, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSAProviderConfigChange(lpnotificationhandle as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSARecv(s: SOCKET, lpbuffers: &[super::ws2::WSABUF], lpnumberofbytesrecvd: Option<*mut u32>, lpflags: *mut u32, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecv(s : SOCKET, lpbuffers : *const super::ws2::WSABUF, dwbuffercount : u32, lpnumberofbytesrecvd : *mut u32, lpflags : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSARecv(s, lpbuffers.as_ptr(), lpbuffers.len().try_into().unwrap(), lpnumberofbytesrecvd.unwrap_or(core::mem::zeroed()) as _, lpflags as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: Option<*const super::ws2::WSABUF>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecvDisconnect(s : SOCKET, lpinbounddisconnectdata : *const super::ws2::WSABUF) -> i32);
    unsafe { WSARecvDisconnect(s, lpinbounddisconnectdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSARecvFrom(s: SOCKET, lpbuffers: &[super::ws2::WSABUF], lpnumberofbytesrecvd: Option<*mut u32>, lpflags: *mut u32, lpfrom: Option<*mut super::ws2::SOCKADDR>, lpfromlen: Option<*mut i32>, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARecvFrom(s : SOCKET, lpbuffers : *const super::ws2::WSABUF, dwbuffercount : u32, lpnumberofbytesrecvd : *mut u32, lpflags : *mut u32, lpfrom : *mut super::ws2::SOCKADDR, lpfromlen : *mut i32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSARecvFrom(s, lpbuffers.as_ptr(), lpbuffers.len().try_into().unwrap(), lpnumberofbytesrecvd.unwrap_or(core::mem::zeroed()) as _, lpflags as _, lpfrom.unwrap_or(core::mem::zeroed()) as _, lpfromlen.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[inline]
pub unsafe fn WSARemoveServiceClass(lpserviceclassid: *const windows_core::GUID) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSARemoveServiceClass(lpserviceclassid : *const windows_core::GUID) -> i32);
    unsafe { WSARemoveServiceClass(lpserviceclassid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAResetEvent(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSAResetEvent(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WSAResetEvent(hevent) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASend(s: SOCKET, lpbuffers: &[super::ws2::WSABUF], lpnumberofbytessent: Option<*mut u32>, dwflags: u32, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASend(s : SOCKET, lpbuffers : *const super::ws2::WSABUF, dwbuffercount : u32, lpnumberofbytessent : *mut u32, dwflags : u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASend(s, lpbuffers.as_ptr(), lpbuffers.len().try_into().unwrap(), lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, dwflags, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: Option<*const super::ws2::WSABUF>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendDisconnect(s : SOCKET, lpoutbounddisconnectdata : *const super::ws2::WSABUF) -> i32);
    unsafe { WSASendDisconnect(s, lpoutbounddisconnectdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASendMsg(handle: SOCKET, lpmsg: *const super::ws2::WSAMSG, dwflags: u32, lpnumberofbytessent: Option<*mut u32>, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendMsg(handle : SOCKET, lpmsg : *const super::ws2::WSAMSG, dwflags : u32, lpnumberofbytessent : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASendMsg(handle, lpmsg, dwflags, lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn WSASendTo(s: SOCKET, lpbuffers: &[super::ws2::WSABUF], lpnumberofbytessent: Option<*mut u32>, dwflags: u32, lpto: Option<*const super::ws2::SOCKADDR>, itolen: i32, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASendTo(s : SOCKET, lpbuffers : *const super::ws2::WSABUF, dwbuffercount : u32, lpnumberofbytessent : *mut u32, dwflags : u32, lpto : *const super::ws2::SOCKADDR, itolen : i32, lpoverlapped : *mut super::minwinbase::OVERLAPPED, lpcompletionroutine : LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32);
    unsafe { WSASendTo(s, lpbuffers.as_ptr(), lpbuffers.len().try_into().unwrap(), lpnumberofbytessent.unwrap_or(core::mem::zeroed()) as _, dwflags, lpto.unwrap_or(core::mem::zeroed()) as _, itolen, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WSASetBlockingHook(lpblockfunc: super::minwindef::FARPROC) -> super::minwindef::FARPROC {
    windows_core::link!("ws2_32.dll" "system" fn WSASetBlockingHook(lpblockfunc : super::minwindef::FARPROC) -> super::minwindef::FARPROC);
    unsafe { WSASetBlockingHook(lpblockfunc) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSASetEvent(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ws2_32.dll" "system" fn WSASetEvent(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { WSASetEvent(hevent) }
}
#[inline]
pub unsafe fn WSASetLastError(ierror: i32) {
    windows_core::link!("ws2_32.dll" "system" fn WSASetLastError(ierror : i32));
    unsafe { WSASetLastError(ierror) }
}
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASetServiceA(lpqsreginfo : *const WSAQUERYSETA, essoperation : WSAESETSERVICEOP, dwcontrolflags : u32) -> i32);
    unsafe { WSASetServiceA(lpqsreginfo, essoperation, dwcontrolflags) }
}
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSASetServiceW(lpqsreginfo : *const WSAQUERYSETW, essoperation : WSAESETSERVICEOP, dwcontrolflags : u32) -> i32);
    unsafe { WSASetServiceW(lpqsreginfo, essoperation, dwcontrolflags) }
}
#[inline]
pub unsafe fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOA>, g: GROUP, dwflags: u32) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSASocketA(af : i32, r#type : i32, protocol : i32, lpprotocolinfo : *const WSAPROTOCOL_INFOA, g : GROUP, dwflags : u32) -> SOCKET);
    unsafe { WSASocketA(af, r#type, protocol, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, g, dwflags) }
}
#[inline]
pub unsafe fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOW>, g: GROUP, dwflags: u32) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn WSASocketW(af : i32, r#type : i32, protocol : i32, lpprotocolinfo : *const WSAPROTOCOL_INFOW, g : GROUP, dwflags : u32) -> SOCKET);
    unsafe { WSASocketW(af, r#type, protocol, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, g, dwflags) }
}
#[inline]
pub unsafe fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSADATA) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAStartup(wversionrequested : u16, lpwsadata : *mut WSADATA) -> i32);
    unsafe { WSAStartup(wversionrequested, lpwsadata as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSAStringToAddressA<P0>(addressstring: P0, addressfamily: i32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOA>, lpaddress: *mut super::ws2::SOCKADDR, lpaddresslength: *mut i32) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAStringToAddressA(addressstring : windows_core::PCSTR, addressfamily : i32, lpprotocolinfo : *const WSAPROTOCOL_INFOA, lpaddress : *mut super::ws2::SOCKADDR, lpaddresslength : *mut i32) -> i32);
    unsafe { WSAStringToAddressA(addressstring.param().abi(), addressfamily, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpaddress as _, lpaddresslength as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn WSAStringToAddressW<P0>(addressstring: P0, addressfamily: i32, lpprotocolinfo: Option<*const WSAPROTOCOL_INFOW>, lpaddress: *mut super::ws2::SOCKADDR, lpaddresslength: *mut i32) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn WSAStringToAddressW(addressstring : windows_core::PCWSTR, addressfamily : i32, lpprotocolinfo : *const WSAPROTOCOL_INFOW, lpaddress : *mut super::ws2::SOCKADDR, lpaddresslength : *mut i32) -> i32);
    unsafe { WSAStringToAddressW(addressstring.param().abi(), addressfamily, lpprotocolinfo.unwrap_or(core::mem::zeroed()) as _, lpaddress as _, lpaddresslength as _) }
}
#[inline]
pub unsafe fn WSAUnhookBlockingHook() -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAUnhookBlockingHook() -> i32);
    unsafe { WSAUnhookBlockingHook() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WSAWaitForMultipleEvents(lphevents: &[super::winnt::HANDLE], fwaitall: bool, dwtimeout: u32, falertable: bool) -> u32 {
    windows_core::link!("ws2_32.dll" "system" fn WSAWaitForMultipleEvents(cevents : u32, lphevents : *const super::winnt::HANDLE, fwaitall : windows_core::BOOL, dwtimeout : u32, falertable : windows_core::BOOL) -> u32);
    unsafe { WSAWaitForMultipleEvents(lphevents.len().try_into().unwrap(), lphevents.as_ptr(), fwaitall.into(), dwtimeout, falertable.into()) }
}
#[inline]
pub unsafe fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn __WSAFDIsSet(fd : SOCKET, param1 : *mut fd_set) -> i32);
    unsafe { __WSAFDIsSet(fd, param1 as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn accept(s: SOCKET, addr: Option<*mut super::ws2::SOCKADDR>, addrlen: Option<*mut i32>) -> SOCKET {
    windows_core::link!("ws2_32.dll" "system" fn accept(s : SOCKET, addr : *mut super::ws2::SOCKADDR, addrlen : *mut i32) -> SOCKET);
    unsafe { accept(s, addr.unwrap_or(core::mem::zeroed()) as _, addrlen.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn bind(s: SOCKET, name: *const super::ws2::SOCKADDR, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn bind(s : SOCKET, name : *const super::ws2::SOCKADDR, namelen : i32) -> i32);
    unsafe { bind(s, name, namelen) }
}
#[inline]
pub unsafe fn closesocket(s: SOCKET) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn closesocket(s : SOCKET) -> i32);
    unsafe { closesocket(s) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn connect(s: SOCKET, name: *const super::ws2::SOCKADDR, namelen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn connect(s : SOCKET, name : *const super::ws2::SOCKADDR, namelen : i32) -> i32);
    unsafe { connect(s, name, namelen) }
}
#[inline]
pub unsafe fn gethostbyaddr(addr: &[u8], r#type: i32) -> *mut hostent {
    windows_core::link!("ws2_32.dll" "system" fn gethostbyaddr(addr : *const i8, len : i32, r#type : i32) -> *mut hostent);
    unsafe { gethostbyaddr(core::mem::transmute(addr.as_ptr()), addr.len().try_into().unwrap(), r#type) }
}
#[inline]
pub unsafe fn gethostbyname<P0>(name: P0) -> *mut hostent
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn gethostbyname(name : windows_core::PCSTR) -> *mut hostent);
    unsafe { gethostbyname(name.param().abi()) }
}
#[inline]
pub unsafe fn gethostname(name: &mut [u8]) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn gethostname(name : *mut i8, namelen : i32) -> i32);
    unsafe { gethostname(core::mem::transmute(name.as_mut_ptr()), name.len().try_into().unwrap()) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn getpeername(s: SOCKET, name: *mut super::ws2::SOCKADDR, namelen: *mut i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getpeername(s : SOCKET, name : *mut super::ws2::SOCKADDR, namelen : *mut i32) -> i32);
    unsafe { getpeername(s, name as _, namelen as _) }
}
#[inline]
pub unsafe fn getprotobyname<P0>(name: P0) -> *mut protoent
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn getprotobyname(name : windows_core::PCSTR) -> *mut protoent);
    unsafe { getprotobyname(name.param().abi()) }
}
#[inline]
pub unsafe fn getprotobynumber(number: i32) -> *mut protoent {
    windows_core::link!("ws2_32.dll" "system" fn getprotobynumber(number : i32) -> *mut protoent);
    unsafe { getprotobynumber(number) }
}
#[inline]
pub unsafe fn getservbyname<P0, P1>(name: P0, proto: P1) -> *mut servent
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn getservbyname(name : windows_core::PCSTR, proto : windows_core::PCSTR) -> *mut servent);
    unsafe { getservbyname(name.param().abi(), proto.param().abi()) }
}
#[inline]
pub unsafe fn getservbyport<P1>(port: i32, proto: P1) -> *mut servent
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn getservbyport(port : i32, proto : windows_core::PCSTR) -> *mut servent);
    unsafe { getservbyport(port, proto.param().abi()) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn getsockname(s: SOCKET, name: *mut super::ws2::SOCKADDR, namelen: *mut i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getsockname(s : SOCKET, name : *mut super::ws2::SOCKADDR, namelen : *mut i32) -> i32);
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
pub unsafe fn inet_addr<P0>(cp: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn inet_addr(cp : windows_core::PCSTR) -> u32);
    unsafe { inet_addr(cp.param().abi()) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn inet_ntoa(r#in: super::inaddr::IN_ADDR) -> *mut i8 {
    windows_core::link!("ws2_32.dll" "system" fn inet_ntoa(r#in : super::inaddr::IN_ADDR) -> *mut i8);
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
pub unsafe fn recv(s: SOCKET, buf: &mut [u8], flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn recv(s : SOCKET, buf : *mut i8, len : i32, flags : i32) -> i32);
    unsafe { recv(s, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap(), flags) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn recvfrom(s: SOCKET, buf: &mut [u8], flags: i32, from: Option<*mut super::ws2::SOCKADDR>, fromlen: Option<*mut i32>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn recvfrom(s : SOCKET, buf : *mut i8, len : i32, flags : i32, from : *mut super::ws2::SOCKADDR, fromlen : *mut i32) -> i32);
    unsafe { recvfrom(s, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap(), flags, from.unwrap_or(core::mem::zeroed()) as _, fromlen.unwrap_or(core::mem::zeroed()) as _) }
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
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn sendto(s: SOCKET, buf: &[u8], flags: i32, to: *const super::ws2::SOCKADDR, tolen: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn sendto(s : SOCKET, buf : *const i8, len : i32, flags : i32, to : *const super::ws2::SOCKADDR, tolen : i32) -> i32);
    unsafe { sendto(s, core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap(), flags, to, tolen) }
}
#[inline]
pub unsafe fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: Option<&[u8]>) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn setsockopt(s : SOCKET, level : i32, optname : i32, optval : *const i8, optlen : i32) -> i32);
    unsafe { setsockopt(s, level, optname, core::mem::transmute(optval.map_or(core::ptr::null(), |slice| slice.as_ptr())), optval.map_or(0, |slice| slice.len().try_into().unwrap())) }
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
pub const ADDR_ANY: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AFPROTOCOLS {
    pub iAddressFamily: i32,
    pub iProtocol: i32,
}
pub const BASE_PROTOCOL: u32 = 1;
pub const BIGENDIAN: u32 = 0;
pub const CF_ACCEPT: u32 = 0;
pub const CF_DEFER: u32 = 2;
pub const CF_REJECT: u32 = 1;
pub const COMP_EQUAL: WSAECOMPARATOR = 0;
pub const COMP_NOTLESS: WSAECOMPARATOR = 1;
pub const FD_ACCEPT: u32 = 8;
pub const FD_ACCEPT_BIT: u32 = 3;
pub const FD_ADDRESS_LIST_CHANGE: u32 = 512;
pub const FD_ADDRESS_LIST_CHANGE_BIT: u32 = 9;
pub const FD_ALL_EVENTS: u32 = 1023;
pub const FD_CLOSE: u32 = 32;
pub const FD_CLOSE_BIT: u32 = 5;
pub const FD_CONNECT: u32 = 16;
pub const FD_CONNECT_BIT: u32 = 4;
pub const FD_GROUP_QOS: u32 = 128;
pub const FD_GROUP_QOS_BIT: u32 = 7;
pub const FD_MAX_EVENTS: u32 = 10;
pub const FD_OOB: u32 = 4;
pub const FD_OOB_BIT: u32 = 2;
pub const FD_QOS: u32 = 64;
pub const FD_QOS_BIT: u32 = 6;
pub const FD_READ: u32 = 1;
pub const FD_READ_BIT: u32 = 0;
pub const FD_ROUTING_INTERFACE_CHANGE: u32 = 256;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: u32 = 8;
pub type FD_SET = fd_set;
pub const FD_SETSIZE: u32 = 64;
pub const FD_WRITE: u32 = 2;
pub const FD_WRITE_BIT: u32 = 1;
pub const FIOASYNC: i32 = -2147195267;
pub const FIONBIO: i32 = -2147195266;
pub const FIONREAD: u32 = 1074030207;
pub const FROM_PROTOCOL_INFO: i32 = -1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GROUP(pub u32);
pub type HOSTENT = hostent;
pub const HOST_NOT_FOUND: u32 = 11001;
pub const IMPLINK_HIGHEXPER: u32 = 158;
pub const IMPLINK_IP: u32 = 155;
pub const IMPLINK_LOWEXPER: u32 = 156;
pub const INCL_WINSOCK_API_PROTOTYPES: u32 = 1;
pub const INCL_WINSOCK_API_TYPEDEFS: u32 = 0;
pub const INVALID_SOCKET: i32 = -1;
pub const JL_BOTH: u32 = 4;
pub const JL_RECEIVER_ONLY: u32 = 2;
pub const JL_SENDER_ONLY: u32 = 1;
pub const LAYERED_PROTOCOL: u32 = 0;
pub type LINGER = linger;
pub const LITTLEENDIAN: u32 = 1;
pub type LPAFPROTOCOLS = *mut AFPROTOCOLS;
#[cfg(all(feature = "qos", feature = "ws2"))]
pub type LPCONDITIONPROC = Option<unsafe extern "system" fn(lpcallerid: *mut super::ws2::WSABUF, lpcallerdata: *mut super::ws2::WSABUF, lpsqos: *mut QOS, lpgqos: *mut QOS, lpcalleeid: *mut super::ws2::WSABUF, lpcalleedata: *mut super::ws2::WSABUF, g: *mut GROUP, dwcallbackdata: usize) -> i32>;
pub type LPFD_SET = *mut fd_set;
pub type LPHOSTENT = *mut hostent;
pub type LPLINGER = *mut linger;
pub type LPPROTOENT = *mut protoent;
#[cfg(all(feature = "qos", feature = "ws2"))]
pub type LPQOS = *mut QOS;
pub type LPSERVENT = *mut servent;
#[cfg(all(feature = "inaddr", feature = "ws2"))]
pub type LPSOCKADDR_IN = *mut super::ws2::SOCKADDR_IN;
pub type LPTIMEVAL = *mut timeval;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type LPWSACOMPLETION = *mut WSACOMPLETION;
pub type LPWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
pub type LPWSADATA = *mut WSADATA;
pub type LPWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type LPWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
pub type LPWSANAMESPACE_INFO = LPWSANAMESPACE_INFOA;
pub type LPWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
#[cfg(feature = "wtypesbase")]
pub type LPWSANAMESPACE_INFOEX = LPWSANAMESPACE_INFOEXA;
#[cfg(feature = "wtypesbase")]
pub type LPWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
#[cfg(feature = "wtypesbase")]
pub type LPWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub type LPWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
pub type LPWSANETWORKEVENTS = *mut WSANETWORKEVENTS;
pub type LPWSANSCLASSINFO = LPWSANSCLASSINFOA;
pub type LPWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
pub type LPWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LPWSAOVERLAPPED = *mut super::minwinbase::OVERLAPPED;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: u32, cbtransferred: u32, lpoverlapped: *mut super::minwinbase::OVERLAPPED, dwflags: u32)>;
pub type LPWSAPOLLFD = *mut WSAPOLLFD;
pub type LPWSAPROTOCOLCHAIN = *mut WSAPROTOCOLCHAIN;
pub type LPWSAPROTOCOL_INFO = LPWSAPROTOCOL_INFOA;
pub type LPWSAPROTOCOL_INFOA = *mut WSAPROTOCOL_INFOA;
pub type LPWSAPROTOCOL_INFOW = *mut WSAPROTOCOL_INFOW;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET = LPWSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2 = LPWSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2A = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSET2W = *mut WSAQUERYSET2W;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSETA = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type LPWSAQUERYSETW = *mut WSAQUERYSETW;
#[cfg(feature = "guiddef")]
pub type LPWSASERVICECLASSINFO = LPWSASERVICECLASSINFOA;
#[cfg(feature = "guiddef")]
pub type LPWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
#[cfg(feature = "guiddef")]
pub type LPWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
pub type LPWSAVERSION = *mut WSAVERSION;
pub const LUP_ADDRCONFIG: u32 = 1048576;
pub const LUP_API_ANSI: u32 = 16777216;
pub const LUP_CONTAINERS: u32 = 2;
pub const LUP_DEEP: u32 = 1;
pub const LUP_DISABLE_IDN_ENCODING: u32 = 8388608;
pub const LUP_DNS_ONLY: u32 = 131072;
pub const LUP_DUAL_ADDR: u32 = 2097152;
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: u32 = 134217728;
pub const LUP_EXTENDED_QUERYSET: u32 = 33554432;
pub const LUP_FILESERVER: u32 = 4194304;
pub const LUP_FLUSHCACHE: u32 = 4096;
pub const LUP_FLUSHPREVIOUS: u32 = 8192;
pub const LUP_FORCE_CLEAR_TEXT: u32 = 1073741824;
pub const LUP_NEAREST: u32 = 8;
pub const LUP_NOCONTAINERS: u32 = 4;
pub const LUP_NON_AUTHORITATIVE: u32 = 16384;
pub const LUP_REQUIRE_SECURE: u32 = 268435456;
pub const LUP_RESERVED_UNUSED: u32 = 524288;
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648;
pub const LUP_RES_SERVICE: u32 = 32768;
pub const LUP_RETURN_ADDR: u32 = 256;
pub const LUP_RETURN_ALIASES: u32 = 1024;
pub const LUP_RETURN_ALL: u32 = 4080;
pub const LUP_RETURN_BLOB: u32 = 512;
pub const LUP_RETURN_COMMENT: u32 = 128;
pub const LUP_RETURN_NAME: u32 = 16;
pub const LUP_RETURN_PREFERRED_NAMES: u32 = 65536;
pub const LUP_RETURN_QUERY_STRING: u32 = 2048;
pub const LUP_RETURN_RESPONSE_FLAGS: u32 = 262144;
pub const LUP_RETURN_TTL: u32 = 536870912;
pub const LUP_RETURN_TYPE: u32 = 32;
pub const LUP_RETURN_VERSION: u32 = 64;
pub const LUP_SECURE: u32 = 32768;
pub const LUP_SECURE_WITH_FALLBACK: u32 = 67108864;
pub const MAXGETHOSTSTRUCT: u32 = 1024;
pub const MAX_PROTOCOL_CHAIN: u32 = 7;
pub const MSG_DONTROUTE: u32 = 4;
pub const MSG_INTERRUPT: u32 = 16;
pub const MSG_MAXIOVLEN: u32 = 16;
pub const MSG_OOB: u32 = 1;
pub const MSG_PARTIAL: u32 = 32768;
pub const MSG_PEEK: u32 = 2;
pub const MSG_PUSH_IMMEDIATE: u32 = 32;
pub const MSG_WAITALL: u32 = 8;
pub const NO_ADDRESS: u32 = 11004;
pub const NO_DATA: u32 = 11004;
pub const NO_RECOVERY: u32 = 11003;
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = 4;
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = 2;
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = 1;
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = 0;
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = 3;
pub const NS_LOCALNAME: u32 = 19;
pub type PAFPROTOCOLS = *mut AFPROTOCOLS;
pub type PFD_SET = *mut fd_set;
pub const PFL_HIDDEN: u32 = 4;
pub const PFL_MATCHES_PROTOCOL_ZERO: u32 = 8;
pub const PFL_MULTIPLE_PROTO_ENTRIES: u32 = 1;
pub const PFL_NETWORKDIRECT_PROVIDER: u32 = 16;
pub const PFL_RECOMMENDED_PROTO_ENTRY: u32 = 2;
pub const PF_APPLETALK: u32 = 16;
pub const PF_ATM: u32 = 22;
pub const PF_BAN: u32 = 21;
pub const PF_BTH: u32 = 32;
pub const PF_CCITT: u32 = 10;
pub const PF_CHAOS: u32 = 5;
pub const PF_DATAKIT: u32 = 9;
pub const PF_DECnet: u32 = 12;
pub const PF_DLI: u32 = 13;
pub const PF_ECMA: u32 = 8;
pub const PF_FIREFOX: u32 = 19;
pub const PF_HYLINK: u32 = 15;
pub const PF_IMPLINK: u32 = 3;
pub const PF_INET: u32 = 2;
pub const PF_INET6: u32 = 23;
pub const PF_IPX: u32 = 6;
pub const PF_ISO: u32 = 7;
pub const PF_LAT: u32 = 14;
pub const PF_MAX: u32 = 35;
pub const PF_NS: u32 = 6;
pub const PF_OSI: u32 = 7;
pub const PF_PUP: u32 = 4;
pub const PF_SNA: u32 = 11;
pub const PF_UNIX: u32 = 1;
pub const PF_UNKNOWN1: u32 = 20;
pub const PF_UNSPEC: u32 = 0;
pub const PF_VOICEVIEW: u32 = 18;
pub type PHOSTENT = *mut hostent;
pub type PLINGER = *mut linger;
pub const POLLERR: u32 = 1;
pub const POLLHUP: u32 = 2;
pub const POLLIN: u32 = 768;
pub const POLLNVAL: u32 = 4;
pub const POLLOUT: u32 = 16;
pub const POLLPRI: u32 = 1024;
pub const POLLRDBAND: u32 = 512;
pub const POLLRDNORM: u32 = 256;
pub const POLLWRBAND: u32 = 32;
pub const POLLWRNORM: u32 = 16;
pub type PPROTOENT = *mut protoent;
pub type PROTOENT = protoent;
pub type PSERVENT = *mut servent;
pub type PTIMEVAL = *mut timeval;
pub const PVD_CONFIG: u32 = 12289;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PWSACOMPLETION = *mut WSACOMPLETION;
pub type PWSACOMPLETIONTYPE = *mut WSACOMPLETIONTYPE;
pub type PWSAECOMPARATOR = *mut WSAECOMPARATOR;
pub type PWSAESETSERVICEOP = *mut WSAESETSERVICEOP;
pub type PWSANAMESPACE_INFO = PWSANAMESPACE_INFOA;
pub type PWSANAMESPACE_INFOA = *mut WSANAMESPACE_INFOA;
#[cfg(feature = "wtypesbase")]
pub type PWSANAMESPACE_INFOEX = PWSANAMESPACE_INFOEXA;
#[cfg(feature = "wtypesbase")]
pub type PWSANAMESPACE_INFOEXA = *mut WSANAMESPACE_INFOEXA;
#[cfg(feature = "wtypesbase")]
pub type PWSANAMESPACE_INFOEXW = *mut WSANAMESPACE_INFOEXW;
pub type PWSANAMESPACE_INFOW = *mut WSANAMESPACE_INFOW;
pub type PWSANSCLASSINFO = PWSANSCLASSINFOA;
pub type PWSANSCLASSINFOA = *mut WSANSCLASSINFOA;
pub type PWSANSCLASSINFOW = *mut WSANSCLASSINFOW;
pub type PWSAPOLLFD = *mut WSAPOLLFD;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET = PWSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2 = PWSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2A = *mut WSAQUERYSET2A;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSET2W = *mut WSAQUERYSET2W;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSETA = *mut WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type PWSAQUERYSETW = *mut WSAQUERYSETW;
#[cfg(feature = "guiddef")]
pub type PWSASERVICECLASSINFO = PWSASERVICECLASSINFOA;
#[cfg(feature = "guiddef")]
pub type PWSASERVICECLASSINFOA = *mut WSASERVICECLASSINFOA;
#[cfg(feature = "guiddef")]
pub type PWSASERVICECLASSINFOW = *mut WSASERVICECLASSINFOW;
pub type PWSAVERSION = *mut WSAVERSION;
#[repr(C)]
#[cfg(all(feature = "qos", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS {
    pub SendingFlowspec: super::qos::FLOWSPEC,
    pub ReceivingFlowspec: super::qos::FLOWSPEC,
    pub ProviderSpecific: super::ws2::WSABUF,
}
pub const RESULT_IS_ADDED: u32 = 16;
pub const RESULT_IS_ALIAS: u32 = 1;
pub const RESULT_IS_CHANGED: u32 = 32;
pub const RESULT_IS_DELETED: u32 = 64;
pub const RES_FLUSH_CACHE: u32 = 2;
pub const RES_SERVICE: u32 = 4;
pub const RES_UNUSED_1: u32 = 1;
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = 2;
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = 1;
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = 0;
pub const SD_BOTH: u32 = 2;
pub const SD_RECEIVE: u32 = 0;
pub const SD_SEND: u32 = 1;
pub const SECURITY_PROTOCOL_NONE: u32 = 0;
pub type SERVENT = servent;
pub const SERVICE_MULTIPLE: u32 = 1;
pub const SERVICE_TYPE_VALUE_IPXPORTA: windows_core::PCSTR = windows_core::s!("IpxSocket");
pub const SERVICE_TYPE_VALUE_IPXPORTW: windows_core::PCWSTR = windows_core::w!("IpxSocket");
pub const SERVICE_TYPE_VALUE_OBJECTIDA: windows_core::PCSTR = windows_core::s!("ObjectId");
pub const SERVICE_TYPE_VALUE_OBJECTIDW: windows_core::PCWSTR = windows_core::w!("ObjectId");
pub const SERVICE_TYPE_VALUE_SAPIDA: windows_core::PCSTR = windows_core::s!("SapId");
pub const SERVICE_TYPE_VALUE_SAPIDW: windows_core::PCWSTR = windows_core::w!("SapId");
pub const SERVICE_TYPE_VALUE_TCPPORTA: windows_core::PCSTR = windows_core::s!("TcpPort");
pub const SERVICE_TYPE_VALUE_TCPPORTW: windows_core::PCWSTR = windows_core::w!("TcpPort");
pub const SERVICE_TYPE_VALUE_UDPPORTA: windows_core::PCSTR = windows_core::s!("UdpPort");
pub const SERVICE_TYPE_VALUE_UDPPORTW: windows_core::PCWSTR = windows_core::w!("UdpPort");
pub const SG_CONSTRAINED_GROUP: u32 = 2;
pub const SG_UNCONSTRAINED_GROUP: u32 = 1;
pub const SIOCATMARK: u32 = 1074033415;
pub const SIOCGHIWAT: u32 = 1074033409;
pub const SIOCGLOWAT: u32 = 1074033411;
pub const SIOCSHIWAT: i32 = -2147192064;
pub const SIOCSLOWAT: i32 = -2147192062;
pub const SIO_NSP_NOTIFY_CHANGE: i32 = -2013265895;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SOCKET(pub usize);
pub const SOCKET_ERROR: i32 = -1;
pub const SOCK_NOTIFY_EVENTS_ALL: u32 = 199;
pub const SOCK_NOTIFY_EVENT_ERR: u32 = 64;
pub const SOCK_NOTIFY_EVENT_HANGUP: u32 = 4;
pub const SOCK_NOTIFY_EVENT_IN: u32 = 1;
pub const SOCK_NOTIFY_EVENT_OUT: u32 = 2;
pub const SOCK_NOTIFY_EVENT_REMOVE: u32 = 128;
pub const SOCK_NOTIFY_OP_DISABLE: u32 = 2;
pub const SOCK_NOTIFY_OP_ENABLE: u32 = 1;
pub const SOCK_NOTIFY_OP_NONE: u32 = 0;
pub const SOCK_NOTIFY_OP_REMOVE: u32 = 4;
pub const SOCK_NOTIFY_REGISTER_EVENTS_ALL: u32 = 7;
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: u32 = 4;
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: u32 = 1;
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: u32 = 0;
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCK_NOTIFY_REGISTRATION {
    pub socket: SOCKET,
    pub completionKey: *mut core::ffi::c_void,
    pub eventFilter: u16,
    pub operation: u8,
    pub triggerFlags: u8,
    pub registrationResult: u32,
}
impl Default for SOCK_NOTIFY_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOCK_NOTIFY_TRIGGER_ALL: u32 = 15;
pub const SOCK_NOTIFY_TRIGGER_EDGE: u32 = 8;
pub const SOCK_NOTIFY_TRIGGER_LEVEL: u32 = 4;
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: u32 = 1;
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: u32 = 2;
pub const SOMAXCONN: u32 = 2147483647;
pub const SO_PROTOCOL_INFO: u32 = 8196;
pub const SO_PROTOCOL_INFOA: u32 = 8196;
pub const SO_PROTOCOL_INFOW: u32 = 8197;
pub const TH_NETDEV: u32 = 1;
pub const TH_TAPI: u32 = 2;
pub type TIMEVAL = timeval;
pub const TRY_AGAIN: u32 = 11002;
pub const WINSOCK_VERSION: u32 = 514;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WSACOMPLETION {
    pub Type: WSACOMPLETIONTYPE,
    pub Parameters: WSACOMPLETION_0,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for WSACOMPLETION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WSACOMPLETION_0 {
    pub WindowMessage: WSACOMPLETION_0_0,
    pub Event: WSACOMPLETION_0_1,
    pub Apc: WSACOMPLETION_0_2,
    pub Port: WSACOMPLETION_0_3,
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for WSACOMPLETION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_0 {
    pub hWnd: super::windef::HWND,
    pub uMsg: u32,
    pub context: super::minwindef::WPARAM,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_1 {
    pub lpOverlapped: LPWSAOVERLAPPED,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WSACOMPLETION_0_2 {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub lpfnCompletionProc: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACOMPLETION_0_3 {
    pub lpOverlapped: LPWSAOVERLAPPED,
    pub hPort: super::winnt::HANDLE,
    pub Key: usize,
}
pub type WSACOMPLETIONTYPE = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut i8,
}
#[cfg(target_arch = "x86")]
impl Default for WSADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSADATA {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut i8,
    pub szDescription: [i8; 257],
    pub szSystemStatus: [i8; 129],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for WSADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSADESCRIPTION_LEN: u32 = 256;
pub type WSAECOMPARATOR = i32;
pub type WSAESETSERVICEOP = i32;
pub type WSANAMESPACE_INFO = WSANAMESPACE_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOA {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: u32,
    pub fActive: windows_core::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: windows_core::PSTR,
}
#[cfg(feature = "wtypesbase")]
pub type WSANAMESPACE_INFOEX = WSANAMESPACE_INFOEXA;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOEXA {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: u32,
    pub fActive: windows_core::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: windows_core::PSTR,
    pub ProviderSpecific: super::wtypesbase::BLOB,
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOEXW {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: u32,
    pub fActive: windows_core::BOOL,
    pub dwVersion: u32,
    pub lpszIdentifier: windows_core::PWSTR,
    pub ProviderSpecific: super::wtypesbase::BLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSANAMESPACE_INFOW {
    pub NSProviderId: windows_core::GUID,
    pub dwNameSpace: u32,
    pub fActive: windows_core::BOOL,
    pub dwVersion: u32,
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
pub const WSANO_ADDRESS: u32 = 11004;
pub type WSANSCLASSINFO = WSANSCLASSINFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSANSCLASSINFOA {
    pub lpszName: windows_core::PSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut core::ffi::c_void,
}
impl Default for WSANSCLASSINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSANSCLASSINFOW {
    pub lpszName: windows_core::PWSTR,
    pub dwNameSpace: u32,
    pub dwValueType: u32,
    pub dwValueSize: u32,
    pub lpValue: *mut core::ffi::c_void,
}
impl Default for WSANSCLASSINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAPOLLFD {
    pub fd: SOCKET,
    pub events: i16,
    pub revents: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOLCHAIN {
    pub ChainLen: i32,
    pub ChainEntries: [u32; 7],
}
impl Default for WSAPROTOCOLCHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSAPROTOCOL_INFO = WSAPROTOCOL_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOL_INFOA {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: windows_core::GUID,
    pub dwCatalogEntryId: u32,
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
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [i8; 256],
}
impl Default for WSAPROTOCOL_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPROTOCOL_INFOW {
    pub dwServiceFlags1: u32,
    pub dwServiceFlags2: u32,
    pub dwServiceFlags3: u32,
    pub dwServiceFlags4: u32,
    pub dwProviderFlags: u32,
    pub ProviderId: windows_core::GUID,
    pub dwCatalogEntryId: u32,
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
    pub dwMessageSize: u32,
    pub dwProviderReserved: u32,
    pub szProtocol: [u16; 256],
}
impl Default for WSAPROTOCOL_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSAPROTOCOL_LEN: u32 = 255;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type WSAQUERYSET = WSAQUERYSETA;
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
pub type WSAQUERYSET2 = WSAQUERYSET2A;
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSET2A {
    pub dwSize: u32,
    pub lpszServiceInstanceName: windows_core::PSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: super::guiddef::LPGUID,
    pub lpszContext: windows_core::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: super::ws2::LPCSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: super::wtypesbase::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSET2W {
    pub dwSize: u32,
    pub lpszServiceInstanceName: windows_core::PWSTR,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: super::guiddef::LPGUID,
    pub lpszContext: windows_core::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: super::ws2::LPCSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: super::wtypesbase::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSETA {
    pub dwSize: u32,
    pub lpszServiceInstanceName: windows_core::PSTR,
    pub lpServiceClassId: super::guiddef::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: super::guiddef::LPGUID,
    pub lpszContext: windows_core::PSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: super::ws2::LPCSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: super::wtypesbase::LPBLOB,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "ws2", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAQUERYSETW {
    pub dwSize: u32,
    pub lpszServiceInstanceName: windows_core::PWSTR,
    pub lpServiceClassId: super::guiddef::LPGUID,
    pub lpVersion: LPWSAVERSION,
    pub lpszComment: windows_core::PWSTR,
    pub dwNameSpace: u32,
    pub lpNSProviderId: super::guiddef::LPGUID,
    pub lpszContext: windows_core::PWSTR,
    pub dwNumberOfProtocols: u32,
    pub lpafpProtocols: LPAFPROTOCOLS,
    pub lpszQueryString: windows_core::PWSTR,
    pub dwNumberOfCsAddrs: u32,
    pub lpcsaBuffer: super::ws2::LPCSADDR_INFO,
    pub dwOutputFlags: u32,
    pub lpBlob: super::wtypesbase::LPBLOB,
}
#[cfg(feature = "guiddef")]
pub type WSASERVICECLASSINFO = WSASERVICECLASSINFOA;
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSASERVICECLASSINFOA {
    pub lpServiceClassId: super::guiddef::LPGUID,
    pub lpszServiceClassName: windows_core::PSTR,
    pub dwCount: u32,
    pub lpClassInfos: LPWSANSCLASSINFOA,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSASERVICECLASSINFOW {
    pub lpServiceClassId: super::guiddef::LPGUID,
    pub lpszServiceClassName: windows_core::PWSTR,
    pub dwCount: u32,
    pub lpClassInfos: LPWSANSCLASSINFOW,
}
pub const WSASYS_STATUS_LEN: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAVERSION {
    pub dwVersion: u32,
    pub ecHow: WSAECOMPARATOR,
}
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: u32 = 64;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: u32 = 4;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: u32 = 2;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: u32 = 16;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: u32 = 8;
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 128;
pub const WSA_FLAG_OVERLAPPED: u32 = 1;
pub const WSA_FLAG_REGISTERED_IO: u32 = 256;
pub const WSA_INFINITE: i32 = -1;
pub const WSA_INVALID_HANDLE: u32 = 6;
pub const WSA_INVALID_PARAMETER: u32 = 87;
pub const WSA_IO_INCOMPLETE: u32 = 996;
pub const WSA_IO_PENDING: u32 = 997;
pub const WSA_MAXIMUM_WAIT_EVENTS: u32 = 64;
pub const WSA_NOT_ENOUGH_MEMORY: u32 = 8;
pub const WSA_OPERATION_ABORTED: u32 = 995;
pub const WSA_WAIT_EVENT_0: u32 = 0;
pub const WSA_WAIT_FAILED: i32 = -1;
pub const WSA_WAIT_IO_COMPLETION: u32 = 192;
pub const WSA_WAIT_TIMEOUT: u32 = 258;
pub const XP1_CONNECTIONLESS: u32 = 1;
pub const XP1_CONNECT_DATA: u32 = 128;
pub const XP1_DISCONNECT_DATA: u32 = 256;
pub const XP1_EXPEDITED_DATA: u32 = 64;
pub const XP1_GRACEFUL_CLOSE: u32 = 32;
pub const XP1_GUARANTEED_DELIVERY: u32 = 2;
pub const XP1_GUARANTEED_ORDER: u32 = 4;
pub const XP1_IFS_HANDLES: u32 = 131072;
pub const XP1_INTERRUPT: u32 = 16384;
pub const XP1_MESSAGE_ORIENTED: u32 = 8;
pub const XP1_MULTIPOINT_CONTROL_PLANE: u32 = 2048;
pub const XP1_MULTIPOINT_DATA_PLANE: u32 = 4096;
pub const XP1_PARTIAL_MESSAGE: u32 = 262144;
pub const XP1_PSEUDO_STREAM: u32 = 16;
pub const XP1_QOS_SUPPORTED: u32 = 8192;
pub const XP1_SAN_SUPPORT_SDP: u32 = 524288;
pub const XP1_SUPPORT_BROADCAST: u32 = 512;
pub const XP1_SUPPORT_MULTIPOINT: u32 = 1024;
pub const XP1_UNI_RECV: u32 = 65536;
pub const XP1_UNI_SEND: u32 = 32768;
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct hostent {
    pub h_name: *mut i8,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i16,
    pub h_length: i16,
    pub h_addr_list: *mut *mut i8,
}
impl Default for hostent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct linger {
    pub l_onoff: u_short,
    pub l_linger: u_short,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct netent {
    pub n_name: *mut i8,
    pub n_aliases: *mut *mut i8,
    pub n_addrtype: i16,
    pub n_net: u_long,
}
impl Default for netent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct protoent {
    pub p_name: *mut i8,
    pub p_aliases: *mut *mut i8,
    pub p_proto: i16,
}
impl Default for protoent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct servent {
    pub s_name: *mut i8,
    pub s_aliases: *mut *mut i8,
    pub s_port: i16,
    pub s_proto: *mut i8,
}
#[cfg(target_arch = "x86")]
impl Default for servent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct servent {
    pub s_name: *mut i8,
    pub s_aliases: *mut *mut i8,
    pub s_proto: *mut i8,
    pub s_port: i16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for servent {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct u_char(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct u_int(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct u_int64(pub u64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct u_long(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct u_short(pub u16);
