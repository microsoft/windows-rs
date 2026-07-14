#[inline]
pub unsafe fn RtlEthernetAddressToStringA(addr: *const DL_EUI48, s: &mut [u8; 18]) -> windows_core::PSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlEthernetAddressToStringA(addr : *const DL_EUI48, s : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { RtlEthernetAddressToStringA(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[inline]
pub unsafe fn RtlEthernetAddressToStringW(addr: *const DL_EUI48, s: &mut [u16; 18]) -> windows_core::PWSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlEthernetAddressToStringW(addr : *const DL_EUI48, s : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { RtlEthernetAddressToStringW(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[inline]
pub unsafe fn RtlEthernetStringToAddressA<P0>(s: P0, terminator: *mut windows_core::PCSTR, addr: *mut DL_EUI48) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlEthernetStringToAddressA(s : windows_core::PCSTR, terminator : *mut windows_core::PCSTR, addr : *mut DL_EUI48) -> i32);
    unsafe { RtlEthernetStringToAddressA(s.param().abi(), terminator as _, addr as _) }
}
#[inline]
pub unsafe fn RtlEthernetStringToAddressW<P0>(s: P0, terminator: *mut windows_core::PCWSTR, addr: *mut DL_EUI48) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlEthernetStringToAddressW(s : windows_core::PCWSTR, terminator : *mut windows_core::PCWSTR, addr : *mut DL_EUI48) -> i32);
    unsafe { RtlEthernetStringToAddressW(s.param().abi(), terminator as _, addr as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4AddressToStringA(addr: *const super::inaddr::IN_ADDR, s: &mut [u8; 16]) -> windows_core::PSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringA(addr : *const super::inaddr::IN_ADDR, s : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { RtlIpv4AddressToStringA(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4AddressToStringExA(address: *const super::inaddr::IN_ADDR, port: u16, addressstring: windows_core::PSTR, addressstringlength: *mut u32) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringExA(address : *const super::inaddr::IN_ADDR, port : u16, addressstring : windows_core::PSTR, addressstringlength : *mut u32) -> i32);
    unsafe { RtlIpv4AddressToStringExA(address, port, addressstring, addressstringlength as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4AddressToStringExW(address: *const super::inaddr::IN_ADDR, port: u16, addressstring: windows_core::PWSTR, addressstringlength: *mut u32) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringExW(address : *const super::inaddr::IN_ADDR, port : u16, addressstring : windows_core::PWSTR, addressstringlength : *mut u32) -> i32);
    unsafe { RtlIpv4AddressToStringExW(address, port, addressstring, addressstringlength as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4AddressToStringW(addr: *const super::inaddr::IN_ADDR, s: &mut [u16; 16]) -> windows_core::PWSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringW(addr : *const super::inaddr::IN_ADDR, s : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { RtlIpv4AddressToStringW(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4StringToAddressA<P0>(s: P0, strict: bool, terminator: *mut windows_core::PCSTR, addr: *mut super::inaddr::IN_ADDR) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressA(s : windows_core::PCSTR, strict : bool, terminator : *mut windows_core::PCSTR, addr : *mut super::inaddr::IN_ADDR) -> i32);
    unsafe { RtlIpv4StringToAddressA(s.param().abi(), strict, terminator as _, addr as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4StringToAddressExA<P0>(addressstring: P0, strict: bool, address: *mut super::inaddr::IN_ADDR, port: *mut u16) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressExA(addressstring : windows_core::PCSTR, strict : bool, address : *mut super::inaddr::IN_ADDR, port : *mut u16) -> i32);
    unsafe { RtlIpv4StringToAddressExA(addressstring.param().abi(), strict, address as _, port as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4StringToAddressExW<P0>(addressstring: P0, strict: bool, address: *mut super::inaddr::IN_ADDR, port: *mut u16) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressExW(addressstring : windows_core::PCWSTR, strict : bool, address : *mut super::inaddr::IN_ADDR, port : *mut u16) -> i32);
    unsafe { RtlIpv4StringToAddressExW(addressstring.param().abi(), strict, address as _, port as _) }
}
#[cfg(feature = "inaddr")]
#[inline]
pub unsafe fn RtlIpv4StringToAddressW<P0>(s: P0, strict: bool, terminator: *mut windows_core::PCWSTR, addr: *mut super::inaddr::IN_ADDR) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressW(s : windows_core::PCWSTR, strict : bool, terminator : *mut windows_core::PCWSTR, addr : *mut super::inaddr::IN_ADDR) -> i32);
    unsafe { RtlIpv4StringToAddressW(s.param().abi(), strict, terminator as _, addr as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6AddressToStringA(addr: *const super::in6addr::IN6_ADDR, s: &mut [u8; 46]) -> windows_core::PSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringA(addr : *const super::in6addr::IN6_ADDR, s : windows_core::PSTR) -> windows_core::PSTR);
    unsafe { RtlIpv6AddressToStringA(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6AddressToStringExA(address: *const super::in6addr::IN6_ADDR, scopeid: u32, port: u16, addressstring: windows_core::PSTR, addressstringlength: *mut u32) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringExA(address : *const super::in6addr::IN6_ADDR, scopeid : u32, port : u16, addressstring : windows_core::PSTR, addressstringlength : *mut u32) -> i32);
    unsafe { RtlIpv6AddressToStringExA(address, scopeid, port, addressstring, addressstringlength as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6AddressToStringExW(address: *const super::in6addr::IN6_ADDR, scopeid: u32, port: u16, addressstring: windows_core::PWSTR, addressstringlength: *mut u32) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringExW(address : *const super::in6addr::IN6_ADDR, scopeid : u32, port : u16, addressstring : windows_core::PWSTR, addressstringlength : *mut u32) -> i32);
    unsafe { RtlIpv6AddressToStringExW(address, scopeid, port, addressstring, addressstringlength as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6AddressToStringW(addr: *const super::in6addr::IN6_ADDR, s: &mut [u16; 46]) -> windows_core::PWSTR {
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringW(addr : *const super::in6addr::IN6_ADDR, s : windows_core::PWSTR) -> windows_core::PWSTR);
    unsafe { RtlIpv6AddressToStringW(addr, core::mem::transmute(s.as_mut_ptr())) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6StringToAddressA<P0>(s: P0, terminator: *mut windows_core::PCSTR, addr: *mut super::in6addr::IN6_ADDR) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressA(s : windows_core::PCSTR, terminator : *mut windows_core::PCSTR, addr : *mut super::in6addr::IN6_ADDR) -> i32);
    unsafe { RtlIpv6StringToAddressA(s.param().abi(), terminator as _, addr as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6StringToAddressExA<P0>(addressstring: P0, address: *mut super::in6addr::IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressExA(addressstring : windows_core::PCSTR, address : *mut super::in6addr::IN6_ADDR, scopeid : *mut u32, port : *mut u16) -> i32);
    unsafe { RtlIpv6StringToAddressExA(addressstring.param().abi(), address as _, scopeid as _, port as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6StringToAddressExW<P0>(addressstring: P0, address: *mut super::in6addr::IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressExW(addressstring : windows_core::PCWSTR, address : *mut super::in6addr::IN6_ADDR, scopeid : *mut u32, port : *mut u16) -> i32);
    unsafe { RtlIpv6StringToAddressExW(addressstring.param().abi(), address as _, scopeid as _, port as _) }
}
#[cfg(feature = "in6addr")]
#[inline]
pub unsafe fn RtlIpv6StringToAddressW<P0>(s: P0, terminator: *mut windows_core::PCWSTR, addr: *mut super::in6addr::IN6_ADDR) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressW(s : windows_core::PCWSTR, terminator : *mut windows_core::PCWSTR, addr : *mut super::in6addr::IN6_ADDR) -> i32);
    unsafe { RtlIpv6StringToAddressW(s.param().abi(), terminator as _, addr as _) }
}
pub const ASSOCIATE_NAMERES_CONTEXT: windows_core::GUID = windows_core::GUID::from_u128(0x59a38b67_d4fe_46e1_ba3c_87ea74ca3049);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub Handle: u64,
}
pub type CONTROL_CHANNEL_TRIGGER_STATUS = i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 2;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS = 0;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_MAX: u32 = 4;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 3;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS = 6;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 1;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 4;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DL_EUI48(pub u8);
pub const IN4ADDR_ANY: u32 = 0;
pub const IN4ADDR_BROADCAST: i32 = -1;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: u32 = 16;
pub const IN4ADDR_LOOPBACK: u32 = 16777343;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: u32 = 8;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: u32 = 4;
pub const IN4ADDR_NONE: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_PORT_RANGE {
    pub StartPort: u16,
    pub NumberOfPorts: u16,
}
pub type INET_PORT_RESERVATION = INET_PORT_RANGE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_PORT_RESERVATION_INFORMATION {
    pub OwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_PORT_RESERVATION_INSTANCE {
    pub Reservation: INET_PORT_RESERVATION,
    pub Token: INET_PORT_RESERVATION_TOKEN,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INET_PORT_RESERVATION_TOKEN {
    pub Token: u64,
}
pub const INVALID_PORT_RESERVATION_TOKEN: u64 = 0;
pub type PASSOCIATE_NAMERES_CONTEXT_INPUT = *mut ASSOCIATE_NAMERES_CONTEXT_INPUT;
pub type PCONTROL_CHANNEL_TRIGGER_STATUS = *mut CONTROL_CHANNEL_TRIGGER_STATUS;
pub type PDL_EUI48 = *mut DL_EUI48;
pub type PINET_PORT_RANGE = *mut INET_PORT_RANGE;
pub type PINET_PORT_RESERVATION = *mut INET_PORT_RANGE;
pub type PINET_PORT_RESERVATION_INFORMATION = *mut INET_PORT_RESERVATION_INFORMATION;
pub type PINET_PORT_RESERVATION_INSTANCE = *mut INET_PORT_RESERVATION_INSTANCE;
pub type PINET_PORT_RESERVATION_TOKEN = *mut INET_PORT_RESERVATION_TOKEN;
pub type PPRIORITY_STATUS = *mut PRIORITY_STATUS;
pub type PRCVALL_IF = *mut RCVALL_IF;
pub type PRCVALL_VALUE = *mut RCVALL_VALUE;
pub type PREAL_TIME_NOTIFICATION_SETTING_INPUT = *mut REAL_TIME_NOTIFICATION_SETTING_INPUT;
pub type PREAL_TIME_NOTIFICATION_SETTING_INPUT_EX = *mut REAL_TIME_NOTIFICATION_SETTING_INPUT_EX;
pub type PREAL_TIME_NOTIFICATION_SETTING_OUTPUT = *mut REAL_TIME_NOTIFICATION_SETTING_OUTPUT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRIORITY_STATUS {
    pub Sender: SOCKET_PRIORITY_HINT,
    pub Receiver: SOCKET_PRIORITY_HINT,
}
pub type PRSS_SCALABILITY_INFO = *mut RSS_SCALABILITY_INFO;
pub type PSOCKET_PRIORITY_HINT = *mut SOCKET_PRIORITY_HINT;
pub type PTCP_ACK_FREQUENCY_PARAMETERS = *mut TCP_ACK_FREQUENCY_PARAMETERS;
pub type PTCP_ICW_LEVEL = *mut TCP_ICW_LEVEL;
pub type PTCP_ICW_PARAMETERS = *mut TCP_ICW_PARAMETERS;
pub type PTCP_INFO_v0 = *mut TCP_INFO_v0;
pub type PTCP_INFO_v1 = *mut TCP_INFO_v1;
pub type PTCP_INFO_v2 = *mut TCP_INFO_v2;
pub type PTCP_INITIAL_RTO_PARAMETERS = *mut TCP_INITIAL_RTO_PARAMETERS;
pub type PTIMESTAMPING_CONFIG = *mut TIMESTAMPING_CONFIG;
pub type PTRANSPORT_SETTING_ID = *mut TRANSPORT_SETTING_ID;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RCVALL_IF {
    pub Mode: RCVALL_VALUE,
    pub Interface: u32,
}
pub const RCVALL_IPLEVEL: RCVALL_VALUE = 3;
pub const RCVALL_MAX: u32 = 3;
pub const RCVALL_OFF: RCVALL_VALUE = 0;
pub const RCVALL_ON: RCVALL_VALUE = 1;
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = 2;
pub type RCVALL_VALUE = i32;
pub const REAL_TIME_NOTIFICATION_CAPABILITY: windows_core::GUID = windows_core::GUID::from_u128(0x6b59819a_5cae_492d_a901_2a3c2c50164f);
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: windows_core::GUID = windows_core::GUID::from_u128(0x6843da03_154a_4616_a508_44371295f96b);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: windows_core::GUID,
    pub Unmark: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    pub ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RSS_SCALABILITY_INFO {
    pub RssEnabled: bool,
}
pub const SIO_ABSORB_RTRALERT: i32 = -1744830459;
pub const SIO_ACQUIRE_PORT_RESERVATION: i32 = -1744830364;
pub const SIO_APPLY_TRANSPORT_SETTING: i32 = -1744830445;
pub const SIO_ASSOCIATE_PORT_RESERVATION: i32 = -1744830362;
pub const SIO_CPU_AFFINITY: i32 = -1744830443;
pub const SIO_DELETE_PEER_TARGET_NAME: i32 = -1744830261;
pub const SIO_GET_TX_TIMESTAMP: i32 = -1744830230;
pub const SIO_INDEX_ADD_MCAST: i32 = -1744830454;
pub const SIO_INDEX_BIND: i32 = -1744830456;
pub const SIO_INDEX_DEL_MCAST: i32 = -1744830453;
pub const SIO_INDEX_MCASTIF: i32 = -1744830455;
pub const SIO_KEEPALIVE_VALS: i32 = -1744830460;
pub const SIO_LIMIT_BROADCASTS: i32 = -1744830457;
pub const SIO_LOOPBACK_FAST_PATH: i32 = -1744830448;
pub const SIO_PRIORITY_HINT: i32 = -1744830440;
pub const SIO_QUERY_RSS_SCALABILITY_INFO: u32 = 1476395218;
pub const SIO_QUERY_SECURITY: i32 = -671088439;
pub const SIO_QUERY_TRANSPORT_SETTING: i32 = -1744830444;
pub const SIO_QUERY_WFP_ALE_ENDPOINT_HANDLE: u32 = 1476395213;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_CONTEXT: i32 = -1744830243;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_RECORDS: i32 = -1744830244;
pub const SIO_RCVALL: i32 = -1744830463;
pub const SIO_RCVALL_IF: i32 = -1744830450;
pub const SIO_RCVALL_IGMPMCAST: i32 = -1744830461;
pub const SIO_RCVALL_MCAST: i32 = -1744830462;
pub const SIO_RCVALL_MCAST_IF: i32 = -1744830451;
pub const SIO_RELEASE_PORT_RESERVATION: i32 = -1744830363;
pub const SIO_SET_PEER_TARGET_NAME: i32 = -1744830262;
pub const SIO_SET_PRIORITY_HINT: i32 = -1744830440;
pub const SIO_SET_SECURITY: i32 = -1744830264;
pub const SIO_SET_WFP_CONNECTION_REDIRECT_RECORDS: i32 = -1744830242;
pub const SIO_SOCKET_USAGE_NOTIFICATION: i32 = -1744830260;
pub const SIO_TCP_INFO: i32 = -671088601;
pub const SIO_TCP_INITIAL_RTO: i32 = -1744830447;
pub const SIO_TCP_SET_ACK_FREQUENCY: i32 = -1744830441;
pub const SIO_TCP_SET_ICW: i32 = -1744830442;
pub const SIO_TIMESTAMPING: i32 = -1744830229;
pub const SIO_UCAST_IF: i32 = -1744830458;
pub const SOCKET_DEFAULT2_QM_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xaec2ef9c_3a4d_4d3e_8842_239942e39a47);
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: u32 = 2;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: u32 = 4;
pub const SOCKET_INFO_CONNECTION_SECURED: u32 = 1;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKET_PEER_TARGET_NAME {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::ws2::SOCKADDR_STORAGE,
    pub PeerTargetNameStringLen: u32,
    pub AllStrings: [u16; 0],
}
#[cfg(feature = "ws2")]
impl Default for SOCKET_PEER_TARGET_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SOCKET_PRIORITY_HINT = i32;
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: u32 = 1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: u32 = 1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: u32 = 2;
pub type SOCKET_SECURITY_PROTOCOL = i32;
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL = 0;
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL = 3;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = 1;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_SECURITY_QUERY_INFO {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
    pub MmSaId: u64,
    pub QmSaId: u64,
    pub NegotiationWinerr: u32,
    pub SaLookupContext: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::ws2::SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::ws2::SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
    pub Flags: u32,
    pub FieldMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_SECURITY_SETTINGS {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKET_SECURITY_SETTINGS_IPSEC {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
    pub IpsecFlags: u32,
    pub AuthipMMPolicyKey: windows_core::GUID,
    pub AuthipQMPolicyKey: windows_core::GUID,
    pub Reserved: windows_core::GUID,
    pub Reserved2: u64,
    pub UserNameStringLen: u32,
    pub DomainNameStringLen: u32,
    pub PasswordStringLen: u32,
    pub AllStrings: [u16; 0],
}
impl Default for SOCKET_SECURITY_SETTINGS_IPSEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SOCKET_SETTINGS_ALLOW_INSECURE: u32 = 2;
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: u32 = 1;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 4;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: u32 = 2;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: u32 = 8;
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: u32 = 1;
pub type SOCKET_USAGE_TYPE = i32;
pub const SO_TIMESTAMP: u32 = 12298;
pub const SO_TIMESTAMP_ID: u32 = 12299;
pub const SYSTEM_CRITICAL_SOCKET: SOCKET_USAGE_TYPE = 1;
pub const SocketMaximumPriorityHintType: SOCKET_PRIORITY_HINT = 3;
pub const SocketPriorityHintLow: SOCKET_PRIORITY_HINT = 1;
pub const SocketPriorityHintNormal: SOCKET_PRIORITY_HINT = 2;
pub const SocketPriorityHintVeryLow: SOCKET_PRIORITY_HINT = 0;
pub type TCPSTATE = i32;
pub const TCPSTATE_CLOSED: TCPSTATE = 0;
pub const TCPSTATE_CLOSE_WAIT: TCPSTATE = 7;
pub const TCPSTATE_CLOSING: TCPSTATE = 8;
pub const TCPSTATE_ESTABLISHED: TCPSTATE = 4;
pub const TCPSTATE_FIN_WAIT_1: TCPSTATE = 5;
pub const TCPSTATE_FIN_WAIT_2: TCPSTATE = 6;
pub const TCPSTATE_LAST_ACK: TCPSTATE = 9;
pub const TCPSTATE_LISTEN: TCPSTATE = 1;
pub const TCPSTATE_MAX: TCPSTATE = 11;
pub const TCPSTATE_SYN_RCVD: TCPSTATE = 3;
pub const TCPSTATE_SYN_SENT: TCPSTATE = 2;
pub const TCPSTATE_TIME_WAIT: TCPSTATE = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ACK_FREQUENCY_PARAMETERS {
    pub TcpDelayedAckFrequency: u8,
}
pub type TCP_ICW_LEVEL = i32;
pub const TCP_ICW_LEVEL_AGGRESSIVE: TCP_ICW_LEVEL = 3;
pub const TCP_ICW_LEVEL_COMPAT: TCP_ICW_LEVEL = 254;
pub const TCP_ICW_LEVEL_DEFAULT: TCP_ICW_LEVEL = 0;
pub const TCP_ICW_LEVEL_EXPERIMENTAL: TCP_ICW_LEVEL = 4;
pub const TCP_ICW_LEVEL_HIGH: TCP_ICW_LEVEL = 1;
pub const TCP_ICW_LEVEL_MAX: TCP_ICW_LEVEL = 255;
pub const TCP_ICW_LEVEL_VERY_HIGH: TCP_ICW_LEVEL = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_ICW_PARAMETERS {
    pub Level: TCP_ICW_LEVEL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_INFO_v0 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: bool,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_INFO_v1 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: bool,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: u64,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: u64,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_INFO_v2 {
    pub State: TCPSTATE,
    pub Mss: u32,
    pub ConnectionTimeMs: u64,
    pub TimestampsEnabled: bool,
    pub RttUs: u32,
    pub MinRttUs: u32,
    pub BytesInFlight: u32,
    pub Cwnd: u32,
    pub SndWnd: u32,
    pub RcvWnd: u32,
    pub RcvBuf: u32,
    pub BytesOut: u64,
    pub BytesIn: u64,
    pub BytesReordered: u32,
    pub BytesRetrans: u32,
    pub FastRetrans: u32,
    pub DupAcksIn: u32,
    pub TimeoutEpisodes: u32,
    pub SynRetrans: u8,
    pub SndLimTransRwin: u32,
    pub SndLimTimeRwin: u32,
    pub SndLimBytesRwin: u64,
    pub SndLimTransCwnd: u32,
    pub SndLimTimeCwnd: u32,
    pub SndLimBytesCwnd: u64,
    pub SndLimTransSnd: u32,
    pub SndLimTimeSnd: u32,
    pub SndLimBytesSnd: u64,
    pub OutOfOrderPktsIn: u32,
    pub EcnNegotiated: bool,
    pub EceAcksIn: u32,
    pub PtoEpisodes: u32,
}
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: u32 = 0;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: u32 = 0;
pub const TCP_INITIAL_RTO_NO_SYN_RETRANSMISSIONS: u8 = 254;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCP_INITIAL_RTO_PARAMETERS {
    pub Rtt: u16,
    pub MaxSynRetransmissions: u8,
}
pub const TCP_INITIAL_RTO_UNSPECIFIED_MAX_SYN_RETRANSMISSIONS: u8 = 255;
pub const TCP_INITIAL_RTO_UNSPECIFIED_RTT: u16 = 65535;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIMESTAMPING_CONFIG {
    pub Flags: u32,
    pub TxTimestampsBuffered: u16,
}
pub const TIMESTAMPING_FLAG_RX: u32 = 1;
pub const TIMESTAMPING_FLAG_TX: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSPORT_SETTING_ID {
    pub Guid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct tcp_keepalive {
    pub onoff: u32,
    pub keepalivetime: u32,
    pub keepaliveinterval: u32,
}
