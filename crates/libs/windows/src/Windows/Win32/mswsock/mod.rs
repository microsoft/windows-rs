#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn AcceptEx(slistensocket: super::SOCKET, sacceptsocket: super::SOCKET, lpoutputbuffer: super::PVOID, dwreceivedatalength: super::DWORD, dwlocaladdresslength: super::DWORD, dwremoteaddresslength: super::DWORD, lpdwbytesreceived: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("mswsock.dll" "system" fn AcceptEx(slistensocket : super::SOCKET, sacceptsocket : super::SOCKET, lpoutputbuffer : super::PVOID, dwreceivedatalength : super::DWORD, dwlocaladdresslength : super::DWORD, dwremoteaddresslength : super::DWORD, lpdwbytesreceived : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { AcceptEx(slistensocket, sacceptsocket, lpoutputbuffer as _, dwreceivedatalength, dwlocaladdresslength, dwremoteaddresslength, lpdwbytesreceived as _, lpoverlapped as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn GetAcceptExSockaddrs(lpoutputbuffer: super::PVOID, dwreceivedatalength: super::DWORD, dwlocaladdresslength: super::DWORD, dwremoteaddresslength: super::DWORD, localsockaddr: *mut *mut super::SOCKADDR, localsockaddrlength: super::LPINT, remotesockaddr: *mut *mut super::SOCKADDR, remotesockaddrlength: super::LPINT) {
    windows_core::link!("mswsock.dll" "system" fn GetAcceptExSockaddrs(lpoutputbuffer : super::PVOID, dwreceivedatalength : super::DWORD, dwlocaladdresslength : super::DWORD, dwremoteaddresslength : super::DWORD, localsockaddr : *mut *mut super::SOCKADDR, localsockaddrlength : super::LPINT, remotesockaddr : *mut *mut super::SOCKADDR, remotesockaddrlength : super::LPINT));
    unsafe { GetAcceptExSockaddrs(lpoutputbuffer, dwreceivedatalength, dwlocaladdresslength, dwremoteaddresslength, localsockaddr as _, localsockaddrlength as _, remotesockaddr as _, remotesockaddrlength as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn TransmitFile(hsocket: super::SOCKET, hfile: super::HANDLE, nnumberofbytestowrite: super::DWORD, nnumberofbytespersend: super::DWORD, lpoverlapped: Option<super::LPOVERLAPPED>, lptransmitbuffers: Option<LPTRANSMIT_FILE_BUFFERS>, dwreserved: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("mswsock.dll" "system" fn TransmitFile(hsocket : super::SOCKET, hfile : super::HANDLE, nnumberofbytestowrite : super::DWORD, nnumberofbytespersend : super::DWORD, lpoverlapped : super::LPOVERLAPPED, lptransmitbuffers : LPTRANSMIT_FILE_BUFFERS, dwreserved : super::DWORD) -> windows_core::BOOL);
    unsafe { TransmitFile(hsocket, hfile, nnumberofbytestowrite, nnumberofbytespersend, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lptransmitbuffers.unwrap_or(core::mem::zeroed()) as _, dwreserved) }
}
#[cfg(all(feature = "shared", feature = "winsock2"))]
#[inline]
pub unsafe fn WSARecvEx(s: super::SOCKET, buf: *mut super::CHAR, len: super::INT, flags: *mut super::INT) -> super::INT {
    windows_core::link!("mswsock.dll" "system" fn WSARecvEx(s : super::SOCKET, buf : *mut super::CHAR, len : super::INT, flags : *mut super::INT) -> super::INT);
    unsafe { WSARecvEx(s, buf as _, len, flags as _) }
}
pub const DE_REUSE_SOCKET: i32 = 2;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type LPFN_ACCEPTEX = Option<unsafe extern "system" fn(slistensocket: super::SOCKET, sacceptsocket: super::SOCKET, lpoutputbuffer: super::PVOID, dwreceivedatalength: super::DWORD, dwlocaladdresslength: super::DWORD, dwremoteaddresslength: super::DWORD, lpdwbytesreceived: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_CONNECTEX = Option<unsafe extern "system" fn(s: super::SOCKET, name: *const super::SOCKADDR, namelen: i32, lpsendbuffer: super::PVOID, dwsenddatalength: super::DWORD, lpdwbytessent: super::LPDWORD, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type LPFN_DISCONNECTEX = Option<unsafe extern "system" fn(s: super::SOCKET, lpoverlapped: super::LPOVERLAPPED, dwflags: super::DWORD, dwreserved: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "ws2"))]
pub type LPFN_GETACCEPTEXSOCKADDRS = Option<unsafe extern "system" fn(lpoutputbuffer: super::PVOID, dwreceivedatalength: super::DWORD, dwlocaladdresslength: super::DWORD, dwremoteaddresslength: super::DWORD, localsockaddr: *mut *mut super::SOCKADDR, localsockaddrlength: super::LPINT, remotesockaddr: *mut *mut super::SOCKADDR, remotesockaddrlength: super::LPINT)>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = Option<unsafe extern "system" fn(cq: super::RIO_CQ)>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIOCREATECOMPLETIONQUEUE = Option<unsafe extern "system" fn(queuesize: super::DWORD, notificationcompletion: PRIO_NOTIFICATION_COMPLETION) -> super::RIO_CQ>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type LPFN_RIOCREATEREQUESTQUEUE = Option<unsafe extern "system" fn(socket: super::SOCKET, maxoutstandingreceive: super::ULONG, maxreceivedatabuffers: super::ULONG, maxoutstandingsend: super::ULONG, maxsenddatabuffers: super::ULONG, receivecq: super::RIO_CQ, sendcq: super::RIO_CQ, socketcontext: super::PVOID) -> super::RIO_RQ>;
#[cfg(all(feature = "mswsockdef", feature = "shared"))]
pub type LPFN_RIODEQUEUECOMPLETION = Option<unsafe extern "system" fn(cq: super::RIO_CQ, array: super::PRIORESULT, arraysize: super::ULONG) -> super::ULONG>;
#[cfg(feature = "mswsockdef")]
pub type LPFN_RIODEREGISTERBUFFER = Option<unsafe extern "system" fn(bufferid: super::RIO_BUFFERID)>;
#[cfg(all(feature = "mswsockdef", feature = "shared"))]
pub type LPFN_RIONOTIFY = Option<unsafe extern "system" fn(cq: super::RIO_CQ) -> super::INT>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIORECEIVE = Option<unsafe extern "system" fn(socketqueue: super::RIO_RQ, pdata: super::PRIO_BUF, databuffercount: super::ULONG, flags: super::DWORD, requestcontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIORECEIVEEX = Option<unsafe extern "system" fn(socketqueue: super::RIO_RQ, pdata: super::PRIO_BUF, databuffercount: super::ULONG, plocaladdress: super::PRIO_BUF, premoteaddress: super::PRIO_BUF, pcontrolcontext: super::PRIO_BUF, pflags: super::PRIO_BUF, flags: super::DWORD, requestcontext: super::PVOID) -> i32>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIOREGISTERBUFFER = Option<unsafe extern "system" fn(databuffer: super::PCHAR, datalength: super::DWORD) -> super::RIO_BUFFERID>;
#[cfg(all(feature = "mswsockdef", feature = "shared"))]
pub type LPFN_RIORESIZECOMPLETIONQUEUE = Option<unsafe extern "system" fn(cq: super::RIO_CQ, queuesize: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "mswsockdef", feature = "shared"))]
pub type LPFN_RIORESIZEREQUESTQUEUE = Option<unsafe extern "system" fn(rq: super::RIO_RQ, maxoutstandingreceive: super::DWORD, maxoutstandingsend: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIOSEND = Option<unsafe extern "system" fn(socketqueue: super::RIO_RQ, pdata: super::PRIO_BUF, databuffercount: super::ULONG, flags: super::DWORD, requestcontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt"))]
pub type LPFN_RIOSENDEX = Option<unsafe extern "system" fn(socketqueue: super::RIO_RQ, pdata: super::PRIO_BUF, databuffercount: super::ULONG, plocaladdress: super::PRIO_BUF, premoteaddress: super::PRIO_BUF, pcontrolcontext: super::PRIO_BUF, pflags: super::PRIO_BUF, flags: super::DWORD, requestcontext: super::PVOID) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type LPFN_TRANSMITFILE = Option<unsafe extern "system" fn(hsocket: super::SOCKET, hfile: super::HANDLE, nnumberofbytestowrite: super::DWORD, nnumberofbytespersend: super::DWORD, lpoverlapped: super::LPOVERLAPPED, lptransmitbuffers: LPTRANSMIT_FILE_BUFFERS, dwreserved: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type LPFN_TRANSMITPACKETS = Option<unsafe extern "system" fn(hsocket: super::SOCKET, lppacketarray: LPTRANSMIT_PACKETS_ELEMENT, nelementcount: super::DWORD, nsendsize: super::DWORD, lpoverlapped: super::LPOVERLAPPED, dwflags: super::DWORD) -> windows_core::BOOL>;
#[cfg(all(feature = "shared", feature = "winsock2"))]
pub type LPFN_WSAPOLL = Option<unsafe extern "system" fn(fdarray: super::LPWSAPOLLFD, nfds: super::ULONG, timeout: super::INT) -> super::INT>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_WSARECVMSG = Option<unsafe extern "system" fn(s: super::SOCKET, lpmsg: super::LPWSAMSG, lpdwnumberofbytesrecvd: super::LPDWORD, lpoverlapped: super::LPWSAOVERLAPPED, lpcompletionroutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> super::INT>;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPFN_WSASENDMSG = Option<unsafe extern "system" fn(s: super::SOCKET, lpmsg: super::LPWSAMSG, dwflags: super::DWORD, lpnumberofbytessent: super::LPDWORD, lpoverlapped: super::LPWSAOVERLAPPED, lpcompletionroutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> super::INT>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPNLA_BLOB = *mut NLA_BLOB;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
#[cfg(all(feature = "shared", feature = "winsock2"))]
pub type LPWSAPOLLDATA = *mut WSAPOLLDATA;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
pub type LPWSASENDMSG = *mut WSASENDMSG;
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = 2;
pub const NLA_ALLUSERS_NETWORK: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct NLA_BLOB {
    pub header: NLA_BLOB_0,
    pub data: NLA_BLOB_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NLA_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_0 {
    pub r#type: NLA_BLOB_DATA_TYPE,
    pub dwSize: super::DWORD,
    pub nextOffset: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union NLA_BLOB_1 {
    pub rawData: [super::CHAR; 1],
    pub interfaceData: NLA_BLOB_1_0,
    pub locationData: NLA_BLOB_1_1,
    pub connectivity: NLA_BLOB_1_2,
    pub ICS: NLA_BLOB_1_3,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NLA_BLOB_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_0 {
    pub dwType: super::DWORD,
    pub dwSpeed: super::DWORD,
    pub adapterName: [super::CHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NLA_BLOB_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_1 {
    pub information: [super::CHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NLA_BLOB_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_1_2 {
    pub r#type: NLA_CONNECTIVITY_TYPE,
    pub internet: NLA_INTERNET,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NLA_BLOB_1_3 {
    pub remote: NLA_BLOB_1_3_0,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NLA_BLOB_1_3_0 {
    pub speed: super::DWORD,
    pub r#type: super::DWORD,
    pub state: super::DWORD,
    pub machineName: [super::WCHAR; 256],
    pub sharedAdapterName: [super::WCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for NLA_BLOB_1_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NLA_BLOB_DATA_TYPE = i32;
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = 3;
pub type NLA_CONNECTIVITY_TYPE = i32;
pub const NLA_FRIENDLY_NAME: i32 = 2;
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
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PNLA_BLOB = *mut NLA_BLOB;
pub type PNLA_BLOB_DATA_TYPE = *mut NLA_BLOB_DATA_TYPE;
pub type PNLA_CONNECTIVITY_TYPE = *mut NLA_CONNECTIVITY_TYPE;
pub type PNLA_INTERNET = *mut NLA_INTERNET;
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt", feature = "winsock2"))]
pub type PRIO_EXTENSION_FUNCTION_TABLE = *mut RIO_EXTENSION_FUNCTION_TABLE;
#[cfg(feature = "winnt")]
pub type PRIO_NOTIFICATION_COMPLETION = *mut RIO_NOTIFICATION_COMPLETION;
pub type PRIO_NOTIFICATION_COMPLETION_TYPE = *mut RIO_NOTIFICATION_COMPLETION_TYPE;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PTRANSMIT_FILE_BUFFERS = *mut TRANSMIT_FILE_BUFFERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTRANSMIT_PACKETS_ELEMENT = *mut TRANSMIT_PACKETS_ELEMENT;
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "mswsockdef", feature = "shared", feature = "winnt", feature = "winsock2"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct RIO_EXTENSION_FUNCTION_TABLE {
    pub cbSize: super::DWORD,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RIO_NOTIFICATION_COMPLETION_0_1 {
    pub IocpHandle: super::HANDLE,
    pub CompletionKey: super::PVOID,
    pub Overlapped: super::PVOID,
}
pub type RIO_NOTIFICATION_COMPLETION_TYPE = i32;
pub const SIO_BASE_HANDLE: i32 = 1207959586;
pub const SIO_BSP_HANDLE: i32 = 1207959579;
pub const SIO_BSP_HANDLE_POLL: i32 = 1207959581;
pub const SIO_BSP_HANDLE_SELECT: i32 = 1207959580;
pub const SIO_EXT_POLL: u32 = 3355443231;
pub const SIO_EXT_SELECT: u32 = 3355443230;
pub const SIO_EXT_SENDMSG: u32 = 3355443232;
pub const SIO_UDP_CONNRESET: u32 = 2550136844;
pub const SIO_UDP_NETRESET: u32 = 2550136847;
pub const SO_CONNDATA: i32 = 28672;
pub const SO_CONNDATALEN: i32 = 28676;
pub const SO_CONNECT_TIME: i32 = 28684;
pub const SO_CONNOPT: i32 = 28673;
pub const SO_CONNOPTLEN: i32 = 28677;
pub const SO_DISCDATA: i32 = 28674;
pub const SO_DISCDATALEN: i32 = 28678;
pub const SO_DISCOPT: i32 = 28675;
pub const SO_DISCOPTLEN: i32 = 28679;
pub const SO_MAXDG: i32 = 28681;
pub const SO_MAXPATHDG: i32 = 28682;
pub const SO_OPENTYPE: i32 = 28680;
pub const SO_SYNCHRONOUS_ALERT: i32 = 16;
pub const SO_SYNCHRONOUS_NONALERT: i32 = 32;
pub const SO_UPDATE_ACCEPT_CONTEXT: i32 = 28683;
pub const SO_UPDATE_CONNECT_CONTEXT: i32 = 28688;
pub const TCP_BSDURGENT: i32 = 28672;
pub const TF_DISCONNECT: i32 = 1;
pub const TF_REUSE_SOCKET: i32 = 2;
pub const TF_USE_DEFAULT_WORKER: i32 = 0;
pub const TF_USE_KERNEL_APC: i32 = 32;
pub const TF_USE_SYSTEM_THREAD: i32 = 16;
pub const TF_WRITE_BEHIND: i32 = 4;
pub const TP_DISCONNECT: i32 = 1;
pub const TP_ELEMENT_EOP: i32 = 4;
pub const TP_ELEMENT_FILE: i32 = 2;
pub const TP_ELEMENT_MEMORY: i32 = 1;
pub const TP_REUSE_SOCKET: i32 = 2;
pub const TP_USE_DEFAULT_WORKER: i32 = 0;
pub const TP_USE_KERNEL_APC: i32 = 32;
pub const TP_USE_SYSTEM_THREAD: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSMIT_FILE_BUFFERS {
    pub Head: super::LPVOID,
    pub HeadLength: super::DWORD,
    pub Tail: super::LPVOID,
    pub TailLength: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TRANSMIT_PACKETS_ELEMENT {
    pub dwElFlags: super::ULONG,
    pub cLength: super::ULONG,
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TRANSMIT_PACKETS_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union TRANSMIT_PACKETS_ELEMENT_0 {
    pub Anonymous: TRANSMIT_PACKETS_ELEMENT_0_0,
    pub pBuffer: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TRANSMIT_PACKETS_ELEMENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TRANSMIT_PACKETS_ELEMENT_0_0 {
    pub nFileOffset: super::LARGE_INTEGER,
    pub hFile: super::HANDLE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TRANSMIT_PACKETS_ELEMENT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winsock2"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSAPOLLDATA {
    pub result: i32,
    pub fds: super::ULONG,
    pub timeout: super::INT,
    pub fdArray: [super::WSAPOLLFD; 0],
}
#[cfg(all(feature = "shared", feature = "winsock2"))]
impl Default for WSAPOLLDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct WSASENDMSG {
    pub lpMsg: super::LPWSAMSG,
    pub dwFlags: super::DWORD,
    pub lpNumberOfBytesSent: super::LPDWORD,
    pub lpOverlapped: super::LPWSAOVERLAPPED,
    pub lpCompletionRoutine: super::LPWSAOVERLAPPED_COMPLETION_ROUTINE,
}
