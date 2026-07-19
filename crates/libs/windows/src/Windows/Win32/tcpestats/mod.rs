pub type PTCP_BOOLEAN_OPTIONAL = *mut TCP_BOOLEAN_OPTIONAL;
pub type PTCP_ESTATS_BANDWIDTH_ROD_v0 = *mut TCP_ESTATS_BANDWIDTH_ROD_v0;
pub type PTCP_ESTATS_BANDWIDTH_RW_v0 = *mut TCP_ESTATS_BANDWIDTH_RW_v0;
pub type PTCP_ESTATS_DATA_ROD_v0 = *mut TCP_ESTATS_DATA_ROD_v0;
pub type PTCP_ESTATS_DATA_RW_v0 = *mut TCP_ESTATS_DATA_RW_v0;
pub type PTCP_ESTATS_FINE_RTT_ROD_v0 = *mut TCP_ESTATS_FINE_RTT_ROD_v0;
pub type PTCP_ESTATS_FINE_RTT_RW_v0 = *mut TCP_ESTATS_FINE_RTT_RW_v0;
pub type PTCP_ESTATS_OBS_REC_ROD_v0 = *mut TCP_ESTATS_OBS_REC_ROD_v0;
pub type PTCP_ESTATS_OBS_REC_RW_v0 = *mut TCP_ESTATS_OBS_REC_RW_v0;
pub type PTCP_ESTATS_PATH_ROD_v0 = *mut TCP_ESTATS_PATH_ROD_v0;
pub type PTCP_ESTATS_PATH_RW_v0 = *mut TCP_ESTATS_PATH_RW_v0;
pub type PTCP_ESTATS_REC_ROD_v0 = *mut TCP_ESTATS_REC_ROD_v0;
pub type PTCP_ESTATS_REC_RW_v0 = *mut TCP_ESTATS_REC_RW_v0;
pub type PTCP_ESTATS_SEND_BUFF_ROD_v0 = *mut TCP_ESTATS_SEND_BUFF_ROD_v0;
pub type PTCP_ESTATS_SEND_BUFF_RW_v0 = *mut TCP_ESTATS_SEND_BUFF_RW_v0;
pub type PTCP_ESTATS_SND_CONG_ROD_v0 = *mut TCP_ESTATS_SND_CONG_ROD_v0;
pub type PTCP_ESTATS_SND_CONG_ROS_v0 = *mut TCP_ESTATS_SND_CONG_ROS_v0;
pub type PTCP_ESTATS_SND_CONG_RW_v0 = *mut TCP_ESTATS_SND_CONG_RW_v0;
pub type PTCP_ESTATS_SYN_OPTS_ROS_v0 = *mut TCP_ESTATS_SYN_OPTS_ROS_v0;
pub type PTCP_ESTATS_TYPE = *mut TCP_ESTATS_TYPE;
pub type PTCP_SOFT_ERROR = *mut TCP_SOFT_ERROR;
pub type TCP_BOOLEAN_OPTIONAL = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_ROD_v0 {
    pub OutboundBandwidth: u64,
    pub InboundBandwidth: u64,
    pub OutboundInstability: u64,
    pub InboundInstability: u64,
    pub OutboundBandwidthPeaked: bool,
    pub InboundBandwidthPeaked: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_BANDWIDTH_RW_v0 {
    pub EnableCollectionOutbound: TCP_BOOLEAN_OPTIONAL,
    pub EnableCollectionInbound: TCP_BOOLEAN_OPTIONAL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_DATA_ROD_v0 {
    pub DataBytesOut: u64,
    pub DataSegsOut: u64,
    pub DataBytesIn: u64,
    pub DataSegsIn: u64,
    pub SegsOut: u64,
    pub SegsIn: u64,
    pub SoftErrors: u32,
    pub SoftErrorReason: u32,
    pub SndUna: u32,
    pub SndNxt: u32,
    pub SndMax: u32,
    pub ThruBytesAcked: u64,
    pub RcvNxt: u32,
    pub ThruBytesReceived: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_DATA_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_ROD_v0 {
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_FINE_RTT_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_ROD_v0 {
    pub CurRwinRcvd: u32,
    pub MaxRwinRcvd: u32,
    pub MinRwinRcvd: u32,
    pub WinScaleRcvd: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_OBS_REC_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_PATH_ROD_v0 {
    pub FastRetran: u32,
    pub Timeouts: u32,
    pub SubsequentTimeouts: u32,
    pub CurTimeoutCount: u32,
    pub AbruptTimeouts: u32,
    pub PktsRetrans: u32,
    pub BytesRetrans: u32,
    pub DupAcksIn: u32,
    pub SacksRcvd: u32,
    pub SackBlocksRcvd: u32,
    pub CongSignals: u32,
    pub PreCongSumCwnd: u32,
    pub PreCongSumRtt: u32,
    pub PostCongSumRtt: u32,
    pub PostCongCountRtt: u32,
    pub EcnSignals: u32,
    pub EceRcvd: u32,
    pub SendStall: u32,
    pub QuenchRcvd: u32,
    pub RetranThresh: u32,
    pub SndDupAckEpisodes: u32,
    pub SumBytesReordered: u32,
    pub NonRecovDa: u32,
    pub NonRecovDaEpisodes: u32,
    pub AckAfterFr: u32,
    pub DsackDups: u32,
    pub SampleRtt: u32,
    pub SmoothedRtt: u32,
    pub RttVar: u32,
    pub MaxRtt: u32,
    pub MinRtt: u32,
    pub SumRtt: u32,
    pub CountRtt: u32,
    pub CurRto: u32,
    pub MaxRto: u32,
    pub MinRto: u32,
    pub CurMss: u32,
    pub MaxMss: u32,
    pub MinMss: u32,
    pub SpuriousRtoDetections: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_PATH_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_REC_ROD_v0 {
    pub CurRwinSent: u32,
    pub MaxRwinSent: u32,
    pub MinRwinSent: u32,
    pub LimRwin: u32,
    pub DupAckEpisodes: u32,
    pub DupAcksOut: u32,
    pub CeRcvd: u32,
    pub EcnSent: u32,
    pub EcnNoncesRcvd: u32,
    pub CurReasmQueue: u32,
    pub MaxReasmQueue: u32,
    pub CurAppRQueue: usize,
    pub MaxAppRQueue: usize,
    pub WinScaleSent: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_REC_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_ROD_v0 {
    pub CurRetxQueue: usize,
    pub MaxRetxQueue: usize,
    pub CurAppWQueue: usize,
    pub MaxAppWQueue: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SEND_BUFF_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_ROD_v0 {
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: usize,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: usize,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: usize,
    pub SlowStart: u32,
    pub CongAvoid: u32,
    pub OtherReductions: u32,
    pub CurCwnd: u32,
    pub MaxSsCwnd: u32,
    pub MaxCaCwnd: u32,
    pub CurSsthresh: u32,
    pub MaxSsthresh: u32,
    pub MinSsthresh: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_ROS_v0 {
    pub LimCwnd: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SND_CONG_RW_v0 {
    pub EnableCollection: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ESTATS_SYN_OPTS_ROS_v0 {
    pub ActiveOpen: bool,
    pub MssRcvd: u32,
    pub MssSent: u32,
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
