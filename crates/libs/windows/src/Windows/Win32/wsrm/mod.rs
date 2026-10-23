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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_FEC_INFO {
    pub FECBlockSize: super::USHORT,
    pub FECProActivePackets: super::USHORT,
    pub FECGroupSize: super::UCHAR,
    pub fFECOnDemandParityEnabled: super::BOOLEAN,
}
pub const RM_FLUSHCACHE: i32 = 1003;
pub const RM_HIGH_SPEED_INTRANET_OPT: i32 = 1014;
pub const RM_LATEJOIN: i32 = 1006;
pub const RM_OPTIONSBASE: i32 = 1000;
pub const RM_RATE_WINDOW_SIZE: i32 = 1001;
pub const RM_RECEIVER_STATISTICS: i32 = 1013;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_RECEIVER_STATS {
    pub NumODataPacketsReceived: super::ULONGLONG,
    pub NumRDataPacketsReceived: super::ULONGLONG,
    pub NumDuplicateDataPackets: super::ULONGLONG,
    pub DataBytesReceived: super::ULONGLONG,
    pub TotalBytesReceived: super::ULONGLONG,
    pub RateKBitsPerSecOverall: super::ULONGLONG,
    pub RateKBitsPerSecLast: super::ULONGLONG,
    pub TrailingEdgeSeqId: super::ULONGLONG,
    pub LeadingEdgeSeqId: super::ULONGLONG,
    pub AverageSequencesInWindow: super::ULONGLONG,
    pub MinSequencesInWindow: super::ULONGLONG,
    pub MaxSequencesInWindow: super::ULONGLONG,
    pub FirstNakSequenceNumber: super::ULONGLONG,
    pub NumPendingNaks: super::ULONGLONG,
    pub NumOutstandingNaks: super::ULONGLONG,
    pub NumDataPacketsBuffered: super::ULONGLONG,
    pub TotalSelectiveNaksSent: super::ULONGLONG,
    pub TotalParityNaksSent: super::ULONGLONG,
}
pub const RM_SENDER_STATISTICS: i32 = 1005;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_SENDER_STATS {
    pub DataBytesSent: super::ULONGLONG,
    pub TotalBytesSent: super::ULONGLONG,
    pub NaksReceived: super::ULONGLONG,
    pub NaksReceivedTooLate: super::ULONGLONG,
    pub NumOutstandingNaks: super::ULONGLONG,
    pub NumNaksAfterRData: super::ULONGLONG,
    pub RepairPacketsSent: super::ULONGLONG,
    pub BufferSpaceAvailable: super::ULONGLONG,
    pub TrailingEdgeSeqId: super::ULONGLONG,
    pub LeadingEdgeSeqId: super::ULONGLONG,
    pub RateKBitsPerSecOverall: super::ULONGLONG,
    pub RateKBitsPerSecLast: super::ULONGLONG,
    pub TotalODataPacketsSent: super::ULONGLONG,
}
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: i32 = 1004;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_SEND_WINDOW {
    pub RateKbitsPerSec: super::ULONG,
    pub WindowSizeInMSecs: super::ULONG,
    pub WindowSizeInBytes: super::ULONG,
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
