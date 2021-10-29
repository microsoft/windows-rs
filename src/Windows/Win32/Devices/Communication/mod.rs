#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpdef: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BuildCommDCBA(lpdef.into_param().abi(), ::std::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpdef: Param0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBAndTimeoutsA(lpdef: super::super::Foundation::PSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BuildCommDCBAndTimeoutsA(lpdef.into_param().abi(), ::std::mem::transmute(lpdcb), ::std::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpdef: Param0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBAndTimeoutsW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BuildCommDCBAndTimeoutsW(lpdef.into_param().abi(), ::std::mem::transmute(lpdcb), ::std::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpdef: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBW(lpdef: super::super::Foundation::PWSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BuildCommDCBW(lpdef.into_param().abi(), ::std::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLEAR_COMM_ERROR_FLAGS(pub u32);
pub const CE_BREAK: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(16u32);
pub const CE_FRAME: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(8u32);
pub const CE_OVERRUN: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(2u32);
pub const CE_RXOVER: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(1u32);
pub const CE_RXPARITY: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(4u32);
impl ::std::convert::From<u32> for CLEAR_COMM_ERROR_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLEAR_COMM_ERROR_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMMCONFIG {
    pub dwSize: u32,
    pub wVersion: u16,
    pub wReserved: u16,
    pub dcb: DCB,
    pub dwProviderSubType: u32,
    pub dwProviderOffset: u32,
    pub dwProviderSize: u32,
    pub wcProviderData: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl COMMCONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMMCONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMMCONFIG")
            .field("dwSize", &self.dwSize)
            .field("wVersion", &self.wVersion)
            .field("wReserved", &self.wReserved)
            .field("dcb", &self.dcb)
            .field("dwProviderSubType", &self.dwProviderSubType)
            .field("dwProviderOffset", &self.dwProviderOffset)
            .field("dwProviderSize", &self.dwProviderSize)
            .field("wcProviderData", &self.wcProviderData)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMMCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.wVersion == other.wVersion && self.wReserved == other.wReserved && self.dcb == other.dcb && self.dwProviderSubType == other.dwProviderSubType && self.dwProviderOffset == other.dwProviderOffset && self.dwProviderSize == other.dwProviderSize && self.wcProviderData == other.wcProviderData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMMCONFIG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMMCONFIG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMMPROP {
    pub wPacketLength: u16,
    pub wPacketVersion: u16,
    pub dwServiceMask: u32,
    pub dwReserved1: u32,
    pub dwMaxTxQueue: u32,
    pub dwMaxRxQueue: u32,
    pub dwMaxBaud: u32,
    pub dwProvSubType: u32,
    pub dwProvCapabilities: u32,
    pub dwSettableParams: u32,
    pub dwSettableBaud: u32,
    pub wSettableData: u16,
    pub wSettableStopParity: COMMPROP_STOP_PARITY,
    pub dwCurrentTxQueue: u32,
    pub dwCurrentRxQueue: u32,
    pub dwProvSpec1: u32,
    pub dwProvSpec2: u32,
    pub wcProvChar: [u16; 1],
}
impl COMMPROP {}
impl ::std::default::Default for COMMPROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMMPROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMMPROP")
            .field("wPacketLength", &self.wPacketLength)
            .field("wPacketVersion", &self.wPacketVersion)
            .field("dwServiceMask", &self.dwServiceMask)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwMaxTxQueue", &self.dwMaxTxQueue)
            .field("dwMaxRxQueue", &self.dwMaxRxQueue)
            .field("dwMaxBaud", &self.dwMaxBaud)
            .field("dwProvSubType", &self.dwProvSubType)
            .field("dwProvCapabilities", &self.dwProvCapabilities)
            .field("dwSettableParams", &self.dwSettableParams)
            .field("dwSettableBaud", &self.dwSettableBaud)
            .field("wSettableData", &self.wSettableData)
            .field("wSettableStopParity", &self.wSettableStopParity)
            .field("dwCurrentTxQueue", &self.dwCurrentTxQueue)
            .field("dwCurrentRxQueue", &self.dwCurrentRxQueue)
            .field("dwProvSpec1", &self.dwProvSpec1)
            .field("dwProvSpec2", &self.dwProvSpec2)
            .field("wcProvChar", &self.wcProvChar)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMMPROP {
    fn eq(&self, other: &Self) -> bool {
        self.wPacketLength == other.wPacketLength
            && self.wPacketVersion == other.wPacketVersion
            && self.dwServiceMask == other.dwServiceMask
            && self.dwReserved1 == other.dwReserved1
            && self.dwMaxTxQueue == other.dwMaxTxQueue
            && self.dwMaxRxQueue == other.dwMaxRxQueue
            && self.dwMaxBaud == other.dwMaxBaud
            && self.dwProvSubType == other.dwProvSubType
            && self.dwProvCapabilities == other.dwProvCapabilities
            && self.dwSettableParams == other.dwSettableParams
            && self.dwSettableBaud == other.dwSettableBaud
            && self.wSettableData == other.wSettableData
            && self.wSettableStopParity == other.wSettableStopParity
            && self.dwCurrentTxQueue == other.dwCurrentTxQueue
            && self.dwCurrentRxQueue == other.dwCurrentRxQueue
            && self.dwProvSpec1 == other.dwProvSpec1
            && self.dwProvSpec2 == other.dwProvSpec2
            && self.wcProvChar == other.wcProvChar
    }
}
impl ::std::cmp::Eq for COMMPROP {}
unsafe impl ::windows::runtime::Abi for COMMPROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMMPROP_STOP_PARITY(pub u16);
pub const STOPBITS_10: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1u16);
pub const STOPBITS_15: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2u16);
pub const STOPBITS_20: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4u16);
pub const PARITY_NONE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(256u16);
pub const PARITY_ODD: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(512u16);
pub const PARITY_EVEN: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1024u16);
pub const PARITY_MARK: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2048u16);
pub const PARITY_SPACE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4096u16);
impl ::std::convert::From<u16> for COMMPROP_STOP_PARITY {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMMPROP_STOP_PARITY {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMMPROP_STOP_PARITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMMPROP_STOP_PARITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
impl COMMTIMEOUTS {}
impl ::std::default::Default for COMMTIMEOUTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMMTIMEOUTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMMTIMEOUTS")
            .field("ReadIntervalTimeout", &self.ReadIntervalTimeout)
            .field("ReadTotalTimeoutMultiplier", &self.ReadTotalTimeoutMultiplier)
            .field("ReadTotalTimeoutConstant", &self.ReadTotalTimeoutConstant)
            .field("WriteTotalTimeoutMultiplier", &self.WriteTotalTimeoutMultiplier)
            .field("WriteTotalTimeoutConstant", &self.WriteTotalTimeoutConstant)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMMTIMEOUTS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadIntervalTimeout == other.ReadIntervalTimeout && self.ReadTotalTimeoutMultiplier == other.ReadTotalTimeoutMultiplier && self.ReadTotalTimeoutConstant == other.ReadTotalTimeoutConstant && self.WriteTotalTimeoutMultiplier == other.WriteTotalTimeoutMultiplier && self.WriteTotalTimeoutConstant == other.WriteTotalTimeoutConstant
    }
}
impl ::std::cmp::Eq for COMMTIMEOUTS {}
unsafe impl ::windows::runtime::Abi for COMMTIMEOUTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMM_EVENT_MASK(pub u32);
pub const EV_BREAK: COMM_EVENT_MASK = COMM_EVENT_MASK(64u32);
pub const EV_CTS: COMM_EVENT_MASK = COMM_EVENT_MASK(8u32);
pub const EV_DSR: COMM_EVENT_MASK = COMM_EVENT_MASK(16u32);
pub const EV_ERR: COMM_EVENT_MASK = COMM_EVENT_MASK(128u32);
pub const EV_EVENT1: COMM_EVENT_MASK = COMM_EVENT_MASK(2048u32);
pub const EV_EVENT2: COMM_EVENT_MASK = COMM_EVENT_MASK(4096u32);
pub const EV_PERR: COMM_EVENT_MASK = COMM_EVENT_MASK(512u32);
pub const EV_RING: COMM_EVENT_MASK = COMM_EVENT_MASK(256u32);
pub const EV_RLSD: COMM_EVENT_MASK = COMM_EVENT_MASK(32u32);
pub const EV_RX80FULL: COMM_EVENT_MASK = COMM_EVENT_MASK(1024u32);
pub const EV_RXCHAR: COMM_EVENT_MASK = COMM_EVENT_MASK(1u32);
pub const EV_RXFLAG: COMM_EVENT_MASK = COMM_EVENT_MASK(2u32);
pub const EV_TXEMPTY: COMM_EVENT_MASK = COMM_EVENT_MASK(4u32);
impl ::std::convert::From<u32> for COMM_EVENT_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMM_EVENT_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for COMM_EVENT_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMM_EVENT_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMM_EVENT_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMM_EVENT_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMM_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMSTAT {
    pub _bitfield: u32,
    pub cbInQue: u32,
    pub cbOutQue: u32,
}
impl COMSTAT {}
impl ::std::default::Default for COMSTAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMSTAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMSTAT").field("_bitfield", &self._bitfield).field("cbInQue", &self.cbInQue).field("cbOutQue", &self.cbOutQue).finish()
    }
}
impl ::std::cmp::PartialEq for COMSTAT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cbInQue == other.cbInQue && self.cbOutQue == other.cbOutQue
    }
}
impl ::std::cmp::Eq for COMSTAT {}
unsafe impl ::windows::runtime::Abi for COMSTAT {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearCommBreak<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ClearCommBreak(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearCommError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearCommError(hfile: super::super::Foundation::HANDLE, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ClearCommError(hfile.into_param().abi(), ::std::mem::transmute(lperrors), ::std::mem::transmute(lpstat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommConfigDialogA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(lpszname: Param0, hwnd: Param1, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommConfigDialogA(lpszname: super::super::Foundation::PSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommConfigDialogA(lpszname.into_param().abi(), hwnd.into_param().abi(), ::std::mem::transmute(lpcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommConfigDialogW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(lpszname: Param0, hwnd: Param1, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommConfigDialogW(lpszname: super::super::Foundation::PWSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CommConfigDialogW(lpszname.into_param().abi(), hwnd.into_param().abi(), ::std::mem::transmute(lpcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DCB {
    pub DCBlength: u32,
    pub BaudRate: u32,
    pub _bitfield: u32,
    pub wReserved: u16,
    pub XonLim: u16,
    pub XoffLim: u16,
    pub ByteSize: u8,
    pub Parity: u8,
    pub StopBits: u8,
    pub XonChar: super::super::Foundation::CHAR,
    pub XoffChar: super::super::Foundation::CHAR,
    pub ErrorChar: super::super::Foundation::CHAR,
    pub EofChar: super::super::Foundation::CHAR,
    pub EvtChar: super::super::Foundation::CHAR,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl DCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DCB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DCB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCB")
            .field("DCBlength", &self.DCBlength)
            .field("BaudRate", &self.BaudRate)
            .field("_bitfield", &self._bitfield)
            .field("wReserved", &self.wReserved)
            .field("XonLim", &self.XonLim)
            .field("XoffLim", &self.XoffLim)
            .field("ByteSize", &self.ByteSize)
            .field("Parity", &self.Parity)
            .field("StopBits", &self.StopBits)
            .field("XonChar", &self.XonChar)
            .field("XoffChar", &self.XoffChar)
            .field("ErrorChar", &self.ErrorChar)
            .field("EofChar", &self.EofChar)
            .field("EvtChar", &self.EvtChar)
            .field("wReserved1", &self.wReserved1)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DCB {
    fn eq(&self, other: &Self) -> bool {
        self.DCBlength == other.DCBlength
            && self.BaudRate == other.BaudRate
            && self._bitfield == other._bitfield
            && self.wReserved == other.wReserved
            && self.XonLim == other.XonLim
            && self.XoffLim == other.XoffLim
            && self.ByteSize == other.ByteSize
            && self.Parity == other.Parity
            && self.StopBits == other.StopBits
            && self.XonChar == other.XonChar
            && self.XoffChar == other.XoffChar
            && self.ErrorChar == other.ErrorChar
            && self.EofChar == other.EofChar
            && self.EvtChar == other.EvtChar
            && self.wReserved1 == other.wReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DCB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DCB {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ESCAPE_COMM_FUNCTION(pub u32);
pub const CLRBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(9u32);
pub const CLRDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(6u32);
pub const CLRRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(4u32);
pub const SETBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(8u32);
pub const SETDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(5u32);
pub const SETRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(3u32);
pub const SETXOFF: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(1u32);
pub const SETXON: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(2u32);
impl ::std::convert::From<u32> for ESCAPE_COMM_FUNCTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ESCAPE_COMM_FUNCTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ESCAPE_COMM_FUNCTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ESCAPE_COMM_FUNCTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ESCAPE_COMM_FUNCTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ESCAPE_COMM_FUNCTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ESCAPE_COMM_FUNCTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EscapeCommFunction<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EscapeCommFunction(hfile: super::super::Foundation::HANDLE, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EscapeCommFunction(hfile.into_param().abi(), ::std::mem::transmute(dwfunc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommConfig<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hcommdev: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommConfig(hcommdev.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommMask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommMask(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommMask(hfile.into_param().abi(), ::std::mem::transmute(lpevtmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommModemStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommModemStatus(hfile: super::super::Foundation::HANDLE, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommModemStatus(hfile.into_param().abi(), ::std::mem::transmute(lpmodemstat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetCommPorts(lpportnumbers: *mut u32, uportnumberscount: u32, puportnumbersfound: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommPorts(lpportnumbers: *mut u32, uportnumberscount: u32, puportnumbersfound: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetCommPorts(::std::mem::transmute(lpportnumbers), ::std::mem::transmute(uportnumberscount), ::std::mem::transmute(puportnumbersfound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommProperties(hfile: super::super::Foundation::HANDLE, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommProperties(hfile.into_param().abi(), ::std::mem::transmute(lpcommprop)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommState(hfile.into_param().abi(), ::std::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommTimeouts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCommTimeouts(hfile.into_param().abi(), ::std::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultCommConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszname: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDefaultCommConfigA(lpszname.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultCommConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszname: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetDefaultCommConfigW(lpszname.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAXLENGTH_NAI: u32 = 72u32;
pub const MAXLENGTH_UICCDATASTORE: u32 = 10u32;
pub const MDM_ANALOG_RLP_OFF: u32 = 1u32;
pub const MDM_ANALOG_RLP_ON: u32 = 0u32;
pub const MDM_ANALOG_V34: u32 = 2u32;
pub const MDM_AUTO_ML_2: u32 = 2u32;
pub const MDM_AUTO_ML_DEFAULT: u32 = 0u32;
pub const MDM_AUTO_ML_NONE: u32 = 1u32;
pub const MDM_AUTO_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_BEARERMODE_ANALOG: u32 = 0u32;
pub const MDM_BEARERMODE_GSM: u32 = 2u32;
pub const MDM_BEARERMODE_ISDN: u32 = 1u32;
pub const MDM_BLIND_DIAL: u32 = 512u32;
pub const MDM_CCITT_OVERRIDE: u32 = 64u32;
pub const MDM_CELLULAR: u32 = 8u32;
pub const MDM_COMPRESSION: u32 = 1u32;
pub const MDM_DIAGNOSTICS: u32 = 2048u32;
pub const MDM_ERROR_CONTROL: u32 = 2u32;
pub const MDM_FLOWCONTROL_HARD: u32 = 16u32;
pub const MDM_FLOWCONTROL_SOFT: u32 = 32u32;
pub const MDM_FORCED_EC: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_CHAP: u32 = 3u32;
pub const MDM_HDLCPPP_AUTH_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_AUTH_MSCHAP: u32 = 4u32;
pub const MDM_HDLCPPP_AUTH_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_AUTH_PAP: u32 = 2u32;
pub const MDM_HDLCPPP_ML_2: u32 = 2u32;
pub const MDM_HDLCPPP_ML_DEFAULT: u32 = 0u32;
pub const MDM_HDLCPPP_ML_NONE: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_56K: u32 = 2u32;
pub const MDM_HDLCPPP_SPEED_64K: u32 = 1u32;
pub const MDM_HDLCPPP_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_MASK_AUTO_SPEED: u32 = 7u32;
pub const MDM_MASK_BEARERMODE: u32 = 61440u32;
pub const MDM_MASK_HDLCPPP_SPEED: u32 = 7u32;
pub const MDM_MASK_PROTOCOLDATA: u32 = 267386880u32;
pub const MDM_MASK_PROTOCOLID: u32 = 983040u32;
pub const MDM_MASK_V110_SPEED: u32 = 15u32;
pub const MDM_MASK_V120_SPEED: u32 = 7u32;
pub const MDM_MASK_X75_DATA: u32 = 7u32;
pub const MDM_PIAFS_INCOMING: u32 = 0u32;
pub const MDM_PIAFS_OUTGOING: u32 = 1u32;
pub const MDM_PROTOCOLID_ANALOG: u32 = 7u32;
pub const MDM_PROTOCOLID_AUTO: u32 = 6u32;
pub const MDM_PROTOCOLID_DEFAULT: u32 = 0u32;
pub const MDM_PROTOCOLID_GPRS: u32 = 8u32;
pub const MDM_PROTOCOLID_HDLCPPP: u32 = 1u32;
pub const MDM_PROTOCOLID_PIAFS: u32 = 9u32;
pub const MDM_PROTOCOLID_V110: u32 = 4u32;
pub const MDM_PROTOCOLID_V120: u32 = 5u32;
pub const MDM_PROTOCOLID_V128: u32 = 2u32;
pub const MDM_PROTOCOLID_X75: u32 = 3u32;
pub const MDM_SHIFT_AUTO_ML: u32 = 6u32;
pub const MDM_SHIFT_AUTO_SPEED: u32 = 0u32;
pub const MDM_SHIFT_BEARERMODE: u32 = 12u32;
pub const MDM_SHIFT_EXTENDEDINFO: u32 = 12u32;
pub const MDM_SHIFT_HDLCPPP_AUTH: u32 = 3u32;
pub const MDM_SHIFT_HDLCPPP_ML: u32 = 6u32;
pub const MDM_SHIFT_HDLCPPP_SPEED: u32 = 0u32;
pub const MDM_SHIFT_PROTOCOLDATA: u32 = 20u32;
pub const MDM_SHIFT_PROTOCOLID: u32 = 16u32;
pub const MDM_SHIFT_PROTOCOLINFO: u32 = 16u32;
pub const MDM_SHIFT_V110_SPEED: u32 = 0u32;
pub const MDM_SHIFT_V120_ML: u32 = 6u32;
pub const MDM_SHIFT_V120_SPEED: u32 = 0u32;
pub const MDM_SHIFT_X75_DATA: u32 = 0u32;
pub const MDM_SPEED_ADJUST: u32 = 128u32;
pub const MDM_TONE_DIAL: u32 = 256u32;
pub const MDM_V110_SPEED_12DOT0K: u32 = 5u32;
pub const MDM_V110_SPEED_14DOT4K: u32 = 6u32;
pub const MDM_V110_SPEED_19DOT2K: u32 = 7u32;
pub const MDM_V110_SPEED_1DOT2K: u32 = 1u32;
pub const MDM_V110_SPEED_28DOT8K: u32 = 8u32;
pub const MDM_V110_SPEED_2DOT4K: u32 = 2u32;
pub const MDM_V110_SPEED_38DOT4K: u32 = 9u32;
pub const MDM_V110_SPEED_4DOT8K: u32 = 3u32;
pub const MDM_V110_SPEED_57DOT6K: u32 = 10u32;
pub const MDM_V110_SPEED_9DOT6K: u32 = 4u32;
pub const MDM_V110_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_2: u32 = 2u32;
pub const MDM_V120_ML_DEFAULT: u32 = 0u32;
pub const MDM_V120_ML_NONE: u32 = 1u32;
pub const MDM_V120_SPEED_56K: u32 = 2u32;
pub const MDM_V120_SPEED_64K: u32 = 1u32;
pub const MDM_V120_SPEED_DEFAULT: u32 = 0u32;
pub const MDM_V23_OVERRIDE: u32 = 1024u32;
pub const MDM_X75_DATA_128K: u32 = 2u32;
pub const MDM_X75_DATA_64K: u32 = 1u32;
pub const MDM_X75_DATA_BTX: u32 = 4u32;
pub const MDM_X75_DATA_DEFAULT: u32 = 0u32;
pub const MDM_X75_DATA_T_70: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MODEMDEVCAPS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwModemProviderVersion: u32,
    pub dwModemManufacturerOffset: u32,
    pub dwModemManufacturerSize: u32,
    pub dwModemModelOffset: u32,
    pub dwModemModelSize: u32,
    pub dwModemVersionOffset: u32,
    pub dwModemVersionSize: u32,
    pub dwDialOptions: MODEMDEVCAPS_DIAL_OPTIONS,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: MODEMDEVCAPS_SPEAKER_VOLUME,
    pub dwSpeakerMode: MODEMDEVCAPS_SPEAKER_MODE,
    pub dwModemOptions: u32,
    pub dwMaxDTERate: u32,
    pub dwMaxDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl MODEMDEVCAPS {}
impl ::std::default::Default for MODEMDEVCAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MODEMDEVCAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODEMDEVCAPS")
            .field("dwActualSize", &self.dwActualSize)
            .field("dwRequiredSize", &self.dwRequiredSize)
            .field("dwDevSpecificOffset", &self.dwDevSpecificOffset)
            .field("dwDevSpecificSize", &self.dwDevSpecificSize)
            .field("dwModemProviderVersion", &self.dwModemProviderVersion)
            .field("dwModemManufacturerOffset", &self.dwModemManufacturerOffset)
            .field("dwModemManufacturerSize", &self.dwModemManufacturerSize)
            .field("dwModemModelOffset", &self.dwModemModelOffset)
            .field("dwModemModelSize", &self.dwModemModelSize)
            .field("dwModemVersionOffset", &self.dwModemVersionOffset)
            .field("dwModemVersionSize", &self.dwModemVersionSize)
            .field("dwDialOptions", &self.dwDialOptions)
            .field("dwCallSetupFailTimer", &self.dwCallSetupFailTimer)
            .field("dwInactivityTimeout", &self.dwInactivityTimeout)
            .field("dwSpeakerVolume", &self.dwSpeakerVolume)
            .field("dwSpeakerMode", &self.dwSpeakerMode)
            .field("dwModemOptions", &self.dwModemOptions)
            .field("dwMaxDTERate", &self.dwMaxDTERate)
            .field("dwMaxDCERate", &self.dwMaxDCERate)
            .field("abVariablePortion", &self.abVariablePortion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MODEMDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwActualSize == other.dwActualSize
            && self.dwRequiredSize == other.dwRequiredSize
            && self.dwDevSpecificOffset == other.dwDevSpecificOffset
            && self.dwDevSpecificSize == other.dwDevSpecificSize
            && self.dwModemProviderVersion == other.dwModemProviderVersion
            && self.dwModemManufacturerOffset == other.dwModemManufacturerOffset
            && self.dwModemManufacturerSize == other.dwModemManufacturerSize
            && self.dwModemModelOffset == other.dwModemModelOffset
            && self.dwModemModelSize == other.dwModemModelSize
            && self.dwModemVersionOffset == other.dwModemVersionOffset
            && self.dwModemVersionSize == other.dwModemVersionSize
            && self.dwDialOptions == other.dwDialOptions
            && self.dwCallSetupFailTimer == other.dwCallSetupFailTimer
            && self.dwInactivityTimeout == other.dwInactivityTimeout
            && self.dwSpeakerVolume == other.dwSpeakerVolume
            && self.dwSpeakerMode == other.dwSpeakerMode
            && self.dwModemOptions == other.dwModemOptions
            && self.dwMaxDTERate == other.dwMaxDTERate
            && self.dwMaxDCERate == other.dwMaxDCERate
            && self.abVariablePortion == other.abVariablePortion
    }
}
impl ::std::cmp::Eq for MODEMDEVCAPS {}
unsafe impl ::windows::runtime::Abi for MODEMDEVCAPS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEMDEVCAPS_DIAL_OPTIONS(pub u32);
pub const DIALOPTION_BILLING: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(64u32);
pub const DIALOPTION_DIALTONE: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(256u32);
pub const DIALOPTION_QUIET: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(128u32);
impl ::std::convert::From<u32> for MODEMDEVCAPS_DIAL_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEMDEVCAPS_DIAL_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEMDEVCAPS_SPEAKER_MODE(pub u32);
pub const MDMSPKRFLAG_CALLSETUP: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(8u32);
pub const MDMSPKRFLAG_DIAL: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(2u32);
pub const MDMSPKRFLAG_OFF: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(1u32);
pub const MDMSPKRFLAG_ON: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(4u32);
impl ::std::convert::From<u32> for MODEMDEVCAPS_SPEAKER_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEMDEVCAPS_SPEAKER_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEMDEVCAPS_SPEAKER_VOLUME(pub u32);
pub const MDMVOLFLAG_HIGH: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(4u32);
pub const MDMVOLFLAG_LOW: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(1u32);
pub const MDMVOLFLAG_MEDIUM: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(2u32);
impl ::std::convert::From<u32> for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MODEMSETTINGS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: MODEM_SPEAKER_VOLUME,
    pub dwSpeakerMode: MODEMSETTINGS_SPEAKER_MODE,
    pub dwPreferredModemOptions: u32,
    pub dwNegotiatedModemOptions: u32,
    pub dwNegotiatedDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl MODEMSETTINGS {}
impl ::std::default::Default for MODEMSETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MODEMSETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODEMSETTINGS")
            .field("dwActualSize", &self.dwActualSize)
            .field("dwRequiredSize", &self.dwRequiredSize)
            .field("dwDevSpecificOffset", &self.dwDevSpecificOffset)
            .field("dwDevSpecificSize", &self.dwDevSpecificSize)
            .field("dwCallSetupFailTimer", &self.dwCallSetupFailTimer)
            .field("dwInactivityTimeout", &self.dwInactivityTimeout)
            .field("dwSpeakerVolume", &self.dwSpeakerVolume)
            .field("dwSpeakerMode", &self.dwSpeakerMode)
            .field("dwPreferredModemOptions", &self.dwPreferredModemOptions)
            .field("dwNegotiatedModemOptions", &self.dwNegotiatedModemOptions)
            .field("dwNegotiatedDCERate", &self.dwNegotiatedDCERate)
            .field("abVariablePortion", &self.abVariablePortion)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MODEMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwActualSize == other.dwActualSize
            && self.dwRequiredSize == other.dwRequiredSize
            && self.dwDevSpecificOffset == other.dwDevSpecificOffset
            && self.dwDevSpecificSize == other.dwDevSpecificSize
            && self.dwCallSetupFailTimer == other.dwCallSetupFailTimer
            && self.dwInactivityTimeout == other.dwInactivityTimeout
            && self.dwSpeakerVolume == other.dwSpeakerVolume
            && self.dwSpeakerMode == other.dwSpeakerMode
            && self.dwPreferredModemOptions == other.dwPreferredModemOptions
            && self.dwNegotiatedModemOptions == other.dwNegotiatedModemOptions
            && self.dwNegotiatedDCERate == other.dwNegotiatedDCERate
            && self.abVariablePortion == other.abVariablePortion
    }
}
impl ::std::cmp::Eq for MODEMSETTINGS {}
unsafe impl ::windows::runtime::Abi for MODEMSETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEMSETTINGS_SPEAKER_MODE(pub u32);
pub const MDMSPKR_CALLSETUP: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(8u32);
pub const MDMSPKR_DIAL: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(2u32);
pub const MDMSPKR_OFF: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(1u32);
pub const MDMSPKR_ON: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(4u32);
impl ::std::convert::From<u32> for MODEMSETTINGS_SPEAKER_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEMSETTINGS_SPEAKER_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEMSETTINGS_SPEAKER_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEMSETTINGS_SPEAKER_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEMSETTINGS_SPEAKER_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEMSETTINGS_SPEAKER_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEMSETTINGS_SPEAKER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEM_SPEAKER_VOLUME(pub u32);
pub const MDMVOL_HIGH: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(2u32);
pub const MDMVOL_LOW: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(0u32);
pub const MDMVOL_MEDIUM: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(1u32);
impl ::std::convert::From<u32> for MODEM_SPEAKER_VOLUME {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEM_SPEAKER_VOLUME {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEM_SPEAKER_VOLUME {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEM_SPEAKER_VOLUME {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEM_SPEAKER_VOLUME {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEM_SPEAKER_VOLUME {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEM_SPEAKER_VOLUME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MODEM_STATUS_FLAGS(pub u32);
pub const MS_CTS_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(16u32);
pub const MS_DSR_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(32u32);
pub const MS_RING_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(64u32);
pub const MS_RLSD_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(128u32);
impl ::std::convert::From<u32> for MODEM_STATUS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODEM_STATUS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODEM_STATUS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODEM_STATUS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenCommPort(::std::mem::transmute(uportnumber), ::std::mem::transmute(dwdesiredaccess), ::std::mem::transmute(dwflagsandattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PURGE_COMM_FLAGS(pub u32);
pub const PURGE_RXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(2u32);
pub const PURGE_RXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(8u32);
pub const PURGE_TXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(1u32);
pub const PURGE_TXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(4u32);
impl ::std::convert::From<u32> for PURGE_COMM_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PURGE_COMM_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PURGE_COMM_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PURGE_COMM_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PURGE_COMM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PurgeComm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PurgeComm(hfile: super::super::Foundation::HANDLE, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PurgeComm(hfile.into_param().abi(), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SID_3GPP_SUPSVCMODEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3620769287, 55143, 17528, [177, 74, 238, 204, 135, 234, 18, 247]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommBreak<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCommBreak(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommConfig<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hcommdev: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCommConfig(hcommdev.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommMask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommMask(hfile: super::super::Foundation::HANDLE, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCommMask(hfile.into_param().abi(), ::std::mem::transmute(dwevtmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpdcb: *const DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *const DCB) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCommState(hfile.into_param().abi(), ::std::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommTimeouts<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetCommTimeouts(hfile.into_param().abi(), ::std::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultCommConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszname: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultCommConfigA(lpszname: super::super::Foundation::PSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDefaultCommConfigA(lpszname.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultCommConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszname: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultCommConfigW(lpszname: super::super::Foundation::PWSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDefaultCommConfigW(lpszname.into_param().abi(), ::std::mem::transmute(lpcc), ::std::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupComm<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupComm(hfile: super::super::Foundation::HANDLE, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetupComm(hfile.into_param().abi(), ::std::mem::transmute(dwinqueue), ::std::mem::transmute(dwoutqueue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TransmitCommChar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::CHAR>>(hfile: Param0, cchar: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TransmitCommChar(hfile: super::super::Foundation::HANDLE, cchar: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TransmitCommChar(hfile.into_param().abi(), cchar.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WaitCommEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitCommEvent(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WaitCommEvent(hfile.into_param().abi(), ::std::mem::transmute(lpevtmask), ::std::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
