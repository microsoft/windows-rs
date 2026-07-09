#[repr(C)]
#[cfg(feature = "Win32_rpcndr")]
#[derive(Clone, Copy)]
pub struct BOID {
    pub rgb: [super::rpcndr::byte; 16],
}
#[cfg(feature = "Win32_rpcndr")]
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ISOFLAG = i32;
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = 16;
pub const ISOFLAG_READONLY: ISOFLAG = 32;
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = 8;
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = 4;
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = 12;
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = 10;
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = 2;
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = 1;
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = 3;
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = 5;
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = 15;
pub type ISOLATIONLEVEL = i32;
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = 16;
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = 65536;
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = -1;
pub type ISOLEVEL = i32;
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = 40;
pub type TX_MISC_CONSTANTS = i32;
pub type XACTCONST = i32;
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = 0;
pub type XACTHEURISTIC = i32;
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = 1;
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = 2;
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = 3;
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [i8; 40],
}
impl Default for XACTOPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type XACTRM = i32;
pub const XACTRM_NOREADONLYPREPARES: XACTRM = 2;
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = 1;
pub type XACTSTAT = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::minwindef::FILETIME,
}
pub const XACTSTAT_ABORTED: XACTSTAT = 512;
pub const XACTSTAT_ABORTING: XACTSTAT = 256;
pub const XACTSTAT_ALL: XACTSTAT = 524287;
pub const XACTSTAT_CLOSED: XACTSTAT = 262144;
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = 128;
pub const XACTSTAT_COMMITTED: XACTSTAT = 1024;
pub const XACTSTAT_COMMITTING: XACTSTAT = 64;
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = 32768;
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = 65536;
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = 2048;
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = 4096;
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = 8192;
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = 16384;
pub const XACTSTAT_INDOUBT: XACTSTAT = 131072;
pub const XACTSTAT_NONE: XACTSTAT = 0;
pub const XACTSTAT_NOTPREPARED: XACTSTAT = 524227;
pub const XACTSTAT_OPEN: XACTSTAT = 3;
pub const XACTSTAT_OPENNORMAL: XACTSTAT = 1;
pub const XACTSTAT_OPENREFUSED: XACTSTAT = 2;
pub const XACTSTAT_PREPARED: XACTSTAT = 8;
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = 32;
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = 16;
pub const XACTSTAT_PREPARING: XACTSTAT = 4;
pub type XACTTC = i32;
pub const XACTTC_ASYNC: XACTTC = 4;
pub const XACTTC_ASYNC_PHASEONE: XACTTC = 4;
pub const XACTTC_NONE: XACTTC = 0;
pub const XACTTC_SYNC: XACTTC = 2;
pub const XACTTC_SYNC_PHASEONE: XACTTC = 1;
pub const XACTTC_SYNC_PHASETWO: XACTTC = 2;
#[repr(C)]
#[cfg(feature = "Win32_rpcndr")]
#[derive(Clone, Copy, Default)]
pub struct XACTTRANSINFO {
    pub uow: XACTUOW,
    pub isoLevel: ISOLEVEL,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
#[cfg(feature = "Win32_rpcndr")]
pub type XACTUOW = BOID;
