pub const EVEN_PARITY: i32 = 2;
pub const IOCTL_INTERNAL_SERENUM_REMOVE_SELF: i32 = 3604999;
pub const IOCTL_SERIAL_APPLY_DEFAULT_CONFIGURATION: i32 = 1769632;
pub const IOCTL_SERIAL_CLEAR_STATS: i32 = 1769616;
pub const IOCTL_SERIAL_CLR_DTR: i32 = 1769512;
pub const IOCTL_SERIAL_CLR_RTS: i32 = 1769524;
pub const IOCTL_SERIAL_CONFIG_SIZE: i32 = 1769600;
pub const IOCTL_SERIAL_GET_BAUD_RATE: i32 = 1769552;
pub const IOCTL_SERIAL_GET_CHARS: i32 = 1769560;
pub const IOCTL_SERIAL_GET_COMMCONFIG: i32 = 1769604;
pub const IOCTL_SERIAL_GET_COMMSTATUS: i32 = 1769580;
pub const IOCTL_SERIAL_GET_DTRRTS: i32 = 1769592;
pub const IOCTL_SERIAL_GET_HANDFLOW: i32 = 1769568;
pub const IOCTL_SERIAL_GET_LINE_CONTROL: i32 = 1769556;
pub const IOCTL_SERIAL_GET_MODEMSTATUS: i32 = 1769576;
pub const IOCTL_SERIAL_GET_MODEM_CONTROL: i32 = 1769620;
pub const IOCTL_SERIAL_GET_PROPERTIES: i32 = 1769588;
pub const IOCTL_SERIAL_GET_STATS: i32 = 1769612;
pub const IOCTL_SERIAL_GET_TIMEOUTS: i32 = 1769504;
pub const IOCTL_SERIAL_GET_WAIT_MASK: i32 = 1769536;
pub const IOCTL_SERIAL_IMMEDIATE_CHAR: i32 = 1769496;
pub const IOCTL_SERIAL_INTERNAL_BASIC_SETTINGS: i32 = 1769484;
pub const IOCTL_SERIAL_INTERNAL_CANCEL_WAIT_WAKE: i32 = 1769480;
pub const IOCTL_SERIAL_INTERNAL_DO_WAIT_WAKE: i32 = 1769476;
pub const IOCTL_SERIAL_INTERNAL_RESTORE_SETTINGS: i32 = 1769488;
pub const IOCTL_SERIAL_PURGE: i32 = 1769548;
pub const IOCTL_SERIAL_RESET_DEVICE: i32 = 1769516;
pub const IOCTL_SERIAL_SET_BAUD_RATE: i32 = 1769476;
pub const IOCTL_SERIAL_SET_BREAK_OFF: i32 = 1769492;
pub const IOCTL_SERIAL_SET_BREAK_ON: i32 = 1769488;
pub const IOCTL_SERIAL_SET_CHARS: i32 = 1769564;
pub const IOCTL_SERIAL_SET_COMMCONFIG: i32 = 1769608;
pub const IOCTL_SERIAL_SET_DTR: i32 = 1769508;
pub const IOCTL_SERIAL_SET_FIFO_CONTROL: i32 = 1769628;
pub const IOCTL_SERIAL_SET_HANDFLOW: i32 = 1769572;
pub const IOCTL_SERIAL_SET_INTERVAL_TIMER_RESOLUTION: i32 = 1769636;
pub const IOCTL_SERIAL_SET_LINE_CONTROL: i32 = 1769484;
pub const IOCTL_SERIAL_SET_MODEM_CONTROL: i32 = 1769624;
pub const IOCTL_SERIAL_SET_QUEUE_SIZE: i32 = 1769480;
pub const IOCTL_SERIAL_SET_RTS: i32 = 1769520;
pub const IOCTL_SERIAL_SET_TIMEOUTS: i32 = 1769500;
pub const IOCTL_SERIAL_SET_WAIT_MASK: i32 = 1769540;
pub const IOCTL_SERIAL_SET_XOFF: i32 = 1769528;
pub const IOCTL_SERIAL_SET_XON: i32 = 1769532;
pub const IOCTL_SERIAL_WAIT_ON_MASK: i32 = 1769544;
pub const IOCTL_SERIAL_XOFF_COUNTER: i32 = 1769584;
pub const MARK_PARITY: i32 = 3;
pub const NO_PARITY: i32 = 0;
pub const ODD_PARITY: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERENUM_PORT_DESC = *mut SERENUM_PORT_DESC;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERENUM_PORT_PARAMETERS = *mut SERENUM_PORT_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERENUM_READPORT = Option<unsafe extern "C" fn(serportaddress: super::PVOID) -> super::UCHAR>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERENUM_WRITEPORT = Option<unsafe extern "C" fn(serportaddress: super::PVOID, value: super::UCHAR)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERIALCONFIG = *mut SERIALCONFIG;
#[cfg(feature = "shared")]
pub type PSERIALPERF_STATS = *mut SERIALPERF_STATS;
#[cfg(feature = "shared")]
pub type PSERIAL_BASIC_SETTINGS = *mut SERIAL_BASIC_SETTINGS;
#[cfg(feature = "shared")]
pub type PSERIAL_BAUD_RATE = *mut SERIAL_BAUD_RATE;
#[cfg(feature = "shared")]
pub type PSERIAL_CHARS = *mut SERIAL_CHARS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERIAL_COMMPROP = *mut SERIAL_COMMPROP;
#[cfg(feature = "shared")]
pub type PSERIAL_HANDFLOW = *mut SERIAL_HANDFLOW;
#[cfg(feature = "shared")]
pub type PSERIAL_LINE_CONTROL = *mut SERIAL_LINE_CONTROL;
#[cfg(feature = "shared")]
pub type PSERIAL_QUEUE_SIZE = *mut SERIAL_QUEUE_SIZE;
#[cfg(feature = "shared")]
pub type PSERIAL_STATUS = *mut SERIAL_STATUS;
#[cfg(feature = "shared")]
pub type PSERIAL_TIMEOUTS = *mut SERIAL_TIMEOUTS;
#[cfg(feature = "shared")]
pub type PSERIAL_XOFF_COUNTER = *mut SERIAL_XOFF_COUNTER;
pub type SERENUM_PORTION = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SERENUM_PORT_DESC {
    pub Size: super::ULONG,
    pub PortHandle: super::PVOID,
    pub PortAddress: super::LARGE_INTEGER,
    pub Reserved: [super::USHORT; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SERENUM_PORT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct SERENUM_PORT_PARAMETERS {
    pub Size: super::ULONG,
    pub ReadAccessor: PSERENUM_READPORT,
    pub WriteAccessor: PSERENUM_WRITEPORT,
    pub SerPortAddress: super::PVOID,
    pub HardwareHandle: super::PVOID,
    pub Portion: SERENUM_PORTION,
    pub NumberAxis: super::USHORT,
    pub Reserved: [super::USHORT; 3],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SERENUM_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERIALCONFIG {
    pub Size: super::ULONG,
    pub Version: super::USHORT,
    pub SubType: super::ULONG,
    pub ProvOffset: super::ULONG,
    pub ProviderSize: super::ULONG,
    pub ProviderData: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SERIALCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIALPERF_STATS {
    pub ReceivedCount: super::ULONG,
    pub TransmittedCount: super::ULONG,
    pub FrameErrorCount: super::ULONG,
    pub SerialOverrunErrorCount: super::ULONG,
    pub BufferOverrunErrorCount: super::ULONG,
    pub ParityErrorCount: super::ULONG,
}
#[cfg(feature = "shared")]
pub const SERIAL_AUTO_RECEIVE: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_AUTO_TRANSMIT: super::ULONG = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_BASIC_SETTINGS {
    pub Timeouts: SERIAL_TIMEOUTS,
    pub HandFlow: SERIAL_HANDFLOW,
    pub RxFifo: super::ULONG,
    pub TxFifo: super::ULONG,
}
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_075: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_110: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_115200: super::ULONG = 131072;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_1200: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_128K: super::ULONG = 65536;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_134_5: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_14400: super::ULONG = 4096;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_150: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_1800: super::ULONG = 128;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_19200: super::ULONG = 8192;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_2400: super::ULONG = 256;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_300: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_38400: super::ULONG = 16384;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_4800: super::ULONG = 512;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_56K: super::ULONG = 32768;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_57600: super::ULONG = 262144;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_600: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_7200: super::ULONG = 1024;
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_9600: super::ULONG = 2048;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_BAUD_RATE {
    pub BaudRate: super::ULONG,
}
#[cfg(feature = "shared")]
pub const SERIAL_BAUD_USER: super::ULONG = 268435456;
#[cfg(feature = "shared")]
pub const SERIAL_BREAK_CHAR: super::ULONG = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_CHARS {
    pub EofChar: super::UCHAR,
    pub ErrorChar: super::UCHAR,
    pub BreakChar: super::UCHAR,
    pub EventChar: super::UCHAR,
    pub XonChar: super::UCHAR,
    pub XoffChar: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SERIAL_COMMPROP {
    pub PacketLength: super::USHORT,
    pub PacketVersion: super::USHORT,
    pub ServiceMask: super::ULONG,
    pub Reserved1: super::ULONG,
    pub MaxTxQueue: super::ULONG,
    pub MaxRxQueue: super::ULONG,
    pub MaxBaud: super::ULONG,
    pub ProvSubType: super::ULONG,
    pub ProvCapabilities: super::ULONG,
    pub SettableParams: super::ULONG,
    pub SettableBaud: super::ULONG,
    pub SettableData: super::USHORT,
    pub SettableStopParity: super::USHORT,
    pub CurrentTxQueue: super::ULONG,
    pub CurrentRxQueue: super::ULONG,
    pub ProvSpec1: super::ULONG,
    pub ProvSpec2: super::ULONG,
    pub ProvChar: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SERIAL_COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const SERIAL_CONTROL_INVALID: super::ULONG = 2147483524;
#[cfg(feature = "shared")]
pub const SERIAL_CTS_HANDSHAKE: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_CTS_STATE: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_16: super::USHORT = 16;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_16X: super::USHORT = 32;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_5: super::USHORT = 1;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_6: super::USHORT = 2;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_7: super::USHORT = 4;
#[cfg(feature = "shared")]
pub const SERIAL_DATABITS_8: super::USHORT = 8;
#[cfg(feature = "shared")]
pub const SERIAL_DCD_HANDSHAKE: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_DCD_STATE: super::ULONG = 128;
#[cfg(feature = "shared")]
pub const SERIAL_DSR_HANDSHAKE: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_DSR_SENSITIVITY: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_DSR_STATE: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_DTR_CONTROL: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_DTR_HANDSHAKE: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_DTR_MASK: super::ULONG = 3;
#[cfg(feature = "shared")]
pub const SERIAL_DTR_STATE: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_ABORT: super::ULONG = 2147483648;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_BREAK: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_CHAR: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_FRAMING: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_OVERRUN: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_PARITY: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_ERROR_QUEUEOVERRUN: super::ULONG = 8;
pub const SERIAL_EV_BREAK: i32 = 64;
pub const SERIAL_EV_CTS: i32 = 8;
pub const SERIAL_EV_DSR: i32 = 16;
pub const SERIAL_EV_ERR: i32 = 128;
pub const SERIAL_EV_EVENT1: i32 = 2048;
pub const SERIAL_EV_EVENT2: i32 = 4096;
pub const SERIAL_EV_PERR: i32 = 512;
pub const SERIAL_EV_RING: i32 = 256;
pub const SERIAL_EV_RLSD: i32 = 32;
pub const SERIAL_EV_RX80FULL: i32 = 1024;
pub const SERIAL_EV_RXCHAR: i32 = 1;
pub const SERIAL_EV_RXFLAG: i32 = 2;
pub const SERIAL_EV_TXEMPTY: i32 = 4;
#[cfg(feature = "shared")]
pub const SERIAL_FLOW_INVALID: super::ULONG = 2147483424;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_HANDFLOW {
    pub ControlHandShake: super::ULONG,
    pub FlowReplace: super::ULONG,
    pub XonLimit: super::LONG,
    pub XoffLimit: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_LINE_CONTROL {
    pub StopBits: super::UCHAR,
    pub Parity: super::UCHAR,
    pub WordLength: super::UCHAR,
}
#[cfg(feature = "shared")]
pub const SERIAL_NULL_STRIPPING: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_OUT_HANDSHAKEMASK: super::ULONG = 56;
#[cfg(feature = "shared")]
pub const SERIAL_PARITY_EVEN: super::USHORT = 1024;
#[cfg(feature = "shared")]
pub const SERIAL_PARITY_MARK: super::USHORT = 2048;
#[cfg(feature = "shared")]
pub const SERIAL_PARITY_NONE: super::USHORT = 256;
#[cfg(feature = "shared")]
pub const SERIAL_PARITY_ODD: super::USHORT = 512;
#[cfg(feature = "shared")]
pub const SERIAL_PARITY_SPACE: super::USHORT = 4096;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_16BITMODE: super::ULONG = 512;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_CD: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_DTRDSR: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_INTTIMEOUTS: super::ULONG = 128;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_PARITY_CHECK: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_RTSCTS: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_SETXCHAR: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_SPECIALCHARS: super::ULONG = 256;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_TOTALTIMEOUTS: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_PCF_XONXOFF: super::ULONG = 16;
pub const SERIAL_PURGE_RXABORT: i32 = 2;
pub const SERIAL_PURGE_RXCLEAR: i32 = 8;
pub const SERIAL_PURGE_TXABORT: i32 = 1;
pub const SERIAL_PURGE_TXCLEAR: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_QUEUE_SIZE {
    pub InSize: super::ULONG,
    pub OutSize: super::ULONG,
}
#[cfg(feature = "shared")]
pub const SERIAL_RI_STATE: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_RTS_CONTROL: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_RTS_HANDSHAKE: super::ULONG = 128;
#[cfg(feature = "shared")]
pub const SERIAL_RTS_MASK: super::ULONG = 192;
#[cfg(feature = "shared")]
pub const SERIAL_RTS_STATE: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_RX_WAITING_FOR_DSR: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_SP_BAUD: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_SP_BRIDGE: super::ULONG = 256;
#[cfg(feature = "shared")]
pub const SERIAL_SP_CARRIER_DETECT: super::ULONG = 64;
#[cfg(feature = "shared")]
pub const SERIAL_SP_DATABITS: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_SP_FAX: super::ULONG = 33;
#[cfg(feature = "shared")]
pub const SERIAL_SP_HANDSHAKING: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_SP_LAT: super::ULONG = 257;
#[cfg(feature = "shared")]
pub const SERIAL_SP_MODEM: super::ULONG = 6;
#[cfg(feature = "shared")]
pub const SERIAL_SP_PARALLEL: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_SP_PARITY: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_SP_PARITY_CHECK: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_SP_RS232: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_SP_RS422: super::ULONG = 3;
#[cfg(feature = "shared")]
pub const SERIAL_SP_RS423: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_SP_RS449: super::ULONG = 5;
#[cfg(feature = "shared")]
pub const SERIAL_SP_SCANNER: super::ULONG = 34;
#[cfg(feature = "shared")]
pub const SERIAL_SP_SERIALCOMM: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_SP_STOPBITS: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_SP_TELNET: super::ULONG = 258;
#[cfg(feature = "shared")]
pub const SERIAL_SP_UNSPECIFIED: super::ULONG = 0;
#[cfg(feature = "shared")]
pub const SERIAL_SP_X25: super::ULONG = 259;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_STATUS {
    pub Errors: super::ULONG,
    pub HoldReasons: super::ULONG,
    pub AmountInInQueue: super::ULONG,
    pub AmountInOutQueue: super::ULONG,
    pub EofReceived: super::BOOLEAN,
    pub WaitForImmediate: super::BOOLEAN,
}
#[cfg(feature = "shared")]
pub const SERIAL_STOPBITS_10: super::USHORT = 1;
#[cfg(feature = "shared")]
pub const SERIAL_STOPBITS_15: super::USHORT = 2;
#[cfg(feature = "shared")]
pub const SERIAL_STOPBITS_20: super::USHORT = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_TIMEOUTS {
    pub ReadIntervalTimeout: super::ULONG,
    pub ReadTotalTimeoutMultiplier: super::ULONG,
    pub ReadTotalTimeoutConstant: super::ULONG,
    pub WriteTotalTimeoutMultiplier: super::ULONG,
    pub WriteTotalTimeoutConstant: super::ULONG,
}
#[cfg(feature = "shared")]
pub const SERIAL_TRANSMIT_TOGGLE: super::ULONG = 192;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_FOR_CTS: super::ULONG = 1;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_FOR_DCD: super::ULONG = 4;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_FOR_DSR: super::ULONG = 2;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_FOR_XON: super::ULONG = 8;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_ON_BREAK: super::ULONG = 32;
#[cfg(feature = "shared")]
pub const SERIAL_TX_WAITING_XOFF_SENT: super::ULONG = 16;
#[cfg(feature = "shared")]
pub const SERIAL_XOFF_CONTINUE: super::ULONG = 2147483648;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERIAL_XOFF_COUNTER {
    pub Timeout: super::ULONG,
    pub Counter: super::LONG,
    pub XoffChar: super::UCHAR,
}
pub const SPACE_PARITY: i32 = 4;
pub const STOP_BITS_1_5: i32 = 1;
pub const STOP_BITS_2: i32 = 2;
pub const STOP_BIT_1: i32 = 0;
pub const SerenumFirstHalf: SERENUM_PORTION = 0;
pub const SerenumSecondHalf: SERENUM_PORTION = 1;
pub const SerenumWhole: SERENUM_PORTION = 2;
