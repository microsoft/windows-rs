#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn QOSAddSocketToFlow(qoshandle: super::super::Foundation::HANDLE, socket: super::super::Networking::WinSock::SOCKET, destaddr: Option<*const super::super::Networking::WinSock::SOCKADDR>, traffictype: QOS_TRAFFIC_TYPE, flags: Option<u32>, flowid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSAddSocketToFlow(qoshandle : super::super::Foundation:: HANDLE, socket : super::super::Networking::WinSock:: SOCKET, destaddr : *const super::super::Networking::WinSock:: SOCKADDR, traffictype : QOS_TRAFFIC_TYPE, flags : u32, flowid : *mut u32) -> windows_core::BOOL);
    unsafe { QOSAddSocketToFlow(qoshandle, socket, destaddr.unwrap_or(core::mem::zeroed()) as _, traffictype, flags.unwrap_or(core::mem::zeroed()) as _, flowid as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn QOSCancel(qoshandle: super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCancel(qoshandle : super::super::Foundation:: HANDLE, overlapped : *const super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSCancel(qoshandle, overlapped) }
}
#[inline]
pub unsafe fn QOSCloseHandle(qoshandle: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCloseHandle(qoshandle : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { QOSCloseHandle(qoshandle) }
}
#[inline]
pub unsafe fn QOSCreateHandle(version: *const QOS_VERSION, qoshandle: *mut super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSCreateHandle(version : *const QOS_VERSION, qoshandle : *mut super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { QOSCreateHandle(version, qoshandle as _) }
}
#[inline]
pub unsafe fn QOSEnumerateFlows(qoshandle: super::super::Foundation::HANDLE, size: *mut u32, buffer: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSEnumerateFlows(qoshandle : super::super::Foundation:: HANDLE, size : *mut u32, buffer : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { QOSEnumerateFlows(qoshandle, size as _, buffer as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn QOSNotifyFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_NOTIFY_FLOW, size: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, flags: Option<u32>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSNotifyFlow(qoshandle : super::super::Foundation:: HANDLE, flowid : u32, operation : QOS_NOTIFY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSNotifyFlow(qoshandle, flowid, operation, size.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn QOSQueryFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_QUERY_FLOW, size: *mut u32, buffer: *mut core::ffi::c_void, flags: Option<u32>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSQueryFlow(qoshandle : super::super::Foundation:: HANDLE, flowid : u32, operation : QOS_QUERY_FLOW, size : *mut u32, buffer : *mut core::ffi::c_void, flags : u32, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSQueryFlow(qoshandle, flowid, operation, size as _, buffer as _, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn QOSRemoveSocketFromFlow(qoshandle: super::super::Foundation::HANDLE, socket: Option<super::super::Networking::WinSock::SOCKET>, flowid: u32, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSRemoveSocketFromFlow(qoshandle : super::super::Foundation:: HANDLE, socket : super::super::Networking::WinSock:: SOCKET, flowid : u32, flags : u32) -> windows_core::BOOL);
    unsafe { QOSRemoveSocketFromFlow(qoshandle, socket.unwrap_or(core::mem::zeroed()) as _, flowid, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn QOSSetFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_SET_FLOW, size: u32, buffer: *const core::ffi::c_void, flags: Option<u32>, overlapped: Option<*mut super::super::System::IO::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSSetFlow(qoshandle : super::super::Foundation:: HANDLE, flowid : u32, operation : QOS_SET_FLOW, size : u32, buffer : *const core::ffi::c_void, flags : u32, overlapped : *mut super::super::System::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { QOSSetFlow(qoshandle, flowid, operation, size, buffer, flags.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn QOSStartTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStartTrackingClient(qoshandle : super::super::Foundation:: HANDLE, destaddr : *const super::super::Networking::WinSock:: SOCKADDR, flags : u32) -> windows_core::BOOL);
    unsafe { QOSStartTrackingClient(qoshandle, destaddr, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn QOSStopTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: Option<u32>) -> windows_core::BOOL {
    windows_core::link!("qwave.dll" "system" fn QOSStopTrackingClient(qoshandle : super::super::Foundation:: HANDLE, destaddr : *const super::super::Networking::WinSock:: SOCKADDR, flags : u32) -> windows_core::BOOL);
    unsafe { QOSStopTrackingClient(qoshandle, destaddr, flags.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TcAddFilter(flowhandle: super::super::Foundation::HANDLE, pgenericfilter: *const TC_GEN_FILTER, pfilterhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcAddFilter(flowhandle : super::super::Foundation:: HANDLE, pgenericfilter : *const TC_GEN_FILTER, pfilterhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcAddFilter(flowhandle, pgenericfilter, pfilterhandle as _) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn TcAddFlow(ifchandle: super::super::Foundation::HANDLE, clflowctx: super::super::Foundation::HANDLE, flags: u32, pgenericflow: *const TC_GEN_FLOW, pflowhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcAddFlow(ifchandle : super::super::Foundation:: HANDLE, clflowctx : super::super::Foundation:: HANDLE, flags : u32, pgenericflow : *const TC_GEN_FLOW, pflowhandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcAddFlow(ifchandle, clflowctx, flags, pgenericflow, pflowhandle as _) }
}
#[inline]
pub unsafe fn TcCloseInterface(ifchandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcCloseInterface(ifchandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcCloseInterface(ifchandle) }
}
#[inline]
pub unsafe fn TcDeleteFilter(filterhandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFilter(filterhandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcDeleteFilter(filterhandle) }
}
#[inline]
pub unsafe fn TcDeleteFlow(flowhandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFlow(flowhandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcDeleteFlow(flowhandle) }
}
#[inline]
pub unsafe fn TcDeregisterClient(clienthandle: super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeregisterClient(clienthandle : super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcDeregisterClient(clienthandle) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn TcEnumerateFlows(ifchandle: super::super::Foundation::HANDLE, penumhandle: *mut super::super::Foundation::HANDLE, pflowcount: *mut u32, pbufsize: *mut u32, buffer: *mut ENUMERATION_BUFFER) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateFlows(ifchandle : super::super::Foundation:: HANDLE, penumhandle : *mut super::super::Foundation:: HANDLE, pflowcount : *mut u32, pbufsize : *mut u32, buffer : *mut ENUMERATION_BUFFER) -> u32);
    unsafe { TcEnumerateFlows(ifchandle, penumhandle as _, pflowcount as _, pbufsize as _, buffer as _) }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[inline]
pub unsafe fn TcEnumerateInterfaces(clienthandle: super::super::Foundation::HANDLE, pbuffersize: *mut u32, interfacebuffer: *mut TC_IFC_DESCRIPTOR) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateInterfaces(clienthandle : super::super::Foundation:: HANDLE, pbuffersize : *mut u32, interfacebuffer : *mut TC_IFC_DESCRIPTOR) -> u32);
    unsafe { TcEnumerateInterfaces(clienthandle, pbuffersize as _, interfacebuffer as _) }
}
#[inline]
pub unsafe fn TcGetFlowNameA(flowhandle: super::super::Foundation::HANDLE, pflowname: &mut [u8]) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameA(flowhandle : super::super::Foundation:: HANDLE, strsize : u32, pflowname : windows_core::PSTR) -> u32);
    unsafe { TcGetFlowNameA(flowhandle, pflowname.len().try_into().unwrap(), core::mem::transmute(pflowname.as_ptr())) }
}
#[inline]
pub unsafe fn TcGetFlowNameW(flowhandle: super::super::Foundation::HANDLE, pflowname: &mut [u16]) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameW(flowhandle : super::super::Foundation:: HANDLE, strsize : u32, pflowname : windows_core::PWSTR) -> u32);
    unsafe { TcGetFlowNameW(flowhandle, pflowname.len().try_into().unwrap(), core::mem::transmute(pflowname.as_ptr())) }
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn TcModifyFlow(flowhandle: super::super::Foundation::HANDLE, pgenericflow: *const TC_GEN_FLOW) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcModifyFlow(flowhandle : super::super::Foundation:: HANDLE, pgenericflow : *const TC_GEN_FLOW) -> u32);
    unsafe { TcModifyFlow(flowhandle, pgenericflow) }
}
#[inline]
pub unsafe fn TcOpenInterfaceA<P0>(pinterfacename: P0, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceA(pinterfacename : windows_core::PCSTR, clienthandle : super::super::Foundation:: HANDLE, clifcctx : super::super::Foundation:: HANDLE, pifchandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcOpenInterfaceA(pinterfacename.param().abi(), clienthandle, clifcctx, pifchandle as _) }
}
#[inline]
pub unsafe fn TcOpenInterfaceW<P0>(pinterfacename: P0, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceW(pinterfacename : windows_core::PCWSTR, clienthandle : super::super::Foundation:: HANDLE, clifcctx : super::super::Foundation:: HANDLE, pifchandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcOpenInterfaceW(pinterfacename.param().abi(), clienthandle, clifcctx, pifchandle as _) }
}
#[inline]
pub unsafe fn TcQueryFlowA<P0>(pflowname: P0, pguidparam: *const windows_core::GUID, pbuffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcQueryFlowA(pflowname : windows_core::PCSTR, pguidparam : *const windows_core::GUID, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    unsafe { TcQueryFlowA(pflowname.param().abi(), pguidparam, pbuffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TcQueryFlowW<P0>(pflowname: P0, pguidparam: *const windows_core::GUID, pbuffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcQueryFlowW(pflowname : windows_core::PCWSTR, pguidparam : *const windows_core::GUID, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    unsafe { TcQueryFlowW(pflowname.param().abi(), pguidparam, pbuffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TcQueryInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const windows_core::GUID, notifychange: bool, pbuffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcQueryInterface(ifchandle : super::super::Foundation:: HANDLE, pguidparam : *const windows_core::GUID, notifychange : bool, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    unsafe { TcQueryInterface(ifchandle, pguidparam, notifychange, pbuffersize as _, buffer as _) }
}
#[inline]
pub unsafe fn TcRegisterClient(tciversion: u32, clregctx: super::super::Foundation::HANDLE, clienthandlerlist: *const TCI_CLIENT_FUNC_LIST, pclienthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcRegisterClient(tciversion : u32, clregctx : super::super::Foundation:: HANDLE, clienthandlerlist : *const TCI_CLIENT_FUNC_LIST, pclienthandle : *mut super::super::Foundation:: HANDLE) -> u32);
    unsafe { TcRegisterClient(tciversion, clregctx, clienthandlerlist, pclienthandle as _) }
}
#[inline]
pub unsafe fn TcSetFlowA<P0>(pflowname: P0, pguidparam: *const windows_core::GUID, buffersize: u32, buffer: *const core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcSetFlowA(pflowname : windows_core::PCSTR, pguidparam : *const windows_core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
    unsafe { TcSetFlowA(pflowname.param().abi(), pguidparam, buffersize, buffer) }
}
#[inline]
pub unsafe fn TcSetFlowW<P0>(pflowname: P0, pguidparam: *const windows_core::GUID, buffersize: u32, buffer: *const core::ffi::c_void) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcSetFlowW(pflowname : windows_core::PCWSTR, pguidparam : *const windows_core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
    unsafe { TcSetFlowW(pflowname.param().abi(), pguidparam, buffersize, buffer) }
}
#[inline]
pub unsafe fn TcSetInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const windows_core::GUID, buffersize: u32, buffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcSetInterface(ifchandle : super::super::Foundation:: HANDLE, pguidparam : *const windows_core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
    unsafe { TcSetInterface(ifchandle, pguidparam, buffersize, buffer) }
}
pub const ABLE_TO_RECV_RSVP: u32 = 50002u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDRESS_LIST_DESCRIPTOR {
    pub MediaType: u32,
    pub AddressList: super::Ndis::NETWORK_ADDRESS_LIST,
}
pub const AD_FLAG_BREAK_BIT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AD_GENERAL_PARAMS {
    pub IntServAwareHopCount: u32,
    pub PathBandwidthEstimate: u32,
    pub MinimumLatency: u32,
    pub PathMTU: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AD_GUARANTEED {
    pub CTotal: u32,
    pub DTotal: u32,
    pub CSum: u32,
    pub DSum: u32,
}
pub const ALLOWED_TO_SEND_DATA: u32 = 50001u32;
pub const ANY_DEST_ADDR: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONTROL_SERVICE {
    pub Length: u32,
    pub Service: u32,
    pub Overrides: AD_GENERAL_PARAMS,
    pub Anonymous: CONTROL_SERVICE_0,
}
impl Default for CONTROL_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CONTROL_SERVICE_0 {
    pub Guaranteed: AD_GUARANTEED,
    pub ParamBuffer: [PARAM_BUFFER; 1],
}
impl Default for CONTROL_SERVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREDENTIAL_SUB_TYPE_ASCII_ID: u32 = 1u32;
pub const CREDENTIAL_SUB_TYPE_KERBEROS_TKT: u32 = 3u32;
pub const CREDENTIAL_SUB_TYPE_PGP_CERT: u32 = 5u32;
pub const CREDENTIAL_SUB_TYPE_UNICODE_ID: u32 = 2u32;
pub const CREDENTIAL_SUB_TYPE_X509_V3_CERT: u32 = 4u32;
pub const CURRENT_TCI_VERSION: u32 = 2u32;
pub const DD_TCP_DEVICE_NAME: windows_core::PCWSTR = windows_core::w!("\\Device\\Tcp");
pub const END_TO_END_QOSABILITY: u32 = 50006u32;
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENUMERATION_BUFFER {
    pub Length: u32,
    pub OwnerProcessId: u32,
    pub FlowNameLength: u16,
    pub FlowName: [u16; 256],
    pub pFlow: *mut TC_GEN_FLOW,
    pub NumberOfFilters: u32,
    pub GenericFilter: [TC_GEN_FILTER; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for ENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ERROR_ADDRESS_TYPE_NOT_SUPPORTED: u32 = 7511u32;
pub const ERROR_DS_MAPPING_EXISTS: u32 = 7518u32;
pub const ERROR_DUPLICATE_FILTER: u32 = 7509u32;
pub const ERROR_FILTER_CONFLICT: u32 = 7510u32;
pub const ERROR_INCOMPATABLE_QOS: u32 = 7513u32;
pub const ERROR_INCOMPATIBLE_TCI_VERSION: u32 = 7501u32;
pub const ERROR_INVALID_ADDRESS_TYPE: u32 = 7508u32;
pub const ERROR_INVALID_DIFFSERV_FLOW: u32 = 7517u32;
pub const ERROR_INVALID_DS_CLASS: u32 = 7520u32;
pub const ERROR_INVALID_FLOW_MODE: u32 = 7516u32;
pub const ERROR_INVALID_PEAK_RATE: u32 = 7504u32;
pub const ERROR_INVALID_QOS_PRIORITY: u32 = 7506u32;
pub const ERROR_INVALID_SD_MODE: u32 = 7505u32;
pub const ERROR_INVALID_SERVICE_TYPE: u32 = 7502u32;
pub const ERROR_INVALID_SHAPE_RATE: u32 = 7519u32;
pub const ERROR_INVALID_TOKEN_RATE: u32 = 7503u32;
pub const ERROR_INVALID_TRAFFIC_CLASS: u32 = 7507u32;
pub const ERROR_TC_NOT_SUPPORTED: u32 = 7514u32;
pub const ERROR_TC_OBJECT_LENGTH_INVALID: u32 = 7515u32;
pub const ERROR_TC_SUPPORTED_OBJECTS_EXIST: u32 = 7512u32;
pub const ERROR_TOO_MANY_CLIENTS: u32 = 7521u32;
pub const FILTERSPECV4: FilterType = FilterType(1i32);
pub const FILTERSPECV4_GPI: FilterType = FilterType(4i32);
pub const FILTERSPECV6: FilterType = FilterType(2i32);
pub const FILTERSPECV6_FLOW: FilterType = FilterType(3i32);
pub const FILTERSPECV6_GPI: FilterType = FilterType(5i32);
pub const FILTERSPEC_END: FilterType = FilterType(6i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLOWDESCRIPTOR {
    pub FlowSpec: super::super::Networking::WinSock::FLOWSPEC,
    pub NumFilters: u32,
    pub FilterList: *mut RSVP_FILTERSPEC,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for FLOWDESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FSCTL_TCP_BASE: u32 = 18u32;
pub const FVEB_UNLOCK_FLAG_AUK_OSFVEINFO: u32 = 512u32;
pub const FVEB_UNLOCK_FLAG_CACHED: u32 = 1u32;
pub const FVEB_UNLOCK_FLAG_EXTERNAL: u32 = 32u32;
pub const FVEB_UNLOCK_FLAG_MEDIA: u32 = 2u32;
pub const FVEB_UNLOCK_FLAG_NBP: u32 = 256u32;
pub const FVEB_UNLOCK_FLAG_NONE: u32 = 0u32;
pub const FVEB_UNLOCK_FLAG_PASSPHRASE: u32 = 128u32;
pub const FVEB_UNLOCK_FLAG_PIN: u32 = 16u32;
pub const FVEB_UNLOCK_FLAG_RECOVERY: u32 = 64u32;
pub const FVEB_UNLOCK_FLAG_TPM: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FilterType(pub i32);
pub const GQOS_API: u32 = 56400u32;
pub const GQOS_ERRORCODE_UNKNOWN: u32 = 4294967295u32;
pub const GQOS_ERRORVALUE_UNKNOWN: u32 = 4294967295u32;
pub const GQOS_KERNEL_TC: u32 = 56700u32;
pub const GQOS_KERNEL_TC_SYS: u32 = 56500u32;
pub const GQOS_NET_ADMISSION: u32 = 56100u32;
pub const GQOS_NET_POLICY: u32 = 56200u32;
pub const GQOS_NO_ERRORCODE: u32 = 0u32;
pub const GQOS_NO_ERRORVALUE: u32 = 0u32;
pub const GQOS_RSVP: u32 = 56300u32;
pub const GQOS_RSVP_SYS: u32 = 56600u32;
pub const GUID_QOS_BESTEFFORT_BANDWIDTH: windows_core::GUID = windows_core::GUID::from_u128(0xed885290_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_ENABLE_AVG_STATS: windows_core::GUID = windows_core::GUID::from_u128(0xbafb6d11_27c4_4801_a46f_ef8080c188c8);
pub const GUID_QOS_ENABLE_WINDOW_ADJUSTMENT: windows_core::GUID = windows_core::GUID::from_u128(0xaa966725_d3e9_4c55_b335_2a00279a1e64);
pub const GUID_QOS_FLOW_8021P_CONFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x08c1e013_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_8021P_NONCONFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x09023f91_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x1147f880_40ed_11d1_2c91_00aa00574915);
pub const GUID_QOS_FLOW_IP_CONFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x07f99a8b_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_IP_NONCONFORMING: windows_core::GUID = windows_core::GUID::from_u128(0x087a5987_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x5c82290a_515a_11d2_8e58_00c04fc9bfcb);
pub const GUID_QOS_ISSLOW_FLOW: windows_core::GUID = windows_core::GUID::from_u128(0xabf273a4_ee07_11d2_be1b_00a0c99ee63b);
pub const GUID_QOS_LATENCY: windows_core::GUID = windows_core::GUID::from_u128(0xfc408ef0_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_MAX_OUTSTANDING_SENDS: windows_core::GUID = windows_core::GUID::from_u128(0x161ffa86_6120_11d1_2c91_00aa00574915);
pub const GUID_QOS_NON_BESTEFFORT_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x185c44e0_40ed_11d1_2c91_00aa00574915);
pub const GUID_QOS_REMAINING_BANDWIDTH: windows_core::GUID = windows_core::GUID::from_u128(0xc4c51720_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_STATISTICS_BUFFER: windows_core::GUID = windows_core::GUID::from_u128(0xbb2c0980_e900_11d1_b07e_0080c71382bf);
pub const GUID_QOS_TIMER_RESOLUTION: windows_core::GUID = windows_core::GUID::from_u128(0xba10cc88_f13e_11d2_be1b_00a0c99ee63b);
pub const HIGHLY_DELAY_SENSITIVE: u32 = 4294967294u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IDPE_ATTR {
    pub PeAttribLength: u16,
    pub PeAttribType: u8,
    pub PeAttribSubType: u8,
    pub PeAttribValue: [u8; 4],
}
impl Default for IDPE_ATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IF_MIB_STATS_ID: u32 = 1u32;
pub const INFO_NOT_AVAILABLE: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union IN_ADDR_IPV4 {
    pub Addr: u32,
    pub AddrBytes: [u8; 4],
}
impl Default for IN_ADDR_IPV4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IN_ADDR_IPV6 {
    pub Addr: [u8; 16],
}
impl Default for IN_ADDR_IPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IPX_PATTERN {
    pub Src: IPX_PATTERN_0,
    pub Dest: IPX_PATTERN_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IPX_PATTERN_0 {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl Default for IPX_PATTERN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_INTFC_INFO_ID: u32 = 259u32;
pub const IP_MIB_ADDRTABLE_ENTRY_ID: u32 = 258u32;
pub const IP_MIB_STATS_ID: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IP_PATTERN {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub SrcAddr: u32,
    pub DstAddr: u32,
    pub S_un: IP_PATTERN_0,
    pub ProtocolId: u8,
    pub Reserved3: [u8; 3],
}
impl Default for IP_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IP_PATTERN_0 {
    pub S_un_ports: IP_PATTERN_0_0,
    pub S_un_icmp: IP_PATTERN_0_1,
    pub S_Spi: u32,
}
impl Default for IP_PATTERN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_PATTERN_0_1 {
    pub s_type: u8,
    pub s_code: u8,
    pub filler: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_PATTERN_0_0 {
    pub s_srcport: u16,
    pub s_dstport: u16,
}
pub const LINE_RATE: u32 = 50003u32;
pub const LOCAL_QOSABILITY: u32 = 50005u32;
pub const LOCAL_TRAFFIC_CONTROL: u32 = 50004u32;
pub const MAX_PHYSADDR_SIZE: u32 = 8u32;
pub const MAX_PLUTON_UPGRADE_FILENAME_LENGTH: u32 = 64u32;
pub const MAX_STRING_LENGTH: u32 = 256u32;
pub const MODERATELY_DELAY_SENSITIVE: u32 = 4294967293u32;
pub const OSDEVICE_TYPE_BLOCKIO_CDROM: u32 = 65539u32;
pub const OSDEVICE_TYPE_BLOCKIO_FILE: u32 = 65541u32;
pub const OSDEVICE_TYPE_BLOCKIO_HARDDISK: u32 = 65537u32;
pub const OSDEVICE_TYPE_BLOCKIO_PARTITION: u32 = 65540u32;
pub const OSDEVICE_TYPE_BLOCKIO_RAMDISK: u32 = 65542u32;
pub const OSDEVICE_TYPE_BLOCKIO_REMOVABLEDISK: u32 = 65538u32;
pub const OSDEVICE_TYPE_BLOCKIO_VIRTUALHARDDISK: u32 = 65543u32;
pub const OSDEVICE_TYPE_CIMFS: u32 = 393216u32;
pub const OSDEVICE_TYPE_COMPOSITE: u32 = 327680u32;
pub const OSDEVICE_TYPE_SERIAL: u32 = 131072u32;
pub const OSDEVICE_TYPE_UDP: u32 = 196608u32;
pub const OSDEVICE_TYPE_UNKNOWN: u32 = 0u32;
pub const OSDEVICE_TYPE_VMBUS: u32 = 262144u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PARAM_BUFFER {
    pub ParameterId: u32,
    pub Length: u32,
    pub Buffer: [u8; 1],
}
impl Default for PARAM_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PE_ATTRIB_TYPE_CREDENTIAL: u32 = 2u32;
pub const PE_ATTRIB_TYPE_POLICY_LOCATOR: u32 = 1u32;
pub const PE_TYPE_APPID: u32 = 3u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PLUTON_UPGRADE_IMAGEDATA {
    pub hashAlgID: u16,
    pub digestSize: u16,
    pub digest: [u8; 64],
    pub fileName: [u16; 64],
}
impl Default for PLUTON_UPGRADE_IMAGEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN: u32 = 1u32;
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN_ENC: u32 = 3u32;
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN: u32 = 2u32;
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN_ENC: u32 = 4u32;
pub const POSITIVE_INFINITY_RATE: u32 = 4294967294u32;
pub const QOSFlowRateCongestion: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(2i32);
pub const QOSFlowRateContentChange: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(1i32);
pub const QOSFlowRateHigherContentEncoding: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(3i32);
pub const QOSFlowRateNotApplicable: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(0i32);
pub const QOSFlowRateUserCaused: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(4i32);
pub const QOSNotifyAvailable: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(2i32);
pub const QOSNotifyCongested: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(0i32);
pub const QOSNotifyUncongested: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(1i32);
pub const QOSQueryFlowFundamentals: QOS_QUERY_FLOW = QOS_QUERY_FLOW(0i32);
pub const QOSQueryOutgoingRate: QOS_QUERY_FLOW = QOS_QUERY_FLOW(2i32);
pub const QOSQueryPacketPriority: QOS_QUERY_FLOW = QOS_QUERY_FLOW(1i32);
pub const QOSSPBASE: u32 = 50000u32;
pub const QOSSP_ERR_BASE: u32 = 56000u32;
pub const QOSSetOutgoingDSCPValue: QOS_SET_FLOW = QOS_SET_FLOW(2i32);
pub const QOSSetOutgoingRate: QOS_SET_FLOW = QOS_SET_FLOW(1i32);
pub const QOSSetTrafficType: QOS_SET_FLOW = QOS_SET_FLOW(0i32);
pub const QOSShapeAndMark: QOS_SHAPING = QOS_SHAPING(1i32);
pub const QOSShapeOnly: QOS_SHAPING = QOS_SHAPING(0i32);
pub const QOSTrafficTypeAudioVideo: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(3i32);
pub const QOSTrafficTypeBackground: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(1i32);
pub const QOSTrafficTypeBestEffort: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(0i32);
pub const QOSTrafficTypeControl: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(5i32);
pub const QOSTrafficTypeExcellentEffort: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(2i32);
pub const QOSTrafficTypeVoice: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(4i32);
pub const QOSUseNonConformantMarkings: QOS_SHAPING = QOS_SHAPING(2i32);
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QOS_DESTADDR {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub SocketAddress: *const super::super::Networking::WinSock::SOCKADDR,
    pub SocketAddressLength: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for QOS_DESTADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QOS_DIFFSERV {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub DSFieldCount: u32,
    pub DiffservRule: [u8; 1],
}
impl Default for QOS_DIFFSERV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_DIFFSERV_RULE {
    pub InboundDSField: u8,
    pub ConformingOutboundDSField: u8,
    pub NonConformingOutboundDSField: u8,
    pub ConformingUserPriority: u8,
    pub NonConformingUserPriority: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_DS_CLASS {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub DSField: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_FLOWRATE_OUTGOING {
    pub Bandwidth: u64,
    pub ShapingBehavior: QOS_SHAPING,
    pub Reason: QOS_FLOWRATE_REASON,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_FLOWRATE_REASON(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_FLOW_FUNDAMENTALS {
    pub BottleneckBandwidthSet: windows_core::BOOL,
    pub BottleneckBandwidth: u64,
    pub AvailableBandwidthSet: windows_core::BOOL,
    pub AvailableBandwidth: u64,
    pub RTTSet: windows_core::BOOL,
    pub RTT: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QOS_FRIENDLY_NAME {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub FriendlyName: [u16; 256],
}
impl Default for QOS_FRIENDLY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QOS_GENERAL_ID_BASE: u32 = 2000u32;
pub const QOS_MAX_OBJECT_STRING_LENGTH: u32 = 256u32;
pub const QOS_NON_ADAPTIVE_FLOW: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_NOTIFY_FLOW(pub i32);
pub const QOS_NOT_SPECIFIED: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_OBJECT_HDR {
    pub ObjectType: u32,
    pub ObjectLength: u32,
}
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_PACKET_PRIORITY {
    pub ConformantDSCPValue: u32,
    pub NonConformantDSCPValue: u32,
    pub ConformantL2Value: u32,
    pub NonConformantL2Value: u32,
}
pub const QOS_QUERYFLOW_FRESH: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_QUERY_FLOW(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_SD_MODE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapeDiscardMode: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_SET_FLOW(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_SHAPING(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_SHAPING_RATE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapingRate: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_TCP_TRAFFIC {
    pub ObjectHdr: QOS_OBJECT_HDR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_TRAFFIC_CLASS {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub TrafficClass: u32,
}
pub const QOS_TRAFFIC_GENERAL_ID_BASE: u32 = 4000u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QOS_TRAFFIC_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QOS_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RHANDLE(pub *mut core::ffi::c_void);
impl RHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl Default for RHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RSVP_ADSPEC {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub GeneralParams: AD_GENERAL_PARAMS,
    pub NumberOfServices: u32,
    pub Services: [CONTROL_SERVICE; 1],
}
impl Default for RSVP_ADSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSVP_DEFAULT_STYLE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RSVP_FILTERSPEC {
    pub Type: FilterType,
    pub Anonymous: RSVP_FILTERSPEC_0,
}
impl Default for RSVP_FILTERSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RSVP_FILTERSPEC_0 {
    pub FilterSpecV4: RSVP_FILTERSPEC_V4,
    pub FilterSpecV6: RSVP_FILTERSPEC_V6,
    pub FilterSpecV6Flow: RSVP_FILTERSPEC_V6_FLOW,
    pub FilterSpecV4Gpi: RSVP_FILTERSPEC_V4_GPI,
    pub FilterSpecV6Gpi: RSVP_FILTERSPEC_V6_GPI,
}
impl Default for RSVP_FILTERSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RSVP_FILTERSPEC_V4 {
    pub Address: IN_ADDR_IPV4,
    pub Unused: u16,
    pub Port: u16,
}
impl Default for RSVP_FILTERSPEC_V4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RSVP_FILTERSPEC_V4_GPI {
    pub Address: IN_ADDR_IPV4,
    pub GeneralPortId: u32,
}
impl Default for RSVP_FILTERSPEC_V4_GPI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSVP_FILTERSPEC_V6 {
    pub Address: IN_ADDR_IPV6,
    pub UnUsed: u16,
    pub Port: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSVP_FILTERSPEC_V6_FLOW {
    pub Address: IN_ADDR_IPV6,
    pub UnUsed: u8,
    pub FlowLabel: [u8; 3],
}
impl Default for RSVP_FILTERSPEC_V6_FLOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSVP_FILTERSPEC_V6_GPI {
    pub Address: IN_ADDR_IPV6,
    pub GeneralPortId: u32,
}
pub const RSVP_FIXED_FILTER_STYLE: u32 = 2u32;
pub const RSVP_OBJECT_ID_BASE: u32 = 1000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSVP_POLICY {
    pub Len: u16,
    pub Type: u16,
    pub Info: [u8; 4],
}
impl Default for RSVP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSVP_POLICY_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub NumPolicyElement: u32,
    pub PolicyElement: [RSVP_POLICY; 1],
}
impl Default for RSVP_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSVP_RESERVE_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub Style: u32,
    pub ConfirmRequest: u32,
    pub PolicyElementList: *mut RSVP_POLICY_INFO,
    pub NumFlowDesc: u32,
    pub FlowDescList: *mut FLOWDESCRIPTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for RSVP_RESERVE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSVP_SHARED_EXPLICIT_STYLE: u32 = 3u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RSVP_STATUS_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub StatusCode: u32,
    pub ExtendedStatus1: u32,
    pub ExtendedStatus2: u32,
}
pub const RSVP_WILDCARD_STYLE: u32 = 1u32;
pub const SERVICETYPE_BESTEFFORT: u32 = 1u32;
pub const SERVICETYPE_CONTROLLEDLOAD: u32 = 2u32;
pub const SERVICETYPE_GENERAL_INFORMATION: u32 = 5u32;
pub const SERVICETYPE_GUARANTEED: u32 = 3u32;
pub const SERVICETYPE_NETWORK_CONTROL: u32 = 10u32;
pub const SERVICETYPE_NETWORK_UNAVAILABLE: u32 = 4u32;
pub const SERVICETYPE_NOCHANGE: u32 = 6u32;
pub const SERVICETYPE_NONCONFORMING: u32 = 9u32;
pub const SERVICETYPE_NOTRAFFIC: u32 = 0u32;
pub const SERVICETYPE_QUALITATIVE: u32 = 13u32;
pub const SERVICE_BESTEFFORT: u32 = 2147549184u32;
pub const SERVICE_CONTROLLEDLOAD: u32 = 2147614720u32;
pub const SERVICE_GUARANTEED: u32 = 2147745792u32;
pub const SERVICE_NO_QOS_SIGNALING: u32 = 1073741824u32;
pub const SERVICE_NO_TRAFFIC_CONTROL: u32 = 2164260864u32;
pub const SERVICE_QUALITATIVE: u32 = 2149580800u32;
pub const SIPAERROR_FIRMWAREFAILURE: u32 = 196609u32;
pub const SIPAERROR_HYPERVISORFAILURE: u32 = 196613u32;
pub const SIPAERROR_INTERNALFAILURE: u32 = 196611u32;
pub const SIPAEVENTTYPE_AGGREGATION: u32 = 1073741824u32;
pub const SIPAEVENTTYPE_AUTHORITY: u32 = 393216u32;
pub const SIPAEVENTTYPE_CONTAINER: u32 = 65536u32;
pub const SIPAEVENTTYPE_DRTM: u32 = 786432u32;
pub const SIPAEVENTTYPE_ELAM: u32 = 589824u32;
pub const SIPAEVENTTYPE_ERROR: u32 = 196608u32;
pub const SIPAEVENTTYPE_INFORMATION: u32 = 131072u32;
pub const SIPAEVENTTYPE_KSR: u32 = 720896u32;
pub const SIPAEVENTTYPE_LOADEDMODULE: u32 = 458752u32;
pub const SIPAEVENTTYPE_NONMEASURED: u32 = 2147483648u32;
pub const SIPAEVENTTYPE_OSPARAMETER: u32 = 327680u32;
pub const SIPAEVENTTYPE_PREOSPARAMETER: u32 = 262144u32;
pub const SIPAEVENTTYPE_TRUSTPOINT: u32 = 524288u32;
pub const SIPAEVENTTYPE_VBS: u32 = 655360u32;
pub const SIPAEVENT_APPLICATION_RETURN: u32 = 131076u32;
pub const SIPAEVENT_APPLICATION_SVN: u32 = 131081u32;
pub const SIPAEVENT_AUTHENTICODEHASH: u32 = 458756u32;
pub const SIPAEVENT_AUTHORITYISSUER: u32 = 458757u32;
pub const SIPAEVENT_AUTHORITYPUBKEY: u32 = 393218u32;
pub const SIPAEVENT_AUTHORITYPUBLISHER: u32 = 458760u32;
pub const SIPAEVENT_AUTHORITYSERIAL: u32 = 458758u32;
pub const SIPAEVENT_AUTHORITYSHA1THUMBPRINT: u32 = 458761u32;
pub const SIPAEVENT_BITLOCKER_UNLOCK: u32 = 131077u32;
pub const SIPAEVENT_BOOTCOUNTER: u32 = 131074u32;
pub const SIPAEVENT_BOOTDEBUGGING: u32 = 262145u32;
pub const SIPAEVENT_BOOT_REVOCATION_LIST: u32 = 262146u32;
pub const SIPAEVENT_CODEINTEGRITY: u32 = 327682u32;
pub const SIPAEVENT_COUNTERID: u32 = 131079u32;
pub const SIPAEVENT_DATAEXECUTIONPREVENTION: u32 = 327684u32;
pub const SIPAEVENT_DRIVER_LOAD_POLICY: u32 = 327694u32;
pub const SIPAEVENT_DRTM_AMD_SMM_HASH: u32 = 786435u32;
pub const SIPAEVENT_DRTM_AMD_SMM_SIGNER_KEY: u32 = 786436u32;
pub const SIPAEVENT_DRTM_SMM_LEVEL: u32 = 786434u32;
pub const SIPAEVENT_DRTM_STATE_AUTH: u32 = 786433u32;
pub const SIPAEVENT_DUMPS_DISABLED: u32 = 327717u32;
pub const SIPAEVENT_DUMP_ENCRYPTION_ENABLED: u32 = 327718u32;
pub const SIPAEVENT_DUMP_ENCRYPTION_KEY_DIGEST: u32 = 327719u32;
pub const SIPAEVENT_ELAM_CONFIGURATION: u32 = 589826u32;
pub const SIPAEVENT_ELAM_KEYNAME: u32 = 589825u32;
pub const SIPAEVENT_ELAM_MEASURED: u32 = 589828u32;
pub const SIPAEVENT_ELAM_POLICY: u32 = 589827u32;
pub const SIPAEVENT_EVENTCOUNTER: u32 = 131078u32;
pub const SIPAEVENT_FILEPATH: u32 = 458753u32;
pub const SIPAEVENT_FLIGHTSIGNING: u32 = 327713u32;
pub const SIPAEVENT_HASHALGORITHMID: u32 = 458755u32;
pub const SIPAEVENT_HIBERNATION_DISABLED: u32 = 327716u32;
pub const SIPAEVENT_HYPERVISOR_BOOT_DMA_PROTECTION: u32 = 327728u32;
pub const SIPAEVENT_HYPERVISOR_DEBUG: u32 = 327693u32;
pub const SIPAEVENT_HYPERVISOR_IOMMU_POLICY: u32 = 327692u32;
pub const SIPAEVENT_HYPERVISOR_LAUNCH_TYPE: u32 = 327690u32;
pub const SIPAEVENT_HYPERVISOR_MMIO_NX_POLICY: u32 = 327696u32;
pub const SIPAEVENT_HYPERVISOR_MSR_FILTER_POLICY: u32 = 327697u32;
pub const SIPAEVENT_HYPERVISOR_PATH: u32 = 327691u32;
pub const SIPAEVENT_IDK_GENERATION_STATUS: u32 = 131084u32;
pub const SIPAEVENT_IMAGEBASE: u32 = 458759u32;
pub const SIPAEVENT_IMAGESIZE: u32 = 458754u32;
pub const SIPAEVENT_IMAGEVALIDATED: u32 = 458762u32;
pub const SIPAEVENT_INFORMATION: u32 = 131073u32;
pub const SIPAEVENT_KSR_SIGNATURE: u32 = 720897u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    pub SignAlgID: u32,
    pub SignatureLength: u32,
    pub Signature: [u8; 1],
}
impl Default for SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_LSAISO_CONFIG: u32 = 327720u32;
pub const SIPAEVENT_MODULE_ORIGINAL_FILENAME: u32 = 458765u32;
pub const SIPAEVENT_MODULE_PLUTON: u32 = 458764u32;
pub const SIPAEVENT_MODULE_SVN: u32 = 458763u32;
pub const SIPAEVENT_MODULE_VERSION: u32 = 458766u32;
pub const SIPAEVENT_MORBIT_API_STATUS: u32 = 131083u32;
pub const SIPAEVENT_MORBIT_NOT_CANCELABLE: u32 = 131080u32;
pub const SIPAEVENT_NOAUTHORITY: u32 = 393217u32;
pub const SIPAEVENT_OSDEVICE: u32 = 327688u32;
pub const SIPAEVENT_OSKERNELDEBUG: u32 = 327681u32;
pub const SIPAEVENT_OS_REVOCATION_LIST: u32 = 327699u32;
pub const SIPAEVENT_PAGEFILE_ENCRYPTION_ENABLED: u32 = 327714u32;
pub const SIPAEVENT_PHYSICALADDRESSEXTENSION: u32 = 327687u32;
pub const SIPAEVENT_PUBLISHER_OEMNAME: u32 = 458767u32;
pub const SIPAEVENT_REFS_ROLLBACK_PROTECTION_FROZEN_VOLUME_CHECKSUM: u32 = 327732u32;
pub const SIPAEVENT_REFS_ROLLBACK_PROTECTION_USER_PAYLOAD_HASH: u32 = 327733u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SIPAEVENT_REFS_ROLLBACK_PROTECTION_USER_PAYLOAD_HASH_DATA {
    pub ChecksumType: u16,
    pub ChecksumBuffer: [u8; 1],
}
impl Default for SIPAEVENT_REFS_ROLLBACK_PROTECTION_USER_PAYLOAD_HASH_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_REFS_ROLLBACK_PROTECTION_VERIFICATION_SUCCEEDED: u32 = 327734u32;
pub const SIPAEVENT_REFS_ROLLBACK_PROTECTION_VOLUME_FIRST_EVER_MOUNT: u32 = 327735u32;
pub const SIPAEVENT_REFS_VOLUME_CHECKPOINT_RECORD_CHECKSUM: u32 = 327731u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    pub CreationTime: i64,
    pub DigestLength: u32,
    pub HashAlgID: u16,
    pub Digest: [u8; 1],
}
impl Default for SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_SAFEMODE: u32 = 327685u32;
pub const SIPAEVENT_SBCP_INFO: u32 = 327721u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    pub PayloadVersion: u32,
    pub VarDataOffset: u32,
    pub HashAlgID: u16,
    pub DigestLength: u16,
    pub Options: u32,
    pub SignersCount: u32,
    pub VarData: [u8; 1],
}
impl Default for SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_SI_POLICY: u32 = 327695u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_SI_POLICY_CERTIFICATE_PAYLOAD {
    pub PublisherCommonNameLength: u16,
    pub IssuerCommonNameLength: u16,
    pub HashAlgID: u32,
    pub DigestLength: u16,
    pub VarLengthData: [u8; 1],
}
impl Default for SIPAEVENT_SI_POLICY_CERTIFICATE_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_SI_POLICY_PAYLOAD {
    pub PolicyVersion: u64,
    pub PolicyNameLength: u16,
    pub HashAlgID: u16,
    pub DigestLength: u32,
    pub VarLengthData: [u8; 1],
}
impl Default for SIPAEVENT_SI_POLICY_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_SI_POLICY_SIGNER: u32 = 327729u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_SI_POLICY_SIGNER_PAYLOAD {
    pub RootID: u32,
    pub CertificatesLength: u32,
    pub CertificatesCount: u16,
    pub PolicyNameLength: u16,
    pub EKUsLength: u16,
    pub EKUsCount: u16,
    pub VarLengthData: [u8; 1],
}
impl Default for SIPAEVENT_SI_POLICY_SIGNER_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_SI_POLICY_UPDATE_SIGNER: u32 = 327730u32;
pub const SIPAEVENT_SMT_STATUS: u32 = 327700u32;
pub const SIPAEVENT_SVN_CHAIN_STATUS: u32 = 131082u32;
pub const SIPAEVENT_SYSTEMROOT: u32 = 327689u32;
pub const SIPAEVENT_TESTSIGNING: u32 = 327683u32;
pub const SIPAEVENT_TRANSFER_CONTROL: u32 = 131075u32;
pub const SIPAEVENT_VBS_DUMP_USES_AMEROOT: u32 = 655369u32;
pub const SIPAEVENT_VBS_HVCI_POLICY: u32 = 655367u32;
pub const SIPAEVENT_VBS_IOMMU_REQUIRED: u32 = 655363u32;
pub const SIPAEVENT_VBS_MANDATORY_ENFORCEMENT: u32 = 655366u32;
pub const SIPAEVENT_VBS_MICROSOFT_BOOT_CHAIN_REQUIRED: u32 = 655368u32;
pub const SIPAEVENT_VBS_MMIO_NX_REQUIRED: u32 = 655364u32;
pub const SIPAEVENT_VBS_MSR_FILTERING_REQUIRED: u32 = 655365u32;
pub const SIPAEVENT_VBS_SECUREBOOT_REQUIRED: u32 = 655362u32;
pub const SIPAEVENT_VBS_VSM_NOSECRETS_ENFORCED: u32 = 655370u32;
pub const SIPAEVENT_VBS_VSM_REQUIRED: u32 = 655361u32;
pub const SIPAEVENT_VSM_DRTM_KEYROLL_DETECTED: u32 = 327739u32;
pub const SIPAEVENT_VSM_IDKS_INFO: u32 = 327715u32;
pub const SIPAEVENT_VSM_IDK_INFO: u32 = 327712u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    pub KeyAlgID: u32,
    pub Anonymous: SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0,
}
impl Default for SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    pub RsaKeyInfo: SIPAEVENT_VSM_IDK_RSA_INFO,
}
impl Default for SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIPAEVENT_VSM_IDK_RSA_INFO {
    pub KeyBitLength: u32,
    pub PublicExpLengthBytes: u32,
    pub ModulusSizeBytes: u32,
    pub PublicKeyData: [u8; 1],
}
impl Default for SIPAEVENT_VSM_IDK_RSA_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIPAEVENT_VSM_LAUNCH_TYPE: u32 = 327698u32;
pub const SIPAEVENT_VSM_SEALED_SI_POLICY: u32 = 327738u32;
pub const SIPAEVENT_VSM_SRTM_ANTI_ROLLBACK_COUNTER: u32 = 327741u32;
pub const SIPAEVENT_VSM_SRTM_UNSEAL_POLICY: u32 = 327740u32;
pub const SIPAEVENT_VTL1_DUMP_CONFIG: u32 = 327744u32;
pub const SIPAEVENT_WINPE: u32 = 327686u32;
pub const SIPAEV_ACTION: u32 = 5u32;
pub const SIPAEV_AMD_ABL_1: u32 = 33290u32;
pub const SIPAEV_AMD_ABL_10: u32 = 33299u32;
pub const SIPAEV_AMD_ABL_11: u32 = 33300u32;
pub const SIPAEV_AMD_ABL_12: u32 = 33301u32;
pub const SIPAEV_AMD_ABL_13: u32 = 33302u32;
pub const SIPAEV_AMD_ABL_14: u32 = 33303u32;
pub const SIPAEV_AMD_ABL_15: u32 = 33304u32;
pub const SIPAEV_AMD_ABL_16: u32 = 33305u32;
pub const SIPAEV_AMD_ABL_17: u32 = 33306u32;
pub const SIPAEV_AMD_ABL_18: u32 = 33307u32;
pub const SIPAEV_AMD_ABL_19: u32 = 33308u32;
pub const SIPAEV_AMD_ABL_2: u32 = 33291u32;
pub const SIPAEV_AMD_ABL_20: u32 = 33309u32;
pub const SIPAEV_AMD_ABL_21: u32 = 33310u32;
pub const SIPAEV_AMD_ABL_22: u32 = 33311u32;
pub const SIPAEV_AMD_ABL_23: u32 = 33312u32;
pub const SIPAEV_AMD_ABL_24: u32 = 33313u32;
pub const SIPAEV_AMD_ABL_25: u32 = 33314u32;
pub const SIPAEV_AMD_ABL_26: u32 = 33315u32;
pub const SIPAEV_AMD_ABL_27: u32 = 33316u32;
pub const SIPAEV_AMD_ABL_28: u32 = 33317u32;
pub const SIPAEV_AMD_ABL_29: u32 = 33318u32;
pub const SIPAEV_AMD_ABL_3: u32 = 33292u32;
pub const SIPAEV_AMD_ABL_30: u32 = 33319u32;
pub const SIPAEV_AMD_ABL_31: u32 = 33320u32;
pub const SIPAEV_AMD_ABL_32: u32 = 33321u32;
pub const SIPAEV_AMD_ABL_33: u32 = 33322u32;
pub const SIPAEV_AMD_ABL_34: u32 = 33323u32;
pub const SIPAEV_AMD_ABL_35: u32 = 33324u32;
pub const SIPAEV_AMD_ABL_36: u32 = 33325u32;
pub const SIPAEV_AMD_ABL_37: u32 = 33326u32;
pub const SIPAEV_AMD_ABL_38: u32 = 33327u32;
pub const SIPAEV_AMD_ABL_39: u32 = 33328u32;
pub const SIPAEV_AMD_ABL_4: u32 = 33293u32;
pub const SIPAEV_AMD_ABL_40: u32 = 33329u32;
pub const SIPAEV_AMD_ABL_41: u32 = 33330u32;
pub const SIPAEV_AMD_ABL_42: u32 = 33331u32;
pub const SIPAEV_AMD_ABL_43: u32 = 33332u32;
pub const SIPAEV_AMD_ABL_44: u32 = 33333u32;
pub const SIPAEV_AMD_ABL_45: u32 = 33334u32;
pub const SIPAEV_AMD_ABL_46: u32 = 33335u32;
pub const SIPAEV_AMD_ABL_47: u32 = 33336u32;
pub const SIPAEV_AMD_ABL_48: u32 = 33337u32;
pub const SIPAEV_AMD_ABL_5: u32 = 33294u32;
pub const SIPAEV_AMD_ABL_6: u32 = 33295u32;
pub const SIPAEV_AMD_ABL_7: u32 = 33296u32;
pub const SIPAEV_AMD_ABL_8: u32 = 33297u32;
pub const SIPAEV_AMD_ABL_9: u32 = 33298u32;
pub const SIPAEV_AMD_ABL_TOC: u32 = 33355u32;
pub const SIPAEV_AMD_AGESA_DRV: u32 = 33538u32;
pub const SIPAEV_AMD_BASE_2: u32 = 33280u32;
pub const SIPAEV_AMD_DRTM_DRV: u32 = 33537u32;
pub const SIPAEV_AMD_FTPM_DRV: u32 = 33536u32;
pub const SIPAEV_AMD_GMI3: u32 = 33363u32;
pub const SIPAEV_AMD_IP_DISCOVERY: u32 = 33351u32;
pub const SIPAEV_AMD_MID_SMU: u32 = 33338u32;
pub const SIPAEV_AMD_MP2_CONFIG: u32 = 33288u32;
pub const SIPAEV_AMD_MP2_FW: u32 = 33289u32;
pub const SIPAEV_AMD_MP5: u32 = 33361u32;
pub const SIPAEV_AMD_MPCCX: u32 = 33362u32;
pub const SIPAEV_AMD_MPIO_FW: u32 = 33360u32;
pub const SIPAEV_AMD_NO_ACTION: u32 = 3u32;
pub const SIPAEV_AMD_PMFW0: u32 = 33287u32;
pub const SIPAEV_AMD_PMU1: u32 = 33358u32;
pub const SIPAEV_AMD_PMU1_DATA: u32 = 33356u32;
pub const SIPAEV_AMD_PMU2: u32 = 33359u32;
pub const SIPAEV_AMD_PMU2_DATA: u32 = 33357u32;
pub const SIPAEV_AMD_PM_FW1: u32 = 33339u32;
pub const SIPAEV_AMD_PSP_BL_END: u32 = 33535u32;
pub const SIPAEV_AMD_PSP_BL_STAGE_1: u32 = 33282u32;
pub const SIPAEV_AMD_PSP_BL_STAGE_2: u32 = 33285u32;
pub const SIPAEV_AMD_PSP_DF_RIB0: u32 = 33367u32;
pub const SIPAEV_AMD_PSP_DF_RIB1: u32 = 33368u32;
pub const SIPAEV_AMD_PSP_DF_RIB10: u32 = 33377u32;
pub const SIPAEV_AMD_PSP_DF_RIB11: u32 = 33378u32;
pub const SIPAEV_AMD_PSP_DF_RIB12: u32 = 33379u32;
pub const SIPAEV_AMD_PSP_DF_RIB13: u32 = 33380u32;
pub const SIPAEV_AMD_PSP_DF_RIB14: u32 = 33381u32;
pub const SIPAEV_AMD_PSP_DF_RIB15: u32 = 33382u32;
pub const SIPAEV_AMD_PSP_DF_RIB2: u32 = 33369u32;
pub const SIPAEV_AMD_PSP_DF_RIB3: u32 = 33370u32;
pub const SIPAEV_AMD_PSP_DF_RIB4: u32 = 33371u32;
pub const SIPAEV_AMD_PSP_DF_RIB5: u32 = 33372u32;
pub const SIPAEV_AMD_PSP_DF_RIB6: u32 = 33373u32;
pub const SIPAEV_AMD_PSP_DF_RIB7: u32 = 33374u32;
pub const SIPAEV_AMD_PSP_DF_RIB8: u32 = 33375u32;
pub const SIPAEV_AMD_PSP_DF_RIB9: u32 = 33376u32;
pub const SIPAEV_AMD_PSP_DF_RIB_TOC: u32 = 33366u32;
pub const SIPAEV_AMD_PSP_END: u32 = 33791u32;
pub const SIPAEV_AMD_PSP_KEYDB: u32 = 33283u32;
pub const SIPAEV_AMD_PSP_L0_SEC_POL: u32 = 33286u32;
pub const SIPAEV_AMD_PSP_L1_SEC_POL: u32 = 33350u32;
pub const SIPAEV_AMD_PSP_SPIROM_CONFIG: u32 = 33365u32;
pub const SIPAEV_AMD_PSP_TOS_KEYDB: u32 = 33354u32;
pub const SIPAEV_AMD_SECURE_DEBUG_UNLOCK: u32 = 33383u32;
pub const SIPAEV_AMD_SL_EVENT_BASE: u32 = 32768u32;
pub const SIPAEV_AMD_SL_LOAD: u32 = 32769u32;
pub const SIPAEV_AMD_SL_LOAD_1: u32 = 32774u32;
pub const SIPAEV_AMD_SL_PSP_FW_SPLT: u32 = 32770u32;
pub const SIPAEV_AMD_SL_PUB_KEY: u32 = 32772u32;
pub const SIPAEV_AMD_SL_SEPARATOR: u32 = 32775u32;
pub const SIPAEV_AMD_SL_SVN: u32 = 32773u32;
pub const SIPAEV_AMD_SL_TSME_RB_FUSE: u32 = 32771u32;
pub const SIPAEV_AMD_SPL_TABLE_FW: u32 = 33284u32;
pub const SIPAEV_AMD_SPL_TABLE_ROM: u32 = 33281u32;
pub const SIPAEV_AMD_SYS_DRV: u32 = 33352u32;
pub const SIPAEV_AMD_TOS: u32 = 33353u32;
pub const SIPAEV_AMD_TPMLITE: u32 = 33364u32;
pub const SIPAEV_AMD_VBL_1: u32 = 33340u32;
pub const SIPAEV_AMD_VBL_10: u32 = 33349u32;
pub const SIPAEV_AMD_VBL_2: u32 = 33341u32;
pub const SIPAEV_AMD_VBL_3: u32 = 33342u32;
pub const SIPAEV_AMD_VBL_4: u32 = 33343u32;
pub const SIPAEV_AMD_VBL_5: u32 = 33344u32;
pub const SIPAEV_AMD_VBL_6: u32 = 33345u32;
pub const SIPAEV_AMD_VBL_7: u32 = 33346u32;
pub const SIPAEV_AMD_VBL_8: u32 = 33347u32;
pub const SIPAEV_AMD_VBL_9: u32 = 33348u32;
pub const SIPAEV_ARM_BASE: u32 = 36864u32;
pub const SIPAEV_ARM_DCE: u32 = 36866u32;
pub const SIPAEV_ARM_DCE_PUBKEY: u32 = 36867u32;
pub const SIPAEV_ARM_DCE_SECONDARY: u32 = 36872u32;
pub const SIPAEV_ARM_DEBUG_CONFIG: u32 = 36870u32;
pub const SIPAEV_ARM_DLME: u32 = 36868u32;
pub const SIPAEV_ARM_DLME_ENTRY_POINT: u32 = 36869u32;
pub const SIPAEV_ARM_DLME_PUBKEY: u32 = 36875u32;
pub const SIPAEV_ARM_DLME_SVN: u32 = 36876u32;
pub const SIPAEV_ARM_NONSECURE_CONFIG: u32 = 36871u32;
pub const SIPAEV_ARM_NO_ACTION: u32 = 36877u32;
pub const SIPAEV_ARM_PCR_SCHEMA: u32 = 36865u32;
pub const SIPAEV_ARM_SECURE_INT_DISABLE: u32 = 36878u32;
pub const SIPAEV_ARM_SEPARATOR: u32 = 36874u32;
pub const SIPAEV_ARM_TZFW: u32 = 36873u32;
pub const SIPAEV_COMPACT_HASH: u32 = 12u32;
pub const SIPAEV_CPU_MICROCODE: u32 = 9u32;
pub const SIPAEV_EFI_ACTION: u32 = 2147483655u32;
pub const SIPAEV_EFI_BOOT_SERVICES_APPLICATION: u32 = 2147483651u32;
pub const SIPAEV_EFI_BOOT_SERVICES_DRIVER: u32 = 2147483652u32;
pub const SIPAEV_EFI_EVENT_BASE: u32 = 2147483648u32;
pub const SIPAEV_EFI_GPT_EVENT: u32 = 2147483654u32;
pub const SIPAEV_EFI_HANDOFF_TABLES: u32 = 2147483657u32;
pub const SIPAEV_EFI_HANDOFF_TABLES2: u32 = 2147483659u32;
pub const SIPAEV_EFI_HCRTM_EVENT: u32 = 2147483664u32;
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB: u32 = 2147483656u32;
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB2: u32 = 2147483658u32;
pub const SIPAEV_EFI_RUNTIME_SERVICES_DRIVER: u32 = 2147483653u32;
pub const SIPAEV_EFI_SPDM_FIRMWARE_BLOB: u32 = 2147483873u32;
pub const SIPAEV_EFI_SPDM_FIRMWARE_CONFIG: u32 = 2147483874u32;
pub const SIPAEV_EFI_VARIABLE_AUTHORITY: u32 = 2147483872u32;
pub const SIPAEV_EFI_VARIABLE_BOOT: u32 = 2147483650u32;
pub const SIPAEV_EFI_VARIABLE_BOOT2: u32 = 2147483660u32;
pub const SIPAEV_EFI_VARIABLE_DRIVER_CONFIG: u32 = 2147483649u32;
pub const SIPAEV_EVENT_TAG: u32 = 6u32;
pub const SIPAEV_IPL: u32 = 13u32;
pub const SIPAEV_IPL_PARTITION_DATA: u32 = 14u32;
pub const SIPAEV_NONHOST_CODE: u32 = 15u32;
pub const SIPAEV_NONHOST_CONFIG: u32 = 16u32;
pub const SIPAEV_NONHOST_INFO: u32 = 17u32;
pub const SIPAEV_NO_ACTION: u32 = 3u32;
pub const SIPAEV_OMIT_BOOT_DEVICE_EVENTS: u32 = 18u32;
pub const SIPAEV_PLATFORM_CONFIG_FLAGS: u32 = 10u32;
pub const SIPAEV_POST_CODE: u32 = 1u32;
pub const SIPAEV_PREBOOT_CERT: u32 = 0u32;
pub const SIPAEV_SEPARATOR: u32 = 4u32;
pub const SIPAEV_S_CRTM_CONTENTS: u32 = 7u32;
pub const SIPAEV_S_CRTM_VERSION: u32 = 8u32;
pub const SIPAEV_TABLE_OF_DEVICES: u32 = 11u32;
pub const SIPAEV_TXT_BIOSAC_REG_DATA: u32 = 1034u32;
pub const SIPAEV_TXT_BOOT_POL_HASH: u32 = 1050u32;
pub const SIPAEV_TXT_BPM_HASH: u32 = 1047u32;
pub const SIPAEV_TXT_BPM_INFO_HASH: u32 = 1049u32;
pub const SIPAEV_TXT_CAP_VALUE: u32 = 1279u32;
pub const SIPAEV_TXT_COLD_BOOT_BIOS_HASH: u32 = 1045u32;
pub const SIPAEV_TXT_COMBINED_HASH: u32 = 1027u32;
pub const SIPAEV_TXT_CPU_SCRTM_STAT: u32 = 1035u32;
pub const SIPAEV_TXT_ELEMENTS_HASH: u32 = 1037u32;
pub const SIPAEV_TXT_EVENT_BASE: u32 = 1024u32;
pub const SIPAEV_TXT_HASH_START: u32 = 1026u32;
pub const SIPAEV_TXT_KM_HASH: u32 = 1046u32;
pub const SIPAEV_TXT_KM_INFO_HASH: u32 = 1048u32;
pub const SIPAEV_TXT_LCP_AUTHORITIES_HASH: u32 = 1043u32;
pub const SIPAEV_TXT_LCP_CONTROL_HASH: u32 = 1036u32;
pub const SIPAEV_TXT_LCP_DETAILS_HASH: u32 = 1042u32;
pub const SIPAEV_TXT_LCP_HASH: u32 = 1041u32;
pub const SIPAEV_TXT_MLE_HASH: u32 = 1028u32;
pub const SIPAEV_TXT_NV_INFO_HASH: u32 = 1044u32;
pub const SIPAEV_TXT_OSSINITDATA_CAP_HASH: u32 = 1039u32;
pub const SIPAEV_TXT_PCR_MAPPING: u32 = 1025u32;
pub const SIPAEV_TXT_RANDOM_VALUE: u32 = 1278u32;
pub const SIPAEV_TXT_SINIT_PUBKEY_HASH: u32 = 1040u32;
pub const SIPAEV_TXT_STM_HASH: u32 = 1038u32;
pub const SIPAEV_UNUSED: u32 = 2u32;
pub const SIPAHDRSIGNATURE: u32 = 1279476311u32;
pub const SIPAKSRHDRSIGNATURE: u32 = 1297240907u32;
pub const SIPALOGVERSION: u32 = 1u32;
pub const TCBASE: u32 = 7500u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct TCG_PCClientPCREventStruct {
    pub pcrIndex: u32,
    pub eventType: u32,
    pub digest: [u8; 20],
    pub eventDataSize: u32,
    pub event: [u8; 1],
}
impl Default for TCG_PCClientPCREventStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct TCG_PCClientTaggedEventStruct {
    pub EventID: u32,
    pub EventDataSize: u32,
    pub EventData: [u8; 1],
}
impl Default for TCG_PCClientTaggedEventStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TCI_ADD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TCI_CLIENT_FUNC_LIST {
    pub ClNotifyHandler: TCI_NOTIFY_HANDLER,
    pub ClAddFlowCompleteHandler: TCI_ADD_FLOW_COMPLETE_HANDLER,
    pub ClModifyFlowCompleteHandler: TCI_MOD_FLOW_COMPLETE_HANDLER,
    pub ClDeleteFlowCompleteHandler: TCI_DEL_FLOW_COMPLETE_HANDLER,
}
pub type TCI_DEL_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
pub type TCI_MOD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
pub type TCI_NOTIFY_HANDLER = Option<unsafe extern "system" fn(clregctx: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, event: u32, subcode: super::super::Foundation::HANDLE, bufsize: u32, buffer: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TC_GEN_FILTER {
    pub AddressType: u16,
    pub PatternSize: u32,
    pub Pattern: *mut core::ffi::c_void,
    pub Mask: *mut core::ffi::c_void,
}
impl Default for TC_GEN_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TC_GEN_FLOW {
    pub SendingFlowspec: super::super::Networking::WinSock::FLOWSPEC,
    pub ReceivingFlowspec: super::super::Networking::WinSock::FLOWSPEC,
    pub TcObjectsLength: u32,
    pub TcObjects: [QOS_OBJECT_HDR; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl Default for TC_GEN_FLOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TC_IFC_DESCRIPTOR {
    pub Length: u32,
    pub pInterfaceName: windows_core::PWSTR,
    pub pInterfaceID: windows_core::PWSTR,
    pub AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}
pub const TC_NONCONF_BORROW: u32 = 0u32;
pub const TC_NONCONF_BORROW_PLUS: u32 = 3u32;
pub const TC_NONCONF_DISCARD: u32 = 2u32;
pub const TC_NONCONF_SHAPE: u32 = 1u32;
pub const TC_NOTIFY_FLOW_CLOSE: u32 = 5u32;
pub const TC_NOTIFY_IFC_CHANGE: u32 = 3u32;
pub const TC_NOTIFY_IFC_CLOSE: u32 = 2u32;
pub const TC_NOTIFY_IFC_UP: u32 = 1u32;
pub const TC_NOTIFY_PARAM_CHANGED: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TC_SUPPORTED_INFO_BUFFER {
    pub InstanceIDLength: u16,
    pub InstanceID: [u16; 256],
    pub InterfaceLuid: u64,
    pub AddrListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl Default for TC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_256: u32 = 32u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_384: u32 = 64u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_512: u32 = 128u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA_1: u32 = 1u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_256: u32 = 2u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_384: u32 = 4u32;
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_512: u32 = 8u32;
pub const WBCL_DIGEST_ALG_BITMAP_SM3_256: u32 = 16u32;
pub const WBCL_DIGEST_ALG_ID_SHA3_256: u32 = 39u32;
pub const WBCL_DIGEST_ALG_ID_SHA3_384: u32 = 40u32;
pub const WBCL_DIGEST_ALG_ID_SHA3_512: u32 = 41u32;
pub const WBCL_DIGEST_ALG_ID_SHA_1: u32 = 4u32;
pub const WBCL_DIGEST_ALG_ID_SHA_2_256: u32 = 11u32;
pub const WBCL_DIGEST_ALG_ID_SHA_2_384: u32 = 12u32;
pub const WBCL_DIGEST_ALG_ID_SHA_2_512: u32 = 13u32;
pub const WBCL_DIGEST_ALG_ID_SM3_256: u32 = 18u32;
pub const WBCL_HASH_LEN_SHA1: u32 = 20u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WBCL_Iterator {
    pub firstElementPtr: *mut core::ffi::c_void,
    pub logSize: u32,
    pub currentElementPtr: *mut core::ffi::c_void,
    pub currentElementSize: u32,
    pub digestSize: u16,
    pub logFormat: u16,
    pub numberOfDigests: u32,
    pub digestSizes: *mut core::ffi::c_void,
    pub supportedAlgorithms: u32,
    pub hashAlgorithm: u16,
}
impl Default for WBCL_Iterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WBCL_LogHdr {
    pub signature: u32,
    pub version: u32,
    pub entries: u32,
    pub length: u32,
}
pub const WBCL_MAX_PLUTON_UPGRADE_HASH_LEN: u32 = 64u32;
pub const ioctl_code: u32 = 1u32;
pub const mCOMPANY: u32 = 402653184u32;
pub const mIOC_IN: u32 = 2147483648u32;
pub const mIOC_OUT: u32 = 1073741824u32;
pub const mIOC_VENDOR: u32 = 67108864u32;
