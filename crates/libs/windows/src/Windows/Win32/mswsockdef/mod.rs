#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIORESULT(pub *mut RIORESULT);
impl PRIORESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIORESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIO_BUF(pub *mut RIO_BUF);
impl PRIO_BUF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIO_BUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIO_BUFFERID(pub *mut *mut RIO_BUFFERID_t);
impl PRIO_BUFFERID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIO_BUFFERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIO_CMSG_BUFFER(pub *mut RIO_CMSG_BUFFER);
impl PRIO_CMSG_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIO_CMSG_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIO_CQ(pub *mut *mut RIO_CQ_t);
impl PRIO_CQ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIO_CQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRIO_RQ(pub *mut *mut RIO_RQ_t);
impl PRIO_RQ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRIO_RQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSA_COMPATIBILITY_BEHAVIOR_ID(pub *mut WSA_COMPATIBILITY_BEHAVIOR_ID);
impl PWSA_COMPATIBILITY_BEHAVIOR_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSA_COMPATIBILITY_BEHAVIOR_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSA_COMPATIBILITY_MODE(pub *mut WSA_COMPATIBILITY_MODE);
impl PWSA_COMPATIBILITY_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSA_COMPATIBILITY_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIORESULT {
    pub Status: i32,
    pub BytesTransferred: u32,
    pub SocketContext: u64,
    pub RequestContext: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIO_BUF {
    pub BufferId: RIO_BUFFERID,
    pub Offset: u32,
    pub Length: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RIO_BUFFERID(pub *mut RIO_BUFFERID_t);
impl RIO_BUFFERID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RIO_BUFFERID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIO_BUFFERID_t(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIO_CMSG_BUFFER {
    pub TotalLength: u32,
}
pub const RIO_CORRUPT_CQ: u32 = 4294967295;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RIO_CQ(pub *mut RIO_CQ_t);
impl RIO_CQ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RIO_CQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIO_CQ_t(pub u8);
pub const RIO_INVALID_BUFFERID: RIO_BUFFERID = RIO_BUFFERID(4294967295i64 as _);
pub const RIO_INVALID_CQ: RIO_CQ = RIO_CQ(0 as _);
pub const RIO_INVALID_RQ: RIO_RQ = RIO_RQ(0 as _);
pub const RIO_MAX_CQ_SIZE: u32 = 134217728;
pub const RIO_MSG_COMMIT_ONLY: u32 = 8;
pub const RIO_MSG_DEFER: u32 = 2;
pub const RIO_MSG_DONT_NOTIFY: u32 = 1;
pub const RIO_MSG_WAITALL: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RIO_RQ(pub *mut RIO_RQ_t);
impl RIO_RQ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RIO_RQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RIO_RQ_t(pub u8);
pub const SIO_SET_COMPATIBILITY_MODE: i32 = -1744830164;
pub type WSA_COMPATIBILITY_BEHAVIOR_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSA_COMPATIBILITY_MODE {
    pub BehaviorId: WSA_COMPATIBILITY_BEHAVIOR_ID,
    pub TargetOsVersion: u32,
}
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = 0;
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = 2;
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = 1;
