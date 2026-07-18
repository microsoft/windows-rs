#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn AcceptEx(slistensocket: super::SOCKET, sacceptsocket: super::SOCKET, lpoutputbuffer: *mut core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("mswsock.dll" "system" fn AcceptEx(slistensocket : super::SOCKET, sacceptsocket : super::SOCKET, lpoutputbuffer : *mut core::ffi::c_void, dwreceivedatalength : u32, dwlocaladdresslength : u32, dwremoteaddresslength : u32, lpdwbytesreceived : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { AcceptEx(slistensocket, sacceptsocket, lpoutputbuffer as _, dwreceivedatalength, dwlocaladdresslength, dwremoteaddresslength, lpdwbytesreceived as _, lpoverlapped as _) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn GetAcceptExSockaddrs(lpoutputbuffer: *const core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut super::SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut super::SOCKADDR, remotesockaddrlength: *mut i32) {
    windows_core::link!("mswsock.dll" "system" fn GetAcceptExSockaddrs(lpoutputbuffer : *const core::ffi::c_void, dwreceivedatalength : u32, dwlocaladdresslength : u32, dwremoteaddresslength : u32, localsockaddr : *mut *mut super::SOCKADDR, localsockaddrlength : *mut i32, remotesockaddr : *mut *mut super::SOCKADDR, remotesockaddrlength : *mut i32));
    unsafe { GetAcceptExSockaddrs(lpoutputbuffer, dwreceivedatalength, dwlocaladdresslength, dwremoteaddresslength, localsockaddr as _, localsockaddrlength as _, remotesockaddr as _, remotesockaddrlength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn TransmitFile(hsocket: super::SOCKET, hfile: super::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: Option<*mut super::OVERLAPPED>, lptransmitbuffers: Option<*const TRANSMIT_FILE_BUFFERS>, dwreserved: u32) -> windows_core::BOOL {
    windows_core::link!("mswsock.dll" "system" fn TransmitFile(hsocket : super::SOCKET, hfile : super::HANDLE, nnumberofbytestowrite : u32, nnumberofbytespersend : u32, lpoverlapped : *mut super::OVERLAPPED, lptransmitbuffers : *const TRANSMIT_FILE_BUFFERS, dwreserved : u32) -> windows_core::BOOL);
    unsafe { TransmitFile(hsocket, hfile, nnumberofbytestowrite, nnumberofbytespersend, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lptransmitbuffers.unwrap_or(core::mem::zeroed()) as _, dwreserved) }
}
#[cfg(feature = "winsock2")]
#[inline]
pub unsafe fn WSARecvEx(s: super::SOCKET, buf: &mut [u8], flags: *mut i32) -> i32 {
    windows_core::link!("mswsock.dll" "system" fn WSARecvEx(s : super::SOCKET, buf : *mut i8, len : i32, flags : *mut i32) -> i32);
    unsafe { WSARecvEx(s, core::mem::transmute(buf.as_mut_ptr()), buf.len().try_into().unwrap(), flags as _) }
}
pub const DE_REUSE_SOCKET: u32 = 2;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
pub type LPFN_ACCEPTEX = Option<unsafe extern "system" fn(slistensocket: super::SOCKET, sacceptsocket: super::SOCKET, lpoutputbuffer: *mut core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_CONNECTEX = Option<unsafe extern "system" fn(s: super::SOCKET, name: *const super::SOCKADDR, namelen: i32, lpsendbuffer: *const core::ffi::c_void, dwsenddatalength: u32, lpdwbytessent: *mut u32, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
pub type LPFN_DISCONNECTEX = Option<unsafe extern "system" fn(s: super::SOCKET, lpoverlapped: *mut super::OVERLAPPED, dwflags: u32, dwreserved: u32) -> windows_core::BOOL>;
#[cfg(feature = "ws2")]
pub type LPFN_GETACCEPTEXSOCKADDRS = Option<unsafe extern "system" fn(lpoutputbuffer: *const core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut super::SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut super::SOCKADDR, remotesockaddrlength: *mut i32)>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = Option<unsafe extern "system" fn(cq: *const super::RIO_CQ_t)>;
#[cfg(all(feature = "mswsockdef", feature = "winnt"))]
pub type LPFN_RIOCREATECOMPLETIONQUEUE = Option<unsafe extern "system" fn(queuesize: u32, notificationcompletion: *const RIO_NOTIFICATION_COMPLETION) -> super::RIO_CQ>;
#[cfg(all(feature = "mswsockdef", feature = "winsock2"))]
pub type LPFN_RIOCREATEREQUESTQUEUE = Option<unsafe extern "system" fn(socket: super::SOCKET, maxoutstandingreceive: u32, maxreceivedatabuffers: u32, maxoutstandingsend: u32, maxsenddatabuffers: u32, receivecq: *const super::RIO_CQ_t, sendcq: *const super::RIO_CQ_t, socketcontext: *const core::ffi::c_void) -> super::RIO_RQ>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIODEQUEUECOMPLETION = Option<unsafe extern "system" fn(cq: *const super::RIO_CQ_t, array: *mut super::RIORESULT, arraysize: u32) -> u32>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIODEREGISTERBUFFER = Option<unsafe extern "system" fn(bufferid: *const super::RIO_BUFFERID_t)>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIONOTIFY = Option<unsafe extern "system" fn(cq: *const super::RIO_CQ_t) -> i32>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIORECEIVE = Option<unsafe extern "system" fn(socketqueue: *const super::RIO_RQ_t, pdata: *const super::RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIORECEIVEEX = Option<unsafe extern "system" fn(socketqueue: *const super::RIO_RQ_t, pdata: *const super::RIO_BUF, databuffercount: u32, plocaladdress: *const super::RIO_BUF, premoteaddress: *const super::RIO_BUF, pcontrolcontext: *const super::RIO_BUF, pflags: *const super::RIO_BUF, flags: u32, requestcontext: *const core::ffi::c_void) -> i32>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIOREGISTERBUFFER = Option<unsafe extern "system" fn(databuffer: *const i8, datalength: u32) -> super::RIO_BUFFERID>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIORESIZECOMPLETIONQUEUE = Option<unsafe extern "system" fn(cq: *const super::RIO_CQ_t, queuesize: u32) -> windows_core::BOOL>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIORESIZEREQUESTQUEUE = Option<unsafe extern "system" fn(rq: *const super::RIO_RQ_t, maxoutstandingreceive: u32, maxoutstandingsend: u32) -> windows_core::BOOL>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIOSEND = Option<unsafe extern "system" fn(socketqueue: *const super::RIO_RQ_t, pdata: *const super::RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIOSENDEX = Option<unsafe extern "system" fn(socketqueue: *const super::RIO_RQ_t, pdata: *const super::RIO_BUF, databuffercount: u32, plocaladdress: *const super::RIO_BUF, premoteaddress: *const super::RIO_BUF, pcontrolcontext: *const super::RIO_BUF, pflags: *const super::RIO_BUF, flags: u32, requestcontext: *const core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
pub type LPFN_TRANSMITFILE = Option<unsafe extern "system" fn(hsocket: super::SOCKET, hfile: super::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
pub type LPFN_TRANSMITPACKETS = Option<unsafe extern "system" fn(hsocket: super::SOCKET, lppacketarray: *const TRANSMIT_PACKETS_ELEMENT, nelementcount: u32, nsendsize: u32, lpoverlapped: *mut super::OVERLAPPED, dwflags: u32) -> windows_core::BOOL>;
#[cfg(feature = "winsock2")]
pub type LPFN_WSAPOLL = Option<unsafe extern "system" fn(fdarray: *mut super::WSAPOLLFD, nfds: u32, timeout: i32) -> i32>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_WSARECVMSG = Option<unsafe extern "system" fn(s: super::SOCKET, lpmsg: *mut super::WSAMSG, lpdwnumberofbytesrecvd: *mut u32, lpoverlapped: *mut super::OVERLAPPED, lpcompletionroutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32>;
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_WSASENDMSG = Option<unsafe extern "system" fn(s: super::SOCKET, lpmsg: *const super::WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::OVERLAPPED, lpcompletionroutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32>;
pub type LPNLA_BLOB = *mut NLA_BLOB;
pub type LPTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
#[cfg(feature = "winnt")]
pub type LPTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
#[cfg(feature = "winsock2")]
pub type LPWSAPOLLDATA = *mut WSAPOLLDATA;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPWSASENDMSG = *mut WSASENDMSG;
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = 2;
pub const NLA_ALLUSERS_NETWORK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NLA_BLOB {
    pub header: NLA_BLOB_0,
    pub data: NLA_BLOB_1,
}
impl Default for NLA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_0 {
    pub r#type: NLA_BLOB_DATA_TYPE,
    pub dwSize: u32,
    pub nextOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NLA_BLOB_1 {
    pub rawData: [i8; 1],
    pub interfaceData: NLA_BLOB_1_0,
    pub locationData: NLA_BLOB_1_1,
    pub connectivity: NLA_BLOB_1_2,
    pub ICS: NLA_BLOB_1_3,
}
impl Default for NLA_BLOB_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_0 {
    pub dwType: u32,
    pub dwSpeed: u32,
    pub adapterName: [i8; 1],
}
impl Default for NLA_BLOB_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_1 {
    pub information: [i8; 1],
}
impl Default for NLA_BLOB_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_1_2 {
    pub r#type: NLA_CONNECTIVITY_TYPE,
    pub internet: NLA_INTERNET,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_1_3 {
    pub remote: NLA_BLOB_1_3_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_3_0 {
    pub speed: u32,
    pub r#type: u32,
    pub state: u32,
    pub machineName: [u16; 256],
    pub sharedAdapterName: [u16; 256],
}
impl Default for NLA_BLOB_1_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NLA_BLOB_DATA_TYPE = i32;
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = 3;
pub type NLA_CONNECTIVITY_TYPE = i32;
pub const NLA_FRIENDLY_NAME: u32 = 2;
pub const NLA_ICS: NLA_BLOB_DATA_TYPE = 4;
pub const NLA_INTERFACE: NLA_BLOB_DATA_TYPE = 1;
pub type NLA_INTERNET = i32;
pub const NLA_INTERNET_NO: NLA_INTERNET = 1;
pub const NLA_INTERNET_UNKNOWN: NLA_INTERNET = 0;
pub const NLA_INTERNET_YES: NLA_INTERNET = 2;
pub const NLA_NETWORK_AD_HOC: NLA_CONNECTIVITY_TYPE = 0;
pub const NLA_NETWORK_MANAGED: NLA_CONNECTIVITY_TYPE = 1;
pub const NLA_NETWORK_UNKNOWN: NLA_CONNECTIVITY_TYPE = 3;
pub const NLA_NETWORK_UNMANAGED: NLA_CONNECTIVITY_TYPE = 2;
pub const NLA_RAW_DATA: NLA_BLOB_DATA_TYPE = 0;
pub type PNLA_BLOB = *mut NLA_BLOB;
pub type PNLA_BLOB_DATA_TYPE = *mut NLA_BLOB_DATA_TYPE;
pub type PNLA_CONNECTIVITY_TYPE = *mut NLA_CONNECTIVITY_TYPE;
pub type PNLA_INTERNET = *mut NLA_INTERNET;
#[cfg(all(feature = "mswsockdef", feature = "winnt", feature = "winsock2"))]
pub type PRIO_EXTENSION_FUNCTION_TABLE = *mut RIO_EXTENSION_FUNCTION_TABLE;
#[cfg(feature = "winnt")]
pub type PRIO_NOTIFICATION_COMPLETION = *mut RIO_NOTIFICATION_COMPLETION;
pub type PRIO_NOTIFICATION_COMPLETION_TYPE = *mut RIO_NOTIFICATION_COMPLETION_TYPE;
pub type PTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
#[cfg(feature = "winnt")]
pub type PTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "mswsockdef", feature = "winnt", feature = "winsock2"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct RIO_EXTENSION_FUNCTION_TABLE {
    pub cbSize: u32,
    pub RIOReceive: LPFN_RIORECEIVE,
    pub RIOReceiveEx: LPFN_RIORECEIVEEX,
    pub RIOSend: LPFN_RIOSEND,
    pub RIOSendEx: LPFN_RIOSENDEX,
    pub RIOCloseCompletionQueue: LPFN_RIOCLOSECOMPLETIONQUEUE,
    pub RIOCreateCompletionQueue: LPFN_RIOCREATECOMPLETIONQUEUE,
    pub RIOCreateRequestQueue: LPFN_RIOCREATEREQUESTQUEUE,
    pub RIODequeueCompletion: LPFN_RIODEQUEUECOMPLETION,
    pub RIODeregisterBuffer: LPFN_RIODEREGISTERBUFFER,
    pub RIONotify: LPFN_RIONOTIFY,
    pub RIORegisterBuffer: LPFN_RIOREGISTERBUFFER,
    pub RIOResizeCompletionQueue: LPFN_RIORESIZECOMPLETIONQUEUE,
    pub RIOResizeRequestQueue: LPFN_RIORESIZEREQUESTQUEUE,
}
pub const RIO_IOCP_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct RIO_NOTIFICATION_COMPLETION {
    pub Type: RIO_NOTIFICATION_COMPLETION_TYPE,
    pub Anonymous: RIO_NOTIFICATION_COMPLETION_0,
}
#[cfg(feature = "winnt")]
impl Default for RIO_NOTIFICATION_COMPLETION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union RIO_NOTIFICATION_COMPLETION_0 {
    pub Event: RIO_NOTIFICATION_COMPLETION_0_0,
    pub Iocp: RIO_NOTIFICATION_COMPLETION_0_1,
}
#[cfg(feature = "winnt")]
impl Default for RIO_NOTIFICATION_COMPLETION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RIO_NOTIFICATION_COMPLETION_0_0 {
    pub EventHandle: super::HANDLE,
    pub NotifyReset: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RIO_NOTIFICATION_COMPLETION_0_1 {
    pub IocpHandle: super::HANDLE,
    pub CompletionKey: *mut core::ffi::c_void,
    pub Overlapped: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for RIO_NOTIFICATION_COMPLETION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RIO_NOTIFICATION_COMPLETION_TYPE = i32;
pub const SIO_BASE_HANDLE: u32 = 1207959586;
pub const SIO_BSP_HANDLE: u32 = 1207959579;
pub const SIO_BSP_HANDLE_POLL: u32 = 1207959581;
pub const SIO_BSP_HANDLE_SELECT: u32 = 1207959580;
pub const SIO_EXT_POLL: i32 = -939524065;
pub const SIO_EXT_SELECT: i32 = -939524066;
pub const SIO_EXT_SENDMSG: i32 = -939524064;
pub const SIO_UDP_CONNRESET: i32 = -1744830452;
pub const SIO_UDP_NETRESET: i32 = -1744830449;
pub const SO_CONNDATA: u32 = 28672;
pub const SO_CONNDATALEN: u32 = 28676;
pub const SO_CONNECT_TIME: u32 = 28684;
pub const SO_CONNOPT: u32 = 28673;
pub const SO_CONNOPTLEN: u32 = 28677;
pub const SO_DISCDATA: u32 = 28674;
pub const SO_DISCDATALEN: u32 = 28678;
pub const SO_DISCOPT: u32 = 28675;
pub const SO_DISCOPTLEN: u32 = 28679;
pub const SO_MAXDG: u32 = 28681;
pub const SO_MAXPATHDG: u32 = 28682;
pub const SO_OPENTYPE: u32 = 28680;
pub const SO_SYNCHRONOUS_ALERT: u32 = 16;
pub const SO_SYNCHRONOUS_NONALERT: u32 = 32;
pub const SO_UPDATE_ACCEPT_CONTEXT: u32 = 28683;
pub const SO_UPDATE_CONNECT_CONTEXT: u32 = 28688;
pub const TCP_BSDURGENT: u32 = 28672;
pub const TF_DISCONNECT: u32 = 1;
pub const TF_REUSE_SOCKET: u32 = 2;
pub const TF_USE_DEFAULT_WORKER: u32 = 0;
pub const TF_USE_KERNEL_APC: u32 = 32;
pub const TF_USE_SYSTEM_THREAD: u32 = 16;
pub const TF_WRITE_BEHIND: u32 = 4;
pub const TP_DISCONNECT: u32 = 1;
pub const TP_ELEMENT_EOP: u32 = 4;
pub const TP_ELEMENT_FILE: u32 = 2;
pub const TP_ELEMENT_MEMORY: u32 = 1;
pub const TP_REUSE_SOCKET: u32 = 2;
pub const TP_USE_DEFAULT_WORKER: u32 = 0;
pub const TP_USE_KERNEL_APC: u32 = 32;
pub const TP_USE_SYSTEM_THREAD: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRANSMIT_FILE_BUFFERS {
    pub Head: *mut core::ffi::c_void,
    pub HeadLength: u32,
    pub Tail: *mut core::ffi::c_void,
    pub TailLength: u32,
}
impl Default for TRANSMIT_FILE_BUFFERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct TRANSMIT_PACKETS_ELEMENT {
    pub dwElFlags: u32,
    pub cLength: u32,
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0,
}
#[cfg(feature = "winnt")]
impl Default for TRANSMIT_PACKETS_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union TRANSMIT_PACKETS_ELEMENT_0 {
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0_0,
    pub pBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for TRANSMIT_PACKETS_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSMIT_PACKETS_ELEMENT_0_0 {
    pub nFileOffset: i64,
    pub hFile: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winsock2")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPOLLDATA {
    pub result: i32,
    pub fds: u32,
    pub timeout: i32,
    pub fdArray: [super::WSAPOLLFD; 0],
}
#[cfg(feature = "winsock2")]
impl Default for WSAPOLLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt", feature = "winsock2", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WSASENDMSG {
    pub lpMsg: super::LPWSAMSG,
    pub dwFlags: u32,
    pub lpNumberOfBytesSent: super::LPDWORD,
    pub lpOverlapped: super::LPWSAOVERLAPPED,
    pub lpCompletionRoutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
