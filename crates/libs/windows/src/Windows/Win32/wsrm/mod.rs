pub const BITS_PER_BYTE: i32 = 8;
pub const E_WINDOW_ADVANCE_BY_TIME: eWINDOW_ADVANCE_METHOD = 1;
pub const E_WINDOW_USE_AS_DATA_CACHE: eWINDOW_ADVANCE_METHOD = 2;
pub const IPPROTO_RM: i32 = 113;
pub const LOG2_BITS_PER_BYTE: i32 = 3;
pub const MAX_MCAST_TTL: i32 = 255;
pub const MAX_WINDOW_INCREMENT_PERCENTAGE: i32 = 25;
pub const RM_ADD_RECEIVE_IF: i32 = 1008;
pub const RM_DEL_RECEIVE_IF: i32 = 1009;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_FEC_INFO {
    pub FECBlockSize: u16,
    pub FECProActivePackets: u16,
    pub FECGroupSize: u8,
    pub fFECOnDemandParityEnabled: bool,
}
pub const RM_FLUSHCACHE: i32 = 1003;
pub const RM_HIGH_SPEED_INTRANET_OPT: i32 = 1014;
pub const RM_LATEJOIN: i32 = 1006;
pub const RM_OPTIONSBASE: i32 = 1000;
pub const RM_RATE_WINDOW_SIZE: i32 = 1001;
pub const RM_RECEIVER_STATISTICS: i32 = 1013;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_RECEIVER_STATS {
    pub NumODataPacketsReceived: u64,
    pub NumRDataPacketsReceived: u64,
    pub NumDuplicateDataPackets: u64,
    pub DataBytesReceived: u64,
    pub TotalBytesReceived: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub AverageSequencesInWindow: u64,
    pub MinSequencesInWindow: u64,
    pub MaxSequencesInWindow: u64,
    pub FirstNakSequenceNumber: u64,
    pub NumPendingNaks: u64,
    pub NumOutstandingNaks: u64,
    pub NumDataPacketsBuffered: u64,
    pub TotalSelectiveNaksSent: u64,
    pub TotalParityNaksSent: u64,
}
pub const RM_SENDER_STATISTICS: i32 = 1005;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_SENDER_STATS {
    pub DataBytesSent: u64,
    pub TotalBytesSent: u64,
    pub NaksReceived: u64,
    pub NaksReceivedTooLate: u64,
    pub NumOutstandingNaks: u64,
    pub NumNaksAfterRData: u64,
    pub RepairPacketsSent: u64,
    pub BufferSpaceAvailable: u64,
    pub TrailingEdgeSeqId: u64,
    pub LeadingEdgeSeqId: u64,
    pub RateKBitsPerSecOverall: u64,
    pub RateKBitsPerSecLast: u64,
    pub TotalODataPacketsSent: u64,
}
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: i32 = 1004;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_SEND_WINDOW {
    pub RateKbitsPerSec: u32,
    pub WindowSizeInMSecs: u32,
    pub WindowSizeInBytes: u32,
}
pub const RM_SEND_WINDOW_ADV_RATE: i32 = 1010;
pub const RM_SET_MCAST_TTL: i32 = 1012;
pub const RM_SET_MESSAGE_BOUNDARY: i32 = 1002;
pub const RM_SET_SEND_IF: i32 = 1007;
pub const RM_USE_FEC: i32 = 1011;
pub const SENDER_DEFAULT_LATE_JOINER_PERCENTAGE: i32 = 0;
pub const SENDER_DEFAULT_RATE_KBITS_PER_SEC: i32 = 56;
pub const SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE: i32 = 15;
pub const SENDER_DEFAULT_WINDOW_SIZE_BYTES: i32 = 10000000;
pub const SENDER_MAX_LATE_JOINER_PERCENTAGE: i32 = 75;
pub type eWINDOW_ADVANCE_METHOD = i32;
