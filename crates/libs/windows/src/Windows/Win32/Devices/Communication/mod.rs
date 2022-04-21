#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpdef: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBA(lpdef: ::windows::core::PCSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BuildCommDCBA(lpdef.into_param().abi(), ::core::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpdef: Param0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBAndTimeoutsA(lpdef: ::windows::core::PCSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BuildCommDCBAndTimeoutsA(lpdef.into_param().abi(), ::core::mem::transmute(lpdcb), ::core::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpdef: Param0, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBAndTimeoutsW(lpdef: ::windows::core::PCWSTR, lpdcb: *mut DCB, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BuildCommDCBAndTimeoutsW(lpdef.into_param().abi(), ::core::mem::transmute(lpdcb), ::core::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BuildCommDCBW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpdef: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BuildCommDCBW(lpdef: ::windows::core::PCWSTR, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BuildCommDCBW(lpdef.into_param().abi(), ::core::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLEAR_COMM_ERROR_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CE_BREAK: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CE_FRAME: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CE_OVERRUN: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CE_RXOVER: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CE_RXPARITY: CLEAR_COMM_ERROR_FLAGS = CLEAR_COMM_ERROR_FLAGS(4u32);
impl ::core::marker::Copy for CLEAR_COMM_ERROR_FLAGS {}
impl ::core::clone::Clone for CLEAR_COMM_ERROR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLEAR_COMM_ERROR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLEAR_COMM_ERROR_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLEAR_COMM_ERROR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLEAR_COMM_ERROR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for COMMCONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMMCONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMMCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMCONFIG").field("dwSize", &self.dwSize).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).field("dcb", &self.dcb).field("dwProviderSubType", &self.dwProviderSubType).field("dwProviderOffset", &self.dwProviderOffset).field("dwProviderSize", &self.dwProviderSize).field("wcProviderData", &self.wcProviderData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMMCONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMMCONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMMCONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMMCONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
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
impl ::core::marker::Copy for COMMPROP {}
impl ::core::clone::Clone for COMMPROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMMPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMPROP")
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
unsafe impl ::windows::core::Abi for COMMPROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMMPROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMMPROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMMPROP {}
impl ::core::default::Default for COMMPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMMPROP_STOP_PARITY(pub u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const STOPBITS_10: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const STOPBITS_15: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const STOPBITS_20: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PARITY_NONE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(256u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PARITY_ODD: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(512u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PARITY_EVEN: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(1024u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PARITY_MARK: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(2048u16);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PARITY_SPACE: COMMPROP_STOP_PARITY = COMMPROP_STOP_PARITY(4096u16);
impl ::core::marker::Copy for COMMPROP_STOP_PARITY {}
impl ::core::clone::Clone for COMMPROP_STOP_PARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMMPROP_STOP_PARITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMMPROP_STOP_PARITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMMPROP_STOP_PARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMPROP_STOP_PARITY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMMPROP_STOP_PARITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMMPROP_STOP_PARITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
impl ::core::marker::Copy for COMMTIMEOUTS {}
impl ::core::clone::Clone for COMMTIMEOUTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMMTIMEOUTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMTIMEOUTS").field("ReadIntervalTimeout", &self.ReadIntervalTimeout).field("ReadTotalTimeoutMultiplier", &self.ReadTotalTimeoutMultiplier).field("ReadTotalTimeoutConstant", &self.ReadTotalTimeoutConstant).field("WriteTotalTimeoutMultiplier", &self.WriteTotalTimeoutMultiplier).field("WriteTotalTimeoutConstant", &self.WriteTotalTimeoutConstant).finish()
    }
}
unsafe impl ::windows::core::Abi for COMMTIMEOUTS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMMTIMEOUTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMMTIMEOUTS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMMTIMEOUTS {}
impl ::core::default::Default for COMMTIMEOUTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMM_EVENT_MASK(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_BREAK: COMM_EVENT_MASK = COMM_EVENT_MASK(64u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_CTS: COMM_EVENT_MASK = COMM_EVENT_MASK(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_DSR: COMM_EVENT_MASK = COMM_EVENT_MASK(16u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_ERR: COMM_EVENT_MASK = COMM_EVENT_MASK(128u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_EVENT1: COMM_EVENT_MASK = COMM_EVENT_MASK(2048u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_EVENT2: COMM_EVENT_MASK = COMM_EVENT_MASK(4096u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_PERR: COMM_EVENT_MASK = COMM_EVENT_MASK(512u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_RING: COMM_EVENT_MASK = COMM_EVENT_MASK(256u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_RLSD: COMM_EVENT_MASK = COMM_EVENT_MASK(32u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_RX80FULL: COMM_EVENT_MASK = COMM_EVENT_MASK(1024u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_RXCHAR: COMM_EVENT_MASK = COMM_EVENT_MASK(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_RXFLAG: COMM_EVENT_MASK = COMM_EVENT_MASK(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EV_TXEMPTY: COMM_EVENT_MASK = COMM_EVENT_MASK(4u32);
impl ::core::marker::Copy for COMM_EVENT_MASK {}
impl ::core::clone::Clone for COMM_EVENT_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMM_EVENT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMM_EVENT_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMM_EVENT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMM_EVENT_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMM_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMM_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMM_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMM_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMM_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub struct COMSTAT {
    pub _bitfield: u32,
    pub cbInQue: u32,
    pub cbOutQue: u32,
}
impl ::core::marker::Copy for COMSTAT {}
impl ::core::clone::Clone for COMSTAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMSTAT").field("_bitfield", &self._bitfield).field("cbInQue", &self.cbInQue).field("cbOutQue", &self.cbOutQue).finish()
    }
}
unsafe impl ::windows::core::Abi for COMSTAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMSTAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMSTAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMSTAT {}
impl ::core::default::Default for COMSTAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearCommBreak<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearCommBreak(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClearCommError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClearCommError(hfile: super::super::Foundation::HANDLE, lperrors: *mut CLEAR_COMM_ERROR_FLAGS, lpstat: *mut COMSTAT) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ClearCommError(hfile.into_param().abi(), ::core::mem::transmute(lperrors), ::core::mem::transmute(lpstat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommConfigDialogA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszname: Param0, hwnd: Param1, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommConfigDialogA(lpszname: ::windows::core::PCSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CommConfigDialogA(lpszname.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(lpcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CommConfigDialogW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(lpszname: Param0, hwnd: Param1, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CommConfigDialogW(lpszname: ::windows::core::PCWSTR, hwnd: super::super::Foundation::HWND, lpcc: *mut COMMCONFIG) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CommConfigDialogW(lpszname.into_param().abi(), hwnd.into_param().abi(), ::core::mem::transmute(lpcc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DCB {
    pub DCBlength: u32,
    pub BaudRate: u32,
    pub _bitfield: u32,
    pub wReserved: u16,
    pub XonLim: u16,
    pub XoffLim: u16,
    pub ByteSize: u8,
    pub Parity: DCB_PARITY,
    pub StopBits: DCB_STOP_BITS,
    pub XonChar: super::super::Foundation::CHAR,
    pub XoffChar: super::super::Foundation::CHAR,
    pub ErrorChar: super::super::Foundation::CHAR,
    pub EofChar: super::super::Foundation::CHAR,
    pub EvtChar: super::super::Foundation::CHAR,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DCB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCB")
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
unsafe impl ::windows::core::Abi for DCB {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DCB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCB>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCB_PARITY(pub u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const EVENPARITY: DCB_PARITY = DCB_PARITY(2u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MARKPARITY: DCB_PARITY = DCB_PARITY(3u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const NOPARITY: DCB_PARITY = DCB_PARITY(0u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const ODDPARITY: DCB_PARITY = DCB_PARITY(1u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SPACEPARITY: DCB_PARITY = DCB_PARITY(4u8);
impl ::core::marker::Copy for DCB_PARITY {}
impl ::core::clone::Clone for DCB_PARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCB_PARITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCB_PARITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCB_PARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCB_PARITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCB_STOP_BITS(pub u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const ONESTOPBIT: DCB_STOP_BITS = DCB_STOP_BITS(0u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const ONE5STOPBITS: DCB_STOP_BITS = DCB_STOP_BITS(1u8);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const TWOSTOPBITS: DCB_STOP_BITS = DCB_STOP_BITS(2u8);
impl ::core::marker::Copy for DCB_STOP_BITS {}
impl ::core::clone::Clone for DCB_STOP_BITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCB_STOP_BITS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCB_STOP_BITS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCB_STOP_BITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCB_STOP_BITS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ESCAPE_COMM_FUNCTION(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CLRBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(9u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CLRDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(6u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const CLRRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(4u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SETBREAK: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SETDTR: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(5u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SETRTS: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(3u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SETXOFF: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const SETXON: ESCAPE_COMM_FUNCTION = ESCAPE_COMM_FUNCTION(2u32);
impl ::core::marker::Copy for ESCAPE_COMM_FUNCTION {}
impl ::core::clone::Clone for ESCAPE_COMM_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ESCAPE_COMM_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ESCAPE_COMM_FUNCTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for ESCAPE_COMM_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESCAPE_COMM_FUNCTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EscapeCommFunction<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EscapeCommFunction(hfile: super::super::Foundation::HANDLE, dwfunc: ESCAPE_COMM_FUNCTION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EscapeCommFunction(hfile.into_param().abi(), ::core::mem::transmute(dwfunc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hcommdev: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommConfig(hcommdev.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommMask(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommMask(hfile.into_param().abi(), ::core::mem::transmute(lpevtmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommModemStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommModemStatus(hfile: super::super::Foundation::HANDLE, lpmodemstat: *mut MODEM_STATUS_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommModemStatus(hfile.into_param().abi(), ::core::mem::transmute(lpmodemstat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[inline]
pub unsafe fn GetCommPorts(lpportnumbers: &mut [u32], puportnumbersfound: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommPorts(lpportnumbers: *mut u32, uportnumberscount: u32, puportnumbersfound: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetCommPorts(::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpportnumbers)), lpportnumbers.len() as _, ::core::mem::transmute(puportnumbersfound)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommProperties(hfile: super::super::Foundation::HANDLE, lpcommprop: *mut COMMPROP) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommProperties(hfile.into_param().abi(), ::core::mem::transmute(lpcommprop)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpdcb: *mut DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *mut DCB) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommState(hfile.into_param().abi(), ::core::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCommTimeouts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *mut COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetCommTimeouts(hfile.into_param().abi(), ::core::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultCommConfigA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszname: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDefaultCommConfigA(lpszname: ::windows::core::PCSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetDefaultCommConfigA(lpszname.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetDefaultCommConfigW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszname: Param0, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDefaultCommConfigW(lpszname: ::windows::core::PCWSTR, lpcc: *mut COMMCONFIG, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetDefaultCommConfigW(lpszname.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MAXLENGTH_NAI: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MAXLENGTH_UICCDATASTORE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_ANALOG_RLP_OFF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_ANALOG_RLP_ON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_ANALOG_V34: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_AUTO_ML_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_AUTO_ML_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_AUTO_ML_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_AUTO_SPEED_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_BEARERMODE_ANALOG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_BEARERMODE_GSM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_BEARERMODE_ISDN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_BLIND_DIAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_CCITT_OVERRIDE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_CELLULAR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_COMPRESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_DIAGNOSTICS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_ERROR_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_FLOWCONTROL_HARD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_FLOWCONTROL_SOFT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_FORCED_EC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_AUTH_CHAP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_AUTH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_AUTH_MSCHAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_AUTH_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_AUTH_PAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_ML_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_ML_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_ML_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_SPEED_56K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_SPEED_64K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_HDLCPPP_SPEED_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_AUTO_SPEED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_BEARERMODE: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_HDLCPPP_SPEED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_PROTOCOLDATA: u32 = 267386880u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_PROTOCOLID: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_V110_SPEED: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_V120_SPEED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_MASK_X75_DATA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PIAFS_INCOMING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PIAFS_OUTGOING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_ANALOG: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_AUTO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_GPRS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_HDLCPPP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_PIAFS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_V110: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_V120: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_V128: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_PROTOCOLID_X75: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_AUTO_ML: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_AUTO_SPEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_BEARERMODE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_EXTENDEDINFO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_HDLCPPP_AUTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_HDLCPPP_ML: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_HDLCPPP_SPEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_PROTOCOLDATA: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_PROTOCOLID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_PROTOCOLINFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_V110_SPEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_V120_ML: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_V120_SPEED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SHIFT_X75_DATA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_SPEED_ADJUST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_TONE_DIAL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_12DOT0K: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_14DOT4K: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_19DOT2K: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_1DOT2K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_28DOT8K: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_2DOT4K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_38DOT4K: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_4DOT8K: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_57DOT6K: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_9DOT6K: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V110_SPEED_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_ML_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_ML_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_ML_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_SPEED_56K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_SPEED_64K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V120_SPEED_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_V23_OVERRIDE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_X75_DATA_128K: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_X75_DATA_64K: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_X75_DATA_BTX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_X75_DATA_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDM_X75_DATA_T_70: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
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
impl ::core::marker::Copy for MODEMDEVCAPS {}
impl ::core::clone::Clone for MODEMDEVCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODEMDEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODEMDEVCAPS")
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
unsafe impl ::windows::core::Abi for MODEMDEVCAPS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MODEMDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MODEMDEVCAPS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MODEMDEVCAPS {}
impl ::core::default::Default for MODEMDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEMDEVCAPS_DIAL_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const DIALOPTION_BILLING: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(64u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const DIALOPTION_DIALTONE: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(256u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const DIALOPTION_QUIET: MODEMDEVCAPS_DIAL_OPTIONS = MODEMDEVCAPS_DIAL_OPTIONS(128u32);
impl ::core::marker::Copy for MODEMDEVCAPS_DIAL_OPTIONS {}
impl ::core::clone::Clone for MODEMDEVCAPS_DIAL_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEMDEVCAPS_DIAL_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEMDEVCAPS_DIAL_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEMDEVCAPS_DIAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_DIAL_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEMDEVCAPS_SPEAKER_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKRFLAG_CALLSETUP: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKRFLAG_DIAL: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKRFLAG_OFF: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKRFLAG_ON: MODEMDEVCAPS_SPEAKER_MODE = MODEMDEVCAPS_SPEAKER_MODE(4u32);
impl ::core::marker::Copy for MODEMDEVCAPS_SPEAKER_MODE {}
impl ::core::clone::Clone for MODEMDEVCAPS_SPEAKER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEMDEVCAPS_SPEAKER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEMDEVCAPS_SPEAKER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEMDEVCAPS_SPEAKER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_SPEAKER_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEMDEVCAPS_SPEAKER_VOLUME(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOLFLAG_HIGH: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(4u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOLFLAG_LOW: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOLFLAG_MEDIUM: MODEMDEVCAPS_SPEAKER_VOLUME = MODEMDEVCAPS_SPEAKER_VOLUME(2u32);
impl ::core::marker::Copy for MODEMDEVCAPS_SPEAKER_VOLUME {}
impl ::core::clone::Clone for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_SPEAKER_VOLUME").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
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
impl ::core::marker::Copy for MODEMSETTINGS {}
impl ::core::clone::Clone for MODEMSETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MODEMSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODEMSETTINGS")
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
unsafe impl ::windows::core::Abi for MODEMSETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MODEMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MODEMSETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for MODEMSETTINGS {}
impl ::core::default::Default for MODEMSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEMSETTINGS_SPEAKER_MODE(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKR_CALLSETUP: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKR_DIAL: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKR_OFF: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMSPKR_ON: MODEMSETTINGS_SPEAKER_MODE = MODEMSETTINGS_SPEAKER_MODE(4u32);
impl ::core::marker::Copy for MODEMSETTINGS_SPEAKER_MODE {}
impl ::core::clone::Clone for MODEMSETTINGS_SPEAKER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEMSETTINGS_SPEAKER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEMSETTINGS_SPEAKER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEMSETTINGS_SPEAKER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMSETTINGS_SPEAKER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEM_SPEAKER_VOLUME(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOL_HIGH: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOL_LOW: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(0u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MDMVOL_MEDIUM: MODEM_SPEAKER_VOLUME = MODEM_SPEAKER_VOLUME(1u32);
impl ::core::marker::Copy for MODEM_SPEAKER_VOLUME {}
impl ::core::clone::Clone for MODEM_SPEAKER_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEM_SPEAKER_VOLUME {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEM_SPEAKER_VOLUME {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEM_SPEAKER_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEM_SPEAKER_VOLUME").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MODEM_STATUS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MS_CTS_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MS_DSR_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MS_RING_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const MS_RLSD_ON: MODEM_STATUS_FLAGS = MODEM_STATUS_FLAGS(128u32);
impl ::core::marker::Copy for MODEM_STATUS_FLAGS {}
impl ::core::clone::Clone for MODEM_STATUS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MODEM_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MODEM_STATUS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MODEM_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEM_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEM_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEM_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenCommPort(uportnumber: u32, dwdesiredaccess: u32, dwflagsandattributes: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenCommPort(::core::mem::transmute(uportnumber), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(dwflagsandattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PURGE_COMM_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PURGE_RXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PURGE_RXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PURGE_TXABORT: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`*"]
pub const PURGE_TXCLEAR: PURGE_COMM_FLAGS = PURGE_COMM_FLAGS(4u32);
impl ::core::marker::Copy for PURGE_COMM_FLAGS {}
impl ::core::clone::Clone for PURGE_COMM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PURGE_COMM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PURGE_COMM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PURGE_COMM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PURGE_COMM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PURGE_COMM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PURGE_COMM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PURGE_COMM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PurgeComm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PurgeComm(hfile: super::super::Foundation::HANDLE, dwflags: PURGE_COMM_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PurgeComm(hfile.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SID_3GPP_SUPSVCMODEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7d08e07_d767_4478_b14a_eecc87ea12f7);
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommBreak<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommBreak(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCommBreak(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommConfig<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hcommdev: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommConfig(hcommdev: super::super::Foundation::HANDLE, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCommConfig(hcommdev.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommMask(hfile: super::super::Foundation::HANDLE, dwevtmask: COMM_EVENT_MASK) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCommMask(hfile.into_param().abi(), ::core::mem::transmute(dwevtmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpdcb: *const DCB) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommState(hfile: super::super::Foundation::HANDLE, lpdcb: *const DCB) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCommState(hfile.into_param().abi(), ::core::mem::transmute(lpdcb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCommTimeouts<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCommTimeouts(hfile: super::super::Foundation::HANDLE, lpcommtimeouts: *const COMMTIMEOUTS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetCommTimeouts(hfile.into_param().abi(), ::core::mem::transmute(lpcommtimeouts)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultCommConfigA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpszname: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultCommConfigA(lpszname: ::windows::core::PCSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDefaultCommConfigA(lpszname.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDefaultCommConfigW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpszname: Param0, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDefaultCommConfigW(lpszname: ::windows::core::PCWSTR, lpcc: *const COMMCONFIG, dwsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetDefaultCommConfigW(lpszname.into_param().abi(), ::core::mem::transmute(lpcc), ::core::mem::transmute(dwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetupComm<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetupComm(hfile: super::super::Foundation::HANDLE, dwinqueue: u32, dwoutqueue: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetupComm(hfile.into_param().abi(), ::core::mem::transmute(dwinqueue), ::core::mem::transmute(dwoutqueue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TransmitCommChar<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::CHAR>>(hfile: Param0, cchar: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TransmitCommChar(hfile: super::super::Foundation::HANDLE, cchar: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TransmitCommChar(hfile.into_param().abi(), cchar.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Devices_Communication\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WaitCommEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitCommEvent(hfile: super::super::Foundation::HANDLE, lpevtmask: *mut COMM_EVENT_MASK, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WaitCommEvent(hfile.into_param().abi(), ::core::mem::transmute(lpevtmask), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
