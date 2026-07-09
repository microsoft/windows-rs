pub type PRIORESULT = *mut RIORESULT;
pub type PRIO_BUF = *mut RIO_BUF;
pub type PRIO_BUFFERID = *mut *mut RIO_BUFFERID_t;
pub type PRIO_CMSG_BUFFER = *mut RIO_CMSG_BUFFER;
pub type PRIO_CQ = *mut *mut RIO_CQ_t;
pub type PRIO_RQ = *mut *mut RIO_RQ_t;
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
#[derive(Clone, Copy)]
pub struct RIO_BUF {
    pub BufferId: RIO_BUFFERID,
    pub Offset: u32,
    pub Length: u32,
}
impl Default for RIO_BUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RIO_BUFFERID = *mut RIO_BUFFERID_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_BUFFERID_t(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295;
pub type RIO_CQ = *mut RIO_CQ_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_CQ_t(pub u8);
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = 4294967295i64 as _;
pub const RIO_INVALID_CQ: RIO_CQ = 0 as _;
pub const RIO_INVALID_RQ: RIO_RQ = 0 as _;
pub const RIO_MAX_CQ_SIZE: u32 = 134217728;
pub const RIO_MSG_COMMIT_ONLY: u32 = 8;
pub const RIO_MSG_DEFER: u32 = 2;
pub const RIO_MSG_DONT_NOTIFY: u32 = 1;
pub const RIO_MSG_WAITALL: u32 = 4;
pub type RIO_RQ = *mut RIO_RQ_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_RQ_t(pub u8);
pub const SIO_SET_COMPATIBILITY_MODE: i32 = -1744830164;
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
