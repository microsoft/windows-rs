#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcAddFilter(flowhandle : super::winnt::HANDLE, pgenericfilter : *const TC_GEN_FILTER, pfilterhandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(all(feature = "qos", feature = "winnt"))]
windows_link::link!("traffic.dll" "system" fn TcAddFlow(ifchandle : super::winnt::HANDLE, clflowctx : super::winnt::HANDLE, flags : u32, pgenericflow : *const TC_GEN_FLOW, pflowhandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcCloseInterface(ifchandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcDeleteFilter(filterhandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcDeleteFlow(flowhandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcDeregisterClient(clienthandle : super::winnt::HANDLE) -> u32);
#[cfg(all(feature = "qos", feature = "winnt"))]
windows_link::link!("traffic.dll" "system" fn TcEnumerateFlows(ifchandle : super::winnt::HANDLE, penumhandle : *mut super::winnt::HANDLE, pflowcount : *mut u32, pbufsize : *mut u32, buffer : *mut ENUMERATION_BUFFER) -> u32);
#[cfg(all(feature = "ntddndis", feature = "winnt"))]
windows_link::link!("traffic.dll" "system" fn TcEnumerateInterfaces(clienthandle : super::winnt::HANDLE, pbuffersize : *mut u32, interfacebuffer : *mut TC_IFC_DESCRIPTOR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcGetFlowNameA(flowhandle : super::winnt::HANDLE, strsize : u32, pflowname : windows_sys::core::PSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcGetFlowNameW(flowhandle : super::winnt::HANDLE, strsize : u32, pflowname : windows_sys::core::PWSTR) -> u32);
#[cfg(all(feature = "qos", feature = "winnt"))]
windows_link::link!("traffic.dll" "system" fn TcModifyFlow(flowhandle : super::winnt::HANDLE, pgenericflow : *const TC_GEN_FLOW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcOpenInterfaceA(pinterfacename : windows_sys::core::PCSTR, clienthandle : super::winnt::HANDLE, clifcctx : super::winnt::HANDLE, pifchandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcOpenInterfaceW(pinterfacename : windows_sys::core::PCWSTR, clienthandle : super::winnt::HANDLE, clifcctx : super::winnt::HANDLE, pifchandle : *mut super::winnt::HANDLE) -> u32);
windows_link::link!("traffic.dll" "system" fn TcQueryFlowA(pflowname : windows_sys::core::PCSTR, pguidparam : *const windows_sys::core::GUID, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
windows_link::link!("traffic.dll" "system" fn TcQueryFlowW(pflowname : windows_sys::core::PCWSTR, pguidparam : *const windows_sys::core::GUID, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcQueryInterface(ifchandle : super::winnt::HANDLE, pguidparam : *const windows_sys::core::GUID, notifychange : bool, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcRegisterClient(tciversion : u32, clregctx : super::winnt::HANDLE, clienthandlerlist : *const TCI_CLIENT_FUNC_LIST, pclienthandle : *mut super::winnt::HANDLE) -> u32);
windows_link::link!("traffic.dll" "system" fn TcSetFlowA(pflowname : windows_sys::core::PCSTR, pguidparam : *const windows_sys::core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
windows_link::link!("traffic.dll" "system" fn TcSetFlowW(pflowname : windows_sys::core::PCWSTR, pguidparam : *const windows_sys::core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("traffic.dll" "system" fn TcSetInterface(ifchandle : super::winnt::HANDLE, pguidparam : *const windows_sys::core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Default)]
pub struct ADDRESS_LIST_DESCRIPTOR {
    pub MediaType: u32,
    pub AddressList: super::ntddndis::NETWORK_ADDRESS_LIST,
}
pub const CURRENT_TCI_VERSION: u32 = 2;
#[repr(C)]
#[cfg(feature = "qos")]
#[derive(Clone, Copy)]
pub struct ENUMERATION_BUFFER {
    pub Length: u32,
    pub OwnerProcessId: u32,
    pub FlowNameLength: u16,
    pub FlowName: [u16; 256],
    pub pFlow: PTC_GEN_FLOW,
    pub NumberOfFilters: u32,
    pub GenericFilter: [TC_GEN_FILTER; 1],
}
#[cfg(feature = "qos")]
impl Default for ENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IPX_PATTERN {
    pub Src: IPX_PATTERN_0,
    pub Dest: IPX_PATTERN_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IPX_PATTERN_1 {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl Default for IPX_PATTERN_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[derive(Clone, Copy, Default)]
pub struct IP_PATTERN_0_0 {
    pub s_srcport: u16,
    pub s_dstport: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IP_PATTERN_0_1 {
    pub s_type: u8,
    pub s_code: u8,
    pub filler: u16,
}
pub const MAX_STRING_LENGTH: u32 = 256;
#[cfg(feature = "ntddndis")]
pub type PADDRESS_LIST_DESCRIPTOR = *mut ADDRESS_LIST_DESCRIPTOR;
#[cfg(feature = "qos")]
pub type PENUMERATION_BUFFER = *mut ENUMERATION_BUFFER;
pub type PIPX_PATTERN = *mut IPX_PATTERN;
pub type PIP_PATTERN = *mut IP_PATTERN;
#[cfg(feature = "winnt")]
pub type PTCI_CLIENT_FUNC_LIST = *mut TCI_CLIENT_FUNC_LIST;
pub type PTC_GEN_FILTER = *mut TC_GEN_FILTER;
#[cfg(feature = "qos")]
pub type PTC_GEN_FLOW = *mut TC_GEN_FLOW;
#[cfg(feature = "ntddndis")]
pub type PTC_IFC_DESCRIPTOR = *mut TC_IFC_DESCRIPTOR;
#[cfg(feature = "ntddndis")]
pub type PTC_SUPPORTED_INFO_BUFFER = *mut TC_SUPPORTED_INFO_BUFFER;
#[cfg(feature = "winnt")]
pub type TCI_ADD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::winnt::HANDLE, status: u32)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct TCI_CLIENT_FUNC_LIST {
    pub ClNotifyHandler: TCI_NOTIFY_HANDLER,
    pub ClAddFlowCompleteHandler: TCI_ADD_FLOW_COMPLETE_HANDLER,
    pub ClModifyFlowCompleteHandler: TCI_MOD_FLOW_COMPLETE_HANDLER,
    pub ClDeleteFlowCompleteHandler: TCI_DEL_FLOW_COMPLETE_HANDLER,
}
#[cfg(feature = "winnt")]
pub type TCI_DEL_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::winnt::HANDLE, status: u32)>;
#[cfg(feature = "winnt")]
pub type TCI_MOD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::winnt::HANDLE, status: u32)>;
#[cfg(feature = "winnt")]
pub type TCI_NOTIFY_HANDLER = Option<unsafe extern "system" fn(clregctx: super::winnt::HANDLE, clifcctx: super::winnt::HANDLE, event: u32, subcode: super::winnt::HANDLE, bufsize: u32, buffer: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[cfg(feature = "qos")]
#[derive(Clone, Copy)]
pub struct TC_GEN_FLOW {
    pub SendingFlowspec: super::qos::FLOWSPEC,
    pub ReceivingFlowspec: super::qos::FLOWSPEC,
    pub TcObjectsLength: u32,
    pub TcObjects: [super::qos::QOS_OBJECT_HDR; 1],
}
#[cfg(feature = "qos")]
impl Default for TC_GEN_FLOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy)]
pub struct TC_IFC_DESCRIPTOR {
    pub Length: u32,
    pub pInterfaceName: windows_sys::core::PWSTR,
    pub pInterfaceID: windows_sys::core::PWSTR,
    pub AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "ntddndis")]
impl Default for TC_IFC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub const TC_INVALID_HANDLE: super::winnt::HANDLE = 0 as _;
pub const TC_NOTIFY_FLOW_CLOSE: u32 = 5;
pub const TC_NOTIFY_IFC_CHANGE: u32 = 3;
pub const TC_NOTIFY_IFC_CLOSE: u32 = 2;
pub const TC_NOTIFY_IFC_UP: u32 = 1;
pub const TC_NOTIFY_PARAM_CHANGED: u32 = 4;
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy)]
pub struct TC_SUPPORTED_INFO_BUFFER {
    pub InstanceIDLength: u16,
    pub InstanceID: [u16; 256],
    pub InterfaceLuid: u64,
    pub AddrListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "ntddndis")]
impl Default for TC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
