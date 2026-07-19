#[cfg(all(feature = "winnt", feature = "winsock2", feature = "ws2"))]
#[inline]
pub unsafe fn QOSAddSocketToFlow(qoshandle: super::HANDLE, socket: super::SOCKET, destaddr: Option<*const super::SOCKADDR>, traffictype: QOS_TRAFFIC_TYPE, flags: Option<u32>, flowid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSAddSocketToFlow(qoshandle : super::HANDLE, socket : super::SOCKET, destaddr : *const super::SOCKADDR, traffictype : QOS_TRAFFIC_TYPE, flags : u32, flowid : *mut u32) -> windows_core::BOOL);
    unsafe { QOSAddSocketToFlow(qoshandle, socket, destaddr.unwrap_or(core::mem::zeroed()) as _, traffictype, flags.unwrap_or(core::mem::zeroed()) as _, flowid as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn QOSCancel(qoshandle: super::HANDLE, overlapped: *const super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCancel(qoshandle : super::HANDLE, overlapped : *const super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSCancel(qoshandle, overlapped) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QOSCloseHandle(qoshandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCloseHandle(qoshandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { QOSCloseHandle(qoshandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QOSCreateHandle(version: *const QOS_VERSION, qoshandle: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCreateHandle(version : *const QOS_VERSION, qoshandle : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { QOSCreateHandle(version, qoshandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QOSEnumerateFlows(qoshandle: super::HANDLE, size: *mut u32, buffer: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSEnumerateFlows(qoshandle : super::HANDLE, size : *mut u32, buffer : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { QOSEnumerateFlows(qoshandle, size as _, buffer as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn QOSNotifyFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_NOTIFY_FLOW, size: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, flags: Option<u32>, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSNotifyFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_NOTIFY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSNotifyFlow(qoshandle, flowid, operation, size.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn QOSQueryFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_QUERY_FLOW, size: *mut u32, buffer: *mut core::ffi::c_void, flags: Option<u32>, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSQueryFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_QUERY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSQueryFlow(qoshandle, flowid, operation, size as _, buffer as _, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn QOSRemoveSocketFromFlow(qoshandle: super::HANDLE, socket: Option<super::SOCKET>, flowid: QOS_FLOWID, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSRemoveSocketFromFlow(qoshandle : super::HANDLE, socket : super::SOCKET, flowid : QOS_FLOWID, flags : u32) -> windows_core::BOOL);
    unsafe { QOSRemoveSocketFromFlow(qoshandle, socket.unwrap_or(core::mem::zeroed()) as _, flowid, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn QOSSetFlow(qoshandle: super::HANDLE, flowid: QOS_FLOWID, operation: QOS_SET_FLOW, size: u32, buffer: *const core::ffi::c_void, flags: Option<u32>, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSSetFlow(qoshandle : super::HANDLE, flowid : QOS_FLOWID, operation : QOS_SET_FLOW, size : u32, buffer : *const core::ffi::c_void, flags : u32, overlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSSetFlow(qoshandle, flowid, operation, size, buffer, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn QOSStartTrackingClient(qoshandle: super::HANDLE, destaddr: *const super::SOCKADDR, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStartTrackingClient(qoshandle : super::HANDLE, destaddr : *const super::SOCKADDR, flags : u32) -> windows_core::BOOL);
    unsafe { QOSStartTrackingClient(qoshandle, destaddr, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winnt", feature = "ws2"))]
#[inline]
pub unsafe fn QOSStopTrackingClient(qoshandle: super::HANDLE, destaddr: *const super::SOCKADDR, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStopTrackingClient(qoshandle : super::HANDLE, destaddr : *const super::SOCKADDR, flags : u32) -> windows_core::BOOL);
    unsafe { QOSStopTrackingClient(qoshandle, destaddr, flags.unwrap_or(core::mem::zeroed()) as _) }
}
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct QOS_FLOWID(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_FLOWRATE_OUTGOING {
    pub Bandwidth: u64,
    pub ShapingBehavior: QOS_SHAPING,
    pub Reason: QOS_FLOWRATE_REASON,
}
pub type QOS_FLOWRATE_REASON = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_FLOW_FUNDAMENTALS {
    pub BottleneckBandwidthSet: windows_core::BOOL,
    pub BottleneckBandwidth: u64,
    pub AvailableBandwidthSet: windows_core::BOOL,
    pub AvailableBandwidth: u64,
    pub RTTSet: windows_core::BOOL,
    pub RTT: u32,
}
pub const QOS_NON_ADAPTIVE_FLOW: u32 = 2;
pub type QOS_NOTIFY_FLOW = i32;
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
