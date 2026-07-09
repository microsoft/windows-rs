#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW {
    pub State: MIB_TCP_STATE,
    pub LocalAddr: super::in6addr::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::in6addr::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for MIB_TCP6ROW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW2 {
    pub LocalAddr: super::in6addr::IN6_ADDR,
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub RemoteAddr: super::in6addr::IN6_ADDR,
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub State: MIB_TCP_STATE,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[cfg(feature = "Win32_in6addr")]
impl Default for MIB_TCP6ROW2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW_OWNER_MODULE {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_TCP6ROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub ucLocalAddr: [u8; 16],
    pub dwLocalScopeId: u32,
    pub dwLocalPort: u32,
    pub ucRemoteAddr: [u8; 16],
    pub dwRemoteScopeId: u32,
    pub dwRemotePort: u32,
    pub dwState: u32,
    pub dwOwningPid: u32,
}
impl Default for MIB_TCP6ROW_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW; 1],
}
#[cfg(feature = "Win32_in6addr")]
impl Default for MIB_TCP6TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_in6addr")]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW2; 1],
}
#[cfg(feature = "Win32_in6addr")]
impl Default for MIB_TCP6TABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_MODULE; 1],
}
impl Default for MIB_TCP6TABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCP6ROW_OWNER_PID; 1],
}
impl Default for MIB_TCP6TABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MIB_TCPROW = MIB_TCPROW_LH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_TCPROW2 {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub dwOffloadState: TCP_CONNECTION_OFFLOAD_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPROW_LH {
    pub Anonymous: MIB_TCPROW_LH_0,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
impl Default for MIB_TCPROW_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_TCPROW_LH_0 {
    pub dwState: u32,
    pub State: MIB_TCP_STATE,
}
impl Default for MIB_TCPROW_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPROW_OWNER_MODULE {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
    pub liCreateTimestamp: i64,
    pub OwningModuleInfo: [u64; 16],
}
impl Default for MIB_TCPROW_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_TCPROW_OWNER_PID {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
    pub dwOwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_TCPROW_W2K {
    pub dwState: u32,
    pub dwLocalAddr: u32,
    pub dwLocalPort: u32,
    pub dwRemoteAddr: u32,
    pub dwRemotePort: u32,
}
pub type MIB_TCPSTATS = MIB_TCPSTATS_LH;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_TCPSTATS2 {
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dw64InSegs: u64,
    pub dw64OutSegs: u64,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPSTATS_LH {
    pub Anonymous: MIB_TCPSTATS_LH_0,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
impl Default for MIB_TCPSTATS_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MIB_TCPSTATS_LH_0 {
    pub dwRtoAlgorithm: u32,
    pub RtoAlgorithm: TCP_RTO_ALGORITHM,
}
impl Default for MIB_TCPSTATS_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MIB_TCPSTATS_W2K {
    pub dwRtoAlgorithm: u32,
    pub dwRtoMin: u32,
    pub dwRtoMax: u32,
    pub dwMaxConn: u32,
    pub dwActiveOpens: u32,
    pub dwPassiveOpens: u32,
    pub dwAttemptFails: u32,
    pub dwEstabResets: u32,
    pub dwCurrEstab: u32,
    pub dwInSegs: u32,
    pub dwOutSegs: u32,
    pub dwRetransSegs: u32,
    pub dwInErrs: u32,
    pub dwOutRsts: u32,
    pub dwNumConns: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW; 1],
}
impl Default for MIB_TCPTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE2 {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW2; 1],
}
impl Default for MIB_TCPTABLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE_OWNER_MODULE {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_MODULE; 1],
}
impl Default for MIB_TCPTABLE_OWNER_MODULE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub dwNumEntries: u32,
    pub table: [MIB_TCPROW_OWNER_PID; 1],
}
impl Default for MIB_TCPTABLE_OWNER_PID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MIB_TCP_MAXCONN_DYNAMIC: u32 = 4294967295;
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
#[cfg(feature = "Win32_in6addr")]
pub type PMIB_TCP6ROW = *mut MIB_TCP6ROW;
#[cfg(feature = "Win32_in6addr")]
pub type PMIB_TCP6ROW2 = *mut MIB_TCP6ROW2;
pub type PMIB_TCP6ROW_OWNER_MODULE = *mut MIB_TCP6ROW_OWNER_MODULE;
pub type PMIB_TCP6ROW_OWNER_PID = *mut MIB_TCP6ROW_OWNER_PID;
#[cfg(feature = "Win32_in6addr")]
pub type PMIB_TCP6TABLE = *mut MIB_TCP6TABLE;
#[cfg(feature = "Win32_in6addr")]
pub type PMIB_TCP6TABLE2 = *mut MIB_TCP6TABLE2;
pub type PMIB_TCP6TABLE_OWNER_MODULE = *mut MIB_TCP6TABLE_OWNER_MODULE;
pub type PMIB_TCP6TABLE_OWNER_PID = *mut MIB_TCP6TABLE_OWNER_PID;
pub type PMIB_TCPROW = *mut MIB_TCPROW_LH;
pub type PMIB_TCPROW2 = *mut MIB_TCPROW2;
pub type PMIB_TCPROW_LH = *mut MIB_TCPROW_LH;
pub type PMIB_TCPROW_OWNER_MODULE = *mut MIB_TCPROW_OWNER_MODULE;
pub type PMIB_TCPROW_OWNER_PID = *mut MIB_TCPROW_OWNER_PID;
pub type PMIB_TCPROW_W2K = *mut MIB_TCPROW_W2K;
pub type PMIB_TCPSTATS = *mut MIB_TCPSTATS_LH;
pub type PMIB_TCPSTATS2 = *mut MIB_TCPSTATS2;
pub type PMIB_TCPSTATS_LH = *mut MIB_TCPSTATS_LH;
pub type PMIB_TCPSTATS_W2K = *mut MIB_TCPSTATS_W2K;
pub type PMIB_TCPTABLE = *mut MIB_TCPTABLE;
pub type PMIB_TCPTABLE2 = *mut MIB_TCPTABLE2;
pub type PMIB_TCPTABLE_OWNER_MODULE = *mut MIB_TCPTABLE_OWNER_MODULE;
pub type PMIB_TCPTABLE_OWNER_PID = *mut MIB_TCPTABLE_OWNER_PID;
pub type PTCP_CONNECTION_OFFLOAD_STATE = *mut TCP_CONNECTION_OFFLOAD_STATE;
pub type PTCP_RTO_ALGORITHM = *mut TCP_RTO_ALGORITHM;
pub const TCPIP_OWNING_MODULE_SIZE: u32 = 16;
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
