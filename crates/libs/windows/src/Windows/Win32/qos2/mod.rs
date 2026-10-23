#[cfg(all(feature = "shared", feature = "winnt", feature = "winsock2", feature = "ws2"))]
#[inline]
pub unsafe fn QOSAddSocketToFlow(qoshandle: super::HANDLE, socket: super::SOCKET, destaddr: Option<super::PSOCKADDR>, traffictype: QOS_TRAFFIC_TYPE, flags: Option<super::DWORD>, flowid: PQOS_FLOWID) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSAddSocketToFlow(qoshandle : super::HANDLE, socket : super::SOCKET, destaddr : super::PSOCKADDR, traffictype : QOS_TRAFFIC_TYPE, flags : super::DWORD, flowid : PQOS_FLOWID) -> windows_core::BOOL);
    unsafe { QOSAddSocketToFlow(qoshandle, socket, destaddr.unwrap_or(core::mem::zeroed()) as _, traffictype, flags.unwrap_or(core::mem::zeroed()) as _, flowid as _) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QOSCancel(qoshandle: super::HANDLE, overlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCancel(qoshandle : super::HANDLE, overlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSCancel(qoshandle, overlapped) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QOSCloseHandle(qoshandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCloseHandle(qoshandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { QOSCloseHandle(qoshandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QOSCreateHandle(version: PQOS_VERSION, qoshandle: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCreateHandle(version : PQOS_VERSION, qoshandle : super::PHANDLE) -> windows_core::BOOL);
    unsafe { QOSCreateHandle(version, qoshandle as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QOSEnumerateFlows(qoshandle: super::HANDLE, size: super::PULONG, buffer: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSEnumerateFlows(qoshandle : super::HANDLE, size : super::PULONG, buffer : super::PVOID) -> windows_core::BOOL);
    unsafe { QOSEnumerateFlows(qoshandle, size as _, buffer as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QOSNotifyFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_NOTIFY_FLOW, size: Option<super::PULONG>, buffer: Option<super::PVOID>, flags: super::DWORD, overlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSNotifyFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_NOTIFY_FLOW, size : super::PULONG, buffer : super::PVOID, flags : super::DWORD, overlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSNotifyFlow(qoshandle, flowid, operation, size.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, flags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QOSQueryFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_QUERY_FLOW, size: super::PULONG, buffer: super::PVOID, flags: Option<super::DWORD>, overlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSQueryFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_QUERY_FLOW, size : super::PULONG, buffer : super::PVOID, flags : super::DWORD, overlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSQueryFlow(qoshandle, flowid, operation, size as _, buffer as _, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn QOSRemoveSocketFromFlow(qoshandle: super::HANDLE, socket: Option<super::SOCKET>, flowid: QOS_FLOWID, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSRemoveSocketFromFlow(qoshandle : super::HANDLE, socket : super::SOCKET, flowid : QOS_FLOWID, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { QOSRemoveSocketFromFlow(qoshandle, socket.unwrap_or(core::mem::zeroed()) as _, flowid, flags) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QOSSetFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_SET_FLOW, size: super::ULONG, buffer: super::PVOID, flags: super::DWORD, overlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSSetFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_SET_FLOW, size : super::ULONG, buffer : super::PVOID, flags : super::DWORD, overlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSSetFlow(qoshandle, flowid, operation, size, buffer, flags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn QOSStartTrackingClient(qoshandle: super::HANDLE, destaddr: super::PSOCKADDR, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStartTrackingClient(qoshandle : super::HANDLE, destaddr : super::PSOCKADDR, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { QOSStartTrackingClient(qoshandle, destaddr, flags) }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn QOSStopTrackingClient(qoshandle: super::HANDLE, destaddr: super::PSOCKADDR, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStopTrackingClient(qoshandle : super::HANDLE, destaddr : super::PSOCKADDR, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { QOSStopTrackingClient(qoshandle, destaddr, flags) }
}
pub type PQOS_FLOWID = *mut u32;
#[cfg(feature = "shared")]
pub type PQOS_FLOWRATE_OUTGOING = *mut QOS_FLOWRATE_OUTGOING;
pub type PQOS_FLOWRATE_REASON = *mut QOS_FLOWRATE_REASON;
#[cfg(feature = "shared")]
pub type PQOS_FLOW_FUNDAMENTALS = *mut QOS_FLOW_FUNDAMENTALS;
pub type PQOS_NOTIFY_FLOW = *mut QOS_NOTIFY_FLOW;
#[cfg(feature = "shared")]
pub type PQOS_PACKET_PRIORITY = *mut QOS_PACKET_PRIORITY;
pub type PQOS_QUERY_FLOW = *mut QOS_QUERY_FLOW;
pub type PQOS_SET_FLOW = *mut QOS_SET_FLOW;
pub type PQOS_SHAPING = *mut QOS_SHAPING;
pub type PQOS_TRAFFIC_TYPE = *mut QOS_TRAFFIC_TYPE;
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_FLOWRATE_OUTGOING {
    pub Bandwidth: super::UINT64,
    pub ShapingBehavior: QOS_SHAPING,
    pub Reason: QOS_FLOWRATE_REASON,
}
pub type QOS_FLOWRATE_REASON = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_FLOW_FUNDAMENTALS {
    pub BottleneckBandwidthSet: windows_core::BOOL,
    pub BottleneckBandwidth: super::UINT64,
    pub AvailableBandwidthSet: windows_core::BOOL,
    pub AvailableBandwidth: super::UINT64,
    pub RTTSet: windows_core::BOOL,
    pub RTT: super::UINT32,
}
pub const QOS_NON_ADAPTIVE_FLOW: i32 = 2;
pub type QOS_NOTIFY_FLOW = i32;
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_PACKET_PRIORITY {
    pub ConformantDSCPValue: super::ULONG,
    pub NonConformantDSCPValue: super::ULONG,
    pub ConformantL2Value: super::ULONG,
    pub NonConformantL2Value: super::ULONG,
}
pub const QOS_QUERYFLOW_FRESH: i32 = 1;
pub type QOS_QUERY_FLOW = i32;
pub type QOS_SET_FLOW = i32;
pub type QOS_SHAPING = i32;
pub type QOS_TRAFFIC_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_VERSION {
    pub MajorVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
}
