pub type BTHNS_INQUIRYBLOB = BTH_QUERY_DEVICE;
#[cfg(feature = "Win32_bthsdpdef")]
pub type BTHNS_RESTRICTIONBLOB = BTH_QUERY_SERVICE;
pub const BTHNS_RESULT_DEVICE_AUTHENTICATED: u32 = 262144;
pub const BTHNS_RESULT_DEVICE_CONNECTED: u32 = 65536;
pub const BTHNS_RESULT_DEVICE_REMEMBERED: u32 = 131072;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type BTHNS_SETBLOB = BTH_SET_SERVICE;
pub const BTHPROTO_L2CAP: u32 = 256;
pub const BTHPROTO_RFCOMM: u32 = 3;
pub const BTH_ADDR_STRING_SIZE: u32 = 12;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy, Default)]
pub struct BTH_INFO_REQ {
    pub btAddr: super::bthdef::BTH_ADDR,
    pub infoType: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BTH_INFO_RSP {
    pub result: u16,
    pub dataLen: u8,
    pub Anonymous: BTH_INFO_RSP_0,
}
impl Default for BTH_INFO_RSP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union BTH_INFO_RSP_0 {
    pub connectionlessMTU: u16,
    pub data: [u8; 44],
}
impl Default for BTH_INFO_RSP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy)]
pub struct BTH_PING_REQ {
    pub btAddr: super::bthdef::BTH_ADDR,
    pub dataLen: u8,
    pub data: [u8; 44],
}
#[cfg(feature = "Win32_bthdef")]
impl Default for BTH_PING_REQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BTH_PING_RSP {
    pub dataLen: u8,
    pub data: [u8; 44],
}
impl Default for BTH_PING_RSP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BTH_QUERY_DEVICE {
    pub LAP: u32,
    pub length: u8,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bthsdpdef")]
#[derive(Clone, Copy)]
pub struct BTH_QUERY_SERVICE {
    pub r#type: u32,
    pub serviceHandle: u32,
    pub uuids: [super::bthsdpdef::SdpQueryUuid; 12],
    pub numRange: u32,
    pub pRange: [super::bthsdpdef::SdpAttributeRange; 1],
}
#[cfg(feature = "Win32_bthsdpdef")]
impl Default for BTH_QUERY_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BTH_SDP_VERSION: u32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct BTH_SET_SERVICE {
    pub pSdpVersion: super::minwindef::PULONG,
    pub pRecordHandle: *mut super::winnt::HANDLE,
    pub fCodService: u32,
    pub Reserved: [u32; 5],
    pub ulRecordLength: u32,
    pub pRecord: [u8; 1],
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for BTH_SET_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BT_PORT_ANY: u32 = 4294967295;
pub const BT_PORT_DYN_FIRST: u32 = 4097;
pub const BT_PORT_MAX: u32 = 65535;
pub const BT_PORT_MIN: u32 = 1;
pub const MSC_BREAK_BIT: u32 = 2;
pub const MSC_DV_BIT: u32 = 128;
pub const MSC_FC_BIT: u32 = 2;
pub const MSC_IC_BIT: u32 = 64;
pub const MSC_RESERVED: u32 = 48;
pub const MSC_RTC_BIT: u32 = 4;
pub const MSC_RTR_BIT: u32 = 8;
pub type PBTHNS_INQUIRYBLOB = *mut BTH_QUERY_DEVICE;
#[cfg(feature = "Win32_bthsdpdef")]
pub type PBTHNS_RESTRICTIONBLOB = *mut BTH_QUERY_SERVICE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PBTHNS_SETBLOB = *mut BTH_SET_SERVICE;
#[cfg(feature = "Win32_bthdef")]
pub type PBTH_INFO_REQ = *mut BTH_INFO_REQ;
pub type PBTH_INFO_RSP = *mut BTH_INFO_RSP;
#[cfg(feature = "Win32_bthdef")]
pub type PBTH_PING_REQ = *mut BTH_PING_REQ;
pub type PBTH_PING_RSP = *mut BTH_PING_RSP;
pub type PBTH_QUERY_DEVICE = *mut BTH_QUERY_DEVICE;
#[cfg(feature = "Win32_bthsdpdef")]
pub type PBTH_QUERY_SERVICE = *mut BTH_QUERY_SERVICE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PBTH_SET_SERVICE = *mut BTH_SET_SERVICE;
pub type PRFCOMM_COMMAND = *mut RFCOMM_COMMAND;
pub type PRFCOMM_MSC_DATA = *mut RFCOMM_MSC_DATA;
pub type PRFCOMM_RLS_DATA = *mut RFCOMM_RLS_DATA;
pub type PRFCOMM_RPN_DATA = *mut RFCOMM_RPN_DATA;
#[cfg(feature = "Win32_bthdef")]
pub type PSOCKADDR_BTH = *mut SOCKADDR_BTH;
pub const RFCOMM_CMD_MSC: u32 = 1;
pub const RFCOMM_CMD_NONE: u32 = 0;
pub const RFCOMM_CMD_RLS: u32 = 2;
pub const RFCOMM_CMD_RPN: u32 = 3;
pub const RFCOMM_CMD_RPN_REQUEST: u32 = 4;
pub const RFCOMM_CMD_RPN_RESPONSE: u32 = 5;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFCOMM_COMMAND {
    pub CmdType: u32,
    pub Data: RFCOMM_COMMAND_0,
}
impl Default for RFCOMM_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RFCOMM_COMMAND_0 {
    pub MSC: RFCOMM_MSC_DATA,
    pub RLS: RFCOMM_RLS_DATA,
    pub RPN: RFCOMM_RPN_DATA,
}
impl Default for RFCOMM_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RFCOMM_MAX_MTU: u32 = 1011;
pub const RFCOMM_MIN_MTU: u32 = 23;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_MSC_DATA {
    pub Signals: u8,
    pub Break: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_RLS_DATA {
    pub LineStatus: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_RPN_DATA {
    pub Baud: u8,
    pub Data: u8,
    pub FlowControl: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
    pub ParameterMask1: u8,
    pub ParameterMask2: u8,
}
pub const RLS_ERROR: u32 = 1;
pub const RLS_FRAMING: u32 = 8;
pub const RLS_OVERRUN: u32 = 2;
pub const RLS_PARITY: u32 = 4;
pub const RPN_BAUD_115200: u32 = 7;
pub const RPN_BAUD_19200: u32 = 4;
pub const RPN_BAUD_230400: u32 = 8;
pub const RPN_BAUD_2400: u32 = 0;
pub const RPN_BAUD_38400: u32 = 5;
pub const RPN_BAUD_4800: u32 = 1;
pub const RPN_BAUD_57600: u32 = 6;
pub const RPN_BAUD_7200: u32 = 2;
pub const RPN_BAUD_9600: u32 = 3;
pub const RPN_DATA_5: u32 = 0;
pub const RPN_DATA_6: u32 = 1;
pub const RPN_DATA_7: u32 = 2;
pub const RPN_DATA_8: u32 = 3;
pub const RPN_FLOW_RTC_IN: u32 = 16;
pub const RPN_FLOW_RTC_OUT: u32 = 32;
pub const RPN_FLOW_RTR_IN: u32 = 4;
pub const RPN_FLOW_RTR_OUT: u32 = 8;
pub const RPN_FLOW_X_IN: u32 = 1;
pub const RPN_FLOW_X_OUT: u32 = 2;
pub const RPN_PARAM_BAUD: u32 = 1;
pub const RPN_PARAM_DATA: u32 = 2;
pub const RPN_PARAM_PARITY: u32 = 8;
pub const RPN_PARAM_P_TYPE: u32 = 16;
pub const RPN_PARAM_RTC_IN: u32 = 16;
pub const RPN_PARAM_RTC_OUT: u32 = 32;
pub const RPN_PARAM_RTR_IN: u32 = 4;
pub const RPN_PARAM_RTR_OUT: u32 = 8;
pub const RPN_PARAM_STOP: u32 = 4;
pub const RPN_PARAM_XOFF: u32 = 64;
pub const RPN_PARAM_XON: u32 = 32;
pub const RPN_PARAM_X_IN: u32 = 1;
pub const RPN_PARAM_X_OUT: u32 = 2;
pub const RPN_PARITY_EVEN: u32 = 24;
pub const RPN_PARITY_MARK: u32 = 40;
pub const RPN_PARITY_NONE: u32 = 0;
pub const RPN_PARITY_ODD: u32 = 8;
pub const RPN_PARITY_SPACE: u32 = 56;
pub const RPN_STOP_1: u32 = 0;
pub const RPN_STOP_1_5: u32 = 4;
pub const SDP_DEFAULT_INQUIRY_MAX_RESPONSES: u32 = 255;
pub const SDP_DEFAULT_INQUIRY_SECONDS: u32 = 6;
pub const SDP_MAX_INQUIRY_SECONDS: u32 = 60;
pub const SDP_SERVICE_ATTRIBUTE_REQUEST: u32 = 2;
pub const SDP_SERVICE_SEARCH_ATTRIBUTE_REQUEST: u32 = 3;
pub const SDP_SERVICE_SEARCH_REQUEST: u32 = 1;
pub const SIO_BTH_INFO: i32 = -671088631;
pub const SIO_BTH_PING: i32 = -671088632;
pub const SIO_RFCOMM_SEND_COMMAND: i32 = -671088539;
pub const SIO_RFCOMM_SESSION_FLOW_OFF: i32 = -671088537;
pub const SIO_RFCOMM_TEST: i32 = -671088536;
pub const SIO_RFCOMM_USECFC: i32 = -671088535;
pub const SIO_RFCOMM_WAIT_COMMAND: i32 = -671088538;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bthdef")]
#[derive(Clone, Copy, Default)]
pub struct SOCKADDR_BTH {
    pub addressFamily: u16,
    pub btAddr: super::bthdef::BTH_ADDR,
    pub serviceClassId: windows_sys::core::GUID,
    pub port: u32,
}
pub const SOL_L2CAP: u32 = 256;
pub const SOL_RFCOMM: u32 = 3;
pub const SOL_SDP: u32 = 257;
pub const SO_BTH_AUTHENTICATE: u32 = 2147483649;
pub const SO_BTH_ENCRYPT: u32 = 2;
pub const SO_BTH_MTU: u32 = 2147483655;
pub const SO_BTH_MTU_MAX: u32 = 2147483656;
pub const SO_BTH_MTU_MIN: u32 = 2147483658;
pub const SVCID_BTH_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06aa63e0_7d60_41ff_afb2_3ee6d2d9392d);
