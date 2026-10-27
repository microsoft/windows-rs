pub const FILE_DEVICE_SMARTCARD: i32 = 49;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: windows_core::GUID = windows_core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
pub const IOCTL_SMARTCARD_CONFISCATE: i32 = 3211280;
pub const IOCTL_SMARTCARD_EJECT: i32 = 3211288;
pub const IOCTL_SMARTCARD_GET_ATTRIBUTE: i32 = 3211272;
pub const IOCTL_SMARTCARD_GET_FEATURE_REQUEST: i32 = 3224864;
pub const IOCTL_SMARTCARD_GET_LAST_ERROR: i32 = 3211324;
pub const IOCTL_SMARTCARD_GET_PERF_CNTR: i32 = 3211328;
pub const IOCTL_SMARTCARD_GET_STATE: i32 = 3211320;
pub const IOCTL_SMARTCARD_IS_ABSENT: i32 = 3211308;
pub const IOCTL_SMARTCARD_IS_PRESENT: i32 = 3211304;
pub const IOCTL_SMARTCARD_POWER: i32 = 3211268;
pub const IOCTL_SMARTCARD_SET_ATTRIBUTE: i32 = 3211276;
pub const IOCTL_SMARTCARD_SET_PROTOCOL: i32 = 3211312;
pub const IOCTL_SMARTCARD_SWALLOW: i32 = 3211292;
pub const IOCTL_SMARTCARD_TRANSMIT: i32 = 3211284;
#[cfg(feature = "shared")]
pub type LPCSCARD_IO_REQUEST = *const SCARD_IO_REQUEST;
#[cfg(feature = "shared")]
pub type LPSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
#[cfg(feature = "shared")]
pub type LPSCARD_T0_COMMAND = *mut SCARD_T0_COMMAND;
#[cfg(feature = "shared")]
pub type LPSCARD_T0_REQUEST = *mut SCARD_T0_REQUEST;
#[cfg(feature = "shared")]
pub type LPSCARD_T1_REQUEST = *mut SCARD_T1_REQUEST;
pub const MAXIMUM_ATTR_STRING_LENGTH: i32 = 32;
pub const MAXIMUM_SMARTCARD_READERS: i32 = 10;
#[cfg(feature = "shared")]
pub type PSCARD_IO_REQUEST = *mut SCARD_IO_REQUEST;
#[cfg(feature = "shared")]
pub type PSCARD_T0_REQUEST = *mut SCARD_T0_REQUEST;
#[cfg(feature = "shared")]
pub type PSCARD_T1_REQUEST = *mut SCARD_T1_REQUEST;
pub const SCARD_ABSENT: i32 = 1;
pub const SCARD_ATR_LENGTH: i32 = 33;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ATR_STRING: super::ULONG = 590595;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CHANNEL_ID: super::ULONG = 131344;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CHARACTERISTICS: super::ULONG = 393552;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_BWT: super::ULONG = 524809;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_CLK: super::ULONG = 524802;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_CWT: super::ULONG = 524810;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_D: super::ULONG = 524804;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_EBC_ENCODING: super::ULONG = 524811;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_F: super::ULONG = 524803;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_IFSC: super::ULONG = 524807;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_IFSD: super::ULONG = 524808;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_IO_STATE: super::ULONG = 590594;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_N: super::ULONG = 524805;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_PROTOCOL_TYPE: super::ULONG = 524801;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_CURRENT_W: super::ULONG = 524806;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEFAULT_CLK: super::ULONG = 196897;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEFAULT_DATA_RATE: super::ULONG = 196899;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME: super::ULONG = 2147418115;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_A: super::ULONG = 2147418115;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_W: super::ULONG = 2147418117;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_IN_USE: super::ULONG = 2147418114;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME: super::ULONG = 2147418116;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_A: super::ULONG = 2147418116;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_W: super::ULONG = 2147418118;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_DEVICE_UNIT: super::ULONG = 2147418113;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ESC_AUTHREQUEST: super::ULONG = 499717;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ESC_CANCEL: super::ULONG = 499715;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ESC_RESET: super::ULONG = 499712;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_EXTENDED_BWT: super::ULONG = 524812;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ICC_INTERFACE_STATUS: super::ULONG = 590593;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ICC_PRESENCE: super::ULONG = 590592;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_ICC_TYPE_PER_ATR: super::ULONG = 590596;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_MAXINPUT: super::ULONG = 499719;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_MAX_CLK: super::ULONG = 196898;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_MAX_DATA_RATE: super::ULONG = 196900;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_MAX_IFSD: super::ULONG = 196901;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_POWER_MGMT_SUPPORT: super::ULONG = 262449;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_PROTOCOL_TYPES: super::ULONG = 196896;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_SUPRESS_T1_IFS_REQUEST: super::ULONG = 2147418119;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_USER_AUTH_INPUT_DEVICE: super::ULONG = 328002;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_USER_TO_CARD_AUTH_DEVICE: super::ULONG = 328000;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_VENDOR_IFD_SERIAL_NO: super::ULONG = 65795;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_VENDOR_IFD_TYPE: super::ULONG = 65793;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_VENDOR_IFD_VERSION: super::ULONG = 65794;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_VENDOR_NAME: super::ULONG = 65792;
#[cfg(feature = "shared")]
pub const SCARD_ATTR_VENDOR_SPECIFIC_INFO: super::ULONG = 499720;
pub const SCARD_CLASS_COMMUNICATIONS: i32 = 2;
pub const SCARD_CLASS_ICC_STATE: i32 = 9;
pub const SCARD_CLASS_IFD_PROTOCOL: i32 = 8;
pub const SCARD_CLASS_MECHANICAL: i32 = 6;
pub const SCARD_CLASS_PERF: i32 = 32766;
pub const SCARD_CLASS_POWER_MGMT: i32 = 4;
pub const SCARD_CLASS_PROTOCOL: i32 = 3;
pub const SCARD_CLASS_SECURITY: i32 = 5;
pub const SCARD_CLASS_SYSTEM: i32 = 32767;
pub const SCARD_CLASS_VENDOR_DEFINED: i32 = 7;
pub const SCARD_CLASS_VENDOR_INFO: i32 = 1;
pub const SCARD_COLD_RESET: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: super::DWORD,
    pub cbPciLength: super::DWORD,
}
pub const SCARD_NEGOTIABLE: i32 = 5;
#[cfg(feature = "shared")]
pub const SCARD_PERF_BYTES_TRANSMITTED: super::ULONG = 2147352578;
#[cfg(feature = "shared")]
pub const SCARD_PERF_NUM_TRANSMISSIONS: super::ULONG = 2147352577;
#[cfg(feature = "shared")]
pub const SCARD_PERF_TRANSMISSION_TIME: super::ULONG = 2147352579;
pub const SCARD_POWERED: i32 = 4;
pub const SCARD_POWER_DOWN: i32 = 0;
pub const SCARD_PRESENT: i32 = 2;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648;
pub const SCARD_PROTOCOL_OPTIMAL: i32 = 0;
pub const SCARD_PROTOCOL_RAW: i32 = 65536;
pub const SCARD_PROTOCOL_T0: i32 = 1;
pub const SCARD_PROTOCOL_T1: i32 = 2;
pub const SCARD_PROTOCOL_Tx: i32 = 3;
pub const SCARD_PROTOCOL_UNDEFINED: i32 = 0;
pub const SCARD_READER_CONFISCATES: i32 = 4;
pub const SCARD_READER_CONTACTLESS: i32 = 8;
pub const SCARD_READER_EJECTS: i32 = 2;
pub const SCARD_READER_SWALLOWS: i32 = 1;
pub const SCARD_READER_TYPE_EMBEDDEDSE: i32 = 2048;
pub const SCARD_READER_TYPE_IDE: i32 = 16;
pub const SCARD_READER_TYPE_KEYBOARD: i32 = 4;
pub const SCARD_READER_TYPE_NFC: i32 = 256;
pub const SCARD_READER_TYPE_NGC: i32 = 1024;
pub const SCARD_READER_TYPE_PARALELL: i32 = 2;
pub const SCARD_READER_TYPE_PCMCIA: i32 = 64;
pub const SCARD_READER_TYPE_SCSI: i32 = 8;
pub const SCARD_READER_TYPE_SERIAL: i32 = 1;
pub const SCARD_READER_TYPE_TPM: i32 = 128;
pub const SCARD_READER_TYPE_UICC: i32 = 512;
pub const SCARD_READER_TYPE_USB: i32 = 32;
pub const SCARD_READER_TYPE_VENDOR: i32 = 240;
pub const SCARD_SPECIFIC: i32 = 6;
pub const SCARD_SWALLOWED: i32 = 3;
pub const SCARD_T0_CMD_LENGTH: i32 = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCARD_T0_COMMAND {
    pub bCla: super::BYTE,
    pub bIns: super::BYTE,
    pub bP1: super::BYTE,
    pub bP2: super::BYTE,
    pub bP3: super::BYTE,
}
pub const SCARD_T0_HEADER_LENGTH: i32 = 7;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: super::BYTE,
    pub bSw2: super::BYTE,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
#[cfg(feature = "shared")]
impl Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [super::BYTE; 5],
}
#[cfg(feature = "shared")]
impl Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_T1_EPILOGUE_LENGTH: i32 = 2;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: i32 = 1;
pub const SCARD_T1_MAX_IFS: i32 = 254;
pub const SCARD_T1_PROLOGUE_LENGTH: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
pub const SCARD_UNKNOWN: i32 = 0;
pub const SCARD_WARM_RESET: i32 = 2;
