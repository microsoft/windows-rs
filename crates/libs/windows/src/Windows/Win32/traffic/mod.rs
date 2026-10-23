#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcAddFilter(flowhandle: super::HANDLE, pgenericfilter: PTC_GEN_FILTER, pfilterhandle: super::PHANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcAddFilter(flowhandle : super::HANDLE, pgenericfilter : PTC_GEN_FILTER, pfilterhandle : super::PHANDLE) -> super::ULONG);
    unsafe { TcAddFilter(flowhandle, pgenericfilter, pfilterhandle as _) }
}
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcAddFlow(ifchandle: super::HANDLE, clflowctx: super::HANDLE, flags: super::ULONG, pgenericflow: PTC_GEN_FLOW, pflowhandle: super::PHANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcAddFlow(ifchandle : super::HANDLE, clflowctx : super::HANDLE, flags : super::ULONG, pgenericflow : PTC_GEN_FLOW, pflowhandle : super::PHANDLE) -> super::ULONG);
    unsafe { TcAddFlow(ifchandle, clflowctx, flags, pgenericflow, pflowhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcCloseInterface(ifchandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcCloseInterface(ifchandle : super::HANDLE) -> super::ULONG);
    unsafe { TcCloseInterface(ifchandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcDeleteFilter(filterhandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFilter(filterhandle : super::HANDLE) -> super::ULONG);
    unsafe { TcDeleteFilter(filterhandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcDeleteFlow(flowhandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFlow(flowhandle : super::HANDLE) -> super::ULONG);
    unsafe { TcDeleteFlow(flowhandle) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcDeregisterClient(clienthandle: super::HANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcDeregisterClient(clienthandle : super::HANDLE) -> super::ULONG);
    unsafe { TcDeregisterClient(clienthandle) }
}
#[cfg(all(feature = "minwindef", feature = "qos", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcEnumerateFlows(ifchandle: super::HANDLE, penumhandle: super::PHANDLE, pflowcount: super::PULONG, pbufsize: super::PULONG, buffer: PENUMERATION_BUFFER) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateFlows(ifchandle : super::HANDLE, penumhandle : super::PHANDLE, pflowcount : super::PULONG, pbufsize : super::PULONG, buffer : PENUMERATION_BUFFER) -> super::ULONG);
    unsafe { TcEnumerateFlows(ifchandle, penumhandle as _, pflowcount as _, pbufsize as _, buffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntddndis", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcEnumerateInterfaces(clienthandle: super::HANDLE, pbuffersize: super::PULONG, interfacebuffer: PTC_IFC_DESCRIPTOR) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateInterfaces(clienthandle : super::HANDLE, pbuffersize : super::PULONG, interfacebuffer : PTC_IFC_DESCRIPTOR) -> super::ULONG);
    unsafe { TcEnumerateInterfaces(clienthandle, pbuffersize as _, interfacebuffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcGetFlowNameA(flowhandle: super::HANDLE, strsize: super::ULONG, pflowname: windows_core::PSTR) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameA(flowhandle : super::HANDLE, strsize : super::ULONG, pflowname : windows_core::PSTR) -> super::ULONG);
    unsafe { TcGetFlowNameA(flowhandle, strsize, pflowname) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcGetFlowNameW(flowhandle: super::HANDLE, strsize: super::ULONG, pflowname: windows_core::PWSTR) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameW(flowhandle : super::HANDLE, strsize : super::ULONG, pflowname : windows_core::PWSTR) -> super::ULONG);
    unsafe { TcGetFlowNameW(flowhandle, strsize, pflowname) }
}
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcModifyFlow(flowhandle: super::HANDLE, pgenericflow: PTC_GEN_FLOW) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcModifyFlow(flowhandle : super::HANDLE, pgenericflow : PTC_GEN_FLOW) -> super::ULONG);
    unsafe { TcModifyFlow(flowhandle, pgenericflow) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcOpenInterfaceA<P0>(pinterfacename: P0, clienthandle: super::HANDLE, clifcctx: super::HANDLE, pifchandle: super::PHANDLE) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceA(pinterfacename : windows_core::PCSTR, clienthandle : super::HANDLE, clifcctx : super::HANDLE, pifchandle : super::PHANDLE) -> super::ULONG);
    unsafe { TcOpenInterfaceA(pinterfacename.param().abi(), clienthandle, clifcctx, pifchandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcOpenInterfaceW<P0>(pinterfacename: P0, clienthandle: super::HANDLE, clifcctx: super::HANDLE, pifchandle: super::PHANDLE) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceW(pinterfacename : windows_core::PCWSTR, clienthandle : super::HANDLE, clifcctx : super::HANDLE, pifchandle : super::PHANDLE) -> super::ULONG);
    unsafe { TcOpenInterfaceW(pinterfacename.param().abi(), clienthandle, clifcctx, pifchandle as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcQueryFlowA<P0>(pflowname: P0, pguidparam: super::LPGUID, pbuffersize: super::PULONG, buffer: super::PVOID) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcQueryFlowA(pflowname : windows_core::PCSTR, pguidparam : super::LPGUID, pbuffersize : super::PULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcQueryFlowA(pflowname.param().abi(), pguidparam, pbuffersize as _, buffer as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcQueryFlowW<P0>(pflowname: P0, pguidparam: super::LPGUID, pbuffersize: super::PULONG, buffer: super::PVOID) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcQueryFlowW(pflowname : windows_core::PCWSTR, pguidparam : super::LPGUID, pbuffersize : super::PULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcQueryFlowW(pflowname.param().abi(), pguidparam, pbuffersize as _, buffer as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcQueryInterface(ifchandle: super::HANDLE, pguidparam: super::LPGUID, notifychange: super::BOOLEAN, pbuffersize: super::PULONG, buffer: super::PVOID) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcQueryInterface(ifchandle : super::HANDLE, pguidparam : super::LPGUID, notifychange : super::BOOLEAN, pbuffersize : super::PULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcQueryInterface(ifchandle, pguidparam, notifychange, pbuffersize as _, buffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcRegisterClient(tciversion: super::ULONG, clregctx: super::HANDLE, clienthandlerlist: PTCI_CLIENT_FUNC_LIST, pclienthandle: super::PHANDLE) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcRegisterClient(tciversion : super::ULONG, clregctx : super::HANDLE, clienthandlerlist : PTCI_CLIENT_FUNC_LIST, pclienthandle : super::PHANDLE) -> super::ULONG);
    unsafe { TcRegisterClient(tciversion, clregctx, clienthandlerlist, pclienthandle as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcSetFlowA<P0>(pflowname: P0, pguidparam: super::LPGUID, buffersize: super::ULONG, buffer: super::PVOID) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcSetFlowA(pflowname : windows_core::PCSTR, pguidparam : super::LPGUID, buffersize : super::ULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcSetFlowA(pflowname.param().abi(), pguidparam, buffersize, buffer) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcSetFlowW<P0>(pflowname: P0, pguidparam: super::LPGUID, buffersize: super::ULONG, buffer: super::PVOID) -> super::ULONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcSetFlowW(pflowname : windows_core::PCWSTR, pguidparam : super::LPGUID, buffersize : super::ULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcSetFlowW(pflowname.param().abi(), pguidparam, buffersize, buffer) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TcSetInterface(ifchandle: super::HANDLE, pguidparam: super::LPGUID, buffersize: super::ULONG, buffer: super::PVOID) -> super::ULONG {
    windows_core::link!("traffic.dll" "system" fn TcSetInterface(ifchandle : super::HANDLE, pguidparam : super::LPGUID, buffersize : super::ULONG, buffer : super::PVOID) -> super::ULONG);
    unsafe { TcSetInterface(ifchandle, pguidparam, buffersize, buffer) }
}
#[repr(C)]
#[cfg(all(feature = "ntddndis", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADDRESS_LIST_DESCRIPTOR {
    pub MediaType: super::ULONG,
    pub AddressList: super::NETWORK_ADDRESS_LIST,
}
pub const CURRENT_TCI_VERSION: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ENUMERATION_BUFFER {
    pub Length: super::ULONG,
    pub OwnerProcessId: super::ULONG,
    pub FlowNameLength: super::USHORT,
    pub FlowName: [super::WCHAR; 256],
    pub pFlow: PTC_GEN_FLOW,
    pub NumberOfFilters: super::ULONG,
    pub GenericFilter: [TC_GEN_FILTER; 1],
}
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
impl Default for ENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IPX_PATTERN {
    pub Src: IPX_PATTERN_0,
    pub Dest: IPX_PATTERN_0,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IPX_PATTERN_0 {
    pub NetworkAddress: super::ULONG,
    pub NodeAddress: [super::UCHAR; 6],
    pub Socket: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for IPX_PATTERN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IP_PATTERN {
    pub Reserved1: super::ULONG,
    pub Reserved2: super::ULONG,
    pub SrcAddr: super::ULONG,
    pub DstAddr: super::ULONG,
    pub S_un: IP_PATTERN_0,
    pub ProtocolId: super::UCHAR,
    pub Reserved3: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for IP_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union IP_PATTERN_0 {
    pub S_un_ports: IP_PATTERN_0_0,
    pub S_un_icmp: IP_PATTERN_0_1,
    pub S_Spi: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IP_PATTERN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IP_PATTERN_0_0 {
    pub s_srcport: super::USHORT,
    pub s_dstport: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IP_PATTERN_0_1 {
    pub s_type: super::UCHAR,
    pub s_code: super::UCHAR,
    pub filler: super::USHORT,
}
pub const MAX_STRING_LENGTH: i32 = 256;
#[cfg(all(feature = "ntddndis", feature = "shared"))]
pub type PADDRESS_LIST_DESCRIPTOR = *mut ADDRESS_LIST_DESCRIPTOR;
#[cfg(all(feature = "qos", feature = "shared", feature = "winnt"))]
pub type PENUMERATION_BUFFER = *mut ENUMERATION_BUFFER;
#[cfg(feature = "shared")]
pub type PIPX_PATTERN = *mut IPX_PATTERN;
#[cfg(feature = "shared")]
pub type PIP_PATTERN = *mut IP_PATTERN;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTCI_CLIENT_FUNC_LIST = *mut TCI_CLIENT_FUNC_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTC_GEN_FILTER = *mut TC_GEN_FILTER;
#[cfg(all(feature = "qos", feature = "shared"))]
pub type PTC_GEN_FLOW = *mut TC_GEN_FLOW;
#[cfg(all(feature = "ntddndis", feature = "shared"))]
pub type PTC_IFC_DESCRIPTOR = *mut TC_IFC_DESCRIPTOR;
#[cfg(all(feature = "ntddndis", feature = "shared", feature = "winnt"))]
pub type PTC_SUPPORTED_INFO_BUFFER = *mut TC_SUPPORTED_INFO_BUFFER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type TCI_ADD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::HANDLE, status: super::ULONG)>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct TCI_CLIENT_FUNC_LIST {
    pub ClNotifyHandler: TCI_NOTIFY_HANDLER,
    pub ClAddFlowCompleteHandler: TCI_ADD_FLOW_COMPLETE_HANDLER,
    pub ClModifyFlowCompleteHandler: TCI_MOD_FLOW_COMPLETE_HANDLER,
    pub ClDeleteFlowCompleteHandler: TCI_DEL_FLOW_COMPLETE_HANDLER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type TCI_DEL_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::HANDLE, status: super::ULONG)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type TCI_MOD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::HANDLE, status: super::ULONG)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type TCI_NOTIFY_HANDLER = Option<unsafe extern "system" fn(clregctx: super::HANDLE, clifcctx: super::HANDLE, event: super::ULONG, subcode: super::HANDLE, bufsize: super::ULONG, buffer: super::PVOID)>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TC_GEN_FILTER {
    pub AddressType: super::USHORT,
    pub PatternSize: super::ULONG,
    pub Pattern: super::PVOID,
    pub Mask: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "qos", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TC_GEN_FLOW {
    pub SendingFlowspec: super::FLOWSPEC,
    pub ReceivingFlowspec: super::FLOWSPEC,
    pub TcObjectsLength: super::ULONG,
    pub TcObjects: [super::QOS_OBJECT_HDR; 1],
}
#[cfg(all(feature = "qos", feature = "shared"))]
impl Default for TC_GEN_FLOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ntddndis", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TC_IFC_DESCRIPTOR {
    pub Length: super::ULONG,
    pub pInterfaceName: windows_core::PWSTR,
    pub pInterfaceID: windows_core::PWSTR,
    pub AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "winnt")]
pub const TC_INVALID_HANDLE: super::HANDLE = 0 as _;
pub const TC_NOTIFY_FLOW_CLOSE: i32 = 5;
pub const TC_NOTIFY_IFC_CHANGE: i32 = 3;
pub const TC_NOTIFY_IFC_CLOSE: i32 = 2;
pub const TC_NOTIFY_IFC_UP: i32 = 1;
pub const TC_NOTIFY_PARAM_CHANGED: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "ntddndis", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TC_SUPPORTED_INFO_BUFFER {
    pub InstanceIDLength: super::USHORT,
    pub InstanceID: [super::WCHAR; 256],
    pub InterfaceLuid: super::ULONG64,
    pub AddrListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(all(feature = "ntddndis", feature = "shared", feature = "winnt"))]
impl Default for TC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
