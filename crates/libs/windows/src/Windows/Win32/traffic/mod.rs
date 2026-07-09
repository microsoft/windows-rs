#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcAddFilter(flowhandle: super::winnt::HANDLE, pgenericfilter: *const TC_GEN_FILTER, pfilterhandle: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcAddFilter(flowhandle : super::winnt::HANDLE, pgenericfilter : *const TC_GEN_FILTER, pfilterhandle : *mut super::winnt::HANDLE) -> u32);
    unsafe { TcAddFilter(flowhandle, pgenericfilter, pfilterhandle as _) }
}
#[cfg(all(feature = "qos", feature = "winnt"))]
#[inline]
pub unsafe fn TcAddFlow(ifchandle: super::winnt::HANDLE, clflowctx: super::winnt::HANDLE, flags: u32, pgenericflow: *const TC_GEN_FLOW, pflowhandle: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcAddFlow(ifchandle : super::winnt::HANDLE, clflowctx : super::winnt::HANDLE, flags : u32, pgenericflow : *const TC_GEN_FLOW, pflowhandle : *mut super::winnt::HANDLE) -> u32);
    unsafe { TcAddFlow(ifchandle, clflowctx, flags, pgenericflow, pflowhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcCloseInterface(ifchandle: super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcCloseInterface(ifchandle : super::winnt::HANDLE) -> u32);
    unsafe { TcCloseInterface(ifchandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcDeleteFilter(filterhandle: super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFilter(filterhandle : super::winnt::HANDLE) -> u32);
    unsafe { TcDeleteFilter(filterhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcDeleteFlow(flowhandle: super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeleteFlow(flowhandle : super::winnt::HANDLE) -> u32);
    unsafe { TcDeleteFlow(flowhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcDeregisterClient(clienthandle: super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcDeregisterClient(clienthandle : super::winnt::HANDLE) -> u32);
    unsafe { TcDeregisterClient(clienthandle) }
}
#[cfg(all(feature = "qos", feature = "winnt"))]
#[inline]
pub unsafe fn TcEnumerateFlows(ifchandle: super::winnt::HANDLE, penumhandle: *mut super::winnt::HANDLE, pflowcount: *mut u32, pbufsize: *mut u32, buffer: *mut ENUMERATION_BUFFER) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateFlows(ifchandle : super::winnt::HANDLE, penumhandle : *mut super::winnt::HANDLE, pflowcount : *mut u32, pbufsize : *mut u32, buffer : *mut ENUMERATION_BUFFER) -> u32);
    unsafe { TcEnumerateFlows(ifchandle, penumhandle as _, pflowcount as _, pbufsize as _, buffer as _) }
}
#[cfg(all(feature = "ntddndis", feature = "winnt"))]
#[inline]
pub unsafe fn TcEnumerateInterfaces(clienthandle: super::winnt::HANDLE, pbuffersize: *mut u32, interfacebuffer: *mut TC_IFC_DESCRIPTOR) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcEnumerateInterfaces(clienthandle : super::winnt::HANDLE, pbuffersize : *mut u32, interfacebuffer : *mut TC_IFC_DESCRIPTOR) -> u32);
    unsafe { TcEnumerateInterfaces(clienthandle, pbuffersize as _, interfacebuffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcGetFlowNameA(flowhandle: super::winnt::HANDLE, pflowname: &mut [u8]) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameA(flowhandle : super::winnt::HANDLE, strsize : u32, pflowname : windows_core::PSTR) -> u32);
    unsafe { TcGetFlowNameA(flowhandle, pflowname.len().try_into().unwrap(), core::mem::transmute(pflowname.as_ptr())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcGetFlowNameW(flowhandle: super::winnt::HANDLE, pflowname: &mut [u16]) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcGetFlowNameW(flowhandle : super::winnt::HANDLE, strsize : u32, pflowname : windows_core::PWSTR) -> u32);
    unsafe { TcGetFlowNameW(flowhandle, pflowname.len().try_into().unwrap(), core::mem::transmute(pflowname.as_ptr())) }
}
#[cfg(all(feature = "qos", feature = "winnt"))]
#[inline]
pub unsafe fn TcModifyFlow(flowhandle: super::winnt::HANDLE, pgenericflow: *const TC_GEN_FLOW) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcModifyFlow(flowhandle : super::winnt::HANDLE, pgenericflow : *const TC_GEN_FLOW) -> u32);
    unsafe { TcModifyFlow(flowhandle, pgenericflow) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcOpenInterfaceA<P0>(pinterfacename: P0, clienthandle: super::winnt::HANDLE, clifcctx: super::winnt::HANDLE, pifchandle: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceA(pinterfacename : windows_core::PCSTR, clienthandle : super::winnt::HANDLE, clifcctx : super::winnt::HANDLE, pifchandle : *mut super::winnt::HANDLE) -> u32);
    unsafe { TcOpenInterfaceA(pinterfacename.param().abi(), clienthandle, clifcctx, pifchandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcOpenInterfaceW<P0>(pinterfacename: P0, clienthandle: super::winnt::HANDLE, clifcctx: super::winnt::HANDLE, pifchandle: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("traffic.dll" "system" fn TcOpenInterfaceW(pinterfacename : windows_core::PCWSTR, clienthandle : super::winnt::HANDLE, clifcctx : super::winnt::HANDLE, pifchandle : *mut super::winnt::HANDLE) -> u32);
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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcQueryInterface(ifchandle: super::winnt::HANDLE, pguidparam: *const windows_core::GUID, notifychange: bool, pbuffersize: *mut u32, buffer: *mut core::ffi::c_void) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcQueryInterface(ifchandle : super::winnt::HANDLE, pguidparam : *const windows_core::GUID, notifychange : bool, pbuffersize : *mut u32, buffer : *mut core::ffi::c_void) -> u32);
    unsafe { TcQueryInterface(ifchandle, pguidparam, notifychange, pbuffersize as _, buffer as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcRegisterClient(tciversion: u32, clregctx: super::winnt::HANDLE, clienthandlerlist: *const TCI_CLIENT_FUNC_LIST, pclienthandle: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcRegisterClient(tciversion : u32, clregctx : super::winnt::HANDLE, clienthandlerlist : *const TCI_CLIENT_FUNC_LIST, pclienthandle : *mut super::winnt::HANDLE) -> u32);
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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TcSetInterface(ifchandle: super::winnt::HANDLE, pguidparam: *const windows_core::GUID, buffersize: u32, buffer: *const core::ffi::c_void) -> u32 {
    windows_core::link!("traffic.dll" "system" fn TcSetInterface(ifchandle : super::winnt::HANDLE, pguidparam : *const windows_core::GUID, buffersize : u32, buffer : *const core::ffi::c_void) -> u32);
    unsafe { TcSetInterface(ifchandle, pguidparam, buffersize, buffer) }
}
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADDRESS_LIST_DESCRIPTOR {
    pub MediaType: u32,
    pub AddressList: super::ntddndis::NETWORK_ADDRESS_LIST,
}
pub const CURRENT_TCI_VERSION: u32 = 2;
#[repr(C)]
#[cfg(feature = "qos")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IPX_PATTERN {
    pub Src: IPX_PATTERN_0,
    pub Dest: IPX_PATTERN_1,
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_PATTERN_0_0 {
    pub s_srcport: u16,
    pub s_dstport: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IP_PATTERN_0_1 {
    pub s_type: u8,
    pub s_code: u8,
    pub filler: u16,
}
pub const MAX_STRING_LENGTH: u32 = 256;
#[cfg(feature = "ntddndis")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRESS_LIST_DESCRIPTOR(pub *mut ADDRESS_LIST_DESCRIPTOR);
#[cfg(feature = "ntddndis")]
impl PADDRESS_LIST_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "ntddndis")]
impl Default for PADDRESS_LIST_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "qos")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENUMERATION_BUFFER(pub *mut ENUMERATION_BUFFER);
#[cfg(feature = "qos")]
impl PENUMERATION_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "qos")]
impl Default for PENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIPX_PATTERN(pub *mut IPX_PATTERN);
impl PIPX_PATTERN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIPX_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIP_PATTERN(pub *mut IP_PATTERN);
impl PIP_PATTERN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIP_PATTERN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTCI_CLIENT_FUNC_LIST(pub *mut TCI_CLIENT_FUNC_LIST);
#[cfg(feature = "winnt")]
impl PTCI_CLIENT_FUNC_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PTCI_CLIENT_FUNC_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTC_GEN_FILTER(pub *mut TC_GEN_FILTER);
impl PTC_GEN_FILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTC_GEN_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "qos")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTC_GEN_FLOW(pub *mut TC_GEN_FLOW);
#[cfg(feature = "qos")]
impl PTC_GEN_FLOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "qos")]
impl Default for PTC_GEN_FLOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ntddndis")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTC_IFC_DESCRIPTOR(pub *mut TC_IFC_DESCRIPTOR);
#[cfg(feature = "ntddndis")]
impl PTC_IFC_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "ntddndis")]
impl Default for PTC_IFC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ntddndis")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTC_SUPPORTED_INFO_BUFFER(pub *mut TC_SUPPORTED_INFO_BUFFER);
#[cfg(feature = "ntddndis")]
impl PTC_SUPPORTED_INFO_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "ntddndis")]
impl Default for PTC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type TCI_ADD_FLOW_COMPLETE_HANDLER = Option<unsafe extern "system" fn(clflowctx: super::winnt::HANDLE, status: u32)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
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
#[cfg(feature = "qos")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TC_IFC_DESCRIPTOR {
    pub Length: u32,
    pub pInterfaceName: windows_core::PWSTR,
    pub pInterfaceID: windows_core::PWSTR,
    pub AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "winnt")]
pub const TC_INVALID_HANDLE: super::winnt::HANDLE = super::winnt::HANDLE(0 as _);
pub const TC_NOTIFY_FLOW_CLOSE: u32 = 5;
pub const TC_NOTIFY_IFC_CHANGE: u32 = 3;
pub const TC_NOTIFY_IFC_CLOSE: u32 = 2;
pub const TC_NOTIFY_IFC_UP: u32 = 1;
pub const TC_NOTIFY_PARAM_CHANGED: u32 = 4;
#[repr(C)]
#[cfg(feature = "ntddndis")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
