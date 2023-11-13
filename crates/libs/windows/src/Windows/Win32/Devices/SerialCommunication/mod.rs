#[inline]
pub unsafe fn ComDBClaimNextFreePort<P0>(hcomdb: P0, comnumber: *mut u32) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBClaimNextFreePort(hcomdb : HCOMDB, comnumber : *mut u32) -> i32);
    ComDBClaimNextFreePort(hcomdb.into_param().abi(), comnumber)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ComDBClaimPort<P0, P1>(hcomdb: P0, comnumber: u32, forceclaim: P1, forced: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBClaimPort(hcomdb : HCOMDB, comnumber : u32, forceclaim : super::super::Foundation:: BOOL, forced : *mut super::super::Foundation:: BOOL) -> i32);
    ComDBClaimPort(hcomdb.into_param().abi(), comnumber, forceclaim.into_param().abi(), ::core::mem::transmute(forced.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ComDBClose<P0>(hcomdb: P0) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBClose(hcomdb : HCOMDB) -> i32);
    ComDBClose(hcomdb.into_param().abi())
}
#[inline]
pub unsafe fn ComDBGetCurrentPortUsage<P0>(hcomdb: P0, buffer: ::core::option::Option<&mut [u8]>, reporttype: u32, maxportsreported: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBGetCurrentPortUsage(hcomdb : HCOMDB, buffer : *mut u8, buffersize : u32, reporttype : u32, maxportsreported : *mut u32) -> i32);
    ComDBGetCurrentPortUsage(hcomdb.into_param().abi(), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), reporttype, ::core::mem::transmute(maxportsreported.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn ComDBOpen(phcomdb: *mut HCOMDB) -> i32 {
    ::windows_targets::link!("msports.dll" "system" fn ComDBOpen(phcomdb : *mut HCOMDB) -> i32);
    ComDBOpen(phcomdb)
}
#[inline]
pub unsafe fn ComDBReleasePort<P0>(hcomdb: P0, comnumber: u32) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBReleasePort(hcomdb : HCOMDB, comnumber : u32) -> i32);
    ComDBReleasePort(hcomdb.into_param().abi(), comnumber)
}
#[inline]
pub unsafe fn ComDBResizeDatabase<P0>(hcomdb: P0, newsize: u32) -> i32
where
    P0: ::windows_core::IntoParam<HCOMDB>,
{
    ::windows_targets::link!("msports.dll" "system" fn ComDBResizeDatabase(hcomdb : HCOMDB, newsize : u32) -> i32);
    ComDBResizeDatabase(hcomdb.into_param().abi(), newsize)
}
pub const CDB_REPORT_BITS: u32 = 0u32;
pub const CDB_REPORT_BYTES: u32 = 1u32;
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_Serial_PortName: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 4 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_Serial_UsbProductId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 3 };
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_DeviceInterface_Serial_UsbVendorId: super::Properties::DEVPROPKEY = super::Properties::DEVPROPKEY { fmtid: ::windows_core::GUID::from_u128(0x4c6bf15c_4c03_4aac_91f5_64c0f852bcf4), pid: 2 };
pub const EVEN_PARITY: u32 = 2u32;
pub const IOCTL_INTERNAL_SERENUM_REMOVE_SELF: u32 = 3604999u32;
pub const IOCTL_SERIAL_APPLY_DEFAULT_CONFIGURATION: u32 = 1769632u32;
pub const IOCTL_SERIAL_CLEAR_STATS: u32 = 1769616u32;
pub const IOCTL_SERIAL_CLR_DTR: u32 = 1769512u32;
pub const IOCTL_SERIAL_CLR_RTS: u32 = 1769524u32;
pub const IOCTL_SERIAL_CONFIG_SIZE: u32 = 1769600u32;
pub const IOCTL_SERIAL_GET_BAUD_RATE: u32 = 1769552u32;
pub const IOCTL_SERIAL_GET_CHARS: u32 = 1769560u32;
pub const IOCTL_SERIAL_GET_COMMCONFIG: u32 = 1769604u32;
pub const IOCTL_SERIAL_GET_COMMSTATUS: u32 = 1769580u32;
pub const IOCTL_SERIAL_GET_DTRRTS: u32 = 1769592u32;
pub const IOCTL_SERIAL_GET_HANDFLOW: u32 = 1769568u32;
pub const IOCTL_SERIAL_GET_LINE_CONTROL: u32 = 1769556u32;
pub const IOCTL_SERIAL_GET_MODEMSTATUS: u32 = 1769576u32;
pub const IOCTL_SERIAL_GET_MODEM_CONTROL: u32 = 1769620u32;
pub const IOCTL_SERIAL_GET_PROPERTIES: u32 = 1769588u32;
pub const IOCTL_SERIAL_GET_STATS: u32 = 1769612u32;
pub const IOCTL_SERIAL_GET_TIMEOUTS: u32 = 1769504u32;
pub const IOCTL_SERIAL_GET_WAIT_MASK: u32 = 1769536u32;
pub const IOCTL_SERIAL_IMMEDIATE_CHAR: u32 = 1769496u32;
pub const IOCTL_SERIAL_INTERNAL_BASIC_SETTINGS: u32 = 1769484u32;
pub const IOCTL_SERIAL_INTERNAL_CANCEL_WAIT_WAKE: u32 = 1769480u32;
pub const IOCTL_SERIAL_INTERNAL_DO_WAIT_WAKE: u32 = 1769476u32;
pub const IOCTL_SERIAL_INTERNAL_RESTORE_SETTINGS: u32 = 1769488u32;
pub const IOCTL_SERIAL_PURGE: u32 = 1769548u32;
pub const IOCTL_SERIAL_RESET_DEVICE: u32 = 1769516u32;
pub const IOCTL_SERIAL_SET_BAUD_RATE: u32 = 1769476u32;
pub const IOCTL_SERIAL_SET_BREAK_OFF: u32 = 1769492u32;
pub const IOCTL_SERIAL_SET_BREAK_ON: u32 = 1769488u32;
pub const IOCTL_SERIAL_SET_CHARS: u32 = 1769564u32;
pub const IOCTL_SERIAL_SET_COMMCONFIG: u32 = 1769608u32;
pub const IOCTL_SERIAL_SET_DTR: u32 = 1769508u32;
pub const IOCTL_SERIAL_SET_FIFO_CONTROL: u32 = 1769628u32;
pub const IOCTL_SERIAL_SET_HANDFLOW: u32 = 1769572u32;
pub const IOCTL_SERIAL_SET_INTERVAL_TIMER_RESOLUTION: u32 = 1769636u32;
pub const IOCTL_SERIAL_SET_LINE_CONTROL: u32 = 1769484u32;
pub const IOCTL_SERIAL_SET_MODEM_CONTROL: u32 = 1769624u32;
pub const IOCTL_SERIAL_SET_QUEUE_SIZE: u32 = 1769480u32;
pub const IOCTL_SERIAL_SET_RTS: u32 = 1769520u32;
pub const IOCTL_SERIAL_SET_TIMEOUTS: u32 = 1769500u32;
pub const IOCTL_SERIAL_SET_WAIT_MASK: u32 = 1769540u32;
pub const IOCTL_SERIAL_SET_XOFF: u32 = 1769528u32;
pub const IOCTL_SERIAL_SET_XON: u32 = 1769532u32;
pub const IOCTL_SERIAL_WAIT_ON_MASK: u32 = 1769544u32;
pub const IOCTL_SERIAL_XOFF_COUNTER: u32 = 1769584u32;
pub const MARK_PARITY: u32 = 3u32;
pub const NO_PARITY: u32 = 0u32;
pub const ODD_PARITY: u32 = 1u32;
pub const SERIAL_EV_BREAK: u32 = 64u32;
pub const SERIAL_EV_CTS: u32 = 8u32;
pub const SERIAL_EV_DSR: u32 = 16u32;
pub const SERIAL_EV_ERR: u32 = 128u32;
pub const SERIAL_EV_EVENT1: u32 = 2048u32;
pub const SERIAL_EV_EVENT2: u32 = 4096u32;
pub const SERIAL_EV_PERR: u32 = 512u32;
pub const SERIAL_EV_RING: u32 = 256u32;
pub const SERIAL_EV_RLSD: u32 = 32u32;
pub const SERIAL_EV_RX80FULL: u32 = 1024u32;
pub const SERIAL_EV_RXCHAR: u32 = 1u32;
pub const SERIAL_EV_RXFLAG: u32 = 2u32;
pub const SERIAL_EV_TXEMPTY: u32 = 4u32;
pub const SERIAL_LSRMST_ESCAPE: u16 = 0u16;
pub const SERIAL_LSRMST_LSR_DATA: u16 = 1u16;
pub const SERIAL_LSRMST_LSR_NODATA: u16 = 2u16;
pub const SERIAL_LSRMST_MST: u16 = 3u16;
pub const SERIAL_PURGE_RXABORT: u32 = 2u32;
pub const SERIAL_PURGE_RXCLEAR: u32 = 8u32;
pub const SERIAL_PURGE_TXABORT: u32 = 1u32;
pub const SERIAL_PURGE_TXCLEAR: u32 = 4u32;
pub const SPACE_PARITY: u32 = 4u32;
pub const STOP_BITS_1_5: u32 = 1u32;
pub const STOP_BITS_2: u32 = 2u32;
pub const STOP_BIT_1: u32 = 0u32;
pub const SerenumFirstHalf: SERENUM_PORTION = SERENUM_PORTION(0i32);
pub const SerenumSecondHalf: SERENUM_PORTION = SERENUM_PORTION(1i32);
pub const SerenumWhole: SERENUM_PORTION = SERENUM_PORTION(2i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERENUM_PORTION(pub i32);
impl ::core::marker::Copy for SERENUM_PORTION {}
impl ::core::clone::Clone for SERENUM_PORTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERENUM_PORTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SERENUM_PORTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SERENUM_PORTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERENUM_PORTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCOMDB(pub isize);
impl HCOMDB {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCOMDB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCOMDB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCOMDB {}
impl ::core::fmt::Debug for HCOMDB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCOMDB").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for HCOMDB {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct SERENUM_PORT_DESC {
    pub Size: u32,
    pub PortHandle: *mut ::core::ffi::c_void,
    pub PortAddress: i64,
    pub Reserved: [u16; 1],
}
impl ::core::marker::Copy for SERENUM_PORT_DESC {}
impl ::core::clone::Clone for SERENUM_PORT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERENUM_PORT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERENUM_PORT_DESC").field("Size", &self.Size).field("PortHandle", &self.PortHandle).field("PortAddress", &self.PortAddress).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for SERENUM_PORT_DESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERENUM_PORT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.PortHandle == other.PortHandle && self.PortAddress == other.PortAddress && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SERENUM_PORT_DESC {}
impl ::core::default::Default for SERENUM_PORT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERENUM_PORT_PARAMETERS {
    pub Size: u32,
    pub ReadAccessor: PSERENUM_READPORT,
    pub WriteAccessor: PSERENUM_WRITEPORT,
    pub SerPortAddress: *mut ::core::ffi::c_void,
    pub HardwareHandle: *mut ::core::ffi::c_void,
    pub Portion: SERENUM_PORTION,
    pub NumberAxis: u16,
    pub Reserved: [u16; 3],
}
impl ::core::marker::Copy for SERENUM_PORT_PARAMETERS {}
impl ::core::clone::Clone for SERENUM_PORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERENUM_PORT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERENUM_PORT_PARAMETERS").field("Size", &self.Size).field("SerPortAddress", &self.SerPortAddress).field("HardwareHandle", &self.HardwareHandle).field("Portion", &self.Portion).field("NumberAxis", &self.NumberAxis).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for SERENUM_PORT_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for SERENUM_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIALCONFIG {
    pub Size: u32,
    pub Version: u16,
    pub SubType: u32,
    pub ProvOffset: u32,
    pub ProviderSize: u32,
    pub ProviderData: [u16; 1],
}
impl ::core::marker::Copy for SERIALCONFIG {}
impl ::core::clone::Clone for SERIALCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIALCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALCONFIG").field("Size", &self.Size).field("Version", &self.Version).field("SubType", &self.SubType).field("ProvOffset", &self.ProvOffset).field("ProviderSize", &self.ProviderSize).field("ProviderData", &self.ProviderData).finish()
    }
}
impl ::windows_core::TypeKind for SERIALCONFIG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIALCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SubType == other.SubType && self.ProvOffset == other.ProvOffset && self.ProviderSize == other.ProviderSize && self.ProviderData == other.ProviderData
    }
}
impl ::core::cmp::Eq for SERIALCONFIG {}
impl ::core::default::Default for SERIALCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIALPERF_STATS {
    pub ReceivedCount: u32,
    pub TransmittedCount: u32,
    pub FrameErrorCount: u32,
    pub SerialOverrunErrorCount: u32,
    pub BufferOverrunErrorCount: u32,
    pub ParityErrorCount: u32,
}
impl ::core::marker::Copy for SERIALPERF_STATS {}
impl ::core::clone::Clone for SERIALPERF_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIALPERF_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIALPERF_STATS").field("ReceivedCount", &self.ReceivedCount).field("TransmittedCount", &self.TransmittedCount).field("FrameErrorCount", &self.FrameErrorCount).field("SerialOverrunErrorCount", &self.SerialOverrunErrorCount).field("BufferOverrunErrorCount", &self.BufferOverrunErrorCount).field("ParityErrorCount", &self.ParityErrorCount).finish()
    }
}
impl ::windows_core::TypeKind for SERIALPERF_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIALPERF_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.ReceivedCount == other.ReceivedCount && self.TransmittedCount == other.TransmittedCount && self.FrameErrorCount == other.FrameErrorCount && self.SerialOverrunErrorCount == other.SerialOverrunErrorCount && self.BufferOverrunErrorCount == other.BufferOverrunErrorCount && self.ParityErrorCount == other.ParityErrorCount
    }
}
impl ::core::cmp::Eq for SERIALPERF_STATS {}
impl ::core::default::Default for SERIALPERF_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_BASIC_SETTINGS {
    pub Timeouts: SERIAL_TIMEOUTS,
    pub HandFlow: SERIAL_HANDFLOW,
    pub RxFifo: u32,
    pub TxFifo: u32,
}
impl ::core::marker::Copy for SERIAL_BASIC_SETTINGS {}
impl ::core::clone::Clone for SERIAL_BASIC_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_BASIC_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_BASIC_SETTINGS").field("Timeouts", &self.Timeouts).field("HandFlow", &self.HandFlow).field("RxFifo", &self.RxFifo).field("TxFifo", &self.TxFifo).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_BASIC_SETTINGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_BASIC_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.Timeouts == other.Timeouts && self.HandFlow == other.HandFlow && self.RxFifo == other.RxFifo && self.TxFifo == other.TxFifo
    }
}
impl ::core::cmp::Eq for SERIAL_BASIC_SETTINGS {}
impl ::core::default::Default for SERIAL_BASIC_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_BAUD_RATE {
    pub BaudRate: u32,
}
impl ::core::marker::Copy for SERIAL_BAUD_RATE {}
impl ::core::clone::Clone for SERIAL_BAUD_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_BAUD_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_BAUD_RATE").field("BaudRate", &self.BaudRate).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_BAUD_RATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_BAUD_RATE {
    fn eq(&self, other: &Self) -> bool {
        self.BaudRate == other.BaudRate
    }
}
impl ::core::cmp::Eq for SERIAL_BAUD_RATE {}
impl ::core::default::Default for SERIAL_BAUD_RATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_CHARS {
    pub EofChar: u8,
    pub ErrorChar: u8,
    pub BreakChar: u8,
    pub EventChar: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
}
impl ::core::marker::Copy for SERIAL_CHARS {}
impl ::core::clone::Clone for SERIAL_CHARS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_CHARS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_CHARS").field("EofChar", &self.EofChar).field("ErrorChar", &self.ErrorChar).field("BreakChar", &self.BreakChar).field("EventChar", &self.EventChar).field("XonChar", &self.XonChar).field("XoffChar", &self.XoffChar).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_CHARS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_CHARS {
    fn eq(&self, other: &Self) -> bool {
        self.EofChar == other.EofChar && self.ErrorChar == other.ErrorChar && self.BreakChar == other.BreakChar && self.EventChar == other.EventChar && self.XonChar == other.XonChar && self.XoffChar == other.XoffChar
    }
}
impl ::core::cmp::Eq for SERIAL_CHARS {}
impl ::core::default::Default for SERIAL_CHARS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::marker::Copy for SERIAL_COMMPROP {}
impl ::core::clone::Clone for SERIAL_COMMPROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_COMMPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_COMMPROP")
            .field("PacketLength", &self.PacketLength)
            .field("PacketVersion", &self.PacketVersion)
            .field("ServiceMask", &self.ServiceMask)
            .field("Reserved1", &self.Reserved1)
            .field("MaxTxQueue", &self.MaxTxQueue)
            .field("MaxRxQueue", &self.MaxRxQueue)
            .field("MaxBaud", &self.MaxBaud)
            .field("ProvSubType", &self.ProvSubType)
            .field("ProvCapabilities", &self.ProvCapabilities)
            .field("SettableParams", &self.SettableParams)
            .field("SettableBaud", &self.SettableBaud)
            .field("SettableData", &self.SettableData)
            .field("SettableStopParity", &self.SettableStopParity)
            .field("CurrentTxQueue", &self.CurrentTxQueue)
            .field("CurrentRxQueue", &self.CurrentRxQueue)
            .field("ProvSpec1", &self.ProvSpec1)
            .field("ProvSpec2", &self.ProvSpec2)
            .field("ProvChar", &self.ProvChar)
            .finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_COMMPROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_COMMPROP {
    fn eq(&self, other: &Self) -> bool {
        self.PacketLength == other.PacketLength && self.PacketVersion == other.PacketVersion && self.ServiceMask == other.ServiceMask && self.Reserved1 == other.Reserved1 && self.MaxTxQueue == other.MaxTxQueue && self.MaxRxQueue == other.MaxRxQueue && self.MaxBaud == other.MaxBaud && self.ProvSubType == other.ProvSubType && self.ProvCapabilities == other.ProvCapabilities && self.SettableParams == other.SettableParams && self.SettableBaud == other.SettableBaud && self.SettableData == other.SettableData && self.SettableStopParity == other.SettableStopParity && self.CurrentTxQueue == other.CurrentTxQueue && self.CurrentRxQueue == other.CurrentRxQueue && self.ProvSpec1 == other.ProvSpec1 && self.ProvSpec2 == other.ProvSpec2 && self.ProvChar == other.ProvChar
    }
}
impl ::core::cmp::Eq for SERIAL_COMMPROP {}
impl ::core::default::Default for SERIAL_COMMPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_HANDFLOW {
    pub ControlHandShake: u32,
    pub FlowReplace: u32,
    pub XonLimit: i32,
    pub XoffLimit: i32,
}
impl ::core::marker::Copy for SERIAL_HANDFLOW {}
impl ::core::clone::Clone for SERIAL_HANDFLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_HANDFLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_HANDFLOW").field("ControlHandShake", &self.ControlHandShake).field("FlowReplace", &self.FlowReplace).field("XonLimit", &self.XonLimit).field("XoffLimit", &self.XoffLimit).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_HANDFLOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_HANDFLOW {
    fn eq(&self, other: &Self) -> bool {
        self.ControlHandShake == other.ControlHandShake && self.FlowReplace == other.FlowReplace && self.XonLimit == other.XonLimit && self.XoffLimit == other.XoffLimit
    }
}
impl ::core::cmp::Eq for SERIAL_HANDFLOW {}
impl ::core::default::Default for SERIAL_HANDFLOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_LINE_CONTROL {
    pub StopBits: u8,
    pub Parity: u8,
    pub WordLength: u8,
}
impl ::core::marker::Copy for SERIAL_LINE_CONTROL {}
impl ::core::clone::Clone for SERIAL_LINE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_LINE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_LINE_CONTROL").field("StopBits", &self.StopBits).field("Parity", &self.Parity).field("WordLength", &self.WordLength).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_LINE_CONTROL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_LINE_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.StopBits == other.StopBits && self.Parity == other.Parity && self.WordLength == other.WordLength
    }
}
impl ::core::cmp::Eq for SERIAL_LINE_CONTROL {}
impl ::core::default::Default for SERIAL_LINE_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_QUEUE_SIZE {
    pub InSize: u32,
    pub OutSize: u32,
}
impl ::core::marker::Copy for SERIAL_QUEUE_SIZE {}
impl ::core::clone::Clone for SERIAL_QUEUE_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_QUEUE_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_QUEUE_SIZE").field("InSize", &self.InSize).field("OutSize", &self.OutSize).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_QUEUE_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_QUEUE_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.InSize == other.InSize && self.OutSize == other.OutSize
    }
}
impl ::core::cmp::Eq for SERIAL_QUEUE_SIZE {}
impl ::core::default::Default for SERIAL_QUEUE_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERIAL_STATUS {
    pub Errors: u32,
    pub HoldReasons: u32,
    pub AmountInInQueue: u32,
    pub AmountInOutQueue: u32,
    pub EofReceived: super::super::Foundation::BOOLEAN,
    pub WaitForImmediate: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERIAL_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERIAL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERIAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_STATUS").field("Errors", &self.Errors).field("HoldReasons", &self.HoldReasons).field("AmountInInQueue", &self.AmountInInQueue).field("AmountInOutQueue", &self.AmountInOutQueue).field("EofReceived", &self.EofReceived).field("WaitForImmediate", &self.WaitForImmediate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for SERIAL_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERIAL_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Errors == other.Errors && self.HoldReasons == other.HoldReasons && self.AmountInInQueue == other.AmountInInQueue && self.AmountInOutQueue == other.AmountInOutQueue && self.EofReceived == other.EofReceived && self.WaitForImmediate == other.WaitForImmediate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERIAL_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERIAL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_TIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
impl ::core::marker::Copy for SERIAL_TIMEOUTS {}
impl ::core::clone::Clone for SERIAL_TIMEOUTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_TIMEOUTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_TIMEOUTS").field("ReadIntervalTimeout", &self.ReadIntervalTimeout).field("ReadTotalTimeoutMultiplier", &self.ReadTotalTimeoutMultiplier).field("ReadTotalTimeoutConstant", &self.ReadTotalTimeoutConstant).field("WriteTotalTimeoutMultiplier", &self.WriteTotalTimeoutMultiplier).field("WriteTotalTimeoutConstant", &self.WriteTotalTimeoutConstant).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_TIMEOUTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_TIMEOUTS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadIntervalTimeout == other.ReadIntervalTimeout && self.ReadTotalTimeoutMultiplier == other.ReadTotalTimeoutMultiplier && self.ReadTotalTimeoutConstant == other.ReadTotalTimeoutConstant && self.WriteTotalTimeoutMultiplier == other.WriteTotalTimeoutMultiplier && self.WriteTotalTimeoutConstant == other.WriteTotalTimeoutConstant
    }
}
impl ::core::cmp::Eq for SERIAL_TIMEOUTS {}
impl ::core::default::Default for SERIAL_TIMEOUTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SERIAL_XOFF_COUNTER {
    pub Timeout: u32,
    pub Counter: i32,
    pub XoffChar: u8,
}
impl ::core::marker::Copy for SERIAL_XOFF_COUNTER {}
impl ::core::clone::Clone for SERIAL_XOFF_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERIAL_XOFF_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERIAL_XOFF_COUNTER").field("Timeout", &self.Timeout).field("Counter", &self.Counter).field("XoffChar", &self.XoffChar).finish()
    }
}
impl ::windows_core::TypeKind for SERIAL_XOFF_COUNTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SERIAL_XOFF_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.Timeout == other.Timeout && self.Counter == other.Counter && self.XoffChar == other.XoffChar
    }
}
impl ::core::cmp::Eq for SERIAL_XOFF_COUNTER {}
impl ::core::default::Default for SERIAL_XOFF_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type PSERENUM_READPORT = ::core::option::Option<unsafe extern "system" fn(serportaddress: *const ::core::ffi::c_void) -> u8>;
pub type PSERENUM_WRITEPORT = ::core::option::Option<unsafe extern "system" fn(serportaddress: *const ::core::ffi::c_void, value: u8) -> ()>;
