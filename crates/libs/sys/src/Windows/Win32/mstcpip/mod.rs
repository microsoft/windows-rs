windows_link::link!("ntdll.dll" "system" fn RtlEthernetAddressToStringA(addr : *const DL_EUI48, s : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
windows_link::link!("ntdll.dll" "system" fn RtlEthernetAddressToStringW(addr : *const DL_EUI48, s : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("ntdll.dll" "system" fn RtlEthernetStringToAddressA(s : windows_sys::core::PCSTR, terminator : *mut windows_sys::core::PCSTR, addr : *mut DL_EUI48) -> i32);
windows_link::link!("ntdll.dll" "system" fn RtlEthernetStringToAddressW(s : windows_sys::core::PCWSTR, terminator : *mut windows_sys::core::PCWSTR, addr : *mut DL_EUI48) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringA(addr : *const super::IN_ADDR, s : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringExA(address : *const super::IN_ADDR, port : u16, addressstring : windows_sys::core::PSTR, addressstringlength : *mut u32) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringExW(address : *const super::IN_ADDR, port : u16, addressstring : windows_sys::core::PWSTR, addressstringlength : *mut u32) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4AddressToStringW(addr : *const super::IN_ADDR, s : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressA(s : windows_sys::core::PCSTR, strict : bool, terminator : *mut windows_sys::core::PCSTR, addr : *mut super::IN_ADDR) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressExA(addressstring : windows_sys::core::PCSTR, strict : bool, address : *mut super::IN_ADDR, port : *mut u16) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressExW(addressstring : windows_sys::core::PCWSTR, strict : bool, address : *mut super::IN_ADDR, port : *mut u16) -> i32);
#[cfg(feature = "inaddr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv4StringToAddressW(s : windows_sys::core::PCWSTR, strict : bool, terminator : *mut windows_sys::core::PCWSTR, addr : *mut super::IN_ADDR) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringA(addr : *const super::IN6_ADDR, s : windows_sys::core::PSTR) -> windows_sys::core::PSTR);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringExA(address : *const super::IN6_ADDR, scopeid : u32, port : u16, addressstring : windows_sys::core::PSTR, addressstringlength : *mut u32) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringExW(address : *const super::IN6_ADDR, scopeid : u32, port : u16, addressstring : windows_sys::core::PWSTR, addressstringlength : *mut u32) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6AddressToStringW(addr : *const super::IN6_ADDR, s : windows_sys::core::PWSTR) -> windows_sys::core::PWSTR);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressA(s : windows_sys::core::PCSTR, terminator : *mut windows_sys::core::PCSTR, addr : *mut super::IN6_ADDR) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressExA(addressstring : windows_sys::core::PCSTR, address : *mut super::IN6_ADDR, scopeid : *mut u32, port : *mut u16) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressExW(addressstring : windows_sys::core::PCWSTR, address : *mut super::IN6_ADDR, scopeid : *mut u32, port : *mut u16) -> i32);
#[cfg(feature = "in6addr")]
windows_link::link!("ntdll.dll" "system" fn RtlIpv6StringToAddressW(s : windows_sys::core::PCWSTR, terminator : *mut windows_sys::core::PCWSTR, addr : *mut super::IN6_ADDR) -> i32);
pub const ASSOCIATE_NAMERES_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59a38b67_d4fe_46e1_ba3c_87ea74ca3049);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub Handle: u64,
}
pub type CONTROL_CHANNEL_TRIGGER_STATUS = i32;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 2;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS = 0;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_MAX: i32 = 4;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 3;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS = 6;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = 1;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = 4;
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DL_EUI48(pub u8);
pub const IN4ADDR_ANY: u32 = 0;
pub const IN4ADDR_BROADCAST: u32 = 4294967295;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: i32 = 16;
pub const IN4ADDR_LOOPBACK: i32 = 16777343;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: i32 = 8;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: i32 = 4;
pub const IN4ADDR_NONE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INET_PORT_RANGE {
    pub StartPort: u16,
    pub NumberOfPorts: u16,
}
pub type INET_PORT_RESERVATION = INET_PORT_RANGE;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INET_PORT_RESERVATION_INFORMATION {
    pub OwningPid: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INET_PORT_RESERVATION_INSTANCE {
    pub Reservation: INET_PORT_RESERVATION,
    pub Token: INET_PORT_RESERVATION_TOKEN,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct RCVALL_IF {
    pub Mode: RCVALL_VALUE,
    pub Interface: u32,
}
pub const RCVALL_IPLEVEL: RCVALL_VALUE = 3;
pub const RCVALL_MAX: i32 = 3;
pub const RCVALL_OFF: RCVALL_VALUE = 0;
pub const RCVALL_ON: RCVALL_VALUE = 1;
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = 2;
pub type RCVALL_VALUE = i32;
pub const REAL_TIME_NOTIFICATION_CAPABILITY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b59819a_5cae_492d_a901_2a3c2c50164f);
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6843da03_154a_4616_a508_44371295f96b);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX {
    pub TransportSettingId: TRANSPORT_SETTING_ID,
    pub BrokerEventGuid: windows_sys::core::GUID,
    pub Unmark: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT {
    pub ChannelStatus: CONTROL_CHANNEL_TRIGGER_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RSS_SCALABILITY_INFO {
    pub RssEnabled: bool,
}
pub const SIO_ABSORB_RTRALERT: u32 = 2550136837;
pub const SIO_ACQUIRE_PORT_RESERVATION: u32 = 2550136932;
pub const SIO_APPLY_TRANSPORT_SETTING: u32 = 2550136851;
pub const SIO_ASSOCIATE_PORT_RESERVATION: u32 = 2550136934;
pub const SIO_CPU_AFFINITY: u32 = 2550136853;
pub const SIO_DELETE_PEER_TARGET_NAME: u32 = 2550137035;
pub const SIO_GET_TX_TIMESTAMP: u32 = 2550137066;
pub const SIO_INDEX_ADD_MCAST: u32 = 2550136842;
pub const SIO_INDEX_BIND: u32 = 2550136840;
pub const SIO_INDEX_DEL_MCAST: u32 = 2550136843;
pub const SIO_INDEX_MCASTIF: u32 = 2550136841;
pub const SIO_KEEPALIVE_VALS: u32 = 2550136836;
pub const SIO_LIMIT_BROADCASTS: u32 = 2550136839;
pub const SIO_LOOPBACK_FAST_PATH: u32 = 2550136848;
pub const SIO_PRIORITY_HINT: u32 = 2550136856;
pub const SIO_QUERY_RSS_SCALABILITY_INFO: i32 = 1476395218;
pub const SIO_QUERY_SECURITY: u32 = 3623878857;
pub const SIO_QUERY_TRANSPORT_SETTING: u32 = 2550136852;
pub const SIO_QUERY_WFP_ALE_ENDPOINT_HANDLE: i32 = 1476395213;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_CONTEXT: u32 = 2550137053;
pub const SIO_QUERY_WFP_CONNECTION_REDIRECT_RECORDS: u32 = 2550137052;
pub const SIO_RCVALL: u32 = 2550136833;
pub const SIO_RCVALL_IF: u32 = 2550136846;
pub const SIO_RCVALL_IGMPMCAST: u32 = 2550136835;
pub const SIO_RCVALL_MCAST: u32 = 2550136834;
pub const SIO_RCVALL_MCAST_IF: u32 = 2550136845;
pub const SIO_RELEASE_PORT_RESERVATION: u32 = 2550136933;
pub const SIO_SET_PEER_TARGET_NAME: u32 = 2550137034;
pub const SIO_SET_PRIORITY_HINT: u32 = 2550136856;
pub const SIO_SET_SECURITY: u32 = 2550137032;
pub const SIO_SET_WFP_CONNECTION_REDIRECT_RECORDS: u32 = 2550137054;
pub const SIO_SOCKET_USAGE_NOTIFICATION: u32 = 2550137036;
pub const SIO_TCP_INFO: u32 = 3623878695;
pub const SIO_TCP_INITIAL_RTO: u32 = 2550136849;
pub const SIO_TCP_SET_ACK_FREQUENCY: u32 = 2550136855;
pub const SIO_TCP_SET_ICW: u32 = 2550136854;
pub const SIO_TIMESTAMPING: u32 = 2550137067;
pub const SIO_UCAST_IF: u32 = 2550136838;
pub const SOCKET_DEFAULT2_QM_POLICY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaec2ef9c_3a4d_4d3e_8842_239942e39a47);
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: i32 = 2;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: i32 = 4;
pub const SOCKET_INFO_CONNECTION_SECURED: i32 = 1;
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy)]
pub struct SOCKET_PEER_TARGET_NAME {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::SOCKADDR_STORAGE,
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
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: i32 = 1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: i32 = 1;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: i32 = 2;
pub type SOCKET_SECURITY_PROTOCOL = i32;
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL = 0;
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL = 3;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = 1;
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SOCKET_SECURITY_QUERY_INFO {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub Flags: u32,
    pub PeerApplicationAccessTokenHandle: u64,
    pub PeerMachineAccessTokenHandle: u64,
    pub MmSaId: u64,
    pub QmSaId: u64,
    pub NegotiationWinerr: u32,
    pub SaLookupContext: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
}
#[repr(C)]
#[cfg(feature = "ws2")]
#[derive(Clone, Copy, Default)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2 {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub PeerAddress: super::SOCKADDR_STORAGE,
    pub PeerTokenAccessMask: u32,
    pub Flags: u32,
    pub FieldMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SOCKET_SECURITY_SETTINGS {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOCKET_SECURITY_SETTINGS_IPSEC {
    pub SecurityProtocol: SOCKET_SECURITY_PROTOCOL,
    pub SecurityFlags: u32,
    pub IpsecFlags: u32,
    pub AuthipMMPolicyKey: windows_sys::core::GUID,
    pub AuthipQMPolicyKey: windows_sys::core::GUID,
    pub Reserved: windows_sys::core::GUID,
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
pub const SOCKET_SETTINGS_ALLOW_INSECURE: i32 = 2;
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: i32 = 1;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: i32 = 4;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: i32 = 2;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: i32 = 8;
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: i32 = 1;
pub type SOCKET_USAGE_TYPE = i32;
pub const SO_TIMESTAMP: i32 = 12298;
pub const SO_TIMESTAMP_ID: i32 = 12299;
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct TCP_ICW_PARAMETERS {
    pub Level: TCP_ICW_LEVEL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
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
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: i32 = 0;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: i32 = 0;
pub const TCP_INITIAL_RTO_NO_SYN_RETRANSMISSIONS: u8 = 254;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TCP_INITIAL_RTO_PARAMETERS {
    pub Rtt: u16,
    pub MaxSynRetransmissions: u8,
}
pub const TCP_INITIAL_RTO_UNSPECIFIED_MAX_SYN_RETRANSMISSIONS: u8 = 255;
pub const TCP_INITIAL_RTO_UNSPECIFIED_RTT: u16 = 65535;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TIMESTAMPING_CONFIG {
    pub Flags: u32,
    pub TxTimestampsBuffered: u16,
}
pub const TIMESTAMPING_FLAG_RX: i32 = 1;
pub const TIMESTAMPING_FLAG_TX: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TRANSPORT_SETTING_ID {
    pub Guid: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tcp_keepalive {
    pub onoff: u32,
    pub keepalivetime: u32,
    pub keepaliveinterval: u32,
}
