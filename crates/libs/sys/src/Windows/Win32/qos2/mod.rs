#[cfg(all(feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def"))]
windows_link::link!("qwave.dll" "system" fn QOSAddSocketToFlow(qoshandle : super::winnt::HANDLE, socket : super::winsock2::SOCKET, destaddr : *const super::ws2def::SOCKADDR, traffictype : QOS_TRAFFIC_TYPE, flags : u32, flowid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("qwave.dll" "system" fn QOSCancel(qoshandle : super::winnt::HANDLE, overlapped : *const super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("qwave.dll" "system" fn QOSCloseHandle(qoshandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("qwave.dll" "system" fn QOSCreateHandle(version : *const QOS_VERSION, qoshandle : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("qwave.dll" "system" fn QOSEnumerateFlows(qoshandle : super::winnt::HANDLE, size : *mut u32, buffer : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("qwave.dll" "system" fn QOSNotifyFlow(qoshandle : super::winnt::HANDLE, flowid : QOS_FLOWID, operation : QOS_NOTIFY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("qwave.dll" "system" fn QOSQueryFlow(qoshandle : super::winnt::HANDLE, flowid : QOS_FLOWID, operation : QOS_QUERY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winsock2"))]
windows_link::link!("qwave.dll" "system" fn QOSRemoveSocketFromFlow(qoshandle : super::winnt::HANDLE, socket : super::winsock2::SOCKET, flowid : QOS_FLOWID, flags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("qwave.dll" "system" fn QOSSetFlow(qoshandle : super::winnt::HANDLE, flowid : QOS_FLOWID, operation : QOS_SET_FLOW, size : u32, buffer : *const core::ffi::c_void, flags : u32, overlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_ws2def"))]
windows_link::link!("qwave.dll" "system" fn QOSStartTrackingClient(qoshandle : super::winnt::HANDLE, destaddr : *const super::ws2def::SOCKADDR, flags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_ws2def"))]
windows_link::link!("qwave.dll" "system" fn QOSStopTrackingClient(qoshandle : super::winnt::HANDLE, destaddr : *const super::ws2def::SOCKADDR, flags : u32) -> windows_sys::core::BOOL);
pub type PQOS_FLOWID = *mut u32;
pub type PQOS_FLOWRATE_OUTGOING = *mut QOS_FLOWRATE_OUTGOING;
pub type PQOS_FLOWRATE_REASON = *mut QOS_FLOWRATE_REASON;
pub type PQOS_FLOW_FUNDAMENTALS = *mut QOS_FLOW_FUNDAMENTALS;
pub type PQOS_NOTIFY_FLOW = *mut QOS_NOTIFY_FLOW;
pub type PQOS_PACKET_PRIORITY = *mut QOS_PACKET_PRIORITY;
pub type PQOS_QUERY_FLOW = *mut QOS_QUERY_FLOW;
pub type PQOS_SET_FLOW = *mut QOS_SET_FLOW;
pub type PQOS_SHAPING = *mut QOS_SHAPING;
pub type PQOS_TRAFFIC_TYPE = *mut QOS_TRAFFIC_TYPE;
pub type PQOS_VERSION = *mut QOS_VERSION;
pub const QOSFlowRateCongestion: QOS_FLOWRATE_REASON = 2;
pub const QOSFlowRateContentChange: QOS_FLOWRATE_REASON = 1;
pub const QOSFlowRateHigherContentEncoding: QOS_FLOWRATE_REASON = 3;
pub const QOSFlowRateNotApplicable: QOS_FLOWRATE_REASON = 0;
pub const QOSFlowRateUserCaused: QOS_FLOWRATE_REASON = 4;
pub const QOSNotifyAvailable: QOS_NOTIFY_FLOW = 2;
pub const QOSNotifyCongested: QOS_NOTIFY_FLOW = 0;
pub const QOSNotifyUncongested: QOS_NOTIFY_FLOW = 1;
pub const QOSQueryFlowFundamentals: QOS_QUERY_FLOW = 0;
pub const QOSQueryOutgoingRate: QOS_QUERY_FLOW = 2;
pub const QOSQueryPacketPriority: QOS_QUERY_FLOW = 1;
pub const QOSSetOutgoingDSCPValue: QOS_SET_FLOW = 2;
pub const QOSSetOutgoingRate: QOS_SET_FLOW = 1;
pub const QOSSetTrafficType: QOS_SET_FLOW = 0;
pub const QOSShapeAndMark: QOS_SHAPING = 1;
pub const QOSShapeOnly: QOS_SHAPING = 0;
pub const QOSTrafficTypeAudioVideo: QOS_TRAFFIC_TYPE = 3;
pub const QOSTrafficTypeBackground: QOS_TRAFFIC_TYPE = 1;
pub const QOSTrafficTypeBestEffort: QOS_TRAFFIC_TYPE = 0;
pub const QOSTrafficTypeControl: QOS_TRAFFIC_TYPE = 5;
pub const QOSTrafficTypeExcellentEffort: QOS_TRAFFIC_TYPE = 2;
pub const QOSTrafficTypeVoice: QOS_TRAFFIC_TYPE = 4;
pub const QOSUseNonConformantMarkings: QOS_SHAPING = 2;
pub type QOS_FLOWID = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOS_FLOWRATE_OUTGOING {
    pub Bandwidth: u64,
    pub ShapingBehavior: QOS_SHAPING,
    pub Reason: QOS_FLOWRATE_REASON,
}
pub type QOS_FLOWRATE_REASON = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOS_FLOW_FUNDAMENTALS {
    pub BottleneckBandwidthSet: windows_sys::core::BOOL,
    pub BottleneckBandwidth: u64,
    pub AvailableBandwidthSet: windows_sys::core::BOOL,
    pub AvailableBandwidth: u64,
    pub RTTSet: windows_sys::core::BOOL,
    pub RTT: u32,
}
pub const QOS_NON_ADAPTIVE_FLOW: u32 = 2;
pub type QOS_NOTIFY_FLOW = i32;
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOS_PACKET_PRIORITY {
    pub ConformantDSCPValue: u32,
    pub NonConformantDSCPValue: u32,
    pub ConformantL2Value: u32,
    pub NonConformantL2Value: u32,
}
pub const QOS_QUERYFLOW_FRESH: u32 = 1;
pub type QOS_QUERY_FLOW = i32;
pub type QOS_SET_FLOW = i32;
pub type QOS_SHAPING = i32;
pub type QOS_TRAFFIC_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QOS_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
