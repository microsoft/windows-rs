pub type PTCP_BOOLEAN_OPTIONAL = *mut TCP_BOOLEAN_OPTIONAL;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_BANDWIDTH_ROD_v0 = *mut TCP_ESTATS_BANDWIDTH_ROD_v0;
pub type PTCP_ESTATS_BANDWIDTH_RW_v0 = *mut TCP_ESTATS_BANDWIDTH_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_DATA_ROD_v0 = *mut TCP_ESTATS_DATA_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_DATA_RW_v0 = *mut TCP_ESTATS_DATA_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_FINE_RTT_ROD_v0 = *mut TCP_ESTATS_FINE_RTT_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_FINE_RTT_RW_v0 = *mut TCP_ESTATS_FINE_RTT_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_OBS_REC_ROD_v0 = *mut TCP_ESTATS_OBS_REC_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_OBS_REC_RW_v0 = *mut TCP_ESTATS_OBS_REC_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_PATH_ROD_v0 = *mut TCP_ESTATS_PATH_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_PATH_RW_v0 = *mut TCP_ESTATS_PATH_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_REC_ROD_v0 = *mut TCP_ESTATS_REC_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_REC_RW_v0 = *mut TCP_ESTATS_REC_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SEND_BUFF_ROD_v0 = *mut TCP_ESTATS_SEND_BUFF_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SEND_BUFF_RW_v0 = *mut TCP_ESTATS_SEND_BUFF_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SND_CONG_ROD_v0 = *mut TCP_ESTATS_SND_CONG_ROD_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SND_CONG_ROS_v0 = *mut TCP_ESTATS_SND_CONG_ROS_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SND_CONG_RW_v0 = *mut TCP_ESTATS_SND_CONG_RW_v0;
#[cfg(feature = "shared")]
pub type PTCP_ESTATS_SYN_OPTS_ROS_v0 = *mut TCP_ESTATS_SYN_OPTS_ROS_v0;
pub type PTCP_ESTATS_TYPE = *mut TCP_ESTATS_TYPE;
pub type PTCP_SOFT_ERROR = *mut TCP_SOFT_ERROR;
pub type TCP_BOOLEAN_OPTIONAL = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    pub OutboundBandwidth: super::ULONG64,
    pub InboundBandwidth: super::ULONG64,
    pub OutboundInstability: super::ULONG64,
    pub InboundInstability: super::ULONG64,
    pub OutboundBandwidthPeaked: super::BOOLEAN,
    pub InboundBandwidthPeaked: super::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    pub EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    pub EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_DATA_ROD_v0 {
    pub DataBytesOut: super::ULONG64,
    pub DataSegsOut: super::ULONG64,
    pub DataBytesIn: super::ULONG64,
    pub DataSegsIn: super::ULONG64,
    pub SegsOut: super::ULONG64,
    pub SegsIn: super::ULONG64,
    pub SoftErrors: super::ULONG,
    pub SoftErrorReason: super::ULONG,
    pub SndUna: super::ULONG,
    pub SndNxt: super::ULONG,
    pub SndMax: super::ULONG,
    pub ThruBytesAcked: super::ULONG64,
    pub RcvNxt: super::ULONG,
    pub ThruBytesReceived: super::ULONG64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_DATA_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    pub RttVar: super::ULONG,
    pub MaxRtt: super::ULONG,
    pub MinRtt: super::ULONG,
    pub SumRtt: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_ROD_v0 {
    pub CurRwinRcvd: super::ULONG,
    pub MaxRwinRcvd: super::ULONG,
    pub MinRwinRcvd: super::ULONG,
    pub WinScaleRcvd: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_PATH_ROD_v0 {
    pub FastRetran: super::ULONG,
    pub Timeouts: super::ULONG,
    pub SubsequentTimeouts: super::ULONG,
    pub CurTimeoutCount: super::ULONG,
    pub AbruptTimeouts: super::ULONG,
    pub PktsRetrans: super::ULONG,
    pub BytesRetrans: super::ULONG,
    pub DupAcksIn: super::ULONG,
    pub SacksRcvd: super::ULONG,
    pub SackBlocksRcvd: super::ULONG,
    pub CongSignals: super::ULONG,
    pub PreCongSumCwnd: super::ULONG,
    pub PreCongSumRtt: super::ULONG,
    pub PostCongSumRtt: super::ULONG,
    pub PostCongCountRtt: super::ULONG,
    pub EcnSignals: super::ULONG,
    pub EceRcvd: super::ULONG,
    pub SendStall: super::ULONG,
    pub QuenchRcvd: super::ULONG,
    pub RetranThresh: super::ULONG,
    pub SndDupAckEpisodes: super::ULONG,
    pub SumBytesReordered: super::ULONG,
    pub NonRecovDa: super::ULONG,
    pub NonRecovDaEpisodes: super::ULONG,
    pub AckAfterFr: super::ULONG,
    pub DsackDups: super::ULONG,
    pub SampleRtt: super::ULONG,
    pub SmoothedRtt: super::ULONG,
    pub RttVar: super::ULONG,
    pub MaxRtt: super::ULONG,
    pub MinRtt: super::ULONG,
    pub SumRtt: super::ULONG,
    pub CountRtt: super::ULONG,
    pub CurRto: super::ULONG,
    pub MaxRto: super::ULONG,
    pub MinRto: super::ULONG,
    pub CurMss: super::ULONG,
    pub MaxMss: super::ULONG,
    pub MinMss: super::ULONG,
    pub SpuriousRtoDetections: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_PATH_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_REC_ROD_v0 {
    pub CurRwinSent: super::ULONG,
    pub MaxRwinSent: super::ULONG,
    pub MinRwinSent: super::ULONG,
    pub LimRwin: super::ULONG,
    pub DupAckEpisodes: super::ULONG,
    pub DupAcksOut: super::ULONG,
    pub CeRcvd: super::ULONG,
    pub EcnSent: super::ULONG,
    pub EcnNoncesRcvd: super::ULONG,
    pub CurReasmQueue: super::ULONG,
    pub MaxReasmQueue: super::ULONG,
    pub CurAppRQueue: super::SIZE_T,
    pub MaxAppRQueue: super::SIZE_T,
    pub WinScaleSent: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_REC_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    pub CurRetxQueue: super::SIZE_T,
    pub MaxRetxQueue: super::SIZE_T,
    pub CurAppWQueue: super::SIZE_T,
    pub MaxAppWQueue: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_ROD_v0 {
    pub SndLimTransRwin: super::ULONG,
    pub SndLimTimeRwin: super::ULONG,
    pub SndLimBytesRwin: super::SIZE_T,
    pub SndLimTransCwnd: super::ULONG,
    pub SndLimTimeCwnd: super::ULONG,
    pub SndLimBytesCwnd: super::SIZE_T,
    pub SndLimTransSnd: super::ULONG,
    pub SndLimTimeSnd: super::ULONG,
    pub SndLimBytesSnd: super::SIZE_T,
    pub SlowStart: super::ULONG,
    pub CongAvoid: super::ULONG,
    pub OtherReductions: super::ULONG,
    pub CurCwnd: super::ULONG,
    pub MaxSsCwnd: super::ULONG,
    pub MaxCaCwnd: super::ULONG,
    pub CurSsthresh: super::ULONG,
    pub MaxSsthresh: super::ULONG,
    pub MinSsthresh: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_ROS_v0 {
    pub LimCwnd: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_RW_v0 {
    pub EnableCollection: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    pub ActiveOpen: super::BOOLEAN,
    pub MssRcvd: super::ULONG,
    pub MssSent: super::ULONG,
}
pub type TCP_ESTATS_TYPE = i32;
pub type TCP_SOFT_ERROR = i32;
pub const TcpBoolOptDisabled: TCP_BOOLEAN_OPTIONAL = 0;
pub const TcpBoolOptEnabled: TCP_BOOLEAN_OPTIONAL = 1;
pub const TcpBoolOptUnchanged: TCP_BOOLEAN_OPTIONAL = -1;
pub const TcpConnectionEstatsBandwidth: TCP_ESTATS_TYPE = 7;
pub const TcpConnectionEstatsData: TCP_ESTATS_TYPE = 1;
pub const TcpConnectionEstatsFineRtt: TCP_ESTATS_TYPE = 8;
pub const TcpConnectionEstatsMaximum: TCP_ESTATS_TYPE = 9;
pub const TcpConnectionEstatsObsRec: TCP_ESTATS_TYPE = 6;
pub const TcpConnectionEstatsPath: TCP_ESTATS_TYPE = 3;
pub const TcpConnectionEstatsRec: TCP_ESTATS_TYPE = 5;
pub const TcpConnectionEstatsSendBuff: TCP_ESTATS_TYPE = 4;
pub const TcpConnectionEstatsSndCong: TCP_ESTATS_TYPE = 2;
pub const TcpConnectionEstatsSynOpts: TCP_ESTATS_TYPE = 0;
pub const TcpErrorAboveAckWindow: TCP_SOFT_ERROR = 4;
pub const TcpErrorAboveDataWindow: TCP_SOFT_ERROR = 2;
pub const TcpErrorAboveTsWindow: TCP_SOFT_ERROR = 6;
pub const TcpErrorBelowAckWindow: TCP_SOFT_ERROR = 3;
pub const TcpErrorBelowDataWindow: TCP_SOFT_ERROR = 1;
pub const TcpErrorBelowTsWindow: TCP_SOFT_ERROR = 5;
pub const TcpErrorDataChecksumError: TCP_SOFT_ERROR = 7;
pub const TcpErrorDataLengthError: TCP_SOFT_ERROR = 8;
pub const TcpErrorMaxSoftError: TCP_SOFT_ERROR = 9;
pub const TcpErrorNone: TCP_SOFT_ERROR = 0;
