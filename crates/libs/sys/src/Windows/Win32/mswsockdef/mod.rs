pub type PRIORESULT = *mut RIORESULT;
#[cfg(feature = "mswsock")]
pub type PRIO_BUF = *mut RIO_BUF;
#[cfg(feature = "mswsock")]
pub type PRIO_BUFFERID = *mut *mut super::RIO_BUFFERID_t;
pub type PRIO_CMSG_BUFFER = *mut RIO_CMSG_BUFFER;
#[cfg(feature = "mswsock")]
pub type PRIO_CQ = *mut *mut super::RIO_CQ_t;
#[cfg(feature = "mswsock")]
pub type PRIO_RQ = *mut *mut super::RIO_RQ_t;
pub type PWSA_COMPATIBILITY_BEHAVIOR_ID = *mut WSA_COMPATIBILITY_BEHAVIOR_ID;
pub type PWSA_COMPATIBILITY_MODE = *mut WSA_COMPATIBILITY_MODE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIORESULT {
    pub Status: i32,
    pub BytesTransferred: u32,
    pub SocketContext: u64,
    pub RequestContext: u64,
}
#[repr(C)]
#[cfg(feature = "mswsock")]
#[derive(Clone, Copy)]
pub struct RIO_BUF {
    pub BufferId: RIO_BUFFERID,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(feature = "mswsock")]
impl Default for RIO_BUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "mswsock")]
pub type RIO_BUFFERID = *mut super::RIO_BUFFERID_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295;
#[cfg(feature = "mswsock")]
pub type RIO_CQ = *mut super::RIO_CQ_t;
#[cfg(feature = "mswsock")]
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = 4294967295i64 as _;
#[cfg(feature = "mswsock")]
pub const RIO_INVALID_CQ: RIO_CQ = 0 as _;
#[cfg(feature = "mswsock")]
pub const RIO_INVALID_RQ: RIO_RQ = 0 as _;
pub const RIO_MAX_CQ_SIZE: i32 = 134217728;
pub const RIO_MSG_COMMIT_ONLY: i32 = 8;
pub const RIO_MSG_DEFER: i32 = 2;
pub const RIO_MSG_DONT_NOTIFY: i32 = 1;
pub const RIO_MSG_WAITALL: i32 = 4;
#[cfg(feature = "mswsock")]
pub type RIO_RQ = *mut super::RIO_RQ_t;
pub const SIO_SET_COMPATIBILITY_MODE: u32 = 2550137132;
pub type WSA_COMPATIBILITY_BEHAVIOR_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: u32,
}
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = 0;
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = 2;
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = 1;
