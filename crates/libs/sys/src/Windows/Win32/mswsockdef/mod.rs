#[cfg(feature = "shared")]
pub type PRIORESULT = *mut RIORESULT;
#[cfg(feature = "shared")]
pub type PRIO_BUF = *mut RIO_BUF;
pub type PRIO_BUFFERID = *mut *mut RIO_BUFFERID_t;
#[cfg(feature = "shared")]
pub type PRIO_CMSG_BUFFER = *mut RIO_CMSG_BUFFER;
pub type PRIO_CQ = *mut *mut RIO_CQ_t;
pub type PRIO_RQ = *mut *mut RIO_RQ_t;
pub type PWSA_COMPATIBILITY_BEHAVIOR_ID = *mut WSA_COMPATIBILITY_BEHAVIOR_ID;
#[cfg(feature = "shared")]
pub type PWSA_COMPATIBILITY_MODE = *mut WSA_COMPATIBILITY_MODE;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RIORESULT {
    pub Status: super::LONG,
    pub BytesTransferred: super::ULONG,
    pub SocketContext: super::ULONGLONG,
    pub RequestContext: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RIO_BUF {
    pub BufferId: RIO_BUFFERID,
    pub Offset: super::ULONG,
    pub Length: super::ULONG,
}
pub type RIO_BUFFERID = *mut RIO_BUFFERID_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_BUFFERID_t(pub u8);
#[cfg(target_arch = "x86")]
pub const RIO_CMSG_BASE_SIZE: u32 = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RIO_CMSG_BASE_SIZE: u64 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: super::ULONG,
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295;
pub type RIO_CQ = *mut RIO_CQ_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_CQ_t(pub u8);
#[cfg(target_arch = "x86")]
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = -1 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = 4294967295i64 as _;
pub const RIO_INVALID_CQ: RIO_CQ = 0 as _;
pub const RIO_INVALID_RQ: RIO_RQ = 0 as _;
pub const RIO_MAX_CQ_SIZE: i32 = 134217728;
pub const RIO_MSG_COMMIT_ONLY: i32 = 8;
pub const RIO_MSG_DEFER: i32 = 2;
pub const RIO_MSG_DONT_NOTIFY: i32 = 1;
pub const RIO_MSG_WAITALL: i32 = 4;
pub type RIO_RQ = *mut RIO_RQ_t;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RIO_RQ_t(pub u8);
pub const SIO_SET_COMPATIBILITY_MODE: u32 = 2550137132;
pub type WSA_COMPATIBILITY_BEHAVIOR_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: super::ULONG,
}
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = 0;
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = 2;
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = 1;
