pub const GUID_DEVINTERFACE_SMARTCARD_READER: windows_core::GUID = windows_core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
pub const IOCTL_SMARTCARD_CONFISCATE: u32 = 3211280;
pub const IOCTL_SMARTCARD_EJECT: u32 = 3211288;
pub const IOCTL_SMARTCARD_GET_ATTRIBUTE: u32 = 3211272;
pub const IOCTL_SMARTCARD_GET_FEATURE_REQUEST: u32 = 3224864;
pub const IOCTL_SMARTCARD_GET_LAST_ERROR: u32 = 3211324;
pub const IOCTL_SMARTCARD_GET_PERF_CNTR: u32 = 3211328;
pub const IOCTL_SMARTCARD_GET_STATE: u32 = 3211320;
pub const IOCTL_SMARTCARD_IS_ABSENT: u32 = 3211308;
pub const IOCTL_SMARTCARD_IS_PRESENT: u32 = 3211304;
pub const IOCTL_SMARTCARD_POWER: u32 = 3211268;
pub const IOCTL_SMARTCARD_SET_ATTRIBUTE: u32 = 3211276;
pub const IOCTL_SMARTCARD_SET_PROTOCOL: u32 = 3211312;
pub const IOCTL_SMARTCARD_SWALLOW: u32 = 3211292;
pub const IOCTL_SMARTCARD_TRANSMIT: u32 = 3211284;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSCARD_IO_REQUEST(pub *const SCARD_IO_REQUEST);
impl LPCSCARD_IO_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCSCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSCARD_IO_REQUEST(pub *mut SCARD_IO_REQUEST);
impl LPSCARD_IO_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSCARD_T0_COMMAND(pub *mut SCARD_T0_COMMAND);
impl LPSCARD_T0_COMMAND {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSCARD_T0_REQUEST(pub *mut SCARD_T0_REQUEST);
impl LPSCARD_T0_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSCARD_T1_REQUEST(pub *mut SCARD_T1_REQUEST);
impl LPSCARD_T1_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32;
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCARD_IO_REQUEST(pub *mut SCARD_IO_REQUEST);
impl PSCARD_IO_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCARD_T0_REQUEST(pub *mut SCARD_T0_REQUEST);
impl PSCARD_T0_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCARD_T1_REQUEST(pub *mut SCARD_T1_REQUEST);
impl PSCARD_T1_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_ABSENT: u32 = 1;
pub const SCARD_ATR_LENGTH: u32 = 33;
pub const SCARD_ATTR_ATR_STRING: u32 = 590595;
pub const SCARD_ATTR_CHANNEL_ID: u32 = 131344;
pub const SCARD_ATTR_CHARACTERISTICS: u32 = 393552;
pub const SCARD_ATTR_CURRENT_BWT: u32 = 524809;
pub const SCARD_ATTR_CURRENT_CLK: u32 = 524802;
pub const SCARD_ATTR_CURRENT_CWT: u32 = 524810;
pub const SCARD_ATTR_CURRENT_D: u32 = 524804;
pub const SCARD_ATTR_CURRENT_EBC_ENCODING: u32 = 524811;
pub const SCARD_ATTR_CURRENT_F: u32 = 524803;
pub const SCARD_ATTR_CURRENT_IFSC: u32 = 524807;
pub const SCARD_ATTR_CURRENT_IFSD: u32 = 524808;
pub const SCARD_ATTR_CURRENT_IO_STATE: u32 = 590594;
pub const SCARD_ATTR_CURRENT_N: u32 = 524805;
pub const SCARD_ATTR_CURRENT_PROTOCOL_TYPE: u32 = 524801;
pub const SCARD_ATTR_CURRENT_W: u32 = 524806;
pub const SCARD_ATTR_DEFAULT_CLK: u32 = 196897;
pub const SCARD_ATTR_DEFAULT_DATA_RATE: u32 = 196899;
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME: u32 = 2147418115;
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_A: u32 = 2147418115;
pub const SCARD_ATTR_DEVICE_FRIENDLY_NAME_W: u32 = 2147418117;
pub const SCARD_ATTR_DEVICE_IN_USE: u32 = 2147418114;
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME: u32 = 2147418116;
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_A: u32 = 2147418116;
pub const SCARD_ATTR_DEVICE_SYSTEM_NAME_W: u32 = 2147418118;
pub const SCARD_ATTR_DEVICE_UNIT: u32 = 2147418113;
pub const SCARD_ATTR_ESC_AUTHREQUEST: u32 = 499717;
pub const SCARD_ATTR_ESC_CANCEL: u32 = 499715;
pub const SCARD_ATTR_ESC_RESET: u32 = 499712;
pub const SCARD_ATTR_EXTENDED_BWT: u32 = 524812;
pub const SCARD_ATTR_ICC_INTERFACE_STATUS: u32 = 590593;
pub const SCARD_ATTR_ICC_PRESENCE: u32 = 590592;
pub const SCARD_ATTR_ICC_TYPE_PER_ATR: u32 = 590596;
pub const SCARD_ATTR_MAXINPUT: u32 = 499719;
pub const SCARD_ATTR_MAX_CLK: u32 = 196898;
pub const SCARD_ATTR_MAX_DATA_RATE: u32 = 196900;
pub const SCARD_ATTR_MAX_IFSD: u32 = 196901;
pub const SCARD_ATTR_POWER_MGMT_SUPPORT: u32 = 262449;
pub const SCARD_ATTR_PROTOCOL_TYPES: u32 = 196896;
pub const SCARD_ATTR_SUPRESS_T1_IFS_REQUEST: u32 = 2147418119;
pub const SCARD_ATTR_USER_AUTH_INPUT_DEVICE: u32 = 328002;
pub const SCARD_ATTR_USER_TO_CARD_AUTH_DEVICE: u32 = 328000;
pub const SCARD_ATTR_VENDOR_IFD_SERIAL_NO: u32 = 65795;
pub const SCARD_ATTR_VENDOR_IFD_TYPE: u32 = 65793;
pub const SCARD_ATTR_VENDOR_IFD_VERSION: u32 = 65794;
pub const SCARD_ATTR_VENDOR_NAME: u32 = 65792;
pub const SCARD_ATTR_VENDOR_SPECIFIC_INFO: u32 = 499720;
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2;
pub const SCARD_CLASS_ICC_STATE: u32 = 9;
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8;
pub const SCARD_CLASS_MECHANICAL: u32 = 6;
pub const SCARD_CLASS_PERF: u32 = 32766;
pub const SCARD_CLASS_POWER_MGMT: u32 = 4;
pub const SCARD_CLASS_PROTOCOL: u32 = 3;
pub const SCARD_CLASS_SECURITY: u32 = 5;
pub const SCARD_CLASS_SYSTEM: u32 = 32767;
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7;
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1;
pub const SCARD_COLD_RESET: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
pub const SCARD_NEGOTIABLE: u32 = 5;
pub const SCARD_PERF_BYTES_TRANSMITTED: u32 = 2147352578;
pub const SCARD_PERF_NUM_TRANSMISSIONS: u32 = 2147352577;
pub const SCARD_PERF_TRANSMISSION_TIME: u32 = 2147352579;
pub const SCARD_POWERED: u32 = 4;
pub const SCARD_POWER_DOWN: u32 = 0;
pub const SCARD_PRESENT: u32 = 2;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648;
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0;
pub const SCARD_PROTOCOL_RAW: u32 = 65536;
pub const SCARD_PROTOCOL_T0: u32 = 1;
pub const SCARD_PROTOCOL_T1: u32 = 2;
pub const SCARD_PROTOCOL_Tx: u32 = 3;
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0;
pub const SCARD_READER_CONFISCATES: u32 = 4;
pub const SCARD_READER_CONTACTLESS: u32 = 8;
pub const SCARD_READER_EJECTS: u32 = 2;
pub const SCARD_READER_SWALLOWS: u32 = 1;
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048;
pub const SCARD_READER_TYPE_IDE: u32 = 16;
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4;
pub const SCARD_READER_TYPE_NFC: u32 = 256;
pub const SCARD_READER_TYPE_NGC: u32 = 1024;
pub const SCARD_READER_TYPE_PARALELL: u32 = 2;
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64;
pub const SCARD_READER_TYPE_SCSI: u32 = 8;
pub const SCARD_READER_TYPE_SERIAL: u32 = 1;
pub const SCARD_READER_TYPE_TPM: u32 = 128;
pub const SCARD_READER_TYPE_UICC: u32 = 512;
pub const SCARD_READER_TYPE_USB: u32 = 32;
pub const SCARD_READER_TYPE_VENDOR: u32 = 240;
pub const SCARD_SPECIFIC: u32 = 6;
pub const SCARD_SWALLOWED: u32 = 3;
pub const SCARD_T0_CMD_LENGTH: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
pub const SCARD_T0_HEADER_LENGTH: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1;
pub const SCARD_T1_MAX_IFS: u32 = 254;
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
pub const SCARD_UNKNOWN: u32 = 0;
pub const SCARD_WARM_RESET: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UWORD(pub u16);
