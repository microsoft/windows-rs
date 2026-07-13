pub const EVEN_PARITY: u32 = 2;
pub const GUID_DEVINTERFACE_COMPORT: windows_core::GUID = windows_core::GUID::from_u128(0x86e0d1e0_8089_11d0_9ce4_08003e301f73);
pub const GUID_DEVINTERFACE_SERENUM_BUS_ENUMERATOR: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e978_e325_11ce_bfc1_08002be10318);
pub const IOCTL_INTERNAL_SERENUM_REMOVE_SELF: u32 = 3604999;
pub const IOCTL_SERIAL_APPLY_DEFAULT_CONFIGURATION: u32 = 1769632;
pub const IOCTL_SERIAL_CLEAR_STATS: u32 = 1769616;
pub const IOCTL_SERIAL_CLR_DTR: u32 = 1769512;
pub const IOCTL_SERIAL_CLR_RTS: u32 = 1769524;
pub const IOCTL_SERIAL_CONFIG_SIZE: u32 = 1769600;
pub const IOCTL_SERIAL_GET_BAUD_RATE: u32 = 1769552;
pub const IOCTL_SERIAL_GET_CHARS: u32 = 1769560;
pub const IOCTL_SERIAL_GET_COMMCONFIG: u32 = 1769604;
pub const IOCTL_SERIAL_GET_COMMSTATUS: u32 = 1769580;
pub const IOCTL_SERIAL_GET_DTRRTS: u32 = 1769592;
pub const IOCTL_SERIAL_GET_HANDFLOW: u32 = 1769568;
pub const IOCTL_SERIAL_GET_LINE_CONTROL: u32 = 1769556;
pub const IOCTL_SERIAL_GET_MODEMSTATUS: u32 = 1769576;
pub const IOCTL_SERIAL_GET_MODEM_CONTROL: u32 = 1769620;
pub const IOCTL_SERIAL_GET_PROPERTIES: u32 = 1769588;
pub const IOCTL_SERIAL_GET_STATS: u32 = 1769612;
pub const IOCTL_SERIAL_GET_TIMEOUTS: u32 = 1769504;
pub const IOCTL_SERIAL_GET_WAIT_MASK: u32 = 1769536;
pub const IOCTL_SERIAL_IMMEDIATE_CHAR: u32 = 1769496;
pub const IOCTL_SERIAL_INTERNAL_BASIC_SETTINGS: u32 = 1769484;
pub const IOCTL_SERIAL_INTERNAL_CANCEL_WAIT_WAKE: u32 = 1769480;
pub const IOCTL_SERIAL_INTERNAL_DO_WAIT_WAKE: u32 = 1769476;
pub const IOCTL_SERIAL_INTERNAL_RESTORE_SETTINGS: u32 = 1769488;
pub const IOCTL_SERIAL_PURGE: u32 = 1769548;
pub const IOCTL_SERIAL_RESET_DEVICE: u32 = 1769516;
pub const IOCTL_SERIAL_SET_BAUD_RATE: u32 = 1769476;
pub const IOCTL_SERIAL_SET_BREAK_OFF: u32 = 1769492;
pub const IOCTL_SERIAL_SET_BREAK_ON: u32 = 1769488;
pub const IOCTL_SERIAL_SET_CHARS: u32 = 1769564;
pub const IOCTL_SERIAL_SET_COMMCONFIG: u32 = 1769608;
pub const IOCTL_SERIAL_SET_DTR: u32 = 1769508;
pub const IOCTL_SERIAL_SET_FIFO_CONTROL: u32 = 1769628;
pub const IOCTL_SERIAL_SET_HANDFLOW: u32 = 1769572;
pub const IOCTL_SERIAL_SET_INTERVAL_TIMER_RESOLUTION: u32 = 1769636;
pub const IOCTL_SERIAL_SET_LINE_CONTROL: u32 = 1769484;
pub const IOCTL_SERIAL_SET_MODEM_CONTROL: u32 = 1769624;
pub const IOCTL_SERIAL_SET_QUEUE_SIZE: u32 = 1769480;
pub const IOCTL_SERIAL_SET_RTS: u32 = 1769520;
pub const IOCTL_SERIAL_SET_TIMEOUTS: u32 = 1769500;
pub const IOCTL_SERIAL_SET_WAIT_MASK: u32 = 1769540;
pub const IOCTL_SERIAL_SET_XOFF: u32 = 1769528;
pub const IOCTL_SERIAL_SET_XON: u32 = 1769532;
pub const IOCTL_SERIAL_WAIT_ON_MASK: u32 = 1769544;
pub const IOCTL_SERIAL_XOFF_COUNTER: u32 = 1769584;
pub const MARK_PARITY: u32 = 3;
pub const NO_PARITY: u32 = 0;
pub const ODD_PARITY: u32 = 1;
pub type PSERENUM_PORT_DESC = *mut SERENUM_PORT_DESC;
pub type PSERENUM_PORT_PARAMETERS = *mut SERENUM_PORT_PARAMETERS;
pub type PSERENUM_READPORT = Option<unsafe extern "system" fn(serportaddress: *const core::ffi::c_void) -> u8>;
pub type PSERENUM_WRITEPORT = Option<unsafe extern "system" fn(serportaddress: *const core::ffi::c_void, value: u8)>;
pub type PSERIALCONFIG = *mut SERIALCONFIG;
pub type PSERIALPERF_STATS = *mut SERIALPERF_STATS;
pub type PSERIAL_BASIC_SETTINGS = *mut SERIAL_BASIC_SETTINGS;
pub type PSERIAL_BAUD_RATE = *mut SERIAL_BAUD_RATE;
pub type PSERIAL_CHARS = *mut SERIAL_CHARS;
pub type PSERIAL_COMMPROP = *mut SERIAL_COMMPROP;
pub type PSERIAL_HANDFLOW = *mut SERIAL_HANDFLOW;
pub type PSERIAL_LINE_CONTROL = *mut SERIAL_LINE_CONTROL;
pub type PSERIAL_QUEUE_SIZE = *mut SERIAL_QUEUE_SIZE;
pub type PSERIAL_STATUS = *mut SERIAL_STATUS;
pub type PSERIAL_TIMEOUTS = *mut SERIAL_TIMEOUTS;
pub type PSERIAL_XOFF_COUNTER = *mut SERIAL_XOFF_COUNTER;
pub type SERENUM_PORTION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERENUM_PORT_DESC {
    pub Size: u32,
    pub PortHandle: *mut core::ffi::c_void,
    pub PortAddress: i64,
    pub Reserved: [u16; 1],
}
impl Default for SERENUM_PORT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERENUM_PORT_PARAMETERS {
    pub Size: u32,
    pub ReadAccessor: PSERENUM_READPORT,
    pub WriteAccessor: PSERENUM_WRITEPORT,
    pub SerPortAddress: *mut core::ffi::c_void,
    pub HardwareHandle: *mut core::ffi::c_void,
    pub Portion: SERENUM_PORTION,
    pub NumberAxis: u16,
    pub Reserved: [u16; 3],
}
impl Default for SERENUM_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERIALCONFIG {
    pub Size: u32,
    pub Version: u16,
    pub SubType: u32,
    pub ProvOffset: u32,
    pub ProviderSize: u32,
    pub ProviderData: [u16; 1],
}
impl Default for SERIALCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIALPERF_STATS {
    pub ReceivedCount: u32,
    pub TransmittedCount: u32,
    pub FrameErrorCount: u32,
    pub SerialOverrunErrorCount: u32,
    pub BufferOverrunErrorCount: u32,
    pub ParityErrorCount: u32,
}
pub const SERIAL_AUTO_RECEIVE: u32 = 2;
pub const SERIAL_AUTO_TRANSMIT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_BASIC_SETTINGS {
    pub Timeouts: SERIAL_TIMEOUTS,
    pub HandFlow: SERIAL_HANDFLOW,
    pub RxFifo: u32,
    pub TxFifo: u32,
}
pub const SERIAL_BAUD_075: u32 = 1;
pub const SERIAL_BAUD_110: u32 = 2;
pub const SERIAL_BAUD_115200: u32 = 131072;
pub const SERIAL_BAUD_1200: u32 = 64;
pub const SERIAL_BAUD_128K: u32 = 65536;
pub const SERIAL_BAUD_134_5: u32 = 4;
pub const SERIAL_BAUD_14400: u32 = 4096;
pub const SERIAL_BAUD_150: u32 = 8;
pub const SERIAL_BAUD_1800: u32 = 128;
pub const SERIAL_BAUD_19200: u32 = 8192;
pub const SERIAL_BAUD_2400: u32 = 256;
pub const SERIAL_BAUD_300: u32 = 16;
pub const SERIAL_BAUD_38400: u32 = 16384;
pub const SERIAL_BAUD_4800: u32 = 512;
pub const SERIAL_BAUD_56K: u32 = 32768;
pub const SERIAL_BAUD_57600: u32 = 262144;
pub const SERIAL_BAUD_600: u32 = 32;
pub const SERIAL_BAUD_7200: u32 = 1024;
pub const SERIAL_BAUD_9600: u32 = 2048;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_BAUD_RATE {
    pub BaudRate: u32,
}
pub const SERIAL_BAUD_USER: u32 = 268435456;
pub const SERIAL_BREAK_CHAR: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_CHARS {
    pub EofChar: u8,
    pub ErrorChar: u8,
    pub BreakChar: u8,
    pub EventChar: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERIAL_COMMPROP {
    pub PacketLength: u16,
    pub PacketVersion: u16,
    pub ServiceMask: u32,
    pub Reserved1: u32,
    pub MaxTxQueue: u32,
    pub MaxRxQueue: u32,
    pub MaxBaud: u32,
    pub ProvSubType: u32,
    pub ProvCapabilities: u32,
    pub SettableParams: u32,
    pub SettableBaud: u32,
    pub SettableData: u16,
    pub SettableStopParity: u16,
    pub CurrentTxQueue: u32,
    pub CurrentRxQueue: u32,
    pub ProvSpec1: u32,
    pub ProvSpec2: u32,
    pub ProvChar: [u16; 1],
}
impl Default for SERIAL_COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERIAL_CONTROL_INVALID: u32 = 2147483524;
pub const SERIAL_CTS_HANDSHAKE: u32 = 8;
pub const SERIAL_CTS_STATE: u32 = 16;
pub const SERIAL_DATABITS_16: u16 = 16;
pub const SERIAL_DATABITS_16X: u16 = 32;
pub const SERIAL_DATABITS_5: u16 = 1;
pub const SERIAL_DATABITS_6: u16 = 2;
pub const SERIAL_DATABITS_7: u16 = 4;
pub const SERIAL_DATABITS_8: u16 = 8;
pub const SERIAL_DCD_HANDSHAKE: u32 = 32;
pub const SERIAL_DCD_STATE: u32 = 128;
pub const SERIAL_DSR_HANDSHAKE: u32 = 16;
pub const SERIAL_DSR_SENSITIVITY: u32 = 64;
pub const SERIAL_DSR_STATE: u32 = 32;
pub const SERIAL_DTR_CONTROL: u32 = 1;
pub const SERIAL_DTR_HANDSHAKE: u32 = 2;
pub const SERIAL_DTR_MASK: u32 = 3;
pub const SERIAL_DTR_STATE: u32 = 1;
pub const SERIAL_ERROR_ABORT: u32 = 2147483648;
pub const SERIAL_ERROR_BREAK: u32 = 1;
pub const SERIAL_ERROR_CHAR: u32 = 4;
pub const SERIAL_ERROR_FRAMING: u32 = 2;
pub const SERIAL_ERROR_OVERRUN: u32 = 4;
pub const SERIAL_ERROR_PARITY: u32 = 16;
pub const SERIAL_ERROR_QUEUEOVERRUN: u32 = 8;
pub const SERIAL_EV_BREAK: u32 = 64;
pub const SERIAL_EV_CTS: u32 = 8;
pub const SERIAL_EV_DSR: u32 = 16;
pub const SERIAL_EV_ERR: u32 = 128;
pub const SERIAL_EV_EVENT1: u32 = 2048;
pub const SERIAL_EV_EVENT2: u32 = 4096;
pub const SERIAL_EV_PERR: u32 = 512;
pub const SERIAL_EV_RING: u32 = 256;
pub const SERIAL_EV_RLSD: u32 = 32;
pub const SERIAL_EV_RX80FULL: u32 = 1024;
pub const SERIAL_EV_RXCHAR: u32 = 1;
pub const SERIAL_EV_RXFLAG: u32 = 2;
pub const SERIAL_EV_TXEMPTY: u32 = 4;
pub const SERIAL_FLOW_INVALID: u32 = 2147483424;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_HANDFLOW {
    pub ControlHandShake: u32,
    pub FlowReplace: u32,
    pub XonLimit: i32,
    pub XoffLimit: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_LINE_CONTROL {
    pub StopBits: u8,
    pub Parity: u8,
    pub WordLength: u8,
}
pub const SERIAL_NULL_STRIPPING: u32 = 8;
pub const SERIAL_OUT_HANDSHAKEMASK: u32 = 56;
pub const SERIAL_PARITY_EVEN: u16 = 1024;
pub const SERIAL_PARITY_MARK: u16 = 2048;
pub const SERIAL_PARITY_NONE: u16 = 256;
pub const SERIAL_PARITY_ODD: u16 = 512;
pub const SERIAL_PARITY_SPACE: u16 = 4096;
pub const SERIAL_PCF_16BITMODE: u32 = 512;
pub const SERIAL_PCF_CD: u32 = 4;
pub const SERIAL_PCF_DTRDSR: u32 = 1;
pub const SERIAL_PCF_INTTIMEOUTS: u32 = 128;
pub const SERIAL_PCF_PARITY_CHECK: u32 = 8;
pub const SERIAL_PCF_RTSCTS: u32 = 2;
pub const SERIAL_PCF_SETXCHAR: u32 = 32;
pub const SERIAL_PCF_SPECIALCHARS: u32 = 256;
pub const SERIAL_PCF_TOTALTIMEOUTS: u32 = 64;
pub const SERIAL_PCF_XONXOFF: u32 = 16;
pub const SERIAL_PURGE_RXABORT: u32 = 2;
pub const SERIAL_PURGE_RXCLEAR: u32 = 8;
pub const SERIAL_PURGE_TXABORT: u32 = 1;
pub const SERIAL_PURGE_TXCLEAR: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_QUEUE_SIZE {
    pub InSize: u32,
    pub OutSize: u32,
}
pub const SERIAL_RI_STATE: u32 = 64;
pub const SERIAL_RTS_CONTROL: u32 = 64;
pub const SERIAL_RTS_HANDSHAKE: u32 = 128;
pub const SERIAL_RTS_MASK: u32 = 192;
pub const SERIAL_RTS_STATE: u32 = 2;
pub const SERIAL_RX_WAITING_FOR_DSR: u32 = 64;
pub const SERIAL_SP_BAUD: u32 = 2;
pub const SERIAL_SP_BRIDGE: u32 = 256;
pub const SERIAL_SP_CARRIER_DETECT: u32 = 64;
pub const SERIAL_SP_DATABITS: u32 = 4;
pub const SERIAL_SP_FAX: u32 = 33;
pub const SERIAL_SP_HANDSHAKING: u32 = 16;
pub const SERIAL_SP_LAT: u32 = 257;
pub const SERIAL_SP_MODEM: u32 = 6;
pub const SERIAL_SP_PARALLEL: u32 = 2;
pub const SERIAL_SP_PARITY: u32 = 1;
pub const SERIAL_SP_PARITY_CHECK: u32 = 32;
pub const SERIAL_SP_RS232: u32 = 1;
pub const SERIAL_SP_RS422: u32 = 3;
pub const SERIAL_SP_RS423: u32 = 4;
pub const SERIAL_SP_RS449: u32 = 5;
pub const SERIAL_SP_SCANNER: u32 = 34;
pub const SERIAL_SP_SERIALCOMM: u32 = 1;
pub const SERIAL_SP_STOPBITS: u32 = 8;
pub const SERIAL_SP_TELNET: u32 = 258;
pub const SERIAL_SP_UNSPECIFIED: u32 = 0;
pub const SERIAL_SP_X25: u32 = 259;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_STATUS {
    pub Errors: u32,
    pub HoldReasons: u32,
    pub AmountInInQueue: u32,
    pub AmountInOutQueue: u32,
    pub EofReceived: bool,
    pub WaitForImmediate: bool,
}
pub const SERIAL_STOPBITS_10: u16 = 1;
pub const SERIAL_STOPBITS_15: u16 = 2;
pub const SERIAL_STOPBITS_20: u16 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_TIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
pub const SERIAL_TRANSMIT_TOGGLE: u32 = 192;
pub const SERIAL_TX_WAITING_FOR_CTS: u32 = 1;
pub const SERIAL_TX_WAITING_FOR_DCD: u32 = 4;
pub const SERIAL_TX_WAITING_FOR_DSR: u32 = 2;
pub const SERIAL_TX_WAITING_FOR_XON: u32 = 8;
pub const SERIAL_TX_WAITING_ON_BREAK: u32 = 32;
pub const SERIAL_TX_WAITING_XOFF_SENT: u32 = 16;
pub const SERIAL_XOFF_CONTINUE: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERIAL_XOFF_COUNTER {
    pub Timeout: u32,
    pub Counter: i32,
    pub XoffChar: u8,
}
pub const SPACE_PARITY: u32 = 4;
pub const STOP_BITS_1_5: u32 = 1;
pub const STOP_BITS_2: u32 = 2;
pub const STOP_BIT_1: u32 = 0;
pub const SerenumFirstHalf: SERENUM_PORTION = 0;
pub const SerenumSecondHalf: SERENUM_PORTION = 1;
pub const SerenumWhole: SERENUM_PORTION = 2;
