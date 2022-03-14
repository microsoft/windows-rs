#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ARP_HARDWARE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ARP_HW_ENET: ARP_HARDWARE_TYPE = ARP_HARDWARE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ARP_HW_802: ARP_HARDWARE_TYPE = ARP_HARDWARE_TYPE(6i32);
impl ::core::marker::Copy for ARP_HARDWARE_TYPE {}
impl ::core::clone::Clone for ARP_HARDWARE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ARP_HARDWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ARP_HARDWARE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ARP_HARDWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARP_HARDWARE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ARP_HEADER {
    pub HardwareAddressSpace: u16,
    pub ProtocolAddressSpace: u16,
    pub HardwareAddressLength: u8,
    pub ProtocolAddressLength: u8,
    pub Opcode: u16,
    pub SenderHardwareAddress: [u8; 1],
}
impl ::core::marker::Copy for ARP_HEADER {}
impl ::core::clone::Clone for ARP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ARP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARP_HEADER").field("HardwareAddressSpace", &self.HardwareAddressSpace).field("ProtocolAddressSpace", &self.ProtocolAddressSpace).field("HardwareAddressLength", &self.HardwareAddressLength).field("ProtocolAddressLength", &self.ProtocolAddressLength).field("Opcode", &self.Opcode).field("SenderHardwareAddress", &self.SenderHardwareAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for ARP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ARP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ARP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ARP_HEADER {}
impl ::core::default::Default for ARP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ARP_OPCODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ARP_REQUEST: ARP_OPCODE = ARP_OPCODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ARP_RESPONSE: ARP_OPCODE = ARP_OPCODE(2i32);
impl ::core::marker::Copy for ARP_OPCODE {}
impl ::core::clone::Clone for ARP_OPCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ARP_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ARP_OPCODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ARP_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARP_OPCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const BYTE_ORDER: u32 = 1234u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const DL_ADDRESS_LENGTH_MAXIMUM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DL_ADDRESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const DlUnicast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const DlMulticast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const DlBroadcast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(2i32);
impl ::core::marker::Copy for DL_ADDRESS_TYPE {}
impl ::core::clone::Clone for DL_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DL_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DL_ADDRESS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DL_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DL_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_EI48 {
    pub Byte: [u8; 3],
}
impl ::core::marker::Copy for DL_EI48 {}
impl ::core::clone::Clone for DL_EI48 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EI48 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EI48 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EI48>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EI48 {}
impl ::core::default::Default for DL_EI48 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_EI64 {
    pub Byte: [u8; 5],
}
impl ::core::marker::Copy for DL_EI64 {}
impl ::core::clone::Clone for DL_EI64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EI64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EI64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EI64>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EI64 {}
impl ::core::default::Default for DL_EI64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_EUI48 {
    pub Byte: [u8; 6],
    pub Anonymous: DL_EUI48_0,
}
impl ::core::marker::Copy for DL_EUI48 {}
impl ::core::clone::Clone for DL_EUI48 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI48 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI48 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI48>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI48 {}
impl ::core::default::Default for DL_EUI48 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct DL_EUI48_0 {
    pub Oui: DL_OUI,
    pub Ei48: DL_EI48,
}
impl ::core::marker::Copy for DL_EUI48_0 {}
impl ::core::clone::Clone for DL_EUI48_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI48_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI48_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI48_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI48_0 {}
impl ::core::default::Default for DL_EUI48_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_EUI64 {
    pub Byte: [u8; 8],
    pub Value: u64,
    pub Anonymous: DL_EUI64_0,
}
impl ::core::marker::Copy for DL_EUI64 {}
impl ::core::clone::Clone for DL_EUI64 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI64>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI64 {}
impl ::core::default::Default for DL_EUI64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct DL_EUI64_0 {
    pub Oui: DL_OUI,
    pub Anonymous: DL_EUI64_0_0,
}
impl ::core::marker::Copy for DL_EUI64_0 {}
impl ::core::clone::Clone for DL_EUI64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI64_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI64_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI64_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI64_0 {}
impl ::core::default::Default for DL_EUI64_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_EUI64_0_0 {
    pub Ei64: DL_EI64,
    pub Anonymous: DL_EUI64_0_0_0,
}
impl ::core::marker::Copy for DL_EUI64_0_0 {}
impl ::core::clone::Clone for DL_EUI64_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI64_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI64_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI64_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI64_0_0 {}
impl ::core::default::Default for DL_EUI64_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct DL_EUI64_0_0_0 {
    pub Type: u8,
    pub Tse: u8,
    pub Ei48: DL_EI48,
}
impl ::core::marker::Copy for DL_EUI64_0_0_0 {}
impl ::core::clone::Clone for DL_EUI64_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_EUI64_0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_EUI64_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_EUI64_0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_EUI64_0_0_0 {}
impl ::core::default::Default for DL_EUI64_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const DL_HEADER_LENGTH_MAXIMUM: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union DL_OUI {
    pub Byte: [u8; 3],
    pub Anonymous: DL_OUI_0,
}
impl ::core::marker::Copy for DL_OUI {}
impl ::core::clone::Clone for DL_OUI {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DL_OUI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_OUI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_OUI>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_OUI {}
impl ::core::default::Default for DL_OUI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct DL_OUI_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DL_OUI_0 {}
impl ::core::clone::Clone for DL_OUI_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DL_OUI_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DL_OUI_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for DL_OUI_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DL_OUI_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_OUI_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DL_OUI_0 {}
impl ::core::default::Default for DL_OUI_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union DL_TEREDO_ADDRESS_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS_0_0 {
    pub Flags: u16,
    pub MappedPort: u16,
    pub MappedAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS_PRV {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS_PRV {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS_PRV>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS_PRV {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS_PRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union DL_TEREDO_ADDRESS_PRV_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS_PRV_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS_PRV_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS_PRV_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS_PRV_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS_PRV_0_0 {
    pub Flags: u16,
    pub MappedPort: u16,
    pub MappedAddress: super::super::Networking::WinSock::IN_ADDR,
    pub LocalAddress: super::super::Networking::WinSock::IN_ADDR,
    pub InterfaceIndex: u32,
    pub LocalPort: u16,
    pub DlDestination: DL_EUI48,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for DL_TEREDO_ADDRESS_PRV_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for DL_TEREDO_ADDRESS_PRV_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TEREDO_ADDRESS_PRV_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for DL_TEREDO_ADDRESS_PRV_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
pub struct DL_TUNNEL_ADDRESS {
    pub CompartmentId: super::super::System::Kernel::COMPARTMENT_ID,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
    pub IpAddress: [u8; 1],
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for DL_TUNNEL_ADDRESS {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for DL_TUNNEL_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for DL_TUNNEL_ADDRESS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for DL_TUNNEL_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DL_TUNNEL_ADDRESS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for DL_TUNNEL_ADDRESS {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for DL_TUNNEL_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ETHERNET_HEADER {
    pub Destination: DL_EUI48,
    pub Source: DL_EUI48,
    pub Anonymous: ETHERNET_HEADER_0,
}
impl ::core::marker::Copy for ETHERNET_HEADER {}
impl ::core::clone::Clone for ETHERNET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ETHERNET_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETHERNET_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETHERNET_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETHERNET_HEADER {}
impl ::core::default::Default for ETHERNET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union ETHERNET_HEADER_0 {
    pub Type: u16,
    pub Length: u16,
}
impl ::core::marker::Copy for ETHERNET_HEADER_0 {}
impl ::core::clone::Clone for ETHERNET_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ETHERNET_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ETHERNET_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ETHERNET_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ETHERNET_HEADER_0 {}
impl ::core::default::Default for ETHERNET_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_802_1AD: u32 = 34984u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_802_1Q: u32 = 33024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_ARP: u32 = 2054u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_IPV4: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_IPV6: u32 = 34525u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETHERNET_TYPE_MINIMUM: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETH_LENGTH_OF_HEADER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETH_LENGTH_OF_SNAP_HEADER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ETH_LENGTH_OF_VLAN_HEADER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const EXT_LEN_UNIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FALLBACK_INDEX(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FallbackIndexTcpFastopen: FALLBACK_INDEX = FALLBACK_INDEX(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FallbackIndexMax: FALLBACK_INDEX = FALLBACK_INDEX(1i32);
impl ::core::marker::Copy for FALLBACK_INDEX {}
impl ::core::clone::Clone for FALLBACK_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FALLBACK_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FALLBACK_INDEX {
    type Abi = Self;
}
impl ::core::fmt::Debug for FALLBACK_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FALLBACK_INDEX").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_ACTION0 {
    pub r#type: u32,
    pub Anonymous: FWPM_ACTION0_0,
}
impl ::core::marker::Copy for FWPM_ACTION0 {}
impl ::core::clone::Clone for FWPM_ACTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FWPM_ACTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_ACTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_ACTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_ACTION0 {}
impl ::core::default::Default for FWPM_ACTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union FWPM_ACTION0_0 {
    pub filterType: ::windows::core::GUID,
    pub calloutKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_ACTION0_0 {}
impl ::core::clone::Clone for FWPM_ACTION0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FWPM_ACTION0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_ACTION0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_ACTION0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_ACTION0_0 {}
impl ::core::default::Default for FWPM_ACTION0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_ADD_LINK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_BEGIN_READ_TXN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_BEGIN_WRITE_TXN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_CLASSIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_ENUM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_OPEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_READ: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_READ_STATS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_SUBSCRIBE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ACTRL_WRITE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_APPC_NETWORK_CAPABILITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT: FWPM_APPC_NETWORK_CAPABILITY_TYPE = FWPM_APPC_NETWORK_CAPABILITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER: FWPM_APPC_NETWORK_CAPABILITY_TYPE = FWPM_APPC_NETWORK_CAPABILITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_PRIVATE_NETWORK: FWPM_APPC_NETWORK_CAPABILITY_TYPE = FWPM_APPC_NETWORK_CAPABILITY_TYPE(2i32);
impl ::core::marker::Copy for FWPM_APPC_NETWORK_CAPABILITY_TYPE {}
impl ::core::clone::Clone for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_APPC_NETWORK_CAPABILITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_AUTO_WEIGHT_BITS: u32 = 60u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CALLOUT0 {
    pub calloutKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub applicableLayer: ::windows::core::GUID,
    pub calloutId: u32,
}
impl ::core::marker::Copy for FWPM_CALLOUT0 {}
impl ::core::clone::Clone for FWPM_CALLOUT0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CALLOUT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT0").field("calloutKey", &self.calloutKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerKey", &self.providerKey).field("providerData", &self.providerData).field("applicableLayer", &self.applicableLayer).field("calloutId", &self.calloutId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CALLOUT0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CALLOUT0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT0 {}
impl ::core::default::Default for FWPM_CALLOUT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779719a4_e695_47b6_a199_7999fec9163b);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9661b6_7c5e_48fd_a130_96678ceacc41);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18729c7a_2f62_4be0_966f_974b21b86df1);
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c3fb801_daff_40e9_91e6_f7ff7e52f7d9);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CALLOUT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub calloutKey: ::windows::core::GUID,
    pub calloutId: u32,
}
impl ::core::marker::Copy for FWPM_CALLOUT_CHANGE0 {}
impl ::core::clone::Clone for FWPM_CALLOUT_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CALLOUT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_CHANGE0").field("changeType", &self.changeType).field("calloutKey", &self.calloutKey).field("calloutId", &self.calloutId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CALLOUT_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CALLOUT_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_CHANGE0 {}
impl ::core::default::Default for FWPM_CALLOUT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_CALLOUT_CHANGE_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const FWPM_CALLOUT_CHANGE0)>;
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_LISTEN_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33486ab5_6d5e_4e65_a00b_a7afed0ba9a1);
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_RESOURCE_ASSIGNMENT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x079b1010_f1c5_4fcd_ae05_da41107abd0b);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CALLOUT_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::core::GUID,
    pub layerKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_CALLOUT_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("layerKey", &self.layerKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CALLOUT_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CALLOUT_FLAG_PERSISTENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CALLOUT_FLAG_REGISTERED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CALLOUT_FLAG_USES_PROVIDER_CONTEXT: u32 = 131072u32;
pub const FWPM_CALLOUT_HTTP_TEMPLATE_SSL_HANDSHAKE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3423249_8d09_4858_9210_95c7fda8e30f);
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac141fc_f75d_4203_b9c8_48e6149c2712);
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c0dda05_e31f_4666_90b0_b3dfad34129a);
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fcb56ec_cd37_4b4f_b108_62c2b1850a0c);
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d08a342_db9e_4fbe_9ed2_57374ce89f79);
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28829633_c4f0_4e66_873f_844db2a899c7);
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf50bec2_c686_429a_884d_b74443e7b0b4);
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb532136_15cb_440b_937c_1717ca320c40);
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdae640cc_e021_4bee_9eb6_a48b275c8c1d);
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dff309b_ba7d_4aba_91aa_ae5c6640c944);
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9a0d6d9_c58c_474e_8aeb_3cfe99d6d53d);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5132900d_5e84_4b5f_80e4_01741e81ff10);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d3ac92_2a6c_4dcf_955f_1c3be009dd99);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df6e7de_fd20_48f2_9f26_f854444cba79);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1e392d3_72ac_47bb_87a7_0122c69434ab);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x191a8a46_0bf8_46cf_b045_4b45dfa6a324);
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80c342e3_1e53_4d6f_9b44_03df5aeee154);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b46bf0a_4523_4e57_aa38_a87987c910d9);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38d87722_ad83_4f11_a91f_df0fb077225b);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70a4196c_835b_4fb0_98e8_075f4d977d46);
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1835363_a6a5_4e62_b180_23db789d8da6);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a700);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a701);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a702);
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fbfc31d_a51c_44dc_acb6_0624a030a703);
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x288b524d_0566_4e19_b612_8f441a2e5949);
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00b84b92_2b5e_4b71_ab0e_aaca43e387e6);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc582280_1677_41e9_94ab_c2fcb15c2eeb);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98e5373c_b884_490f_b65f_2f6a4a575195);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d55f008_0c01_4f92_b26e_a08a94569b8d);
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63018537_f281_4dc4_83d3_8dec18b7ade2);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CALLOUT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CALLOUT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_CALLOUT_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CALLOUT_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CALLOUT_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe183ecb2_3a7f_4b54_8ad9_76050ed880ca);
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0378cf41_bf98_4603_81f2_7f12586079f6);
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3e10ab3_2c25_4279_ac36_c30fc181bec4);
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e22085_a341_42fc_a279_aec94e689c56);
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f23f5d0_40c4_4c41_a254_46d8dba8957c);
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb25152f0_991c_4f53_bbe7_d24b45fe632c);
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x215a0b39_4b7e_4eda_8ce4_179679df6224);
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x838b37a1_5c12_4d34_8b38_078728b2d25c);
pub const FWPM_CALLOUT_TEREDO_ALE_LISTEN_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81a434e7_f60c_4378_bab8_c625a30f0197);
pub const FWPM_CALLOUT_TEREDO_ALE_RESOURCE_ASSIGNMENT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31b95392_066e_42a2_b7db_92f8acdd56f9);
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V4_SILENT_DROP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeda08606_2494_4d78_89bc_67837c03b969);
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V6_SILENT_DROP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8693cc74_a075_4156_b476_9286eece814e);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CHANGE_ADD: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CHANGE_DELETE: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CHANGE_TYPE_MAX: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for FWPM_CHANGE_TYPE {}
impl ::core::clone::Clone for FWPM_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_CLASSIFY_OPTION0 {
    pub r#type: FWP_CLASSIFY_OPTION_TYPE,
    pub value: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_CLASSIFY_OPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_CLASSIFY_OPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_CLASSIFY_OPTION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_CLASSIFY_OPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CLASSIFY_OPTION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_CLASSIFY_OPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_CLASSIFY_OPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_CLASSIFY_OPTIONS0 {
    pub numOptions: u32,
    pub options: *mut FWPM_CLASSIFY_OPTION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_CLASSIFY_OPTIONS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_CLASSIFY_OPTIONS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_CLASSIFY_OPTIONS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CLASSIFY_OPTIONS0").field("numOptions", &self.numOptions).field("options", &self.options).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_CLASSIFY_OPTIONS0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_CLASSIFY_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CLASSIFY_OPTIONS0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_CLASSIFY_OPTIONS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_CLASSIFY_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_CONDITION_ALE_APP_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd78e1e87_8644_4ea5_9437_d809ecefc971);
pub const FWPM_CONDITION_ALE_EFFECTIVE_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1277b9a_b781_40fc_9671_e5f1b989f34e);
pub const FWPM_CONDITION_ALE_NAP_CONTEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46275a9d_c03f_4d77_b784_1c57f4d02753);
pub const FWPM_CONDITION_ALE_ORIGINAL_APP_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e6cd086_e1fb_4212_842f_8a9f993fb3f6);
pub const FWPM_CONDITION_ALE_PACKAGE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71bc78fa_f17c_4997_a602_6abb261f351c);
pub const FWPM_CONDITION_ALE_PROMISCUOUS_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c974776_7182_46e9_afd3_b02910e30334);
pub const FWPM_CONDITION_ALE_REAUTH_REASON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb482d227_1979_4a98_8044_18bbe6237542);
pub const FWPM_CONDITION_ALE_REMOTE_MACHINE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aa47f51_7f93_4508_a271_81abb00c9cab);
pub const FWPM_CONDITION_ALE_REMOTE_USER_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf63073b7_0189_4ab0_95a4_6123cbfab862);
pub const FWPM_CONDITION_ALE_SECURITY_ATTRIBUTE_FQBN_VALUE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37a57699_5883_4963_92b8_3e704688b0ad);
pub const FWPM_CONDITION_ALE_SIO_FIREWALL_SYSTEM_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f4e088_cb98_4efb_a2c7_ad07332643db);
pub const FWPM_CONDITION_ALE_USER_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf043a0a_b34d_4f86_979c_c90371af6e66);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc088db3_1792_4a71_b0f9_037d21cd828b);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_PROFILE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdfe6aab_c083_4142_8679_c08f95329c61);
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89f990de_e798_4e6d_ab76_7c9558292e6f);
pub const FWPM_CONDITION_ARRIVAL_TUNNEL_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x511166dc_7a8c_4aa7_b533_95ab59fb0340);
pub const FWPM_CONDITION_AUTHENTICATION_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb458cd5_da7b_4ef9_8d43_7b0a840332f2);
pub const FWPM_CONDITION_CLIENT_CERT_KEY_LENGTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3ec00c7_05f4_4df7_91f2_5f60d91ff443);
pub const FWPM_CONDITION_CLIENT_CERT_OID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc491ad5e_f882_4283_b916_436b103ff4ad);
pub const FWPM_CONDITION_CLIENT_TOKEN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc228fc1e_403a_4478_be05_c9baa4c05ace);
pub const FWPM_CONDITION_COMPARTMENT_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35a791ab_04ac_4ff2_a6bb_da6cfac71806);
pub const FWPM_CONDITION_CURRENT_PROFILE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3033c9_c0e3_4759_937d_5758c65d4ae3);
pub const FWPM_CONDITION_DCOM_APP_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2e7b4d_3112_4770_b636_4d24ae3a6af2);
pub const FWPM_CONDITION_DESTINATION_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35cf6522_4139_45ee_a0d5_67b80949d879);
pub const FWPM_CONDITION_DESTINATION_SUB_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b7d4399_d4c7_4738_a2f5_e994b43da388);
pub const FWPM_CONDITION_DIRECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8784c146_ca97_44d6_9fd1_19fb1840cbf7);
pub const FWPM_CONDITION_EMBEDDED_LOCAL_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4672a468_8a0a_4202_abb4_849e92e66809);
pub const FWPM_CONDITION_EMBEDDED_LOCAL_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfca394d_acdb_484e_b8e6_2aff79757345);
pub const FWPM_CONDITION_EMBEDDED_PROTOCOL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07784107_a29e_4c7b_9ec7_29c44afafdbc);
pub const FWPM_CONDITION_EMBEDDED_REMOTE_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77ee4b39_3273_4671_b63b_ab6feb66eeb6);
pub const FWPM_CONDITION_EMBEDDED_REMOTE_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcae4d6a1_2968_40ed_a4ce_547160dda88d);
pub const FWPM_CONDITION_ETHER_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd08948d_a219_4d52_bb98_1a5540ee7b4e);
pub const FWPM_CONDITION_FLAGS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x632ce23b_5167_435c_86d7_e903684aa80c);
pub const FWPM_CONDITION_IMAGE_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd024de4d_deaa_4317_9c85_e40ef6e140c3);
pub const FWPM_CONDITION_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x667fd755_d695_434a_8af5_d3835a1259bc);
pub const FWPM_CONDITION_INTERFACE_MAC_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6e63dce_1f4b_4c6b_b6ef_1165e71f8ee7);
pub const FWPM_CONDITION_INTERFACE_QUARANTINE_EPOCH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcce68d5e_053b_43a8_9a6f_33384c28e4f6);
pub const FWPM_CONDITION_INTERFACE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaf8cd14_e09e_4c93_a5ae_c5c13b73ffca);
pub const FWPM_CONDITION_IPSEC_POLICY_KEY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad37dee3_722f_45cc_a4e3_068048124452);
pub const FWPM_CONDITION_IPSEC_SECURITY_REALM_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37a57700_5884_4964_92b8_3e704688b0ad);
pub const FWPM_CONDITION_IP_ARRIVAL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x618a9b6d_386b_4136_ad6e_b51587cfb1cd);
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d79133b_b390_45c6_8699_acaceaafed33);
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec1b7c9_4eea_4f5e_b9ef_76beaaaf17ee);
pub const FWPM_CONDITION_IP_DESTINATION_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce6def45_60fb_4a7b_a304_af30a117000e);
pub const FWPM_CONDITION_IP_FORWARD_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1076b8a5_6323_4c5e_9810_e8d3fc9e6136);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ee00de_c1ef_4617_bfe3_ffd8f5a08957);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ec7f6c4_376b_45d7_9e9c_d337cedcd237);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03a629cb_6e52_49f8_9c41_5709633c09cf);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2381be84_7524_45b3_a05b_1e637d9c7a6a);
pub const FWPM_CONDITION_IP_LOCAL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cd62a49_59c3_4969_b7f3_bda5d32890a4);
pub const FWPM_CONDITION_IP_LOCAL_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c1ba1af_5765_453f_af22_a8f791ac775b);
pub const FWPM_CONDITION_IP_NEXTHOP_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeabe448a_a711_4d64_85b7_3f76b65299c7);
pub const FWPM_CONDITION_IP_NEXTHOP_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93ae8f5b_7f6f_4719_98c8_14e97429ef04);
pub const FWPM_CONDITION_IP_PHYSICAL_ARRIVAL_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda50d5c8_fa0d_4c89_b032_6e62136d1e96);
pub const FWPM_CONDITION_IP_PHYSICAL_NEXTHOP_INTERFACE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf09bd5ce_5150_48be_b098_c25152fb1f92);
pub const FWPM_CONDITION_IP_PROTOCOL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3971ef2b_623e_4f9a_8cb1_6e79b806b9a7);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb235ae9a_1d64_49b8_a44c_5ff3d9095045);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1febb610_3bcc_45e1_bc36_2e067e2cb186);
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246e1d8c_8bee_4018_9b98_31d4582f3361);
pub const FWPM_CONDITION_IP_REMOTE_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc35a604d_d22b_4e1a_91b4_68f674ee674b);
pub const FWPM_CONDITION_IP_SOURCE_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae96897e_2e94_4bc9_b313_b27ee80e574d);
pub const FWPM_CONDITION_IP_SOURCE_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6afef91_3df4_4730_a214_f5426aebf821);
pub const FWPM_CONDITION_KM_AUTH_NAP_CONTEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35d0ea0e_15ca_492b_900e_97fd46352cce);
pub const FWPM_CONDITION_KM_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeef4582_ef8f_4f7b_858b_9077d122de47);
pub const FWPM_CONDITION_KM_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff0f5f49_0ceb_481b_8638_1479791f3f2c);
pub const FWPM_CONDITION_L2_FLAGS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bc43cbf_37ba_45f1_b74a_82ff518eeb10);
pub const FWPM_CONDITION_LOCAL_INTERFACE_PROFILE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ebf7562_9f18_4d06_9941_a7a625744d71);
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04ea2a93_858c_4027_b613_b43180c7859e);
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae052932_ef42_4e99_b129_f3b3139e34f7);
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd999e981_7948_4c83_b742_c84e3b678f8f);
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc31355c_3073_4ffb_a14f_79415cb1ead1);
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x408f2ed4_3a70_4b4d_92a6_415ac20e2f12);
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x027fedb4_f1c1_4030_b564_ee777fd867ea);
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b795451_f1f6_4d05_b7cb_21779d802336);
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c1b72e4_299e_4437_a298_bc3f014b3dc2);
pub const FWPM_CONDITION_NDIS_MEDIA_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb31cef1_791d_473b_89d1_61c5984304a0);
pub const FWPM_CONDITION_NDIS_PHYSICAL_MEDIA_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34c79823_c229_44f2_b83c_74020882ae77);
pub const FWPM_CONDITION_NDIS_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb7bb42b_2dac_4cd4_a59a_e0bdce1e6834);
pub const FWPM_CONDITION_NET_EVENT_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x206e9996_490e_40cf_b831_b38641eb6fcb);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x138e6888_7ab8_4d65_9ee8_0591bcf6a494);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_PROFILE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7ff9a56_cdaa_472b_84db_d23963c1d1bf);
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97537c6c_d9a3_4767_a381_e942675cd920);
pub const FWPM_CONDITION_NEXTHOP_SUB_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8a6122_0577_45a7_9aaf_825fbeb4fb95);
pub const FWPM_CONDITION_NEXTHOP_TUNNEL_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b1a111_987b_4720_99dd_c7c576fa2d4c);
pub const FWPM_CONDITION_ORIGINAL_ICMP_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076dfdbe_c56c_4f72_ae8a_2cfe7e5c8286);
pub const FWPM_CONDITION_ORIGINAL_PROFILE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46ea1551_2255_492b_8019_aabeee349f40);
pub const FWPM_CONDITION_PEER_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b539082_eb90_4186_a6cc_de5b63235016);
pub const FWPM_CONDITION_PIPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bd0741d_e3df_4e24_8634_762046eef6eb);
pub const FWPM_CONDITION_PROCESS_WITH_RPC_IF_UUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe31180a8_bbbd_4d14_a65e_7157b06233bb);
pub const FWPM_CONDITION_QM_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf64fc6d1_f9cb_43d2_8a5f_e13bc894f265);
pub const FWPM_CONDITION_REAUTHORIZE_REASON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11205e8c_11ae_457a_8a44_477026dd764a);
pub const FWPM_CONDITION_REMOTE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68166fd_0682_4c89_b8f5_86436c7ef9b7);
pub const FWPM_CONDITION_REMOTE_USER_TOKEN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9bf0ee66_06c9_41b9_84da_288cb43af51f);
pub const FWPM_CONDITION_RESERVED0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678f4deb_45af_4882_93fe_19d4729d9834);
pub const FWPM_CONDITION_RESERVED1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd818f827_5c69_48eb_bf80_d86b17755f97);
pub const FWPM_CONDITION_RESERVED10: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb979e282_d621_4c8c_b184_b105a61c36ce);
pub const FWPM_CONDITION_RESERVED11: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d62ee4d_023d_411f_9582_43acbb795975);
pub const FWPM_CONDITION_RESERVED12: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3677c32_7e35_4ddc_93da_e8c33fc923c7);
pub const FWPM_CONDITION_RESERVED13: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x335a3e90_84aa_42f5_9e6f_59309536a44c);
pub const FWPM_CONDITION_RESERVED14: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30e44da2_2f1a_4116_a559_f907de83604a);
pub const FWPM_CONDITION_RESERVED15: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab8340f_afe0_43d1_80d8_5ca456962de3);
pub const FWPM_CONDITION_RESERVED2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d4123d_e15b_4e84_b7a8_dce16f7b62d9);
pub const FWPM_CONDITION_RESERVED3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f6e8ca3_6606_4932_97c7_e1f20710af3b);
pub const FWPM_CONDITION_RESERVED4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f58e642_b937_495e_a94b_f6b051a49250);
pub const FWPM_CONDITION_RESERVED5: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ba8f6cd_f77c_43e6_8847_11939dc5db5a);
pub const FWPM_CONDITION_RESERVED6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf13d84bd_59d5_44c4_8817_5ecdae1805bd);
pub const FWPM_CONDITION_RESERVED7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a0f930_45dd_4983_aa33_efc7b611af08);
pub const FWPM_CONDITION_RESERVED8: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f424974_0c12_4816_9b47_9a547db39a32);
pub const FWPM_CONDITION_RESERVED9: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce78e10f_13ff_4c70_8643_36ad1879afa3);
pub const FWPM_CONDITION_RPC_AUTH_LEVEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5a0aed5_59ac_46ea_be05_a5f05ecf446e);
pub const FWPM_CONDITION_RPC_AUTH_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaba74ab_0d67_43e7_986e_75b84f82f594);
pub const FWPM_CONDITION_RPC_EP_FLAGS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x218b814a_0a39_49b8_8e71_c20c39c7dd2e);
pub const FWPM_CONDITION_RPC_EP_VALUE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdccea0b9_0886_4360_9c6a_ab043a24fba9);
pub const FWPM_CONDITION_RPC_IF_FLAG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x238a8a32_3199_467d_871c_272621ab3896);
pub const FWPM_CONDITION_RPC_IF_UUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c9c7d9f_0075_4d35_a0d1_8311c4cf6af1);
pub const FWPM_CONDITION_RPC_IF_VERSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeabfd9b7_1262_4a2e_adaa_5f96f6fe326d);
pub const FWPM_CONDITION_RPC_PROTOCOL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2717bc74_3a35_4ce7_b7ef_c838fabdec45);
pub const FWPM_CONDITION_RPC_PROXY_AUTH_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40953fe2_8565_4759_8488_1771b4b4b5db);
pub const FWPM_CONDITION_RPC_SERVER_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb605a225_c3b3_48c7_9833_7aefa9527546);
pub const FWPM_CONDITION_RPC_SERVER_PORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8090f645_9ad5_4e3b_9f9f_8023ca097909);
pub const FWPM_CONDITION_SEC_ENCRYPT_ALGORITHM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d306ef0_e974_4f74_b5c7_591b0da7d562);
pub const FWPM_CONDITION_SEC_KEY_SIZE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4772183b_ccf8_4aeb_bce1_c6c6161c8fe4);
pub const FWPM_CONDITION_SOURCE_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2311334d_c92d_45bf_9496_edf447820e2d);
pub const FWPM_CONDITION_SOURCE_SUB_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055edd9d_acd2_4361_8dab_f9525d97662f);
pub const FWPM_CONDITION_SUB_INTERFACE_INDEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cd42473_d621_4be3_ae8c_72a348d283e1);
pub const FWPM_CONDITION_TUNNEL_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a40437_8779_4868_a261_f5a902f1c0cd);
pub const FWPM_CONDITION_VLAN_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x938eab21_3618_4e64_9ca5_2141ebda1ca2);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ed48be4_c926_49f6_a4f6_ef3030e3fc16);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa9b3f06_2f1a_4c57_9e68_a7098b28dbfe);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_VM_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6106aace_4de1_4c84_9671_3637f8bcf731);
pub const FWPM_CONDITION_VSWITCH_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4a414ba_437b_4de6_9946_d99c1b95b312);
pub const FWPM_CONDITION_VSWITCH_NETWORK_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d48b4b_e77a_40b4_9155_392c906c2608);
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f4ef24b_b2c1_4938_ba33_a1ecbed512ba);
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_TYPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6b040a2_edaf_4c36_908b_f2f58ae43807);
pub const FWPM_CONDITION_VSWITCH_SOURCE_VM_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c2a9ec2_9fc6_42bc_bdd8_406d4da0be64);
pub const FWPM_CONDITION_VSWITCH_TENANT_NETWORK_ID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc04843c_79e6_4e44_a025_65b9bb0f9f94);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_CONNECTION0 {
    pub connectionId: u64,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: FWPM_CONNECTION0_0,
    pub Anonymous2: FWPM_CONNECTION0_1,
    pub providerKey: *mut ::windows::core::GUID,
    pub ipsecTrafficModeType: IPSEC_TRAFFIC_TYPE,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub mmCrypto: IKEEXT_PROPOSAL0,
    pub mmPeer: IKEEXT_CREDENTIAL2,
    pub emPeer: IKEEXT_CREDENTIAL2,
    pub bytesTransferredIn: u64,
    pub bytesTransferredOut: u64,
    pub bytesTransferredTotal: u64,
    pub startSysTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_CONNECTION0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_CONNECTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_CONNECTION0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_CONNECTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CONNECTION0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_CONNECTION0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_CONNECTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FWPM_CONNECTION0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_CONNECTION0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_CONNECTION0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_CONNECTION0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_CONNECTION0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CONNECTION0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_CONNECTION0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_CONNECTION0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FWPM_CONNECTION0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_CONNECTION0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_CONNECTION0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_CONNECTION0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_CONNECTION0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CONNECTION0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_CONNECTION0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_CONNECTION0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FWPM_CONNECTION_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, eventtype: FWPM_CONNECTION_EVENT_TYPE, connection: *const FWPM_CONNECTION0)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CONNECTION_ENUM_FLAG_QUERY_BYTES_TRANSFERRED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CONNECTION_ENUM_TEMPLATE0 {
    pub connectionId: u64,
    pub flags: u32,
}
impl ::core::marker::Copy for FWPM_CONNECTION_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CONNECTION_ENUM_TEMPLATE0").field("connectionId", &self.connectionId).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CONNECTION_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CONNECTION_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_CONNECTION_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CONNECTION_EVENT_ADD: FWPM_CONNECTION_EVENT_TYPE = FWPM_CONNECTION_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CONNECTION_EVENT_DELETE: FWPM_CONNECTION_EVENT_TYPE = FWPM_CONNECTION_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CONNECTION_EVENT_MAX: FWPM_CONNECTION_EVENT_TYPE = FWPM_CONNECTION_EVENT_TYPE(2i32);
impl ::core::marker::Copy for FWPM_CONNECTION_EVENT_TYPE {}
impl ::core::clone::Clone for FWPM_CONNECTION_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_CONNECTION_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_CONNECTION_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_CONNECTION_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_CONNECTION_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_CONNECTION_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CONNECTION_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_CONNECTION_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CONNECTION_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_CONNECTION_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_CONNECTION_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_CONNECTION_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_DISPLAY_DATA0 {
    pub name: ::windows::core::PWSTR,
    pub description: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for FWPM_DISPLAY_DATA0 {}
impl ::core::clone::Clone for FWPM_DISPLAY_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_DISPLAY_DATA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_DISPLAY_DATA0").field("name", &self.name).field("description", &self.description).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_DISPLAY_DATA0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_DISPLAY_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_DISPLAY_DATA0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_DISPLAY_DATA0 {}
impl ::core::default::Default for FWPM_DISPLAY_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_DYNAMIC_KEYWORD_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(notification: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_ENGINE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_COLLECT_NET_EVENTS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_NAME_CACHE: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_PACKET_QUEUING: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_OPTION_MAX: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(6i32);
impl ::core::marker::Copy for FWPM_ENGINE_OPTION {}
impl ::core::clone::Clone for FWPM_ENGINE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_ENGINE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_ENGINE_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_ENGINE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_ENGINE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_OPTION_PACKET_BATCH_INBOUND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_FORWARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_INBOUND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_NONE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_FIELD0 {
    pub fieldKey: *mut ::windows::core::GUID,
    pub r#type: FWPM_FIELD_TYPE,
    pub dataType: FWP_DATA_TYPE,
}
impl ::core::marker::Copy for FWPM_FIELD0 {}
impl ::core::clone::Clone for FWPM_FIELD0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_FIELD0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FIELD0").field("fieldKey", &self.fieldKey).field("type", &self.r#type).field("dataType", &self.dataType).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_FIELD0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_FIELD0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FIELD0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_FIELD0 {}
impl ::core::default::Default for FWPM_FIELD0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_FIELD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FIELD_RAW_DATA: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FIELD_IP_ADDRESS: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FIELD_FLAGS: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FIELD_TYPE_MAX: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(3i32);
impl ::core::marker::Copy for FWPM_FIELD_TYPE {}
impl ::core::clone::Clone for FWPM_FIELD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_FIELD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_FIELD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER0 {
    pub filterKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: FWPM_FILTER_FLAGS,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub layerKey: ::windows::core::GUID,
    pub subLayerKey: ::windows::core::GUID,
    pub weight: FWP_VALUE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub action: FWPM_ACTION0,
    pub Anonymous: FWPM_FILTER0_0,
    pub reserved: *mut ::windows::core::GUID,
    pub filterId: u64,
    pub effectiveWeight: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_FILTER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_FILTER0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_FILTER0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_FILTER0_0 {
    pub rawContext: u64,
    pub providerContextKey: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_FILTER0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_FILTER0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_FILTER0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_FILTER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub filterKey: ::windows::core::GUID,
    pub filterId: u64,
}
impl ::core::marker::Copy for FWPM_FILTER_CHANGE0 {}
impl ::core::clone::Clone for FWPM_FILTER_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_FILTER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_CHANGE0").field("changeType", &self.changeType).field("filterKey", &self.filterKey).field("filterId", &self.filterId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_FILTER_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_FILTER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_FILTER_CHANGE0 {}
impl ::core::default::Default for FWPM_FILTER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_FILTER_CHANGE_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const FWPM_FILTER_CHANGE0)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_CONDITION0 {
    pub fieldKey: ::windows::core::GUID,
    pub matchType: FWP_MATCH_TYPE,
    pub conditionValue: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_FILTER_CONDITION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_FILTER_CONDITION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_FILTER_CONDITION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER_CONDITION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER_CONDITION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER_CONDITION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_CONDITION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::core::GUID,
    pub layerKey: ::windows::core::GUID,
    pub enumType: FWP_FILTER_ENUM_TYPE,
    pub flags: u32,
    pub providerContextTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub actionMask: u32,
    pub calloutKey: *mut ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_FILTER_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("layerKey", &self.layerKey).field("enumType", &self.enumType).field("flags", &self.flags).field("providerContextTemplate", &self.providerContextTemplate).field("numFilterConditions", &self.numFilterConditions).field("filterCondition", &self.filterCondition).field("actionMask", &self.actionMask).field("calloutKey", &self.calloutKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_FILTER_ENUM_TEMPLATE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER_ENUM_TEMPLATE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_FILTER_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_NONE: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_PERSISTENT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_BOOTTIME: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_HAS_PROVIDER_CONTEXT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_DISABLED: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_INDEXED: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(64u32);
impl ::core::marker::Copy for FWPM_FILTER_FLAGS {}
impl ::core::clone::Clone for FWPM_FILTER_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_FILTER_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FWPM_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FWPM_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_GAMEOS_ONLY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_RESERVED0: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_RESERVED1: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_SILENT_MODE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_FILTER_FLAG_SYSTEMOS_ONLY: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_FILTER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_FILTER_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_FILTER_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_FILTER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_FILTER_SUBSCRIPTION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_FILTER_SUBSCRIPTION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_KEYING_MODULE_AUTHIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11e3dae0_dd26_4590_857d_ab4b28d1a095);
pub const FWPM_KEYING_MODULE_IKE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9bbf787_82a8_45bb_a400_5d7e5952c7a9);
pub const FWPM_KEYING_MODULE_IKEV2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x041792cc_8f07_419d_a394_716968cb1647);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_LAYER0 {
    pub layerKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub numFields: u32,
    pub field: *mut FWPM_FIELD0,
    pub defaultSubLayerKey: ::windows::core::GUID,
    pub layerId: u16,
}
impl ::core::marker::Copy for FWPM_LAYER0 {}
impl ::core::clone::Clone for FWPM_LAYER0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_LAYER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER0").field("layerKey", &self.layerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("numFields", &self.numFields).field("field", &self.field).field("defaultSubLayerKey", &self.defaultSubLayerKey).field("layerId", &self.layerId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_LAYER0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_LAYER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_LAYER0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_LAYER0 {}
impl ::core::default::Default for FWPM_LAYER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc38d57d1_05a7_4c33_904f_7fbceee60e82);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd632a801_f5ba_4ad6_96e3_607017d9836a);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a72393b_319f_44bc_84c3_ba54dcb3b6b4);
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc97bc3b8_c9a3_4e33_8695_8e17aad4de09);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88bb5dad_76d7_4227_9c71_df0a3ed7be7e);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x371dfada_9f26_45fd_b4eb_c29eb212893f);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ac9de24_17dd_4814_b4bd_a9fbc95a321b);
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60703b07_63c8_48e9_ada3_12b1af40a617);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1cd9fe7_f4b5_4273_96c0_592e487b8650);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eeaa99b_bd22_4227_919f_0073c63357b1);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3b42c97_9f04_4672_b87e_cee9c483257f);
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89455b97_dbe1_453f_a224_13da895af396);
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66978cad_c704_42ac_86ac_7c1a231bd253);
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbef02c9c_606b_4536_8c26_1c2fc7b631d4);
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6e63c8c_b784_4562_aa7d_0a67cfcaf9a3);
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x587e54a7_8046_42ba_a0aa_b716250fc7fd);
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4766427_e2a2_467a_bd7e_dbcd1bd85a09);
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb536ccd_4755_4ba9_9ff7_f9edf8699c7b);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf80470a_5596_4c13_9992_539e6fe57967);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x146ae4a9_a1d2_4d43_a31a_4c42682b8e4f);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7021d2b3_dfa4_406e_afeb_6afaf7e70efd);
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46928636_bbca_4b76_941d_0fa7f5d7d372);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1247d66d_0b60_4a15_8d44_7155d0f53a0c);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5812a2_c3ff_4eca_b88d_c79e20ac6322);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55a650e1_5f0a_4eca_a653_88f53b26aa8c);
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbc998bb_c51f_4c1a_bb4f_9775fcacab2f);
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74365cce_ccb0_401a_bfc1_b89934ad7e15);
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4e5ce80_edcc_4e13_8a2f_b91454bb057b);
pub const FWPM_LAYER_DATAGRAM_DATA_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d08bf4e_45f6_4930_a922_417098e20027);
pub const FWPM_LAYER_DATAGRAM_DATA_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18e330c6_7248_4e52_aaab_472ed67704fd);
pub const FWPM_LAYER_DATAGRAM_DATA_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa45fe2f_3cba_4427_87fc_57b9a4b10d00);
pub const FWPM_LAYER_DATAGRAM_DATA_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09d1dfe1_9b86_4a42_be9d_8c315b92a5d0);
pub const FWPM_LAYER_EGRESS_VSWITCH_ETHERNET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86c872b0_76fa_4b79_93a4_0750530ae292);
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb92350b6_91f0_46b6_bdc4_871dfd4a7c98);
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b2def23_1881_40bd_82f4_4254e63141cb);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_LAYER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl ::core::marker::Copy for FWPM_LAYER_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_LAYER_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_LAYER_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_LAYER_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_LAYER_FLAG_BUFFERED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_LAYER_FLAG_BUILTIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_LAYER_FLAG_CLASSIFY_MOSTLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_LAYER_FLAG_KERNEL: u32 = 1u32;
pub const FWPM_LAYER_IKEEXT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb14b7bdb_dbbd_473e_bed4_8b4708d4f270);
pub const FWPM_LAYER_IKEEXT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb64786b3_f687_4eb9_89d2_8ef32acdabe2);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61499990_3cb6_4e84_b950_53b94b6964f3);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b17075_ebaf_4053_a4e7_213c8121ede5);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f9bdff_3b2d_4e5d_b8c6_c720651fe898);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6e7ccc0_08fb_468d_a472_9771d5595e09);
pub const FWPM_LAYER_INBOUND_IPPACKET_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc86fd1bf_21cd_497e_a0bb_17425c885c58);
pub const FWPM_LAYER_INBOUND_IPPACKET_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a230d0_a8c0_44f2_916e_991b53ded1f7);
pub const FWPM_LAYER_INBOUND_IPPACKET_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf52032cb_991c_46e7_971d_2601459a91ca);
pub const FWPM_LAYER_INBOUND_IPPACKET_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb24c279_93b4_47a2_83ad_ae1698b50885);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_ETHERNET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeffb7edb_0055_4f9a_a231_4ff8131ad191);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4220bd3_62ce_4f08_ae88_b56e8526df50);
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE_FAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x853aaa8e_2b78_4d24_a804_36db08b29711);
pub const FWPM_LAYER_INBOUND_RESERVED2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4fb8d55_c076_46d8_a2c7_6a4c722ca4ed);
pub const FWPM_LAYER_INBOUND_TRANSPORT_FAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe41d2719_05c7_40f0_8983_ea8d17bbc2f6);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5926dfc8_e3cf_4426_a283_dc393f5d0f9d);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac4a9833_f69d_4648_b261_6dc84835ef39);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x634a869f_fc23_4b90_b0c1_bf620a36ae6f);
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a6ff955_3b2b_49d2_9848_ad9d72dcaab7);
pub const FWPM_LAYER_INGRESS_VSWITCH_ETHERNET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d98577a_9a87_41ec_9718_7cf589c9f32d);
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2696ff6_774f_4554_9f7d_3da3945f8e85);
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ee314fc_7d8a_47f4_b7e3_291a36da4e12);
pub const FWPM_LAYER_IPFORWARD_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa82acc24_4ee1_4ee1_b465_fd1d25cb10a4);
pub const FWPM_LAYER_IPFORWARD_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e9ea773_2fae_4210_8f17_34129ef369eb);
pub const FWPM_LAYER_IPFORWARD_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b964818_19c7_493a_b71f_832c3684d28c);
pub const FWPM_LAYER_IPFORWARD_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31524a5d_1dfe_472f_bb93_518ee945d8a2);
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf02b1526_a459_4a51_b9e3_759de52b9d2c);
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f755cf6_2fd4_4e88_b3e4_a91bca495235);
pub const FWPM_LAYER_IPSEC_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeda65c74_610d_4bc5_948f_3c4f89556867);
pub const FWPM_LAYER_IPSEC_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13c48442_8d87_4261_9a29_59d2abc348b4);
pub const FWPM_LAYER_KM_AUTHORIZATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4aa226e9_9020_45fb_956a_c0249d841195);
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c2aa681_905b_4ccd_a467_4dd811d07b7b);
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92d592fa_6b01_434a_9dea_d1e96ea97da9);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41390100_564c_4b32_bc1d_718048354d7c);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3598d36_0561_4588_a6bf_e955e3f6264b);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fb03b60_7b8d_4dfa_badd_980176fc4e12);
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65f2e647_8d0c_4f47_b19b_33a4d3f1357c);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e5c9fae_8a84_4135_a331_950b54229ecd);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08e4bcb5_b647_48f3_953c_e5ddbd03937e);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3b3ab6b_3564_488c_9117_f34e82142763);
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9513d7c4_a934_49dc_91a7_6ccb80cc02e3);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_ETHERNET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x694673bc_d6db_4870_adee_0acdbdb7f4b2);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94c44912_9d6f_4ebf_b995_05ab8a088d1b);
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE_FAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470df946_c962_486f_9446_8293cbc75eb8);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_FAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13ed4388_a070_4815_9935_7a9be6408b78);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09e61aea_d214_46e2_9b21_b26b0b2f28c8);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5f10551_bdb0_43d7_a313_50e211f4d68a);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1735bde_013f_4655_b351_a49e15762df0);
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf433df69_ccbd_482e_b9b2_57165658c3b3);
pub const FWPM_LAYER_RPC_EPMAP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9247bc61_eb07_47ee_872c_bfd78bfd1616);
pub const FWPM_LAYER_RPC_EP_ADD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x618dffc7_c450_4943_95db_99b4c16a55d4);
pub const FWPM_LAYER_RPC_PROXY_CONN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94a4b50b_ba5c_4f27_907a_229fac0c2a7a);
pub const FWPM_LAYER_RPC_PROXY_IF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8a38615_e12c_41ac_98df_121ad981aade);
pub const FWPM_LAYER_RPC_UM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a89dda_95e4_40f3_adc7_7688a9c847e1);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_LAYER_STATISTICS0 {
    pub layerId: ::windows::core::GUID,
    pub classifyPermitCount: u32,
    pub classifyBlockCount: u32,
    pub classifyVetoCount: u32,
    pub numCacheEntries: u32,
}
impl ::core::marker::Copy for FWPM_LAYER_STATISTICS0 {}
impl ::core::clone::Clone for FWPM_LAYER_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_LAYER_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER_STATISTICS0").field("layerId", &self.layerId).field("classifyPermitCount", &self.classifyPermitCount).field("classifyBlockCount", &self.classifyBlockCount).field("classifyVetoCount", &self.classifyVetoCount).field("numCacheEntries", &self.numCacheEntries).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_LAYER_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_LAYER_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_LAYER_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_LAYER_STATISTICS0 {}
impl ::core::default::Default for FWPM_LAYER_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_LAYER_STREAM_PACKET_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf52d8ec_cb2d_44e5_ad92_f8dc38d2eb29);
pub const FWPM_LAYER_STREAM_PACKET_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779a8ca3_f099_468f_b5d4_83535c461c02);
pub const FWPM_LAYER_STREAM_V4: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b89653c_c170_49e4_b1cd_e0eeeee19a3e);
pub const FWPM_LAYER_STREAM_V4_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25c4c2c2_25ff_4352_82f9_c54a4a4726dc);
pub const FWPM_LAYER_STREAM_V6: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47c9137a_7ec4_46b3_b6e4_48e926b1eda4);
pub const FWPM_LAYER_STREAM_V6_DISCARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10a59fc7_b628_4c41_9eb8_cf37d55103cf);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT0 {
    pub header: FWPM_NET_EVENT_HEADER0,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT0_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE0,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE0,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP0,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT1 {
    pub header: FWPM_NET_EVENT_HEADER1,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT1_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP1,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT2 {
    pub header: FWPM_NET_EVENT_HEADER2,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT2_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT3 {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT3_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT3_0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE1,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE0,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT3_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT3_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT4_ {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT4__0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT4_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT4_ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT4_ {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT4_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT4_>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT4_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT4_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT4__0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT4__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT4__0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT4__0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT4__0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT4__0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT4__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT4__0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT5_ {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT5__0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT5_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT5_ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT5_ {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT5_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT5_>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT5_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT5_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT5__0 {
    pub ikeMmFailure: *mut FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_,
    pub ikeQmFailure: *mut FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_,
    pub ikeEmFailure: *mut FWPM_NET_EVENT_IKEEXT_EM_FAILURE1,
    pub classifyDrop: *mut FWPM_NET_EVENT_CLASSIFY_DROP2,
    pub ipsecDrop: *mut FWPM_NET_EVENT_IPSEC_KERNEL_DROP0,
    pub idpDrop: *mut FWPM_NET_EVENT_IPSEC_DOSP_DROP0,
    pub classifyAllow: *mut FWPM_NET_EVENT_CLASSIFY_ALLOW0,
    pub capabilityDrop: *mut FWPM_NET_EVENT_CAPABILITY_DROP0,
    pub capabilityAllow: *mut FWPM_NET_EVENT_CAPABILITY_ALLOW0,
    pub classifyDropMac: *mut FWPM_NET_EVENT_CLASSIFY_DROP_MAC0,
    pub lpmPacketArrival: *mut FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT5__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT5__0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT5__0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT5__0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT5__0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT5__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT5__0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, event: *const FWPM_NET_EVENT1)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK1 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, event: *const FWPM_NET_EVENT2)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK2 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, event: *const FWPM_NET_EVENT3)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK3 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, event: *const FWPM_NET_EVENT4_)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK4 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, event: *const FWPM_NET_EVENT5_)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CAPABILITY_ALLOW0").field("networkCapabilityId", &self.networkCapabilityId).field("filterId", &self.filterId).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CAPABILITY_ALLOW0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CAPABILITY_DROP0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CAPABILITY_DROP0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CAPABILITY_DROP0").field("networkCapabilityId", &self.networkCapabilityId).field("filterId", &self.filterId).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CAPABILITY_DROP0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_DROP0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_ALLOW0").field("filterId", &self.filterId).field("layerId", &self.layerId).field("reauthReason", &self.reauthReason).field("originalProfile", &self.originalProfile).field("currentProfile", &self.currentProfile).field("msFwpDirection", &self.msFwpDirection).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CLASSIFY_ALLOW0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP0 {
    pub filterId: u64,
    pub layerId: u16,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_CLASSIFY_DROP0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP0").field("filterId", &self.filterId).field("layerId", &self.layerId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CLASSIFY_DROP0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP0 {}
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP1 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CLASSIFY_DROP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP1").field("filterId", &self.filterId).field("layerId", &self.layerId).field("reauthReason", &self.reauthReason).field("originalProfile", &self.originalProfile).field("currentProfile", &self.currentProfile).field("msFwpDirection", &self.msFwpDirection).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CLASSIFY_DROP1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP2 {
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: super::super::Foundation::BOOL,
    pub vSwitchId: FWP_BYTE_BLOB,
    pub vSwitchSourcePort: u32,
    pub vSwitchDestinationPort: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CLASSIFY_DROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP2")
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .field("vSwitchId", &self.vSwitchId)
            .field("vSwitchSourcePort", &self.vSwitchSourcePort)
            .field("vSwitchDestinationPort", &self.vSwitchDestinationPort)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CLASSIFY_DROP2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    pub localMacAddr: FWP_BYTE_ARRAY6,
    pub remoteMacAddr: FWP_BYTE_ARRAY6,
    pub mediaType: u32,
    pub ifType: u32,
    pub etherType: u16,
    pub ndisPortNumber: u32,
    pub reserved: u32,
    pub vlanTag: u16,
    pub ifLuid: u64,
    pub filterId: u64,
    pub layerId: u16,
    pub reauthReason: u32,
    pub originalProfile: u32,
    pub currentProfile: u32,
    pub msFwpDirection: u32,
    pub isLoopback: super::super::Foundation::BOOL,
    pub vSwitchId: FWP_BYTE_BLOB,
    pub vSwitchSourcePort: u32,
    pub vSwitchDestinationPort: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP_MAC0")
            .field("localMacAddr", &self.localMacAddr)
            .field("remoteMacAddr", &self.remoteMacAddr)
            .field("mediaType", &self.mediaType)
            .field("ifType", &self.ifType)
            .field("etherType", &self.etherType)
            .field("ndisPortNumber", &self.ndisPortNumber)
            .field("reserved", &self.reserved)
            .field("vlanTag", &self.vlanTag)
            .field("ifLuid", &self.ifLuid)
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .field("vSwitchId", &self.vSwitchId)
            .field("vSwitchSourcePort", &self.vSwitchSourcePort)
            .field("vSwitchDestinationPort", &self.vSwitchDestinationPort)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_CLASSIFY_DROP_MAC0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_ENUM_TEMPLATE0").field("startTime", &self.startTime).field("endTime", &self.endTime).field("numFilterConditions", &self.numFilterConditions).field("filterCondition", &self.filterCondition).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_ENUM_TEMPLATE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_APP_ID_SET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_EFFECTIVE_NAME_SET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_ENTERPRISE_ID_SET: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_IP_PROTOCOL_SET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_IP_VERSION_SET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_LOCAL_ADDR_SET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_LOCAL_PORT_SET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_PACKAGE_ID_SET: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_POLICY_FLAGS_SET: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_REAUTH_REASON_SET: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_REMOTE_ADDR_SET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_REMOTE_PORT_SET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_SCOPE_ID_SET: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_FLAG_USER_ID_SET: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER0 {
    pub timeStamp: super::super::Foundation::FILETIME,
    pub flags: u32,
    pub ipVersion: FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous1: FWPM_NET_EVENT_HEADER0_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER0_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: FWP_BYTE_BLOB,
    pub userId: *mut super::super::Security::SID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER0_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER0_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER0_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER0_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER1 {
    pub timeStamp: super::super::Foundation::FILETIME,
    pub flags: u32,
    pub ipVersion: FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous1: FWPM_NET_EVENT_HEADER1_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER1_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: FWP_BYTE_BLOB,
    pub userId: *mut super::super::Security::SID,
    pub Anonymous3: FWPM_NET_EVENT_HEADER1_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER1_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER1_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER1_2 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER1_2_0 {
    pub reserved1: FWP_AF,
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER1_2_0_0 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_2_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_2_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_2_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    pub reserved2: FWP_BYTE_ARRAY6,
    pub reserved3: FWP_BYTE_ARRAY6,
    pub reserved4: u32,
    pub reserved5: u32,
    pub reserved6: u16,
    pub reserved7: u32,
    pub reserved8: u32,
    pub reserved9: u16,
    pub reserved10: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER1_2_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_HEADER1_2_0_0_0").field("reserved2", &self.reserved2).field("reserved3", &self.reserved3).field("reserved4", &self.reserved4).field("reserved5", &self.reserved5).field("reserved6", &self.reserved6).field("reserved7", &self.reserved7).field("reserved8", &self.reserved8).field("reserved9", &self.reserved9).field("reserved10", &self.reserved10).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER1_2_0_0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER2 {
    pub timeStamp: super::super::Foundation::FILETIME,
    pub flags: u32,
    pub ipVersion: FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous1: FWPM_NET_EVENT_HEADER2_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER2_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: FWP_BYTE_BLOB,
    pub userId: *mut super::super::Security::SID,
    pub addressFamily: FWP_AF,
    pub packageSid: *mut super::super::Security::SID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER2_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER2_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER2_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_HEADER3 {
    pub timeStamp: super::super::Foundation::FILETIME,
    pub flags: u32,
    pub ipVersion: FWP_IP_VERSION,
    pub ipProtocol: u8,
    pub Anonymous1: FWPM_NET_EVENT_HEADER3_0,
    pub Anonymous2: FWPM_NET_EVENT_HEADER3_1,
    pub localPort: u16,
    pub remotePort: u16,
    pub scopeId: u32,
    pub appId: FWP_BYTE_BLOB,
    pub userId: *mut super::super::Security::SID,
    pub addressFamily: FWP_AF,
    pub packageSid: *mut super::super::Security::SID,
    pub enterpriseId: ::windows::core::PWSTR,
    pub policyFlags: u64,
    pub effectiveName: FWP_BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER3_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER3_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER3_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER3_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER3_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_HEADER3_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_HEADER3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_HEADER3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_HEADER3_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_HEADER3_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_HEADER3_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_HEADER3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub emState: IKEEXT_EM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub emAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub qmFilterId: u64,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE0").field("failureErrorCode", &self.failureErrorCode).field("failurePoint", &self.failurePoint).field("flags", &self.flags).field("emState", &self.emState).field("saRole", &self.saRole).field("emAuthMethod", &self.emAuthMethod).field("endCertHash", &self.endCertHash).field("mmId", &self.mmId).field("qmFilterId", &self.qmFilterId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_EM_FAILURE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub emState: IKEEXT_EM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub emAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub qmFilterId: u64,
    pub localPrincipalNameForAuth: ::windows::core::PWSTR,
    pub remotePrincipalNameForAuth: ::windows::core::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut ::windows::core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut ::windows::core::PWSTR,
    pub saTrafficType: IPSEC_TRAFFIC_TYPE,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE1")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("emState", &self.emState)
            .field("saRole", &self.saRole)
            .field("emAuthMethod", &self.emAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("qmFilterId", &self.qmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("saTrafficType", &self.saTrafficType)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_EM_FAILURE1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_BENIGN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_MULTIPLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub mmState: IKEEXT_MM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub mmAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE0").field("failureErrorCode", &self.failureErrorCode).field("failurePoint", &self.failurePoint).field("flags", &self.flags).field("keyingModuleType", &self.keyingModuleType).field("mmState", &self.mmState).field("saRole", &self.saRole).field("mmAuthMethod", &self.mmAuthMethod).field("endCertHash", &self.endCertHash).field("mmId", &self.mmId).field("mmFilterId", &self.mmFilterId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_MM_FAILURE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub mmState: IKEEXT_MM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub mmAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
    pub localPrincipalNameForAuth: ::windows::core::PWSTR,
    pub remotePrincipalNameForAuth: ::windows::core::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut ::windows::core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE1")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("keyingModuleType", &self.keyingModuleType)
            .field("mmState", &self.mmState)
            .field("saRole", &self.saRole)
            .field("mmAuthMethod", &self.mmAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("mmFilterId", &self.mmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_MM_FAILURE1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub flags: u32,
    pub keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub mmState: IKEEXT_MM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub mmAuthMethod: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub endCertHash: [u8; 20],
    pub mmId: u64,
    pub mmFilterId: u64,
    pub localPrincipalNameForAuth: ::windows::core::PWSTR,
    pub remotePrincipalNameForAuth: ::windows::core::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut ::windows::core::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut ::windows::core::PWSTR,
    pub providerContextKey: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("keyingModuleType", &self.keyingModuleType)
            .field("mmState", &self.mmState)
            .field("saRole", &self.saRole)
            .field("mmAuthMethod", &self.mmAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("mmFilterId", &self.mmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("providerContextKey", &self.providerContextKey)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_BENIGN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_MULTIPLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub qmState: IKEEXT_QM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub saTrafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous1: FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0,
    pub Anonymous2: FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1,
    pub qmFilterId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    pub failureErrorCode: u32,
    pub failurePoint: IPSEC_FAILURE_POINT,
    pub keyingModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub qmState: IKEEXT_QM_SA_STATE,
    pub saRole: IKEEXT_SA_ROLE,
    pub saTrafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous1: FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0,
    pub Anonymous2: FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1,
    pub qmFilterId: u64,
    pub mmSaLuid: u64,
    pub mmProviderContextKey: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0,
    pub Anonymous2: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1,
    pub failureStatus: i32,
    pub direction: FWP_DIRECTION,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IPSEC_DOSP_DROP0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    pub publicHostV4Addr: u32,
    pub publicHostV6Addr: [u8; 16],
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    pub internalHostV4Addr: u32,
    pub internalHostV6Addr: [u8; 16],
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    pub failureStatus: i32,
    pub direction: FWP_DIRECTION,
    pub spi: u32,
    pub filterId: u64,
    pub layerId: u16,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {}
impl ::core::clone::Clone for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IPSEC_KERNEL_DROP0").field("failureStatus", &self.failureStatus).field("direction", &self.direction).field("spi", &self.spi).field("filterId", &self.filterId).field("layerId", &self.layerId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_IPSEC_KERNEL_DROP0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_ALLOW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_DROP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_CLASSIFY_ALLOW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_BCAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_MCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_KEYWORD_PORT_SCANNING_DROP: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    pub spi: u32,
}
impl ::core::marker::Copy for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {}
impl ::core::clone::Clone for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_").field("spi", &self.spi).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {}
impl ::core::default::Default for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_NET_EVENT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_NET_EVENT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_SUBSCRIPTION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_NET_EVENT_SUBSCRIPTION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_NET_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_LPM_PACKET_ARRIVAL: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_NET_EVENT_TYPE_MAX: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(11i32);
impl ::core::marker::Copy for FWPM_NET_EVENT_TYPE {}
impl ::core::clone::Clone for FWPM_NET_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_NET_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_NET_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER0 {
    pub providerKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerData: FWP_BYTE_BLOB,
    pub serviceName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for FWPM_PROVIDER0 {}
impl ::core::clone::Clone for FWPM_PROVIDER0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER0").field("providerKey", &self.providerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerData", &self.providerData).field("serviceName", &self.serviceName).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER0 {}
impl ::core::default::Default for FWPM_PROVIDER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_PROVIDER_CHANGE0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CHANGE0").field("changeType", &self.changeType).field("providerKey", &self.providerKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CHANGE0 {}
impl ::core::default::Default for FWPM_PROVIDER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_PROVIDER_CHANGE_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const FWPM_PROVIDER_CHANGE0)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT0 {
    pub providerContextKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT0_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_PROVIDER_CONTEXT0_0 {
    pub keyingPolicy: *mut IPSEC_KEYING_POLICY0,
    pub ikeQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY0,
    pub ikeQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY0,
    pub authipQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY0,
    pub authipQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY0,
    pub ikeMmPolicy: *mut IKEEXT_POLICY0,
    pub authIpMmPolicy: *mut IKEEXT_POLICY0,
    pub dataBuffer: *mut FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT1 {
    pub providerContextKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT1_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_PROVIDER_CONTEXT1_0 {
    pub keyingPolicy: *mut IPSEC_KEYING_POLICY0,
    pub ikeQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY1,
    pub ikeQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY1,
    pub authipQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY1,
    pub authipQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY1,
    pub ikeMmPolicy: *mut IKEEXT_POLICY1,
    pub authIpMmPolicy: *mut IKEEXT_POLICY1,
    pub dataBuffer: *mut FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY1,
    pub ikeV2MmPolicy: *mut IKEEXT_POLICY1,
    pub idpOptions: *mut IPSEC_DOSP_OPTIONS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT2 {
    pub providerContextKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT2_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_PROVIDER_CONTEXT2_0 {
    pub keyingPolicy: *mut IPSEC_KEYING_POLICY1,
    pub ikeQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub ikeQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY2,
    pub authipQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub authipQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY2,
    pub ikeMmPolicy: *mut IKEEXT_POLICY2,
    pub authIpMmPolicy: *mut IKEEXT_POLICY2,
    pub dataBuffer: *mut FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY2,
    pub ikeV2QmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub ikeV2MmPolicy: *mut IKEEXT_POLICY2,
    pub idpOptions: *mut IPSEC_DOSP_OPTIONS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT2_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT2_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT3_ {
    pub providerContextKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT3__0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT3_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT3_ {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT3_ {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT3_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT3_>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT3_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT3_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_PROVIDER_CONTEXT3__0 {
    pub keyingPolicy: *mut IPSEC_KEYING_POLICY1,
    pub ikeQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub ikeQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY3_,
    pub authipQmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub authipQmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY3_,
    pub ikeMmPolicy: *mut IKEEXT_POLICY2,
    pub authIpMmPolicy: *mut IKEEXT_POLICY2,
    pub dataBuffer: *mut FWP_BYTE_BLOB,
    pub classifyOptions: *mut FWPM_CLASSIFY_OPTIONS0,
    pub ikeV2QmTunnelPolicy: *mut IPSEC_TUNNEL_POLICY3_,
    pub ikeV2QmTransportPolicy: *mut IPSEC_TRANSPORT_POLICY2,
    pub ikeV2MmPolicy: *mut IKEEXT_POLICY2,
    pub idpOptions: *mut IPSEC_DOSP_OPTIONS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT3__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT3__0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT3__0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT3__0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT3__0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT3__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT3__0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_CONTEXT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerContextKey: ::windows::core::GUID,
    pub providerContextId: u64,
}
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT_CHANGE0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_CHANGE0").field("changeType", &self.changeType).field("providerContextKey", &self.providerContextKey).field("providerContextId", &self.providerContextId).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_CHANGE0 {}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const FWPM_PROVIDER_CONTEXT_CHANGE0)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::core::GUID,
    pub providerContextType: FWPM_PROVIDER_CONTEXT_TYPE,
}
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("providerContextType", &self.providerContextType).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_PROVIDER_CONTEXT_FLAG_DOWNLEVEL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_PROVIDER_CONTEXT_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_AUTHIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb25ea800_0d02_46ed_92bd_7fa84bb73e9d);
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_IPSEC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c2d4144_f8e0_42c0_94ce_7ccfc63b2f9b);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub flags: FWPM_SUBSCRIPTION_FLAGS,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_PROVIDER_CONTEXT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_KEYING_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKE_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_AUTHIP_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_CLASSIFY_OPTIONS_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_GENERAL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKEV2_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_DOSP_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_PROVIDER_CONTEXT_TYPE_MAX: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(13i32);
impl ::core::marker::Copy for FWPM_PROVIDER_CONTEXT_TYPE {}
impl ::core::clone::Clone for FWPM_PROVIDER_CONTEXT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_CONTEXT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_PROVIDER_CONTEXT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl ::core::marker::Copy for FWPM_PROVIDER_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_PROVIDER_FLAG_DISABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_PROVIDER_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_PROVIDER_IKEEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ad9216_ccde_456c_8b16_e9f04e60a90b);
pub const FWPM_PROVIDER_IPSEC_DOSP_CONFIG: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c6c05a9_c05c_4bb9_8338_2327814ce8bf);
pub const FWPM_PROVIDER_MPSSVC_EDP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa90296f7_46b8_4457_8f84_b05e05d3c622);
pub const FWPM_PROVIDER_MPSSVC_TENANT_RESTRICTIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0718ff9_44da_4f50_9dc2_c963a4247613);
pub const FWPM_PROVIDER_MPSSVC_WF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdecc16ca_3f33_4346_be1e_8fb4ae0f3d62);
pub const FWPM_PROVIDER_MPSSVC_WSH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b153735_1049_4480_aab4_d1b9bdc03710);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_PROVIDER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_PROVIDER_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_PROVIDER_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_PROVIDER_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_PROVIDER_TCP_CHIMNEY_OFFLOAD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x896aa19e_9a34_4bcb_ae79_beb9127c84b9);
pub const FWPM_PROVIDER_TCP_TEMPLATES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76cfcd30_3394_432d_bed3_441ae50e63c3);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_SERVICE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SERVICE_STOPPED: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SERVICE_START_PENDING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SERVICE_STOP_PENDING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SERVICE_RUNNING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SERVICE_STATE_MAX: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(4i32);
impl ::core::marker::Copy for FWPM_SERVICE_STATE {}
impl ::core::clone::Clone for FWPM_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_SERVICE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_SESSION0 {
    pub sessionKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub txnWaitTimeoutInMSec: u32,
    pub processId: u32,
    pub sid: *mut super::super::Security::SID,
    pub username: ::windows::core::PWSTR,
    pub kernelMode: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWPM_SESSION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWPM_SESSION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_SESSION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SESSION0").field("sessionKey", &self.sessionKey).field("displayData", &self.displayData).field("flags", &self.flags).field("txnWaitTimeoutInMSec", &self.txnWaitTimeoutInMSec).field("processId", &self.processId).field("sid", &self.sid).field("username", &self.username).field("kernelMode", &self.kernelMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWPM_SESSION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_SESSION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SESSION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_SESSION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_SESSION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SESSION_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl ::core::marker::Copy for FWPM_SESSION_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SESSION_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SESSION_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SESSION_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SESSION_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SESSION_FLAG_DYNAMIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SESSION_FLAG_RESERVED: u32 = 268435456u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_STATISTICS0 {
    pub numLayerStatistics: u32,
    pub layerStatistics: *mut FWPM_LAYER_STATISTICS0,
    pub inboundAllowedConnectionsV4: u32,
    pub inboundBlockedConnectionsV4: u32,
    pub outboundAllowedConnectionsV4: u32,
    pub outboundBlockedConnectionsV4: u32,
    pub inboundAllowedConnectionsV6: u32,
    pub inboundBlockedConnectionsV6: u32,
    pub outboundAllowedConnectionsV6: u32,
    pub outboundBlockedConnectionsV6: u32,
    pub inboundActiveConnectionsV4: u32,
    pub outboundActiveConnectionsV4: u32,
    pub inboundActiveConnectionsV6: u32,
    pub outboundActiveConnectionsV6: u32,
    pub reauthDirInbound: u64,
    pub reauthDirOutbound: u64,
    pub reauthFamilyV4: u64,
    pub reauthFamilyV6: u64,
    pub reauthProtoOther: u64,
    pub reauthProtoIPv4: u64,
    pub reauthProtoIPv6: u64,
    pub reauthProtoICMP: u64,
    pub reauthProtoICMP6: u64,
    pub reauthProtoUDP: u64,
    pub reauthProtoTCP: u64,
    pub reauthReasonPolicyChange: u64,
    pub reauthReasonNewArrivalInterface: u64,
    pub reauthReasonNewNextHopInterface: u64,
    pub reauthReasonProfileCrossing: u64,
    pub reauthReasonClassifyCompletion: u64,
    pub reauthReasonIPSecPropertiesChanged: u64,
    pub reauthReasonMidStreamInspection: u64,
    pub reauthReasonSocketPropertyChanged: u64,
    pub reauthReasonNewInboundMCastBCastPacket: u64,
    pub reauthReasonEDPPolicyChanged: u64,
    pub reauthReasonProxyHandleChanged: u64,
}
impl ::core::marker::Copy for FWPM_STATISTICS0 {}
impl ::core::clone::Clone for FWPM_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_STATISTICS0")
            .field("numLayerStatistics", &self.numLayerStatistics)
            .field("layerStatistics", &self.layerStatistics)
            .field("inboundAllowedConnectionsV4", &self.inboundAllowedConnectionsV4)
            .field("inboundBlockedConnectionsV4", &self.inboundBlockedConnectionsV4)
            .field("outboundAllowedConnectionsV4", &self.outboundAllowedConnectionsV4)
            .field("outboundBlockedConnectionsV4", &self.outboundBlockedConnectionsV4)
            .field("inboundAllowedConnectionsV6", &self.inboundAllowedConnectionsV6)
            .field("inboundBlockedConnectionsV6", &self.inboundBlockedConnectionsV6)
            .field("outboundAllowedConnectionsV6", &self.outboundAllowedConnectionsV6)
            .field("outboundBlockedConnectionsV6", &self.outboundBlockedConnectionsV6)
            .field("inboundActiveConnectionsV4", &self.inboundActiveConnectionsV4)
            .field("outboundActiveConnectionsV4", &self.outboundActiveConnectionsV4)
            .field("inboundActiveConnectionsV6", &self.inboundActiveConnectionsV6)
            .field("outboundActiveConnectionsV6", &self.outboundActiveConnectionsV6)
            .field("reauthDirInbound", &self.reauthDirInbound)
            .field("reauthDirOutbound", &self.reauthDirOutbound)
            .field("reauthFamilyV4", &self.reauthFamilyV4)
            .field("reauthFamilyV6", &self.reauthFamilyV6)
            .field("reauthProtoOther", &self.reauthProtoOther)
            .field("reauthProtoIPv4", &self.reauthProtoIPv4)
            .field("reauthProtoIPv6", &self.reauthProtoIPv6)
            .field("reauthProtoICMP", &self.reauthProtoICMP)
            .field("reauthProtoICMP6", &self.reauthProtoICMP6)
            .field("reauthProtoUDP", &self.reauthProtoUDP)
            .field("reauthProtoTCP", &self.reauthProtoTCP)
            .field("reauthReasonPolicyChange", &self.reauthReasonPolicyChange)
            .field("reauthReasonNewArrivalInterface", &self.reauthReasonNewArrivalInterface)
            .field("reauthReasonNewNextHopInterface", &self.reauthReasonNewNextHopInterface)
            .field("reauthReasonProfileCrossing", &self.reauthReasonProfileCrossing)
            .field("reauthReasonClassifyCompletion", &self.reauthReasonClassifyCompletion)
            .field("reauthReasonIPSecPropertiesChanged", &self.reauthReasonIPSecPropertiesChanged)
            .field("reauthReasonMidStreamInspection", &self.reauthReasonMidStreamInspection)
            .field("reauthReasonSocketPropertyChanged", &self.reauthReasonSocketPropertyChanged)
            .field("reauthReasonNewInboundMCastBCastPacket", &self.reauthReasonNewInboundMCastBCastPacket)
            .field("reauthReasonEDPPolicyChanged", &self.reauthReasonEDPPolicyChanged)
            .field("reauthReasonProxyHandleChanged", &self.reauthReasonProxyHandleChanged)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_STATISTICS0 {}
impl ::core::default::Default for FWPM_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SUBLAYER0 {
    pub subLayerKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::core::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub weight: u16,
}
impl ::core::marker::Copy for FWPM_SUBLAYER0 {}
impl ::core::clone::Clone for FWPM_SUBLAYER0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SUBLAYER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER0").field("subLayerKey", &self.subLayerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerKey", &self.providerKey).field("providerData", &self.providerData).field("weight", &self.weight).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SUBLAYER0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SUBLAYER0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER0 {}
impl ::core::default::Default for FWPM_SUBLAYER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SUBLAYER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub subLayerKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_SUBLAYER_CHANGE0 {}
impl ::core::clone::Clone for FWPM_SUBLAYER_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SUBLAYER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_CHANGE0").field("changeType", &self.changeType).field("subLayerKey", &self.subLayerKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SUBLAYER_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SUBLAYER_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_CHANGE0 {}
impl ::core::default::Default for FWPM_SUBLAYER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_SUBLAYER_CHANGE_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const FWPM_SUBLAYER_CHANGE0)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_SUBLAYER_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SUBLAYER_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {}
impl ::core::default::Default for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SUBLAYER_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_SUBLAYER_INSPECTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877519e1_e6a9_41a5_81b4_8c4f118e4a60);
pub const FWPM_SUBLAYER_IPSEC_DOSP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe076d572_5d3d_48ef_802b_909eddb098bd);
pub const FWPM_SUBLAYER_IPSEC_FORWARD_OUTBOUND_TUNNEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5082e73_8f71_4559_8a9a_101cea04ef87);
pub const FWPM_SUBLAYER_IPSEC_SECURITY_REALM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37a57701_5884_4964_92b8_3e704688b0ad);
pub const FWPM_SUBLAYER_IPSEC_TUNNEL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83f299ed_9ff4_4967_aff4_c309f4dab827);
pub const FWPM_SUBLAYER_LIPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b75c0ce_ff60_4711_a70f_b4958cc3b2d0);
pub const FWPM_SUBLAYER_MPSSVC_EDP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09a47e38_fa97_471b_b123_18bcd7e65071);
pub const FWPM_SUBLAYER_MPSSVC_QUARANTINE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2302);
pub const FWPM_SUBLAYER_MPSSVC_TENANT_RESTRICTIONS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec6c7e1_fdd9_478a_b55f_ff8ba1d2c17d);
pub const FWPM_SUBLAYER_MPSSVC_WF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2301);
pub const FWPM_SUBLAYER_MPSSVC_WSH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3cdd441_af90_41ba_a745_7c6008ff2300);
pub const FWPM_SUBLAYER_RPC_AUDIT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x758c84f4_fb48_4de9_9aeb_3ed9551ab1fd);
pub const FWPM_SUBLAYER_SECURE_SOCKET: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15a66e17_3f3c_4f7b_aa6c_812aa613dd82);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SUBLAYER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_SUBLAYER_ENUM_TEMPLATE0,
    pub flags: FWPM_SUBSCRIPTION_FLAGS,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_SUBLAYER_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SUBLAYER_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SUBLAYER_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const FWPM_SUBLAYER_TCP_CHIMNEY_OFFLOAD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x337608b9_b7d5_4d5f_82f9_3618618bc058);
pub const FWPM_SUBLAYER_TCP_TEMPLATES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24421dcf_0ac5_4caa_9e14_50f6e3636af0);
pub const FWPM_SUBLAYER_TEREDO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba69dc66_5176_4979_9c89_26a7b46a8327);
pub const FWPM_SUBLAYER_UNIVERSAL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeebecc03_ced4_4380_819a_2734397b2b74);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_SUBSCRIPTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_ADD: FWPM_SUBSCRIPTION_FLAGS = FWPM_SUBSCRIPTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_DELETE: FWPM_SUBSCRIPTION_FLAGS = FWPM_SUBSCRIPTION_FLAGS(2u32);
impl ::core::marker::Copy for FWPM_SUBSCRIPTION_FLAGS {}
impl ::core::clone::Clone for FWPM_SUBSCRIPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_SUBSCRIPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_SUBSCRIPTION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_SUBSCRIPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SUBSCRIPTION_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SYSTEM_PORTS0 {
    pub numTypes: u32,
    pub types: *mut FWPM_SYSTEM_PORTS_BY_TYPE0,
}
impl ::core::marker::Copy for FWPM_SYSTEM_PORTS0 {}
impl ::core::clone::Clone for FWPM_SYSTEM_PORTS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORTS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SYSTEM_PORTS0").field("numTypes", &self.numTypes).field("types", &self.types).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SYSTEM_PORTS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SYSTEM_PORTS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SYSTEM_PORTS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SYSTEM_PORTS0 {}
impl ::core::default::Default for FWPM_SYSTEM_PORTS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_SYSTEM_PORTS_BY_TYPE0 {
    pub r#type: FWPM_SYSTEM_PORT_TYPE,
    pub numPorts: u32,
    pub ports: *mut u16,
}
impl ::core::marker::Copy for FWPM_SYSTEM_PORTS_BY_TYPE0 {}
impl ::core::clone::Clone for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SYSTEM_PORTS_BY_TYPE0").field("type", &self.r#type).field("numPorts", &self.numPorts).field("ports", &self.ports).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_SYSTEM_PORTS_BY_TYPE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_SYSTEM_PORTS_BY_TYPE0 {}
impl ::core::default::Default for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type FWPM_SYSTEM_PORTS_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, sysports: *const FWPM_SYSTEM_PORTS0)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_SYSTEM_PORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SYSTEM_PORT_RPC_EPMAP: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SYSTEM_PORT_TEREDO: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SYSTEM_PORT_IPHTTPS_IN: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SYSTEM_PORT_IPHTTPS_OUT: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_SYSTEM_PORT_TYPE_MAX: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(4i32);
impl ::core::marker::Copy for FWPM_SYSTEM_PORT_TYPE {}
impl ::core::clone::Clone for FWPM_SYSTEM_PORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_SYSTEM_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_SYSTEM_PORT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SYSTEM_PORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_TUNNEL_FLAG_ENABLE_VIRTUAL_IF_TUNNELING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_TUNNEL_FLAG_POINT_TO_POINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_TUNNEL_FLAG_RESERVED0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_TXN_READ_ONLY: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0 {
    pub eventType: FWPM_VSWITCH_EVENT_TYPE,
    pub vSwitchId: ::windows::core::PWSTR,
    pub Anonymous: FWPM_VSWITCH_EVENT0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_VSWITCH_EVENT0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_VSWITCH_EVENT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union FWPM_VSWITCH_EVENT0_0 {
    pub positionInfo: FWPM_VSWITCH_EVENT0_0_0,
    pub reorderInfo: FWPM_VSWITCH_EVENT0_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_VSWITCH_EVENT0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_VSWITCH_EVENT0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0_0_0 {
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_VSWITCH_EVENT0_0_0").field("numvSwitchFilterExtensions", &self.numvSwitchFilterExtensions).field("vSwitchFilterExtensions", &self.vSwitchFilterExtensions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT0_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_VSWITCH_EVENT0_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_VSWITCH_EVENT0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0_0_1 {
    pub inRequiredPosition: super::super::Foundation::BOOL,
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT0_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT0_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT0_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_VSWITCH_EVENT0_0_1").field("inRequiredPosition", &self.inRequiredPosition).field("numvSwitchFilterExtensions", &self.numvSwitchFilterExtensions).field("vSwitchFilterExtensions", &self.vSwitchFilterExtensions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT0_0_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_VSWITCH_EVENT0_0_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT0_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_VSWITCH_EVENT0_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type FWPM_VSWITCH_EVENT_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, vswitchevent: *const FWPM_VSWITCH_EVENT0) -> u32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {}
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_VSWITCH_EVENT_SUBSCRIPTION0").field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWPM_VSWITCH_EVENT_SUBSCRIPTION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {}
impl ::core::default::Default for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWPM_VSWITCH_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_VSWITCH_EVENT_MAX: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(5i32);
impl ::core::marker::Copy for FWPM_VSWITCH_EVENT_TYPE {}
impl ::core::clone::Clone for FWPM_VSWITCH_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWPM_VSWITCH_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWPM_VSWITCH_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_VSWITCH_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_WEIGHT_RANGE_IKE_EXEMPTIONS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPM_WEIGHT_RANGE_IPSEC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_ALE_ENDPOINT_FLAG_IPSEC_SECURED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_CLASSIFY_OUT_FLAG_ABSORB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_CHECK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_POSSIBLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_CLASSIFY_OUT_FLAG_BUFFER_LIMIT_REACHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_CLASSIFY_OUT_FLAG_NO_MORE_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_CLEAR_ACTION_RIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_OR_CONDITIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_RESERVED0: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_RESERVED1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_FILTER_FLAG_SILENT_MODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_ABSORB: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_CACHE_SAFE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_CONNECTION_FAILING_INDICATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_ENFORCE_QUERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_IS_LOCAL_ONLY_FLOW: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_IS_LOOSE_SOURCE_FLOW: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_MID_STREAM_INSPECTION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_RECLASSIFY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_INCOMING_FLAG_RESERVED0: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_INCOMING_FLAG_IS_RAW_IPV4_FRAMING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_INCOMING_FLAG_IS_RAW_IPV6_FRAMING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_INCOMING_FLAG_RECLASSIFY_MULTI_DESTINATION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_ETHERNET_MAC_HEADER_SIZE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_VSWITCH_DESTINATION_PORT_ID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_VSWITCH_PACKET_CONTEXT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_VSWITCH_SOURCE_NIC_INDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_VSWITCH_SOURCE_PORT_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_L2_METADATA_FIELD_WIFI_OPERATION_MODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_ALE_CLASSIFY_REQUIRED: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_COMPARTMENT_ID: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_COMPLETION_HANDLE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_DESTINATION_INTERFACE_INDEX: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_DESTINATION_PREFIX: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_DISCARD_REASON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_ETHER_FRAME_LENGTH: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_FLOW_HANDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_FORWARD_LAYER_INBOUND_PASS_THRU: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_FORWARD_LAYER_OUTBOUND_PASS_THRU: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_FRAGMENT_DATA: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_ICMP_ID_AND_SEQUENCE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_IP_HEADER_SIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_LOCAL_REDIRECT_TARGET_PID: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_ORIGINAL_DESTINATION: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PACKET_DIRECTION: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PACKET_SYSTEM_CRITICAL: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PARENT_ENDPOINT_HANDLE: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PATH_MTU: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PROCESS_ID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_PROCESS_PATH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_REDIRECT_RECORD_HANDLE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_REMOTE_SCOPE_ID: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_RESERVED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_SOURCE_INTERFACE_INDEX: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_SUB_PROCESS_TAG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_SYSTEM_FLAGS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_TOKEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_TRANSPORT_CONTROL_DATA: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_TRANSPORT_ENDPOINT_HANDLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_TRANSPORT_HEADER_INCLUDE_HEADER: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_METADATA_FIELD_TRANSPORT_HEADER_SIZE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWPS_RIGHT_ACTION_WRITE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTION_FLAG_CALLOUT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTION_FLAG_NON_TERMINATING: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTION_FLAG_TERMINATING: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTION_NONE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTION_NONE_NO_MATCH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ACTRL_MATCH_FILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_AF(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_AF_INET: FWP_AF = FWP_AF(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_AF_INET6: FWP_AF = FWP_AF(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_AF_ETHER: FWP_AF = FWP_AF(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_AF_NONE: FWP_AF = FWP_AF(3i32);
impl ::core::marker::Copy for FWP_AF {}
impl ::core::clone::Clone for FWP_AF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_AF {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_AF {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_AF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_AF").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_BYTEMAP_ARRAY64_SIZE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWP_BYTE_ARRAY16 {
    pub byteArray16: [u8; 16],
}
impl ::core::marker::Copy for FWP_BYTE_ARRAY16 {}
impl ::core::clone::Clone for FWP_BYTE_ARRAY16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWP_BYTE_ARRAY16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_ARRAY16").field("byteArray16", &self.byteArray16).finish()
    }
}
unsafe impl ::windows::core::Abi for FWP_BYTE_ARRAY16 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWP_BYTE_ARRAY16 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_BYTE_ARRAY16>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWP_BYTE_ARRAY16 {}
impl ::core::default::Default for FWP_BYTE_ARRAY16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWP_BYTE_ARRAY6 {
    pub byteArray6: [u8; 6],
}
impl ::core::marker::Copy for FWP_BYTE_ARRAY6 {}
impl ::core::clone::Clone for FWP_BYTE_ARRAY6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWP_BYTE_ARRAY6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_ARRAY6").field("byteArray6", &self.byteArray6).finish()
    }
}
unsafe impl ::windows::core::Abi for FWP_BYTE_ARRAY6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWP_BYTE_ARRAY6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_BYTE_ARRAY6>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWP_BYTE_ARRAY6 {}
impl ::core::default::Default for FWP_BYTE_ARRAY6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_BYTE_ARRAY6_SIZE: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWP_BYTE_BLOB {
    pub size: u32,
    pub data: *mut u8,
}
impl ::core::marker::Copy for FWP_BYTE_BLOB {}
impl ::core::clone::Clone for FWP_BYTE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWP_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_BLOB").field("size", &self.size).field("data", &self.data).finish()
    }
}
unsafe impl ::windows::core::Abi for FWP_BYTE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWP_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_BYTE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWP_BYTE_BLOB {}
impl ::core::default::Default for FWP_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_L2_BATCH_CLASSIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_MID_STREAM_INSPECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_OFFLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_RECLASSIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_RSC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_URO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ALLOW_USO: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_CONDITIONAL_ON_FLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_ENABLE_COMMIT_ADD_NOTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CALLOUT_FLAG_RESERVED2: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_CLASSIFY_OPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_MULTICAST_STATE: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_LOOSE_SOURCE_MAPPING: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_UNICAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_MCAST_BCAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_SECURITY_FLAGS: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_MM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_QM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_LOCAL_ONLY_MAPPING: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CLASSIFY_OPTION_MAX: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(8i32);
impl ::core::marker::Copy for FWP_CLASSIFY_OPTION_TYPE {}
impl ::core::clone::Clone for FWP_CLASSIFY_OPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_CLASSIFY_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_CLASSIFY_OPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_CLASSIFY_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_CLASSIFY_OPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_APPCONTAINER_LOOPBACK: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_AUTH_FW: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_CONNECTION_REDIRECTED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_FRAGMENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_FRAGMENT_GROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_HONORING_POLICY_AUTHORIZE: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_IMPLICIT_BIND: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_INBOUND_PASS_THRU: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_IPSEC_NATT_RECLASSIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_IPSEC_SECURED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_LOOPBACK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_NAME_APP_SPECIFIED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_NON_APPCONTAINER_LOOPBACK: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_OUTBOUND_PASS_THRU: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_PROMISCUOUS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_PROXY_CONNECTION: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_RAW_ENDPOINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_REASSEMBLED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_REAUTHORIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_RECLASSIFY: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_RESERVED: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_IS_WILDCARD_BIND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_FLAG_REQUIRES_ALE_CLASSIFY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IF_CONNECTOR_PRESENT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_IP_FRAGMENT_GROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_MALFORMED_PACKET: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_MOBILE_BROADBAND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_NATIVE_ETHERNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_VM2VM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_WIFI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_L2_IS_WIFI_DIRECT_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_CHECK_OFFLOAD: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_CLASSIFY_COMPLETION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_EDP_POLICY_CHANGED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_IPSEC_PROPERTIES_CHANGED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_MID_STREAM_INSPECTION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_ARRIVAL_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_INBOUND_MCAST_BCAST_PACKET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_NEXTHOP_INTERFACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_POLICY_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROFILE_CROSSING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROXY_HANDLE_CHANGED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_REAUTHORIZE_REASON_SOCKET_PROPERTY_CHANGED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_ALLOW_EDGE_TRAFFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_DENY_EDGE_TRAFFIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_IS_SYSTEM_PORT_RPC: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_CONDITION_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_CONDITION_VALUE0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_CONDITION_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_CONDITION_VALUE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_CONDITION_VALUE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_CONDITION_VALUE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_CONDITION_VALUE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_CONDITION_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_CONDITION_VALUE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWP_CONDITION_VALUE0_0 {
    pub uint8: u8,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: *mut u64,
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
    pub int64: *mut i64,
    pub float32: f32,
    pub double64: *mut f64,
    pub byteArray16: *mut FWP_BYTE_ARRAY16,
    pub byteBlob: *mut FWP_BYTE_BLOB,
    pub sid: *mut super::super::Security::SID,
    pub sd: *mut FWP_BYTE_BLOB,
    pub tokenInformation: *mut FWP_TOKEN_INFORMATION,
    pub tokenAccessInformation: *mut FWP_BYTE_BLOB,
    pub unicodeString: ::windows::core::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
    pub v4AddrMask: *mut FWP_V4_ADDR_AND_MASK,
    pub v6AddrMask: *mut FWP_V6_ADDR_AND_MASK,
    pub rangeValue: *mut FWP_RANGE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_CONDITION_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_CONDITION_VALUE0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_CONDITION_VALUE0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_CONDITION_VALUE0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_CONDITION_VALUE0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_CONDITION_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_CONDITION_VALUE0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_EMPTY: FWP_DATA_TYPE = FWP_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_UINT8: FWP_DATA_TYPE = FWP_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_UINT16: FWP_DATA_TYPE = FWP_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_UINT32: FWP_DATA_TYPE = FWP_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_UINT64: FWP_DATA_TYPE = FWP_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_INT8: FWP_DATA_TYPE = FWP_DATA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_INT16: FWP_DATA_TYPE = FWP_DATA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_INT32: FWP_DATA_TYPE = FWP_DATA_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_INT64: FWP_DATA_TYPE = FWP_DATA_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FLOAT: FWP_DATA_TYPE = FWP_DATA_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_DOUBLE: FWP_DATA_TYPE = FWP_DATA_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_BYTE_ARRAY16_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_BYTE_BLOB_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_SID: FWP_DATA_TYPE = FWP_DATA_TYPE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_SECURITY_DESCRIPTOR_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_TOKEN_INFORMATION_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_TOKEN_ACCESS_INFORMATION_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_UNICODE_STRING_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_BYTE_ARRAY6_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_SINGLE_DATA_TYPE_MAX: FWP_DATA_TYPE = FWP_DATA_TYPE(255i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_V4_ADDR_MASK: FWP_DATA_TYPE = FWP_DATA_TYPE(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_V6_ADDR_MASK: FWP_DATA_TYPE = FWP_DATA_TYPE(257i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_RANGE_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(258i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_DATA_TYPE_MAX: FWP_DATA_TYPE = FWP_DATA_TYPE(259i32);
impl ::core::marker::Copy for FWP_DATA_TYPE {}
impl ::core::clone::Clone for FWP_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_DIRECTION_OUTBOUND: FWP_DIRECTION = FWP_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_DIRECTION_INBOUND: FWP_DIRECTION = FWP_DIRECTION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_DIRECTION_MAX: FWP_DIRECTION = FWP_DIRECTION(2i32);
impl ::core::marker::Copy for FWP_DIRECTION {}
impl ::core::clone::Clone for FWP_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_DIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_ETHER_ENCAP_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ETHER_ENCAP_METHOD_ETHER_V2: FWP_ETHER_ENCAP_METHOD = FWP_ETHER_ENCAP_METHOD(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ETHER_ENCAP_METHOD_SNAP: FWP_ETHER_ENCAP_METHOD = FWP_ETHER_ENCAP_METHOD(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_ETHER_ENCAP_METHOD_SNAP_W_OUI_ZERO: FWP_ETHER_ENCAP_METHOD = FWP_ETHER_ENCAP_METHOD(3i32);
impl ::core::marker::Copy for FWP_ETHER_ENCAP_METHOD {}
impl ::core::clone::Clone for FWP_ETHER_ENCAP_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_ETHER_ENCAP_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_ETHER_ENCAP_METHOD {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_ETHER_ENCAP_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_ETHER_ENCAP_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_BEST_TERMINATING_MATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_BOOTTIME_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_BOOTTIME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_DISABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FLAG_SORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_FILTER_ENUM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_FULLY_CONTAINED: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_OVERLAPPING: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_FILTER_ENUM_TYPE_MAX: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(2i32);
impl ::core::marker::Copy for FWP_FILTER_ENUM_TYPE {}
impl ::core::clone::Clone for FWP_FILTER_ENUM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_FILTER_ENUM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_FILTER_ENUM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_FILTER_ENUM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_FILTER_ENUM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_IP_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_IP_VERSION_V4: FWP_IP_VERSION = FWP_IP_VERSION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_IP_VERSION_V6: FWP_IP_VERSION = FWP_IP_VERSION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_IP_VERSION_NONE: FWP_IP_VERSION = FWP_IP_VERSION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_IP_VERSION_MAX: FWP_IP_VERSION = FWP_IP_VERSION(3i32);
impl ::core::marker::Copy for FWP_IP_VERSION {}
impl ::core::clone::Clone for FWP_IP_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_IP_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_IP_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_MATCH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_GREATER: FWP_MATCH_TYPE = FWP_MATCH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_LESS: FWP_MATCH_TYPE = FWP_MATCH_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_GREATER_OR_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_LESS_OR_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_RANGE: FWP_MATCH_TYPE = FWP_MATCH_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_FLAGS_ALL_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_FLAGS_ANY_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_FLAGS_NONE_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_EQUAL_CASE_INSENSITIVE: FWP_MATCH_TYPE = FWP_MATCH_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_NOT_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_PREFIX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_NOT_PREFIX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_MATCH_TYPE_MAX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(13i32);
impl ::core::marker::Copy for FWP_MATCH_TYPE {}
impl ::core::clone::Clone for FWP_MATCH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_MATCH_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_MATCH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_ALLOW_GLOBAL_MULTICAST_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_ALLOW_MULTICAST_STATE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_DENY_MULTICAST_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_DISABLE_LOCAL_ONLY_MAPPING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_DISABLE_LOOSE_SOURCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_ENABLE_LOCAL_ONLY_MAPPING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_OPTION_VALUE_ENABLE_LOOSE_SOURCE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_RANGE0 {
    pub valueLow: FWP_VALUE0,
    pub valueHigh: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_RANGE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_RANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_RANGE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_RANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_RANGE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_RANGE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_RANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_TOKEN_INFORMATION {
    pub sidCount: u32,
    pub sids: *mut super::super::Security::SID_AND_ATTRIBUTES,
    pub restrictedSidCount: u32,
    pub restrictedSids: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_TOKEN_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_TOKEN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWP_TOKEN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_TOKEN_INFORMATION").field("sidCount", &self.sidCount).field("sids", &self.sids).field("restrictedSidCount", &self.restrictedSidCount).field("restrictedSids", &self.restrictedSids).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_TOKEN_INFORMATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_TOKEN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_TOKEN_INFORMATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_TOKEN_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_TOKEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWP_V4_ADDR_AND_MASK {
    pub addr: u32,
    pub mask: u32,
}
impl ::core::marker::Copy for FWP_V4_ADDR_AND_MASK {}
impl ::core::clone::Clone for FWP_V4_ADDR_AND_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWP_V4_ADDR_AND_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_V4_ADDR_AND_MASK").field("addr", &self.addr).field("mask", &self.mask).finish()
    }
}
unsafe impl ::windows::core::Abi for FWP_V4_ADDR_AND_MASK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWP_V4_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_V4_ADDR_AND_MASK>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWP_V4_ADDR_AND_MASK {}
impl ::core::default::Default for FWP_V4_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct FWP_V6_ADDR_AND_MASK {
    pub addr: [u8; 16],
    pub prefixLength: u8,
}
impl ::core::marker::Copy for FWP_V6_ADDR_AND_MASK {}
impl ::core::clone::Clone for FWP_V6_ADDR_AND_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FWP_V6_ADDR_AND_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_V6_ADDR_AND_MASK").field("addr", &self.addr).field("prefixLength", &self.prefixLength).finish()
    }
}
unsafe impl ::windows::core::Abi for FWP_V6_ADDR_AND_MASK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FWP_V6_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_V6_ADDR_AND_MASK>()) == 0 }
    }
}
impl ::core::cmp::Eq for FWP_V6_ADDR_AND_MASK {}
impl ::core::default::Default for FWP_V6_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_V6_ADDR_SIZE: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_VALUE0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_VALUE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_VALUE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_VALUE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_VALUE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_VALUE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWP_VALUE0_0 {
    pub uint8: u8,
    pub uint16: u16,
    pub uint32: u32,
    pub uint64: *mut u64,
    pub int8: i8,
    pub int16: i16,
    pub int32: i32,
    pub int64: *mut i64,
    pub float32: f32,
    pub double64: *mut f64,
    pub byteArray16: *mut FWP_BYTE_ARRAY16,
    pub byteBlob: *mut FWP_BYTE_BLOB,
    pub sid: *mut super::super::Security::SID,
    pub sd: *mut FWP_BYTE_BLOB,
    pub tokenInformation: *mut FWP_TOKEN_INFORMATION,
    pub tokenAccessInformation: *mut FWP_BYTE_BLOB,
    pub unicodeString: ::windows::core::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for FWP_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for FWP_VALUE0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for FWP_VALUE0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_VALUE0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FWP_VALUE0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_VALUE0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FWP_VSWITCH_NETWORK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_VSWITCH_NETWORK_TYPE_UNKNOWN: FWP_VSWITCH_NETWORK_TYPE = FWP_VSWITCH_NETWORK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_VSWITCH_NETWORK_TYPE_PRIVATE: FWP_VSWITCH_NETWORK_TYPE = FWP_VSWITCH_NETWORK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_VSWITCH_NETWORK_TYPE_INTERNAL: FWP_VSWITCH_NETWORK_TYPE = FWP_VSWITCH_NETWORK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const FWP_VSWITCH_NETWORK_TYPE_EXTERNAL: FWP_VSWITCH_NETWORK_TYPE = FWP_VSWITCH_NETWORK_TYPE(3i32);
impl ::core::marker::Copy for FWP_VSWITCH_NETWORK_TYPE {}
impl ::core::clone::Clone for FWP_VSWITCH_NETWORK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FWP_VSWITCH_NETWORK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FWP_VSWITCH_NETWORK_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FWP_VSWITCH_NETWORK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_VSWITCH_NETWORK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, callout: *const FWPM_CALLOUT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutAdd0(enginehandle: super::super::Foundation::HANDLE, callout: *const FWPM_CALLOUT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(callout), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_CALLOUT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDeleteById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u32) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutDeleteById0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CALLOUT0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CALLOUT0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u32, callout: *mut *mut FWPM_CALLOUT0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetById0(enginehandle: super::super::Foundation::HANDLE, id: u32, callout: *mut *mut FWPM_CALLOUT0) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(callout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, callout: *mut *mut FWPM_CALLOUT0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, callout: *mut *mut FWPM_CALLOUT0) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(callout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutSubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_CALLOUT_SUBSCRIPTION0, callback: FWPM_CALLOUT_CHANGE_CALLBACK0, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_CALLOUT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutSubscribeChanges0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(changehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_CALLOUT_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_CALLOUT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutUnsubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, changehandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmCalloutUnsubscribeChanges0(enginehandle.into_param().abi(), changehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_CONNECTION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CONNECTION0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_CONNECTION0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, connection: *mut *mut FWPM_CONNECTION0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, connection: *mut *mut FWPM_CONNECTION0) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(connection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionSubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_CONNECTION_SUBSCRIPTION0, callback: FWPM_CONNECTION_CALLBACK0, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_CONNECTION_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionSubscribe0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, eventshandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmConnectionUnsubscribe0(enginehandle.into_param().abi(), eventshandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmDynamicKeywordSubscribe0(flags: u32, callback: FWPM_DYNAMIC_KEYWORD_CALLBACK0, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmDynamicKeywordSubscribe0(flags: u32, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmDynamicKeywordSubscribe0(::core::mem::transmute(flags), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(subscriptionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmDynamicKeywordUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(subscriptionhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmDynamicKeywordUnsubscribe0(subscriptionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmDynamicKeywordUnsubscribe0(subscriptionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmEngineClose0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineClose0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmEngineClose0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetOption0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, option: FWPM_ENGINE_OPTION, value: *mut *mut FWP_VALUE0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineGetOption0(enginehandle: super::super::Foundation::HANDLE, option: FWPM_ENGINE_OPTION, value: *mut *mut FWP_VALUE0) -> u32;
        }
        ::core::mem::transmute(FwpmEngineGetOption0(enginehandle.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmEngineGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_System_Rpc\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Rpc"))]
#[inline]
pub unsafe fn FwpmEngineOpen0<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(servername: Param0, authnservice: u32, authidentity: *const super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W, session: *const FWPM_SESSION0, enginehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineOpen0(servername: ::windows::core::PCWSTR, authnservice: u32, authidentity: *const super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W, session: *const FWPM_SESSION0, enginehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmEngineOpen0(servername.into_param().abi(), ::core::mem::transmute(authnservice), ::core::mem::transmute(authidentity), ::core::mem::transmute(session), ::core::mem::transmute(enginehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetOption0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, option: FWPM_ENGINE_OPTION, newvalue: *const FWP_VALUE0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineSetOption0(enginehandle: super::super::Foundation::HANDLE, option: FWPM_ENGINE_OPTION, newvalue: *const FWP_VALUE0) -> u32;
        }
        ::core::mem::transmute(FwpmEngineSetOption0(enginehandle.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(newvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmEngineSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, filter: *const FWPM_FILTER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterAdd0(enginehandle: super::super::Foundation::HANDLE, filter: *const FWPM_FILTER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(FwpmFilterAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(filter), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_FILTER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_FILTER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmFilterCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDeleteById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::core::mem::transmute(FwpmFilterDeleteById0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmFilterDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmFilterDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_FILTER0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_FILTER0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmFilterEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, filter: *mut *mut FWPM_FILTER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, filter: *mut *mut FWPM_FILTER0) -> u32;
        }
        ::core::mem::transmute(FwpmFilterGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(filter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, filter: *mut *mut FWPM_FILTER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, filter: *mut *mut FWPM_FILTER0) -> u32;
        }
        ::core::mem::transmute(FwpmFilterGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(filter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmFilterGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmFilterSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_FILTER_SUBSCRIPTION0, callback: FWPM_FILTER_CHANGE_CALLBACK0, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_FILTER_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmFilterSubscribeChanges0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(changehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_FILTER_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_FILTER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmFilterSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterUnsubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, changehandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmFilterUnsubscribeChanges0(enginehandle.into_param().abi(), changehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[inline]
pub unsafe fn FwpmFreeMemory0(p: *mut *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFreeMemory0(p: *mut *mut ::core::ffi::c_void);
        }
        FwpmFreeMemory0(::core::mem::transmute(p))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[inline]
pub unsafe fn FwpmGetAppIdFromFileName0<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filename: Param0, appid: *mut *mut FWP_BYTE_BLOB) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmGetAppIdFromFileName0(filename: ::windows::core::PCWSTR, appid: *mut *mut FWP_BYTE_BLOB) -> u32;
        }
        ::core::mem::transmute(FwpmGetAppIdFromFileName0(filename.into_param().abi(), ::core::mem::transmute(appid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT0, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT0, filterconditions: &[FWPM_FILTER_CONDITION0], sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd0(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT0, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT0, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmIPsecTunnelAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(mainmodepolicy), ::core::mem::transmute(tunnelpolicy), filterconditions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(filterconditions)), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT1, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT1, filterconditions: &[FWPM_FILTER_CONDITION0], keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd1(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT1, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT1, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmIPsecTunnelAdd1(enginehandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(mainmodepolicy), ::core::mem::transmute(tunnelpolicy), filterconditions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(filterconditions)), ::core::mem::transmute(keymodkey), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT2, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT2, filterconditions: &[FWPM_FILTER_CONDITION0], keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd2(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT2, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT2, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmIPsecTunnelAdd2(enginehandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(mainmodepolicy), ::core::mem::transmute(tunnelpolicy), filterconditions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(filterconditions)), ::core::mem::transmute(keymodkey), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT3_, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT3_, filterconditions: &[FWPM_FILTER_CONDITION0], keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd3(enginehandle: super::super::Foundation::HANDLE, flags: u32, mainmodepolicy: *const FWPM_PROVIDER_CONTEXT3_, tunnelpolicy: *const FWPM_PROVIDER_CONTEXT3_, numfilterconditions: u32, filterconditions: *const FWPM_FILTER_CONDITION0, keymodkey: *const ::windows::core::GUID, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmIPsecTunnelAdd3(enginehandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(mainmodepolicy), ::core::mem::transmute(tunnelpolicy), filterconditions.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(filterconditions)), ::core::mem::transmute(keymodkey), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmIPsecTunnelDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_LAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_LAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmLayerCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmLayerDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_LAYER0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_LAYER0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmLayerEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u16, layer: *mut *mut FWPM_LAYER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetById0(enginehandle: super::super::Foundation::HANDLE, id: u16, layer: *mut *mut FWPM_LAYER0) -> u32;
        }
        ::core::mem::transmute(FwpmLayerGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(layer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, layer: *mut *mut FWPM_LAYER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, layer: *mut *mut FWPM_LAYER0) -> u32;
        }
        ::core::mem::transmute(FwpmLayerGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(layer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmLayerGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmLayerSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_NET_EVENT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT1, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum1(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT2, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum2(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum3(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT3, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum3(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT4_, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum4(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT4_, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum4(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum5<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT5_, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum5(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_NET_EVENT5_, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventEnum5(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: FWPM_NET_EVENT_CALLBACK0, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscribe0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: FWPM_NET_EVENT_CALLBACK1, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe1(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscribe1(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: FWPM_NET_EVENT_CALLBACK2, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe2(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscribe2(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: FWPM_NET_EVENT_CALLBACK3, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe3(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscribe3(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe4<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: FWPM_NET_EVENT_CALLBACK4, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe4(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscribe4(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_NET_EVENT_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_NET_EVENT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmNetEventUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, eventshandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventUnsubscribe0(enginehandle.into_param().abi(), eventshandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventsGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventsGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventsSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmNetEventsSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, provider: *const FWPM_PROVIDER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderAdd0(enginehandle: super::super::Foundation::HANDLE, provider: *const FWPM_PROVIDER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmProviderAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(provider), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, providercontext: *const FWPM_PROVIDER_CONTEXT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd0(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT0, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(providercontext), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, providercontext: *const FWPM_PROVIDER_CONTEXT1, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd1(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT1, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextAdd1(enginehandle.into_param().abi(), ::core::mem::transmute(providercontext), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, providercontext: *const FWPM_PROVIDER_CONTEXT2, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd2(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT2, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextAdd2(enginehandle.into_param().abi(), ::core::mem::transmute(providercontext), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, providercontext: *const FWPM_PROVIDER_CONTEXT3_, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd3(enginehandle: super::super::Foundation::HANDLE, providercontext: *const FWPM_PROVIDER_CONTEXT3_, sd: *const super::super::Security::SECURITY_DESCRIPTOR, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextAdd3(enginehandle.into_param().abi(), ::core::mem::transmute(providercontext), ::core::mem::transmute(sd), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDeleteById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextDeleteById0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT1, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextEnum1(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT2, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextEnum2(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT3_, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum3(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT3_, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextEnum3(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetById1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById2(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetById2(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById3(enginehandle: super::super::Foundation::HANDLE, id: u64, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetById3(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey1(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetByKey1(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey2(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetByKey2(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey3<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey3(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetByKey3(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(providercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextSubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback: FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextSubscribeChanges0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(changehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextUnsubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, changehandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderContextUnsubscribeChanges0(enginehandle.into_param().abi(), changehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_PROVIDER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmProviderDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_PROVIDER0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, provider: *mut *mut FWPM_PROVIDER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, provider: *mut *mut FWPM_PROVIDER0) -> u32;
        }
        ::core::mem::transmute(FwpmProviderGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(provider)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmProviderGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmProviderSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderSubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_PROVIDER_SUBSCRIPTION0, callback: FWPM_PROVIDER_CHANGE_CALLBACK0, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_PROVIDER_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderSubscribeChanges0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(changehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_PROVIDER_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_PROVIDER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmProviderSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderUnsubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, changehandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmProviderUnsubscribeChanges0(enginehandle.into_param().abi(), changehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_SESSION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_SESSION_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSessionCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSessionDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSessionEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SESSION0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SESSION0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmSessionEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, sublayer: *const FWPM_SUBLAYER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerAdd0(enginehandle: super::super::Foundation::HANDLE, sublayer: *const FWPM_SUBLAYER0, sd: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerAdd0(enginehandle.into_param().abi(), ::core::mem::transmute(sublayer), ::core::mem::transmute(sd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const FWPM_SUBLAYER_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerDeleteByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerDeleteByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut FWPM_SUBLAYER0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, sublayer: *mut *mut FWPM_SUBLAYER0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerGetByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, sublayer: *mut *mut FWPM_SUBLAYER0) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerGetByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(sublayer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, key: *const ::windows::core::GUID, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerSubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_SUBLAYER_SUBSCRIPTION0, callback: FWPM_SUBLAYER_CHANGE_CALLBACK0, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_SUBLAYER_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, changehandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerSubscribeChanges0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(changehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut FWPM_SUBLAYER_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut FWPM_SUBLAYER_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerUnsubscribeChanges0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, changehandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerUnsubscribeChanges0(enginehandle: super::super::Foundation::HANDLE, changehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSubLayerUnsubscribeChanges0(enginehandle.into_param().abi(), changehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, sysports: *mut *mut FWPM_SYSTEM_PORTS0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsGet0(enginehandle: super::super::Foundation::HANDLE, sysports: *mut *mut FWPM_SYSTEM_PORTS0) -> u32;
        }
        ::core::mem::transmute(FwpmSystemPortsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(sysports)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsSubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, reserved: *mut ::core::ffi::c_void, callback: FWPM_SYSTEM_PORTS_CALLBACK0, context: *const ::core::ffi::c_void, sysportshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsSubscribe0(enginehandle: super::super::Foundation::HANDLE, reserved: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, sysportshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSystemPortsSubscribe0(enginehandle.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(sysportshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, sysportshandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, sysportshandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmSystemPortsUnsubscribe0(enginehandle.into_param().abi(), sysportshandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionAbort0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionAbort0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmTransactionAbort0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionBegin0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionBegin0(enginehandle: super::super::Foundation::HANDLE, flags: u32) -> u32;
        }
        ::core::mem::transmute(FwpmTransactionBegin0(enginehandle.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionCommit0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionCommit0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmTransactionCommit0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmvSwitchEventSubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback: FWPM_VSWITCH_EVENT_CALLBACK0, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const FWPM_VSWITCH_EVENT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, subscriptionhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmvSwitchEventSubscribe0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(subscriptionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmvSwitchEventUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscriptionhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, subscriptionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(FwpmvSwitchEventUnsubscribe0(enginehandle.into_param().abi(), subscriptionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventsGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(FwpmvSwitchEventsGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventsSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(FwpmvSwitchEventsSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICMP4_TIME_EXCEED_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_TIME_EXCEED_TRANSIT: ICMP4_TIME_EXCEED_CODE = ICMP4_TIME_EXCEED_CODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_TIME_EXCEED_REASSEMBLY: ICMP4_TIME_EXCEED_CODE = ICMP4_TIME_EXCEED_CODE(1i32);
impl ::core::marker::Copy for ICMP4_TIME_EXCEED_CODE {}
impl ::core::clone::Clone for ICMP4_TIME_EXCEED_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICMP4_TIME_EXCEED_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ICMP4_TIME_EXCEED_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICMP4_TIME_EXCEED_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_TIME_EXCEED_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ICMP4_UNREACH_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_NET: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_HOST: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_PROTOCOL: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_PORT: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_FRAG_NEEDED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_SOURCEROUTE_FAILED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_NET_UNKNOWN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_HOST_UNKNOWN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_ISOLATED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_NET_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_HOST_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_NET_TOS: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_HOST_TOS: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP4_UNREACH_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(13i32);
impl ::core::marker::Copy for ICMP4_UNREACH_CODE {}
impl ::core::clone::Clone for ICMP4_UNREACH_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ICMP4_UNREACH_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ICMP4_UNREACH_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ICMP4_UNREACH_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICMP4_UNREACH_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_DST_UNREACH_ADDR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_DST_UNREACH_ADMIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_DST_UNREACH_BEYONDSCOPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_DST_UNREACH_NOPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_DST_UNREACH_NOROUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_PARAMPROB_HEADER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_PARAMPROB_NEXTHEADER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_PARAMPROB_OPTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_TIME_EXCEED_REASSEMBLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMP6_TIME_EXCEED_TRANSIT: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMPV4_ADDRESS_MASK_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub AddressMask: u32,
}
impl ::core::marker::Copy for ICMPV4_ADDRESS_MASK_MESSAGE {}
impl ::core::clone::Clone for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMPV4_ADDRESS_MASK_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMPV4_ADDRESS_MASK_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMPV4_ADDRESS_MASK_MESSAGE {}
impl ::core::default::Default for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMPV4_INVALID_PREFERENCE_LEVEL: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct ICMPV4_ROUTER_ADVERT_ENTRY {
    pub RouterAdvertAddr: super::super::Networking::WinSock::IN_ADDR,
    pub PreferenceLevel: i32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for ICMPV4_ROUTER_ADVERT_ENTRY {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for ICMPV4_ROUTER_ADVERT_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMPV4_ROUTER_ADVERT_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for ICMPV4_ROUTER_ADVERT_ENTRY {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMPV4_ROUTER_ADVERT_HEADER {
    pub RaHeader: ICMP_MESSAGE,
}
impl ::core::marker::Copy for ICMPV4_ROUTER_ADVERT_HEADER {}
impl ::core::clone::Clone for ICMPV4_ROUTER_ADVERT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMPV4_ROUTER_ADVERT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMPV4_ROUTER_ADVERT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMPV4_ROUTER_ADVERT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMPV4_ROUTER_ADVERT_HEADER {}
impl ::core::default::Default for ICMPV4_ROUTER_ADVERT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMPV4_ROUTER_SOLICIT {
    pub RsHeader: ICMP_MESSAGE,
}
impl ::core::marker::Copy for ICMPV4_ROUTER_SOLICIT {}
impl ::core::clone::Clone for ICMPV4_ROUTER_SOLICIT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMPV4_ROUTER_SOLICIT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMPV4_ROUTER_SOLICIT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMPV4_ROUTER_SOLICIT>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMPV4_ROUTER_SOLICIT {}
impl ::core::default::Default for ICMPV4_ROUTER_SOLICIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMPV4_TIMESTAMP_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub OriginateTimestamp: u32,
    pub ReceiveTimestamp: u32,
    pub TransmitTimestamp: u32,
}
impl ::core::marker::Copy for ICMPV4_TIMESTAMP_MESSAGE {}
impl ::core::clone::Clone for ICMPV4_TIMESTAMP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMPV4_TIMESTAMP_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMPV4_TIMESTAMP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMPV4_TIMESTAMP_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMPV4_TIMESTAMP_MESSAGE {}
impl ::core::default::Default for ICMPV4_TIMESTAMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ICMPV6_ECHO_REQUEST_FLAG_REVERSE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMP_HEADER {
    pub Type: u8,
    pub Code: u8,
    pub Checksum: u16,
}
impl ::core::marker::Copy for ICMP_HEADER {}
impl ::core::clone::Clone for ICMP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ICMP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICMP_HEADER").field("Type", &self.Type).field("Code", &self.Code).field("Checksum", &self.Checksum).finish()
    }
}
unsafe impl ::windows::core::Abi for ICMP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMP_HEADER {}
impl ::core::default::Default for ICMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct ICMP_MESSAGE {
    pub Header: ICMP_HEADER,
    pub Data: ICMP_MESSAGE_0,
}
impl ::core::marker::Copy for ICMP_MESSAGE {}
impl ::core::clone::Clone for ICMP_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMP_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMP_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMP_MESSAGE {}
impl ::core::default::Default for ICMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union ICMP_MESSAGE_0 {
    pub Data32: [u32; 1],
    pub Data16: [u16; 2],
    pub Data8: [u8; 4],
}
impl ::core::marker::Copy for ICMP_MESSAGE_0 {}
impl ::core::clone::Clone for ICMP_MESSAGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ICMP_MESSAGE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ICMP_MESSAGE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ICMP_MESSAGE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ICMP_MESSAGE_0 {}
impl ::core::default::Default for ICMP_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMPV3_QUERY_HEADER {
    pub Type: u8,
    pub Anonymous1: IGMPV3_QUERY_HEADER_0,
    pub Checksum: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN_ADDR,
    pub _bitfield: u8,
    pub Anonymous2: IGMPV3_QUERY_HEADER_1,
    pub SourceCount: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_QUERY_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_QUERY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_QUERY_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IGMPV3_QUERY_HEADER_0 {
    pub MaxRespCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_QUERY_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_QUERY_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_QUERY_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_QUERY_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_QUERY_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMPV3_QUERY_HEADER_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IGMPV3_QUERY_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IGMPV3_QUERY_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_QUERY_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_QUERY_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_QUERY_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_QUERY_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_QUERY_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IGMPV3_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_1_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_QUERY_HEADER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_QUERY_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_QUERY_HEADER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_QUERY_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_QUERY_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMPV3_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_QUERY_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_QUERY_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IGMPV3_QUERY_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IGMPV3_QUERY_HEADER_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_QUERY_HEADER_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_QUERY_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_QUERY_HEADER_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_QUERY_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_QUERY_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IGMPV3_REPORT_HEADER {
    pub Type: u8,
    pub Reserved: u8,
    pub Checksum: u16,
    pub Reserved2: u16,
    pub RecordCount: u16,
}
impl ::core::marker::Copy for IGMPV3_REPORT_HEADER {}
impl ::core::clone::Clone for IGMPV3_REPORT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IGMPV3_REPORT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IGMPV3_REPORT_HEADER").field("Type", &self.Type).field("Reserved", &self.Reserved).field("Checksum", &self.Checksum).field("Reserved2", &self.Reserved2).field("RecordCount", &self.RecordCount).finish()
    }
}
unsafe impl ::windows::core::Abi for IGMPV3_REPORT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IGMPV3_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_REPORT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IGMPV3_REPORT_HEADER {}
impl ::core::default::Default for IGMPV3_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMPV3_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMPV3_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMPV3_REPORT_RECORD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMPV3_REPORT_RECORD_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMPV3_REPORT_RECORD_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMPV3_REPORT_RECORD_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMPV3_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMPV3_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMP_HEADER {
    pub Anonymous1: IGMP_HEADER_0,
    pub Anonymous2: IGMP_HEADER_1,
    pub Checksum: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMP_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMP_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMP_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMP_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IGMP_HEADER_0 {
    pub Anonymous: IGMP_HEADER_0_0,
    pub VersionType: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMP_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMP_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMP_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMP_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMP_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMP_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMP_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMP_HEADER_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMP_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMP_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IGMP_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IGMP_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMP_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMP_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMP_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMP_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMP_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IGMP_HEADER_1 {
    pub Reserved: u8,
    pub MaxRespTime: u8,
    pub Code: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IGMP_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IGMP_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IGMP_HEADER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IGMP_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IGMP_HEADER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IGMP_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IGMP_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_LEAVE_GROUP_TYPE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IGMP_MAX_RESP_CODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_MAX_RESP_CODE_TYPE_NORMAL: IGMP_MAX_RESP_CODE_TYPE = IGMP_MAX_RESP_CODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_MAX_RESP_CODE_TYPE_FLOAT: IGMP_MAX_RESP_CODE_TYPE = IGMP_MAX_RESP_CODE_TYPE(1i32);
impl ::core::marker::Copy for IGMP_MAX_RESP_CODE_TYPE {}
impl ::core::clone::Clone for IGMP_MAX_RESP_CODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IGMP_MAX_RESP_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IGMP_MAX_RESP_CODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IGMP_MAX_RESP_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGMP_MAX_RESP_CODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_QUERY_TYPE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_VERSION1_REPORT_TYPE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_VERSION2_REPORT_TYPE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IGMP_VERSION3_REPORT_TYPE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_IMPERSONATION_NONE: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_IMPERSONATION_MAX: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE = IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(2i32);
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_AUTHENTICATION_METHOD0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD0_0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD0 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD0 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_AUTHENTICATION_METHOD0_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD0_0 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD0_0 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_AUTHENTICATION_METHOD1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD1_0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD1 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD1 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_AUTHENTICATION_METHOD1_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    pub eapAuthentication: IKEEXT_EAP_AUTHENTICATION0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD1_0 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD1_0 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_AUTHENTICATION_METHOD2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD2_0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD2 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD2 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_AUTHENTICATION_METHOD2_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION1,
    pub reservedAuthentication: IKEEXT_RESERVED_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION2,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    pub eapAuthentication: IKEEXT_EAP_AUTHENTICATION0,
}
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD2_0 {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_AUTHENTICATION_METHOD2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD2_0 {}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_AUTHENTICATION_METHOD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_PRESHARED_KEY: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERTIFICATE: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KERBEROS: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_ANONYMOUS: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SSL: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_NTLM_V2: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_IPV6_CGA: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERTIFICATE_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERTIFICATE_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SSL_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SSL_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EAP: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_RESERVED: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(13i32);
impl ::core::marker::Copy for IKEEXT_AUTHENTICATION_METHOD_TYPE {}
impl ::core::clone::Clone for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_AUTHENTICATION_METHOD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION0_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION0_1,
    pub flags: IKEEXT_CERT_AUTH,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0").field("inboundRootArraySize", &self.inboundRootArraySize).field("inboundRootArray", &self.inboundRootArray).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0").field("outboundRootArraySize", &self.outboundRootArraySize).field("outboundRootArray", &self.outboundRootArray).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION1_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION1_1,
    pub flags: IKEEXT_CERT_AUTH,
    pub localCertLocationUrl: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0").field("inboundRootArraySize", &self.inboundRootArraySize).field("inboundRootArray", &self.inboundRootArray).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION1_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0").field("outboundRootArraySize", &self.outboundRootArraySize).field("outboundRootArray", &self.outboundRootArray).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1,
    pub flags: IKEEXT_CERT_AUTH,
    pub localCertLocationUrl: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0").field("inboundRootArraySize", &self.inboundRootArraySize).field("inboundRootCriteria", &self.inboundRootCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    pub inboundEnterpriseStoreArraySize: u32,
    pub inboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1").field("inboundEnterpriseStoreArraySize", &self.inboundEnterpriseStoreArraySize).field("inboundEnterpriseStoreCriteria", &self.inboundEnterpriseStoreCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    pub inboundRootStoreArraySize: u32,
    pub inboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2").field("inboundRootStoreArraySize", &self.inboundRootStoreArraySize).field("inboundTrustedRootStoreCriteria", &self.inboundTrustedRootStoreCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0").field("outboundRootArraySize", &self.outboundRootArraySize).field("outboundRootCriteria", &self.outboundRootCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    pub outboundEnterpriseStoreArraySize: u32,
    pub outboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1").field("outboundEnterpriseStoreArraySize", &self.outboundEnterpriseStoreArraySize).field("outboundEnterpriseStoreCriteria", &self.outboundEnterpriseStoreCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    pub outboundRootStoreArraySize: u32,
    pub outboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2").field("outboundRootStoreArraySize", &self.outboundRootStoreArraySize).field("outboundTrustedRootStoreCriteria", &self.outboundTrustedRootStoreCriteria).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL0 {
    pub subjectName: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub flags: u32,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_CREDENTIAL0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL0").field("subjectName", &self.subjectName).field("certHash", &self.certHash).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_CREDENTIAL0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL1 {
    pub subjectName: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub flags: u32,
    pub certificate: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_CREDENTIAL1 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL1").field("subjectName", &self.subjectName).field("certHash", &self.certHash).field("flags", &self.flags).field("certificate", &self.certificate).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_CREDENTIAL1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL1 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERTIFICATE_CRITERIA0 {
    pub certData: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub eku: *mut IKEEXT_CERT_EKUS0,
    pub name: *mut IKEEXT_CERT_NAME0,
    pub flags: u32,
}
impl ::core::marker::Copy for IKEEXT_CERTIFICATE_CRITERIA0 {}
impl ::core::clone::Clone for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CRITERIA0").field("certData", &self.certData).field("certHash", &self.certHash).field("eku", &self.eku).field("name", &self.name).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERTIFICATE_CRITERIA0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERTIFICATE_CRITERIA0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CRITERIA0 {}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_CERT_AUTH(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(32u32);
impl ::core::marker::Copy for IKEEXT_CERT_AUTH {}
impl ::core::clone::Clone for IKEEXT_CERT_AUTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_CERT_AUTH {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_AUTH {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_CERT_AUTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_AUTH").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_CERT_AUTH {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_CERT_AUTH {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_CERT_CONFIG_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CONFIG_ENTERPRISE_STORE: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CONFIG_UNSPECIFIED: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CONFIG_TYPE_MAX: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(4i32);
impl ::core::marker::Copy for IKEEXT_CERT_CONFIG_TYPE {}
impl ::core::clone::Clone for IKEEXT_CERT_CONFIG_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_CERT_CONFIG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_CONFIG_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_CERT_CONFIG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_CONFIG_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_CERT_CRITERIA_NAME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_DNS: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_UPN: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_RFC822: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_CN: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_OU: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_O: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_DC: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_CRITERIA_NAME_TYPE_MAX: IKEEXT_CERT_CRITERIA_NAME_TYPE = IKEEXT_CERT_CRITERIA_NAME_TYPE(7i32);
impl ::core::marker::Copy for IKEEXT_CERT_CRITERIA_NAME_TYPE {}
impl ::core::clone::Clone for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_CRITERIA_NAME_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERT_EKUS0 {
    pub numEku: u32,
    pub eku: *mut ::windows::core::PSTR,
}
impl ::core::marker::Copy for IKEEXT_CERT_EKUS0 {}
impl ::core::clone::Clone for IKEEXT_CERT_EKUS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_EKUS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_EKUS0").field("numEku", &self.numEku).field("eku", &self.eku).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_EKUS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_EKUS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERT_EKUS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_EKUS0 {}
impl ::core::default::Default for IKEEXT_CERT_EKUS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_CERT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_INTERMEDIATE_CA: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(256u32);
impl ::core::marker::Copy for IKEEXT_CERT_FLAGS {}
impl ::core::clone::Clone for IKEEXT_CERT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_CERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_CERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_CERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_CERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CERT_HASH_LEN: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERT_NAME0 {
    pub nameType: IKEEXT_CERT_CRITERIA_NAME_TYPE,
    pub certName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for IKEEXT_CERT_NAME0 {}
impl ::core::clone::Clone for IKEEXT_CERT_NAME0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_NAME0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_NAME0").field("nameType", &self.nameType).field("certName", &self.certName).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_NAME0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_NAME0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERT_NAME0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_NAME0 {}
impl ::core::default::Default for IKEEXT_CERT_NAME0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CERT_ROOT_CONFIG0 {
    pub certData: FWP_BYTE_BLOB,
    pub flags: IKEEXT_CERT_FLAGS,
}
impl ::core::marker::Copy for IKEEXT_CERT_ROOT_CONFIG0 {}
impl ::core::clone::Clone for IKEEXT_CERT_ROOT_CONFIG0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_ROOT_CONFIG0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_ROOT_CONFIG0").field("certData", &self.certData).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CERT_ROOT_CONFIG0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_ROOT_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CERT_ROOT_CONFIG0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_ROOT_CONFIG0 {}
impl ::core::default::Default for IKEEXT_CERT_ROOT_CONFIG0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CIPHER_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_CIPHER_TYPE,
    pub keyLen: u32,
    pub rounds: u32,
}
impl ::core::marker::Copy for IKEEXT_CIPHER_ALGORITHM0 {}
impl ::core::clone::Clone for IKEEXT_CIPHER_ALGORITHM0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CIPHER_ALGORITHM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CIPHER_ALGORITHM0").field("algoIdentifier", &self.algoIdentifier).field("keyLen", &self.keyLen).field("rounds", &self.rounds).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CIPHER_ALGORITHM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CIPHER_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CIPHER_ALGORITHM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CIPHER_ALGORITHM0 {}
impl ::core::default::Default for IKEEXT_CIPHER_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_CIPHER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_DES: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_3DES: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_AES_128: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_AES_192: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_AES_256: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_AES_GCM_128_16ICV: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_AES_GCM_256_16ICV: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_CIPHER_TYPE_MAX: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(7i32);
impl ::core::marker::Copy for IKEEXT_CIPHER_TYPE {}
impl ::core::clone::Clone for IKEEXT_CIPHER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_CIPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CIPHER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_CIPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CIPHER_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_COMMON_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
impl ::core::marker::Copy for IKEEXT_COMMON_STATISTICS0 {}
impl ::core::clone::Clone for IKEEXT_COMMON_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_COMMON_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COMMON_STATISTICS0").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("totalPacketsReceived", &self.totalPacketsReceived).field("totalInvalidPacketsReceived", &self.totalInvalidPacketsReceived).field("currentQueuedWorkitems", &self.currentQueuedWorkitems).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_COMMON_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_COMMON_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_COMMON_STATISTICS0 {}
impl ::core::default::Default for IKEEXT_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_COMMON_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
impl ::core::marker::Copy for IKEEXT_COMMON_STATISTICS1 {}
impl ::core::clone::Clone for IKEEXT_COMMON_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_COMMON_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COMMON_STATISTICS1").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("totalPacketsReceived", &self.totalPacketsReceived).field("totalInvalidPacketsReceived", &self.totalInvalidPacketsReceived).field("currentQueuedWorkitems", &self.currentQueuedWorkitems).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_COMMON_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_COMMON_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_COMMON_STATISTICS1 {}
impl ::core::default::Default for IKEEXT_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_COOKIE_PAIR0 {
    pub initiator: u64,
    pub responder: u64,
}
impl ::core::marker::Copy for IKEEXT_COOKIE_PAIR0 {}
impl ::core::clone::Clone for IKEEXT_COOKIE_PAIR0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_COOKIE_PAIR0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COOKIE_PAIR0").field("initiator", &self.initiator).field("responder", &self.responder).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_COOKIE_PAIR0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_COOKIE_PAIR0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_COOKIE_PAIR0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_COOKIE_PAIR0 {}
impl ::core::default::Default for IKEEXT_COOKIE_PAIR0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL0_0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL0 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CREDENTIAL0_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL0,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL0_0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL0_0 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL1_0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL1 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL1 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CREDENTIAL1_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL1_0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL1_0 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL2_0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL2 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL2 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_CREDENTIAL2_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL2_0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL2_0 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIALS0 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIALS0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIALS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS0").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIALS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIALS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS0 {}
impl ::core::default::Default for IKEEXT_CREDENTIALS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIALS1 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR1,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIALS1 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIALS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS1").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIALS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIALS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS1 {}
impl ::core::default::Default for IKEEXT_CREDENTIALS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIALS2 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR2,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIALS2 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIALS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS2").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIALS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIALS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS2 {}
impl ::core::default::Default for IKEEXT_CREDENTIALS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL_PAIR0 {
    pub localCredentials: IKEEXT_CREDENTIAL0,
    pub peerCredentials: IKEEXT_CREDENTIAL0,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL_PAIR0 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL_PAIR0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL_PAIR0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL_PAIR0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL_PAIR0 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL_PAIR1 {
    pub localCredentials: IKEEXT_CREDENTIAL1,
    pub peerCredentials: IKEEXT_CREDENTIAL1,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL_PAIR1 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL_PAIR1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL_PAIR1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL_PAIR1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL_PAIR1 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_CREDENTIAL_PAIR2 {
    pub localCredentials: IKEEXT_CREDENTIAL2,
    pub peerCredentials: IKEEXT_CREDENTIAL2,
}
impl ::core::marker::Copy for IKEEXT_CREDENTIAL_PAIR2 {}
impl ::core::clone::Clone for IKEEXT_CREDENTIAL_PAIR2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_CREDENTIAL_PAIR2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_CREDENTIAL_PAIR2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIAL_PAIR2 {}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_DH_GROUP(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_NONE: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_1: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_2: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_14: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_2048: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_ECP_256: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_ECP_384: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_24: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_DH_GROUP_MAX: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(7i32);
impl ::core::marker::Copy for IKEEXT_DH_GROUP {}
impl ::core::clone::Clone for IKEEXT_DH_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_DH_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_DH_GROUP {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_DH_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_DH_GROUP").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_EAP_AUTHENTICATION0 {
    pub flags: IKEEXT_EAP_AUTHENTICATION_FLAGS,
}
impl ::core::marker::Copy for IKEEXT_EAP_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_EAP_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_EAP_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EAP_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EAP_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_EAP_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_EAP_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_EAP_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_EAP_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_EAP_AUTHENTICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY: IKEEXT_EAP_AUTHENTICATION_FLAGS = IKEEXT_EAP_AUTHENTICATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY: IKEEXT_EAP_AUTHENTICATION_FLAGS = IKEEXT_EAP_AUTHENTICATION_FLAGS(2u32);
impl ::core::marker::Copy for IKEEXT_EAP_AUTHENTICATION_FLAGS {}
impl ::core::clone::Clone for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_EAP_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_EM_POLICY0 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
impl ::core::marker::Copy for IKEEXT_EM_POLICY0 {}
impl ::core::clone::Clone for IKEEXT_EM_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY0").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EM_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_EM_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY0 {}
impl ::core::default::Default for IKEEXT_EM_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_EM_POLICY1 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
impl ::core::marker::Copy for IKEEXT_EM_POLICY1 {}
impl ::core::clone::Clone for IKEEXT_EM_POLICY1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY1").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EM_POLICY1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_EM_POLICY1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY1 {}
impl ::core::default::Default for IKEEXT_EM_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_EM_POLICY2 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
impl ::core::marker::Copy for IKEEXT_EM_POLICY2 {}
impl ::core::clone::Clone for IKEEXT_EM_POLICY2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY2").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EM_POLICY2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_EM_POLICY2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY2 {}
impl ::core::default::Default for IKEEXT_EM_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_EM_SA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_NONE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_SENT_ATTS: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_SSPI_SENT: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_AUTH_COMPLETE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_FINAL: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_COMPLETE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_EM_SA_STATE_MAX: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(6i32);
impl ::core::marker::Copy for IKEEXT_EM_SA_STATE {}
impl ::core::clone::Clone for IKEEXT_EM_SA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_EM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_EM_SA_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_EM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_EM_SA_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_INTEGRITY_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_INTEGRITY_TYPE,
}
impl ::core::marker::Copy for IKEEXT_INTEGRITY_ALGORITHM0 {}
impl ::core::clone::Clone for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_INTEGRITY_ALGORITHM0").field("algoIdentifier", &self.algoIdentifier).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_INTEGRITY_ALGORITHM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_INTEGRITY_ALGORITHM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_INTEGRITY_ALGORITHM0 {}
impl ::core::default::Default for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_INTEGRITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_INTEGRITY_MD5: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_INTEGRITY_SHA1: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_INTEGRITY_SHA_256: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_INTEGRITY_SHA_384: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_INTEGRITY_TYPE_MAX: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(4i32);
impl ::core::marker::Copy for IKEEXT_INTEGRITY_TYPE {}
impl ::core::clone::Clone for IKEEXT_INTEGRITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_INTEGRITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_INTEGRITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_INTEGRITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_INTEGRITY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    pub keyContainerName: ::windows::core::PWSTR,
    pub cspName: ::windows::core::PWSTR,
    pub cspType: u32,
    pub cgaModifier: FWP_BYTE_ARRAY16,
    pub cgaCollisionCount: u8,
}
impl ::core::marker::Copy for IKEEXT_IPV6_CGA_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IPV6_CGA_AUTHENTICATION0").field("keyContainerName", &self.keyContainerName).field("cspName", &self.cspName).field("cspType", &self.cspType).field("cgaModifier", &self.cgaModifier).field("cgaCollisionCount", &self.cgaCollisionCount).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_IPV6_CGA_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
impl ::core::marker::Copy for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {}
impl ::core::clone::Clone for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0").field("totalSocketReceiveFailures", &self.totalSocketReceiveFailures).field("totalSocketSendFailures", &self.totalSocketSendFailures).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
impl ::core::marker::Copy for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {}
impl ::core::clone::Clone for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1").field("totalSocketReceiveFailures", &self.totalSocketReceiveFailures).field("totalSocketSendFailures", &self.totalSocketSendFailures).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    pub currentActiveMainModes: u32,
    pub totalMainModesStarted: u32,
    pub totalSuccessfulMainModes: u32,
    pub totalFailedMainModes: u32,
    pub totalResponderMainModes: u32,
    pub currentNewResponderMainModes: u32,
    pub currentActiveQuickModes: u32,
    pub totalQuickModesStarted: u32,
    pub totalSuccessfulQuickModes: u32,
    pub totalFailedQuickModes: u32,
    pub totalAcquires: u32,
    pub totalReinitAcquires: u32,
    pub currentActiveExtendedModes: u32,
    pub totalExtendedModesStarted: u32,
    pub totalSuccessfulExtendedModes: u32,
    pub totalFailedExtendedModes: u32,
    pub totalImpersonationExtendedModes: u32,
    pub totalImpersonationMainModes: u32,
}
impl ::core::marker::Copy for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {}
impl ::core::clone::Clone for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field("currentNewResponderMainModes", &self.currentNewResponderMainModes)
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field("currentActiveExtendedModes", &self.currentActiveExtendedModes)
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field("totalSuccessfulExtendedModes", &self.totalSuccessfulExtendedModes)
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field("totalImpersonationExtendedModes", &self.totalImpersonationExtendedModes)
            .field("totalImpersonationMainModes", &self.totalImpersonationMainModes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    pub currentActiveMainModes: u32,
    pub totalMainModesStarted: u32,
    pub totalSuccessfulMainModes: u32,
    pub totalFailedMainModes: u32,
    pub totalResponderMainModes: u32,
    pub currentNewResponderMainModes: u32,
    pub currentActiveQuickModes: u32,
    pub totalQuickModesStarted: u32,
    pub totalSuccessfulQuickModes: u32,
    pub totalFailedQuickModes: u32,
    pub totalAcquires: u32,
    pub totalReinitAcquires: u32,
    pub currentActiveExtendedModes: u32,
    pub totalExtendedModesStarted: u32,
    pub totalSuccessfulExtendedModes: u32,
    pub totalFailedExtendedModes: u32,
    pub totalImpersonationExtendedModes: u32,
    pub totalImpersonationMainModes: u32,
}
impl ::core::marker::Copy for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {}
impl ::core::clone::Clone for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field("currentNewResponderMainModes", &self.currentNewResponderMainModes)
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field("currentActiveExtendedModes", &self.currentActiveExtendedModes)
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field("totalSuccessfulExtendedModes", &self.totalSuccessfulExtendedModes)
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field("totalImpersonationExtendedModes", &self.totalImpersonationExtendedModes)
            .field("totalImpersonationMainModes", &self.totalImpersonationMainModes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_KERBEROS_AUTHENTICATION0 {
    pub flags: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS,
}
impl ::core::marker::Copy for IKEEXT_KERBEROS_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KERBEROS_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_KERBEROS_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_KERBEROS_AUTHENTICATION1 {
    pub flags: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS,
    pub proxyServer: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for IKEEXT_KERBEROS_AUTHENTICATION1 {}
impl ::core::clone::Clone for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION1").field("flags", &self.flags).field("proxyServer", &self.proxyServer).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KERBEROS_AUTHENTICATION1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_KERBEROS_AUTHENTICATION1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION1 {}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS = IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS = IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(2u32);
impl ::core::marker::Copy for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {}
impl ::core::clone::Clone for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_KERBEROS_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_KEYMODULE_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
impl ::core::marker::Copy for IKEEXT_KEYMODULE_STATISTICS0 {}
impl ::core::clone::Clone for IKEEXT_KEYMODULE_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KEYMODULE_STATISTICS0").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("errorFrequencyTable", &self.errorFrequencyTable).field("mainModeNegotiationTime", &self.mainModeNegotiationTime).field("quickModeNegotiationTime", &self.quickModeNegotiationTime).field("extendedModeNegotiationTime", &self.extendedModeNegotiationTime).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KEYMODULE_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_KEYMODULE_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS0 {}
impl ::core::default::Default for IKEEXT_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_KEYMODULE_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
impl ::core::marker::Copy for IKEEXT_KEYMODULE_STATISTICS1 {}
impl ::core::clone::Clone for IKEEXT_KEYMODULE_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KEYMODULE_STATISTICS1").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("errorFrequencyTable", &self.errorFrequencyTable).field("mainModeNegotiationTime", &self.mainModeNegotiationTime).field("quickModeNegotiationTime", &self.quickModeNegotiationTime).field("extendedModeNegotiationTime", &self.extendedModeNegotiationTime).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KEYMODULE_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_KEYMODULE_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS1 {}
impl ::core::default::Default for IKEEXT_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_KEY_MODULE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KEY_MODULE_IKE: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KEY_MODULE_AUTHIP: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KEY_MODULE_IKEV2: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_KEY_MODULE_MAX: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(3i32);
impl ::core::marker::Copy for IKEEXT_KEY_MODULE_TYPE {}
impl ::core::clone::Clone for IKEEXT_KEY_MODULE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_KEY_MODULE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_KEY_MODULE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_KEY_MODULE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_KEY_MODULE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_MM_SA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_NONE: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_SA_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_SSPI_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_FINAL: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_FINAL_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_COMPLETE: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_MM_SA_STATE_MAX: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(6i32);
impl ::core::marker::Copy for IKEEXT_MM_SA_STATE {}
impl ::core::clone::Clone for IKEEXT_MM_SA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_MM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_MM_SA_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_MM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_MM_SA_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_NAME_CREDENTIAL0 {
    pub principalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for IKEEXT_NAME_CREDENTIAL0 {}
impl ::core::clone::Clone for IKEEXT_NAME_CREDENTIAL0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_NAME_CREDENTIAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_NAME_CREDENTIAL0").field("principalName", &self.principalName).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_NAME_CREDENTIAL0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_NAME_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_NAME_CREDENTIAL0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_NAME_CREDENTIAL0 {}
impl ::core::default::Default for IKEEXT_NAME_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_NTLM_V2_AUTHENTICATION0 {
    pub flags: u32,
}
impl ::core::marker::Copy for IKEEXT_NTLM_V2_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_NTLM_V2_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_NTLM_V2_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_NTLM_V2_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_POLICY0 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: IKEEXT_POLICY_FLAG,
    pub maxDynamicFilters: u32,
}
impl ::core::marker::Copy for IKEEXT_POLICY0 {}
impl ::core::clone::Clone for IKEEXT_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY0").field("softExpirationTime", &self.softExpirationTime).field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).field("numIkeProposals", &self.numIkeProposals).field("ikeProposals", &self.ikeProposals).field("flags", &self.flags).field("maxDynamicFilters", &self.maxDynamicFilters).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY0 {}
impl ::core::default::Default for IKEEXT_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_POLICY1 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: IKEEXT_POLICY_FLAG,
    pub maxDynamicFilters: u32,
    pub retransmitDurationSecs: u32,
}
impl ::core::marker::Copy for IKEEXT_POLICY1 {}
impl ::core::clone::Clone for IKEEXT_POLICY1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY1")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field("initiatorImpersonationType", &self.initiatorImpersonationType)
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_POLICY1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_POLICY1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY1 {}
impl ::core::default::Default for IKEEXT_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_POLICY2 {
    pub softExpirationTime: u32,
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub numIkeProposals: u32,
    pub ikeProposals: *mut IKEEXT_PROPOSAL0,
    pub flags: IKEEXT_POLICY_FLAG,
    pub maxDynamicFilters: u32,
    pub retransmitDurationSecs: u32,
}
impl ::core::marker::Copy for IKEEXT_POLICY2 {}
impl ::core::clone::Clone for IKEEXT_POLICY2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY2")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field("initiatorImpersonationType", &self.initiatorImpersonationType)
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_POLICY2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_POLICY2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY2 {}
impl ::core::default::Default for IKEEXT_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_ENABLE_IKEV2_FRAGMENTATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_POLICY_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(8u32);
impl ::core::marker::Copy for IKEEXT_POLICY_FLAG {}
impl ::core::clone::Clone for IKEEXT_POLICY_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_POLICY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_POLICY_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_POLICY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_POLICY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_POLICY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_POLICY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_IMS_VPN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_FLAG_SITE_TO_SITE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_POLICY_SUPPORT_LOW_POWER_MODE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    pub presharedKey: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION0").field("presharedKey", &self.presharedKey).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_PRESHARED_KEY_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    pub presharedKey: FWP_BYTE_BLOB,
    pub flags: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS,
}
impl ::core::marker::Copy for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {}
impl ::core::clone::Clone for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION1").field("presharedKey", &self.presharedKey).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_PRESHARED_KEY_AUTHENTICATION1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS = IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS = IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(2u32);
impl ::core::marker::Copy for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {}
impl ::core::clone::Clone for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_PROPOSAL0 {
    pub cipherAlgorithm: IKEEXT_CIPHER_ALGORITHM0,
    pub integrityAlgorithm: IKEEXT_INTEGRITY_ALGORITHM0,
    pub maxLifetimeSeconds: u32,
    pub dhGroup: IKEEXT_DH_GROUP,
    pub quickModeLimit: u32,
}
impl ::core::marker::Copy for IKEEXT_PROPOSAL0 {}
impl ::core::clone::Clone for IKEEXT_PROPOSAL0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_PROPOSAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PROPOSAL0").field("cipherAlgorithm", &self.cipherAlgorithm).field("integrityAlgorithm", &self.integrityAlgorithm).field("maxLifetimeSeconds", &self.maxLifetimeSeconds).field("dhGroup", &self.dhGroup).field("quickModeLimit", &self.quickModeLimit).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_PROPOSAL0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_PROPOSAL0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_PROPOSAL0 {}
impl ::core::default::Default for IKEEXT_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_QM_SA_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_QM_SA_STATE_NONE: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_QM_SA_STATE_INITIAL: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_QM_SA_STATE_FINAL: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_QM_SA_STATE_COMPLETE: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_QM_SA_STATE_MAX: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(4i32);
impl ::core::marker::Copy for IKEEXT_QM_SA_STATE {}
impl ::core::clone::Clone for IKEEXT_QM_SA_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_QM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_QM_SA_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_QM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_QM_SA_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_RESERVED_AUTHENTICATION0 {
    pub flags: IKEEXT_RESERVED_AUTHENTICATION_FLAGS,
}
impl ::core::marker::Copy for IKEEXT_RESERVED_AUTHENTICATION0 {}
impl ::core::clone::Clone for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_RESERVED_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_RESERVED_AUTHENTICATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_RESERVED_AUTHENTICATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_RESERVED_AUTHENTICATION0 {}
impl ::core::default::Default for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_RESERVED_AUTHENTICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION: IKEEXT_RESERVED_AUTHENTICATION_FLAGS = IKEEXT_RESERVED_AUTHENTICATION_FLAGS(1u32);
impl ::core::marker::Copy for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {}
impl ::core::clone::Clone for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_RESERVED_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_SA_DETAILS0 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS0_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS0,
    pub ikePolicyKey: ::windows::core::GUID,
    pub virtualIfTunnelId: u64,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS0 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS0 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_SA_DETAILS0_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS0_0 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS0_0 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_SA_DETAILS1 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS1_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS1,
    pub ikePolicyKey: ::windows::core::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS1 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS1 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_SA_DETAILS1_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS1_0 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS1_0 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_SA_DETAILS2 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS2_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS2,
    pub ikePolicyKey: ::windows::core::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS2 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS2 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_SA_DETAILS2_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl ::core::marker::Copy for IKEEXT_SA_DETAILS2_0 {}
impl ::core::clone::Clone for IKEEXT_SA_DETAILS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_DETAILS2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_SA_DETAILS2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_DETAILS2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_SA_DETAILS2_0 {}
impl ::core::default::Default for IKEEXT_SA_DETAILS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IKEEXT_SA_ENUM_TEMPLATE0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
    pub remoteSubNet: FWP_CONDITION_VALUE0,
    pub localMainModeCertHash: FWP_BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IKEEXT_SA_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IKEEXT_SA_ENUM_TEMPLATE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_SA_ENUM_TEMPLATE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IKEEXT_SA_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IKEEXT_SA_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SA_ROLE_INITIATOR: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SA_ROLE_RESPONDER: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IKEEXT_SA_ROLE_MAX: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(2i32);
impl ::core::marker::Copy for IKEEXT_SA_ROLE {}
impl ::core::clone::Clone for IKEEXT_SA_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEEXT_SA_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_SA_ROLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEEXT_SA_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_SA_ROLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_STATISTICS0 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS0,
}
impl ::core::marker::Copy for IKEEXT_STATISTICS0 {}
impl ::core::clone::Clone for IKEEXT_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_STATISTICS0").field("ikeStatistics", &self.ikeStatistics).field("authipStatistics", &self.authipStatistics).field("commonStatistics", &self.commonStatistics).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_STATISTICS0 {}
impl ::core::default::Default for IKEEXT_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_STATISTICS1 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub ikeV2Statistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS1,
}
impl ::core::marker::Copy for IKEEXT_STATISTICS1 {}
impl ::core::clone::Clone for IKEEXT_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEEXT_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_STATISTICS1").field("ikeStatistics", &self.ikeStatistics).field("authipStatistics", &self.authipStatistics).field("ikeV2Statistics", &self.ikeV2Statistics).field("commonStatistics", &self.commonStatistics).finish()
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_STATISTICS1 {}
impl ::core::default::Default for IKEEXT_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IKEEXT_TRAFFIC0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IKEEXT_TRAFFIC0_0,
    pub Anonymous2: IKEEXT_TRAFFIC0_1,
    pub authIpFilterId: u64,
}
impl ::core::marker::Copy for IKEEXT_TRAFFIC0 {}
impl ::core::clone::Clone for IKEEXT_TRAFFIC0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_TRAFFIC0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_TRAFFIC0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_TRAFFIC0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_TRAFFIC0 {}
impl ::core::default::Default for IKEEXT_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IKEEXT_TRAFFIC0_0 {}
impl ::core::clone::Clone for IKEEXT_TRAFFIC0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_TRAFFIC0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_TRAFFIC0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_TRAFFIC0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_TRAFFIC0_0 {}
impl ::core::default::Default for IKEEXT_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IKEEXT_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IKEEXT_TRAFFIC0_1 {}
impl ::core::clone::Clone for IKEEXT_TRAFFIC0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IKEEXT_TRAFFIC0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEEXT_TRAFFIC0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEEXT_TRAFFIC0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEEXT_TRAFFIC0_1 {}
impl ::core::default::Default for IKEEXT_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IN6_EMBEDDEDV4_BITS_IN_BYTE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IN6_EMBEDDEDV4_UOCTET_POSITION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP4_OFF_MASK: u32 = 65311u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6F_MORE_FRAG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6F_OFF_MASK: u32 = 63743u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6F_RESERVED_MASK: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_MUTABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_TYPE_DISCARD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_TYPE_FORCEICMP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_TYPE_ICMP: u32 = 192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_TYPE_SKIP: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_ADDRESS_INFO0 {
    pub numV4Addresses: u32,
    pub v4Addresses: *mut u32,
    pub numV6Addresses: u32,
    pub v6Addresses: *mut FWP_BYTE_ARRAY16,
}
impl ::core::marker::Copy for IPSEC_ADDRESS_INFO0 {}
impl ::core::clone::Clone for IPSEC_ADDRESS_INFO0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_ADDRESS_INFO0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ADDRESS_INFO0").field("numV4Addresses", &self.numV4Addresses).field("v4Addresses", &self.v4Addresses).field("numV6Addresses", &self.numV6Addresses).field("v6Addresses", &self.v6Addresses).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_ADDRESS_INFO0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_ADDRESS_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_ADDRESS_INFO0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_ADDRESS_INFO0 {}
impl ::core::default::Default for IPSEC_ADDRESS_INFO0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub udpEspValidationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub invalidClearTextInbound: u32,
    pub saNotInitializedOnInbound: u32,
    pub receiveOverIncorrectSaInbound: u32,
    pub secureReceivesNotMatchingFilters: u32,
}
impl ::core::marker::Copy for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound)
            .field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound)
            .field("udpEspValidationFailuresOnInbound", &self.udpEspValidationFailuresOnInbound)
            .field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound)
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field("receiveOverIncorrectSaInbound", &self.receiveOverIncorrectSaInbound)
            .field("secureReceivesNotMatchingFilters", &self.secureReceivesNotMatchingFilters)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {}
impl ::core::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub udpEspValidationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub invalidClearTextInbound: u32,
    pub saNotInitializedOnInbound: u32,
    pub receiveOverIncorrectSaInbound: u32,
    pub secureReceivesNotMatchingFilters: u32,
    pub totalDropPacketsInbound: u32,
}
impl ::core::marker::Copy for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {}
impl ::core::clone::Clone for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound)
            .field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound)
            .field("udpEspValidationFailuresOnInbound", &self.udpEspValidationFailuresOnInbound)
            .field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound)
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field("receiveOverIncorrectSaInbound", &self.receiveOverIncorrectSaInbound)
            .field("secureReceivesNotMatchingFilters", &self.secureReceivesNotMatchingFilters)
            .field("totalDropPacketsInbound", &self.totalDropPacketsInbound)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {}
impl ::core::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AGGREGATE_SA_STATISTICS0 {
    pub activeSas: u32,
    pub pendingSaNegotiations: u32,
    pub totalSasAdded: u32,
    pub totalSasDeleted: u32,
    pub successfulRekeys: u32,
    pub activeTunnels: u32,
    pub offloadedSas: u32,
}
impl ::core::marker::Copy for IPSEC_AGGREGATE_SA_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_SA_STATISTICS0").field("activeSas", &self.activeSas).field("pendingSaNegotiations", &self.pendingSaNegotiations).field("totalSasAdded", &self.totalSasAdded).field("totalSasDeleted", &self.totalSasDeleted).field("successfulRekeys", &self.successfulRekeys).field("activeTunnels", &self.activeTunnels).field("offloadedSas", &self.offloadedSas).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AGGREGATE_SA_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AGGREGATE_SA_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_SA_STATISTICS0 {}
impl ::core::default::Default for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AH_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
impl ::core::marker::Copy for IPSEC_AH_DROP_PACKET_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AH_DROP_PACKET_STATISTICS0").field("invalidSpisOnInbound", &self.invalidSpisOnInbound).field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound).field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound).field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AH_DROP_PACKET_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AH_DROP_PACKET_STATISTICS0 {}
impl ::core::default::Default for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
}
impl ::core::marker::Copy for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {}
impl ::core::clone::Clone for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_AND_CIPHER_TRANSFORM0").field("authTransform", &self.authTransform).field("cipherTransform", &self.cipherTransform).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AUTH_AND_CIPHER_TRANSFORM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {}
impl ::core::default::Default for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_GCM_AES_128: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_GCM_AES_192: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_GCM_AES_256: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_HMAC_MD5_96: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_1_96: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_256_128: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_CONFIG_MAX: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AUTH_TRANSFORM0 {
    pub authTransformId: IPSEC_AUTH_TRANSFORM_ID0,
    pub cryptoModuleId: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for IPSEC_AUTH_TRANSFORM0 {}
impl ::core::clone::Clone for IPSEC_AUTH_TRANSFORM0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AUTH_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_TRANSFORM0").field("authTransformId", &self.authTransformId).field("cryptoModuleId", &self.cryptoModuleId).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AUTH_TRANSFORM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AUTH_TRANSFORM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_TRANSFORM0 {}
impl ::core::default::Default for IPSEC_AUTH_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_AUTH_TRANSFORM_ID0 {
    pub authType: IPSEC_AUTH_TYPE,
    pub authConfig: u8,
}
impl ::core::marker::Copy for IPSEC_AUTH_TRANSFORM_ID0 {}
impl ::core::clone::Clone for IPSEC_AUTH_TRANSFORM_ID0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_AUTH_TRANSFORM_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_TRANSFORM_ID0").field("authType", &self.authType).field("authConfig", &self.authConfig).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AUTH_TRANSFORM_ID0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_AUTH_TRANSFORM_ID0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_TRANSFORM_ID0 {}
impl ::core::default::Default for IPSEC_AUTH_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_AUTH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_MD5: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_SHA_1: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_SHA_256: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_AES_128: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_AES_192: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_AES_256: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_AUTH_MAX: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(6i32);
impl ::core::marker::Copy for IPSEC_AUTH_TYPE {}
impl ::core::clone::Clone for IPSEC_AUTH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_AUTH_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_AUTH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_CBC_3DES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_CBC_AES_128: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_CBC_AES_192: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_CBC_AES_256: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_CBC_DES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_GCM_AES_128: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_GCM_AES_192: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_GCM_AES_256: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_CONFIG_MAX: u32 = 9u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_CIPHER_TRANSFORM0 {
    pub cipherTransformId: IPSEC_CIPHER_TRANSFORM_ID0,
    pub cryptoModuleId: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for IPSEC_CIPHER_TRANSFORM0 {}
impl ::core::clone::Clone for IPSEC_CIPHER_TRANSFORM0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_CIPHER_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_CIPHER_TRANSFORM0").field("cipherTransformId", &self.cipherTransformId).field("cryptoModuleId", &self.cryptoModuleId).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_CIPHER_TRANSFORM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_CIPHER_TRANSFORM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_CIPHER_TRANSFORM0 {}
impl ::core::default::Default for IPSEC_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_CIPHER_TRANSFORM_ID0 {
    pub cipherType: IPSEC_CIPHER_TYPE,
    pub cipherConfig: u8,
}
impl ::core::marker::Copy for IPSEC_CIPHER_TRANSFORM_ID0 {}
impl ::core::clone::Clone for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_CIPHER_TRANSFORM_ID0").field("cipherType", &self.cipherType).field("cipherConfig", &self.cipherConfig).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_CIPHER_TRANSFORM_ID0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_CIPHER_TRANSFORM_ID0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_CIPHER_TRANSFORM_ID0 {}
impl ::core::default::Default for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_CIPHER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_DES: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_3DES: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_AES_128: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_AES_192: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_AES_256: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_CIPHER_TYPE_MAX: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(6i32);
impl ::core::marker::Copy for IPSEC_CIPHER_TYPE {}
impl ::core::clone::Clone for IPSEC_CIPHER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_CIPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_CIPHER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_CIPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_CIPHER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_DSCP_DISABLE_VALUE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_DOSP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV1: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV2: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_DISABLE_AUTHIP: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_DISABLE_DEFAULT_BLOCK: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_FILTER_BLOCK: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_FLAG_FILTER_EXEMPT: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(32u32);
impl ::core::marker::Copy for IPSEC_DOSP_FLAGS {}
impl ::core::clone::Clone for IPSEC_DOSP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_DOSP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_DOSP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_DOSP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_DOSP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_DOSP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_DOSP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_DOSP_OPTIONS0 {
    pub stateIdleTimeoutSeconds: u32,
    pub perIPRateLimitQueueIdleTimeoutSeconds: u32,
    pub ipV6IPsecUnauthDscp: u8,
    pub ipV6IPsecUnauthRateLimitBytesPerSec: u32,
    pub ipV6IPsecUnauthPerIPRateLimitBytesPerSec: u32,
    pub ipV6IPsecAuthDscp: u8,
    pub ipV6IPsecAuthRateLimitBytesPerSec: u32,
    pub icmpV6Dscp: u8,
    pub icmpV6RateLimitBytesPerSec: u32,
    pub ipV6FilterExemptDscp: u8,
    pub ipV6FilterExemptRateLimitBytesPerSec: u32,
    pub defBlockExemptDscp: u8,
    pub defBlockExemptRateLimitBytesPerSec: u32,
    pub maxStateEntries: u32,
    pub maxPerIPRateLimitQueues: u32,
    pub flags: IPSEC_DOSP_FLAGS,
    pub numPublicIFLuids: u32,
    pub publicIFLuids: *mut u64,
    pub numInternalIFLuids: u32,
    pub internalIFLuids: *mut u64,
    pub publicV6AddrMask: FWP_V6_ADDR_AND_MASK,
    pub internalV6AddrMask: FWP_V6_ADDR_AND_MASK,
}
impl ::core::marker::Copy for IPSEC_DOSP_OPTIONS0 {}
impl ::core::clone::Clone for IPSEC_DOSP_OPTIONS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_DOSP_OPTIONS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_OPTIONS0")
            .field("stateIdleTimeoutSeconds", &self.stateIdleTimeoutSeconds)
            .field("perIPRateLimitQueueIdleTimeoutSeconds", &self.perIPRateLimitQueueIdleTimeoutSeconds)
            .field("ipV6IPsecUnauthDscp", &self.ipV6IPsecUnauthDscp)
            .field("ipV6IPsecUnauthRateLimitBytesPerSec", &self.ipV6IPsecUnauthRateLimitBytesPerSec)
            .field("ipV6IPsecUnauthPerIPRateLimitBytesPerSec", &self.ipV6IPsecUnauthPerIPRateLimitBytesPerSec)
            .field("ipV6IPsecAuthDscp", &self.ipV6IPsecAuthDscp)
            .field("ipV6IPsecAuthRateLimitBytesPerSec", &self.ipV6IPsecAuthRateLimitBytesPerSec)
            .field("icmpV6Dscp", &self.icmpV6Dscp)
            .field("icmpV6RateLimitBytesPerSec", &self.icmpV6RateLimitBytesPerSec)
            .field("ipV6FilterExemptDscp", &self.ipV6FilterExemptDscp)
            .field("ipV6FilterExemptRateLimitBytesPerSec", &self.ipV6FilterExemptRateLimitBytesPerSec)
            .field("defBlockExemptDscp", &self.defBlockExemptDscp)
            .field("defBlockExemptRateLimitBytesPerSec", &self.defBlockExemptRateLimitBytesPerSec)
            .field("maxStateEntries", &self.maxStateEntries)
            .field("maxPerIPRateLimitQueues", &self.maxPerIPRateLimitQueues)
            .field("flags", &self.flags)
            .field("numPublicIFLuids", &self.numPublicIFLuids)
            .field("publicIFLuids", &self.publicIFLuids)
            .field("numInternalIFLuids", &self.numInternalIFLuids)
            .field("internalIFLuids", &self.internalIFLuids)
            .field("publicV6AddrMask", &self.publicV6AddrMask)
            .field("internalV6AddrMask", &self.internalV6AddrMask)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_DOSP_OPTIONS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_DOSP_OPTIONS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_OPTIONS0 {}
impl ::core::default::Default for IPSEC_DOSP_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_DOSP_RATE_LIMIT_DISABLE_VALUE: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_DOSP_STATE0 {
    pub publicHostV6Addr: [u8; 16],
    pub internalHostV6Addr: [u8; 16],
    pub totalInboundIPv6IPsecAuthPackets: u64,
    pub totalOutboundIPv6IPsecAuthPackets: u64,
    pub durationSecs: u32,
}
impl ::core::marker::Copy for IPSEC_DOSP_STATE0 {}
impl ::core::clone::Clone for IPSEC_DOSP_STATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_DOSP_STATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATE0").field("publicHostV6Addr", &self.publicHostV6Addr).field("internalHostV6Addr", &self.internalHostV6Addr).field("totalInboundIPv6IPsecAuthPackets", &self.totalInboundIPv6IPsecAuthPackets).field("totalOutboundIPv6IPsecAuthPackets", &self.totalOutboundIPv6IPsecAuthPackets).field("durationSecs", &self.durationSecs).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_DOSP_STATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_DOSP_STATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATE0 {}
impl ::core::default::Default for IPSEC_DOSP_STATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    pub publicV6AddrMask: FWP_V6_ADDR_AND_MASK,
    pub internalV6AddrMask: FWP_V6_ADDR_AND_MASK,
}
impl ::core::marker::Copy for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATE_ENUM_TEMPLATE0").field("publicV6AddrMask", &self.publicV6AddrMask).field("internalV6AddrMask", &self.internalV6AddrMask).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_DOSP_STATE_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {}
impl ::core::default::Default for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_DOSP_STATISTICS0 {
    pub totalStateEntriesCreated: u64,
    pub currentStateEntries: u64,
    pub totalInboundAllowedIPv6IPsecUnauthPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundOtherDiscardedIPv6IPsecUnauthPkts: u64,
    pub totalInboundAllowedIPv6IPsecAuthPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6IPsecAuthPkts: u64,
    pub totalInboundOtherDiscardedIPv6IPsecAuthPkts: u64,
    pub totalInboundAllowedICMPv6Pkts: u64,
    pub totalInboundRatelimitDiscardedICMPv6Pkts: u64,
    pub totalInboundAllowedIPv6FilterExemptPkts: u64,
    pub totalInboundRatelimitDiscardedIPv6FilterExemptPkts: u64,
    pub totalInboundDiscardedIPv6FilterBlockPkts: u64,
    pub totalInboundAllowedDefBlockExemptPkts: u64,
    pub totalInboundRatelimitDiscardedDefBlockExemptPkts: u64,
    pub totalInboundDiscardedDefBlockPkts: u64,
    pub currentInboundIPv6IPsecUnauthPerIPRateLimitQueues: u64,
}
impl ::core::marker::Copy for IPSEC_DOSP_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_DOSP_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_DOSP_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATISTICS0")
            .field("totalStateEntriesCreated", &self.totalStateEntriesCreated)
            .field("currentStateEntries", &self.currentStateEntries)
            .field("totalInboundAllowedIPv6IPsecUnauthPkts", &self.totalInboundAllowedIPv6IPsecUnauthPkts)
            .field("totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts", &self.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts", &self.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundOtherDiscardedIPv6IPsecUnauthPkts", &self.totalInboundOtherDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundAllowedIPv6IPsecAuthPkts", &self.totalInboundAllowedIPv6IPsecAuthPkts)
            .field("totalInboundRatelimitDiscardedIPv6IPsecAuthPkts", &self.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts)
            .field("totalInboundOtherDiscardedIPv6IPsecAuthPkts", &self.totalInboundOtherDiscardedIPv6IPsecAuthPkts)
            .field("totalInboundAllowedICMPv6Pkts", &self.totalInboundAllowedICMPv6Pkts)
            .field("totalInboundRatelimitDiscardedICMPv6Pkts", &self.totalInboundRatelimitDiscardedICMPv6Pkts)
            .field("totalInboundAllowedIPv6FilterExemptPkts", &self.totalInboundAllowedIPv6FilterExemptPkts)
            .field("totalInboundRatelimitDiscardedIPv6FilterExemptPkts", &self.totalInboundRatelimitDiscardedIPv6FilterExemptPkts)
            .field("totalInboundDiscardedIPv6FilterBlockPkts", &self.totalInboundDiscardedIPv6FilterBlockPkts)
            .field("totalInboundAllowedDefBlockExemptPkts", &self.totalInboundAllowedDefBlockExemptPkts)
            .field("totalInboundRatelimitDiscardedDefBlockExemptPkts", &self.totalInboundRatelimitDiscardedDefBlockExemptPkts)
            .field("totalInboundDiscardedDefBlockPkts", &self.totalInboundDiscardedDefBlockPkts)
            .field("currentInboundIPv6IPsecUnauthPerIPRateLimitQueues", &self.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_DOSP_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_DOSP_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATISTICS0 {}
impl ::core::default::Default for IPSEC_DOSP_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
impl ::core::marker::Copy for IPSEC_ESP_DROP_PACKET_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ESP_DROP_PACKET_STATISTICS0").field("invalidSpisOnInbound", &self.invalidSpisOnInbound).field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound).field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound).field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound).field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_ESP_DROP_PACKET_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {}
impl ::core::default::Default for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_FAILURE_POINT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_FAILURE_NONE: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_FAILURE_ME: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_FAILURE_PEER: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_FAILURE_POINT_MAX: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(3i32);
impl ::core::marker::Copy for IPSEC_FAILURE_POINT {}
impl ::core::clone::Clone for IPSEC_FAILURE_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_FAILURE_POINT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_FAILURE_POINT {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_FAILURE_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_FAILURE_POINT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_GETSPI0 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC0,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI0_0,
    pub rngCryptoModuleID: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for IPSEC_GETSPI0 {}
impl ::core::clone::Clone for IPSEC_GETSPI0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_GETSPI0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_GETSPI0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_GETSPI0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_GETSPI0 {}
impl ::core::default::Default for IPSEC_GETSPI0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_GETSPI0_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl ::core::marker::Copy for IPSEC_GETSPI0_0 {}
impl ::core::clone::Clone for IPSEC_GETSPI0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_GETSPI0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_GETSPI0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_GETSPI0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_GETSPI0_0 {}
impl ::core::default::Default for IPSEC_GETSPI0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_GETSPI1 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC1,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI1_0,
    pub rngCryptoModuleID: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for IPSEC_GETSPI1 {}
impl ::core::clone::Clone for IPSEC_GETSPI1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_GETSPI1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_GETSPI1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_GETSPI1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_GETSPI1 {}
impl ::core::default::Default for IPSEC_GETSPI1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_GETSPI1_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl ::core::marker::Copy for IPSEC_GETSPI1_0 {}
impl ::core::clone::Clone for IPSEC_GETSPI1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_GETSPI1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_GETSPI1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_GETSPI1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_GETSPI1_0 {}
impl ::core::default::Default for IPSEC_GETSPI1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_ID0 {
    pub mmTargetName: ::windows::core::PWSTR,
    pub emTargetName: ::windows::core::PWSTR,
    pub numTokens: u32,
    pub tokens: *mut IPSEC_TOKEN0,
    pub explicitCredentials: u64,
    pub logonId: u64,
}
impl ::core::marker::Copy for IPSEC_ID0 {}
impl ::core::clone::Clone for IPSEC_ID0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ID0").field("mmTargetName", &self.mmTargetName).field("emTargetName", &self.emTargetName).field("numTokens", &self.numTokens).field("tokens", &self.tokens).field("explicitCredentials", &self.explicitCredentials).field("logonId", &self.logonId).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_ID0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_ID0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_ID0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_ID0 {}
impl ::core::default::Default for IPSEC_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_KEYING_POLICY0 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut ::windows::core::GUID,
}
impl ::core::marker::Copy for IPSEC_KEYING_POLICY0 {}
impl ::core::clone::Clone for IPSEC_KEYING_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_KEYING_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYING_POLICY0").field("numKeyMods", &self.numKeyMods).field("keyModKeys", &self.keyModKeys).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_KEYING_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_KEYING_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_KEYING_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_KEYING_POLICY0 {}
impl ::core::default::Default for IPSEC_KEYING_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_KEYING_POLICY1 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut ::windows::core::GUID,
    pub flags: u32,
}
impl ::core::marker::Copy for IPSEC_KEYING_POLICY1 {}
impl ::core::clone::Clone for IPSEC_KEYING_POLICY1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_KEYING_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYING_POLICY1").field("numKeyMods", &self.numKeyMods).field("keyModKeys", &self.keyModKeys).field("flags", &self.flags).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_KEYING_POLICY1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_KEYING_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_KEYING_POLICY1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_KEYING_POLICY1 {}
impl ::core::default::Default for IPSEC_KEYING_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_KEYING_POLICY_FLAG_TERMINATING_MATCH: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_KEYMODULE_STATE0 {
    pub keyModuleKey: ::windows::core::GUID,
    pub stateBlob: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IPSEC_KEYMODULE_STATE0 {}
impl ::core::clone::Clone for IPSEC_KEYMODULE_STATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_KEYMODULE_STATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYMODULE_STATE0").field("keyModuleKey", &self.keyModuleKey).field("stateBlob", &self.stateBlob).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_KEYMODULE_STATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_KEYMODULE_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_KEYMODULE_STATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_KEYMODULE_STATE0 {}
impl ::core::default::Default for IPSEC_KEYMODULE_STATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_KEY_MANAGER0 {
    pub keyManagerKey: ::windows::core::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub keyDictationTimeoutHint: u8,
}
impl ::core::marker::Copy for IPSEC_KEY_MANAGER0 {}
impl ::core::clone::Clone for IPSEC_KEY_MANAGER0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_KEY_MANAGER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEY_MANAGER0").field("keyManagerKey", &self.keyManagerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("keyDictationTimeoutHint", &self.keyDictationTimeoutHint).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_KEY_MANAGER0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_KEY_MANAGER0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_KEY_MANAGER0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_KEY_MANAGER0 {}
impl ::core::default::Default for IPSEC_KEY_MANAGER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_KEY_MANAGER_CALLBACKS0 {
    pub reserved: ::windows::core::GUID,
    pub flags: u32,
    pub keyDictationCheck: IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0,
    pub keyDictation: IPSEC_KEY_MANAGER_DICTATE_KEY0,
    pub keyNotify: IPSEC_KEY_MANAGER_NOTIFY_KEY0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_KEY_MANAGER_CALLBACKS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEY_MANAGER_CALLBACKS0").field("reserved", &self.reserved).field("flags", &self.flags).field("keyDictationCheck", &self.keyDictationCheck.map(|f| f as usize)).field("keyDictation", &self.keyDictation.map(|f| f as usize)).field("keyNotify", &self.keyNotify.map(|f| f as usize)).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_KEY_MANAGER_CALLBACKS0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_KEY_MANAGER_CALLBACKS0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_KEY_MANAGER_CALLBACKS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type IPSEC_KEY_MANAGER_DICTATE_KEY0 = ::core::option::Option<unsafe extern "system" fn(inboundsadetails: *mut IPSEC_SA_DETAILS1, outboundsadetails: *mut IPSEC_SA_DETAILS1, keyingmodulegenkey: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_KEY_MANAGER_FLAG_DICTATE_KEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0 = ::core::option::Option<unsafe extern "system" fn(iketraffic: *const IKEEXT_TRAFFIC0, willdictatekey: *mut super::super::Foundation::BOOL, weight: *mut u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type IPSEC_KEY_MANAGER_NOTIFY_KEY0 = ::core::option::Option<unsafe extern "system" fn(inboundsa: *const IPSEC_SA_DETAILS1, outboundsa: *const IPSEC_SA_DETAILS1)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_PFS_GROUP(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_NONE: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_1: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_2: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_2048: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_14: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_ECP_256: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_ECP_384: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_MM: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_24: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_PFS_MAX: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(8i32);
impl ::core::marker::Copy for IPSEC_PFS_GROUP {}
impl ::core::clone::Clone for IPSEC_PFS_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_PFS_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_PFS_GROUP {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_PFS_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_PFS_GROUP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_POLICY_FLAG(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_ND_SECURE: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_ND_BOUNDARY: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_PEER_BEHIND_NAT: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_GENERAL_NAT_TRAVERSAL: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_SECOND_LIFETIME: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_BYTE_LIFETIME: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_CLEAR_DF_ON_TUNNEL: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_ENABLE_V6_IN_V4_TUNNELING: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(256u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_ENABLE_SERVER_ADDR_ASSIGNMENT: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(512u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_TUNNEL_ALLOW_OUTBOUND_CLEAR_CONNECTION: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ALREADY_SECURE_CONNECTION: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ICMPV6: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_DICTATE_KEY: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(8192u32);
impl ::core::marker::Copy for IPSEC_POLICY_FLAG {}
impl ::core::clone::Clone for IPSEC_POLICY_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_POLICY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_POLICY_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_POLICY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_POLICY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_POLICY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_POLICY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_NOTIFY_KEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_RESERVED1: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_POLICY_FLAG_SITE_TO_SITE_TUNNEL: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_PROPOSAL0 {
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub numSaTransforms: u32,
    pub saTransforms: *mut IPSEC_SA_TRANSFORM0,
    pub pfsGroup: IPSEC_PFS_GROUP,
}
impl ::core::marker::Copy for IPSEC_PROPOSAL0 {}
impl ::core::clone::Clone for IPSEC_PROPOSAL0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_PROPOSAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_PROPOSAL0").field("lifetime", &self.lifetime).field("numSaTransforms", &self.numSaTransforms).field("saTransforms", &self.saTransforms).field("pfsGroup", &self.pfsGroup).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_PROPOSAL0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_PROPOSAL0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_PROPOSAL0 {}
impl ::core::default::Default for IPSEC_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA0 {
    pub spi: u32,
    pub saTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA0_0,
}
impl ::core::marker::Copy for IPSEC_SA0 {}
impl ::core::clone::Clone for IPSEC_SA0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA0 {}
impl ::core::default::Default for IPSEC_SA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_SA0_0 {
    pub ahInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espAuthInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espCipherInformation: *mut IPSEC_SA_CIPHER_INFORMATION0,
    pub espAuthAndCipherInformation: *mut IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0,
    pub espAuthFwInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
}
impl ::core::marker::Copy for IPSEC_SA0_0 {}
impl ::core::clone::Clone for IPSEC_SA0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA0_0 {}
impl ::core::default::Default for IPSEC_SA0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    pub saCipherInformation: IPSEC_SA_CIPHER_INFORMATION0,
    pub saAuthInformation: IPSEC_SA_AUTH_INFORMATION0,
}
impl ::core::marker::Copy for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {}
impl ::core::clone::Clone for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0").field("saCipherInformation", &self.saCipherInformation).field("saAuthInformation", &self.saAuthInformation).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {}
impl ::core::default::Default for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_AUTH_INFORMATION0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub authKey: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IPSEC_SA_AUTH_INFORMATION0 {}
impl ::core::clone::Clone for IPSEC_SA_AUTH_INFORMATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_AUTH_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_AUTH_INFORMATION0").field("authTransform", &self.authTransform).field("authKey", &self.authKey).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_AUTH_INFORMATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_AUTH_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_AUTH_INFORMATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_AUTH_INFORMATION0 {}
impl ::core::default::Default for IPSEC_SA_AUTH_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_BUNDLE0 {
    pub flags: IPSEC_SA_BUNDLE_FLAGS,
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub idleTimeoutSeconds: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub ipsecId: *mut IPSEC_ID0,
    pub napContext: u32,
    pub qmSaId: u32,
    pub numSAs: u32,
    pub saList: *mut IPSEC_SA0,
    pub keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_SA_BUNDLE0_0,
    pub mmSaId: u64,
    pub pfsGroup: IPSEC_PFS_GROUP,
}
impl ::core::marker::Copy for IPSEC_SA_BUNDLE0 {}
impl ::core::clone::Clone for IPSEC_SA_BUNDLE0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_BUNDLE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_BUNDLE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_BUNDLE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_BUNDLE0 {}
impl ::core::default::Default for IPSEC_SA_BUNDLE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_SA_BUNDLE0_0 {
    pub peerV4PrivateAddress: u32,
}
impl ::core::marker::Copy for IPSEC_SA_BUNDLE0_0 {}
impl ::core::clone::Clone for IPSEC_SA_BUNDLE0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_BUNDLE0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_BUNDLE0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_BUNDLE0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_BUNDLE0_0 {}
impl ::core::default::Default for IPSEC_SA_BUNDLE0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_BUNDLE1 {
    pub flags: IPSEC_SA_BUNDLE_FLAGS,
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub idleTimeoutSeconds: u32,
    pub ndAllowClearTimeoutSeconds: u32,
    pub ipsecId: *mut IPSEC_ID0,
    pub napContext: u32,
    pub qmSaId: u32,
    pub numSAs: u32,
    pub saList: *mut IPSEC_SA0,
    pub keyModuleState: *mut IPSEC_KEYMODULE_STATE0,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_SA_BUNDLE1_0,
    pub mmSaId: u64,
    pub pfsGroup: IPSEC_PFS_GROUP,
    pub saLookupContext: ::windows::core::GUID,
    pub qmFilterId: u64,
}
impl ::core::marker::Copy for IPSEC_SA_BUNDLE1 {}
impl ::core::clone::Clone for IPSEC_SA_BUNDLE1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_BUNDLE1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_BUNDLE1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_BUNDLE1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_BUNDLE1 {}
impl ::core::default::Default for IPSEC_SA_BUNDLE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_SA_BUNDLE1_0 {
    pub peerV4PrivateAddress: u32,
}
impl ::core::marker::Copy for IPSEC_SA_BUNDLE1_0 {}
impl ::core::clone::Clone for IPSEC_SA_BUNDLE1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_BUNDLE1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_BUNDLE1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_BUNDLE1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_BUNDLE1_0 {}
impl ::core::default::Default for IPSEC_SA_BUNDLE1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_SA_BUNDLE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ND_SECURE: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ND_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_NAT_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_GUARANTEE_ENCRYPTION: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ALLOW_NULL_TARGET_NAME_MATCH: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_CLEAR_DF_ON_TUNNEL: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ASSUME_UDP_CONTEXT_OUTBOUND: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_SUPPRESS_DUPLICATE_DELETION: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_PEER_SUPPORTS_GUARANTEE_ENCRYPTION: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(16384u32);
impl ::core::marker::Copy for IPSEC_SA_BUNDLE_FLAGS {}
impl ::core::clone::Clone for IPSEC_SA_BUNDLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_SA_BUNDLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_BUNDLE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_SA_BUNDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_SA_BUNDLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_ENABLE_OPTIONAL_ASYMMETRIC_IDLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_INBOUND_CONNECTIONS: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_OUTBOUND_CONNECTIONS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_FORWARD_PATH_INITIATOR: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_IP_IN_IP_PKT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_LOCALLY_DICTATED_KEYS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_LOW_POWER_MODE_SUPPORT: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_NLB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_NO_EXPLICIT_CRED_MATCH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_NO_IMPERSONATION_LUID_VERIFY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_NO_MACHINE_LUID_VERIFY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_SA_OFFLOADED: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_BUNDLE_FLAG_USING_DICTATED_KEYS: u32 = 524288u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_CIPHER_INFORMATION0 {
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
    pub cipherKey: FWP_BYTE_BLOB,
}
impl ::core::marker::Copy for IPSEC_SA_CIPHER_INFORMATION0 {}
impl ::core::clone::Clone for IPSEC_SA_CIPHER_INFORMATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_CIPHER_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CIPHER_INFORMATION0").field("cipherTransform", &self.cipherTransform).field("cipherKey", &self.cipherKey).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_CIPHER_INFORMATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CIPHER_INFORMATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_CIPHER_INFORMATION0 {}
impl ::core::default::Default for IPSEC_SA_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT0 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS0,
    pub outboundSa: *mut IPSEC_SA_DETAILS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_CONTEXT0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT0").field("saContextId", &self.saContextId).field("inboundSa", &self.inboundSa).field("outboundSa", &self.outboundSa).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CONTEXT0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT1 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS1,
    pub outboundSa: *mut IPSEC_SA_DETAILS1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_CONTEXT1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT1").field("saContextId", &self.saContextId).field("inboundSa", &self.inboundSa).field("outboundSa", &self.outboundSa).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CONTEXT1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub type IPSEC_SA_CONTEXT_CALLBACK0 = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, change: *const IPSEC_SA_CONTEXT_CHANGE0)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_CONTEXT_CHANGE0 {
    pub changeType: IPSEC_SA_CONTEXT_EVENT_TYPE0,
    pub saContextId: u64,
}
impl ::core::marker::Copy for IPSEC_SA_CONTEXT_CHANGE0 {}
impl ::core::clone::Clone for IPSEC_SA_CONTEXT_CHANGE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT_CHANGE0").field("changeType", &self.changeType).field("saContextId", &self.saContextId).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT_CHANGE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CONTEXT_CHANGE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT_CHANGE0 {}
impl ::core::default::Default for IPSEC_SA_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CONTEXT_ENUM_TEMPLATE0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_SA_CONTEXT_EVENT_TYPE0(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_CONTEXT_EVENT_ADD: IPSEC_SA_CONTEXT_EVENT_TYPE0 = IPSEC_SA_CONTEXT_EVENT_TYPE0(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_CONTEXT_EVENT_DELETE: IPSEC_SA_CONTEXT_EVENT_TYPE0 = IPSEC_SA_CONTEXT_EVENT_TYPE0(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_SA_CONTEXT_EVENT_MAX: IPSEC_SA_CONTEXT_EVENT_TYPE0 = IPSEC_SA_CONTEXT_EVENT_TYPE0(3i32);
impl ::core::marker::Copy for IPSEC_SA_CONTEXT_EVENT_TYPE0 {}
impl ::core::clone::Clone for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_SA_CONTEXT_EVENT_TYPE0").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_CONTEXT_SUBSCRIPTION0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_DETAILS0 {
    pub ipVersion: FWP_IP_VERSION,
    pub saDirection: FWP_DIRECTION,
    pub traffic: IPSEC_TRAFFIC0,
    pub saBundle: IPSEC_SA_BUNDLE0,
    pub Anonymous: IPSEC_SA_DETAILS0_0,
    pub transportFilter: *mut FWPM_FILTER0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_DETAILS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_DETAILS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_DETAILS0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_DETAILS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_DETAILS0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_DETAILS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union IPSEC_SA_DETAILS0_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_DETAILS0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_DETAILS0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_DETAILS0_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_DETAILS0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_DETAILS0_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_DETAILS0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_DETAILS1 {
    pub ipVersion: FWP_IP_VERSION,
    pub saDirection: FWP_DIRECTION,
    pub traffic: IPSEC_TRAFFIC1,
    pub saBundle: IPSEC_SA_BUNDLE1,
    pub Anonymous: IPSEC_SA_DETAILS1_0,
    pub transportFilter: *mut FWPM_FILTER0,
    pub virtualIfTunnelInfo: IPSEC_VIRTUAL_IF_TUNNEL_INFO0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_DETAILS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_DETAILS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_DETAILS1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_DETAILS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_DETAILS1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_DETAILS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union IPSEC_SA_DETAILS1_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for IPSEC_SA_DETAILS1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for IPSEC_SA_DETAILS1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for IPSEC_SA_DETAILS1_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_DETAILS1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_DETAILS1_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_DETAILS1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_ENUM_TEMPLATE0 {
    pub saDirection: FWP_DIRECTION,
}
impl ::core::marker::Copy for IPSEC_SA_ENUM_TEMPLATE0 {}
impl ::core::clone::Clone for IPSEC_SA_ENUM_TEMPLATE0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_ENUM_TEMPLATE0").field("saDirection", &self.saDirection).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_ENUM_TEMPLATE0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_ENUM_TEMPLATE0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_ENUM_TEMPLATE0 {}
impl ::core::default::Default for IPSEC_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_IDLE_TIMEOUT0 {
    pub idleTimeoutSeconds: u32,
    pub idleTimeoutSecondsFailOver: u32,
}
impl ::core::marker::Copy for IPSEC_SA_IDLE_TIMEOUT0 {}
impl ::core::clone::Clone for IPSEC_SA_IDLE_TIMEOUT0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_IDLE_TIMEOUT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_IDLE_TIMEOUT0").field("idleTimeoutSeconds", &self.idleTimeoutSeconds).field("idleTimeoutSecondsFailOver", &self.idleTimeoutSecondsFailOver).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_IDLE_TIMEOUT0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_IDLE_TIMEOUT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_IDLE_TIMEOUT0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_IDLE_TIMEOUT0 {}
impl ::core::default::Default for IPSEC_SA_IDLE_TIMEOUT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_LIFETIME0 {
    pub lifetimeSeconds: u32,
    pub lifetimeKilobytes: u32,
    pub lifetimePackets: u32,
}
impl ::core::marker::Copy for IPSEC_SA_LIFETIME0 {}
impl ::core::clone::Clone for IPSEC_SA_LIFETIME0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_SA_LIFETIME0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_LIFETIME0").field("lifetimeSeconds", &self.lifetimeSeconds).field("lifetimeKilobytes", &self.lifetimeKilobytes).field("lifetimePackets", &self.lifetimePackets).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_LIFETIME0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_LIFETIME0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_LIFETIME0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_LIFETIME0 {}
impl ::core::default::Default for IPSEC_SA_LIFETIME0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_SA_TRANSFORM0 {
    pub ipsecTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA_TRANSFORM0_0,
}
impl ::core::marker::Copy for IPSEC_SA_TRANSFORM0 {}
impl ::core::clone::Clone for IPSEC_SA_TRANSFORM0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_TRANSFORM0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_TRANSFORM0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_TRANSFORM0 {}
impl ::core::default::Default for IPSEC_SA_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_SA_TRANSFORM0_0 {
    pub ahTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espAuthTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espCipherTransform: *mut IPSEC_CIPHER_TRANSFORM0,
    pub espAuthAndCipherTransform: *mut IPSEC_AUTH_AND_CIPHER_TRANSFORM0,
    pub espAuthFwTransform: *mut IPSEC_AUTH_TRANSFORM0,
}
impl ::core::marker::Copy for IPSEC_SA_TRANSFORM0_0 {}
impl ::core::clone::Clone for IPSEC_SA_TRANSFORM0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_SA_TRANSFORM0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_SA_TRANSFORM0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_SA_TRANSFORM0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_SA_TRANSFORM0_0 {}
impl ::core::default::Default for IPSEC_SA_TRANSFORM0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_STATISTICS0 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
}
impl ::core::marker::Copy for IPSEC_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_STATISTICS0").field("aggregateSaStatistics", &self.aggregateSaStatistics).field("espDropPacketStatistics", &self.espDropPacketStatistics).field("ahDropPacketStatistics", &self.ahDropPacketStatistics).field("aggregateDropPacketStatistics", &self.aggregateDropPacketStatistics).field("inboundTrafficStatistics", &self.inboundTrafficStatistics).field("outboundTrafficStatistics", &self.outboundTrafficStatistics).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_STATISTICS0 {}
impl ::core::default::Default for IPSEC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_STATISTICS1 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
}
impl ::core::marker::Copy for IPSEC_STATISTICS1 {}
impl ::core::clone::Clone for IPSEC_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_STATISTICS1").field("aggregateSaStatistics", &self.aggregateSaStatistics).field("espDropPacketStatistics", &self.espDropPacketStatistics).field("ahDropPacketStatistics", &self.ahDropPacketStatistics).field("aggregateDropPacketStatistics", &self.aggregateDropPacketStatistics).field("inboundTrafficStatistics", &self.inboundTrafficStatistics).field("outboundTrafficStatistics", &self.outboundTrafficStatistics).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_STATISTICS1 {}
impl ::core::default::Default for IPSEC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TOKEN0 {
    pub r#type: IPSEC_TOKEN_TYPE,
    pub principal: IPSEC_TOKEN_PRINCIPAL,
    pub mode: IPSEC_TOKEN_MODE,
    pub token: u64,
}
impl ::core::marker::Copy for IPSEC_TOKEN0 {}
impl ::core::clone::Clone for IPSEC_TOKEN0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TOKEN0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TOKEN0").field("type", &self.r#type).field("principal", &self.principal).field("mode", &self.mode).field("token", &self.token).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TOKEN0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TOKEN0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TOKEN0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TOKEN0 {}
impl ::core::default::Default for IPSEC_TOKEN0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_TOKEN_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_MODE_MAIN: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_MODE_EXTENDED: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_MODE_MAX: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(2i32);
impl ::core::marker::Copy for IPSEC_TOKEN_MODE {}
impl ::core::clone::Clone for IPSEC_TOKEN_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_TOKEN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TOKEN_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_TOKEN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_TOKEN_PRINCIPAL(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_PRINCIPAL_LOCAL: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_PRINCIPAL_PEER: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_PRINCIPAL_MAX: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(2i32);
impl ::core::marker::Copy for IPSEC_TOKEN_PRINCIPAL {}
impl ::core::clone::Clone for IPSEC_TOKEN_PRINCIPAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_TOKEN_PRINCIPAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TOKEN_PRINCIPAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_TOKEN_PRINCIPAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_PRINCIPAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_TOKEN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_TYPE_MACHINE: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_TYPE_IMPERSONATION: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TOKEN_TYPE_MAX: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(2i32);
impl ::core::marker::Copy for IPSEC_TOKEN_TYPE {}
impl ::core::clone::Clone for IPSEC_TOKEN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TOKEN_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TRAFFIC0_0,
    pub Anonymous2: IPSEC_TRAFFIC0_1,
    pub trafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous3: IPSEC_TRAFFIC0_2,
    pub remotePort: u16,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC0 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC0 {}
impl ::core::default::Default for IPSEC_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC0_0 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC0_0 {}
impl ::core::default::Default for IPSEC_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC0_1 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC0_1 {}
impl ::core::default::Default for IPSEC_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC0_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC0_2 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC0_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC0_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC0_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC0_2 {}
impl ::core::default::Default for IPSEC_TRAFFIC0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC1 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TRAFFIC1_0,
    pub Anonymous2: IPSEC_TRAFFIC1_1,
    pub trafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous3: IPSEC_TRAFFIC1_2,
    pub remotePort: u16,
    pub localPort: u16,
    pub ipProtocol: u8,
    pub localIfLuid: u64,
    pub realIfProfileId: u32,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC1 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC1 {}
impl ::core::default::Default for IPSEC_TRAFFIC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC1_0 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC1_0 {}
impl ::core::default::Default for IPSEC_TRAFFIC1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC1_1 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC1_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC1_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC1_1 {}
impl ::core::default::Default for IPSEC_TRAFFIC1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC1_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC1_2 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC1_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC1_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC1_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC1_2 {}
impl ::core::default::Default for IPSEC_TRAFFIC1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC_SELECTOR0_ {
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TRAFFIC_SELECTOR0__0,
    pub Anonymous2: IPSEC_TRAFFIC_SELECTOR0__1,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_SELECTOR0_ {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_SELECTOR0_ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_SELECTOR0_ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_SELECTOR0_>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0_ {}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR0_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC_SELECTOR0__0 {
    pub startV4Address: u32,
    pub startV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_SELECTOR0__0 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_SELECTOR0__0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_SELECTOR0__0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0__0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_SELECTOR0__0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0__0 {}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR0__0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TRAFFIC_SELECTOR0__1 {
    pub endV4Address: u32,
    pub endV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_SELECTOR0__1 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_SELECTOR0__1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_SELECTOR0__1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0__1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_SELECTOR0__1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0__1 {}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR0__1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    pub flags: u32,
    pub numLocalTrafficSelectors: u32,
    pub localTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0_,
    pub numRemoteTrafficSelectors: u32,
    pub remoteTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0_,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_SELECTOR_POLICY0_").field("flags", &self.flags).field("numLocalTrafficSelectors", &self.numLocalTrafficSelectors).field("localTrafficSelectors", &self.localTrafficSelectors).field("numRemoteTrafficSelectors", &self.numRemoteTrafficSelectors).field("remoteTrafficSelectors", &self.remoteTrafficSelectors).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_SELECTOR_POLICY0_>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC_STATISTICS0 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_STATISTICS0 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_STATISTICS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_STATISTICS0").field("encryptedByteCount", &self.encryptedByteCount).field("authenticatedAHByteCount", &self.authenticatedAHByteCount).field("authenticatedESPByteCount", &self.authenticatedESPByteCount).field("transportByteCount", &self.transportByteCount).field("tunnelByteCount", &self.tunnelByteCount).field("offloadByteCount", &self.offloadByteCount).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_STATISTICS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_STATISTICS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_STATISTICS0 {}
impl ::core::default::Default for IPSEC_TRAFFIC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRAFFIC_STATISTICS1 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
    pub totalSuccessfulPackets: u64,
}
impl ::core::marker::Copy for IPSEC_TRAFFIC_STATISTICS1 {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_STATISTICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_STATISTICS1").field("encryptedByteCount", &self.encryptedByteCount).field("authenticatedAHByteCount", &self.authenticatedAHByteCount).field("authenticatedESPByteCount", &self.authenticatedESPByteCount).field("transportByteCount", &self.transportByteCount).field("tunnelByteCount", &self.tunnelByteCount).field("offloadByteCount", &self.offloadByteCount).field("totalSuccessfulPackets", &self.totalSuccessfulPackets).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_STATISTICS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRAFFIC_STATISTICS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_STATISTICS1 {}
impl ::core::default::Default for IPSEC_TRAFFIC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_TRAFFIC_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRAFFIC_TYPE_TRANSPORT: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRAFFIC_TYPE_TUNNEL: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRAFFIC_TYPE_MAX: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(2i32);
impl ::core::marker::Copy for IPSEC_TRAFFIC_TYPE {}
impl ::core::clone::Clone for IPSEC_TRAFFIC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRAFFIC_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TRAFFIC_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPSEC_TRANSFORM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_AH: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_ESP_AUTH: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_ESP_CIPHER: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_ESP_AUTH_AND_CIPHER: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_ESP_AUTH_FW: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPSEC_TRANSFORM_TYPE_MAX: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(6i32);
impl ::core::marker::Copy for IPSEC_TRANSFORM_TYPE {}
impl ::core::clone::Clone for IPSEC_TRANSFORM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPSEC_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRANSFORM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPSEC_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRANSPORT_POLICY0 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY0,
}
impl ::core::marker::Copy for IPSEC_TRANSPORT_POLICY0 {}
impl ::core::clone::Clone for IPSEC_TRANSPORT_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY0").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRANSPORT_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRANSPORT_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY0 {}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRANSPORT_POLICY1 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY1,
}
impl ::core::marker::Copy for IPSEC_TRANSPORT_POLICY1 {}
impl ::core::clone::Clone for IPSEC_TRANSPORT_POLICY1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY1").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRANSPORT_POLICY1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRANSPORT_POLICY1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY1 {}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TRANSPORT_POLICY2 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY2,
}
impl ::core::marker::Copy for IPSEC_TRANSPORT_POLICY2 {}
impl ::core::clone::Clone for IPSEC_TRANSPORT_POLICY2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY2").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TRANSPORT_POLICY2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TRANSPORT_POLICY2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY2 {}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_ENDPOINT0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINT0_0,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINT0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINT0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINT0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINT0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINT0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINT0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINT0_0 {
    pub v4Address: u32,
    pub v6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINT0_0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINT0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINT0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINT0_0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_ENDPOINTS0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS0_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS0_1,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS0_0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0_0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS0_1 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0_1 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_ENDPOINTS1 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS1_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS1_1,
    pub localIfLuid: u64,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS1 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS1_0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1_0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS1_1 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS1_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS1_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1_1 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_ENDPOINTS2 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS2_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS2_1,
    pub localIfLuid: u64,
    pub remoteFqdn: ::windows::core::PWSTR,
    pub numAddresses: u32,
    pub remoteAddresses: *mut IPSEC_TUNNEL_ENDPOINT0,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS2 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS2_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS2_0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2_0 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPSEC_TUNNEL_ENDPOINTS2_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl ::core::marker::Copy for IPSEC_TUNNEL_ENDPOINTS2_1 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_ENDPOINTS2_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_ENDPOINTS2_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2_1 {}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_POLICY0 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS0,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY0,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_POLICY0 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_POLICY0 {}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_POLICY1 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS1,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY1,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_POLICY1 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_POLICY1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_POLICY1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_POLICY1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_POLICY1 {}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_POLICY2 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY2,
    pub fwdPathSaLifetime: u32,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_POLICY2 {}
impl ::core::clone::Clone for IPSEC_TUNNEL_POLICY2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_POLICY2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_POLICY2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_POLICY2 {}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_TUNNEL_POLICY3_ {
    pub flags: u32,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY2,
    pub fwdPathSaLifetime: u32,
    pub compartmentId: u32,
    pub numTrafficSelectorPolicy: u32,
    pub trafficSelectorPolicies: *mut IPSEC_TRAFFIC_SELECTOR_POLICY0_,
}
impl ::core::marker::Copy for IPSEC_TUNNEL_POLICY3_ {}
impl ::core::clone::Clone for IPSEC_TUNNEL_POLICY3_ {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPSEC_TUNNEL_POLICY3_ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_TUNNEL_POLICY3_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_TUNNEL_POLICY3_>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_TUNNEL_POLICY3_ {}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY3_ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_V4_UDP_ENCAPSULATION0 {
    pub localUdpEncapPort: u16,
    pub remoteUdpEncapPort: u16,
}
impl ::core::marker::Copy for IPSEC_V4_UDP_ENCAPSULATION0 {}
impl ::core::clone::Clone for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_V4_UDP_ENCAPSULATION0").field("localUdpEncapPort", &self.localUdpEncapPort).field("remoteUdpEncapPort", &self.remoteUdpEncapPort).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_V4_UDP_ENCAPSULATION0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_V4_UDP_ENCAPSULATION0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_V4_UDP_ENCAPSULATION0 {}
impl ::core::default::Default for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    pub virtualIfTunnelId: u64,
    pub trafficSelectorId: u64,
}
impl ::core::marker::Copy for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {}
impl ::core::clone::Clone for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_VIRTUAL_IF_TUNNEL_INFO0").field("virtualIfTunnelId", &self.virtualIfTunnelId).field("trafficSelectorId", &self.trafficSelectorId).finish()
    }
}
unsafe impl ::windows::core::Abi for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPSEC_VIRTUAL_IF_TUNNEL_INFO0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {}
impl ::core::default::Default for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPTLS_METADATA {
    pub SequenceNumber: u64,
}
impl ::core::marker::Copy for IPTLS_METADATA {}
impl ::core::clone::Clone for IPTLS_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPTLS_METADATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPTLS_METADATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPTLS_METADATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPTLS_METADATA {}
impl ::core::default::Default for IPTLS_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV4_HEADER {
    pub Anonymous1: IPV4_HEADER_0,
    pub Anonymous2: IPV4_HEADER_1,
    pub TotalLength: u16,
    pub Identification: u16,
    pub Anonymous3: IPV4_HEADER_2,
    pub TimeToLive: u8,
    pub Protocol: u8,
    pub HeaderChecksum: u16,
    pub SourceAddress: super::super::Networking::WinSock::IN_ADDR,
    pub DestinationAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IPV4_HEADER_0 {
    pub VersionAndHeaderLength: u8,
    pub Anonymous: IPV4_HEADER_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV4_HEADER_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IPV4_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV4_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IPV4_HEADER_1 {
    pub TypeOfServiceAndEcnField: u8,
    pub Anonymous: IPV4_HEADER_1_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV4_HEADER_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IPV4_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV4_HEADER_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IPV4_HEADER_2 {
    pub FlagsAndOffset: u16,
    pub Anonymous: IPV4_HEADER_2_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_2 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV4_HEADER_2_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV4_HEADER_2_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV4_HEADER_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IPV4_HEADER_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV4_HEADER_2_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV4_HEADER_2_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV4_HEADER_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_HEADER_2_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV4_HEADER_2_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV4_HEADER_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV4_MAX_MINIMUM_MTU: u32 = 576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV4_MINIMUM_MTU: u32 = 576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV4_MIN_MINIMUM_MTU: u32 = 352u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV4_OPTION_HEADER {
    pub Anonymous: IPV4_OPTION_HEADER_0,
    pub OptionLength: u8,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV4_OPTION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_OPTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_OPTION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_OPTION_HEADER {}
impl ::core::default::Default for IPV4_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPV4_OPTION_HEADER_0 {
    pub OptionType: u8,
    pub Anonymous: IPV4_OPTION_HEADER_0_0,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER_0 {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV4_OPTION_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_OPTION_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_OPTION_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_OPTION_HEADER_0 {}
impl ::core::default::Default for IPV4_OPTION_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV4_OPTION_HEADER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_OPTION_HEADER_0_0 {}
impl ::core::clone::Clone for IPV4_OPTION_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV4_OPTION_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV4_OPTION_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV4_OPTION_HEADER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_OPTION_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_OPTION_HEADER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_OPTION_HEADER_0_0 {}
impl ::core::default::Default for IPV4_OPTION_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPV4_OPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_EOL: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_NOP: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_SECURITY: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(130i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_LSRR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(131i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_TS: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(68i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_RR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_SSRR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(137i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_SID: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(136i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_ROUTER_ALERT: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(148i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPT_MULTIDEST: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(149i32);
impl ::core::marker::Copy for IPV4_OPTION_TYPE {}
impl ::core::clone::Clone for IPV4_OPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPV4_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPV4_OPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPV4_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPV4_OPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV4_ROUTING_HEADER {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
}
impl ::core::marker::Copy for IPV4_ROUTING_HEADER {}
impl ::core::clone::Clone for IPV4_ROUTING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV4_ROUTING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_ROUTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_ROUTING_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_ROUTING_HEADER {}
impl ::core::default::Default for IPV4_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV4_TIMESTAMP_OPTION {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV4_TIMESTAMP_OPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_TIMESTAMP_OPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_TIMESTAMP_OPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_TIMESTAMP_OPTION {}
impl ::core::default::Default for IPV4_TIMESTAMP_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPV4_TIMESTAMP_OPTION_0 {
    pub FlagsOverflow: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0_0,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION_0 {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV4_TIMESTAMP_OPTION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_TIMESTAMP_OPTION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_TIMESTAMP_OPTION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_TIMESTAMP_OPTION_0 {}
impl ::core::default::Default for IPV4_TIMESTAMP_OPTION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV4_TIMESTAMP_OPTION_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV4_TIMESTAMP_OPTION_0_0 {}
impl ::core::clone::Clone for IPV4_TIMESTAMP_OPTION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV4_TIMESTAMP_OPTION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV4_TIMESTAMP_OPTION_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV4_TIMESTAMP_OPTION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV4_TIMESTAMP_OPTION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV4_TIMESTAMP_OPTION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV4_TIMESTAMP_OPTION_0_0 {}
impl ::core::default::Default for IPV4_TIMESTAMP_OPTION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV4_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_ECN_MASK: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_ECN_SHIFT: u32 = 12u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_EXTENSION_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for IPV6_EXTENSION_HEADER {}
impl ::core::clone::Clone for IPV6_EXTENSION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_EXTENSION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_EXTENSION_HEADER").field("NextHeader", &self.NextHeader).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_EXTENSION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_EXTENSION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_EXTENSION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_EXTENSION_HEADER {}
impl ::core::default::Default for IPV6_EXTENSION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_FLOW_LABEL_MASK: u32 = 4294905600u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_FRAGMENT_HEADER {
    pub NextHeader: u8,
    pub Reserved: u8,
    pub Anonymous: IPV6_FRAGMENT_HEADER_0,
    pub Id: u32,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV6_FRAGMENT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_FRAGMENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_FRAGMENT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_FRAGMENT_HEADER {}
impl ::core::default::Default for IPV6_FRAGMENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPV6_FRAGMENT_HEADER_0 {
    pub Anonymous: IPV6_FRAGMENT_HEADER_0_0,
    pub OffsetAndFlags: u16,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER_0 {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV6_FRAGMENT_HEADER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_FRAGMENT_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_FRAGMENT_HEADER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_FRAGMENT_HEADER_0 {}
impl ::core::default::Default for IPV6_FRAGMENT_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_FRAGMENT_HEADER_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IPV6_FRAGMENT_HEADER_0_0 {}
impl ::core::clone::Clone for IPV6_FRAGMENT_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_FRAGMENT_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_FRAGMENT_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_FRAGMENT_HEADER_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_FRAGMENT_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_FRAGMENT_HEADER_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_FRAGMENT_HEADER_0_0 {}
impl ::core::default::Default for IPV6_FRAGMENT_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_FULL_TRAFFIC_CLASS_MASK: u32 = 61455u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV6_HEADER {
    pub Anonymous: IPV6_HEADER_0,
    pub PayloadLength: u16,
    pub NextHeader: u8,
    pub HopLimit: u8,
    pub SourceAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub DestinationAddress: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV6_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV6_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV6_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV6_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV6_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV6_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union IPV6_HEADER_0 {
    pub VersionClassFlow: u32,
    pub Anonymous: IPV6_HEADER_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV6_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV6_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV6_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV6_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV6_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV6_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IPV6_HEADER_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for IPV6_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for IPV6_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for IPV6_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for IPV6_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for IPV6_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for IPV6_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for IPV6_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_MINIMUM_MTU: u32 = 1280u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0,
    pub Value: u32,
}
impl ::core::marker::Copy for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {}
impl ::core::clone::Clone for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {}
impl ::core::default::Default for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl ::core::marker::Copy for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {}
impl ::core::clone::Clone for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0").field("_bitfield", &self._bitfield).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {}
impl ::core::default::Default for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_OPTION_HEADER {
    pub Type: u8,
    pub DataLength: u8,
}
impl ::core::marker::Copy for IPV6_OPTION_HEADER {}
impl ::core::clone::Clone for IPV6_OPTION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_OPTION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_HEADER").field("Type", &self.Type).field("DataLength", &self.DataLength).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_OPTION_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_OPTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_OPTION_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_HEADER {}
impl ::core::default::Default for IPV6_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_OPTION_JUMBOGRAM {
    pub Header: IPV6_OPTION_HEADER,
    pub JumbogramLength: [u8; 4],
}
impl ::core::marker::Copy for IPV6_OPTION_JUMBOGRAM {}
impl ::core::clone::Clone for IPV6_OPTION_JUMBOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_OPTION_JUMBOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_JUMBOGRAM").field("Header", &self.Header).field("JumbogramLength", &self.JumbogramLength).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_OPTION_JUMBOGRAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_OPTION_JUMBOGRAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_OPTION_JUMBOGRAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_JUMBOGRAM {}
impl ::core::default::Default for IPV6_OPTION_JUMBOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_OPTION_ROUTER_ALERT {
    pub Header: IPV6_OPTION_HEADER,
    pub Value: [u8; 2],
}
impl ::core::marker::Copy for IPV6_OPTION_ROUTER_ALERT {}
impl ::core::clone::Clone for IPV6_OPTION_ROUTER_ALERT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_OPTION_ROUTER_ALERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_OPTION_ROUTER_ALERT").field("Header", &self.Header).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_OPTION_ROUTER_ALERT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_OPTION_ROUTER_ALERT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_OPTION_ROUTER_ALERT>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_OPTION_ROUTER_ALERT {}
impl ::core::default::Default for IPV6_OPTION_ROUTER_ALERT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPV6_OPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_PAD1: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_PADN: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_TUNNEL_LIMIT: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_ROUTER_ALERT: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_JUMBO: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(194i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP6OPT_NSAP_ADDR: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(195i32);
impl ::core::marker::Copy for IPV6_OPTION_TYPE {}
impl ::core::clone::Clone for IPV6_OPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPV6_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPV6_OPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPV6_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPV6_OPTION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_ROUTER_ADVERTISEMENT_FLAGS_0,
    pub Value: u8,
}
impl ::core::marker::Copy for IPV6_ROUTER_ADVERTISEMENT_FLAGS {}
impl ::core::clone::Clone for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_ROUTER_ADVERTISEMENT_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_ROUTER_ADVERTISEMENT_FLAGS {}
impl ::core::default::Default for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {}
impl ::core::clone::Clone for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_ROUTER_ADVERTISEMENT_FLAGS_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_ROUTER_ADVERTISEMENT_FLAGS_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {}
impl ::core::default::Default for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct IPV6_ROUTING_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
    pub RoutingType: u8,
    pub SegmentsLeft: u8,
    pub Reserved: [u8; 4],
}
impl ::core::marker::Copy for IPV6_ROUTING_HEADER {}
impl ::core::clone::Clone for IPV6_ROUTING_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPV6_ROUTING_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPV6_ROUTING_HEADER").field("NextHeader", &self.NextHeader).field("Length", &self.Length).field("RoutingType", &self.RoutingType).field("SegmentsLeft", &self.SegmentsLeft).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for IPV6_ROUTING_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPV6_ROUTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPV6_ROUTING_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPV6_ROUTING_HEADER {}
impl ::core::default::Default for IPV6_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_TRAFFIC_CLASS_MASK: u32 = 49167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IPV6_VERSION: u32 = 96u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IP_OPTION_TIMESTAMP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPTION_TIMESTAMP_ONLY: IP_OPTION_TIMESTAMP_FLAGS = IP_OPTION_TIMESTAMP_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPTION_TIMESTAMP_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS = IP_OPTION_TIMESTAMP_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_OPTION_TIMESTAMP_SPECIFIC_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS = IP_OPTION_TIMESTAMP_FLAGS(3i32);
impl ::core::marker::Copy for IP_OPTION_TIMESTAMP_FLAGS {}
impl ::core::clone::Clone for IP_OPTION_TIMESTAMP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IP_OPTION_TIMESTAMP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IP_OPTION_TIMESTAMP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for IP_OPTION_TIMESTAMP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IP_OPTION_TIMESTAMP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const IP_VER_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(IPsecDospGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospGetStatistics0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, idpstatistics: *mut IPSEC_DOSP_STATISTICS0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospGetStatistics0(enginehandle: super::super::Foundation::HANDLE, idpstatistics: *mut IPSEC_DOSP_STATISTICS0) -> u32;
        }
        ::core::mem::transmute(IPsecDospGetStatistics0(enginehandle.into_param().abi(), ::core::mem::transmute(idpstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(IPsecDospSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_DOSP_STATE_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecDospStateCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecDospStateDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_DOSP_STATE0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_DOSP_STATE0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecDospStateEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecGetStatistics0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, ipsecstatistics: *mut IPSEC_STATISTICS0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecGetStatistics0(enginehandle: super::super::Foundation::HANDLE, ipsecstatistics: *mut IPSEC_STATISTICS0) -> u32;
        }
        ::core::mem::transmute(IPsecGetStatistics0(enginehandle.into_param().abi(), ::core::mem::transmute(ipsecstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecGetStatistics1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, ipsecstatistics: *mut IPSEC_STATISTICS1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecGetStatistics1(enginehandle: super::super::Foundation::HANDLE, ipsecstatistics: *mut IPSEC_STATISTICS1) -> u32;
        }
        ::core::mem::transmute(IPsecGetStatistics1(enginehandle.into_param().abi(), ::core::mem::transmute(ipsecstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerAddAndRegister0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, keymanager: *const IPSEC_KEY_MANAGER0, keymanagercallbacks: *const IPSEC_KEY_MANAGER_CALLBACKS0, keymgmthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerAddAndRegister0(enginehandle: super::super::Foundation::HANDLE, keymanager: *const IPSEC_KEY_MANAGER0, keymanagercallbacks: *const IPSEC_KEY_MANAGER_CALLBACKS0, keymgmthandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecKeyManagerAddAndRegister0(enginehandle.into_param().abi(), ::core::mem::transmute(keymanager), ::core::mem::transmute(keymanagercallbacks), ::core::mem::transmute(keymgmthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerGetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerGetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(IPsecKeyManagerGetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerSetSecurityInfoByKey0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerSetSecurityInfoByKey0(enginehandle: super::super::Foundation::HANDLE, reserved: *const ::core::ffi::c_void, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(IPsecKeyManagerSetSecurityInfoByKey0(enginehandle.into_param().abi(), ::core::mem::transmute(reserved), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecKeyManagerUnregisterAndDelete0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, keymgmthandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerUnregisterAndDelete0(enginehandle: super::super::Foundation::HANDLE, keymgmthandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecKeyManagerUnregisterAndDelete0(enginehandle.into_param().abi(), keymgmthandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecKeyManagersGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut IPSEC_KEY_MANAGER0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagersGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut IPSEC_KEY_MANAGER0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecKeyManagersGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddInbound0(enginehandle: super::super::Foundation::HANDLE, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE0) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextAddInbound0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(inboundbundle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddInbound1(enginehandle: super::super::Foundation::HANDLE, id: u64, inboundbundle: *const IPSEC_SA_BUNDLE1) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextAddInbound1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(inboundbundle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddOutbound0(enginehandle: super::super::Foundation::HANDLE, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE0) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextAddOutbound0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(outboundbundle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddOutbound1(enginehandle: super::super::Foundation::HANDLE, id: u64, outboundbundle: *const IPSEC_SA_BUNDLE1) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextAddOutbound1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(outboundbundle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextCreate0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, outboundtraffic: *const IPSEC_TRAFFIC0, inboundfilterid: *mut u64, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreate0(enginehandle: super::super::Foundation::HANDLE, outboundtraffic: *const IPSEC_TRAFFIC0, inboundfilterid: *mut u64, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextCreate0(enginehandle.into_param().abi(), ::core::mem::transmute(outboundtraffic), ::core::mem::transmute(inboundfilterid), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextCreate1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, outboundtraffic: *const IPSEC_TRAFFIC1, virtualiftunnelinfo: *const IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid: *mut u64, id: *mut u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreate1(enginehandle: super::super::Foundation::HANDLE, outboundtraffic: *const IPSEC_TRAFFIC1, virtualiftunnelinfo: *const IPSEC_VIRTUAL_IF_TUNNEL_INFO0, inboundfilterid: *mut u64, id: *mut u64) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextCreate1(enginehandle.into_param().abi(), ::core::mem::transmute(outboundtraffic), ::core::mem::transmute(virtualiftunnelinfo), ::core::mem::transmute(inboundfilterid), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextDeleteById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextDeleteById0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_CONTEXT1, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextEnum1(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextExpire0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextExpire0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextExpire0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT0) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(sacontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, sacontext: *mut *mut IPSEC_SA_CONTEXT1) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextGetById1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(sacontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, getspi: *const IPSEC_GETSPI0, inboundspi: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetSpi0(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI0, inboundspi: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextGetSpi0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(getspi), ::core::mem::transmute(inboundspi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetSpi1(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextGetSpi1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(getspi), ::core::mem::transmute(inboundspi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextSetSpi0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSetSpi0(enginehandle: super::super::Foundation::HANDLE, id: u64, getspi: *const IPSEC_GETSPI1, inboundspi: u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextSetSpi0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(getspi), ::core::mem::transmute(inboundspi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextSubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, subscription: *const IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback: IPSEC_SA_CONTEXT_CALLBACK0, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSubscribe0(enginehandle: super::super::Foundation::HANDLE, subscription: *const IPSEC_SA_CONTEXT_SUBSCRIPTION0, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, eventshandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextSubscribe0(enginehandle.into_param().abi(), ::core::mem::transmute(subscription), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(eventshandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextSubscriptionsGet0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, entries: *mut *mut *mut IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSubscriptionsGet0(enginehandle: super::super::Foundation::HANDLE, entries: *mut *mut *mut IPSEC_SA_CONTEXT_SUBSCRIPTION0, numentries: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextSubscriptionsGet0(enginehandle.into_param().abi(), ::core::mem::transmute(entries), ::core::mem::transmute(numentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextUnsubscribe0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, eventshandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextUnsubscribe0(enginehandle: super::super::Foundation::HANDLE, eventshandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextUnsubscribe0(enginehandle.into_param().abi(), eventshandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, flags: u64, newvalues: *const IPSEC_SA_CONTEXT1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextUpdate0(enginehandle: super::super::Foundation::HANDLE, flags: u64, newvalues: *const IPSEC_SA_CONTEXT1) -> u32;
        }
        ::core::mem::transmute(IPsecSaContextUpdate0(enginehandle.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(newvalues)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const IPSEC_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IPSEC_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDbGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(IPsecSaDbGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDbSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(IPsecSaDbSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IPsecSaDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IPSEC_SA_DETAILS1, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IPsecSaEnum1(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextGetStatistics0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, ikeextstatistics: *mut IKEEXT_STATISTICS0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextGetStatistics0(enginehandle: super::super::Foundation::HANDLE, ikeextstatistics: *mut IKEEXT_STATISTICS0) -> u32;
        }
        ::core::mem::transmute(IkeextGetStatistics0(enginehandle.into_param().abi(), ::core::mem::transmute(ikeextstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextGetStatistics1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, ikeextstatistics: *mut IKEEXT_STATISTICS1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextGetStatistics1(enginehandle: super::super::Foundation::HANDLE, ikeextstatistics: *mut IKEEXT_STATISTICS1) -> u32;
        }
        ::core::mem::transmute(IkeextGetStatistics1(enginehandle.into_param().abi(), ::core::mem::transmute(ikeextstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumtemplate: *const IKEEXT_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaCreateEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumtemplate: *const IKEEXT_SA_ENUM_TEMPLATE0, enumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IkeextSaCreateEnumHandle0(enginehandle.into_param().abi(), ::core::mem::transmute(enumtemplate), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDbGetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *mut super::super::Foundation::PSID, sidgroup: *mut super::super::Foundation::PSID, dacl: *mut *mut super::super::Security::ACL, sacl: *mut *mut super::super::Security::ACL, securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(IkeextSaDbGetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDbSetSecurityInfo0(enginehandle: super::super::Foundation::HANDLE, securityinfo: u32, sidowner: *const super::super::Security::SID, sidgroup: *const super::super::Security::SID, dacl: *const super::super::Security::ACL, sacl: *const super::super::Security::ACL) -> u32;
        }
        ::core::mem::transmute(IkeextSaDbSetSecurityInfo0(enginehandle.into_param().abi(), ::core::mem::transmute(securityinfo), ::core::mem::transmute(sidowner), ::core::mem::transmute(sidgroup), ::core::mem::transmute(dacl), ::core::mem::transmute(sacl)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaDeleteById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::core::mem::transmute(IkeextSaDeleteById0(enginehandle.into_param().abi(), ::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDestroyEnumHandle0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(IkeextSaDestroyEnumHandle0(enginehandle.into_param().abi(), enumhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum0(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS0, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IkeextSaEnum0(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum1(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS1, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IkeextSaEnum1(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, enumhandle: Param1, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum2(enginehandle: super::super::Foundation::HANDLE, enumhandle: super::super::Foundation::HANDLE, numentriesrequested: u32, entries: *mut *mut *mut IKEEXT_SA_DETAILS2, numentriesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(IkeextSaEnum2(enginehandle.into_param().abi(), enumhandle.into_param().abi(), ::core::mem::transmute(numentriesrequested), ::core::mem::transmute(entries), ::core::mem::transmute(numentriesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById0<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, sa: *mut *mut IKEEXT_SA_DETAILS0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById0(enginehandle: super::super::Foundation::HANDLE, id: u64, sa: *mut *mut IKEEXT_SA_DETAILS0) -> u32;
        }
        ::core::mem::transmute(IkeextSaGetById0(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(sa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, salookupcontext: *const ::windows::core::GUID, sa: *mut *mut IKEEXT_SA_DETAILS1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById1(enginehandle: super::super::Foundation::HANDLE, id: u64, salookupcontext: *const ::windows::core::GUID, sa: *mut *mut IKEEXT_SA_DETAILS1) -> u32;
        }
        ::core::mem::transmute(IkeextSaGetById1(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(salookupcontext), ::core::mem::transmute(sa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(enginehandle: Param0, id: u64, salookupcontext: *const ::windows::core::GUID, sa: *mut *mut IKEEXT_SA_DETAILS2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById2(enginehandle: super::super::Foundation::HANDLE, id: u64, salookupcontext: *const ::windows::core::GUID, sa: *mut *mut IKEEXT_SA_DETAILS2) -> u32;
        }
        ::core::mem::transmute(IkeextSaGetById2(enginehandle.into_param().abi(), ::core::mem::transmute(id), ::core::mem::transmute(salookupcontext), ::core::mem::transmute(sa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MAX_IPV4_HLEN: u32 = 60u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MAX_IPV4_PACKET: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MAX_IPV6_PAYLOAD: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLDV2_QUERY_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub Anonymous1: MLDV2_QUERY_HEADER_0,
    pub Reserved: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub _bitfield: u8,
    pub Anonymous2: MLDV2_QUERY_HEADER_1,
    pub SourceCount: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_QUERY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_QUERY_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_QUERY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_QUERY_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MLDV2_QUERY_HEADER_0 {
    pub MaxRespCode: u16,
    pub Anonymous: MLDV2_QUERY_HEADER_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_QUERY_HEADER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_QUERY_HEADER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_QUERY_HEADER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_QUERY_HEADER_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_QUERY_HEADER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLDV2_QUERY_HEADER_0_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for MLDV2_QUERY_HEADER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLDV2_QUERY_HEADER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_QUERY_HEADER_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_QUERY_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_QUERY_HEADER_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_QUERY_HEADER_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_QUERY_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union MLDV2_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: MLDV2_QUERY_HEADER_1_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_QUERY_HEADER_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_QUERY_HEADER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_QUERY_HEADER_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_QUERY_HEADER_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_QUERY_HEADER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLDV2_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_QUERY_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_QUERY_HEADER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for MLDV2_QUERY_HEADER_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLDV2_QUERY_HEADER_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_QUERY_HEADER_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_QUERY_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_QUERY_HEADER_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_QUERY_HEADER_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_QUERY_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct MLDV2_REPORT_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub Reserved: u16,
    pub RecordCount: u16,
}
impl ::core::marker::Copy for MLDV2_REPORT_HEADER {}
impl ::core::clone::Clone for MLDV2_REPORT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MLDV2_REPORT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MLDV2_REPORT_HEADER").field("IcmpHeader", &self.IcmpHeader).field("Reserved", &self.Reserved).field("RecordCount", &self.RecordCount).finish()
    }
}
unsafe impl ::windows::core::Abi for MLDV2_REPORT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MLDV2_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_REPORT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MLDV2_REPORT_HEADER {}
impl ::core::default::Default for MLDV2_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLDV2_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLDV2_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLDV2_REPORT_RECORD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLDV2_REPORT_RECORD_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLDV2_REPORT_RECORD_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLDV2_REPORT_RECORD_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLDV2_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLDV2_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLD_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub MaxRespTime: u16,
    pub Reserved: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MLD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MLD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MLD_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MLD_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MLD_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MLD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MLD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MLD_MAX_RESP_CODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MLD_MAX_RESP_CODE_TYPE_NORMAL: MLD_MAX_RESP_CODE_TYPE = MLD_MAX_RESP_CODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MLD_MAX_RESP_CODE_TYPE_FLOAT: MLD_MAX_RESP_CODE_TYPE = MLD_MAX_RESP_CODE_TYPE(1i32);
impl ::core::marker::Copy for MLD_MAX_RESP_CODE_TYPE {}
impl ::core::clone::Clone for MLD_MAX_RESP_CODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MLD_MAX_RESP_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MLD_MAX_RESP_CODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MLD_MAX_RESP_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MLD_MAX_RESP_CODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_NA_FLAG_OVERRIDE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_NA_FLAG_ROUTER: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_NA_FLAG_SOLICITED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ND_OPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_SOURCE_LINKADDR: ND_OPTION_TYPE = ND_OPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_TARGET_LINKADDR: ND_OPTION_TYPE = ND_OPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PREFIX_INFORMATION: ND_OPTION_TYPE = ND_OPTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_REDIRECTED_HEADER: ND_OPTION_TYPE = ND_OPTION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_MTU: ND_OPTION_TYPE = ND_OPTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_NBMA_SHORTCUT_LIMIT: ND_OPTION_TYPE = ND_OPTION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_ADVERTISEMENT_INTERVAL: ND_OPTION_TYPE = ND_OPTION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_HOME_AGENT_INFORMATION: ND_OPTION_TYPE = ND_OPTION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_SOURCE_ADDR_LIST: ND_OPTION_TYPE = ND_OPTION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_TARGET_ADDR_LIST: ND_OPTION_TYPE = ND_OPTION_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_ROUTE_INFO: ND_OPTION_TYPE = ND_OPTION_TYPE(24i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_RDNSS: ND_OPTION_TYPE = ND_OPTION_TYPE(25i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_DNSSL: ND_OPTION_TYPE = ND_OPTION_TYPE(31i32);
impl ::core::marker::Copy for ND_OPTION_TYPE {}
impl ::core::clone::Clone for ND_OPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ND_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ND_OPTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ND_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ND_OPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_DNSSL_MIN_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PI_FLAG_AUTO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PI_FLAG_ONLINK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PI_FLAG_ROUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PI_FLAG_ROUTER_ADDR: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_PI_FLAG_SITE_PREFIX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_RDNSS_MIN_LEN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_OPT_RI_FLAG_PREFERENCE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_RA_FLAG_HOME_AGENT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_RA_FLAG_MANAGED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_RA_FLAG_OTHER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const ND_RA_FLAG_PREFERENCE: u32 = 24u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NPI_MODULEID {
    pub Length: u16,
    pub Type: NPI_MODULEID_TYPE,
    pub Anonymous: NPI_MODULEID_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NPI_MODULEID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NPI_MODULEID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NPI_MODULEID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NPI_MODULEID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NPI_MODULEID>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NPI_MODULEID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NPI_MODULEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union NPI_MODULEID_0 {
    pub Guid: ::windows::core::GUID,
    pub IfLuid: super::super::Foundation::LUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NPI_MODULEID_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NPI_MODULEID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NPI_MODULEID_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NPI_MODULEID_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NPI_MODULEID_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NPI_MODULEID_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NPI_MODULEID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NPI_MODULEID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MIT_GUID: NPI_MODULEID_TYPE = NPI_MODULEID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const MIT_IF_LUID: NPI_MODULEID_TYPE = NPI_MODULEID_TYPE(2i32);
impl ::core::marker::Copy for NPI_MODULEID_TYPE {}
impl ::core::clone::Clone for NPI_MODULEID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NPI_MODULEID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NPI_MODULEID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NPI_MODULEID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NPI_MODULEID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SIZEOF_IP_OPT_ROUTERALERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SIZEOF_IP_OPT_ROUTING_HEADER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SIZEOF_IP_OPT_SECURITY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SIZEOF_IP_OPT_STREAMIDENTIFIER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SIZEOF_IP_OPT_TIMESTAMP_HEADER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SNAP_CONTROL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SNAP_DSAP: u32 = 170u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct SNAP_HEADER {
    pub Dsap: u8,
    pub Ssap: u8,
    pub Control: u8,
    pub Oui: [u8; 3],
    pub Type: u16,
}
impl ::core::marker::Copy for SNAP_HEADER {}
impl ::core::clone::Clone for SNAP_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SNAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNAP_HEADER").field("Dsap", &self.Dsap).field("Ssap", &self.Ssap).field("Control", &self.Control).field("Oui", &self.Oui).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for SNAP_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SNAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SNAP_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SNAP_HEADER {}
impl ::core::default::Default for SNAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SNAP_OUI: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const SNAP_SSAP: u32 = 170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_ACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_CWR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_ECE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_FIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_EOL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_FASTOPEN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_MSS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_NOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_SACK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_SACK_PERMITTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_TS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_OPT_WS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_PSH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_RST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_SYN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TH_URG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TUNNEL_SUB_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TUNNEL_SUB_TYPE_NONE: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TUNNEL_SUB_TYPE_CP: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TUNNEL_SUB_TYPE_IPTLS: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const TUNNEL_SUB_TYPE_HA: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(3i32);
impl ::core::marker::Copy for TUNNEL_SUB_TYPE {}
impl ::core::clone::Clone for TUNNEL_SUB_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TUNNEL_SUB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TUNNEL_SUB_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TUNNEL_SUB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TUNNEL_SUB_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct VLAN_TAG {
    pub Anonymous: VLAN_TAG_0,
    pub Type: u16,
}
impl ::core::marker::Copy for VLAN_TAG {}
impl ::core::clone::Clone for VLAN_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VLAN_TAG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VLAN_TAG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VLAN_TAG>()) == 0 }
    }
}
impl ::core::cmp::Eq for VLAN_TAG {}
impl ::core::default::Default for VLAN_TAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub union VLAN_TAG_0 {
    pub Tag: u16,
    pub Anonymous: VLAN_TAG_0_0,
}
impl ::core::marker::Copy for VLAN_TAG_0 {}
impl ::core::clone::Clone for VLAN_TAG_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VLAN_TAG_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VLAN_TAG_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VLAN_TAG_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for VLAN_TAG_0 {}
impl ::core::default::Default for VLAN_TAG_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct VLAN_TAG_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for VLAN_TAG_0_0 {}
impl ::core::clone::Clone for VLAN_TAG_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VLAN_TAG_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VLAN_TAG_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for VLAN_TAG_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VLAN_TAG_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VLAN_TAG_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for VLAN_TAG_0_0 {}
impl ::core::default::Default for VLAN_TAG_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const _BIG_ENDIAN: u32 = 4321u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const _LITTLE_ENDIAN: u32 = 1234u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub const _PDP_ENDIAN: u32 = 3412u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_neighbor_advert {
    pub nd_na_hdr: ICMP_MESSAGE,
    pub nd_na_target: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_neighbor_advert {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_neighbor_advert {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_neighbor_advert {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_neighbor_advert {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_neighbor_advert>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_neighbor_advert {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_neighbor_advert {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_neighbor_solicit {
    pub nd_ns_hdr: ICMP_MESSAGE,
    pub nd_ns_target: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_neighbor_solicit {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_neighbor_solicit {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_neighbor_solicit {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_neighbor_solicit {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_neighbor_solicit>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_neighbor_solicit {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_neighbor_solicit {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_opt_dnssl {
    pub nd_opt_dnssl_type: u8,
    pub nd_opt_dnssl_len: u8,
    pub nd_opt_dnssl_reserved: u16,
    pub nd_opt_dnssl_lifetime: u32,
}
impl ::core::marker::Copy for nd_opt_dnssl {}
impl ::core::clone::Clone for nd_opt_dnssl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for nd_opt_dnssl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_dnssl").field("nd_opt_dnssl_type", &self.nd_opt_dnssl_type).field("nd_opt_dnssl_len", &self.nd_opt_dnssl_len).field("nd_opt_dnssl_reserved", &self.nd_opt_dnssl_reserved).field("nd_opt_dnssl_lifetime", &self.nd_opt_dnssl_lifetime).finish()
    }
}
unsafe impl ::windows::core::Abi for nd_opt_dnssl {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_opt_dnssl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_dnssl>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_opt_dnssl {}
impl ::core::default::Default for nd_opt_dnssl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_opt_hdr {
    pub nd_opt_type: u8,
    pub nd_opt_len: u8,
}
impl ::core::marker::Copy for nd_opt_hdr {}
impl ::core::clone::Clone for nd_opt_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for nd_opt_hdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_hdr").field("nd_opt_type", &self.nd_opt_type).field("nd_opt_len", &self.nd_opt_len).finish()
    }
}
unsafe impl ::windows::core::Abi for nd_opt_hdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_opt_hdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_hdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_opt_hdr {}
impl ::core::default::Default for nd_opt_hdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_opt_mtu {
    pub nd_opt_mtu_type: u8,
    pub nd_opt_mtu_len: u8,
    pub nd_opt_mtu_reserved: u16,
    pub nd_opt_mtu_mtu: u32,
}
impl ::core::marker::Copy for nd_opt_mtu {}
impl ::core::clone::Clone for nd_opt_mtu {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for nd_opt_mtu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_mtu").field("nd_opt_mtu_type", &self.nd_opt_mtu_type).field("nd_opt_mtu_len", &self.nd_opt_mtu_len).field("nd_opt_mtu_reserved", &self.nd_opt_mtu_reserved).field("nd_opt_mtu_mtu", &self.nd_opt_mtu_mtu).finish()
    }
}
unsafe impl ::windows::core::Abi for nd_opt_mtu {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_opt_mtu {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_mtu>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_opt_mtu {}
impl ::core::default::Default for nd_opt_mtu {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_opt_prefix_info {
    pub nd_opt_pi_type: u8,
    pub nd_opt_pi_len: u8,
    pub nd_opt_pi_prefix_len: u8,
    pub Anonymous1: nd_opt_prefix_info_0,
    pub nd_opt_pi_valid_time: u32,
    pub nd_opt_pi_preferred_time: u32,
    pub Anonymous2: nd_opt_prefix_info_1,
    pub nd_opt_pi_prefix: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_prefix_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_prefix_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_prefix_info {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_prefix_info {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_prefix_info>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_prefix_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_prefix_info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union nd_opt_prefix_info_0 {
    pub nd_opt_pi_flags_reserved: u8,
    pub Flags: nd_opt_prefix_info_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_prefix_info_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_prefix_info_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_prefix_info_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_prefix_info_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_prefix_info_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_prefix_info_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_prefix_info_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_opt_prefix_info_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_prefix_info_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_prefix_info_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for nd_opt_prefix_info_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_prefix_info_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_prefix_info_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_prefix_info_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_prefix_info_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_prefix_info_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_prefix_info_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union nd_opt_prefix_info_1 {
    pub nd_opt_pi_reserved2: u32,
    pub Anonymous: nd_opt_prefix_info_1_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_prefix_info_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_prefix_info_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_prefix_info_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_prefix_info_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_prefix_info_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_prefix_info_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_prefix_info_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_opt_prefix_info_1_0 {
    pub nd_opt_pi_reserved3: [u8; 3],
    pub nd_opt_pi_site_prefix_len: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_prefix_info_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_prefix_info_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for nd_opt_prefix_info_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_prefix_info_1_0").field("nd_opt_pi_reserved3", &self.nd_opt_pi_reserved3).field("nd_opt_pi_site_prefix_len", &self.nd_opt_pi_site_prefix_len).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_prefix_info_1_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_prefix_info_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_prefix_info_1_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_prefix_info_1_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_prefix_info_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_opt_rd_hdr {
    pub nd_opt_rh_type: u8,
    pub nd_opt_rh_len: u8,
    pub nd_opt_rh_reserved1: u16,
    pub nd_opt_rh_reserved2: u32,
}
impl ::core::marker::Copy for nd_opt_rd_hdr {}
impl ::core::clone::Clone for nd_opt_rd_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for nd_opt_rd_hdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_rd_hdr").field("nd_opt_rh_type", &self.nd_opt_rh_type).field("nd_opt_rh_len", &self.nd_opt_rh_len).field("nd_opt_rh_reserved1", &self.nd_opt_rh_reserved1).field("nd_opt_rh_reserved2", &self.nd_opt_rh_reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for nd_opt_rd_hdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_opt_rd_hdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_rd_hdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_opt_rd_hdr {}
impl ::core::default::Default for nd_opt_rd_hdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_opt_rdnss {
    pub nd_opt_rdnss_type: u8,
    pub nd_opt_rdnss_len: u8,
    pub nd_opt_rdnss_reserved: u16,
    pub nd_opt_rdnss_lifetime: u32,
}
impl ::core::marker::Copy for nd_opt_rdnss {}
impl ::core::clone::Clone for nd_opt_rdnss {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for nd_opt_rdnss {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_rdnss").field("nd_opt_rdnss_type", &self.nd_opt_rdnss_type).field("nd_opt_rdnss_len", &self.nd_opt_rdnss_len).field("nd_opt_rdnss_reserved", &self.nd_opt_rdnss_reserved).field("nd_opt_rdnss_lifetime", &self.nd_opt_rdnss_lifetime).finish()
    }
}
unsafe impl ::windows::core::Abi for nd_opt_rdnss {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_opt_rdnss {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_rdnss>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_opt_rdnss {}
impl ::core::default::Default for nd_opt_rdnss {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_opt_route_info {
    pub nd_opt_ri_type: u8,
    pub nd_opt_ri_len: u8,
    pub nd_opt_ri_prefix_len: u8,
    pub Anonymous: nd_opt_route_info_0,
    pub nd_opt_ri_route_lifetime: u32,
    pub nd_opt_ri_prefix: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_route_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_route_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_route_info {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_route_info {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_route_info>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_route_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_route_info {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union nd_opt_route_info_0 {
    pub nd_opt_ri_flags_reserved: u8,
    pub Flags: nd_opt_route_info_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_route_info_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_route_info_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_route_info_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_route_info_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_route_info_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_route_info_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_route_info_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_opt_route_info_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_opt_route_info_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_opt_route_info_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for nd_opt_route_info_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("nd_opt_route_info_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_opt_route_info_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_opt_route_info_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_opt_route_info_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_opt_route_info_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_opt_route_info_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_redirect {
    pub nd_rd_hdr: ICMP_MESSAGE,
    pub nd_rd_target: super::super::Networking::WinSock::IN6_ADDR,
    pub nd_rd_dst: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for nd_redirect {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for nd_redirect {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for nd_redirect {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for nd_redirect {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_redirect>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for nd_redirect {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for nd_redirect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_router_advert {
    pub nd_ra_hdr: ICMP_MESSAGE,
    pub nd_ra_reachable: u32,
    pub nd_ra_retransmit: u32,
}
impl ::core::marker::Copy for nd_router_advert {}
impl ::core::clone::Clone for nd_router_advert {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for nd_router_advert {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_router_advert {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_router_advert>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_router_advert {}
impl ::core::default::Default for nd_router_advert {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct nd_router_solicit {
    pub nd_rs_hdr: ICMP_MESSAGE,
}
impl ::core::marker::Copy for nd_router_solicit {}
impl ::core::clone::Clone for nd_router_solicit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for nd_router_solicit {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for nd_router_solicit {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<nd_router_solicit>()) == 0 }
    }
}
impl ::core::cmp::Eq for nd_router_solicit {}
impl ::core::default::Default for nd_router_solicit {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_hdr {
    pub th_sport: u16,
    pub th_dport: u16,
    pub th_seq: u32,
    pub th_ack: u32,
    pub _bitfield: u8,
    pub th_flags: u8,
    pub th_win: u16,
    pub th_sum: u16,
    pub th_urp: u16,
}
impl ::core::marker::Copy for tcp_hdr {}
impl ::core::clone::Clone for tcp_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tcp_hdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_hdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_hdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_hdr {}
impl ::core::default::Default for tcp_hdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_fastopen {
    pub Kind: u8,
    pub Length: u8,
    pub Cookie: [u8; 1],
}
impl ::core::marker::Copy for tcp_opt_fastopen {}
impl ::core::clone::Clone for tcp_opt_fastopen {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_opt_fastopen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_opt_fastopen").field("Kind", &self.Kind).field("Length", &self.Length).field("Cookie", &self.Cookie).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_fastopen {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_fastopen {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_fastopen>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_fastopen {}
impl ::core::default::Default for tcp_opt_fastopen {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_mss {
    pub Kind: u8,
    pub Length: u8,
    pub Mss: u16,
}
impl ::core::marker::Copy for tcp_opt_mss {}
impl ::core::clone::Clone for tcp_opt_mss {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_mss {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_mss {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_mss>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_mss {}
impl ::core::default::Default for tcp_opt_mss {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_sack {
    pub Kind: u8,
    pub Length: u8,
    pub Block: [tcp_opt_sack_0; 1],
}
impl ::core::marker::Copy for tcp_opt_sack {}
impl ::core::clone::Clone for tcp_opt_sack {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_sack {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_sack {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_sack>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_sack {}
impl ::core::default::Default for tcp_opt_sack {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_sack_0 {
    pub Left: u32,
    pub Right: u32,
}
impl ::core::marker::Copy for tcp_opt_sack_0 {}
impl ::core::clone::Clone for tcp_opt_sack_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_sack_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_sack_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_sack_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_sack_0 {}
impl ::core::default::Default for tcp_opt_sack_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_sack_permitted {
    pub Kind: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for tcp_opt_sack_permitted {}
impl ::core::clone::Clone for tcp_opt_sack_permitted {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_opt_sack_permitted {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_opt_sack_permitted").field("Kind", &self.Kind).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_sack_permitted {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_sack_permitted {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_sack_permitted>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_sack_permitted {}
impl ::core::default::Default for tcp_opt_sack_permitted {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_ts {
    pub Kind: u8,
    pub Length: u8,
    pub Val: u32,
    pub EcR: u32,
}
impl ::core::marker::Copy for tcp_opt_ts {}
impl ::core::clone::Clone for tcp_opt_ts {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_ts {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_ts {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_ts>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_ts {}
impl ::core::default::Default for tcp_opt_ts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_unknown {
    pub Kind: u8,
    pub Length: u8,
}
impl ::core::marker::Copy for tcp_opt_unknown {}
impl ::core::clone::Clone for tcp_opt_unknown {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_opt_unknown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_opt_unknown").field("Kind", &self.Kind).field("Length", &self.Length).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_unknown {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_unknown {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_unknown>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_unknown {}
impl ::core::default::Default for tcp_opt_unknown {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`*"]
pub struct tcp_opt_ws {
    pub Kind: u8,
    pub Length: u8,
    pub ShiftCnt: u8,
}
impl ::core::marker::Copy for tcp_opt_ws {}
impl ::core::clone::Clone for tcp_opt_ws {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for tcp_opt_ws {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("tcp_opt_ws").field("Kind", &self.Kind).field("Length", &self.Length).field("ShiftCnt", &self.ShiftCnt).finish()
    }
}
unsafe impl ::windows::core::Abi for tcp_opt_ws {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tcp_opt_ws {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tcp_opt_ws>()) == 0 }
    }
}
impl ::core::cmp::Eq for tcp_opt_ws {}
impl ::core::default::Default for tcp_opt_ws {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
