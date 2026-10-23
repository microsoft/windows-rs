#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW {
    pub State: MIB_TCP_STATE,
    pub LocalAddr: super::IN6_ADDR,
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub RemoteAddr: super::IN6_ADDR,
    pub dwRemoteScopeId: super::DWORD,
    pub dwRemotePort: super::DWORD,
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_TCP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW2 {
    pub LocalAddr: super::IN6_ADDR,
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub RemoteAddr: super::IN6_ADDR,
    pub dwRemoteScopeId: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub State: MIB_TCP_STATE,
    pub dwOwningPid: super::DWORD,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_TCP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [super::UCHAR; 16],
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub ucRemoteAddr: [super::UCHAR; 16],
    pub dwRemoteScopeId: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub dwState: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_TCP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub ucLocalAddr: [super::UCHAR; 16],
    pub dwLocalScopeId: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub ucRemoteAddr: [super::UCHAR; 16],
    pub dwRemoteScopeId: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub dwState: super::DWORD,
    pub dwOwningPid: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIB_TCP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCP6ROW; 1],
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_TCP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE2 {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCP6ROW2; 1],
}
#[cfg(all(feature = "in6addr", feature = "shared"))]
impl Default for MIB_TCP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE_OWNER_MODULE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCP6ROW_OWNER_MODULE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_TCP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCP6ROW_OWNER_PID; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_TCP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type MIB_TCPROW = MIB_TCPROW_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCPROW2 {
    pub dwState: super::DWORD,
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_TCPROW_LH {
    pub Anonymous: MIB_TCPROW_LH_0,
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIB_TCPROW_LH_0 {
    pub dwState: super::DWORD,
    pub State: MIB_TCP_STATE,
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_TCPROW_OWNER_MODULE {
    pub dwState: super::DWORD,
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub dwOwningPid: super::DWORD,
    pub liCreateTimestamp: super::LARGE_INTEGER,
    pub OwningModuleInfo: [super::ULONGLONG; 16],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_TCPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: super::DWORD,
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
    pub dwOwningPid: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCPROW_W2K {
    pub dwState: super::DWORD,
    pub dwLocalAddr: super::DWORD,
    pub dwLocalPort: super::DWORD,
    pub dwRemoteAddr: super::DWORD,
    pub dwRemotePort: super::DWORD,
}
#[cfg(feature = "shared")]
pub type MIB_TCPSTATS = MIB_TCPSTATS_LH;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCPSTATS2 {
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
    pub dwRtoMin: super::DWORD,
    pub dwRtoMax: super::DWORD,
    pub dwMaxConn: super::DWORD,
    pub dwActiveOpens: super::DWORD,
    pub dwPassiveOpens: super::DWORD,
    pub dwAttemptFails: super::DWORD,
    pub dwEstabResets: super::DWORD,
    pub dwCurrEstab: super::DWORD,
    pub dw64InSegs: super::DWORD64,
    pub dw64OutSegs: super::DWORD64,
    pub dwRetransSegs: super::DWORD,
    pub dwInErrs: super::DWORD,
    pub dwOutRsts: super::DWORD,
    pub dwNumConns: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_TCPSTATS_LH {
    pub Anonymous: MIB_TCPSTATS_LH_0,
    pub dwRtoMin: super::DWORD,
    pub dwRtoMax: super::DWORD,
    pub dwMaxConn: super::DWORD,
    pub dwActiveOpens: super::DWORD,
    pub dwPassiveOpens: super::DWORD,
    pub dwAttemptFails: super::DWORD,
    pub dwEstabResets: super::DWORD,
    pub dwCurrEstab: super::DWORD,
    pub dwInSegs: super::DWORD,
    pub dwOutSegs: super::DWORD,
    pub dwRetransSegs: super::DWORD,
    pub dwInErrs: super::DWORD,
    pub dwOutRsts: super::DWORD,
    pub dwNumConns: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MIB_TCPSTATS_LH_0 {
    pub dwRtoAlgorithm: super::DWORD,
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MIB_TCPSTATS_W2K {
    pub dwRtoAlgorithm: super::DWORD,
    pub dwRtoMin: super::DWORD,
    pub dwRtoMax: super::DWORD,
    pub dwMaxConn: super::DWORD,
    pub dwActiveOpens: super::DWORD,
    pub dwPassiveOpens: super::DWORD,
    pub dwAttemptFails: super::DWORD,
    pub dwEstabResets: super::DWORD,
    pub dwCurrEstab: super::DWORD,
    pub dwInSegs: super::DWORD,
    pub dwOutSegs: super::DWORD,
    pub dwRetransSegs: super::DWORD,
    pub dwInErrs: super::DWORD,
    pub dwOutRsts: super::DWORD,
    pub dwNumConns: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCPROW; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_TCPTABLE2 {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCPROW2; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE_OWNER_MODULE {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCPROW_OWNER_MODULE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MIB_TCPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: super::DWORD,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}
#[cfg(feature = "shared")]
impl Default for MIB_TCPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const MIB_TCP_MAXCONN_DYNAMIC: super::ULONG = 4294967295;
pub const MIB_TCP_RTO_CONSTANT: TCP_RTO_ALGORITHM = 2;
pub const MIB_TCP_RTO_OTHER: TCP_RTO_ALGORITHM = 1;
pub const MIB_TCP_RTO_RSRE: TCP_RTO_ALGORITHM = 3;
pub const MIB_TCP_RTO_VANJ: TCP_RTO_ALGORITHM = 4;
pub type MIB_TCP_STATE = i32;
pub const MIB_TCP_STATE_CLOSED: MIB_TCP_STATE = 1;
pub const MIB_TCP_STATE_CLOSE_WAIT: MIB_TCP_STATE = 8;
pub const MIB_TCP_STATE_CLOSING: MIB_TCP_STATE = 9;
pub const MIB_TCP_STATE_DELETE_TCB: MIB_TCP_STATE = 12;
pub const MIB_TCP_STATE_ESTAB: MIB_TCP_STATE = 5;
pub const MIB_TCP_STATE_FIN_WAIT1: MIB_TCP_STATE = 6;
pub const MIB_TCP_STATE_FIN_WAIT2: MIB_TCP_STATE = 7;
pub const MIB_TCP_STATE_LAST_ACK: MIB_TCP_STATE = 10;
pub const MIB_TCP_STATE_LISTEN: MIB_TCP_STATE = 2;
pub const MIB_TCP_STATE_RESERVED: MIB_TCP_STATE = 100;
pub const MIB_TCP_STATE_SYN_RCVD: MIB_TCP_STATE = 4;
pub const MIB_TCP_STATE_SYN_SENT: MIB_TCP_STATE = 3;
pub const MIB_TCP_STATE_TIME_WAIT: MIB_TCP_STATE = 11;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_TCP6ROW = *mut MIB_TCP6ROW;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_TCP6ROW2 = *mut MIB_TCP6ROW2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_TCP6ROW_OWNER_MODULE = *mut MIB_TCP6ROW_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_TCP6ROW_OWNER_PID = *mut MIB_TCP6ROW_OWNER_PID;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_TCP6TABLE = *mut MIB_TCP6TABLE;
#[cfg(all(feature = "in6addr", feature = "shared"))]
pub type PMIB_TCP6TABLE2 = *mut MIB_TCP6TABLE2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_TCP6TABLE_OWNER_MODULE = *mut MIB_TCP6TABLE_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_TCP6TABLE_OWNER_PID = *mut MIB_TCP6TABLE_OWNER_PID;
#[cfg(feature = "shared")]
pub type PMIB_TCPROW = *mut MIB_TCPROW_LH;
#[cfg(feature = "shared")]
pub type PMIB_TCPROW2 = *mut MIB_TCPROW2;
#[cfg(feature = "shared")]
pub type PMIB_TCPROW_LH = *mut MIB_TCPROW_LH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_TCPROW_OWNER_MODULE = *mut MIB_TCPROW_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_TCPROW_OWNER_PID = *mut MIB_TCPROW_OWNER_PID;
#[cfg(feature = "shared")]
pub type PMIB_TCPROW_W2K = *mut MIB_TCPROW_W2K;
#[cfg(feature = "shared")]
pub type PMIB_TCPSTATS = *mut MIB_TCPSTATS_LH;
#[cfg(feature = "shared")]
pub type PMIB_TCPSTATS2 = *mut MIB_TCPSTATS2;
#[cfg(feature = "shared")]
pub type PMIB_TCPSTATS_LH = *mut MIB_TCPSTATS_LH;
#[cfg(feature = "shared")]
pub type PMIB_TCPSTATS_W2K = *mut MIB_TCPSTATS_W2K;
#[cfg(feature = "shared")]
pub type PMIB_TCPTABLE = *mut MIB_TCPTABLE;
#[cfg(feature = "shared")]
pub type PMIB_TCPTABLE2 = *mut MIB_TCPTABLE2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMIB_TCPTABLE_OWNER_MODULE = *mut MIB_TCPTABLE_OWNER_MODULE;
#[cfg(feature = "shared")]
pub type PMIB_TCPTABLE_OWNER_PID = *mut MIB_TCPTABLE_OWNER_PID;
pub type PTCP_CONNECTION_OFFLOAD_STATE = *mut TCP_CONNECTION_OFFLOAD_STATE;
pub type PTCP_RTO_ALGORITHM = *mut TCP_RTO_ALGORITHM;
pub type TCP_CONNECTION_OFFLOAD_STATE = i32;
pub type TCP_RTO_ALGORITHM = i32;
pub const TcpConnectionOffloadStateInHost: TCP_CONNECTION_OFFLOAD_STATE = 0;
pub const TcpConnectionOffloadStateMax: TCP_CONNECTION_OFFLOAD_STATE = 4;
pub const TcpConnectionOffloadStateOffloaded: TCP_CONNECTION_OFFLOAD_STATE = 2;
pub const TcpConnectionOffloadStateOffloading: TCP_CONNECTION_OFFLOAD_STATE = 1;
pub const TcpConnectionOffloadStateUploading: TCP_CONNECTION_OFFLOAD_STATE = 3;
pub const TcpRtoAlgorithmConstant: TCP_RTO_ALGORITHM = 2;
pub const TcpRtoAlgorithmOther: TCP_RTO_ALGORITHM = 1;
pub const TcpRtoAlgorithmRsre: TCP_RTO_ALGORITHM = 3;
pub const TcpRtoAlgorithmVanj: TCP_RTO_ALGORITHM = 4;
