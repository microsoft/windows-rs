#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ARP_HARDWARE_TYPE(pub i32);
pub const ARP_HW_ENET: ARP_HARDWARE_TYPE = ARP_HARDWARE_TYPE(1i32);
pub const ARP_HW_802: ARP_HARDWARE_TYPE = ARP_HARDWARE_TYPE(6i32);
impl ::std::convert::From<i32> for ARP_HARDWARE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ARP_HARDWARE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ARP_HEADER {
    pub HardwareAddressSpace: u16,
    pub ProtocolAddressSpace: u16,
    pub HardwareAddressLength: u8,
    pub ProtocolAddressLength: u8,
    pub Opcode: u16,
    pub SenderHardwareAddress: [u8; 1],
}
impl ARP_HEADER {}
impl ::std::default::Default for ARP_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ARP_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ARP_HEADER")
            .field("HardwareAddressSpace", &self.HardwareAddressSpace)
            .field("ProtocolAddressSpace", &self.ProtocolAddressSpace)
            .field("HardwareAddressLength", &self.HardwareAddressLength)
            .field("ProtocolAddressLength", &self.ProtocolAddressLength)
            .field("Opcode", &self.Opcode)
            .field("SenderHardwareAddress", &self.SenderHardwareAddress)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ARP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.HardwareAddressSpace == other.HardwareAddressSpace
            && self.ProtocolAddressSpace == other.ProtocolAddressSpace
            && self.HardwareAddressLength == other.HardwareAddressLength
            && self.ProtocolAddressLength == other.ProtocolAddressLength
            && self.Opcode == other.Opcode
            && self.SenderHardwareAddress == other.SenderHardwareAddress
    }
}
impl ::std::cmp::Eq for ARP_HEADER {}
unsafe impl ::windows::runtime::Abi for ARP_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ARP_OPCODE(pub i32);
pub const ARP_REQUEST: ARP_OPCODE = ARP_OPCODE(1i32);
pub const ARP_RESPONSE: ARP_OPCODE = ARP_OPCODE(2i32);
impl ::std::convert::From<i32> for ARP_OPCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ARP_OPCODE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const BYTE_ORDER: u32 = 1234u32;
pub const DL_ADDRESS_LENGTH_MAXIMUM: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DL_ADDRESS_TYPE(pub i32);
pub const DlUnicast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(0i32);
pub const DlMulticast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(1i32);
pub const DlBroadcast: DL_ADDRESS_TYPE = DL_ADDRESS_TYPE(2i32);
impl ::std::convert::From<i32> for DL_ADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DL_ADDRESS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_EI48 {
    pub Byte: [u8; 3],
}
impl DL_EI48 {}
impl ::std::default::Default for DL_EI48 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EI48 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EI48 {}
unsafe impl ::windows::runtime::Abi for DL_EI48 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_EI64 {
    pub Byte: [u8; 5],
}
impl DL_EI64 {}
impl ::std::default::Default for DL_EI64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EI64 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EI64 {}
unsafe impl ::windows::runtime::Abi for DL_EI64 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_EUI48 {
    pub Byte: [u8; 6],
    pub Anonymous: DL_EUI48_0,
}
impl DL_EUI48 {}
impl ::std::default::Default for DL_EUI48 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI48 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI48 {}
unsafe impl ::windows::runtime::Abi for DL_EUI48 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DL_EUI48_0 {
    pub Oui: DL_OUI,
    pub Ei48: DL_EI48,
}
impl DL_EUI48_0 {}
impl ::std::default::Default for DL_EUI48_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI48_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI48_0 {}
unsafe impl ::windows::runtime::Abi for DL_EUI48_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_EUI64 {
    pub Byte: [u8; 8],
    pub Value: u64,
    pub Anonymous: DL_EUI64_0,
}
impl DL_EUI64 {}
impl ::std::default::Default for DL_EUI64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI64 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI64 {}
unsafe impl ::windows::runtime::Abi for DL_EUI64 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DL_EUI64_0 {
    pub Oui: DL_OUI,
    pub Anonymous: DL_EUI64_0_0,
}
impl DL_EUI64_0 {}
impl ::std::default::Default for DL_EUI64_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI64_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI64_0 {}
unsafe impl ::windows::runtime::Abi for DL_EUI64_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_EUI64_0_0 {
    pub Ei64: DL_EI64,
    pub Anonymous: DL_EUI64_0_0_0,
}
impl DL_EUI64_0_0 {}
impl ::std::default::Default for DL_EUI64_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI64_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI64_0_0 {}
unsafe impl ::windows::runtime::Abi for DL_EUI64_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DL_EUI64_0_0_0 {
    pub Type: u8,
    pub Tse: u8,
    pub Ei48: DL_EI48,
}
impl DL_EUI64_0_0_0 {}
impl ::std::default::Default for DL_EUI64_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_EUI64_0_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_EUI64_0_0_0 {}
unsafe impl ::windows::runtime::Abi for DL_EUI64_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DL_HEADER_LENGTH_MAXIMUM: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DL_OUI {
    pub Byte: [u8; 3],
    pub Anonymous: DL_OUI_0,
}
impl DL_OUI {}
impl ::std::default::Default for DL_OUI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DL_OUI {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DL_OUI {}
unsafe impl ::windows::runtime::Abi for DL_OUI {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DL_OUI_0 {
    pub _bitfield: u8,
}
impl DL_OUI_0 {}
impl ::std::default::Default for DL_OUI_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DL_OUI_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DL_OUI_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for DL_OUI_0 {}
unsafe impl ::windows::runtime::Abi for DL_OUI_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl DL_TEREDO_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union DL_TEREDO_ADDRESS_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl DL_TEREDO_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS_0_0 {
    pub Flags: u16,
    pub MappedPort: u16,
    pub MappedAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl DL_TEREDO_ADDRESS_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct DL_TEREDO_ADDRESS_PRV {
    pub Reserved: [u8; 6],
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl DL_TEREDO_ADDRESS_PRV {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS_PRV {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS_PRV {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS_PRV {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union DL_TEREDO_ADDRESS_PRV_0 {
    pub Eui64: DL_EUI64,
    pub Anonymous: DL_TEREDO_ADDRESS_PRV_0_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl DL_TEREDO_ADDRESS_PRV_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS_PRV_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS_PRV_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS_PRV_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
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
impl DL_TEREDO_ADDRESS_PRV_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for DL_TEREDO_ADDRESS_PRV_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for DL_TEREDO_ADDRESS_PRV_0_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for DL_TEREDO_ADDRESS_PRV_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
pub struct DL_TUNNEL_ADDRESS {
    pub CompartmentId: super::super::System::Kernel::COMPARTMENT_ID,
    pub ScopeId: super::super::Networking::WinSock::SCOPE_ID,
    pub IpAddress: [u8; 1],
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl DL_TUNNEL_ADDRESS {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::std::default::Default for DL_TUNNEL_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::std::cmp::PartialEq for DL_TUNNEL_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
impl ::std::cmp::Eq for DL_TUNNEL_ADDRESS {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::runtime::Abi for DL_TUNNEL_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ETHERNET_HEADER {
    pub Destination: DL_EUI48,
    pub Source: DL_EUI48,
    pub Anonymous: ETHERNET_HEADER_0,
}
impl ETHERNET_HEADER {}
impl ::std::default::Default for ETHERNET_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ETHERNET_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ETHERNET_HEADER {}
unsafe impl ::windows::runtime::Abi for ETHERNET_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union ETHERNET_HEADER_0 {
    pub Type: u16,
    pub Length: u16,
}
impl ETHERNET_HEADER_0 {}
impl ::std::default::Default for ETHERNET_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ETHERNET_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ETHERNET_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for ETHERNET_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ETHERNET_TYPE_802_1AD: u32 = 34984u32;
pub const ETHERNET_TYPE_802_1Q: u32 = 33024u32;
pub const ETHERNET_TYPE_ARP: u32 = 2054u32;
pub const ETHERNET_TYPE_IPV4: u32 = 2048u32;
pub const ETHERNET_TYPE_IPV6: u32 = 34525u32;
pub const ETHERNET_TYPE_MINIMUM: u32 = 1536u32;
pub const ETH_LENGTH_OF_HEADER: u32 = 14u32;
pub const ETH_LENGTH_OF_SNAP_HEADER: u32 = 8u32;
pub const ETH_LENGTH_OF_VLAN_HEADER: u32 = 4u32;
pub const EXT_LEN_UNIT: u32 = 8u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FALLBACK_INDEX(pub i32);
pub const FallbackIndexTcpFastopen: FALLBACK_INDEX = FALLBACK_INDEX(0i32);
pub const FallbackIndexMax: FALLBACK_INDEX = FALLBACK_INDEX(1i32);
impl ::std::convert::From<i32> for FALLBACK_INDEX {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FALLBACK_INDEX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_ACTION0 {
    pub r#type: u32,
    pub Anonymous: FWPM_ACTION0_0,
}
impl FWPM_ACTION0 {}
impl ::std::default::Default for FWPM_ACTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_ACTION0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_ACTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_ACTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_ACTION0_0 {
    pub filterType: ::windows::runtime::GUID,
    pub calloutKey: ::windows::runtime::GUID,
}
impl FWPM_ACTION0_0 {}
impl ::std::default::Default for FWPM_ACTION0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_ACTION0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_ACTION0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_ACTION0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_ACTRL_ADD: u32 = 1u32;
pub const FWPM_ACTRL_ADD_LINK: u32 = 2u32;
pub const FWPM_ACTRL_BEGIN_READ_TXN: u32 = 4u32;
pub const FWPM_ACTRL_BEGIN_WRITE_TXN: u32 = 8u32;
pub const FWPM_ACTRL_CLASSIFY: u32 = 16u32;
pub const FWPM_ACTRL_ENUM: u32 = 32u32;
pub const FWPM_ACTRL_OPEN: u32 = 64u32;
pub const FWPM_ACTRL_READ: u32 = 128u32;
pub const FWPM_ACTRL_READ_STATS: u32 = 256u32;
pub const FWPM_ACTRL_SUBSCRIBE: u32 = 512u32;
pub const FWPM_ACTRL_WRITE: u32 = 1024u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_APPC_NETWORK_CAPABILITY_TYPE(pub i32);
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT: FWPM_APPC_NETWORK_CAPABILITY_TYPE =
    FWPM_APPC_NETWORK_CAPABILITY_TYPE(0i32);
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_CLIENT_SERVER: FWPM_APPC_NETWORK_CAPABILITY_TYPE =
    FWPM_APPC_NETWORK_CAPABILITY_TYPE(1i32);
pub const FWPM_APPC_NETWORK_CAPABILITY_INTERNET_PRIVATE_NETWORK: FWPM_APPC_NETWORK_CAPABILITY_TYPE =
    FWPM_APPC_NETWORK_CAPABILITY_TYPE(2i32);
impl ::std::convert::From<i32> for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_AUTO_WEIGHT_BITS: u32 = 60u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_CALLOUT0 {
    pub calloutKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub applicableLayer: ::windows::runtime::GUID,
    pub calloutId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_CALLOUT0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_CALLOUT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_CALLOUT0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CALLOUT0")
            .field("calloutKey", &self.calloutKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("providerKey", &self.providerKey)
            .field("providerData", &self.providerData)
            .field("applicableLayer", &self.applicableLayer)
            .field("calloutId", &self.calloutId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_CALLOUT0 {
    fn eq(&self, other: &Self) -> bool {
        self.calloutKey == other.calloutKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.providerKey == other.providerKey
            && self.providerData == other.providerData
            && self.applicableLayer == other.applicableLayer
            && self.calloutId == other.calloutId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_CALLOUT0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_CALLOUT0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_1: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2006391204,
        59029,
        18358,
        [161, 153, 121, 153, 254, 201, 22, 59],
    );
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_2: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4019610038,
        31838,
        18685,
        [161, 48, 150, 103, 140, 234, 204, 65],
    );
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_3: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        410164346,
        12130,
        19424,
        [150, 111, 151, 75, 33, 184, 109, 241],
    );
pub const FWPM_CALLOUT_BUILT_IN_RESERVED_4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1816115201,
        56063,
        16617,
        [145, 230, 247, 255, 126, 82, 247, 217],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_CALLOUT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub calloutKey: ::windows::runtime::GUID,
    pub calloutId: u32,
}
impl FWPM_CALLOUT_CHANGE0 {}
impl ::std::default::Default for FWPM_CALLOUT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_CALLOUT_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CALLOUT_CHANGE0")
            .field("changeType", &self.changeType)
            .field("calloutKey", &self.calloutKey)
            .field("calloutId", &self.calloutId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_CALLOUT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType
            && self.calloutKey == other.calloutKey
            && self.calloutId == other.calloutId
    }
}
impl ::std::cmp::Eq for FWPM_CALLOUT_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CALLOUT_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_CALLOUT_CHANGE_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    change: *const FWPM_CALLOUT_CHANGE0,
);
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_LISTEN_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        860383925,
        27998,
        20069,
        [160, 11, 167, 175, 237, 11, 169, 161],
    );
pub const FWPM_CALLOUT_EDGE_TRAVERSAL_ALE_RESOURCE_ASSIGNMENT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        127602704,
        61893,
        20429,
        [174, 5, 218, 65, 16, 122, 189, 11],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_CALLOUT_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::runtime::GUID,
    pub layerKey: ::windows::runtime::GUID,
}
impl FWPM_CALLOUT_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CALLOUT_ENUM_TEMPLATE0")
            .field("providerKey", &self.providerKey)
            .field("layerKey", &self.layerKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey && self.layerKey == other.layerKey
    }
}
impl ::std::cmp::Eq for FWPM_CALLOUT_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_CALLOUT_FLAG_PERSISTENT: u32 = 65536u32;
pub const FWPM_CALLOUT_FLAG_REGISTERED: u32 = 262144u32;
pub const FWPM_CALLOUT_FLAG_USES_PROVIDER_CONTEXT: u32 = 131072u32;
pub const FWPM_CALLOUT_HTTP_TEMPLATE_SSL_HANDSHAKE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3007459913,
        36105,
        18520,
        [146, 16, 149, 199, 253, 168, 227, 15],
    );
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1791050236,
        63325,
        16899,
        [185, 200, 72, 230, 20, 156, 39, 18],
    );
pub const FWPM_CALLOUT_IPSEC_ALE_CONNECT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1275976197,
        58143,
        18022,
        [144, 176, 179, 223, 173, 52, 18, 154],
    );
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        801855212,
        52535,
        19279,
        [177, 8, 98, 194, 177, 133, 10, 12],
    );
pub const FWPM_CALLOUT_IPSEC_DOSP_FORWARD_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1829282626,
        56222,
        20414,
        [158, 210, 87, 55, 76, 232, 159, 121],
    );
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        679646771,
        50416,
        20070,
        [135, 63, 132, 77, 178, 168, 153, 199],
    );
pub const FWPM_CALLOUT_IPSEC_FORWARD_INBOUND_TUNNEL_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2941304514,
        50822,
        17050,
        [136, 77, 183, 68, 67, 231, 176, 180],
    );
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4216529206,
        5579,
        17419,
        [147, 124, 23, 23, 202, 50, 12, 64],
    );
pub const FWPM_CALLOUT_IPSEC_FORWARD_OUTBOUND_TUNNEL_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3672522956,
        57377,
        19438,
        [158, 182, 164, 139, 39, 92, 140, 29],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2113876123,
        47741,
        19130,
        [145, 170, 174, 92, 102, 64, 201, 68],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_INITIATE_SECURE_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2845890265,
        50572,
        18254,
        [138, 235, 60, 254, 153, 214, 213, 61],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1362268173,
        24196,
        19295,
        [128, 228, 1, 116, 30, 129, 255, 16],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1238609042,
        10860,
        19919,
        [149, 95, 28, 59, 224, 9, 221, 153],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1039591390,
        64800,
        18674,
        [159, 38, 248, 84, 68, 76, 186, 121],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_ALE_ACCEPT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2716046035,
        29356,
        18363,
        [135, 167, 1, 34, 198, 148, 52, 171],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        421169734,
        3064,
        18127,
        [176, 69, 75, 69, 223, 166, 163, 36],
    );
pub const FWPM_CALLOUT_IPSEC_INBOUND_TUNNEL_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2160280291,
        7763,
        19823,
        [155, 68, 3, 223, 90, 238, 225, 84],
    );
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1262927626,
        17699,
        20055,
        [170, 56, 168, 121, 135, 201, 16, 217],
    );
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        953710370,
        44419,
        20241,
        [169, 31, 223, 15, 176, 119, 34, 91],
    );
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1889802604,
        33627,
        20400,
        [152, 232, 7, 95, 77, 151, 125, 70],
    );
pub const FWPM_CALLOUT_IPSEC_OUTBOUND_TUNNEL_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4051915619,
        42661,
        20066,
        [177, 128, 35, 219, 120, 157, 141, 166],
    );
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1606402845,
        42268,
        17628,
        [172, 182, 6, 36, 160, 48, 167, 0],
    );
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_CONNECT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1606402845,
        42268,
        17628,
        [172, 182, 6, 36, 160, 48, 167, 1],
    );
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1606402845,
        42268,
        17628,
        [172, 182, 6, 36, 160, 48, 167, 2],
    );
pub const FWPM_CALLOUT_POLICY_SILENT_MODE_AUTH_RECV_ACCEPT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1606402845,
        42268,
        17628,
        [172, 182, 6, 36, 160, 48, 167, 3],
    );
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        680219213,
        1382,
        19993,
        [182, 18, 143, 68, 26, 46, 89, 73],
    );
pub const FWPM_CALLOUT_RESERVED_AUTH_CONNECT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        12077970,
        11102,
        19313,
        [171, 14, 170, 202, 67, 227, 135, 230],
    );
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3159892608,
        5751,
        16873,
        [148, 171, 194, 252, 177, 92, 46, 235],
    );
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_CONNECT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2565158716,
        47236,
        18703,
        [182, 95, 47, 106, 74, 87, 81, 149],
    );
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        760606728,
        3073,
        20370,
        [178, 110, 160, 138, 148, 86, 155, 141],
    );
pub const FWPM_CALLOUT_SET_OPTIONS_AUTH_RECV_ACCEPT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1661044023,
        62081,
        19908,
        [131, 211, 141, 236, 24, 183, 173, 226],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_CALLOUT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CALLOUT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_CALLOUT_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CALLOUT_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_CALLOUT_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CALLOUT_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3783519410,
        14975,
        19284,
        [138, 217, 118, 5, 14, 216, 128, 202],
    );
pub const FWPM_CALLOUT_TCP_CHIMNEY_ACCEPT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        58249025,
        49048,
        17923,
        [129, 242, 127, 18, 88, 96, 121, 246],
    );
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4091611827,
        11301,
        17017,
        [172, 54, 195, 15, 193, 129, 190, 196],
    );
pub const FWPM_CALLOUT_TCP_CHIMNEY_CONNECT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        971120773,
        41793,
        17148,
        [162, 121, 174, 201, 78, 104, 156, 86],
    );
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        790885840,
        16580,
        19521,
        [162, 84, 70, 216, 219, 168, 149, 124],
    );
pub const FWPM_CALLOUT_TCP_TEMPLATES_ACCEPT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2991674096,
        39196,
        20307,
        [187, 231, 210, 75, 69, 254, 99, 44],
    );
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        559549241,
        19326,
        20186,
        [140, 228, 23, 150, 121, 223, 98, 36],
    );
pub const FWPM_CALLOUT_TCP_TEMPLATES_CONNECT_LAYER_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2206939041,
        23570,
        19764,
        [139, 56, 7, 135, 40, 178, 210, 92],
    );
pub const FWPM_CALLOUT_TEREDO_ALE_LISTEN_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2175022311,
        62988,
        17272,
        [186, 184, 198, 37, 163, 15, 1, 151],
    );
pub const FWPM_CALLOUT_TEREDO_ALE_RESOURCE_ASSIGNMENT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        834229138,
        1646,
        17058,
        [183, 219, 146, 248, 172, 221, 86, 249],
    );
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V4_SILENT_DROP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3986720262,
        9364,
        19832,
        [137, 188, 103, 131, 124, 3, 185, 105],
    );
pub const FWPM_CALLOUT_WFP_TRANSPORT_LAYER_V6_SILENT_DROP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2257833076,
        41077,
        16726,
        [180, 118, 146, 134, 238, 206, 129, 78],
    );
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_CHANGE_TYPE(pub i32);
pub const FWPM_CHANGE_ADD: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(1i32);
pub const FWPM_CHANGE_DELETE: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(2i32);
pub const FWPM_CHANGE_TYPE_MAX: FWPM_CHANGE_TYPE = FWPM_CHANGE_TYPE(3i32);
impl ::std::convert::From<i32> for FWPM_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_CHANGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_CLASSIFY_OPTION0 {
    pub r#type: FWP_CLASSIFY_OPTION_TYPE,
    pub value: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_CLASSIFY_OPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_CLASSIFY_OPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_CLASSIFY_OPTION0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_CLASSIFY_OPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_CLASSIFY_OPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_CLASSIFY_OPTIONS0 {
    pub numOptions: u32,
    pub options: *mut FWPM_CLASSIFY_OPTION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_CLASSIFY_OPTIONS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_CLASSIFY_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_CLASSIFY_OPTIONS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CLASSIFY_OPTIONS0")
            .field("numOptions", &self.numOptions)
            .field("options", &self.options)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_CLASSIFY_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numOptions == other.numOptions && self.options == other.options
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_CLASSIFY_OPTIONS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_CLASSIFY_OPTIONS0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_CONDITION_ALE_APP_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3616415367,
        34372,
        20133,
        [148, 55, 216, 9, 236, 239, 201, 113],
    );
pub const FWPM_CONDITION_ALE_EFFECTIVE_NAME: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2972154778,
        46977,
        16636,
        [150, 113, 229, 241, 185, 137, 243, 78],
    );
pub const FWPM_CONDITION_ALE_NAP_CONTEXT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1176984221,
        49215,
        19831,
        [183, 132, 28, 87, 244, 208, 39, 83],
    );
pub const FWPM_CONDITION_ALE_ORIGINAL_APP_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        242012294,
        57851,
        16914,
        [132, 47, 138, 159, 153, 63, 179, 246],
    );
pub const FWPM_CONDITION_ALE_PACKAGE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1908177146,
        61820,
        18839,
        [166, 2, 106, 187, 38, 31, 53, 28],
    );
pub const FWPM_CONDITION_ALE_PROMISCUOUS_MODE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        479676278,
        29058,
        18153,
        [175, 211, 176, 41, 16, 227, 3, 52],
    );
pub const FWPM_CONDITION_ALE_REAUTH_REASON: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3028472359,
        6521,
        19096,
        [128, 68, 24, 187, 230, 35, 117, 66],
    );
pub const FWPM_CONDITION_ALE_REMOTE_MACHINE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        446988113,
        32659,
        17672,
        [162, 113, 129, 171, 176, 12, 156, 171],
    );
pub const FWPM_CONDITION_ALE_REMOTE_USER_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4130370487,
        393,
        19120,
        [149, 164, 97, 35, 203, 250, 184, 98],
    );
pub const FWPM_CONDITION_ALE_SECURITY_ATTRIBUTE_FQBN_VALUE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        933590681,
        22659,
        18787,
        [146, 184, 62, 112, 70, 136, 176, 173],
    );
pub const FWPM_CONDITION_ALE_SIO_FIREWALL_SYSTEM_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3119833224,
        52120,
        20219,
        [162, 199, 173, 7, 51, 38, 67, 219],
    );
pub const FWPM_CONDITION_ALE_USER_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2936289802,
        45901,
        20358,
        [151, 156, 201, 3, 113, 175, 110, 102],
    );
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3423112627,
        6034,
        19057,
        [176, 249, 3, 125, 33, 205, 130, 139],
    );
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_PROFILE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3456002731,
        49283,
        16706,
        [134, 121, 192, 143, 149, 50, 156, 97],
    );
pub const FWPM_CONDITION_ARRIVAL_INTERFACE_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2314834142,
        59288,
        20077,
        [171, 118, 124, 149, 88, 41, 46, 111],
    );
pub const FWPM_CONDITION_ARRIVAL_TUNNEL_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1360094940,
        31372,
        19111,
        [181, 51, 149, 171, 89, 251, 3, 64],
    );
pub const FWPM_CONDITION_AUTHENTICATION_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3947203797,
        55931,
        20217,
        [141, 67, 123, 10, 132, 3, 50, 242],
    );
pub const FWPM_CONDITION_CLIENT_CERT_KEY_LENGTH: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2750152903,
        1524,
        19959,
        [145, 242, 95, 96, 217, 31, 244, 67],
    );
pub const FWPM_CONDITION_CLIENT_CERT_OID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3297881438,
        63618,
        17027,
        [185, 22, 67, 107, 16, 63, 244, 173],
    );
pub const FWPM_CONDITION_CLIENT_TOKEN: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3257465886,
        16442,
        17528,
        [190, 5, 201, 186, 164, 192, 90, 206],
    );
pub const FWPM_CONDITION_COMPARTMENT_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        900174251,
        1196,
        20466,
        [166, 187, 218, 108, 250, 199, 24, 6],
    );
pub const FWPM_CONDITION_CURRENT_PROFILE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2872062921,
        49379,
        18265,
        [147, 125, 87, 88, 198, 93, 74, 227],
    );
pub const FWPM_CONDITION_DCOM_APP_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4281236301,
        12562,
        18288,
        [182, 54, 77, 36, 174, 58, 106, 242],
    );
pub const FWPM_CONDITION_DESTINATION_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        902784290,
        16697,
        17902,
        [160, 213, 103, 184, 9, 73, 216, 121],
    );
pub const FWPM_CONDITION_DESTINATION_SUB_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        729629593,
        54471,
        18232,
        [162, 245, 233, 148, 180, 61, 163, 136],
    );
pub const FWPM_CONDITION_DIRECTION: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2273624390,
        51863,
        17622,
        [159, 209, 25, 251, 24, 64, 203, 247],
    );
pub const FWPM_CONDITION_EMBEDDED_LOCAL_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1181918312,
        35338,
        16898,
        [171, 180, 132, 158, 146, 230, 104, 9],
    );
pub const FWPM_CONDITION_EMBEDDED_LOCAL_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3217701197,
        44251,
        18510,
        [184, 230, 42, 255, 121, 117, 115, 69],
    );
pub const FWPM_CONDITION_EMBEDDED_PROTOCOL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        125321479,
        41630,
        19579,
        [158, 199, 41, 196, 74, 250, 253, 188],
    );
pub const FWPM_CONDITION_EMBEDDED_REMOTE_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2012105529,
        12915,
        18033,
        [182, 59, 171, 111, 235, 102, 238, 182],
    );
pub const FWPM_CONDITION_EMBEDDED_REMOTE_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3403994785,
        10600,
        16621,
        [164, 206, 84, 113, 96, 221, 168, 141],
    );
pub const FWPM_CONDITION_ETHER_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4245197965,
        41497,
        19794,
        [187, 152, 26, 85, 64, 238, 123, 78],
    );
pub const FWPM_CONDITION_FLAGS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1663885883,
    20839,
    17244,
    [134, 215, 233, 3, 104, 74, 168, 12],
);
pub const FWPM_CONDITION_IMAGE_NAME: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3492077133,
        57002,
        17175,
        [156, 133, 228, 14, 246, 225, 64, 195],
    );
pub const FWPM_CONDITION_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1719654229,
        54933,
        17226,
        [138, 245, 211, 131, 90, 18, 89, 188],
    );
pub const FWPM_CONDITION_INTERFACE_MAC_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4142284238,
        8011,
        19563,
        [182, 239, 17, 101, 231, 31, 142, 231],
    );
pub const FWPM_CONDITION_INTERFACE_QUARANTINE_EPOCH: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3437661534,
        1339,
        17320,
        [154, 111, 51, 56, 76, 40, 228, 246],
    );
pub const FWPM_CONDITION_INTERFACE_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3673738516,
        57502,
        19603,
        [165, 174, 197, 193, 59, 115, 255, 202],
    );
pub const FWPM_CONDITION_IPSEC_POLICY_KEY: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2906119907,
        29231,
        17868,
        [164, 227, 6, 128, 72, 18, 68, 82],
    );
pub const FWPM_CONDITION_IPSEC_SECURITY_REALM_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        933590784,
        22660,
        18788,
        [146, 184, 62, 112, 70, 136, 176, 173],
    );
pub const FWPM_CONDITION_IP_ARRIVAL_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1636473709,
        14443,
        16694,
        [173, 110, 181, 21, 135, 207, 177, 205],
    );
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        762909499,
        45968,
        17862,
        [134, 153, 172, 172, 234, 175, 237, 51],
    );
pub const FWPM_CONDITION_IP_DESTINATION_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        516011977,
        20202,
        20318,
        [185, 239, 118, 190, 170, 175, 23, 238],
    );
pub const FWPM_CONDITION_IP_DESTINATION_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3463311173,
        24827,
        19067,
        [163, 4, 175, 48, 161, 23, 0, 14],
    );
pub const FWPM_CONDITION_IP_FORWARD_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        276215973,
        25379,
        19550,
        [152, 16, 232, 211, 252, 158, 97, 54],
    );
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3656253662,
        49647,
        17943,
        [191, 227, 255, 216, 245, 160, 137, 87],
    );
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1858598596,
        14187,
        17879,
        [158, 156, 211, 55, 206, 220, 210, 55],
    );
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(61221323, 28242, 18936, [156, 65, 87, 9, 99, 60, 9, 207]);
pub const FWPM_CONDITION_IP_LOCAL_ADDRESS_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        595705476,
        29988,
        17843,
        [160, 91, 30, 99, 125, 156, 122, 106],
    );
pub const FWPM_CONDITION_IP_LOCAL_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1289103945,
        22979,
        18793,
        [183, 243, 189, 165, 211, 40, 144, 164],
    );
pub const FWPM_CONDITION_IP_LOCAL_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        203137455,
        22373,
        17727,
        [175, 34, 168, 247, 145, 172, 119, 91],
    );
pub const FWPM_CONDITION_IP_NEXTHOP_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3938337930,
        42769,
        19812,
        [133, 183, 63, 118, 182, 82, 153, 199],
    );
pub const FWPM_CONDITION_IP_NEXTHOP_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2477690715,
        32623,
        18201,
        [152, 200, 20, 233, 116, 41, 239, 4],
    );
pub const FWPM_CONDITION_IP_PHYSICAL_ARRIVAL_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3662730696,
        64013,
        19593,
        [176, 50, 110, 98, 19, 109, 30, 150],
    );
pub const FWPM_CONDITION_IP_PHYSICAL_NEXTHOP_INTERFACE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4036744654,
        20816,
        18622,
        [176, 152, 194, 81, 82, 251, 31, 146],
    );
pub const FWPM_CONDITION_IP_PROTOCOL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        963768107,
        25150,
        20378,
        [140, 177, 110, 121, 184, 6, 185, 167],
    );
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2989862554,
        7524,
        18872,
        [164, 76, 95, 243, 217, 9, 80, 69],
    );
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        535541264,
        15308,
        17889,
        [188, 54, 46, 6, 126, 44, 177, 134],
    );
pub const FWPM_CONDITION_IP_REMOTE_ADDRESS_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        611196300,
        35822,
        16408,
        [155, 152, 49, 212, 88, 47, 51, 97],
    );
pub const FWPM_CONDITION_IP_REMOTE_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3277480013,
        53803,
        19994,
        [145, 180, 104, 246, 116, 238, 103, 75],
    );
pub const FWPM_CONDITION_IP_SOURCE_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2929101182,
        11924,
        19401,
        [179, 19, 178, 126, 232, 14, 87, 77],
    );
pub const FWPM_CONDITION_IP_SOURCE_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2796547985,
        15860,
        18224,
        [162, 20, 245, 66, 106, 235, 248, 33],
    );
pub const FWPM_CONDITION_KM_AUTH_NAP_CONTEXT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        902883854,
        5578,
        18731,
        [144, 14, 151, 253, 70, 53, 44, 206],
    );
pub const FWPM_CONDITION_KM_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4277093762,
    61327,
    20347,
    [133, 139, 144, 119, 209, 34, 222, 71],
);
pub const FWPM_CONDITION_KM_TYPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4279197513,
    3307,
    18459,
    [134, 56, 20, 121, 121, 31, 63, 44],
);
pub const FWPM_CONDITION_L2_FLAGS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2076458175,
    14266,
    17905,
    [183, 74, 130, 255, 81, 142, 235, 16],
);
pub const FWPM_CONDITION_LOCAL_INTERFACE_PROFILE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1321170274,
        40728,
        19718,
        [153, 65, 167, 166, 37, 116, 77, 113],
    );
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        82455187,
        34188,
        16423,
        [182, 19, 180, 49, 128, 199, 133, 158],
    );
pub const FWPM_CONDITION_MAC_DESTINATION_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2919573810,
        61250,
        20121,
        [177, 41, 243, 179, 19, 158, 52, 247],
    );
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3650742657,
        31048,
        19587,
        [183, 66, 200, 78, 59, 103, 143, 143],
    );
pub const FWPM_CONDITION_MAC_LOCAL_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3425776988,
        12403,
        20475,
        [161, 79, 121, 65, 92, 177, 234, 209],
    );
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1083125460,
        14960,
        19277,
        [146, 166, 65, 90, 194, 14, 47, 18],
    );
pub const FWPM_CONDITION_MAC_REMOTE_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        41938356,
        61889,
        16432,
        [181, 100, 238, 119, 127, 216, 103, 234],
    );
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2071549009,
        61942,
        19717,
        [183, 203, 33, 119, 157, 128, 35, 54],
    );
pub const FWPM_CONDITION_MAC_SOURCE_ADDRESS_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1545302756,
        10654,
        17463,
        [162, 152, 188, 63, 1, 75, 61, 194],
    );
pub const FWPM_CONDITION_NDIS_MEDIA_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3409039089,
        31005,
        18235,
        [137, 209, 97, 197, 152, 67, 4, 160],
    );
pub const FWPM_CONDITION_NDIS_PHYSICAL_MEDIA_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        885495843,
        49705,
        17650,
        [184, 60, 116, 2, 8, 130, 174, 119],
    );
pub const FWPM_CONDITION_NDIS_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3682317355,
        11692,
        19668,
        [165, 154, 224, 189, 206, 30, 104, 52],
    );
pub const FWPM_CONDITION_NET_EVENT_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        544119190,
        18702,
        16591,
        [184, 49, 179, 134, 65, 235, 111, 203],
    );
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        328099976,
        31416,
        19813,
        [158, 232, 5, 145, 188, 246, 164, 148],
    );
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_PROFILE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3623852630,
        52650,
        18219,
        [132, 219, 210, 57, 99, 193, 209, 191],
    );
pub const FWPM_CONDITION_NEXTHOP_INTERFACE_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2538830956,
        55715,
        18279,
        [163, 129, 233, 66, 103, 92, 217, 32],
    );
pub const FWPM_CONDITION_NEXTHOP_SUB_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4018823458,
        1399,
        17831,
        [154, 175, 130, 95, 190, 180, 251, 149],
    );
pub const FWPM_CONDITION_NEXTHOP_TUNNEL_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1924243729,
        39035,
        18208,
        [153, 221, 199, 197, 118, 250, 45, 76],
    );
pub const FWPM_CONDITION_ORIGINAL_ICMP_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        124648894,
        50540,
        20338,
        [174, 138, 44, 254, 126, 92, 130, 134],
    );
pub const FWPM_CONDITION_ORIGINAL_PROFILE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1189746001,
        8789,
        18731,
        [128, 25, 170, 190, 238, 52, 159, 64],
    );
pub const FWPM_CONDITION_PEER_NAME: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2605944962,
        60304,
        16774,
        [166, 204, 222, 91, 99, 35, 80, 22],
    );
pub const FWPM_CONDITION_PIPE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    466646045,
    58335,
    20004,
    [134, 52, 118, 32, 70, 238, 246, 235],
);
pub const FWPM_CONDITION_PROCESS_WITH_RPC_IF_UUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3809575080,
        48061,
        19732,
        [166, 94, 113, 87, 176, 98, 51, 187],
    );
pub const FWPM_CONDITION_QM_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4132423377,
    63947,
    17362,
    [138, 95, 225, 59, 200, 148, 242, 101],
);
pub const FWPM_CONDITION_REAUTHORIZE_REASON: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        287334028,
        4526,
        17786,
        [138, 68, 71, 112, 38, 221, 118, 74],
    );
pub const FWPM_CONDITION_REMOTE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4135675645,
        1666,
        19593,
        [184, 245, 134, 67, 108, 126, 249, 183],
    );
pub const FWPM_CONDITION_REMOTE_USER_TOKEN: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2616258150,
        1737,
        16825,
        [132, 218, 40, 140, 180, 58, 245, 31],
    );
pub const FWPM_CONDITION_RESERVED0: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1737444843,
        17839,
        18562,
        [147, 254, 25, 212, 114, 157, 152, 52],
    );
pub const FWPM_CONDITION_RESERVED1: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3625515047,
        23657,
        18667,
        [191, 128, 216, 107, 23, 117, 95, 151],
    );
pub const FWPM_CONDITION_RESERVED10: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3111772802,
        54817,
        19596,
        [177, 132, 177, 5, 166, 28, 54, 206],
    );
pub const FWPM_CONDITION_RESERVED11: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        761458253,
        573,
        16671,
        [149, 130, 67, 172, 187, 121, 89, 117],
    );
pub const FWPM_CONDITION_RESERVED12: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2741468210,
        32309,
        19932,
        [147, 218, 232, 195, 63, 201, 35, 199],
    );
pub const FWPM_CONDITION_RESERVED13: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        861552272,
        33962,
        17141,
        [158, 111, 89, 48, 149, 54, 164, 76],
    );
pub const FWPM_CONDITION_RESERVED14: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        820268450,
        12058,
        16662,
        [165, 89, 249, 7, 222, 131, 96, 74],
    );
pub const FWPM_CONDITION_RESERVED15: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3132634127,
        45024,
        17361,
        [128, 216, 92, 164, 86, 150, 45, 227],
    );
pub const FWPM_CONDITION_RESERVED2: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1406407229,
        57691,
        20100,
        [183, 168, 220, 225, 111, 123, 98, 217],
    );
pub const FWPM_CONDITION_RESERVED3: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2137951395,
        26118,
        18738,
        [151, 199, 225, 242, 7, 16, 175, 59],
    );
pub const FWPM_CONDITION_RESERVED4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1599661634,
        47415,
        18782,
        [169, 75, 246, 176, 81, 164, 146, 80],
    );
pub const FWPM_CONDITION_RESERVED5: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2611541709,
        63356,
        17382,
        [136, 71, 17, 147, 157, 197, 219, 90],
    );
pub const FWPM_CONDITION_RESERVED6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4047340733,
        22997,
        17604,
        [136, 23, 94, 205, 174, 24, 5, 189],
    );
pub const FWPM_CONDITION_RESERVED7: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1705048368,
        17885,
        18819,
        [170, 51, 239, 199, 182, 17, 175, 8],
    );
pub const FWPM_CONDITION_RESERVED8: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1329744244,
        3090,
        18454,
        [155, 71, 154, 84, 125, 179, 154, 50],
    );
pub const FWPM_CONDITION_RESERVED9: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3464028431,
        5119,
        19568,
        [134, 67, 54, 173, 24, 121, 175, 163],
    );
pub const FWPM_CONDITION_RPC_AUTH_LEVEL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3852512981,
        22956,
        18154,
        [190, 5, 165, 240, 94, 207, 68, 110],
    );
pub const FWPM_CONDITION_RPC_AUTH_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3669652651,
        3431,
        17383,
        [152, 110, 117, 184, 79, 130, 245, 148],
    );
pub const FWPM_CONDITION_RPC_EP_FLAGS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        562790730,
        2617,
        18872,
        [142, 113, 194, 12, 57, 199, 221, 46],
    );
pub const FWPM_CONDITION_RPC_EP_VALUE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3704529081,
        2182,
        17248,
        [156, 106, 171, 4, 58, 36, 251, 169],
    );
pub const FWPM_CONDITION_RPC_IF_FLAG: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        596281906,
        12697,
        18045,
        [135, 28, 39, 38, 33, 171, 56, 150],
    );
pub const FWPM_CONDITION_RPC_IF_UUID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2090630559,
        117,
        19765,
        [160, 209, 131, 17, 196, 207, 106, 241],
    );
pub const FWPM_CONDITION_RPC_IF_VERSION: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3938441655,
        4706,
        18990,
        [173, 170, 95, 150, 246, 254, 50, 109],
    );
pub const FWPM_CONDITION_RPC_PROTOCOL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        655866996,
        14901,
        19687,
        [183, 239, 200, 56, 250, 189, 236, 69],
    );
pub const FWPM_CONDITION_RPC_PROXY_AUTH_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1083523042,
        34149,
        18265,
        [132, 136, 23, 113, 180, 180, 181, 219],
    );
pub const FWPM_CONDITION_RPC_SERVER_NAME: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3053822501,
        50099,
        18631,
        [152, 51, 122, 239, 169, 82, 117, 70],
    );
pub const FWPM_CONDITION_RPC_SERVER_PORT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2156983877,
        39637,
        20027,
        [159, 159, 128, 35, 202, 9, 121, 9],
    );
pub const FWPM_CONDITION_SEC_ENCRYPT_ALGORITHM: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        221277936,
        59764,
        20340,
        [181, 199, 89, 27, 13, 167, 213, 98],
    );
pub const FWPM_CONDITION_SEC_KEY_SIZE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1198659643,
        52472,
        19179,
        [188, 225, 198, 198, 22, 28, 143, 228],
    );
pub const FWPM_CONDITION_SOURCE_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        588329805,
        51501,
        17855,
        [148, 150, 237, 244, 71, 130, 14, 45],
    );
pub const FWPM_CONDITION_SOURCE_SUB_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        90103197,
        44242,
        17249,
        [141, 171, 249, 82, 93, 151, 102, 47],
    );
pub const FWPM_CONDITION_SUB_INTERFACE_INDEX: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        215229555,
        54817,
        19427,
        [174, 140, 114, 163, 72, 210, 131, 225],
    );
pub const FWPM_CONDITION_TUNNEL_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2007237687,
        34681,
        18536,
        [162, 97, 245, 169, 2, 241, 192, 205],
    );
pub const FWPM_CONDITION_VLAN_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2475600673,
    13848,
    20068,
    [156, 165, 33, 65, 235, 218, 28, 162],
);
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2396294116,
        51494,
        18934,
        [164, 246, 239, 48, 48, 227, 252, 22],
    );
pub const FWPM_CONDITION_VSWITCH_DESTINATION_INTERFACE_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4204478214,
        12058,
        19543,
        [158, 104, 167, 9, 139, 40, 219, 254],
    );
pub const FWPM_CONDITION_VSWITCH_DESTINATION_VM_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1627826894,
        19937,
        19588,
        [150, 113, 54, 55, 248, 188, 247, 49],
    );
pub const FWPM_CONDITION_VSWITCH_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3299087546,
        17275,
        19942,
        [153, 70, 217, 156, 27, 149, 179, 18],
    );
pub const FWPM_CONDITION_VSWITCH_NETWORK_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        299141963,
        59258,
        16564,
        [145, 85, 57, 44, 144, 108, 38, 8],
    );
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2135880267,
        45761,
        18744,
        [186, 51, 161, 236, 190, 213, 18, 186],
    );
pub const FWPM_CONDITION_VSWITCH_SOURCE_INTERFACE_TYPE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3870310562,
        60847,
        19510,
        [144, 139, 242, 245, 138, 228, 56, 7],
    );
pub const FWPM_CONDITION_VSWITCH_SOURCE_VM_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2620038850,
        40902,
        17084,
        [189, 216, 64, 109, 77, 160, 190, 100],
    );
pub const FWPM_CONDITION_VSWITCH_TENANT_NETWORK_ID: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3691283516,
        31206,
        20036,
        [160, 37, 101, 185, 187, 15, 159, 148],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_CONNECTION0 {
    pub connectionId: u64,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: FWPM_CONNECTION0_0,
    pub Anonymous2: FWPM_CONNECTION0_1,
    pub providerKey: *mut ::windows::runtime::GUID,
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
impl FWPM_CONNECTION0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_CONNECTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_CONNECTION0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_CONNECTION0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_CONNECTION0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl FWPM_CONNECTION0_0 {}
impl ::std::default::Default for FWPM_CONNECTION0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_CONNECTION0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_CONNECTION0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_CONNECTION0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl FWPM_CONNECTION0_1 {}
impl ::std::default::Default for FWPM_CONNECTION0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_CONNECTION0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_CONNECTION0_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type FWPM_CONNECTION_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    eventtype: FWPM_CONNECTION_EVENT_TYPE,
    connection: *const FWPM_CONNECTION0,
);
pub const FWPM_CONNECTION_ENUM_FLAG_QUERY_BYTES_TRANSFERRED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_CONNECTION_ENUM_TEMPLATE0 {
    pub connectionId: u64,
    pub flags: u32,
}
impl FWPM_CONNECTION_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CONNECTION_ENUM_TEMPLATE0")
            .field("connectionId", &self.connectionId)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.connectionId == other.connectionId && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for FWPM_CONNECTION_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_CONNECTION_EVENT_TYPE(pub i32);
pub const FWPM_CONNECTION_EVENT_ADD: FWPM_CONNECTION_EVENT_TYPE = FWPM_CONNECTION_EVENT_TYPE(0i32);
pub const FWPM_CONNECTION_EVENT_DELETE: FWPM_CONNECTION_EVENT_TYPE =
    FWPM_CONNECTION_EVENT_TYPE(1i32);
pub const FWPM_CONNECTION_EVENT_MAX: FWPM_CONNECTION_EVENT_TYPE = FWPM_CONNECTION_EVENT_TYPE(2i32);
impl ::std::convert::From<i32> for FWPM_CONNECTION_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_CONNECTION_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_CONNECTION_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_CONNECTION_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_CONNECTION_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_CONNECTION_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_CONNECTION_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_DISPLAY_DATA0 {
    pub name: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_DISPLAY_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_DISPLAY_DATA0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_DISPLAY_DATA0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_DISPLAY_DATA0")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_DISPLAY_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_DISPLAY_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_DISPLAY_DATA0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_DYNAMIC_KEYWORD_CALLBACK0 = unsafe extern "system" fn(
    notification: *mut ::std::ffi::c_void,
    context: *mut ::std::ffi::c_void,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_ENGINE_OPTION(pub i32);
pub const FWPM_ENGINE_COLLECT_NET_EVENTS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(0i32);
pub const FWPM_ENGINE_NET_EVENT_MATCH_ANY_KEYWORDS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(1i32);
pub const FWPM_ENGINE_NAME_CACHE: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(2i32);
pub const FWPM_ENGINE_MONITOR_IPSEC_CONNECTIONS: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(3i32);
pub const FWPM_ENGINE_PACKET_QUEUING: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(4i32);
pub const FWPM_ENGINE_TXN_WATCHDOG_TIMEOUT_IN_MSEC: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(5i32);
pub const FWPM_ENGINE_OPTION_MAX: FWPM_ENGINE_OPTION = FWPM_ENGINE_OPTION(6i32);
impl ::std::convert::From<i32> for FWPM_ENGINE_OPTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_ENGINE_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_ENGINE_OPTION_PACKET_BATCH_INBOUND: u32 = 4u32;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_FORWARD: u32 = 2u32;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_INBOUND: u32 = 1u32;
pub const FWPM_ENGINE_OPTION_PACKET_QUEUE_NONE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_FIELD0 {
    pub fieldKey: *mut ::windows::runtime::GUID,
    pub r#type: FWPM_FIELD_TYPE,
    pub dataType: FWP_DATA_TYPE,
}
impl FWPM_FIELD0 {}
impl ::std::default::Default for FWPM_FIELD0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_FIELD0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_FIELD0")
            .field("fieldKey", &self.fieldKey)
            .field("r#type", &self.r#type)
            .field("dataType", &self.dataType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_FIELD0 {
    fn eq(&self, other: &Self) -> bool {
        self.fieldKey == other.fieldKey
            && self.r#type == other.r#type
            && self.dataType == other.dataType
    }
}
impl ::std::cmp::Eq for FWPM_FIELD0 {}
unsafe impl ::windows::runtime::Abi for FWPM_FIELD0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_FIELD_TYPE(pub i32);
pub const FWPM_FIELD_RAW_DATA: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(0i32);
pub const FWPM_FIELD_IP_ADDRESS: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(1i32);
pub const FWPM_FIELD_FLAGS: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(2i32);
pub const FWPM_FIELD_TYPE_MAX: FWPM_FIELD_TYPE = FWPM_FIELD_TYPE(3i32);
impl ::std::convert::From<i32> for FWPM_FIELD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_FIELD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER0 {
    pub filterKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: FWPM_FILTER_FLAGS,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub layerKey: ::windows::runtime::GUID,
    pub subLayerKey: ::windows::runtime::GUID,
    pub weight: FWP_VALUE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub action: FWPM_ACTION0,
    pub Anonymous: FWPM_FILTER0_0,
    pub reserved: *mut ::windows::runtime::GUID,
    pub filterId: u64,
    pub effectiveWeight: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_FILTER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_FILTER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_FILTER0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_FILTER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_FILTER0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_FILTER0_0 {
    pub rawContext: u64,
    pub providerContextKey: ::windows::runtime::GUID,
}
impl FWPM_FILTER0_0 {}
impl ::std::default::Default for FWPM_FILTER0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_FILTER0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_FILTER0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_FILTER0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_FILTER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub filterKey: ::windows::runtime::GUID,
    pub filterId: u64,
}
impl FWPM_FILTER_CHANGE0 {}
impl ::std::default::Default for FWPM_FILTER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_FILTER_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_FILTER_CHANGE0")
            .field("changeType", &self.changeType)
            .field("filterKey", &self.filterKey)
            .field("filterId", &self.filterId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_FILTER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType
            && self.filterKey == other.filterKey
            && self.filterId == other.filterId
    }
}
impl ::std::cmp::Eq for FWPM_FILTER_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_FILTER_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_FILTER_CHANGE_CALLBACK0 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, change: *const FWPM_FILTER_CHANGE0);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_CONDITION0 {
    pub fieldKey: ::windows::runtime::GUID,
    pub matchType: FWP_MATCH_TYPE,
    pub conditionValue: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_FILTER_CONDITION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_FILTER_CONDITION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_FILTER_CONDITION0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_FILTER_CONDITION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_FILTER_CONDITION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::runtime::GUID,
    pub layerKey: ::windows::runtime::GUID,
    pub enumType: FWP_FILTER_ENUM_TYPE,
    pub flags: u32,
    pub providerContextTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
    pub actionMask: u32,
    pub calloutKey: *mut ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_FILTER_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_FILTER_ENUM_TEMPLATE0")
            .field("providerKey", &self.providerKey)
            .field("layerKey", &self.layerKey)
            .field("enumType", &self.enumType)
            .field("flags", &self.flags)
            .field("providerContextTemplate", &self.providerContextTemplate)
            .field("numFilterConditions", &self.numFilterConditions)
            .field("filterCondition", &self.filterCondition)
            .field("actionMask", &self.actionMask)
            .field("calloutKey", &self.calloutKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey
            && self.layerKey == other.layerKey
            && self.enumType == other.enumType
            && self.flags == other.flags
            && self.providerContextTemplate == other.providerContextTemplate
            && self.numFilterConditions == other.numFilterConditions
            && self.filterCondition == other.filterCondition
            && self.actionMask == other.actionMask
            && self.calloutKey == other.calloutKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_FILTER_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_FILTER_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_FILTER_FLAGS(pub u32);
pub const FWPM_FILTER_FLAG_NONE: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(0u32);
pub const FWPM_FILTER_FLAG_PERSISTENT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(1u32);
pub const FWPM_FILTER_FLAG_BOOTTIME: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(2u32);
pub const FWPM_FILTER_FLAG_HAS_PROVIDER_CONTEXT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(4u32);
pub const FWPM_FILTER_FLAG_CLEAR_ACTION_RIGHT: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(8u32);
pub const FWPM_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: FWPM_FILTER_FLAGS =
    FWPM_FILTER_FLAGS(16u32);
pub const FWPM_FILTER_FLAG_DISABLED: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(32u32);
pub const FWPM_FILTER_FLAG_INDEXED: FWPM_FILTER_FLAGS = FWPM_FILTER_FLAGS(64u32);
impl ::std::convert::From<u32> for FWPM_FILTER_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_FILTER_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FWPM_FILTER_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FWPM_FILTER_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FWPM_FILTER_FLAG_GAMEOS_ONLY: u32 = 512u32;
pub const FWPM_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: u32 = 128u32;
pub const FWPM_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: u32 = 2048u32;
pub const FWPM_FILTER_FLAG_RESERVED0: u32 = 4096u32;
pub const FWPM_FILTER_FLAG_RESERVED1: u32 = 8192u32;
pub const FWPM_FILTER_FLAG_SILENT_MODE: u32 = 1024u32;
pub const FWPM_FILTER_FLAG_SYSTEMOS_ONLY: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_FILTER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_FILTER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_FILTER_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_FILTER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_FILTER_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_FILTER_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_FILTER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_FILTER_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_FILTER_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_KEYING_MODULE_AUTHIP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        300145376,
        56614,
        17808,
        [133, 125, 171, 75, 40, 209, 160, 149],
    );
pub const FWPM_KEYING_MODULE_IKE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2847668103,
    33448,
    17851,
    [164, 0, 93, 126, 89, 82, 199, 169],
);
pub const FWPM_KEYING_MODULE_IKEV2: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        68653772,
        36615,
        16797,
        [163, 148, 113, 105, 104, 203, 22, 71],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_LAYER0 {
    pub layerKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub numFields: u32,
    pub field: *mut FWPM_FIELD0,
    pub defaultSubLayerKey: ::windows::runtime::GUID,
    pub layerId: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_LAYER0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_LAYER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_LAYER0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_LAYER0")
            .field("layerKey", &self.layerKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("numFields", &self.numFields)
            .field("field", &self.field)
            .field("defaultSubLayerKey", &self.defaultSubLayerKey)
            .field("layerId", &self.layerId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_LAYER0 {
    fn eq(&self, other: &Self) -> bool {
        self.layerKey == other.layerKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.numFields == other.numFields
            && self.field == other.field
            && self.defaultSubLayerKey == other.defaultSubLayerKey
            && self.layerId == other.layerId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_LAYER0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_LAYER0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3280820177,
        1447,
        19507,
        [144, 79, 127, 188, 238, 230, 14, 130],
    );
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3593644033,
        62906,
        19158,
        [150, 227, 96, 112, 23, 217, 131, 106],
    );
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1248999739,
        12703,
        17596,
        [132, 195, 186, 84, 220, 179, 182, 180],
    );
pub const FWPM_LAYER_ALE_AUTH_CONNECT_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3380331448,
        51619,
        20019,
        [134, 149, 142, 23, 170, 212, 222, 9],
    );
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2293980589,
        30423,
        16935,
        [156, 113, 223, 10, 62, 215, 190, 126],
    );
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        924711642,
        40742,
        17917,
        [180, 235, 194, 158, 178, 18, 137, 63],
    );
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2060049956,
        6109,
        18452,
        [180, 189, 169, 251, 201, 90, 50, 27],
    );
pub const FWPM_LAYER_ALE_AUTH_LISTEN_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1617967879,
        25544,
        18665,
        [173, 163, 18, 177, 175, 64, 166, 23],
    );
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3788349415,
        62645,
        17011,
        [150, 192, 89, 46, 72, 123, 134, 80],
    );
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2666178971,
        48418,
        16935,
        [145, 159, 0, 115, 198, 51, 87, 177],
    );
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2746494103,
        40708,
        18034,
        [184, 126, 206, 233, 196, 131, 37, 127],
    );
pub const FWPM_LAYER_ALE_AUTH_RECV_ACCEPT_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2303024023,
        56289,
        17727,
        [162, 36, 19, 218, 137, 90, 243, 150],
    );
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1721207981,
        50948,
        17068,
        [134, 172, 124, 26, 35, 27, 210, 83],
    );
pub const FWPM_LAYER_ALE_BIND_REDIRECT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3203411100,
        24683,
        17718,
        [140, 38, 28, 47, 199, 182, 49, 212],
    );
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3336977548,
        46980,
        17762,
        [170, 125, 10, 103, 207, 202, 249, 163],
    );
pub const FWPM_LAYER_ALE_CONNECT_REDIRECT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1484674215,
        32838,
        17082,
        [160, 170, 183, 22, 37, 15, 199, 253],
    );
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3027657767,
        58018,
        18042,
        [189, 126, 219, 205, 27, 216, 90, 9],
    );
pub const FWPM_LAYER_ALE_ENDPOINT_CLOSURE_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3142806733,
        18261,
        19369,
        [159, 247, 249, 237, 248, 105, 156, 123],
    );
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2944419594,
        21910,
        19475,
        [153, 146, 83, 158, 111, 229, 121, 103],
    );
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        342549673,
        41426,
        19779,
        [163, 26, 76, 66, 104, 43, 142, 79],
    );
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1881264819,
        57252,
        16494,
        [175, 235, 106, 250, 247, 231, 14, 253],
    );
pub const FWPM_LAYER_ALE_FLOW_ESTABLISHED_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1184007734,
        48074,
        19318,
        [148, 29, 15, 167, 245, 215, 211, 114],
    );
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        306697837,
        2912,
        18965,
        [141, 68, 113, 85, 208, 245, 58, 12],
    );
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        190321314,
        50175,
        20170,
        [184, 141, 199, 158, 32, 172, 99, 34],
    );
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1436963041,
        24330,
        20170,
        [166, 83, 136, 245, 59, 38, 170, 140],
    );
pub const FWPM_LAYER_ALE_RESOURCE_ASSIGNMENT_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3418986683,
        50463,
        19482,
        [187, 79, 151, 117, 252, 172, 171, 47],
    );
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1949719758,
        52400,
        16410,
        [191, 193, 184, 153, 52, 173, 126, 21],
    );
pub const FWPM_LAYER_ALE_RESOURCE_RELEASE_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4108701312,
        60876,
        19987,
        [138, 47, 185, 20, 84, 187, 5, 123],
    );
pub const FWPM_LAYER_DATAGRAM_DATA_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1023983438,
        17910,
        18736,
        [169, 34, 65, 112, 152, 226, 0, 39],
    );
pub const FWPM_LAYER_DATAGRAM_DATA_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        417542342,
        29256,
        20050,
        [170, 171, 71, 46, 214, 119, 4, 253],
    );
pub const FWPM_LAYER_DATAGRAM_DATA_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4198891055,
        15546,
        17447,
        [135, 252, 87, 185, 164, 177, 13, 0],
    );
pub const FWPM_LAYER_DATAGRAM_DATA_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        164749281,
        39814,
        19010,
        [190, 157, 140, 49, 91, 146, 165, 208],
    );
pub const FWPM_LAYER_EGRESS_VSWITCH_ETHERNET: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2261283504,
        30458,
        19321,
        [147, 164, 7, 80, 83, 10, 226, 146],
    );
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3106099382,
        37360,
        18102,
        [189, 196, 135, 29, 253, 74, 124, 152],
    );
pub const FWPM_LAYER_EGRESS_VSWITCH_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        455995171,
        6273,
        16573,
        [130, 244, 66, 84, 230, 49, 65, 203],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_LAYER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl FWPM_LAYER_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_LAYER_ENUM_TEMPLATE0")
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for FWPM_LAYER_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_LAYER_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_LAYER_FLAG_BUFFERED: u32 = 8u32;
pub const FWPM_LAYER_FLAG_BUILTIN: u32 = 2u32;
pub const FWPM_LAYER_FLAG_CLASSIFY_MOSTLY: u32 = 4u32;
pub const FWPM_LAYER_FLAG_KERNEL: u32 = 1u32;
pub const FWPM_LAYER_IKEEXT_V4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2974514139,
    56253,
    18238,
    [190, 212, 139, 71, 8, 212, 242, 112],
);
pub const FWPM_LAYER_IKEEXT_V6: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3058140851,
    63111,
    20153,
    [137, 210, 142, 243, 42, 205, 171, 226],
);
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1632213392,
        15542,
        20100,
        [185, 80, 83, 185, 75, 105, 100, 243],
    );
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2796646517,
        60335,
        16467,
        [164, 231, 33, 60, 129, 33, 237, 229],
    );
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1710865919,
        15149,
        20061,
        [184, 198, 199, 32, 101, 31, 232, 152],
    );
pub const FWPM_LAYER_INBOUND_ICMP_ERROR_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2800209088,
        2299,
        18061,
        [164, 114, 151, 113, 213, 89, 94, 9],
    );
pub const FWPM_LAYER_INBOUND_IPPACKET_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3362771391,
        8653,
        18814,
        [160, 187, 23, 66, 92, 136, 92, 88],
    );
pub const FWPM_LAYER_INBOUND_IPPACKET_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3047305424,
        43200,
        17650,
        [145, 110, 153, 27, 83, 222, 209, 247],
    );
pub const FWPM_LAYER_INBOUND_IPPACKET_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4112528075,
        39196,
        18151,
        [151, 29, 38, 1, 69, 154, 145, 202],
    );
pub const FWPM_LAYER_INBOUND_IPPACKET_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3139748473,
        37812,
        18338,
        [131, 173, 174, 22, 152, 181, 8, 133],
    );
pub const FWPM_LAYER_INBOUND_MAC_FRAME_ETHERNET: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4026236635,
        85,
        20378,
        [162, 49, 79, 248, 19, 26, 209, 145],
    );
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3559001043,
        25294,
        20232,
        [174, 136, 181, 110, 133, 38, 223, 80],
    );
pub const FWPM_LAYER_INBOUND_MAC_FRAME_NATIVE_FAST: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2235214478,
        11128,
        19748,
        [168, 4, 54, 219, 8, 178, 151, 17],
    );
pub const FWPM_LAYER_INBOUND_RESERVED2: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4110126421,
        49270,
        18136,
        [162, 199, 106, 76, 114, 44, 164, 237],
    );
pub const FWPM_LAYER_INBOUND_TRANSPORT_FAST: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3827115801,
        1479,
        16624,
        [137, 131, 234, 141, 23, 187, 194, 246],
    );
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1495719880,
        58319,
        17446,
        [162, 131, 220, 57, 63, 93, 15, 157],
    );
pub const FWPM_LAYER_INBOUND_TRANSPORT_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2890569779,
        63133,
        17992,
        [178, 97, 109, 200, 72, 53, 239, 57],
    );
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1665828511,
        64547,
        19344,
        [176, 193, 191, 98, 10, 54, 174, 111],
    );
pub const FWPM_LAYER_INBOUND_TRANSPORT_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        711981397,
        15147,
        18898,
        [152, 72, 173, 157, 114, 220, 170, 183],
    );
pub const FWPM_LAYER_INGRESS_VSWITCH_ETHERNET: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2107135866,
        39559,
        16876,
        [151, 24, 124, 245, 137, 201, 243, 45],
    );
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2993254390,
        30543,
        17748,
        [159, 125, 61, 163, 148, 95, 142, 133],
    );
pub const FWPM_LAYER_INGRESS_VSWITCH_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1591940348,
        32138,
        18420,
        [183, 227, 41, 26, 54, 218, 78, 18],
    );
pub const FWPM_LAYER_IPFORWARD_V4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2821377060,
    20193,
    20193,
    [180, 101, 253, 29, 37, 203, 16, 164],
);
pub const FWPM_LAYER_IPFORWARD_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2661197683,
        12206,
        16912,
        [143, 23, 52, 18, 158, 243, 105, 235],
    );
pub const FWPM_LAYER_IPFORWARD_V6: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2073446424,
    6599,
    18746,
    [183, 31, 131, 44, 54, 132, 210, 140],
);
pub const FWPM_LAYER_IPFORWARD_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        827476573,
        7678,
        18223,
        [187, 147, 81, 142, 233, 69, 216, 162],
    );
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4029355302,
        42073,
        19025,
        [185, 227, 117, 157, 229, 43, 157, 44],
    );
pub const FWPM_LAYER_IPSEC_KM_DEMUX_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        796220662,
        12244,
        20104,
        [179, 228, 169, 27, 202, 73, 82, 53],
    );
pub const FWPM_LAYER_IPSEC_V4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3987102836,
    24845,
    19397,
    [148, 143, 60, 79, 137, 85, 104, 103],
);
pub const FWPM_LAYER_IPSEC_V6: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    331646018,
    36231,
    16993,
    [154, 41, 89, 210, 171, 195, 72, 180],
);
pub const FWPM_LAYER_KM_AUTHORIZATION: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1252140777,
        36896,
        17915,
        [149, 106, 192, 36, 157, 132, 17, 149],
    );
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        204121729,
        36955,
        19661,
        [164, 103, 77, 216, 17, 208, 123, 123],
    );
pub const FWPM_LAYER_NAME_RESOLUTION_CACHE_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2463470330,
        27393,
        17226,
        [157, 234, 209, 233, 110, 169, 125, 169],
    );
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1094254848,
        22092,
        19250,
        [188, 29, 113, 128, 72, 53, 77, 124],
    );
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3008990518,
        1377,
        17800,
        [166, 191, 233, 85, 227, 246, 38, 75],
    );
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2142255968,
        31629,
        19962,
        [186, 221, 152, 1, 118, 252, 78, 18],
    );
pub const FWPM_LAYER_OUTBOUND_ICMP_ERROR_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1710417479,
        36108,
        20295,
        [177, 155, 51, 164, 211, 241, 53, 124],
    );
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        509386670,
        35460,
        16693,
        [163, 49, 149, 11, 84, 34, 158, 205],
    );
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        149208245,
        46663,
        18675,
        [149, 60, 229, 221, 189, 3, 147, 126],
    );
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2746461035,
        13668,
        18572,
        [145, 23, 243, 78, 130, 20, 39, 99],
    );
pub const FWPM_LAYER_OUTBOUND_IPPACKET_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2501105604,
        43316,
        18908,
        [145, 167, 108, 203, 128, 204, 2, 227],
    );
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_ETHERNET: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1766224828,
        55003,
        18544,
        [173, 238, 10, 205, 189, 183, 244, 178],
    );
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2495891730,
        40303,
        20159,
        [185, 149, 5, 171, 138, 8, 141, 27],
    );
pub const FWPM_LAYER_OUTBOUND_MAC_FRAME_NATIVE_FAST: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1192098118,
        51554,
        18543,
        [148, 70, 130, 147, 203, 199, 94, 184],
    );
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_FAST: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        334316424,
        41072,
        18453,
        [153, 53, 122, 155, 230, 64, 139, 120],
    );
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        166075114,
        53780,
        18146,
        [155, 33, 178, 107, 11, 47, 40, 200],
    );
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3320907089,
        48560,
        17367,
        [163, 19, 80, 226, 17, 244, 214, 138],
    );
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3782433758,
        319,
        18005,
        [179, 81, 164, 158, 21, 118, 45, 240],
    );
pub const FWPM_LAYER_OUTBOUND_TRANSPORT_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4097040233,
        52413,
        18478,
        [185, 178, 87, 22, 86, 88, 195, 179],
    );
pub const FWPM_LAYER_RPC_EPMAP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    2454174817,
    60167,
    18414,
    [135, 44, 191, 215, 139, 253, 22, 22],
);
pub const FWPM_LAYER_RPC_EP_ADD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1636696007,
    50256,
    18755,
    [149, 219, 153, 180, 193, 106, 85, 212],
);
pub const FWPM_LAYER_RPC_PROXY_CONN: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2493822219,
        47708,
        20263,
        [144, 122, 34, 159, 172, 12, 42, 122],
    );
pub const FWPM_LAYER_RPC_PROXY_IF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4171466261,
    57644,
    16812,
    [152, 223, 18, 26, 217, 129, 170, 222],
);
pub const FWPM_LAYER_RPC_UM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1973984730,
    38372,
    16627,
    [173, 199, 118, 136, 169, 200, 71, 225],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_LAYER_STATISTICS0 {
    pub layerId: ::windows::runtime::GUID,
    pub classifyPermitCount: u32,
    pub classifyBlockCount: u32,
    pub classifyVetoCount: u32,
    pub numCacheEntries: u32,
}
impl FWPM_LAYER_STATISTICS0 {}
impl ::std::default::Default for FWPM_LAYER_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_LAYER_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_LAYER_STATISTICS0")
            .field("layerId", &self.layerId)
            .field("classifyPermitCount", &self.classifyPermitCount)
            .field("classifyBlockCount", &self.classifyBlockCount)
            .field("classifyVetoCount", &self.classifyVetoCount)
            .field("numCacheEntries", &self.numCacheEntries)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_LAYER_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.layerId == other.layerId
            && self.classifyPermitCount == other.classifyPermitCount
            && self.classifyBlockCount == other.classifyBlockCount
            && self.classifyVetoCount == other.classifyVetoCount
            && self.numCacheEntries == other.numCacheEntries
    }
}
impl ::std::cmp::Eq for FWPM_LAYER_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for FWPM_LAYER_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_LAYER_STREAM_PACKET_V4: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2941442284,
        52013,
        17637,
        [173, 146, 248, 220, 56, 210, 235, 41],
    );
pub const FWPM_LAYER_STREAM_PACKET_V6: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2006617251,
        61593,
        18063,
        [181, 212, 131, 83, 92, 70, 28, 2],
    );
pub const FWPM_LAYER_STREAM_V4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    998860092,
    49520,
    18916,
    [177, 205, 224, 238, 238, 225, 154, 62],
);
pub const FWPM_LAYER_STREAM_V4_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        633651906,
        9727,
        17234,
        [130, 249, 197, 74, 74, 71, 38, 220],
    );
pub const FWPM_LAYER_STREAM_V6: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1204360058,
    32452,
    18099,
    [182, 228, 72, 233, 38, 177, 237, 164],
);
pub const FWPM_LAYER_STREAM_V6_DISCARD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        279289799,
        46632,
        19521,
        [158, 184, 207, 55, 213, 81, 3, 207],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT0 {
    pub header: FWPM_NET_EVENT_HEADER0,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT1 {
    pub header: FWPM_NET_EVENT_HEADER1,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT1_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT2 {
    pub header: FWPM_NET_EVENT_HEADER2,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT2_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT3 {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT3_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT3_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT4_ {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT4__0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT4_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT4_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT4_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT4_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT4_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT4__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT4__0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT4__0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT4__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT4__0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT5_ {
    pub header: FWPM_NET_EVENT_HEADER3,
    pub r#type: FWPM_NET_EVENT_TYPE,
    pub Anonymous: FWPM_NET_EVENT5__0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT5_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT5_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT5_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT5_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT5_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT5__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT5__0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT5__0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT5__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT5__0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK0 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, event: *const FWPM_NET_EVENT1);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK1 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, event: *const FWPM_NET_EVENT2);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK2 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, event: *const FWPM_NET_EVENT3);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK3 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, event: *const FWPM_NET_EVENT4_);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type FWPM_NET_EVENT_CALLBACK4 =
    unsafe extern "system" fn(context: *mut ::std::ffi::c_void, event: *const FWPM_NET_EVENT5_);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_NET_EVENT_CAPABILITY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CAPABILITY_ALLOW0")
            .field("networkCapabilityId", &self.networkCapabilityId)
            .field("filterId", &self.filterId)
            .field("isLoopback", &self.isLoopback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        self.networkCapabilityId == other.networkCapabilityId
            && self.filterId == other.filterId
            && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_NET_EVENT_CAPABILITY_DROP0 {
    pub networkCapabilityId: FWPM_APPC_NETWORK_CAPABILITY_TYPE,
    pub filterId: u64,
    pub isLoopback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_NET_EVENT_CAPABILITY_DROP0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CAPABILITY_DROP0")
            .field("networkCapabilityId", &self.networkCapabilityId)
            .field("filterId", &self.filterId)
            .field("isLoopback", &self.isLoopback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.networkCapabilityId == other.networkCapabilityId
            && self.filterId == other.filterId
            && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_DROP0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_CLASSIFY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CLASSIFY_ALLOW0")
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId
            && self.layerId == other.layerId
            && self.reauthReason == other.reauthReason
            && self.originalProfile == other.originalProfile
            && self.currentProfile == other.currentProfile
            && self.msFwpDirection == other.msFwpDirection
            && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_NET_EVENT_CLASSIFY_DROP0 {
    pub filterId: u64,
    pub layerId: u16,
}
impl FWPM_NET_EVENT_CLASSIFY_DROP0 {}
impl ::std::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP0")
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId && self.layerId == other.layerId
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_CLASSIFY_DROP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP1")
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId
            && self.layerId == other.layerId
            && self.reauthReason == other.reauthReason
            && self.originalProfile == other.originalProfile
            && self.currentProfile == other.currentProfile
            && self.msFwpDirection == other.msFwpDirection
            && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_CLASSIFY_DROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP2")
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
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId
            && self.layerId == other.layerId
            && self.reauthReason == other.reauthReason
            && self.originalProfile == other.originalProfile
            && self.currentProfile == other.currentProfile
            && self.msFwpDirection == other.msFwpDirection
            && self.isLoopback == other.isLoopback
            && self.vSwitchId == other.vSwitchId
            && self.vSwitchSourcePort == other.vSwitchSourcePort
            && self.vSwitchDestinationPort == other.vSwitchDestinationPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP_MAC0")
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
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn eq(&self, other: &Self) -> bool {
        self.localMacAddr == other.localMacAddr
            && self.remoteMacAddr == other.remoteMacAddr
            && self.mediaType == other.mediaType
            && self.ifType == other.ifType
            && self.etherType == other.etherType
            && self.ndisPortNumber == other.ndisPortNumber
            && self.reserved == other.reserved
            && self.vlanTag == other.vlanTag
            && self.ifLuid == other.ifLuid
            && self.filterId == other.filterId
            && self.layerId == other.layerId
            && self.reauthReason == other.reauthReason
            && self.originalProfile == other.originalProfile
            && self.currentProfile == other.currentProfile
            && self.msFwpDirection == other.msFwpDirection
            && self.isLoopback == other.isLoopback
            && self.vSwitchId == other.vSwitchId
            && self.vSwitchSourcePort == other.vSwitchSourcePort
            && self.vSwitchDestinationPort == other.vSwitchDestinationPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
    pub numFilterConditions: u32,
    pub filterCondition: *mut FWPM_FILTER_CONDITION0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_ENUM_TEMPLATE0")
            .field("startTime", &self.startTime)
            .field("endTime", &self.endTime)
            .field("numFilterConditions", &self.numFilterConditions)
            .field("filterCondition", &self.filterCondition)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime
            && self.endTime == other.endTime
            && self.numFilterConditions == other.numFilterConditions
            && self.filterCondition == other.filterCondition
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_NET_EVENT_FLAG_APP_ID_SET: u32 = 32u32;
pub const FWPM_NET_EVENT_FLAG_EFFECTIVE_NAME_SET: u32 = 8192u32;
pub const FWPM_NET_EVENT_FLAG_ENTERPRISE_ID_SET: u32 = 2048u32;
pub const FWPM_NET_EVENT_FLAG_IP_PROTOCOL_SET: u32 = 1u32;
pub const FWPM_NET_EVENT_FLAG_IP_VERSION_SET: u32 = 256u32;
pub const FWPM_NET_EVENT_FLAG_LOCAL_ADDR_SET: u32 = 2u32;
pub const FWPM_NET_EVENT_FLAG_LOCAL_PORT_SET: u32 = 8u32;
pub const FWPM_NET_EVENT_FLAG_PACKAGE_ID_SET: u32 = 1024u32;
pub const FWPM_NET_EVENT_FLAG_POLICY_FLAGS_SET: u32 = 4096u32;
pub const FWPM_NET_EVENT_FLAG_REAUTH_REASON_SET: u32 = 512u32;
pub const FWPM_NET_EVENT_FLAG_REMOTE_ADDR_SET: u32 = 4u32;
pub const FWPM_NET_EVENT_FLAG_REMOTE_PORT_SET: u32 = 16u32;
pub const FWPM_NET_EVENT_FLAG_SCOPE_ID_SET: u32 = 128u32;
pub const FWPM_NET_EVENT_FLAG_USER_ID_SET: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_HEADER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_HEADER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER0_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER0_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER0_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER0_1 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER0_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_HEADER1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER1_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER1_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER1_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER1_1 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER1_2 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0,
}
impl FWPM_NET_EVENT_HEADER1_2 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_2 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_NET_EVENT_HEADER1_2_0 {
    pub reserved1: FWP_AF,
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0,
}
impl FWPM_NET_EVENT_HEADER1_2_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER1_2_0_0 {
    pub Anonymous: FWPM_NET_EVENT_HEADER1_2_0_0_0,
}
impl FWPM_NET_EVENT_HEADER1_2_0_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_2_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_HEADER1_2_0_0_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("reserved2", &self.reserved2)
            .field("reserved3", &self.reserved3)
            .field("reserved4", &self.reserved4)
            .field("reserved5", &self.reserved5)
            .field("reserved6", &self.reserved6)
            .field("reserved7", &self.reserved7)
            .field("reserved8", &self.reserved8)
            .field("reserved9", &self.reserved9)
            .field("reserved10", &self.reserved10)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved2 == other.reserved2
            && self.reserved3 == other.reserved3
            && self.reserved4 == other.reserved4
            && self.reserved5 == other.reserved5
            && self.reserved6 == other.reserved6
            && self.reserved7 == other.reserved7
            && self.reserved8 == other.reserved8
            && self.reserved9 == other.reserved9
            && self.reserved10 == other.reserved10
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER1_2_0_0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER1_2_0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_HEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_HEADER2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER2_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER2_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER2_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER2_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER2_1 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER2_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER2_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub enterpriseId: super::super::Foundation::PWSTR,
    pub policyFlags: u64,
    pub effectiveName: FWP_BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_HEADER3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_HEADER3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER3_0 {
    pub localAddrV4: u32,
    pub localAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER3_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER3_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER3_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_HEADER3_1 {
    pub remoteAddrV4: u32,
    pub remoteAddrV6: FWP_BYTE_ARRAY16,
}
impl FWPM_NET_EVENT_HEADER3_1 {}
impl ::std::default::Default for FWPM_NET_EVENT_HEADER3_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_HEADER3_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_HEADER3_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_HEADER3_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {}
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE0")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("emState", &self.emState)
            .field("saRole", &self.saRole)
            .field("emAuthMethod", &self.emAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("qmFilterId", &self.qmFilterId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.emState == other.emState
            && self.saRole == other.saRole
            && self.emAuthMethod == other.emAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.qmFilterId == other.qmFilterId
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub localPrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub remotePrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut super::super::Foundation::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut super::super::Foundation::PWSTR,
    pub saTrafficType: IPSEC_TRAFFIC_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE1")
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
            .field(
                "remotePrincipalNameForAuth",
                &self.remotePrincipalNameForAuth,
            )
            .field(
                "numLocalPrincipalGroupSids",
                &self.numLocalPrincipalGroupSids,
            )
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field(
                "numRemotePrincipalGroupSids",
                &self.numRemotePrincipalGroupSids,
            )
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("saTrafficType", &self.saTrafficType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.emState == other.emState
            && self.saRole == other.saRole
            && self.emAuthMethod == other.emAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.qmFilterId == other.qmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
            && self.saTrafficType == other.saTrafficType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_BENIGN: u32 = 2u32;
pub const FWPM_NET_EVENT_IKEEXT_EM_FAILURE_FLAG_MULTIPLE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {}
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE0")
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
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.keyingModuleType == other.keyingModuleType
            && self.mmState == other.mmState
            && self.saRole == other.saRole
            && self.mmAuthMethod == other.mmAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.mmFilterId == other.mmFilterId
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub localPrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub remotePrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut super::super::Foundation::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE1")
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
            .field(
                "remotePrincipalNameForAuth",
                &self.remotePrincipalNameForAuth,
            )
            .field(
                "numLocalPrincipalGroupSids",
                &self.numLocalPrincipalGroupSids,
            )
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field(
                "numRemotePrincipalGroupSids",
                &self.numRemotePrincipalGroupSids,
            )
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.keyingModuleType == other.keyingModuleType
            && self.mmState == other.mmState
            && self.saRole == other.saRole
            && self.mmAuthMethod == other.mmAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.mmFilterId == other.mmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub localPrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub remotePrincipalNameForAuth: super::super::Foundation::PWSTR,
    pub numLocalPrincipalGroupSids: u32,
    pub localPrincipalGroupSids: *mut super::super::Foundation::PWSTR,
    pub numRemotePrincipalGroupSids: u32,
    pub remotePrincipalGroupSids: *mut super::super::Foundation::PWSTR,
    pub providerContextKey: *mut ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_")
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
            .field(
                "remotePrincipalNameForAuth",
                &self.remotePrincipalNameForAuth,
            )
            .field(
                "numLocalPrincipalGroupSids",
                &self.numLocalPrincipalGroupSids,
            )
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field(
                "numRemotePrincipalGroupSids",
                &self.numRemotePrincipalGroupSids,
            )
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("providerContextKey", &self.providerContextKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.keyingModuleType == other.keyingModuleType
            && self.mmState == other.mmState
            && self.saRole == other.saRole
            && self.mmAuthMethod == other.mmAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.mmFilterId == other.mmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
            && self.providerContextKey == other.providerContextKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2_ {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_BENIGN: u32 = 1u32;
pub const FWPM_NET_EVENT_IKEEXT_MM_FAILURE_FLAG_MULTIPLE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub mmProviderContextKey: ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1__1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0,
    pub Anonymous2: FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1,
    pub failureStatus: i32,
    pub direction: FWP_DIRECTION,
}
impl FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {}
impl ::std::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    pub publicHostV4Addr: u32,
    pub publicHostV6Addr: [u8; 16],
}
impl FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {}
impl ::std::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    pub internalHostV4Addr: u32,
    pub internalHostV6Addr: [u8; 16],
}
impl FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {}
impl ::std::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IPSEC_DOSP_DROP0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    pub failureStatus: i32,
    pub direction: FWP_DIRECTION,
    pub spi: u32,
    pub filterId: u64,
    pub layerId: u16,
}
impl FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {}
impl ::std::default::Default for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_IPSEC_KERNEL_DROP0")
            .field("failureStatus", &self.failureStatus)
            .field("direction", &self.direction)
            .field("spi", &self.spi)
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureStatus == other.failureStatus
            && self.direction == other.direction
            && self.spi == other.spi
            && self.filterId == other.filterId
            && self.layerId == other.layerId
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_ALLOW: u32 = 8u32;
pub const FWPM_NET_EVENT_KEYWORD_CAPABILITY_DROP: u32 = 4u32;
pub const FWPM_NET_EVENT_KEYWORD_CLASSIFY_ALLOW: u32 = 16u32;
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_BCAST: u32 = 2u32;
pub const FWPM_NET_EVENT_KEYWORD_INBOUND_MCAST: u32 = 1u32;
pub const FWPM_NET_EVENT_KEYWORD_PORT_SCANNING_DROP: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    pub spi: u32,
}
impl FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {}
impl ::std::default::Default for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_")
            .field("spi", &self.spi)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    fn eq(&self, other: &Self) -> bool {
        self.spi == other.spi
    }
}
impl ::std::cmp::Eq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_NET_EVENT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_NET_EVENT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_NET_EVENT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_NET_EVENT_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_NET_EVENT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_NET_EVENT_TYPE(pub i32);
pub const FWPM_NET_EVENT_TYPE_IKEEXT_MM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(0i32);
pub const FWPM_NET_EVENT_TYPE_IKEEXT_QM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(1i32);
pub const FWPM_NET_EVENT_TYPE_IKEEXT_EM_FAILURE: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(2i32);
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(3i32);
pub const FWPM_NET_EVENT_TYPE_IPSEC_KERNEL_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(4i32);
pub const FWPM_NET_EVENT_TYPE_IPSEC_DOSP_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(5i32);
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_ALLOW: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(6i32);
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_DROP: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(7i32);
pub const FWPM_NET_EVENT_TYPE_CAPABILITY_ALLOW: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(8i32);
pub const FWPM_NET_EVENT_TYPE_CLASSIFY_DROP_MAC: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(9i32);
pub const FWPM_NET_EVENT_TYPE_LPM_PACKET_ARRIVAL: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(10i32);
pub const FWPM_NET_EVENT_TYPE_MAX: FWPM_NET_EVENT_TYPE = FWPM_NET_EVENT_TYPE(11i32);
impl ::std::convert::From<i32> for FWPM_NET_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_NET_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_PROVIDER0 {
    pub providerKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerData: FWP_BYTE_BLOB,
    pub serviceName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_PROVIDER0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_PROVIDER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_PROVIDER0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER0")
            .field("providerKey", &self.providerKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("providerData", &self.providerData)
            .field("serviceName", &self.serviceName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_PROVIDER0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.providerData == other.providerData
            && self.serviceName == other.serviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_PROVIDER0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerKey: ::windows::runtime::GUID,
}
impl FWPM_PROVIDER_CHANGE0 {}
impl ::std::default::Default for FWPM_PROVIDER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_CHANGE0")
            .field("changeType", &self.changeType)
            .field("providerKey", &self.providerKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.providerKey == other.providerKey
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_PROVIDER_CHANGE_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    change: *const FWPM_PROVIDER_CHANGE0,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT0 {
    pub providerContextKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT0_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_PROVIDER_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_PROVIDER_CONTEXT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT1 {
    pub providerContextKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT1_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_PROVIDER_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_PROVIDER_CONTEXT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT2 {
    pub providerContextKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT2_0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_PROVIDER_CONTEXT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_PROVIDER_CONTEXT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_PROVIDER_CONTEXT3_ {
    pub providerContextKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub r#type: FWPM_PROVIDER_CONTEXT_TYPE,
    pub Anonymous: FWPM_PROVIDER_CONTEXT3__0,
    pub providerContextId: u64,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_PROVIDER_CONTEXT3_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT3_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT3_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT3_ {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT3_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_PROVIDER_CONTEXT3__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT3__0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT3__0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT3__0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT3__0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_CONTEXT_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub providerContextKey: ::windows::runtime::GUID,
    pub providerContextId: u64,
}
impl FWPM_PROVIDER_CONTEXT_CHANGE0 {}
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_CONTEXT_CHANGE0")
            .field("changeType", &self.changeType)
            .field("providerContextKey", &self.providerContextKey)
            .field("providerContextId", &self.providerContextId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType
            && self.providerContextKey == other.providerContextKey
            && self.providerContextId == other.providerContextId
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    change: *const FWPM_PROVIDER_CONTEXT_CHANGE0,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerContextType: FWPM_PROVIDER_CONTEXT_TYPE,
}
impl FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0")
            .field("providerKey", &self.providerKey)
            .field("providerContextType", &self.providerContextType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey
            && self.providerContextType == other.providerContextType
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_PROVIDER_CONTEXT_FLAG_DOWNLEVEL: u32 = 2u32;
pub const FWPM_PROVIDER_CONTEXT_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_AUTHIP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2992547840,
        3330,
        18157,
        [146, 189, 127, 168, 75, 183, 62, 157],
    );
pub const FWPM_PROVIDER_CONTEXT_SECURE_SOCKET_IPSEC: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2351776068,
        63712,
        17088,
        [148, 206, 124, 207, 198, 59, 47, 155],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    pub flags: FWPM_SUBSCRIPTION_FLAGS,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_PROVIDER_CONTEXT_TYPE(pub i32);
pub const FWPM_IPSEC_KEYING_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(0i32);
pub const FWPM_IPSEC_IKE_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(1i32);
pub const FWPM_IPSEC_IKE_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(2i32);
pub const FWPM_IPSEC_AUTHIP_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(3i32);
pub const FWPM_IPSEC_AUTHIP_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(4i32);
pub const FWPM_IPSEC_IKE_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(5i32);
pub const FWPM_IPSEC_AUTHIP_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(6i32);
pub const FWPM_CLASSIFY_OPTIONS_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(7i32);
pub const FWPM_GENERAL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(8i32);
pub const FWPM_IPSEC_IKEV2_QM_TUNNEL_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(9i32);
pub const FWPM_IPSEC_IKEV2_MM_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(10i32);
pub const FWPM_IPSEC_DOSP_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE = FWPM_PROVIDER_CONTEXT_TYPE(11i32);
pub const FWPM_IPSEC_IKEV2_QM_TRANSPORT_CONTEXT: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(12i32);
pub const FWPM_PROVIDER_CONTEXT_TYPE_MAX: FWPM_PROVIDER_CONTEXT_TYPE =
    FWPM_PROVIDER_CONTEXT_TYPE(13i32);
impl ::std::convert::From<i32> for FWPM_PROVIDER_CONTEXT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_CONTEXT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl FWPM_PROVIDER_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_ENUM_TEMPLATE0")
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_PROVIDER_FLAG_DISABLED: u32 = 16u32;
pub const FWPM_PROVIDER_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_PROVIDER_IKEEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    279810582,
    52446,
    17772,
    [139, 22, 233, 240, 78, 96, 169, 11],
);
pub const FWPM_PROVIDER_IPSEC_DOSP_CONFIG: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1013712297,
        49244,
        19385,
        [131, 56, 35, 39, 129, 76, 232, 191],
    );
pub const FWPM_PROVIDER_MPSSVC_EDP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2835519223,
        18104,
        17495,
        [143, 132, 176, 94, 5, 211, 198, 34],
    );
pub const FWPM_PROVIDER_MPSSVC_TENANT_RESTRICTIONS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3497103353,
        17626,
        20304,
        [157, 194, 201, 99, 164, 36, 118, 19],
    );
pub const FWPM_PROVIDER_MPSSVC_WF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3737917130,
    16179,
    17222,
    [190, 30, 143, 180, 174, 15, 61, 98],
);
pub const FWPM_PROVIDER_MPSSVC_WSH: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1259681589,
        4169,
        17536,
        [170, 180, 209, 185, 189, 192, 55, 16],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_PROVIDER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_PROVIDER_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_PROVIDER_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_PROVIDER_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_PROVIDER_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_PROVIDER_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_PROVIDER_TCP_CHIMNEY_OFFLOAD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2305466782,
        39476,
        19403,
        [174, 121, 190, 185, 18, 124, 132, 185],
    );
pub const FWPM_PROVIDER_TCP_TEMPLATES: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        1993329968,
        13204,
        17197,
        [190, 211, 68, 26, 229, 14, 99, 195],
    );
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_SERVICE_STATE(pub i32);
pub const FWPM_SERVICE_STOPPED: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(0i32);
pub const FWPM_SERVICE_START_PENDING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(1i32);
pub const FWPM_SERVICE_STOP_PENDING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(2i32);
pub const FWPM_SERVICE_RUNNING: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(3i32);
pub const FWPM_SERVICE_STATE_MAX: FWPM_SERVICE_STATE = FWPM_SERVICE_STATE(4i32);
impl ::std::convert::From<i32> for FWPM_SERVICE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_SERVICE_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWPM_SESSION0 {
    pub sessionKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub txnWaitTimeoutInMSec: u32,
    pub processId: u32,
    pub sid: *mut super::super::Security::SID,
    pub username: super::super::Foundation::PWSTR,
    pub kernelMode: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWPM_SESSION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWPM_SESSION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWPM_SESSION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SESSION0")
            .field("sessionKey", &self.sessionKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("txnWaitTimeoutInMSec", &self.txnWaitTimeoutInMSec)
            .field("processId", &self.processId)
            .field("sid", &self.sid)
            .field("username", &self.username)
            .field("kernelMode", &self.kernelMode)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWPM_SESSION0 {
    fn eq(&self, other: &Self) -> bool {
        self.sessionKey == other.sessionKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.txnWaitTimeoutInMSec == other.txnWaitTimeoutInMSec
            && self.processId == other.processId
            && self.sid == other.sid
            && self.username == other.username
            && self.kernelMode == other.kernelMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWPM_SESSION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWPM_SESSION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SESSION_ENUM_TEMPLATE0 {
    pub reserved: u64,
}
impl FWPM_SESSION_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SESSION_ENUM_TEMPLATE0")
            .field("reserved", &self.reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::std::cmp::Eq for FWPM_SESSION_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SESSION_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_SESSION_FLAG_DYNAMIC: u32 = 1u32;
pub const FWPM_SESSION_FLAG_RESERVED: u32 = 268435456u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl FWPM_STATISTICS0 {}
impl ::std::default::Default for FWPM_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_STATISTICS0")
            .field("numLayerStatistics", &self.numLayerStatistics)
            .field("layerStatistics", &self.layerStatistics)
            .field(
                "inboundAllowedConnectionsV4",
                &self.inboundAllowedConnectionsV4,
            )
            .field(
                "inboundBlockedConnectionsV4",
                &self.inboundBlockedConnectionsV4,
            )
            .field(
                "outboundAllowedConnectionsV4",
                &self.outboundAllowedConnectionsV4,
            )
            .field(
                "outboundBlockedConnectionsV4",
                &self.outboundBlockedConnectionsV4,
            )
            .field(
                "inboundAllowedConnectionsV6",
                &self.inboundAllowedConnectionsV6,
            )
            .field(
                "inboundBlockedConnectionsV6",
                &self.inboundBlockedConnectionsV6,
            )
            .field(
                "outboundAllowedConnectionsV6",
                &self.outboundAllowedConnectionsV6,
            )
            .field(
                "outboundBlockedConnectionsV6",
                &self.outboundBlockedConnectionsV6,
            )
            .field(
                "inboundActiveConnectionsV4",
                &self.inboundActiveConnectionsV4,
            )
            .field(
                "outboundActiveConnectionsV4",
                &self.outboundActiveConnectionsV4,
            )
            .field(
                "inboundActiveConnectionsV6",
                &self.inboundActiveConnectionsV6,
            )
            .field(
                "outboundActiveConnectionsV6",
                &self.outboundActiveConnectionsV6,
            )
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
            .field(
                "reauthReasonNewArrivalInterface",
                &self.reauthReasonNewArrivalInterface,
            )
            .field(
                "reauthReasonNewNextHopInterface",
                &self.reauthReasonNewNextHopInterface,
            )
            .field(
                "reauthReasonProfileCrossing",
                &self.reauthReasonProfileCrossing,
            )
            .field(
                "reauthReasonClassifyCompletion",
                &self.reauthReasonClassifyCompletion,
            )
            .field(
                "reauthReasonIPSecPropertiesChanged",
                &self.reauthReasonIPSecPropertiesChanged,
            )
            .field(
                "reauthReasonMidStreamInspection",
                &self.reauthReasonMidStreamInspection,
            )
            .field(
                "reauthReasonSocketPropertyChanged",
                &self.reauthReasonSocketPropertyChanged,
            )
            .field(
                "reauthReasonNewInboundMCastBCastPacket",
                &self.reauthReasonNewInboundMCastBCastPacket,
            )
            .field(
                "reauthReasonEDPPolicyChanged",
                &self.reauthReasonEDPPolicyChanged,
            )
            .field(
                "reauthReasonProxyHandleChanged",
                &self.reauthReasonProxyHandleChanged,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numLayerStatistics == other.numLayerStatistics
            && self.layerStatistics == other.layerStatistics
            && self.inboundAllowedConnectionsV4 == other.inboundAllowedConnectionsV4
            && self.inboundBlockedConnectionsV4 == other.inboundBlockedConnectionsV4
            && self.outboundAllowedConnectionsV4 == other.outboundAllowedConnectionsV4
            && self.outboundBlockedConnectionsV4 == other.outboundBlockedConnectionsV4
            && self.inboundAllowedConnectionsV6 == other.inboundAllowedConnectionsV6
            && self.inboundBlockedConnectionsV6 == other.inboundBlockedConnectionsV6
            && self.outboundAllowedConnectionsV6 == other.outboundAllowedConnectionsV6
            && self.outboundBlockedConnectionsV6 == other.outboundBlockedConnectionsV6
            && self.inboundActiveConnectionsV4 == other.inboundActiveConnectionsV4
            && self.outboundActiveConnectionsV4 == other.outboundActiveConnectionsV4
            && self.inboundActiveConnectionsV6 == other.inboundActiveConnectionsV6
            && self.outboundActiveConnectionsV6 == other.outboundActiveConnectionsV6
            && self.reauthDirInbound == other.reauthDirInbound
            && self.reauthDirOutbound == other.reauthDirOutbound
            && self.reauthFamilyV4 == other.reauthFamilyV4
            && self.reauthFamilyV6 == other.reauthFamilyV6
            && self.reauthProtoOther == other.reauthProtoOther
            && self.reauthProtoIPv4 == other.reauthProtoIPv4
            && self.reauthProtoIPv6 == other.reauthProtoIPv6
            && self.reauthProtoICMP == other.reauthProtoICMP
            && self.reauthProtoICMP6 == other.reauthProtoICMP6
            && self.reauthProtoUDP == other.reauthProtoUDP
            && self.reauthProtoTCP == other.reauthProtoTCP
            && self.reauthReasonPolicyChange == other.reauthReasonPolicyChange
            && self.reauthReasonNewArrivalInterface == other.reauthReasonNewArrivalInterface
            && self.reauthReasonNewNextHopInterface == other.reauthReasonNewNextHopInterface
            && self.reauthReasonProfileCrossing == other.reauthReasonProfileCrossing
            && self.reauthReasonClassifyCompletion == other.reauthReasonClassifyCompletion
            && self.reauthReasonIPSecPropertiesChanged == other.reauthReasonIPSecPropertiesChanged
            && self.reauthReasonMidStreamInspection == other.reauthReasonMidStreamInspection
            && self.reauthReasonSocketPropertyChanged == other.reauthReasonSocketPropertyChanged
            && self.reauthReasonNewInboundMCastBCastPacket
                == other.reauthReasonNewInboundMCastBCastPacket
            && self.reauthReasonEDPPolicyChanged == other.reauthReasonEDPPolicyChanged
            && self.reauthReasonProxyHandleChanged == other.reauthReasonProxyHandleChanged
    }
}
impl ::std::cmp::Eq for FWPM_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for FWPM_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_SUBLAYER0 {
    pub subLayerKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub providerKey: *mut ::windows::runtime::GUID,
    pub providerData: FWP_BYTE_BLOB,
    pub weight: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_SUBLAYER0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_SUBLAYER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_SUBLAYER0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SUBLAYER0")
            .field("subLayerKey", &self.subLayerKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("providerKey", &self.providerKey)
            .field("providerData", &self.providerData)
            .field("weight", &self.weight)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_SUBLAYER0 {
    fn eq(&self, other: &Self) -> bool {
        self.subLayerKey == other.subLayerKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.providerKey == other.providerKey
            && self.providerData == other.providerData
            && self.weight == other.weight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_SUBLAYER0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_SUBLAYER0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SUBLAYER_CHANGE0 {
    pub changeType: FWPM_CHANGE_TYPE,
    pub subLayerKey: ::windows::runtime::GUID,
}
impl FWPM_SUBLAYER_CHANGE0 {}
impl ::std::default::Default for FWPM_SUBLAYER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SUBLAYER_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SUBLAYER_CHANGE0")
            .field("changeType", &self.changeType)
            .field("subLayerKey", &self.subLayerKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SUBLAYER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.subLayerKey == other.subLayerKey
    }
}
impl ::std::cmp::Eq for FWPM_SUBLAYER_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SUBLAYER_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_SUBLAYER_CHANGE_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    change: *const FWPM_SUBLAYER_CHANGE0,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    pub providerKey: *mut ::windows::runtime::GUID,
}
impl FWPM_SUBLAYER_ENUM_TEMPLATE0 {}
impl ::std::default::Default for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SUBLAYER_ENUM_TEMPLATE0")
            .field("providerKey", &self.providerKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey
    }
}
impl ::std::cmp::Eq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_SUBLAYER_FLAG_PERSISTENT: u32 = 1u32;
pub const FWPM_SUBLAYER_INSPECTION: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2272598497,
        59049,
        16805,
        [129, 180, 140, 79, 17, 142, 74, 96],
    );
pub const FWPM_SUBLAYER_IPSEC_DOSP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3765884274,
        23869,
        18671,
        [128, 43, 144, 158, 221, 176, 152, 189],
    );
pub const FWPM_SUBLAYER_IPSEC_FORWARD_OUTBOUND_TUNNEL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2768776819,
        36721,
        17753,
        [138, 154, 16, 28, 234, 4, 239, 135],
    );
pub const FWPM_SUBLAYER_IPSEC_SECURITY_REALM: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        933590785,
        22660,
        18788,
        [146, 184, 62, 112, 70, 136, 176, 173],
    );
pub const FWPM_SUBLAYER_IPSEC_TUNNEL: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2213714413,
        40948,
        18791,
        [175, 244, 195, 9, 244, 218, 184, 39],
    );
pub const FWPM_SUBLAYER_LIPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    460701902,
    65376,
    18193,
    [167, 15, 180, 149, 140, 195, 178, 208],
);
pub const FWPM_SUBLAYER_MPSSVC_EDP: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        161775160,
        64151,
        18203,
        [177, 35, 24, 188, 215, 230, 80, 113],
    );
pub const FWPM_SUBLAYER_MPSSVC_QUARANTINE: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3016610881,
        44944,
        16826,
        [167, 69, 124, 96, 8, 255, 35, 2],
    );
pub const FWPM_SUBLAYER_MPSSVC_TENANT_RESTRICTIONS: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        516343777,
        64985,
        18314,
        [181, 95, 255, 139, 161, 210, 193, 125],
    );
pub const FWPM_SUBLAYER_MPSSVC_WF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3016610881,
    44944,
    16826,
    [167, 69, 124, 96, 8, 255, 35, 1],
);
pub const FWPM_SUBLAYER_MPSSVC_WSH: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3016610881,
        44944,
        16826,
        [167, 69, 124, 96, 8, 255, 35, 0],
    );
pub const FWPM_SUBLAYER_RPC_AUDIT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    1972143348,
    64328,
    19945,
    [154, 235, 62, 217, 85, 26, 177, 253],
);
pub const FWPM_SUBLAYER_SECURE_SOCKET: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        363228695,
        16188,
        20347,
        [170, 108, 129, 42, 166, 19, 221, 130],
    );
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SUBLAYER_SUBSCRIPTION0 {
    pub enumTemplate: *mut FWPM_SUBLAYER_ENUM_TEMPLATE0,
    pub flags: FWPM_SUBSCRIPTION_FLAGS,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_SUBLAYER_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SUBLAYER_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_SUBLAYER_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SUBLAYER_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_SUBLAYER_TCP_CHIMNEY_OFFLOAD: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        863373497,
        47061,
        19807,
        [130, 249, 54, 24, 97, 139, 192, 88],
    );
pub const FWPM_SUBLAYER_TCP_TEMPLATES: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        608312783,
        2757,
        19626,
        [158, 20, 80, 246, 227, 99, 106, 240],
    );
pub const FWPM_SUBLAYER_TEREDO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3127499878,
    20854,
    18809,
    [156, 137, 38, 167, 180, 106, 131, 39],
);
pub const FWPM_SUBLAYER_UNIVERSAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    4005481475,
    52948,
    17280,
    [129, 154, 39, 52, 57, 123, 43, 116],
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_SUBSCRIPTION_FLAGS(pub u32);
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_ADD: FWPM_SUBSCRIPTION_FLAGS =
    FWPM_SUBSCRIPTION_FLAGS(1u32);
pub const FWPM_SUBSCRIPTION_FLAG_NOTIFY_ON_DELETE: FWPM_SUBSCRIPTION_FLAGS =
    FWPM_SUBSCRIPTION_FLAGS(2u32);
impl ::std::convert::From<u32> for FWPM_SUBSCRIPTION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_SUBSCRIPTION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FWPM_SUBSCRIPTION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FWPM_SUBSCRIPTION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FWPM_SUBSCRIPTION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FWPM_SUBSCRIPTION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FWPM_SUBSCRIPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SYSTEM_PORTS0 {
    pub numTypes: u32,
    pub types: *mut FWPM_SYSTEM_PORTS_BY_TYPE0,
}
impl FWPM_SYSTEM_PORTS0 {}
impl ::std::default::Default for FWPM_SYSTEM_PORTS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SYSTEM_PORTS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SYSTEM_PORTS0")
            .field("numTypes", &self.numTypes)
            .field("types", &self.types)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SYSTEM_PORTS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numTypes == other.numTypes && self.types == other.types
    }
}
impl ::std::cmp::Eq for FWPM_SYSTEM_PORTS0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SYSTEM_PORTS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_SYSTEM_PORTS_BY_TYPE0 {
    pub r#type: FWPM_SYSTEM_PORT_TYPE,
    pub numPorts: u32,
    pub ports: *mut u16,
}
impl FWPM_SYSTEM_PORTS_BY_TYPE0 {}
impl ::std::default::Default for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_SYSTEM_PORTS_BY_TYPE0")
            .field("r#type", &self.r#type)
            .field("numPorts", &self.numPorts)
            .field("ports", &self.ports)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.numPorts == other.numPorts && self.ports == other.ports
    }
}
impl ::std::cmp::Eq for FWPM_SYSTEM_PORTS_BY_TYPE0 {}
unsafe impl ::windows::runtime::Abi for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type FWPM_SYSTEM_PORTS_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    sysports: *const FWPM_SYSTEM_PORTS0,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_SYSTEM_PORT_TYPE(pub i32);
pub const FWPM_SYSTEM_PORT_RPC_EPMAP: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(0i32);
pub const FWPM_SYSTEM_PORT_TEREDO: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(1i32);
pub const FWPM_SYSTEM_PORT_IPHTTPS_IN: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(2i32);
pub const FWPM_SYSTEM_PORT_IPHTTPS_OUT: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(3i32);
pub const FWPM_SYSTEM_PORT_TYPE_MAX: FWPM_SYSTEM_PORT_TYPE = FWPM_SYSTEM_PORT_TYPE(4i32);
impl ::std::convert::From<i32> for FWPM_SYSTEM_PORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_SYSTEM_PORT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_TUNNEL_FLAG_ENABLE_VIRTUAL_IF_TUNNELING: u32 = 2u32;
pub const FWPM_TUNNEL_FLAG_POINT_TO_POINT: u32 = 1u32;
pub const FWPM_TUNNEL_FLAG_RESERVED0: u32 = 4u32;
pub const FWPM_TXN_READ_ONLY: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0 {
    pub eventType: FWPM_VSWITCH_EVENT_TYPE,
    pub vSwitchId: super::super::Foundation::PWSTR,
    pub Anonymous: FWPM_VSWITCH_EVENT0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_VSWITCH_EVENT0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_VSWITCH_EVENT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_VSWITCH_EVENT0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_VSWITCH_EVENT0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union FWPM_VSWITCH_EVENT0_0 {
    pub positionInfo: FWPM_VSWITCH_EVENT0_0_0,
    pub reorderInfo: FWPM_VSWITCH_EVENT0_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_VSWITCH_EVENT0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_VSWITCH_EVENT0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_VSWITCH_EVENT0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0_0_0 {
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_VSWITCH_EVENT0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_VSWITCH_EVENT0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_VSWITCH_EVENT0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_positionInfo_e__Struct")
            .field(
                "numvSwitchFilterExtensions",
                &self.numvSwitchFilterExtensions,
            )
            .field("vSwitchFilterExtensions", &self.vSwitchFilterExtensions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.numvSwitchFilterExtensions == other.numvSwitchFilterExtensions
            && self.vSwitchFilterExtensions == other.vSwitchFilterExtensions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_VSWITCH_EVENT0_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FWPM_VSWITCH_EVENT0_0_1 {
    pub inRequiredPosition: super::super::Foundation::BOOL,
    pub numvSwitchFilterExtensions: u32,
    pub vSwitchFilterExtensions: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FWPM_VSWITCH_EVENT0_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FWPM_VSWITCH_EVENT0_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FWPM_VSWITCH_EVENT0_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_reorderInfo_e__Struct")
            .field("inRequiredPosition", &self.inRequiredPosition)
            .field(
                "numvSwitchFilterExtensions",
                &self.numvSwitchFilterExtensions,
            )
            .field("vSwitchFilterExtensions", &self.vSwitchFilterExtensions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FWPM_VSWITCH_EVENT0_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.inRequiredPosition == other.inRequiredPosition
            && self.numvSwitchFilterExtensions == other.numvSwitchFilterExtensions
            && self.vSwitchFilterExtensions == other.vSwitchFilterExtensions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FWPM_VSWITCH_EVENT0_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT0_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type FWPM_VSWITCH_EVENT_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    vswitchevent: *const FWPM_VSWITCH_EVENT0,
) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
impl FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {}
impl ::std::default::Default for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWPM_VSWITCH_EVENT_SUBSCRIPTION0")
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::std::cmp::Eq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {}
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWPM_VSWITCH_EVENT_TYPE(pub i32);
pub const FWPM_VSWITCH_EVENT_FILTER_ADD_TO_INCOMPLETE_LAYER: FWPM_VSWITCH_EVENT_TYPE =
    FWPM_VSWITCH_EVENT_TYPE(0i32);
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_NOT_IN_REQUIRED_POSITION: FWPM_VSWITCH_EVENT_TYPE =
    FWPM_VSWITCH_EVENT_TYPE(1i32);
pub const FWPM_VSWITCH_EVENT_ENABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE =
    FWPM_VSWITCH_EVENT_TYPE(2i32);
pub const FWPM_VSWITCH_EVENT_DISABLED_FOR_INSPECTION: FWPM_VSWITCH_EVENT_TYPE =
    FWPM_VSWITCH_EVENT_TYPE(3i32);
pub const FWPM_VSWITCH_EVENT_FILTER_ENGINE_REORDER: FWPM_VSWITCH_EVENT_TYPE =
    FWPM_VSWITCH_EVENT_TYPE(4i32);
pub const FWPM_VSWITCH_EVENT_MAX: FWPM_VSWITCH_EVENT_TYPE = FWPM_VSWITCH_EVENT_TYPE(5i32);
impl ::std::convert::From<i32> for FWPM_VSWITCH_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWPM_VSWITCH_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWPM_WEIGHT_RANGE_IKE_EXEMPTIONS: u32 = 12u32;
pub const FWPM_WEIGHT_RANGE_IPSEC: u32 = 0u32;
pub const FWPS_ALE_ENDPOINT_FLAG_IPSEC_SECURED: u32 = 1u32;
pub const FWPS_CLASSIFY_OUT_FLAG_ABSORB: u32 = 1u32;
pub const FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_CHECK: u32 = 8u32;
pub const FWPS_CLASSIFY_OUT_FLAG_ALE_FAST_CACHE_POSSIBLE: u32 = 16u32;
pub const FWPS_CLASSIFY_OUT_FLAG_BUFFER_LIMIT_REACHED: u32 = 2u32;
pub const FWPS_CLASSIFY_OUT_FLAG_NO_MORE_DATA: u32 = 4u32;
pub const FWPS_FILTER_FLAG_CLEAR_ACTION_RIGHT: u32 = 1u32;
pub const FWPS_FILTER_FLAG_HAS_SECURITY_REALM_PROVIDER_CONTEXT: u32 = 8u32;
pub const FWPS_FILTER_FLAG_IPSEC_NO_ACQUIRE_INITIATE: u32 = 32u32;
pub const FWPS_FILTER_FLAG_OR_CONDITIONS: u32 = 4u32;
pub const FWPS_FILTER_FLAG_PERMIT_IF_CALLOUT_UNREGISTERED: u32 = 2u32;
pub const FWPS_FILTER_FLAG_RESERVED0: u32 = 64u32;
pub const FWPS_FILTER_FLAG_RESERVED1: u32 = 128u32;
pub const FWPS_FILTER_FLAG_SILENT_MODE: u32 = 16u32;
pub const FWPS_INCOMING_FLAG_ABSORB: u32 = 4u32;
pub const FWPS_INCOMING_FLAG_CACHE_SAFE: u32 = 1u32;
pub const FWPS_INCOMING_FLAG_CONNECTION_FAILING_INDICATION: u32 = 8u32;
pub const FWPS_INCOMING_FLAG_ENFORCE_QUERY: u32 = 2u32;
pub const FWPS_INCOMING_FLAG_IS_LOCAL_ONLY_FLOW: u32 = 128u32;
pub const FWPS_INCOMING_FLAG_IS_LOOSE_SOURCE_FLOW: u32 = 64u32;
pub const FWPS_INCOMING_FLAG_MID_STREAM_INSPECTION: u32 = 16u32;
pub const FWPS_INCOMING_FLAG_RECLASSIFY: u32 = 32u32;
pub const FWPS_INCOMING_FLAG_RESERVED0: u32 = 256u32;
pub const FWPS_L2_INCOMING_FLAG_IS_RAW_IPV4_FRAMING: u32 = 1u32;
pub const FWPS_L2_INCOMING_FLAG_IS_RAW_IPV6_FRAMING: u32 = 2u32;
pub const FWPS_L2_INCOMING_FLAG_RECLASSIFY_MULTI_DESTINATION: u32 = 8u32;
pub const FWPS_L2_METADATA_FIELD_ETHERNET_MAC_HEADER_SIZE: u32 = 1u32;
pub const FWPS_L2_METADATA_FIELD_RESERVED: u32 = 2147483648u32;
pub const FWPS_L2_METADATA_FIELD_VSWITCH_DESTINATION_PORT_ID: u32 = 32u32;
pub const FWPS_L2_METADATA_FIELD_VSWITCH_PACKET_CONTEXT: u32 = 16u32;
pub const FWPS_L2_METADATA_FIELD_VSWITCH_SOURCE_NIC_INDEX: u32 = 8u32;
pub const FWPS_L2_METADATA_FIELD_VSWITCH_SOURCE_PORT_ID: u32 = 4u32;
pub const FWPS_L2_METADATA_FIELD_WIFI_OPERATION_MODE: u32 = 2u32;
pub const FWPS_METADATA_FIELD_ALE_CLASSIFY_REQUIRED: u32 = 4194304u32;
pub const FWPS_METADATA_FIELD_COMPARTMENT_ID: u32 = 2048u32;
pub const FWPS_METADATA_FIELD_COMPLETION_HANDLE: u32 = 16384u32;
pub const FWPS_METADATA_FIELD_DESTINATION_INTERFACE_INDEX: u32 = 512u32;
pub const FWPS_METADATA_FIELD_DESTINATION_PREFIX: u32 = 16777216u32;
pub const FWPS_METADATA_FIELD_DISCARD_REASON: u32 = 1u32;
pub const FWPS_METADATA_FIELD_ETHER_FRAME_LENGTH: u32 = 33554432u32;
pub const FWPS_METADATA_FIELD_FLOW_HANDLE: u32 = 2u32;
pub const FWPS_METADATA_FIELD_FORWARD_LAYER_INBOUND_PASS_THRU: u32 = 2097152u32;
pub const FWPS_METADATA_FIELD_FORWARD_LAYER_OUTBOUND_PASS_THRU: u32 = 1048576u32;
pub const FWPS_METADATA_FIELD_FRAGMENT_DATA: u32 = 4096u32;
pub const FWPS_METADATA_FIELD_ICMP_ID_AND_SEQUENCE: u32 = 134217728u32;
pub const FWPS_METADATA_FIELD_IP_HEADER_SIZE: u32 = 4u32;
pub const FWPS_METADATA_FIELD_LOCAL_REDIRECT_TARGET_PID: u32 = 268435456u32;
pub const FWPS_METADATA_FIELD_ORIGINAL_DESTINATION: u32 = 536870912u32;
pub const FWPS_METADATA_FIELD_PACKET_DIRECTION: u32 = 262144u32;
pub const FWPS_METADATA_FIELD_PACKET_SYSTEM_CRITICAL: u32 = 524288u32;
pub const FWPS_METADATA_FIELD_PARENT_ENDPOINT_HANDLE: u32 = 67108864u32;
pub const FWPS_METADATA_FIELD_PATH_MTU: u32 = 8192u32;
pub const FWPS_METADATA_FIELD_PROCESS_ID: u32 = 32u32;
pub const FWPS_METADATA_FIELD_PROCESS_PATH: u32 = 8u32;
pub const FWPS_METADATA_FIELD_REDIRECT_RECORD_HANDLE: u32 = 1073741824u32;
pub const FWPS_METADATA_FIELD_REMOTE_SCOPE_ID: u32 = 131072u32;
pub const FWPS_METADATA_FIELD_RESERVED: u32 = 128u32;
pub const FWPS_METADATA_FIELD_SOURCE_INTERFACE_INDEX: u32 = 256u32;
pub const FWPS_METADATA_FIELD_SUB_PROCESS_TAG: u32 = 2147483648u32;
pub const FWPS_METADATA_FIELD_SYSTEM_FLAGS: u32 = 64u32;
pub const FWPS_METADATA_FIELD_TOKEN: u32 = 16u32;
pub const FWPS_METADATA_FIELD_TRANSPORT_CONTROL_DATA: u32 = 65536u32;
pub const FWPS_METADATA_FIELD_TRANSPORT_ENDPOINT_HANDLE: u32 = 32768u32;
pub const FWPS_METADATA_FIELD_TRANSPORT_HEADER_INCLUDE_HEADER: u32 = 8388608u32;
pub const FWPS_METADATA_FIELD_TRANSPORT_HEADER_SIZE: u32 = 1024u32;
pub const FWPS_RIGHT_ACTION_WRITE: u32 = 1u32;
pub const FWP_ACTION_FLAG_CALLOUT: u32 = 16384u32;
pub const FWP_ACTION_FLAG_NON_TERMINATING: u32 = 8192u32;
pub const FWP_ACTION_FLAG_TERMINATING: u32 = 4096u32;
pub const FWP_ACTION_NONE: u32 = 7u32;
pub const FWP_ACTION_NONE_NO_MATCH: u32 = 8u32;
pub const FWP_ACTRL_MATCH_FILTER: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_AF(pub i32);
pub const FWP_AF_INET: FWP_AF = FWP_AF(0i32);
pub const FWP_AF_INET6: FWP_AF = FWP_AF(1i32);
pub const FWP_AF_ETHER: FWP_AF = FWP_AF(2i32);
pub const FWP_AF_NONE: FWP_AF = FWP_AF(3i32);
impl ::std::convert::From<i32> for FWP_AF {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_AF {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_BYTEMAP_ARRAY64_SIZE: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWP_BYTE_ARRAY16 {
    pub byteArray16: [u8; 16],
}
impl FWP_BYTE_ARRAY16 {}
impl ::std::default::Default for FWP_BYTE_ARRAY16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWP_BYTE_ARRAY16 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_BYTE_ARRAY16")
            .field("byteArray16", &self.byteArray16)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWP_BYTE_ARRAY16 {
    fn eq(&self, other: &Self) -> bool {
        self.byteArray16 == other.byteArray16
    }
}
impl ::std::cmp::Eq for FWP_BYTE_ARRAY16 {}
unsafe impl ::windows::runtime::Abi for FWP_BYTE_ARRAY16 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWP_BYTE_ARRAY6 {
    pub byteArray6: [u8; 6],
}
impl FWP_BYTE_ARRAY6 {}
impl ::std::default::Default for FWP_BYTE_ARRAY6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWP_BYTE_ARRAY6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_BYTE_ARRAY6")
            .field("byteArray6", &self.byteArray6)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWP_BYTE_ARRAY6 {
    fn eq(&self, other: &Self) -> bool {
        self.byteArray6 == other.byteArray6
    }
}
impl ::std::cmp::Eq for FWP_BYTE_ARRAY6 {}
unsafe impl ::windows::runtime::Abi for FWP_BYTE_ARRAY6 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_BYTE_ARRAY6_SIZE: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWP_BYTE_BLOB {
    pub size: u32,
    pub data: *mut u8,
}
impl FWP_BYTE_BLOB {}
impl ::std::default::Default for FWP_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWP_BYTE_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_BYTE_BLOB")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWP_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::std::cmp::Eq for FWP_BYTE_BLOB {}
unsafe impl ::windows::runtime::Abi for FWP_BYTE_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_CALLOUT_FLAG_ALLOW_L2_BATCH_CLASSIFY: u32 = 128u32;
pub const FWP_CALLOUT_FLAG_ALLOW_MID_STREAM_INSPECTION: u32 = 8u32;
pub const FWP_CALLOUT_FLAG_ALLOW_OFFLOAD: u32 = 2u32;
pub const FWP_CALLOUT_FLAG_ALLOW_RECLASSIFY: u32 = 16u32;
pub const FWP_CALLOUT_FLAG_ALLOW_RSC: u32 = 64u32;
pub const FWP_CALLOUT_FLAG_ALLOW_URO: u32 = 512u32;
pub const FWP_CALLOUT_FLAG_ALLOW_USO: u32 = 256u32;
pub const FWP_CALLOUT_FLAG_CONDITIONAL_ON_FLOW: u32 = 1u32;
pub const FWP_CALLOUT_FLAG_ENABLE_COMMIT_ADD_NOTIFY: u32 = 4u32;
pub const FWP_CALLOUT_FLAG_RESERVED1: u32 = 32u32;
pub const FWP_CALLOUT_FLAG_RESERVED2: u32 = 1024u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_CLASSIFY_OPTION_TYPE(pub i32);
pub const FWP_CLASSIFY_OPTION_MULTICAST_STATE: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(0i32);
pub const FWP_CLASSIFY_OPTION_LOOSE_SOURCE_MAPPING: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(1i32);
pub const FWP_CLASSIFY_OPTION_UNICAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(2i32);
pub const FWP_CLASSIFY_OPTION_MCAST_BCAST_LIFETIME: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(3i32);
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_SECURITY_FLAGS: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(4i32);
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_MM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(5i32);
pub const FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_QM_POLICY_KEY: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(6i32);
pub const FWP_CLASSIFY_OPTION_LOCAL_ONLY_MAPPING: FWP_CLASSIFY_OPTION_TYPE =
    FWP_CLASSIFY_OPTION_TYPE(7i32);
pub const FWP_CLASSIFY_OPTION_MAX: FWP_CLASSIFY_OPTION_TYPE = FWP_CLASSIFY_OPTION_TYPE(8i32);
impl ::std::convert::From<i32> for FWP_CLASSIFY_OPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_CLASSIFY_OPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_CONDITION_FLAG_IS_APPCONTAINER_LOOPBACK: u32 = 4194304u32;
pub const FWP_CONDITION_FLAG_IS_AUTH_FW: u32 = 65536u32;
pub const FWP_CONDITION_FLAG_IS_CONNECTION_REDIRECTED: u32 = 1048576u32;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT: u32 = 32u32;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT_GROUP: u32 = 64u32;
pub const FWP_CONDITION_FLAG_IS_HONORING_POLICY_AUTHORIZE: u32 = 33554432u32;
pub const FWP_CONDITION_FLAG_IS_IMPLICIT_BIND: u32 = 512u32;
pub const FWP_CONDITION_FLAG_IS_INBOUND_PASS_THRU: u32 = 524288u32;
pub const FWP_CONDITION_FLAG_IS_IPSEC_NATT_RECLASSIFY: u32 = 128u32;
pub const FWP_CONDITION_FLAG_IS_IPSEC_SECURED: u32 = 2u32;
pub const FWP_CONDITION_FLAG_IS_LOOPBACK: u32 = 1u32;
pub const FWP_CONDITION_FLAG_IS_NAME_APP_SPECIFIED: u32 = 16384u32;
pub const FWP_CONDITION_FLAG_IS_NON_APPCONTAINER_LOOPBACK: u32 = 8388608u32;
pub const FWP_CONDITION_FLAG_IS_OUTBOUND_PASS_THRU: u32 = 262144u32;
pub const FWP_CONDITION_FLAG_IS_PROMISCUOUS: u32 = 32768u32;
pub const FWP_CONDITION_FLAG_IS_PROXY_CONNECTION: u32 = 2097152u32;
pub const FWP_CONDITION_FLAG_IS_RAW_ENDPOINT: u32 = 16u32;
pub const FWP_CONDITION_FLAG_IS_REASSEMBLED: u32 = 1024u32;
pub const FWP_CONDITION_FLAG_IS_REAUTHORIZE: u32 = 4u32;
pub const FWP_CONDITION_FLAG_IS_RECLASSIFY: u32 = 131072u32;
pub const FWP_CONDITION_FLAG_IS_RESERVED: u32 = 16777216u32;
pub const FWP_CONDITION_FLAG_IS_WILDCARD_BIND: u32 = 8u32;
pub const FWP_CONDITION_FLAG_REQUIRES_ALE_CLASSIFY: u32 = 256u32;
pub const FWP_CONDITION_L2_IF_CONNECTOR_PRESENT: u32 = 128u32;
pub const FWP_CONDITION_L2_IS_IP_FRAGMENT_GROUP: u32 = 64u32;
pub const FWP_CONDITION_L2_IS_MALFORMED_PACKET: u32 = 32u32;
pub const FWP_CONDITION_L2_IS_MOBILE_BROADBAND: u32 = 4u32;
pub const FWP_CONDITION_L2_IS_NATIVE_ETHERNET: u32 = 1u32;
pub const FWP_CONDITION_L2_IS_VM2VM: u32 = 16u32;
pub const FWP_CONDITION_L2_IS_WIFI: u32 = 2u32;
pub const FWP_CONDITION_L2_IS_WIFI_DIRECT_DATA: u32 = 8u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_CHECK_OFFLOAD: u32 = 65536u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_CLASSIFY_COMPLETION: u32 = 16u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_EDP_POLICY_CHANGED: u32 = 512u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_IPSEC_PROPERTIES_CHANGED: u32 = 32u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_MID_STREAM_INSPECTION: u32 = 64u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_ARRIVAL_INTERFACE: u32 = 2u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_INBOUND_MCAST_BCAST_PACKET: u32 = 256u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_NEXTHOP_INTERFACE: u32 = 4u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_POLICY_CHANGE: u32 = 1u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROFILE_CROSSING: u32 = 8u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROXY_HANDLE_CHANGED: u32 = 16384u32;
pub const FWP_CONDITION_REAUTHORIZE_REASON_SOCKET_PROPERTY_CHANGED: u32 = 128u32;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_ALLOW_EDGE_TRAFFIC: u32 = 2u32;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_DENY_EDGE_TRAFFIC: u32 = 4u32;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_IS_SYSTEM_PORT_RPC: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_CONDITION_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_CONDITION_VALUE0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_CONDITION_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_CONDITION_VALUE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_CONDITION_VALUE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_CONDITION_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_CONDITION_VALUE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub unicodeString: super::super::Foundation::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
    pub v4AddrMask: *mut FWP_V4_ADDR_AND_MASK,
    pub v6AddrMask: *mut FWP_V6_ADDR_AND_MASK,
    pub rangeValue: *mut FWP_RANGE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_CONDITION_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_CONDITION_VALUE0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_CONDITION_VALUE0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_CONDITION_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_CONDITION_VALUE0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_DATA_TYPE(pub i32);
pub const FWP_EMPTY: FWP_DATA_TYPE = FWP_DATA_TYPE(0i32);
pub const FWP_UINT8: FWP_DATA_TYPE = FWP_DATA_TYPE(1i32);
pub const FWP_UINT16: FWP_DATA_TYPE = FWP_DATA_TYPE(2i32);
pub const FWP_UINT32: FWP_DATA_TYPE = FWP_DATA_TYPE(3i32);
pub const FWP_UINT64: FWP_DATA_TYPE = FWP_DATA_TYPE(4i32);
pub const FWP_INT8: FWP_DATA_TYPE = FWP_DATA_TYPE(5i32);
pub const FWP_INT16: FWP_DATA_TYPE = FWP_DATA_TYPE(6i32);
pub const FWP_INT32: FWP_DATA_TYPE = FWP_DATA_TYPE(7i32);
pub const FWP_INT64: FWP_DATA_TYPE = FWP_DATA_TYPE(8i32);
pub const FWP_FLOAT: FWP_DATA_TYPE = FWP_DATA_TYPE(9i32);
pub const FWP_DOUBLE: FWP_DATA_TYPE = FWP_DATA_TYPE(10i32);
pub const FWP_BYTE_ARRAY16_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(11i32);
pub const FWP_BYTE_BLOB_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(12i32);
pub const FWP_SID: FWP_DATA_TYPE = FWP_DATA_TYPE(13i32);
pub const FWP_SECURITY_DESCRIPTOR_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(14i32);
pub const FWP_TOKEN_INFORMATION_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(15i32);
pub const FWP_TOKEN_ACCESS_INFORMATION_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(16i32);
pub const FWP_UNICODE_STRING_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(17i32);
pub const FWP_BYTE_ARRAY6_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(18i32);
pub const FWP_SINGLE_DATA_TYPE_MAX: FWP_DATA_TYPE = FWP_DATA_TYPE(255i32);
pub const FWP_V4_ADDR_MASK: FWP_DATA_TYPE = FWP_DATA_TYPE(256i32);
pub const FWP_V6_ADDR_MASK: FWP_DATA_TYPE = FWP_DATA_TYPE(257i32);
pub const FWP_RANGE_TYPE: FWP_DATA_TYPE = FWP_DATA_TYPE(258i32);
pub const FWP_DATA_TYPE_MAX: FWP_DATA_TYPE = FWP_DATA_TYPE(259i32);
impl ::std::convert::From<i32> for FWP_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_DATA_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_DIRECTION(pub i32);
pub const FWP_DIRECTION_OUTBOUND: FWP_DIRECTION = FWP_DIRECTION(0i32);
pub const FWP_DIRECTION_INBOUND: FWP_DIRECTION = FWP_DIRECTION(1i32);
pub const FWP_DIRECTION_MAX: FWP_DIRECTION = FWP_DIRECTION(2i32);
impl ::std::convert::From<i32> for FWP_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_DIRECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_ETHER_ENCAP_METHOD(pub i32);
pub const FWP_ETHER_ENCAP_METHOD_ETHER_V2: FWP_ETHER_ENCAP_METHOD = FWP_ETHER_ENCAP_METHOD(0i32);
pub const FWP_ETHER_ENCAP_METHOD_SNAP: FWP_ETHER_ENCAP_METHOD = FWP_ETHER_ENCAP_METHOD(1i32);
pub const FWP_ETHER_ENCAP_METHOD_SNAP_W_OUI_ZERO: FWP_ETHER_ENCAP_METHOD =
    FWP_ETHER_ENCAP_METHOD(3i32);
impl ::std::convert::From<i32> for FWP_ETHER_ENCAP_METHOD {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_ETHER_ENCAP_METHOD {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_FILTER_ENUM_FLAG_BEST_TERMINATING_MATCH: u32 = 1u32;
pub const FWP_FILTER_ENUM_FLAG_BOOTTIME_ONLY: u32 = 4u32;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_BOOTTIME: u32 = 8u32;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_DISABLED: u32 = 16u32;
pub const FWP_FILTER_ENUM_FLAG_RESERVED1: u32 = 32u32;
pub const FWP_FILTER_ENUM_FLAG_SORTED: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_FILTER_ENUM_TYPE(pub i32);
pub const FWP_FILTER_ENUM_FULLY_CONTAINED: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(0i32);
pub const FWP_FILTER_ENUM_OVERLAPPING: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(1i32);
pub const FWP_FILTER_ENUM_TYPE_MAX: FWP_FILTER_ENUM_TYPE = FWP_FILTER_ENUM_TYPE(2i32);
impl ::std::convert::From<i32> for FWP_FILTER_ENUM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_FILTER_ENUM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_IP_VERSION(pub i32);
pub const FWP_IP_VERSION_V4: FWP_IP_VERSION = FWP_IP_VERSION(0i32);
pub const FWP_IP_VERSION_V6: FWP_IP_VERSION = FWP_IP_VERSION(1i32);
pub const FWP_IP_VERSION_NONE: FWP_IP_VERSION = FWP_IP_VERSION(2i32);
pub const FWP_IP_VERSION_MAX: FWP_IP_VERSION = FWP_IP_VERSION(3i32);
impl ::std::convert::From<i32> for FWP_IP_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_IP_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_MATCH_TYPE(pub i32);
pub const FWP_MATCH_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(0i32);
pub const FWP_MATCH_GREATER: FWP_MATCH_TYPE = FWP_MATCH_TYPE(1i32);
pub const FWP_MATCH_LESS: FWP_MATCH_TYPE = FWP_MATCH_TYPE(2i32);
pub const FWP_MATCH_GREATER_OR_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(3i32);
pub const FWP_MATCH_LESS_OR_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(4i32);
pub const FWP_MATCH_RANGE: FWP_MATCH_TYPE = FWP_MATCH_TYPE(5i32);
pub const FWP_MATCH_FLAGS_ALL_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(6i32);
pub const FWP_MATCH_FLAGS_ANY_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(7i32);
pub const FWP_MATCH_FLAGS_NONE_SET: FWP_MATCH_TYPE = FWP_MATCH_TYPE(8i32);
pub const FWP_MATCH_EQUAL_CASE_INSENSITIVE: FWP_MATCH_TYPE = FWP_MATCH_TYPE(9i32);
pub const FWP_MATCH_NOT_EQUAL: FWP_MATCH_TYPE = FWP_MATCH_TYPE(10i32);
pub const FWP_MATCH_PREFIX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(11i32);
pub const FWP_MATCH_NOT_PREFIX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(12i32);
pub const FWP_MATCH_TYPE_MAX: FWP_MATCH_TYPE = FWP_MATCH_TYPE(13i32);
impl ::std::convert::From<i32> for FWP_MATCH_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_MATCH_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_OPTION_VALUE_ALLOW_GLOBAL_MULTICAST_STATE: u32 = 2u32;
pub const FWP_OPTION_VALUE_ALLOW_MULTICAST_STATE: u32 = 0u32;
pub const FWP_OPTION_VALUE_DENY_MULTICAST_STATE: u32 = 1u32;
pub const FWP_OPTION_VALUE_DISABLE_LOCAL_ONLY_MAPPING: u32 = 0u32;
pub const FWP_OPTION_VALUE_DISABLE_LOOSE_SOURCE: u32 = 0u32;
pub const FWP_OPTION_VALUE_ENABLE_LOCAL_ONLY_MAPPING: u32 = 1u32;
pub const FWP_OPTION_VALUE_ENABLE_LOOSE_SOURCE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_RANGE0 {
    pub valueLow: FWP_VALUE0,
    pub valueHigh: FWP_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_RANGE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_RANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_RANGE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_RANGE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_RANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_TOKEN_INFORMATION {
    pub sidCount: u32,
    pub sids: *mut super::super::Security::SID_AND_ATTRIBUTES,
    pub restrictedSidCount: u32,
    pub restrictedSids: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_TOKEN_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_TOKEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for FWP_TOKEN_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_TOKEN_INFORMATION")
            .field("sidCount", &self.sidCount)
            .field("sids", &self.sids)
            .field("restrictedSidCount", &self.restrictedSidCount)
            .field("restrictedSids", &self.restrictedSids)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_TOKEN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.sidCount == other.sidCount
            && self.sids == other.sids
            && self.restrictedSidCount == other.restrictedSidCount
            && self.restrictedSids == other.restrictedSids
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_TOKEN_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_TOKEN_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWP_V4_ADDR_AND_MASK {
    pub addr: u32,
    pub mask: u32,
}
impl FWP_V4_ADDR_AND_MASK {}
impl ::std::default::Default for FWP_V4_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWP_V4_ADDR_AND_MASK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_V4_ADDR_AND_MASK")
            .field("addr", &self.addr)
            .field("mask", &self.mask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWP_V4_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.mask == other.mask
    }
}
impl ::std::cmp::Eq for FWP_V4_ADDR_AND_MASK {}
unsafe impl ::windows::runtime::Abi for FWP_V4_ADDR_AND_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FWP_V6_ADDR_AND_MASK {
    pub addr: [u8; 16],
    pub prefixLength: u8,
}
impl FWP_V6_ADDR_AND_MASK {}
impl ::std::default::Default for FWP_V6_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FWP_V6_ADDR_AND_MASK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FWP_V6_ADDR_AND_MASK")
            .field("addr", &self.addr)
            .field("prefixLength", &self.prefixLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FWP_V6_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.prefixLength == other.prefixLength
    }
}
impl ::std::cmp::Eq for FWP_V6_ADDR_AND_MASK {}
unsafe impl ::windows::runtime::Abi for FWP_V6_ADDR_AND_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FWP_V6_ADDR_SIZE: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct FWP_VALUE0 {
    pub r#type: FWP_DATA_TYPE,
    pub Anonymous: FWP_VALUE0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_VALUE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_VALUE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_VALUE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_VALUE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub unicodeString: super::super::Foundation::PWSTR,
    pub byteArray6: *mut FWP_BYTE_ARRAY6,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl FWP_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for FWP_VALUE0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for FWP_VALUE0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for FWP_VALUE0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for FWP_VALUE0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FWP_VSWITCH_NETWORK_TYPE(pub i32);
pub const FWP_VSWITCH_NETWORK_TYPE_UNKNOWN: FWP_VSWITCH_NETWORK_TYPE =
    FWP_VSWITCH_NETWORK_TYPE(0i32);
pub const FWP_VSWITCH_NETWORK_TYPE_PRIVATE: FWP_VSWITCH_NETWORK_TYPE =
    FWP_VSWITCH_NETWORK_TYPE(1i32);
pub const FWP_VSWITCH_NETWORK_TYPE_INTERNAL: FWP_VSWITCH_NETWORK_TYPE =
    FWP_VSWITCH_NETWORK_TYPE(2i32);
pub const FWP_VSWITCH_NETWORK_TYPE_EXTERNAL: FWP_VSWITCH_NETWORK_TYPE =
    FWP_VSWITCH_NETWORK_TYPE(3i32);
impl ::std::convert::From<i32> for FWP_VSWITCH_NETWORK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FWP_VSWITCH_NETWORK_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    callout: *const FWPM_CALLOUT0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                callout: *const FWPM_CALLOUT0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(callout),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_CALLOUT_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_CALLOUT_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDeleteById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDeleteById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutDeleteById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_CALLOUT0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_CALLOUT0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u32,
    callout: *mut *mut FWPM_CALLOUT0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u32,
                callout: *mut *mut FWPM_CALLOUT0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(callout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    callout: *mut *mut FWPM_CALLOUT0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                callout: *mut *mut FWPM_CALLOUT0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(callout),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmCalloutSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutSubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_CALLOUT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_CALLOUT_CHANGE_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    changehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_CALLOUT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                changehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutSubscribeChanges0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(changehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_CALLOUT_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_CALLOUT_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmCalloutUnsubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    changehandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmCalloutUnsubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                changehandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmCalloutUnsubscribeChanges0(
            enginehandle.into_param().abi(),
            changehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_CONNECTION_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_CONNECTION_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_CONNECTION0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_CONNECTION0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    connection: *mut *mut FWPM_CONNECTION0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                connection: *mut *mut FWPM_CONNECTION0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(connection),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmConnectionGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmConnectionSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionSubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_CONNECTION_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_CONNECTION_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionSubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_CONNECTION_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionSubscribe0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmConnectionUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    eventshandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmConnectionUnsubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                eventshandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmConnectionUnsubscribe0(
            enginehandle.into_param().abi(),
            eventshandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmDynamicKeywordSubscribe0(
    flags: u32,
    callback: ::std::option::Option<FWPM_DYNAMIC_KEYWORD_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    subscriptionhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmDynamicKeywordSubscribe0(
                flags: u32,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                subscriptionhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmDynamicKeywordSubscribe0(
            ::std::mem::transmute(flags),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(subscriptionhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmDynamicKeywordUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    subscriptionhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmDynamicKeywordUnsubscribe0(
                subscriptionhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmDynamicKeywordUnsubscribe0(
            subscriptionhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmEngineClose0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineClose0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(FwpmEngineClose0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetOption0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    option: FWPM_ENGINE_OPTION,
    value: *mut *mut FWP_VALUE0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineGetOption0(
                enginehandle: super::super::Foundation::HANDLE,
                option: FWPM_ENGINE_OPTION,
                value: *mut *mut FWP_VALUE0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmEngineGetOption0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(option),
            ::std::mem::transmute(value),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmEngineGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_Rpc"
))]
#[inline]
pub unsafe fn FwpmEngineOpen0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    servername: Param0,
    authnservice: u32,
    authidentity: *const super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W,
    session: *const FWPM_SESSION0,
    enginehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineOpen0(
                servername: super::super::Foundation::PWSTR,
                authnservice: u32,
                authidentity: *const super::super::System::Rpc::SEC_WINNT_AUTH_IDENTITY_W,
                session: *const FWPM_SESSION0,
                enginehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmEngineOpen0(
            servername.into_param().abi(),
            ::std::mem::transmute(authnservice),
            ::std::mem::transmute(authidentity),
            ::std::mem::transmute(session),
            ::std::mem::transmute(enginehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetOption0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    option: FWPM_ENGINE_OPTION,
    newvalue: *const FWP_VALUE0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineSetOption0(
                enginehandle: super::super::Foundation::HANDLE,
                option: FWPM_ENGINE_OPTION,
                newvalue: *const FWP_VALUE0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmEngineSetOption0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(option),
            ::std::mem::transmute(newvalue),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmEngineSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmEngineSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmEngineSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    filter: *const FWPM_FILTER0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                filter: *const FWPM_FILTER0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(filter),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_FILTER_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_FILTER_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDeleteById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDeleteById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterDeleteById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_FILTER0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_FILTER0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    filter: *mut *mut FWPM_FILTER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                filter: *mut *mut FWPM_FILTER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(filter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    filter: *mut *mut FWPM_FILTER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                filter: *mut *mut FWPM_FILTER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(filter),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_FILTER_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_FILTER_CHANGE_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    changehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_FILTER_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                changehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterSubscribeChanges0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(changehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmFilterSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_FILTER_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_FILTER_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmFilterUnsubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    changehandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFilterUnsubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                changehandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmFilterUnsubscribeChanges0(
            enginehandle.into_param().abi(),
            changehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn FwpmFreeMemory0(p: *mut *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmFreeMemory0(p: *mut *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(FwpmFreeMemory0(::std::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmGetAppIdFromFileName0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    filename: Param0,
    appid: *mut *mut FWP_BYTE_BLOB,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmGetAppIdFromFileName0(
                filename: super::super::Foundation::PWSTR,
                appid: *mut *mut FWP_BYTE_BLOB,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmGetAppIdFromFileName0(
            filename.into_param().abi(),
            ::std::mem::transmute(appid),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u32,
    mainmodepolicy: *const FWPM_PROVIDER_CONTEXT0,
    tunnelpolicy: *const FWPM_PROVIDER_CONTEXT0,
    numfilterconditions: u32,
    filterconditions: *const FWPM_FILTER_CONDITION0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u32,
                mainmodepolicy: *const FWPM_PROVIDER_CONTEXT0,
                tunnelpolicy: *const FWPM_PROVIDER_CONTEXT0,
                numfilterconditions: u32,
                filterconditions: *const FWPM_FILTER_CONDITION0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmIPsecTunnelAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(mainmodepolicy),
            ::std::mem::transmute(tunnelpolicy),
            ::std::mem::transmute(numfilterconditions),
            ::std::mem::transmute(filterconditions),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u32,
    mainmodepolicy: *const FWPM_PROVIDER_CONTEXT1,
    tunnelpolicy: *const FWPM_PROVIDER_CONTEXT1,
    numfilterconditions: u32,
    filterconditions: *const FWPM_FILTER_CONDITION0,
    keymodkey: *const ::windows::runtime::GUID,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd1(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u32,
                mainmodepolicy: *const FWPM_PROVIDER_CONTEXT1,
                tunnelpolicy: *const FWPM_PROVIDER_CONTEXT1,
                numfilterconditions: u32,
                filterconditions: *const FWPM_FILTER_CONDITION0,
                keymodkey: *const ::windows::runtime::GUID,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmIPsecTunnelAdd1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(mainmodepolicy),
            ::std::mem::transmute(tunnelpolicy),
            ::std::mem::transmute(numfilterconditions),
            ::std::mem::transmute(filterconditions),
            ::std::mem::transmute(keymodkey),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u32,
    mainmodepolicy: *const FWPM_PROVIDER_CONTEXT2,
    tunnelpolicy: *const FWPM_PROVIDER_CONTEXT2,
    numfilterconditions: u32,
    filterconditions: *const FWPM_FILTER_CONDITION0,
    keymodkey: *const ::windows::runtime::GUID,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd2(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u32,
                mainmodepolicy: *const FWPM_PROVIDER_CONTEXT2,
                tunnelpolicy: *const FWPM_PROVIDER_CONTEXT2,
                numfilterconditions: u32,
                filterconditions: *const FWPM_FILTER_CONDITION0,
                keymodkey: *const ::windows::runtime::GUID,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmIPsecTunnelAdd2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(mainmodepolicy),
            ::std::mem::transmute(tunnelpolicy),
            ::std::mem::transmute(numfilterconditions),
            ::std::mem::transmute(filterconditions),
            ::std::mem::transmute(keymodkey),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmIPsecTunnelAdd3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u32,
    mainmodepolicy: *const FWPM_PROVIDER_CONTEXT3_,
    tunnelpolicy: *const FWPM_PROVIDER_CONTEXT3_,
    numfilterconditions: u32,
    filterconditions: *const FWPM_FILTER_CONDITION0,
    keymodkey: *const ::windows::runtime::GUID,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelAdd3(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u32,
                mainmodepolicy: *const FWPM_PROVIDER_CONTEXT3_,
                tunnelpolicy: *const FWPM_PROVIDER_CONTEXT3_,
                numfilterconditions: u32,
                filterconditions: *const FWPM_FILTER_CONDITION0,
                keymodkey: *const ::windows::runtime::GUID,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmIPsecTunnelAdd3(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(mainmodepolicy),
            ::std::mem::transmute(tunnelpolicy),
            ::std::mem::transmute(numfilterconditions),
            ::std::mem::transmute(filterconditions),
            ::std::mem::transmute(keymodkey),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmIPsecTunnelDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmIPsecTunnelDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmIPsecTunnelDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_LAYER_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_LAYER_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_LAYER0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_LAYER0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u16,
    layer: *mut *mut FWPM_LAYER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u16,
                layer: *mut *mut FWPM_LAYER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(layer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmLayerGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    layer: *mut *mut FWPM_LAYER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                layer: *mut *mut FWPM_LAYER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(layer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmLayerGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmLayerSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmLayerSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmLayerSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_NET_EVENT_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_NET_EVENT_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmNetEventDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT1,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum1(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT1,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum1(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT2,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum2(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT2,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum2(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT3,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum3(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT3,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum3(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum4<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT4_,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum4(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT4_,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum4(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventEnum5<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_NET_EVENT5_,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventEnum5(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_NET_EVENT5_,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventEnum5(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_NET_EVENT_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscribe0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_NET_EVENT_CALLBACK1>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe1(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscribe1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_NET_EVENT_CALLBACK2>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe2(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscribe2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_NET_EVENT_CALLBACK3>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe3(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscribe3(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscribe4<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_NET_EVENT_CALLBACK4>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscribe4(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_NET_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscribe4(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_NET_EVENT_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_NET_EVENT_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmNetEventUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    eventshandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventUnsubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                eventshandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventUnsubscribe0(
            enginehandle.into_param().abi(),
            eventshandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventsGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventsGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventsGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmNetEventsSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmNetEventsSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmNetEventsSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    provider: *const FWPM_PROVIDER0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                provider: *const FWPM_PROVIDER0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(provider),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    providercontext: *const FWPM_PROVIDER_CONTEXT0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                providercontext: *const FWPM_PROVIDER_CONTEXT0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(providercontext),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    providercontext: *const FWPM_PROVIDER_CONTEXT1,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd1(
                enginehandle: super::super::Foundation::HANDLE,
                providercontext: *const FWPM_PROVIDER_CONTEXT1,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextAdd1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(providercontext),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    providercontext: *const FWPM_PROVIDER_CONTEXT2,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd2(
                enginehandle: super::super::Foundation::HANDLE,
                providercontext: *const FWPM_PROVIDER_CONTEXT2,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextAdd2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(providercontext),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextAdd3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    providercontext: *const FWPM_PROVIDER_CONTEXT3_,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextAdd3(
                enginehandle: super::super::Foundation::HANDLE,
                providercontext: *const FWPM_PROVIDER_CONTEXT3_,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextAdd3(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(providercontext),
            ::std::mem::transmute(sd),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDeleteById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDeleteById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextDeleteById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT1,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum1(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT1,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextEnum1(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT2,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum2(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT2,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextEnum2(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextEnum3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT3_,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextEnum3(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT3_,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextEnum3(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetById1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById2(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetById2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetById3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetById3(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetById3(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey1(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT1,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetByKey1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey2(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT2,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetByKey2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetByKey3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetByKey3(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                providercontext: *mut *mut FWPM_PROVIDER_CONTEXT3_,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetByKey3(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(providercontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderContextSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextSubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_PROVIDER_CONTEXT_CHANGE_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    changehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                changehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextSubscribeChanges0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(changehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderContextUnsubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    changehandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderContextUnsubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                changehandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderContextUnsubscribeChanges0(
            enginehandle.into_param().abi(),
            changehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_PROVIDER_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_PROVIDER_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_PROVIDER0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_PROVIDER0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    provider: *mut *mut FWPM_PROVIDER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                provider: *mut *mut FWPM_PROVIDER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(provider),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmProviderSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderSubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_PROVIDER_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_PROVIDER_CHANGE_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    changehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_PROVIDER_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                changehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderSubscribeChanges0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(changehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_PROVIDER_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_PROVIDER_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmProviderUnsubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    changehandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmProviderUnsubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                changehandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmProviderUnsubscribeChanges0(
            enginehandle.into_param().abi(),
            changehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSessionCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_SESSION_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_SESSION_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSessionCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSessionDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSessionDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSessionEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_SESSION0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSessionEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_SESSION0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSessionEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerAdd0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    sublayer: *const FWPM_SUBLAYER0,
    sd: *const super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerAdd0(
                enginehandle: super::super::Foundation::HANDLE,
                sublayer: *const FWPM_SUBLAYER0,
                sd: *const super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerAdd0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(sublayer),
            ::std::mem::transmute(sd),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const FWPM_SUBLAYER_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const FWPM_SUBLAYER_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerDeleteByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerDeleteByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerDeleteByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut FWPM_SUBLAYER0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut FWPM_SUBLAYER0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerGetByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    sublayer: *mut *mut FWPM_SUBLAYER0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerGetByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                sublayer: *mut *mut FWPM_SUBLAYER0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerGetByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(sublayer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmSubLayerSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    key: *const ::windows::runtime::GUID,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                key: *const ::windows::runtime::GUID,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(key),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerSubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_SUBLAYER_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_SUBLAYER_CHANGE_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    changehandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_SUBLAYER_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                changehandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerSubscribeChanges0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(changehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut FWPM_SUBLAYER_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut FWPM_SUBLAYER_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSubLayerUnsubscribeChanges0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    changehandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSubLayerUnsubscribeChanges0(
                enginehandle: super::super::Foundation::HANDLE,
                changehandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSubLayerUnsubscribeChanges0(
            enginehandle.into_param().abi(),
            changehandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    sysports: *mut *mut FWPM_SYSTEM_PORTS0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                sysports: *mut *mut FWPM_SYSTEM_PORTS0,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSystemPortsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(sysports),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsSubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    reserved: *mut ::std::ffi::c_void,
    callback: ::std::option::Option<FWPM_SYSTEM_PORTS_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    sysportshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsSubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                reserved: *mut ::std::ffi::c_void,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                sysportshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSystemPortsSubscribe0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(sysportshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmSystemPortsUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    sysportshandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmSystemPortsUnsubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                sysportshandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmSystemPortsUnsubscribe0(
            enginehandle.into_param().abi(),
            sysportshandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionAbort0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionAbort0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(FwpmTransactionAbort0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionBegin0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionBegin0(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmTransactionBegin0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmTransactionCommit0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmTransactionCommit0(enginehandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(FwpmTransactionCommit0(enginehandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmvSwitchEventSubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const FWPM_VSWITCH_EVENT_SUBSCRIPTION0,
    callback: ::std::option::Option<FWPM_VSWITCH_EVENT_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    subscriptionhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventSubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const FWPM_VSWITCH_EVENT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                subscriptionhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmvSwitchEventSubscribe0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(subscriptionhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FwpmvSwitchEventUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscriptionhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventUnsubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                subscriptionhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmvSwitchEventUnsubscribe0(
            enginehandle.into_param().abi(),
            subscriptionhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventsGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmvSwitchEventsGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FwpmvSwitchEventsSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FwpmvSwitchEventsSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(FwpmvSwitchEventsSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ICMP4_TIME_EXCEED_CODE(pub i32);
pub const ICMP4_TIME_EXCEED_TRANSIT: ICMP4_TIME_EXCEED_CODE = ICMP4_TIME_EXCEED_CODE(0i32);
pub const ICMP4_TIME_EXCEED_REASSEMBLY: ICMP4_TIME_EXCEED_CODE = ICMP4_TIME_EXCEED_CODE(1i32);
impl ::std::convert::From<i32> for ICMP4_TIME_EXCEED_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ICMP4_TIME_EXCEED_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ICMP4_UNREACH_CODE(pub i32);
pub const ICMP4_UNREACH_NET: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(0i32);
pub const ICMP4_UNREACH_HOST: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(1i32);
pub const ICMP4_UNREACH_PROTOCOL: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(2i32);
pub const ICMP4_UNREACH_PORT: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(3i32);
pub const ICMP4_UNREACH_FRAG_NEEDED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(4i32);
pub const ICMP4_UNREACH_SOURCEROUTE_FAILED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(5i32);
pub const ICMP4_UNREACH_NET_UNKNOWN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(6i32);
pub const ICMP4_UNREACH_HOST_UNKNOWN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(7i32);
pub const ICMP4_UNREACH_ISOLATED: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(8i32);
pub const ICMP4_UNREACH_NET_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(9i32);
pub const ICMP4_UNREACH_HOST_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(10i32);
pub const ICMP4_UNREACH_NET_TOS: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(11i32);
pub const ICMP4_UNREACH_HOST_TOS: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(12i32);
pub const ICMP4_UNREACH_ADMIN: ICMP4_UNREACH_CODE = ICMP4_UNREACH_CODE(13i32);
impl ::std::convert::From<i32> for ICMP4_UNREACH_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ICMP4_UNREACH_CODE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICMP6_DST_UNREACH_ADDR: u32 = 3u32;
pub const ICMP6_DST_UNREACH_ADMIN: u32 = 1u32;
pub const ICMP6_DST_UNREACH_BEYONDSCOPE: u32 = 2u32;
pub const ICMP6_DST_UNREACH_NOPORT: u32 = 4u32;
pub const ICMP6_DST_UNREACH_NOROUTE: u32 = 0u32;
pub const ICMP6_PARAMPROB_HEADER: u32 = 0u32;
pub const ICMP6_PARAMPROB_NEXTHEADER: u32 = 1u32;
pub const ICMP6_PARAMPROB_OPTION: u32 = 2u32;
pub const ICMP6_TIME_EXCEED_REASSEMBLY: u32 = 1u32;
pub const ICMP6_TIME_EXCEED_TRANSIT: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMPV4_ADDRESS_MASK_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub AddressMask: u32,
}
impl ICMPV4_ADDRESS_MASK_MESSAGE {}
impl ::std::default::Default for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMPV4_ADDRESS_MASK_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMPV4_ADDRESS_MASK_MESSAGE {}
unsafe impl ::windows::runtime::Abi for ICMPV4_ADDRESS_MASK_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICMPV4_INVALID_PREFERENCE_LEVEL: u32 = 2147483648u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct ICMPV4_ROUTER_ADVERT_ENTRY {
    pub RouterAdvertAddr: super::super::Networking::WinSock::IN_ADDR,
    pub PreferenceLevel: i32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ICMPV4_ROUTER_ADVERT_ENTRY {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for ICMPV4_ROUTER_ADVERT_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for ICMPV4_ROUTER_ADVERT_ENTRY {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for ICMPV4_ROUTER_ADVERT_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMPV4_ROUTER_ADVERT_HEADER {
    pub RaHeader: ICMP_MESSAGE,
}
impl ICMPV4_ROUTER_ADVERT_HEADER {}
impl ::std::default::Default for ICMPV4_ROUTER_ADVERT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMPV4_ROUTER_ADVERT_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMPV4_ROUTER_ADVERT_HEADER {}
unsafe impl ::windows::runtime::Abi for ICMPV4_ROUTER_ADVERT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMPV4_ROUTER_SOLICIT {
    pub RsHeader: ICMP_MESSAGE,
}
impl ICMPV4_ROUTER_SOLICIT {}
impl ::std::default::Default for ICMPV4_ROUTER_SOLICIT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMPV4_ROUTER_SOLICIT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMPV4_ROUTER_SOLICIT {}
unsafe impl ::windows::runtime::Abi for ICMPV4_ROUTER_SOLICIT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMPV4_TIMESTAMP_MESSAGE {
    pub Header: ICMP_MESSAGE,
    pub OriginateTimestamp: u32,
    pub ReceiveTimestamp: u32,
    pub TransmitTimestamp: u32,
}
impl ICMPV4_TIMESTAMP_MESSAGE {}
impl ::std::default::Default for ICMPV4_TIMESTAMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMPV4_TIMESTAMP_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMPV4_TIMESTAMP_MESSAGE {}
unsafe impl ::windows::runtime::Abi for ICMPV4_TIMESTAMP_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ICMPV6_ECHO_REQUEST_FLAG_REVERSE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMP_HEADER {
    pub Type: u8,
    pub Code: u8,
    pub Checksum: u16,
}
impl ICMP_HEADER {}
impl ::std::default::Default for ICMP_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ICMP_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ICMP_HEADER")
            .field("Type", &self.Type)
            .field("Code", &self.Code)
            .field("Checksum", &self.Checksum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ICMP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Code == other.Code && self.Checksum == other.Checksum
    }
}
impl ::std::cmp::Eq for ICMP_HEADER {}
unsafe impl ::windows::runtime::Abi for ICMP_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ICMP_MESSAGE {
    pub Header: ICMP_HEADER,
    pub Data: ICMP_MESSAGE_0,
}
impl ICMP_MESSAGE {}
impl ::std::default::Default for ICMP_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMP_MESSAGE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMP_MESSAGE {}
unsafe impl ::windows::runtime::Abi for ICMP_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union ICMP_MESSAGE_0 {
    pub Data32: [u32; 1],
    pub Data16: [u16; 2],
    pub Data8: [u8; 4],
}
impl ICMP_MESSAGE_0 {}
impl ::std::default::Default for ICMP_MESSAGE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ICMP_MESSAGE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ICMP_MESSAGE_0 {}
unsafe impl ::windows::runtime::Abi for ICMP_MESSAGE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IGMPV3_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for IGMPV3_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for IGMPV3_QUERY_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for IGMPV3_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for IGMPV3_QUERY_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IGMPV3_QUERY_HEADER_0 {
    pub MaxRespCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_0_0,
}
impl IGMPV3_QUERY_HEADER_0 {}
impl ::std::default::Default for IGMPV3_QUERY_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IGMPV3_QUERY_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IGMPV3_QUERY_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IGMPV3_QUERY_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IGMPV3_QUERY_HEADER_0_0 {
    pub _bitfield: u8,
}
impl IGMPV3_QUERY_HEADER_0_0 {}
impl ::std::default::Default for IGMPV3_QUERY_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IGMPV3_QUERY_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IGMPV3_QUERY_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IGMPV3_QUERY_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IGMPV3_QUERY_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IGMPV3_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: IGMPV3_QUERY_HEADER_1_0,
}
impl IGMPV3_QUERY_HEADER_1 {}
impl ::std::default::Default for IGMPV3_QUERY_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IGMPV3_QUERY_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IGMPV3_QUERY_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for IGMPV3_QUERY_HEADER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IGMPV3_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
impl IGMPV3_QUERY_HEADER_1_0 {}
impl ::std::default::Default for IGMPV3_QUERY_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IGMPV3_QUERY_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IGMPV3_QUERY_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IGMPV3_QUERY_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for IGMPV3_QUERY_HEADER_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IGMPV3_REPORT_HEADER {
    pub Type: u8,
    pub Reserved: u8,
    pub Checksum: u16,
    pub Reserved2: u16,
    pub RecordCount: u16,
}
impl IGMPV3_REPORT_HEADER {}
impl ::std::default::Default for IGMPV3_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IGMPV3_REPORT_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IGMPV3_REPORT_HEADER")
            .field("Type", &self.Type)
            .field("Reserved", &self.Reserved)
            .field("Checksum", &self.Checksum)
            .field("Reserved2", &self.Reserved2)
            .field("RecordCount", &self.RecordCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IGMPV3_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type
            && self.Reserved == other.Reserved
            && self.Checksum == other.Checksum
            && self.Reserved2 == other.Reserved2
            && self.RecordCount == other.RecordCount
    }
}
impl ::std::cmp::Eq for IGMPV3_REPORT_HEADER {}
unsafe impl ::windows::runtime::Abi for IGMPV3_REPORT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMPV3_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl IGMPV3_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for IGMPV3_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for IGMPV3_REPORT_RECORD_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for IGMPV3_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for IGMPV3_REPORT_RECORD_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct IGMP_HEADER {
    pub Anonymous1: IGMP_HEADER_0,
    pub Anonymous2: IGMP_HEADER_1,
    pub Checksum: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl IGMP_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for IGMP_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for IGMP_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for IGMP_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for IGMP_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IGMP_HEADER_0 {
    pub Anonymous: IGMP_HEADER_0_0,
    pub VersionType: u8,
}
impl IGMP_HEADER_0 {}
impl ::std::default::Default for IGMP_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IGMP_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IGMP_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IGMP_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IGMP_HEADER_0_0 {
    pub _bitfield: u8,
}
impl IGMP_HEADER_0_0 {}
impl ::std::default::Default for IGMP_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IGMP_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IGMP_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IGMP_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IGMP_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IGMP_HEADER_1 {
    pub Reserved: u8,
    pub MaxRespTime: u8,
    pub Code: u8,
}
impl IGMP_HEADER_1 {}
impl ::std::default::Default for IGMP_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IGMP_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IGMP_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for IGMP_HEADER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IGMP_LEAVE_GROUP_TYPE: u32 = 23u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IGMP_MAX_RESP_CODE_TYPE(pub i32);
pub const IGMP_MAX_RESP_CODE_TYPE_NORMAL: IGMP_MAX_RESP_CODE_TYPE = IGMP_MAX_RESP_CODE_TYPE(0i32);
pub const IGMP_MAX_RESP_CODE_TYPE_FLOAT: IGMP_MAX_RESP_CODE_TYPE = IGMP_MAX_RESP_CODE_TYPE(1i32);
impl ::std::convert::From<i32> for IGMP_MAX_RESP_CODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IGMP_MAX_RESP_CODE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IGMP_QUERY_TYPE: u32 = 17u32;
pub const IGMP_VERSION1_REPORT_TYPE: u32 = 18u32;
pub const IGMP_VERSION2_REPORT_TYPE: u32 = 22u32;
pub const IGMP_VERSION3_REPORT_TYPE: u32 = 34u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(pub i32);
pub const IKEEXT_IMPERSONATION_NONE: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE =
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(0i32);
pub const IKEEXT_IMPERSONATION_SOCKET_PRINCIPAL: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE =
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(1i32);
pub const IKEEXT_IMPERSONATION_MAX: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE =
    IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE(2i32);
impl ::std::convert::From<i32> for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_AUTHENTICATION_METHOD0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_AUTHENTICATION_METHOD0_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION0,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_AUTHENTICATION_METHOD1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_AUTHENTICATION_METHOD1_0 {
    pub presharedKeyAuthentication: IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificateAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub kerberosAuthentication: IKEEXT_KERBEROS_AUTHENTICATION0,
    pub ntlmV2Authentication: IKEEXT_NTLM_V2_AUTHENTICATION0,
    pub sslAuthentication: IKEEXT_CERTIFICATE_AUTHENTICATION1,
    pub cgaAuthentication: IKEEXT_IPV6_CGA_AUTHENTICATION0,
    pub eapAuthentication: IKEEXT_EAP_AUTHENTICATION0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_AUTHENTICATION_METHOD2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub Anonymous: IKEEXT_AUTHENTICATION_METHOD2_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_AUTHENTICATION_METHOD2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_AUTHENTICATION_METHOD2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_AUTHENTICATION_METHOD2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_AUTHENTICATION_METHOD_TYPE(pub i32);
pub const IKEEXT_PRESHARED_KEY: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(0i32);
pub const IKEEXT_CERTIFICATE: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(1i32);
pub const IKEEXT_KERBEROS: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(2i32);
pub const IKEEXT_ANONYMOUS: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(3i32);
pub const IKEEXT_SSL: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(4i32);
pub const IKEEXT_NTLM_V2: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(5i32);
pub const IKEEXT_IPV6_CGA: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(6i32);
pub const IKEEXT_CERTIFICATE_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(7i32);
pub const IKEEXT_CERTIFICATE_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(8i32);
pub const IKEEXT_SSL_ECDSA_P256: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(9i32);
pub const IKEEXT_SSL_ECDSA_P384: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(10i32);
pub const IKEEXT_EAP: IKEEXT_AUTHENTICATION_METHOD_TYPE = IKEEXT_AUTHENTICATION_METHOD_TYPE(11i32);
pub const IKEEXT_RESERVED: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(12i32);
pub const IKEEXT_AUTHENTICATION_METHOD_TYPE_MAX: IKEEXT_AUTHENTICATION_METHOD_TYPE =
    IKEEXT_AUTHENTICATION_METHOD_TYPE(13i32);
impl ::std::convert::From<i32> for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION0_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION0_1,
    pub flags: IKEEXT_CERT_AUTH,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("inboundRootArraySize", &self.inboundRootArraySize)
            .field("inboundRootArray", &self.inboundRootArray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.inboundRootArraySize == other.inboundRootArraySize
            && self.inboundRootArray == other.inboundRootArray
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("outboundRootArraySize", &self.outboundRootArraySize)
            .field("outboundRootArray", &self.outboundRootArray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.outboundRootArraySize == other.outboundRootArraySize
            && self.outboundRootArray == other.outboundRootArray
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION0_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION1_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION1_1,
    pub flags: IKEEXT_CERT_AUTH,
    pub localCertLocationUrl: FWP_BYTE_BLOB,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION1 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0,
    pub inboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub inboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("inboundRootArraySize", &self.inboundRootArraySize)
            .field("inboundRootArray", &self.inboundRootArray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.inboundRootArraySize == other.inboundRootArraySize
            && self.inboundRootArray == other.inboundRootArray
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    pub Anonymous: IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0,
    pub outboundEnterpriseStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
    pub outboundTrustedRootStoreConfig: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootArray: *mut IKEEXT_CERT_ROOT_CONFIG0,
}
impl IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("outboundRootArraySize", &self.outboundRootArraySize)
            .field("outboundRootArray", &self.outboundRootArray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.outboundRootArraySize == other.outboundRootArraySize
            && self.outboundRootArray == other.outboundRootArray
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION1_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    pub inboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_0,
    pub outboundConfigType: IKEEXT_CERT_CONFIG_TYPE,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1,
    pub flags: IKEEXT_CERT_AUTH,
    pub localCertLocationUrl: FWP_BYTE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    pub inboundRootArraySize: u32,
    pub inboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("inboundRootArraySize", &self.inboundRootArraySize)
            .field("inboundRootCriteria", &self.inboundRootCriteria)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.inboundRootArraySize == other.inboundRootArraySize
            && self.inboundRootCriteria == other.inboundRootCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    pub inboundEnterpriseStoreArraySize: u32,
    pub inboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field(
                "inboundEnterpriseStoreArraySize",
                &self.inboundEnterpriseStoreArraySize,
            )
            .field(
                "inboundEnterpriseStoreCriteria",
                &self.inboundEnterpriseStoreCriteria,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.inboundEnterpriseStoreArraySize == other.inboundEnterpriseStoreArraySize
            && self.inboundEnterpriseStoreCriteria == other.inboundEnterpriseStoreCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    pub inboundRootStoreArraySize: u32,
    pub inboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous3_e__Struct")
            .field("inboundRootStoreArraySize", &self.inboundRootStoreArraySize)
            .field(
                "inboundTrustedRootStoreCriteria",
                &self.inboundTrustedRootStoreCriteria,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.inboundRootStoreArraySize == other.inboundRootStoreArraySize
            && self.inboundTrustedRootStoreCriteria == other.inboundTrustedRootStoreCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    pub Anonymous1: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0,
    pub Anonymous2: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1,
    pub Anonymous3: IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    pub outboundRootArraySize: u32,
    pub outboundRootCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous1_e__Struct")
            .field("outboundRootArraySize", &self.outboundRootArraySize)
            .field("outboundRootCriteria", &self.outboundRootCriteria)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.outboundRootArraySize == other.outboundRootArraySize
            && self.outboundRootCriteria == other.outboundRootCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    pub outboundEnterpriseStoreArraySize: u32,
    pub outboundEnterpriseStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous2_e__Struct")
            .field(
                "outboundEnterpriseStoreArraySize",
                &self.outboundEnterpriseStoreArraySize,
            )
            .field(
                "outboundEnterpriseStoreCriteria",
                &self.outboundEnterpriseStoreCriteria,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    fn eq(&self, other: &Self) -> bool {
        self.outboundEnterpriseStoreArraySize == other.outboundEnterpriseStoreArraySize
            && self.outboundEnterpriseStoreCriteria == other.outboundEnterpriseStoreCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    pub outboundRootStoreArraySize: u32,
    pub outboundTrustedRootStoreCriteria: *mut IKEEXT_CERTIFICATE_CRITERIA0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous3_e__Struct")
            .field(
                "outboundRootStoreArraySize",
                &self.outboundRootStoreArraySize,
            )
            .field(
                "outboundTrustedRootStoreCriteria",
                &self.outboundTrustedRootStoreCriteria,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.outboundRootStoreArraySize == other.outboundRootStoreArraySize
            && self.outboundTrustedRootStoreCriteria == other.outboundTrustedRootStoreCriteria
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_AUTHENTICATION2_1_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL0 {
    pub subjectName: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub flags: u32,
}
impl IKEEXT_CERTIFICATE_CREDENTIAL0 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL0")
            .field("subjectName", &self.subjectName)
            .field("certHash", &self.certHash)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.subjectName == other.subjectName
            && self.certHash == other.certHash
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERTIFICATE_CREDENTIAL1 {
    pub subjectName: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub flags: u32,
    pub certificate: FWP_BYTE_BLOB,
}
impl IKEEXT_CERTIFICATE_CREDENTIAL1 {}
impl ::std::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL1")
            .field("subjectName", &self.subjectName)
            .field("certHash", &self.certHash)
            .field("flags", &self.flags)
            .field("certificate", &self.certificate)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn eq(&self, other: &Self) -> bool {
        self.subjectName == other.subjectName
            && self.certHash == other.certHash
            && self.flags == other.flags
            && self.certificate == other.certificate
    }
}
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERTIFICATE_CRITERIA0 {
    pub certData: FWP_BYTE_BLOB,
    pub certHash: FWP_BYTE_BLOB,
    pub eku: *mut IKEEXT_CERT_EKUS0,
    pub name: *mut IKEEXT_CERT_NAME0,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERTIFICATE_CRITERIA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERTIFICATE_CRITERIA0")
            .field("certData", &self.certData)
            .field("certHash", &self.certHash)
            .field("eku", &self.eku)
            .field("name", &self.name)
            .field("flags", &self.flags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn eq(&self, other: &Self) -> bool {
        self.certData == other.certData
            && self.certHash == other.certHash
            && self.eku == other.eku
            && self.name == other.name
            && self.flags == other.flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERTIFICATE_CRITERIA0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERTIFICATE_CRITERIA0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_CERT_AUTH(pub u32);
pub const IKEEXT_CERT_AUTH_FLAG_SSL_ONE_WAY: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(1u32);
pub const IKEEXT_CERT_AUTH_ENABLE_CRL_CHECK_STRONG: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(4u32);
pub const IKEEXT_CERT_AUTH_DISABLE_SSL_CERT_VALIDATION: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(8u32);
pub const IKEEXT_CERT_AUTH_ALLOW_HTTP_CERT_LOOKUP: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(16u32);
pub const IKEEXT_CERT_AUTH_URL_CONTAINS_BUNDLE: IKEEXT_CERT_AUTH = IKEEXT_CERT_AUTH(32u32);
impl ::std::convert::From<u32> for IKEEXT_CERT_AUTH {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_AUTH {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_CERT_AUTH {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_CERT_AUTH {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_CRL_CHECK: u32 = 2u32;
pub const IKEEXT_CERT_AUTH_FLAG_DISABLE_REQUEST_PAYLOAD: u32 = 64u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_CERT_CONFIG_TYPE(pub i32);
pub const IKEEXT_CERT_CONFIG_EXPLICIT_TRUST_LIST: IKEEXT_CERT_CONFIG_TYPE =
    IKEEXT_CERT_CONFIG_TYPE(0i32);
pub const IKEEXT_CERT_CONFIG_ENTERPRISE_STORE: IKEEXT_CERT_CONFIG_TYPE =
    IKEEXT_CERT_CONFIG_TYPE(1i32);
pub const IKEEXT_CERT_CONFIG_TRUSTED_ROOT_STORE: IKEEXT_CERT_CONFIG_TYPE =
    IKEEXT_CERT_CONFIG_TYPE(2i32);
pub const IKEEXT_CERT_CONFIG_UNSPECIFIED: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(3i32);
pub const IKEEXT_CERT_CONFIG_TYPE_MAX: IKEEXT_CERT_CONFIG_TYPE = IKEEXT_CERT_CONFIG_TYPE(4i32);
impl ::std::convert::From<i32> for IKEEXT_CERT_CONFIG_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_CONFIG_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IKEEXT_CERT_CREDENTIAL_FLAG_NAP_CERT: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_CERT_CRITERIA_NAME_TYPE(pub i32);
pub const IKEEXT_CERT_CRITERIA_DNS: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(0i32);
pub const IKEEXT_CERT_CRITERIA_UPN: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(1i32);
pub const IKEEXT_CERT_CRITERIA_RFC822: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(2i32);
pub const IKEEXT_CERT_CRITERIA_CN: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(3i32);
pub const IKEEXT_CERT_CRITERIA_OU: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(4i32);
pub const IKEEXT_CERT_CRITERIA_O: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(5i32);
pub const IKEEXT_CERT_CRITERIA_DC: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(6i32);
pub const IKEEXT_CERT_CRITERIA_NAME_TYPE_MAX: IKEEXT_CERT_CRITERIA_NAME_TYPE =
    IKEEXT_CERT_CRITERIA_NAME_TYPE(7i32);
impl ::std::convert::From<i32> for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERT_EKUS0 {
    pub numEku: u32,
    pub eku: *mut super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERT_EKUS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERT_EKUS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERT_EKUS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERT_EKUS0")
            .field("numEku", &self.numEku)
            .field("eku", &self.eku)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERT_EKUS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numEku == other.numEku && self.eku == other.eku
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERT_EKUS0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_EKUS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_CERT_FLAGS(pub u32);
pub const IKEEXT_CERT_FLAG_ENABLE_ACCOUNT_MAPPING: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(1u32);
pub const IKEEXT_CERT_FLAG_DISABLE_REQUEST_PAYLOAD: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(2u32);
pub const IKEEXT_CERT_FLAG_USE_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(4u32);
pub const IKEEXT_CERT_FLAG_INTERMEDIATE_CA: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(8u32);
pub const IKEEXT_CERT_FLAG_IGNORE_INIT_CERT_MAP_FAILURE: IKEEXT_CERT_FLAGS =
    IKEEXT_CERT_FLAGS(16u32);
pub const IKEEXT_CERT_FLAG_PREFER_NAP_CERTIFICATE_OUTBOUND: IKEEXT_CERT_FLAGS =
    IKEEXT_CERT_FLAGS(32u32);
pub const IKEEXT_CERT_FLAG_SELECT_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(64u32);
pub const IKEEXT_CERT_FLAG_VERIFY_NAP_CERTIFICATE: IKEEXT_CERT_FLAGS = IKEEXT_CERT_FLAGS(128u32);
pub const IKEEXT_CERT_FLAG_FOLLOW_RENEWAL_CERTIFICATE: IKEEXT_CERT_FLAGS =
    IKEEXT_CERT_FLAGS(256u32);
impl ::std::convert::From<u32> for IKEEXT_CERT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_CERT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_CERT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IKEEXT_CERT_HASH_LEN: u32 = 20u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CERT_NAME0 {
    pub nameType: IKEEXT_CERT_CRITERIA_NAME_TYPE,
    pub certName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CERT_NAME0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CERT_NAME0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CERT_NAME0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERT_NAME0")
            .field("nameType", &self.nameType)
            .field("certName", &self.certName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CERT_NAME0 {
    fn eq(&self, other: &Self) -> bool {
        self.nameType == other.nameType && self.certName == other.certName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CERT_NAME0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_NAME0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CERT_ROOT_CONFIG0 {
    pub certData: FWP_BYTE_BLOB,
    pub flags: IKEEXT_CERT_FLAGS,
}
impl IKEEXT_CERT_ROOT_CONFIG0 {}
impl ::std::default::Default for IKEEXT_CERT_ROOT_CONFIG0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CERT_ROOT_CONFIG0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CERT_ROOT_CONFIG0")
            .field("certData", &self.certData)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CERT_ROOT_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        self.certData == other.certData && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_CERT_ROOT_CONFIG0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CERT_ROOT_CONFIG0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_CIPHER_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_CIPHER_TYPE,
    pub keyLen: u32,
    pub rounds: u32,
}
impl IKEEXT_CIPHER_ALGORITHM0 {}
impl ::std::default::Default for IKEEXT_CIPHER_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_CIPHER_ALGORITHM0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CIPHER_ALGORITHM0")
            .field("algoIdentifier", &self.algoIdentifier)
            .field("keyLen", &self.keyLen)
            .field("rounds", &self.rounds)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_CIPHER_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier
            && self.keyLen == other.keyLen
            && self.rounds == other.rounds
    }
}
impl ::std::cmp::Eq for IKEEXT_CIPHER_ALGORITHM0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_CIPHER_ALGORITHM0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_CIPHER_TYPE(pub i32);
pub const IKEEXT_CIPHER_DES: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(0i32);
pub const IKEEXT_CIPHER_3DES: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(1i32);
pub const IKEEXT_CIPHER_AES_128: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(2i32);
pub const IKEEXT_CIPHER_AES_192: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(3i32);
pub const IKEEXT_CIPHER_AES_256: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(4i32);
pub const IKEEXT_CIPHER_AES_GCM_128_16ICV: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(5i32);
pub const IKEEXT_CIPHER_AES_GCM_256_16ICV: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(6i32);
pub const IKEEXT_CIPHER_TYPE_MAX: IKEEXT_CIPHER_TYPE = IKEEXT_CIPHER_TYPE(7i32);
impl ::std::convert::From<i32> for IKEEXT_CIPHER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_CIPHER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_COMMON_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
impl IKEEXT_COMMON_STATISTICS0 {}
impl ::std::default::Default for IKEEXT_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_COMMON_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_COMMON_STATISTICS0")
            .field("v4Statistics", &self.v4Statistics)
            .field("v6Statistics", &self.v6Statistics)
            .field("totalPacketsReceived", &self.totalPacketsReceived)
            .field(
                "totalInvalidPacketsReceived",
                &self.totalInvalidPacketsReceived,
            )
            .field("currentQueuedWorkitems", &self.currentQueuedWorkitems)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics
            && self.v6Statistics == other.v6Statistics
            && self.totalPacketsReceived == other.totalPacketsReceived
            && self.totalInvalidPacketsReceived == other.totalInvalidPacketsReceived
            && self.currentQueuedWorkitems == other.currentQueuedWorkitems
    }
}
impl ::std::cmp::Eq for IKEEXT_COMMON_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_COMMON_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_COMMON_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1,
    pub totalPacketsReceived: u32,
    pub totalInvalidPacketsReceived: u32,
    pub currentQueuedWorkitems: u32,
}
impl IKEEXT_COMMON_STATISTICS1 {}
impl ::std::default::Default for IKEEXT_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_COMMON_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_COMMON_STATISTICS1")
            .field("v4Statistics", &self.v4Statistics)
            .field("v6Statistics", &self.v6Statistics)
            .field("totalPacketsReceived", &self.totalPacketsReceived)
            .field(
                "totalInvalidPacketsReceived",
                &self.totalInvalidPacketsReceived,
            )
            .field("currentQueuedWorkitems", &self.currentQueuedWorkitems)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics
            && self.v6Statistics == other.v6Statistics
            && self.totalPacketsReceived == other.totalPacketsReceived
            && self.totalInvalidPacketsReceived == other.totalInvalidPacketsReceived
            && self.currentQueuedWorkitems == other.currentQueuedWorkitems
    }
}
impl ::std::cmp::Eq for IKEEXT_COMMON_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_COMMON_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_COOKIE_PAIR0 {
    pub initiator: u64,
    pub responder: u64,
}
impl IKEEXT_COOKIE_PAIR0 {}
impl ::std::default::Default for IKEEXT_COOKIE_PAIR0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_COOKIE_PAIR0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_COOKIE_PAIR0")
            .field("initiator", &self.initiator)
            .field("responder", &self.responder)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_COOKIE_PAIR0 {
    fn eq(&self, other: &Self) -> bool {
        self.initiator == other.initiator && self.responder == other.responder
    }
}
impl ::std::cmp::Eq for IKEEXT_COOKIE_PAIR0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_COOKIE_PAIR0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL0 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_CREDENTIAL0_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION0,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL0,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL1 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL1_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_CREDENTIAL1_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL1_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL2 {
    pub authenticationMethodType: IKEEXT_AUTHENTICATION_METHOD_TYPE,
    pub impersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
    pub Anonymous: IKEEXT_CREDENTIAL2_0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IKEEXT_CREDENTIAL2_0 {
    pub presharedKey: *mut IKEEXT_PRESHARED_KEY_AUTHENTICATION1,
    pub certificate: *mut IKEEXT_CERTIFICATE_CREDENTIAL1,
    pub name: *mut IKEEXT_NAME_CREDENTIAL0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL2_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL2_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIALS0 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIALS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIALS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CREDENTIALS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CREDENTIALS0")
            .field("numCredentials", &self.numCredentials)
            .field("credentials", &self.credentials)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIALS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIALS0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIALS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIALS1 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR1,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIALS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIALS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CREDENTIALS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CREDENTIALS1")
            .field("numCredentials", &self.numCredentials)
            .field("credentials", &self.credentials)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIALS1 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIALS1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIALS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIALS2 {
    pub numCredentials: u32,
    pub credentials: *mut IKEEXT_CREDENTIAL_PAIR2,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIALS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIALS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_CREDENTIALS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_CREDENTIALS2")
            .field("numCredentials", &self.numCredentials)
            .field("credentials", &self.credentials)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIALS2 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIALS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIALS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL_PAIR0 {
    pub localCredentials: IKEEXT_CREDENTIAL0,
    pub peerCredentials: IKEEXT_CREDENTIAL0,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL_PAIR0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL_PAIR0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL_PAIR0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL_PAIR0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL_PAIR1 {
    pub localCredentials: IKEEXT_CREDENTIAL1,
    pub peerCredentials: IKEEXT_CREDENTIAL1,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL_PAIR1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL_PAIR1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL_PAIR1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL_PAIR1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_CREDENTIAL_PAIR2 {
    pub localCredentials: IKEEXT_CREDENTIAL2,
    pub peerCredentials: IKEEXT_CREDENTIAL2,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_CREDENTIAL_PAIR2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_CREDENTIAL_PAIR2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_CREDENTIAL_PAIR2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_CREDENTIAL_PAIR2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_CREDENTIAL_PAIR2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_DH_GROUP(pub i32);
pub const IKEEXT_DH_GROUP_NONE: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(0i32);
pub const IKEEXT_DH_GROUP_1: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(1i32);
pub const IKEEXT_DH_GROUP_2: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(2i32);
pub const IKEEXT_DH_GROUP_14: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(3i32);
pub const IKEEXT_DH_GROUP_2048: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(3i32);
pub const IKEEXT_DH_ECP_256: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(4i32);
pub const IKEEXT_DH_ECP_384: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(5i32);
pub const IKEEXT_DH_GROUP_24: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(6i32);
pub const IKEEXT_DH_GROUP_MAX: IKEEXT_DH_GROUP = IKEEXT_DH_GROUP(7i32);
impl ::std::convert::From<i32> for IKEEXT_DH_GROUP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_DH_GROUP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_EAP_AUTHENTICATION0 {
    pub flags: IKEEXT_EAP_AUTHENTICATION_FLAGS,
}
impl IKEEXT_EAP_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_EAP_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_EAP_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_EAP_AUTHENTICATION0")
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_EAP_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_EAP_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_EAP_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_EAP_AUTHENTICATION_FLAGS(pub u32);
pub const IKEEXT_EAP_FLAG_LOCAL_AUTH_ONLY: IKEEXT_EAP_AUTHENTICATION_FLAGS =
    IKEEXT_EAP_AUTHENTICATION_FLAGS(1u32);
pub const IKEEXT_EAP_FLAG_REMOTE_AUTH_ONLY: IKEEXT_EAP_AUTHENTICATION_FLAGS =
    IKEEXT_EAP_AUTHENTICATION_FLAGS(2u32);
impl ::std::convert::From<u32> for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_EM_POLICY0 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD0,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_EM_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_EM_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_EM_POLICY0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_EM_POLICY0")
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_EM_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_EM_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_EM_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_EM_POLICY1 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD1,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_EM_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_EM_POLICY1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_EM_POLICY1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_EM_POLICY1")
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_EM_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_EM_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_EM_POLICY1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_EM_POLICY2 {
    pub numAuthenticationMethods: u32,
    pub authenticationMethods: *mut IKEEXT_AUTHENTICATION_METHOD2,
    pub initiatorImpersonationType: IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_EM_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_EM_POLICY2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_EM_POLICY2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_EM_POLICY2")
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_EM_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_EM_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_EM_POLICY2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_EM_SA_STATE(pub i32);
pub const IKEEXT_EM_SA_STATE_NONE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(0i32);
pub const IKEEXT_EM_SA_STATE_SENT_ATTS: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(1i32);
pub const IKEEXT_EM_SA_STATE_SSPI_SENT: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(2i32);
pub const IKEEXT_EM_SA_STATE_AUTH_COMPLETE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(3i32);
pub const IKEEXT_EM_SA_STATE_FINAL: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(4i32);
pub const IKEEXT_EM_SA_STATE_COMPLETE: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(5i32);
pub const IKEEXT_EM_SA_STATE_MAX: IKEEXT_EM_SA_STATE = IKEEXT_EM_SA_STATE(6i32);
impl ::std::convert::From<i32> for IKEEXT_EM_SA_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_EM_SA_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_INTEGRITY_ALGORITHM0 {
    pub algoIdentifier: IKEEXT_INTEGRITY_TYPE,
}
impl IKEEXT_INTEGRITY_ALGORITHM0 {}
impl ::std::default::Default for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_INTEGRITY_ALGORITHM0")
            .field("algoIdentifier", &self.algoIdentifier)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier
    }
}
impl ::std::cmp::Eq for IKEEXT_INTEGRITY_ALGORITHM0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_INTEGRITY_ALGORITHM0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_INTEGRITY_TYPE(pub i32);
pub const IKEEXT_INTEGRITY_MD5: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(0i32);
pub const IKEEXT_INTEGRITY_SHA1: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(1i32);
pub const IKEEXT_INTEGRITY_SHA_256: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(2i32);
pub const IKEEXT_INTEGRITY_SHA_384: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(3i32);
pub const IKEEXT_INTEGRITY_TYPE_MAX: IKEEXT_INTEGRITY_TYPE = IKEEXT_INTEGRITY_TYPE(4i32);
impl ::std::convert::From<i32> for IKEEXT_INTEGRITY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_INTEGRITY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    pub keyContainerName: super::super::Foundation::PWSTR,
    pub cspName: super::super::Foundation::PWSTR,
    pub cspType: u32,
    pub cgaModifier: FWP_BYTE_ARRAY16,
    pub cgaCollisionCount: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_IPV6_CGA_AUTHENTICATION0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_IPV6_CGA_AUTHENTICATION0")
            .field("keyContainerName", &self.keyContainerName)
            .field("cspName", &self.cspName)
            .field("cspType", &self.cspType)
            .field("cgaModifier", &self.cgaModifier)
            .field("cgaCollisionCount", &self.cgaCollisionCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyContainerName == other.keyContainerName
            && self.cspName == other.cspName
            && self.cspType == other.cspType
            && self.cgaModifier == other.cgaModifier
            && self.cgaCollisionCount == other.cgaCollisionCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
impl IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {}
impl ::std::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0")
            .field(
                "totalSocketReceiveFailures",
                &self.totalSocketReceiveFailures,
            )
            .field("totalSocketSendFailures", &self.totalSocketSendFailures)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.totalSocketReceiveFailures == other.totalSocketReceiveFailures
            && self.totalSocketSendFailures == other.totalSocketSendFailures
    }
}
impl ::std::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    pub totalSocketReceiveFailures: u32,
    pub totalSocketSendFailures: u32,
}
impl IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {}
impl ::std::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1")
            .field(
                "totalSocketReceiveFailures",
                &self.totalSocketReceiveFailures,
            )
            .field("totalSocketSendFailures", &self.totalSocketSendFailures)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.totalSocketReceiveFailures == other.totalSocketReceiveFailures
            && self.totalSocketSendFailures == other.totalSocketSendFailures
    }
}
impl ::std::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {}
impl ::std::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field(
                "currentNewResponderMainModes",
                &self.currentNewResponderMainModes,
            )
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field(
                "currentActiveExtendedModes",
                &self.currentActiveExtendedModes,
            )
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field(
                "totalSuccessfulExtendedModes",
                &self.totalSuccessfulExtendedModes,
            )
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field(
                "totalImpersonationExtendedModes",
                &self.totalImpersonationExtendedModes,
            )
            .field(
                "totalImpersonationMainModes",
                &self.totalImpersonationMainModes,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.currentActiveMainModes == other.currentActiveMainModes
            && self.totalMainModesStarted == other.totalMainModesStarted
            && self.totalSuccessfulMainModes == other.totalSuccessfulMainModes
            && self.totalFailedMainModes == other.totalFailedMainModes
            && self.totalResponderMainModes == other.totalResponderMainModes
            && self.currentNewResponderMainModes == other.currentNewResponderMainModes
            && self.currentActiveQuickModes == other.currentActiveQuickModes
            && self.totalQuickModesStarted == other.totalQuickModesStarted
            && self.totalSuccessfulQuickModes == other.totalSuccessfulQuickModes
            && self.totalFailedQuickModes == other.totalFailedQuickModes
            && self.totalAcquires == other.totalAcquires
            && self.totalReinitAcquires == other.totalReinitAcquires
            && self.currentActiveExtendedModes == other.currentActiveExtendedModes
            && self.totalExtendedModesStarted == other.totalExtendedModesStarted
            && self.totalSuccessfulExtendedModes == other.totalSuccessfulExtendedModes
            && self.totalFailedExtendedModes == other.totalFailedExtendedModes
            && self.totalImpersonationExtendedModes == other.totalImpersonationExtendedModes
            && self.totalImpersonationMainModes == other.totalImpersonationMainModes
    }
}
impl ::std::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {}
impl ::std::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field(
                "currentNewResponderMainModes",
                &self.currentNewResponderMainModes,
            )
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field(
                "currentActiveExtendedModes",
                &self.currentActiveExtendedModes,
            )
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field(
                "totalSuccessfulExtendedModes",
                &self.totalSuccessfulExtendedModes,
            )
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field(
                "totalImpersonationExtendedModes",
                &self.totalImpersonationExtendedModes,
            )
            .field(
                "totalImpersonationMainModes",
                &self.totalImpersonationMainModes,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.currentActiveMainModes == other.currentActiveMainModes
            && self.totalMainModesStarted == other.totalMainModesStarted
            && self.totalSuccessfulMainModes == other.totalSuccessfulMainModes
            && self.totalFailedMainModes == other.totalFailedMainModes
            && self.totalResponderMainModes == other.totalResponderMainModes
            && self.currentNewResponderMainModes == other.currentNewResponderMainModes
            && self.currentActiveQuickModes == other.currentActiveQuickModes
            && self.totalQuickModesStarted == other.totalQuickModesStarted
            && self.totalSuccessfulQuickModes == other.totalSuccessfulQuickModes
            && self.totalFailedQuickModes == other.totalFailedQuickModes
            && self.totalAcquires == other.totalAcquires
            && self.totalReinitAcquires == other.totalReinitAcquires
            && self.currentActiveExtendedModes == other.currentActiveExtendedModes
            && self.totalExtendedModesStarted == other.totalExtendedModesStarted
            && self.totalSuccessfulExtendedModes == other.totalSuccessfulExtendedModes
            && self.totalFailedExtendedModes == other.totalFailedExtendedModes
            && self.totalImpersonationExtendedModes == other.totalImpersonationExtendedModes
            && self.totalImpersonationMainModes == other.totalImpersonationMainModes
    }
}
impl ::std::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION0 {
    pub flags: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS,
}
impl IKEEXT_KERBEROS_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION0")
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_KERBEROS_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_KERBEROS_AUTHENTICATION1 {
    pub flags: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS,
    pub proxyServer: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_KERBEROS_AUTHENTICATION1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION1")
            .field("flags", &self.flags)
            .field("proxyServer", &self.proxyServer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.proxyServer == other.proxyServer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_KERBEROS_AUTHENTICATION1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(pub u32);
pub const IKEEXT_KERB_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION:
    IKEEXT_KERBEROS_AUTHENTICATION_FLAGS = IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(1u32);
pub const IKEEXT_KERB_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: IKEEXT_KERBEROS_AUTHENTICATION_FLAGS =
    IKEEXT_KERBEROS_AUTHENTICATION_FLAGS(2u32);
impl ::std::convert::From<u32> for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IKEEXT_KERB_AUTH_FORCE_PROXY_ON_INITIATOR: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_KEYMODULE_STATISTICS0 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
impl IKEEXT_KEYMODULE_STATISTICS0 {}
impl ::std::default::Default for IKEEXT_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_KEYMODULE_STATISTICS0")
            .field("v4Statistics", &self.v4Statistics)
            .field("v6Statistics", &self.v6Statistics)
            .field("errorFrequencyTable", &self.errorFrequencyTable)
            .field("mainModeNegotiationTime", &self.mainModeNegotiationTime)
            .field("quickModeNegotiationTime", &self.quickModeNegotiationTime)
            .field(
                "extendedModeNegotiationTime",
                &self.extendedModeNegotiationTime,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics
            && self.v6Statistics == other.v6Statistics
            && self.errorFrequencyTable == other.errorFrequencyTable
            && self.mainModeNegotiationTime == other.mainModeNegotiationTime
            && self.quickModeNegotiationTime == other.quickModeNegotiationTime
            && self.extendedModeNegotiationTime == other.extendedModeNegotiationTime
    }
}
impl ::std::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_KEYMODULE_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_KEYMODULE_STATISTICS1 {
    pub v4Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub v6Statistics: IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1,
    pub errorFrequencyTable: [u32; 97],
    pub mainModeNegotiationTime: u32,
    pub quickModeNegotiationTime: u32,
    pub extendedModeNegotiationTime: u32,
}
impl IKEEXT_KEYMODULE_STATISTICS1 {}
impl ::std::default::Default for IKEEXT_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_KEYMODULE_STATISTICS1")
            .field("v4Statistics", &self.v4Statistics)
            .field("v6Statistics", &self.v6Statistics)
            .field("errorFrequencyTable", &self.errorFrequencyTable)
            .field("mainModeNegotiationTime", &self.mainModeNegotiationTime)
            .field("quickModeNegotiationTime", &self.quickModeNegotiationTime)
            .field(
                "extendedModeNegotiationTime",
                &self.extendedModeNegotiationTime,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics
            && self.v6Statistics == other.v6Statistics
            && self.errorFrequencyTable == other.errorFrequencyTable
            && self.mainModeNegotiationTime == other.mainModeNegotiationTime
            && self.quickModeNegotiationTime == other.quickModeNegotiationTime
            && self.extendedModeNegotiationTime == other.extendedModeNegotiationTime
    }
}
impl ::std::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_KEYMODULE_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_KEY_MODULE_TYPE(pub i32);
pub const IKEEXT_KEY_MODULE_IKE: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(0i32);
pub const IKEEXT_KEY_MODULE_AUTHIP: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(1i32);
pub const IKEEXT_KEY_MODULE_IKEV2: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(2i32);
pub const IKEEXT_KEY_MODULE_MAX: IKEEXT_KEY_MODULE_TYPE = IKEEXT_KEY_MODULE_TYPE(3i32);
impl ::std::convert::From<i32> for IKEEXT_KEY_MODULE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_KEY_MODULE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_MM_SA_STATE(pub i32);
pub const IKEEXT_MM_SA_STATE_NONE: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(0i32);
pub const IKEEXT_MM_SA_STATE_SA_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(1i32);
pub const IKEEXT_MM_SA_STATE_SSPI_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(2i32);
pub const IKEEXT_MM_SA_STATE_FINAL: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(3i32);
pub const IKEEXT_MM_SA_STATE_FINAL_SENT: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(4i32);
pub const IKEEXT_MM_SA_STATE_COMPLETE: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(5i32);
pub const IKEEXT_MM_SA_STATE_MAX: IKEEXT_MM_SA_STATE = IKEEXT_MM_SA_STATE(6i32);
impl ::std::convert::From<i32> for IKEEXT_MM_SA_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_MM_SA_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_NAME_CREDENTIAL0 {
    pub principalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_NAME_CREDENTIAL0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_NAME_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_NAME_CREDENTIAL0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_NAME_CREDENTIAL0")
            .field("principalName", &self.principalName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_NAME_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.principalName == other.principalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_NAME_CREDENTIAL0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_NAME_CREDENTIAL0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_NTLM_V2_AUTHENTICATION0 {
    pub flags: u32,
}
impl IKEEXT_NTLM_V2_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_NTLM_V2_AUTHENTICATION0")
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_NTLM_V2_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IKEEXT_NTLM_V2_AUTH_DONT_ACCEPT_EXPLICIT_CREDENTIALS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_POLICY0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_POLICY0")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime
            && self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
            && self.numIkeProposals == other.numIkeProposals
            && self.ikeProposals == other.ikeProposals
            && self.flags == other.flags
            && self.maxDynamicFilters == other.maxDynamicFilters
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_POLICY1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_POLICY1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_POLICY1")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime
            && self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
            && self.numIkeProposals == other.numIkeProposals
            && self.ikeProposals == other.ikeProposals
            && self.flags == other.flags
            && self.maxDynamicFilters == other.maxDynamicFilters
            && self.retransmitDurationSecs == other.retransmitDurationSecs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_POLICY1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_POLICY2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IKEEXT_POLICY2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_POLICY2")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field(
                "initiatorImpersonationType",
                &self.initiatorImpersonationType,
            )
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime
            && self.numAuthenticationMethods == other.numAuthenticationMethods
            && self.authenticationMethods == other.authenticationMethods
            && self.initiatorImpersonationType == other.initiatorImpersonationType
            && self.numIkeProposals == other.numIkeProposals
            && self.ikeProposals == other.ikeProposals
            && self.flags == other.flags
            && self.maxDynamicFilters == other.maxDynamicFilters
            && self.retransmitDurationSecs == other.retransmitDurationSecs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_POLICY2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IKEEXT_POLICY_ENABLE_IKEV2_FRAGMENTATION: u32 = 128u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_POLICY_FLAG(pub u32);
pub const IKEEXT_POLICY_FLAG_DISABLE_DIAGNOSTICS: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(1u32);
pub const IKEEXT_POLICY_FLAG_NO_MACHINE_LUID_VERIFY: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(2u32);
pub const IKEEXT_POLICY_FLAG_NO_IMPERSONATION_LUID_VERIFY: IKEEXT_POLICY_FLAG =
    IKEEXT_POLICY_FLAG(4u32);
pub const IKEEXT_POLICY_FLAG_ENABLE_OPTIONAL_DH: IKEEXT_POLICY_FLAG = IKEEXT_POLICY_FLAG(8u32);
impl ::std::convert::From<u32> for IKEEXT_POLICY_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_POLICY_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_POLICY_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_POLICY_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IKEEXT_POLICY_FLAG_IMS_VPN: u32 = 64u32;
pub const IKEEXT_POLICY_FLAG_MOBIKE_NOT_SUPPORTED: u32 = 16u32;
pub const IKEEXT_POLICY_FLAG_SITE_TO_SITE: u32 = 32u32;
pub const IKEEXT_POLICY_SUPPORT_LOW_POWER_MODE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    pub presharedKey: FWP_BYTE_BLOB,
}
impl IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION0")
            .field("presharedKey", &self.presharedKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.presharedKey == other.presharedKey
    }
}
impl ::std::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    pub presharedKey: FWP_BYTE_BLOB,
    pub flags: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS,
}
impl IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {}
impl ::std::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION1")
            .field("presharedKey", &self.presharedKey)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.presharedKey == other.presharedKey && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(pub u32);
pub const IKEEXT_PSK_FLAG_LOCAL_AUTH_ONLY: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS =
    IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(1u32);
pub const IKEEXT_PSK_FLAG_REMOTE_AUTH_ONLY: IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS =
    IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS(2u32);
impl ::std::convert::From<u32> for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_PROPOSAL0 {
    pub cipherAlgorithm: IKEEXT_CIPHER_ALGORITHM0,
    pub integrityAlgorithm: IKEEXT_INTEGRITY_ALGORITHM0,
    pub maxLifetimeSeconds: u32,
    pub dhGroup: IKEEXT_DH_GROUP,
    pub quickModeLimit: u32,
}
impl IKEEXT_PROPOSAL0 {}
impl ::std::default::Default for IKEEXT_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_PROPOSAL0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_PROPOSAL0")
            .field("cipherAlgorithm", &self.cipherAlgorithm)
            .field("integrityAlgorithm", &self.integrityAlgorithm)
            .field("maxLifetimeSeconds", &self.maxLifetimeSeconds)
            .field("dhGroup", &self.dhGroup)
            .field("quickModeLimit", &self.quickModeLimit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherAlgorithm == other.cipherAlgorithm
            && self.integrityAlgorithm == other.integrityAlgorithm
            && self.maxLifetimeSeconds == other.maxLifetimeSeconds
            && self.dhGroup == other.dhGroup
            && self.quickModeLimit == other.quickModeLimit
    }
}
impl ::std::cmp::Eq for IKEEXT_PROPOSAL0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_PROPOSAL0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_QM_SA_STATE(pub i32);
pub const IKEEXT_QM_SA_STATE_NONE: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(0i32);
pub const IKEEXT_QM_SA_STATE_INITIAL: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(1i32);
pub const IKEEXT_QM_SA_STATE_FINAL: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(2i32);
pub const IKEEXT_QM_SA_STATE_COMPLETE: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(3i32);
pub const IKEEXT_QM_SA_STATE_MAX: IKEEXT_QM_SA_STATE = IKEEXT_QM_SA_STATE(4i32);
impl ::std::convert::From<i32> for IKEEXT_QM_SA_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_QM_SA_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_RESERVED_AUTHENTICATION0 {
    pub flags: IKEEXT_RESERVED_AUTHENTICATION_FLAGS,
}
impl IKEEXT_RESERVED_AUTHENTICATION0 {}
impl ::std::default::Default for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_RESERVED_AUTHENTICATION0")
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IKEEXT_RESERVED_AUTHENTICATION0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_RESERVED_AUTHENTICATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_RESERVED_AUTHENTICATION_FLAGS(pub u32);
pub const IKEEXT_RESERVED_AUTH_DISABLE_INITIATOR_TOKEN_GENERATION:
    IKEEXT_RESERVED_AUTHENTICATION_FLAGS = IKEEXT_RESERVED_AUTHENTICATION_FLAGS(1u32);
impl ::std::convert::From<u32> for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_SA_DETAILS0 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS0_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS0,
    pub ikePolicyKey: ::windows::runtime::GUID,
    pub virtualIfTunnelId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_SA_DETAILS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_SA_DETAILS0_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IKEEXT_SA_DETAILS0_0 {}
impl ::std::default::Default for IKEEXT_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS0_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_SA_DETAILS1 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS1_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS1,
    pub ikePolicyKey: ::windows::runtime::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: FWP_BYTE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_SA_DETAILS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_SA_DETAILS1_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IKEEXT_SA_DETAILS1_0 {}
impl ::std::default::Default for IKEEXT_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS1_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IKEEXT_SA_DETAILS2 {
    pub saId: u64,
    pub keyModuleType: IKEEXT_KEY_MODULE_TYPE,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IKEEXT_SA_DETAILS2_0,
    pub ikeTraffic: IKEEXT_TRAFFIC0,
    pub ikeProposal: IKEEXT_PROPOSAL0,
    pub cookiePair: IKEEXT_COOKIE_PAIR0,
    pub ikeCredentials: IKEEXT_CREDENTIALS2,
    pub ikePolicyKey: ::windows::runtime::GUID,
    pub virtualIfTunnelId: u64,
    pub correlationKey: FWP_BYTE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl IKEEXT_SA_DETAILS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IKEEXT_SA_DETAILS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_SA_DETAILS2_0 {
    pub v4UdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IKEEXT_SA_DETAILS2_0 {}
impl ::std::default::Default for IKEEXT_SA_DETAILS2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_SA_DETAILS2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_SA_DETAILS2_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_DETAILS2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IKEEXT_SA_ENUM_TEMPLATE0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
    pub remoteSubNet: FWP_CONDITION_VALUE0,
    pub localMainModeCertHash: FWP_BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IKEEXT_SA_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IKEEXT_SA_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IKEEXT_SA_ROLE(pub i32);
pub const IKEEXT_SA_ROLE_INITIATOR: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(0i32);
pub const IKEEXT_SA_ROLE_RESPONDER: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(1i32);
pub const IKEEXT_SA_ROLE_MAX: IKEEXT_SA_ROLE = IKEEXT_SA_ROLE(2i32);
impl ::std::convert::From<i32> for IKEEXT_SA_ROLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEEXT_SA_ROLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_STATISTICS0 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS0,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS0,
}
impl IKEEXT_STATISTICS0 {}
impl ::std::default::Default for IKEEXT_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_STATISTICS0")
            .field("ikeStatistics", &self.ikeStatistics)
            .field("authipStatistics", &self.authipStatistics)
            .field("commonStatistics", &self.commonStatistics)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.ikeStatistics == other.ikeStatistics
            && self.authipStatistics == other.authipStatistics
            && self.commonStatistics == other.commonStatistics
    }
}
impl ::std::cmp::Eq for IKEEXT_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_STATISTICS1 {
    pub ikeStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub authipStatistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub ikeV2Statistics: IKEEXT_KEYMODULE_STATISTICS1,
    pub commonStatistics: IKEEXT_COMMON_STATISTICS1,
}
impl IKEEXT_STATISTICS1 {}
impl ::std::default::Default for IKEEXT_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEEXT_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEEXT_STATISTICS1")
            .field("ikeStatistics", &self.ikeStatistics)
            .field("authipStatistics", &self.authipStatistics)
            .field("ikeV2Statistics", &self.ikeV2Statistics)
            .field("commonStatistics", &self.commonStatistics)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IKEEXT_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.ikeStatistics == other.ikeStatistics
            && self.authipStatistics == other.authipStatistics
            && self.ikeV2Statistics == other.ikeV2Statistics
            && self.commonStatistics == other.commonStatistics
    }
}
impl ::std::cmp::Eq for IKEEXT_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IKEEXT_TRAFFIC0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IKEEXT_TRAFFIC0_0,
    pub Anonymous2: IKEEXT_TRAFFIC0_1,
    pub authIpFilterId: u64,
}
impl IKEEXT_TRAFFIC0 {}
impl ::std::default::Default for IKEEXT_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_TRAFFIC0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_TRAFFIC0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_TRAFFIC0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IKEEXT_TRAFFIC0_0 {}
impl ::std::default::Default for IKEEXT_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_TRAFFIC0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_TRAFFIC0_0 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_TRAFFIC0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IKEEXT_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IKEEXT_TRAFFIC0_1 {}
impl ::std::default::Default for IKEEXT_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IKEEXT_TRAFFIC0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IKEEXT_TRAFFIC0_1 {}
unsafe impl ::windows::runtime::Abi for IKEEXT_TRAFFIC0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IN6_EMBEDDEDV4_BITS_IN_BYTE: u32 = 8u32;
pub const IN6_EMBEDDEDV4_UOCTET_POSITION: u32 = 8u32;
pub const IP4_OFF_MASK: u32 = 65311u32;
pub const IP6F_MORE_FRAG: u32 = 256u32;
pub const IP6F_OFF_MASK: u32 = 63743u32;
pub const IP6F_RESERVED_MASK: u32 = 1536u32;
pub const IP6OPT_MUTABLE: u32 = 32u32;
pub const IP6OPT_TYPE_DISCARD: u32 = 64u32;
pub const IP6OPT_TYPE_FORCEICMP: u32 = 128u32;
pub const IP6OPT_TYPE_ICMP: u32 = 192u32;
pub const IP6OPT_TYPE_SKIP: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_ADDRESS_INFO0 {
    pub numV4Addresses: u32,
    pub v4Addresses: *mut u32,
    pub numV6Addresses: u32,
    pub v6Addresses: *mut FWP_BYTE_ARRAY16,
}
impl IPSEC_ADDRESS_INFO0 {}
impl ::std::default::Default for IPSEC_ADDRESS_INFO0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_ADDRESS_INFO0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_ADDRESS_INFO0")
            .field("numV4Addresses", &self.numV4Addresses)
            .field("v4Addresses", &self.v4Addresses)
            .field("numV6Addresses", &self.numV6Addresses)
            .field("v6Addresses", &self.v6Addresses)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_ADDRESS_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        self.numV4Addresses == other.numV4Addresses
            && self.v4Addresses == other.v4Addresses
            && self.numV6Addresses == other.numV6Addresses
            && self.v6Addresses == other.v6Addresses
    }
}
impl ::std::cmp::Eq for IPSEC_ADDRESS_INFO0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_ADDRESS_INFO0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {}
impl ::std::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field(
                "decryptionFailuresOnInbound",
                &self.decryptionFailuresOnInbound,
            )
            .field(
                "authenticationFailuresOnInbound",
                &self.authenticationFailuresOnInbound,
            )
            .field(
                "udpEspValidationFailuresOnInbound",
                &self.udpEspValidationFailuresOnInbound,
            )
            .field(
                "replayCheckFailuresOnInbound",
                &self.replayCheckFailuresOnInbound,
            )
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field(
                "receiveOverIncorrectSaInbound",
                &self.receiveOverIncorrectSaInbound,
            )
            .field(
                "secureReceivesNotMatchingFilters",
                &self.secureReceivesNotMatchingFilters,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound
            && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound
            && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound
            && self.udpEspValidationFailuresOnInbound == other.udpEspValidationFailuresOnInbound
            && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound
            && self.invalidClearTextInbound == other.invalidClearTextInbound
            && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
            && self.receiveOverIncorrectSaInbound == other.receiveOverIncorrectSaInbound
            && self.secureReceivesNotMatchingFilters == other.secureReceivesNotMatchingFilters
    }
}
impl ::std::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {}
impl ::std::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field(
                "decryptionFailuresOnInbound",
                &self.decryptionFailuresOnInbound,
            )
            .field(
                "authenticationFailuresOnInbound",
                &self.authenticationFailuresOnInbound,
            )
            .field(
                "udpEspValidationFailuresOnInbound",
                &self.udpEspValidationFailuresOnInbound,
            )
            .field(
                "replayCheckFailuresOnInbound",
                &self.replayCheckFailuresOnInbound,
            )
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field(
                "receiveOverIncorrectSaInbound",
                &self.receiveOverIncorrectSaInbound,
            )
            .field(
                "secureReceivesNotMatchingFilters",
                &self.secureReceivesNotMatchingFilters,
            )
            .field("totalDropPacketsInbound", &self.totalDropPacketsInbound)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound
            && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound
            && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound
            && self.udpEspValidationFailuresOnInbound == other.udpEspValidationFailuresOnInbound
            && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound
            && self.invalidClearTextInbound == other.invalidClearTextInbound
            && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
            && self.receiveOverIncorrectSaInbound == other.receiveOverIncorrectSaInbound
            && self.secureReceivesNotMatchingFilters == other.secureReceivesNotMatchingFilters
            && self.totalDropPacketsInbound == other.totalDropPacketsInbound
    }
}
impl ::std::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_AGGREGATE_SA_STATISTICS0 {
    pub activeSas: u32,
    pub pendingSaNegotiations: u32,
    pub totalSasAdded: u32,
    pub totalSasDeleted: u32,
    pub successfulRekeys: u32,
    pub activeTunnels: u32,
    pub offloadedSas: u32,
}
impl IPSEC_AGGREGATE_SA_STATISTICS0 {}
impl ::std::default::Default for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AGGREGATE_SA_STATISTICS0")
            .field("activeSas", &self.activeSas)
            .field("pendingSaNegotiations", &self.pendingSaNegotiations)
            .field("totalSasAdded", &self.totalSasAdded)
            .field("totalSasDeleted", &self.totalSasDeleted)
            .field("successfulRekeys", &self.successfulRekeys)
            .field("activeTunnels", &self.activeTunnels)
            .field("offloadedSas", &self.offloadedSas)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.activeSas == other.activeSas
            && self.pendingSaNegotiations == other.pendingSaNegotiations
            && self.totalSasAdded == other.totalSasAdded
            && self.totalSasDeleted == other.totalSasDeleted
            && self.successfulRekeys == other.successfulRekeys
            && self.activeTunnels == other.activeTunnels
            && self.offloadedSas == other.offloadedSas
    }
}
impl ::std::cmp::Eq for IPSEC_AGGREGATE_SA_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AGGREGATE_SA_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_AH_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
impl IPSEC_AH_DROP_PACKET_STATISTICS0 {}
impl ::std::default::Default for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AH_DROP_PACKET_STATISTICS0")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field(
                "authenticationFailuresOnInbound",
                &self.authenticationFailuresOnInbound,
            )
            .field(
                "replayCheckFailuresOnInbound",
                &self.replayCheckFailuresOnInbound,
            )
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound
            && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound
            && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound
            && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
    }
}
impl ::std::cmp::Eq for IPSEC_AH_DROP_PACKET_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
}
impl IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {}
impl ::std::default::Default for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AUTH_AND_CIPHER_TRANSFORM0")
            .field("authTransform", &self.authTransform)
            .field("cipherTransform", &self.cipherTransform)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransform == other.authTransform && self.cipherTransform == other.cipherTransform
    }
}
impl ::std::cmp::Eq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPSEC_AUTH_CONFIG_GCM_AES_128: u32 = 3u32;
pub const IPSEC_AUTH_CONFIG_GCM_AES_192: u32 = 4u32;
pub const IPSEC_AUTH_CONFIG_GCM_AES_256: u32 = 5u32;
pub const IPSEC_AUTH_CONFIG_HMAC_MD5_96: u32 = 0u32;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_1_96: u32 = 1u32;
pub const IPSEC_AUTH_CONFIG_HMAC_SHA_256_128: u32 = 2u32;
pub const IPSEC_AUTH_CONFIG_MAX: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_AUTH_TRANSFORM0 {
    pub authTransformId: IPSEC_AUTH_TRANSFORM_ID0,
    pub cryptoModuleId: *mut ::windows::runtime::GUID,
}
impl IPSEC_AUTH_TRANSFORM0 {}
impl ::std::default::Default for IPSEC_AUTH_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AUTH_TRANSFORM0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AUTH_TRANSFORM0")
            .field("authTransformId", &self.authTransformId)
            .field("cryptoModuleId", &self.cryptoModuleId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AUTH_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransformId == other.authTransformId && self.cryptoModuleId == other.cryptoModuleId
    }
}
impl ::std::cmp::Eq for IPSEC_AUTH_TRANSFORM0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AUTH_TRANSFORM0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_AUTH_TRANSFORM_ID0 {
    pub authType: IPSEC_AUTH_TYPE,
    pub authConfig: u8,
}
impl IPSEC_AUTH_TRANSFORM_ID0 {}
impl ::std::default::Default for IPSEC_AUTH_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_AUTH_TRANSFORM_ID0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_AUTH_TRANSFORM_ID0")
            .field("authType", &self.authType)
            .field("authConfig", &self.authConfig)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_AUTH_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.authType == other.authType && self.authConfig == other.authConfig
    }
}
impl ::std::cmp::Eq for IPSEC_AUTH_TRANSFORM_ID0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_AUTH_TRANSFORM_ID0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_AUTH_TYPE(pub i32);
pub const IPSEC_AUTH_MD5: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(0i32);
pub const IPSEC_AUTH_SHA_1: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(1i32);
pub const IPSEC_AUTH_SHA_256: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(2i32);
pub const IPSEC_AUTH_AES_128: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(3i32);
pub const IPSEC_AUTH_AES_192: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(4i32);
pub const IPSEC_AUTH_AES_256: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(5i32);
pub const IPSEC_AUTH_MAX: IPSEC_AUTH_TYPE = IPSEC_AUTH_TYPE(6i32);
impl ::std::convert::From<i32> for IPSEC_AUTH_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_AUTH_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPSEC_CIPHER_CONFIG_CBC_3DES: u32 = 2u32;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_128: u32 = 3u32;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_192: u32 = 4u32;
pub const IPSEC_CIPHER_CONFIG_CBC_AES_256: u32 = 5u32;
pub const IPSEC_CIPHER_CONFIG_CBC_DES: u32 = 1u32;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_128: u32 = 6u32;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_192: u32 = 7u32;
pub const IPSEC_CIPHER_CONFIG_GCM_AES_256: u32 = 8u32;
pub const IPSEC_CIPHER_CONFIG_MAX: u32 = 9u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_CIPHER_TRANSFORM0 {
    pub cipherTransformId: IPSEC_CIPHER_TRANSFORM_ID0,
    pub cryptoModuleId: *mut ::windows::runtime::GUID,
}
impl IPSEC_CIPHER_TRANSFORM0 {}
impl ::std::default::Default for IPSEC_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_CIPHER_TRANSFORM0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_CIPHER_TRANSFORM0")
            .field("cipherTransformId", &self.cipherTransformId)
            .field("cryptoModuleId", &self.cryptoModuleId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherTransformId == other.cipherTransformId
            && self.cryptoModuleId == other.cryptoModuleId
    }
}
impl ::std::cmp::Eq for IPSEC_CIPHER_TRANSFORM0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_CIPHER_TRANSFORM0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_CIPHER_TRANSFORM_ID0 {
    pub cipherType: IPSEC_CIPHER_TYPE,
    pub cipherConfig: u8,
}
impl IPSEC_CIPHER_TRANSFORM_ID0 {}
impl ::std::default::Default for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_CIPHER_TRANSFORM_ID0")
            .field("cipherType", &self.cipherType)
            .field("cipherConfig", &self.cipherConfig)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherType == other.cipherType && self.cipherConfig == other.cipherConfig
    }
}
impl ::std::cmp::Eq for IPSEC_CIPHER_TRANSFORM_ID0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_CIPHER_TRANSFORM_ID0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_CIPHER_TYPE(pub i32);
pub const IPSEC_CIPHER_TYPE_DES: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(1i32);
pub const IPSEC_CIPHER_TYPE_3DES: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(2i32);
pub const IPSEC_CIPHER_TYPE_AES_128: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(3i32);
pub const IPSEC_CIPHER_TYPE_AES_192: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(4i32);
pub const IPSEC_CIPHER_TYPE_AES_256: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(5i32);
pub const IPSEC_CIPHER_TYPE_MAX: IPSEC_CIPHER_TYPE = IPSEC_CIPHER_TYPE(6i32);
impl ::std::convert::From<i32> for IPSEC_CIPHER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_CIPHER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPSEC_DOSP_DSCP_DISABLE_VALUE: u32 = 255u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_DOSP_FLAGS(pub u32);
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV1: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(1u32);
pub const IPSEC_DOSP_FLAG_ENABLE_IKEV2: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(2u32);
pub const IPSEC_DOSP_FLAG_DISABLE_AUTHIP: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(4u32);
pub const IPSEC_DOSP_FLAG_DISABLE_DEFAULT_BLOCK: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(8u32);
pub const IPSEC_DOSP_FLAG_FILTER_BLOCK: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(16u32);
pub const IPSEC_DOSP_FLAG_FILTER_EXEMPT: IPSEC_DOSP_FLAGS = IPSEC_DOSP_FLAGS(32u32);
impl ::std::convert::From<u32> for IPSEC_DOSP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_DOSP_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IPSEC_DOSP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IPSEC_DOSP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_DOSP_OPTIONS0 {}
impl ::std::default::Default for IPSEC_DOSP_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_DOSP_OPTIONS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_DOSP_OPTIONS0")
            .field("stateIdleTimeoutSeconds", &self.stateIdleTimeoutSeconds)
            .field(
                "perIPRateLimitQueueIdleTimeoutSeconds",
                &self.perIPRateLimitQueueIdleTimeoutSeconds,
            )
            .field("ipV6IPsecUnauthDscp", &self.ipV6IPsecUnauthDscp)
            .field(
                "ipV6IPsecUnauthRateLimitBytesPerSec",
                &self.ipV6IPsecUnauthRateLimitBytesPerSec,
            )
            .field(
                "ipV6IPsecUnauthPerIPRateLimitBytesPerSec",
                &self.ipV6IPsecUnauthPerIPRateLimitBytesPerSec,
            )
            .field("ipV6IPsecAuthDscp", &self.ipV6IPsecAuthDscp)
            .field(
                "ipV6IPsecAuthRateLimitBytesPerSec",
                &self.ipV6IPsecAuthRateLimitBytesPerSec,
            )
            .field("icmpV6Dscp", &self.icmpV6Dscp)
            .field(
                "icmpV6RateLimitBytesPerSec",
                &self.icmpV6RateLimitBytesPerSec,
            )
            .field("ipV6FilterExemptDscp", &self.ipV6FilterExemptDscp)
            .field(
                "ipV6FilterExemptRateLimitBytesPerSec",
                &self.ipV6FilterExemptRateLimitBytesPerSec,
            )
            .field("defBlockExemptDscp", &self.defBlockExemptDscp)
            .field(
                "defBlockExemptRateLimitBytesPerSec",
                &self.defBlockExemptRateLimitBytesPerSec,
            )
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
impl ::std::cmp::PartialEq for IPSEC_DOSP_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        self.stateIdleTimeoutSeconds == other.stateIdleTimeoutSeconds
            && self.perIPRateLimitQueueIdleTimeoutSeconds
                == other.perIPRateLimitQueueIdleTimeoutSeconds
            && self.ipV6IPsecUnauthDscp == other.ipV6IPsecUnauthDscp
            && self.ipV6IPsecUnauthRateLimitBytesPerSec == other.ipV6IPsecUnauthRateLimitBytesPerSec
            && self.ipV6IPsecUnauthPerIPRateLimitBytesPerSec
                == other.ipV6IPsecUnauthPerIPRateLimitBytesPerSec
            && self.ipV6IPsecAuthDscp == other.ipV6IPsecAuthDscp
            && self.ipV6IPsecAuthRateLimitBytesPerSec == other.ipV6IPsecAuthRateLimitBytesPerSec
            && self.icmpV6Dscp == other.icmpV6Dscp
            && self.icmpV6RateLimitBytesPerSec == other.icmpV6RateLimitBytesPerSec
            && self.ipV6FilterExemptDscp == other.ipV6FilterExemptDscp
            && self.ipV6FilterExemptRateLimitBytesPerSec
                == other.ipV6FilterExemptRateLimitBytesPerSec
            && self.defBlockExemptDscp == other.defBlockExemptDscp
            && self.defBlockExemptRateLimitBytesPerSec == other.defBlockExemptRateLimitBytesPerSec
            && self.maxStateEntries == other.maxStateEntries
            && self.maxPerIPRateLimitQueues == other.maxPerIPRateLimitQueues
            && self.flags == other.flags
            && self.numPublicIFLuids == other.numPublicIFLuids
            && self.publicIFLuids == other.publicIFLuids
            && self.numInternalIFLuids == other.numInternalIFLuids
            && self.internalIFLuids == other.internalIFLuids
            && self.publicV6AddrMask == other.publicV6AddrMask
            && self.internalV6AddrMask == other.internalV6AddrMask
    }
}
impl ::std::cmp::Eq for IPSEC_DOSP_OPTIONS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_DOSP_OPTIONS0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPSEC_DOSP_RATE_LIMIT_DISABLE_VALUE: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_DOSP_STATE0 {
    pub publicHostV6Addr: [u8; 16],
    pub internalHostV6Addr: [u8; 16],
    pub totalInboundIPv6IPsecAuthPackets: u64,
    pub totalOutboundIPv6IPsecAuthPackets: u64,
    pub durationSecs: u32,
}
impl IPSEC_DOSP_STATE0 {}
impl ::std::default::Default for IPSEC_DOSP_STATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_DOSP_STATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_DOSP_STATE0")
            .field("publicHostV6Addr", &self.publicHostV6Addr)
            .field("internalHostV6Addr", &self.internalHostV6Addr)
            .field(
                "totalInboundIPv6IPsecAuthPackets",
                &self.totalInboundIPv6IPsecAuthPackets,
            )
            .field(
                "totalOutboundIPv6IPsecAuthPackets",
                &self.totalOutboundIPv6IPsecAuthPackets,
            )
            .field("durationSecs", &self.durationSecs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_DOSP_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.publicHostV6Addr == other.publicHostV6Addr
            && self.internalHostV6Addr == other.internalHostV6Addr
            && self.totalInboundIPv6IPsecAuthPackets == other.totalInboundIPv6IPsecAuthPackets
            && self.totalOutboundIPv6IPsecAuthPackets == other.totalOutboundIPv6IPsecAuthPackets
            && self.durationSecs == other.durationSecs
    }
}
impl ::std::cmp::Eq for IPSEC_DOSP_STATE0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_DOSP_STATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    pub publicV6AddrMask: FWP_V6_ADDR_AND_MASK,
    pub internalV6AddrMask: FWP_V6_ADDR_AND_MASK,
}
impl IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {}
impl ::std::default::Default for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_DOSP_STATE_ENUM_TEMPLATE0")
            .field("publicV6AddrMask", &self.publicV6AddrMask)
            .field("internalV6AddrMask", &self.internalV6AddrMask)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.publicV6AddrMask == other.publicV6AddrMask
            && self.internalV6AddrMask == other.internalV6AddrMask
    }
}
impl ::std::cmp::Eq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_DOSP_STATISTICS0 {}
impl ::std::default::Default for IPSEC_DOSP_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_DOSP_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_DOSP_STATISTICS0")
            .field("totalStateEntriesCreated", &self.totalStateEntriesCreated)
            .field("currentStateEntries", &self.currentStateEntries)
            .field(
                "totalInboundAllowedIPv6IPsecUnauthPkts",
                &self.totalInboundAllowedIPv6IPsecUnauthPkts,
            )
            .field(
                "totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts",
                &self.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts,
            )
            .field(
                "totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts",
                &self.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts,
            )
            .field(
                "totalInboundOtherDiscardedIPv6IPsecUnauthPkts",
                &self.totalInboundOtherDiscardedIPv6IPsecUnauthPkts,
            )
            .field(
                "totalInboundAllowedIPv6IPsecAuthPkts",
                &self.totalInboundAllowedIPv6IPsecAuthPkts,
            )
            .field(
                "totalInboundRatelimitDiscardedIPv6IPsecAuthPkts",
                &self.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts,
            )
            .field(
                "totalInboundOtherDiscardedIPv6IPsecAuthPkts",
                &self.totalInboundOtherDiscardedIPv6IPsecAuthPkts,
            )
            .field(
                "totalInboundAllowedICMPv6Pkts",
                &self.totalInboundAllowedICMPv6Pkts,
            )
            .field(
                "totalInboundRatelimitDiscardedICMPv6Pkts",
                &self.totalInboundRatelimitDiscardedICMPv6Pkts,
            )
            .field(
                "totalInboundAllowedIPv6FilterExemptPkts",
                &self.totalInboundAllowedIPv6FilterExemptPkts,
            )
            .field(
                "totalInboundRatelimitDiscardedIPv6FilterExemptPkts",
                &self.totalInboundRatelimitDiscardedIPv6FilterExemptPkts,
            )
            .field(
                "totalInboundDiscardedIPv6FilterBlockPkts",
                &self.totalInboundDiscardedIPv6FilterBlockPkts,
            )
            .field(
                "totalInboundAllowedDefBlockExemptPkts",
                &self.totalInboundAllowedDefBlockExemptPkts,
            )
            .field(
                "totalInboundRatelimitDiscardedDefBlockExemptPkts",
                &self.totalInboundRatelimitDiscardedDefBlockExemptPkts,
            )
            .field(
                "totalInboundDiscardedDefBlockPkts",
                &self.totalInboundDiscardedDefBlockPkts,
            )
            .field(
                "currentInboundIPv6IPsecUnauthPerIPRateLimitQueues",
                &self.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_DOSP_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.totalStateEntriesCreated == other.totalStateEntriesCreated
            && self.currentStateEntries == other.currentStateEntries
            && self.totalInboundAllowedIPv6IPsecUnauthPkts
                == other.totalInboundAllowedIPv6IPsecUnauthPkts
            && self.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts
                == other.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts
                == other.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundOtherDiscardedIPv6IPsecUnauthPkts
                == other.totalInboundOtherDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundAllowedIPv6IPsecAuthPkts
                == other.totalInboundAllowedIPv6IPsecAuthPkts
            && self.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts
                == other.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts
            && self.totalInboundOtherDiscardedIPv6IPsecAuthPkts
                == other.totalInboundOtherDiscardedIPv6IPsecAuthPkts
            && self.totalInboundAllowedICMPv6Pkts == other.totalInboundAllowedICMPv6Pkts
            && self.totalInboundRatelimitDiscardedICMPv6Pkts
                == other.totalInboundRatelimitDiscardedICMPv6Pkts
            && self.totalInboundAllowedIPv6FilterExemptPkts
                == other.totalInboundAllowedIPv6FilterExemptPkts
            && self.totalInboundRatelimitDiscardedIPv6FilterExemptPkts
                == other.totalInboundRatelimitDiscardedIPv6FilterExemptPkts
            && self.totalInboundDiscardedIPv6FilterBlockPkts
                == other.totalInboundDiscardedIPv6FilterBlockPkts
            && self.totalInboundAllowedDefBlockExemptPkts
                == other.totalInboundAllowedDefBlockExemptPkts
            && self.totalInboundRatelimitDiscardedDefBlockExemptPkts
                == other.totalInboundRatelimitDiscardedDefBlockExemptPkts
            && self.totalInboundDiscardedDefBlockPkts == other.totalInboundDiscardedDefBlockPkts
            && self.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues
                == other.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues
    }
}
impl ::std::cmp::Eq for IPSEC_DOSP_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_DOSP_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    pub invalidSpisOnInbound: u32,
    pub decryptionFailuresOnInbound: u32,
    pub authenticationFailuresOnInbound: u32,
    pub replayCheckFailuresOnInbound: u32,
    pub saNotInitializedOnInbound: u32,
}
impl IPSEC_ESP_DROP_PACKET_STATISTICS0 {}
impl ::std::default::Default for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_ESP_DROP_PACKET_STATISTICS0")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field(
                "decryptionFailuresOnInbound",
                &self.decryptionFailuresOnInbound,
            )
            .field(
                "authenticationFailuresOnInbound",
                &self.authenticationFailuresOnInbound,
            )
            .field(
                "replayCheckFailuresOnInbound",
                &self.replayCheckFailuresOnInbound,
            )
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound
            && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound
            && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound
            && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound
            && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
    }
}
impl ::std::cmp::Eq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_FAILURE_POINT(pub i32);
pub const IPSEC_FAILURE_NONE: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(0i32);
pub const IPSEC_FAILURE_ME: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(1i32);
pub const IPSEC_FAILURE_PEER: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(2i32);
pub const IPSEC_FAILURE_POINT_MAX: IPSEC_FAILURE_POINT = IPSEC_FAILURE_POINT(3i32);
impl ::std::convert::From<i32> for IPSEC_FAILURE_POINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_FAILURE_POINT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_GETSPI0 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC0,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI0_0,
    pub rngCryptoModuleID: *mut ::windows::runtime::GUID,
}
impl IPSEC_GETSPI0 {}
impl ::std::default::Default for IPSEC_GETSPI0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_GETSPI0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_GETSPI0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_GETSPI0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_GETSPI0_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IPSEC_GETSPI0_0 {}
impl ::std::default::Default for IPSEC_GETSPI0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_GETSPI0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_GETSPI0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_GETSPI0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_GETSPI1 {
    pub inboundIpsecTraffic: IPSEC_TRAFFIC1,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_GETSPI1_0,
    pub rngCryptoModuleID: *mut ::windows::runtime::GUID,
}
impl IPSEC_GETSPI1 {}
impl ::std::default::Default for IPSEC_GETSPI1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_GETSPI1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_GETSPI1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_GETSPI1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_GETSPI1_0 {
    pub inboundUdpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IPSEC_GETSPI1_0 {}
impl ::std::default::Default for IPSEC_GETSPI1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_GETSPI1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_GETSPI1_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_GETSPI1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_ID0 {
    pub mmTargetName: super::super::Foundation::PWSTR,
    pub emTargetName: super::super::Foundation::PWSTR,
    pub numTokens: u32,
    pub tokens: *mut IPSEC_TOKEN0,
    pub explicitCredentials: u64,
    pub logonId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_ID0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_ID0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPSEC_ID0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_ID0")
            .field("mmTargetName", &self.mmTargetName)
            .field("emTargetName", &self.emTargetName)
            .field("numTokens", &self.numTokens)
            .field("tokens", &self.tokens)
            .field("explicitCredentials", &self.explicitCredentials)
            .field("logonId", &self.logonId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.mmTargetName == other.mmTargetName
            && self.emTargetName == other.emTargetName
            && self.numTokens == other.numTokens
            && self.tokens == other.tokens
            && self.explicitCredentials == other.explicitCredentials
            && self.logonId == other.logonId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_ID0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_ID0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_KEYING_POLICY0 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut ::windows::runtime::GUID,
}
impl IPSEC_KEYING_POLICY0 {}
impl ::std::default::Default for IPSEC_KEYING_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_KEYING_POLICY0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_KEYING_POLICY0")
            .field("numKeyMods", &self.numKeyMods)
            .field("keyModKeys", &self.keyModKeys)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_KEYING_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numKeyMods == other.numKeyMods && self.keyModKeys == other.keyModKeys
    }
}
impl ::std::cmp::Eq for IPSEC_KEYING_POLICY0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_KEYING_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_KEYING_POLICY1 {
    pub numKeyMods: u32,
    pub keyModKeys: *mut ::windows::runtime::GUID,
    pub flags: u32,
}
impl IPSEC_KEYING_POLICY1 {}
impl ::std::default::Default for IPSEC_KEYING_POLICY1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_KEYING_POLICY1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_KEYING_POLICY1")
            .field("numKeyMods", &self.numKeyMods)
            .field("keyModKeys", &self.keyModKeys)
            .field("flags", &self.flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_KEYING_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numKeyMods == other.numKeyMods
            && self.keyModKeys == other.keyModKeys
            && self.flags == other.flags
    }
}
impl ::std::cmp::Eq for IPSEC_KEYING_POLICY1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_KEYING_POLICY1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPSEC_KEYING_POLICY_FLAG_TERMINATING_MATCH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_KEYMODULE_STATE0 {
    pub keyModuleKey: ::windows::runtime::GUID,
    pub stateBlob: FWP_BYTE_BLOB,
}
impl IPSEC_KEYMODULE_STATE0 {}
impl ::std::default::Default for IPSEC_KEYMODULE_STATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_KEYMODULE_STATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_KEYMODULE_STATE0")
            .field("keyModuleKey", &self.keyModuleKey)
            .field("stateBlob", &self.stateBlob)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_KEYMODULE_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyModuleKey == other.keyModuleKey && self.stateBlob == other.stateBlob
    }
}
impl ::std::cmp::Eq for IPSEC_KEYMODULE_STATE0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_KEYMODULE_STATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_KEY_MANAGER0 {
    pub keyManagerKey: ::windows::runtime::GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: u32,
    pub keyDictationTimeoutHint: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_KEY_MANAGER0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_KEY_MANAGER0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPSEC_KEY_MANAGER0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_KEY_MANAGER0")
            .field("keyManagerKey", &self.keyManagerKey)
            .field("displayData", &self.displayData)
            .field("flags", &self.flags)
            .field("keyDictationTimeoutHint", &self.keyDictationTimeoutHint)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_KEY_MANAGER0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyManagerKey == other.keyManagerKey
            && self.displayData == other.displayData
            && self.flags == other.flags
            && self.keyDictationTimeoutHint == other.keyDictationTimeoutHint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_KEY_MANAGER0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_KEY_MANAGER0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_KEY_MANAGER_CALLBACKS0 {
    pub reserved: ::windows::runtime::GUID,
    pub flags: u32,
    pub keyDictationCheck: ::std::option::Option<IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0>,
    pub keyDictation: ::std::option::Option<IPSEC_KEY_MANAGER_DICTATE_KEY0>,
    pub keyNotify: ::std::option::Option<IPSEC_KEY_MANAGER_NOTIFY_KEY0>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IPSEC_KEY_MANAGER_CALLBACKS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_KEY_MANAGER_CALLBACKS0")
            .field("reserved", &self.reserved)
            .field("flags", &self.flags)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
            && self.flags == other.flags
            && self.keyDictationCheck.map(|f| f as usize)
                == other.keyDictationCheck.map(|f| f as usize)
            && self.keyDictation.map(|f| f as usize) == other.keyDictation.map(|f| f as usize)
            && self.keyNotify.map(|f| f as usize) == other.keyNotify.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_KEY_MANAGER_CALLBACKS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_KEY_MANAGER_CALLBACKS0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type IPSEC_KEY_MANAGER_DICTATE_KEY0 = unsafe extern "system" fn(
    inboundsadetails: *mut IPSEC_SA_DETAILS1,
    outboundsadetails: *mut IPSEC_SA_DETAILS1,
    keyingmodulegenkey: *mut super::super::Foundation::BOOL,
) -> u32;
pub const IPSEC_KEY_MANAGER_FLAG_DICTATE_KEY: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub type IPSEC_KEY_MANAGER_KEY_DICTATION_CHECK0 = unsafe extern "system" fn(
    iketraffic: *const IKEEXT_TRAFFIC0,
    willdictatekey: *mut super::super::Foundation::BOOL,
    weight: *mut u32,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type IPSEC_KEY_MANAGER_NOTIFY_KEY0 = unsafe extern "system" fn(
    inboundsa: *const IPSEC_SA_DETAILS1,
    outboundsa: *const IPSEC_SA_DETAILS1,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_PFS_GROUP(pub i32);
pub const IPSEC_PFS_NONE: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(0i32);
pub const IPSEC_PFS_1: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(1i32);
pub const IPSEC_PFS_2: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(2i32);
pub const IPSEC_PFS_2048: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(3i32);
pub const IPSEC_PFS_14: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(3i32);
pub const IPSEC_PFS_ECP_256: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(4i32);
pub const IPSEC_PFS_ECP_384: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(5i32);
pub const IPSEC_PFS_MM: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(6i32);
pub const IPSEC_PFS_24: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(7i32);
pub const IPSEC_PFS_MAX: IPSEC_PFS_GROUP = IPSEC_PFS_GROUP(8i32);
impl ::std::convert::From<i32> for IPSEC_PFS_GROUP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_PFS_GROUP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_POLICY_FLAG(pub u32);
pub const IPSEC_POLICY_FLAG_ND_SECURE: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(2u32);
pub const IPSEC_POLICY_FLAG_ND_BOUNDARY: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(4u32);
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_PEER_BEHIND_NAT: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(16u32);
pub const IPSEC_POLICY_FLAG_NAT_ENCAP_ALLOW_GENERAL_NAT_TRAVERSAL: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(32u32);
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_SECOND_LIFETIME: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(64u32);
pub const IPSEC_POLICY_FLAG_DONT_NEGOTIATE_BYTE_LIFETIME: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(128u32);
pub const IPSEC_POLICY_FLAG_CLEAR_DF_ON_TUNNEL: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(8u32);
pub const IPSEC_POLICY_FLAG_ENABLE_V6_IN_V4_TUNNELING: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(256u32);
pub const IPSEC_POLICY_FLAG_ENABLE_SERVER_ADDR_ASSIGNMENT: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(512u32);
pub const IPSEC_POLICY_FLAG_TUNNEL_ALLOW_OUTBOUND_CLEAR_CONNECTION: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(1024u32);
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ALREADY_SECURE_CONNECTION: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(2048u32);
pub const IPSEC_POLICY_FLAG_TUNNEL_BYPASS_ICMPV6: IPSEC_POLICY_FLAG = IPSEC_POLICY_FLAG(4096u32);
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_DICTATE_KEY: IPSEC_POLICY_FLAG =
    IPSEC_POLICY_FLAG(8192u32);
impl ::std::convert::From<u32> for IPSEC_POLICY_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_POLICY_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IPSEC_POLICY_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IPSEC_POLICY_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IPSEC_POLICY_FLAG_KEY_MANAGER_ALLOW_NOTIFY_KEY: u32 = 16384u32;
pub const IPSEC_POLICY_FLAG_RESERVED1: u32 = 32768u32;
pub const IPSEC_POLICY_FLAG_SITE_TO_SITE_TUNNEL: u32 = 65536u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_PROPOSAL0 {
    pub lifetime: IPSEC_SA_LIFETIME0,
    pub numSaTransforms: u32,
    pub saTransforms: *mut IPSEC_SA_TRANSFORM0,
    pub pfsGroup: IPSEC_PFS_GROUP,
}
impl IPSEC_PROPOSAL0 {}
impl ::std::default::Default for IPSEC_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_PROPOSAL0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_PROPOSAL0")
            .field("lifetime", &self.lifetime)
            .field("numSaTransforms", &self.numSaTransforms)
            .field("saTransforms", &self.saTransforms)
            .field("pfsGroup", &self.pfsGroup)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.lifetime == other.lifetime
            && self.numSaTransforms == other.numSaTransforms
            && self.saTransforms == other.saTransforms
            && self.pfsGroup == other.pfsGroup
    }
}
impl ::std::cmp::Eq for IPSEC_PROPOSAL0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_PROPOSAL0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA0 {
    pub spi: u32,
    pub saTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA0_0,
}
impl IPSEC_SA0 {}
impl ::std::default::Default for IPSEC_SA0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA0_0 {
    pub ahInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espAuthInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
    pub espCipherInformation: *mut IPSEC_SA_CIPHER_INFORMATION0,
    pub espAuthAndCipherInformation: *mut IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0,
    pub espAuthFwInformation: *mut IPSEC_SA_AUTH_INFORMATION0,
}
impl IPSEC_SA0_0 {}
impl ::std::default::Default for IPSEC_SA0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    pub saCipherInformation: IPSEC_SA_CIPHER_INFORMATION0,
    pub saAuthInformation: IPSEC_SA_AUTH_INFORMATION0,
}
impl IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {}
impl ::std::default::Default for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0")
            .field("saCipherInformation", &self.saCipherInformation)
            .field("saAuthInformation", &self.saAuthInformation)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.saCipherInformation == other.saCipherInformation
            && self.saAuthInformation == other.saAuthInformation
    }
}
impl ::std::cmp::Eq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_AUTH_INFORMATION0 {
    pub authTransform: IPSEC_AUTH_TRANSFORM0,
    pub authKey: FWP_BYTE_BLOB,
}
impl IPSEC_SA_AUTH_INFORMATION0 {}
impl ::std::default::Default for IPSEC_SA_AUTH_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_AUTH_INFORMATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_AUTH_INFORMATION0")
            .field("authTransform", &self.authTransform)
            .field("authKey", &self.authKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_AUTH_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransform == other.authTransform && self.authKey == other.authKey
    }
}
impl ::std::cmp::Eq for IPSEC_SA_AUTH_INFORMATION0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_AUTH_INFORMATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_SA_BUNDLE0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_SA_BUNDLE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_SA_BUNDLE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_SA_BUNDLE0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_BUNDLE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA_BUNDLE0_0 {
    pub peerV4PrivateAddress: u32,
}
impl IPSEC_SA_BUNDLE0_0 {}
impl ::std::default::Default for IPSEC_SA_BUNDLE0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_BUNDLE0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_BUNDLE0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_BUNDLE0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
    pub saLookupContext: ::windows::runtime::GUID,
    pub qmFilterId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_SA_BUNDLE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_SA_BUNDLE1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_SA_BUNDLE1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_SA_BUNDLE1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_BUNDLE1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA_BUNDLE1_0 {
    pub peerV4PrivateAddress: u32,
}
impl IPSEC_SA_BUNDLE1_0 {}
impl ::std::default::Default for IPSEC_SA_BUNDLE1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_BUNDLE1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_BUNDLE1_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_BUNDLE1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_SA_BUNDLE_FLAGS(pub u32);
pub const IPSEC_SA_BUNDLE_FLAG_ND_SECURE: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(1u32);
pub const IPSEC_SA_BUNDLE_FLAG_ND_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS = IPSEC_SA_BUNDLE_FLAGS(2u32);
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_NAT_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(4u32);
pub const IPSEC_SA_BUNDLE_FLAG_GUARANTEE_ENCRYPTION: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(8u32);
pub const IPSEC_SA_BUNDLE_FLAG_ALLOW_NULL_TARGET_NAME_MATCH: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(512u32);
pub const IPSEC_SA_BUNDLE_FLAG_CLEAR_DF_ON_TUNNEL: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(1024u32);
pub const IPSEC_SA_BUNDLE_FLAG_ASSUME_UDP_CONTEXT_OUTBOUND: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(2048u32);
pub const IPSEC_SA_BUNDLE_FLAG_ND_PEER_BOUNDARY: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(4096u32);
pub const IPSEC_SA_BUNDLE_FLAG_SUPPRESS_DUPLICATE_DELETION: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(8192u32);
pub const IPSEC_SA_BUNDLE_FLAG_PEER_SUPPORTS_GUARANTEE_ENCRYPTION: IPSEC_SA_BUNDLE_FLAGS =
    IPSEC_SA_BUNDLE_FLAGS(16384u32);
impl ::std::convert::From<u32> for IPSEC_SA_BUNDLE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_BUNDLE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const IPSEC_SA_BUNDLE_FLAG_ENABLE_OPTIONAL_ASYMMETRIC_IDLE: u32 = 262144u32;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_INBOUND_CONNECTIONS: u32 = 32768u32;
pub const IPSEC_SA_BUNDLE_FLAG_FORCE_OUTBOUND_CONNECTIONS: u32 = 65536u32;
pub const IPSEC_SA_BUNDLE_FLAG_FORWARD_PATH_INITIATOR: u32 = 131072u32;
pub const IPSEC_SA_BUNDLE_FLAG_IP_IN_IP_PKT: u32 = 4194304u32;
pub const IPSEC_SA_BUNDLE_FLAG_LOCALLY_DICTATED_KEYS: u32 = 1048576u32;
pub const IPSEC_SA_BUNDLE_FLAG_LOW_POWER_MODE_SUPPORT: u32 = 8388608u32;
pub const IPSEC_SA_BUNDLE_FLAG_NLB: u32 = 16u32;
pub const IPSEC_SA_BUNDLE_FLAG_NO_EXPLICIT_CRED_MATCH: u32 = 128u32;
pub const IPSEC_SA_BUNDLE_FLAG_NO_IMPERSONATION_LUID_VERIFY: u32 = 64u32;
pub const IPSEC_SA_BUNDLE_FLAG_NO_MACHINE_LUID_VERIFY: u32 = 32u32;
pub const IPSEC_SA_BUNDLE_FLAG_SA_OFFLOADED: u32 = 2097152u32;
pub const IPSEC_SA_BUNDLE_FLAG_USING_DICTATED_KEYS: u32 = 524288u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_CIPHER_INFORMATION0 {
    pub cipherTransform: IPSEC_CIPHER_TRANSFORM0,
    pub cipherKey: FWP_BYTE_BLOB,
}
impl IPSEC_SA_CIPHER_INFORMATION0 {}
impl ::std::default::Default for IPSEC_SA_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_CIPHER_INFORMATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_CIPHER_INFORMATION0")
            .field("cipherTransform", &self.cipherTransform)
            .field("cipherKey", &self.cipherKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherTransform == other.cipherTransform && self.cipherKey == other.cipherKey
    }
}
impl ::std::cmp::Eq for IPSEC_SA_CIPHER_INFORMATION0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CIPHER_INFORMATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT0 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS0,
    pub outboundSa: *mut IPSEC_SA_DETAILS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IPSEC_SA_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for IPSEC_SA_CONTEXT0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_CONTEXT0")
            .field("saContextId", &self.saContextId)
            .field("inboundSa", &self.inboundSa)
            .field("outboundSa", &self.outboundSa)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_CONTEXT0 {
    fn eq(&self, other: &Self) -> bool {
        self.saContextId == other.saContextId
            && self.inboundSa == other.inboundSa
            && self.outboundSa == other.outboundSa
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT1 {
    pub saContextId: u64,
    pub inboundSa: *mut IPSEC_SA_DETAILS1,
    pub outboundSa: *mut IPSEC_SA_DETAILS1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IPSEC_SA_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for IPSEC_SA_CONTEXT1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_CONTEXT1")
            .field("saContextId", &self.saContextId)
            .field("inboundSa", &self.inboundSa)
            .field("outboundSa", &self.outboundSa)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_CONTEXT1 {
    fn eq(&self, other: &Self) -> bool {
        self.saContextId == other.saContextId
            && self.inboundSa == other.inboundSa
            && self.outboundSa == other.outboundSa
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type IPSEC_SA_CONTEXT_CALLBACK0 = unsafe extern "system" fn(
    context: *mut ::std::ffi::c_void,
    change: *const IPSEC_SA_CONTEXT_CHANGE0,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_CONTEXT_CHANGE0 {
    pub changeType: IPSEC_SA_CONTEXT_EVENT_TYPE0,
    pub saContextId: u64,
}
impl IPSEC_SA_CONTEXT_CHANGE0 {}
impl ::std::default::Default for IPSEC_SA_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_CONTEXT_CHANGE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_CONTEXT_CHANGE0")
            .field("changeType", &self.changeType)
            .field("saContextId", &self.saContextId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.saContextId == other.saContextId
    }
}
impl ::std::cmp::Eq for IPSEC_SA_CONTEXT_CHANGE0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT_CHANGE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    pub localSubNet: FWP_CONDITION_VALUE0,
    pub remoteSubNet: FWP_CONDITION_VALUE0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_SA_CONTEXT_EVENT_TYPE0(pub i32);
pub const IPSEC_SA_CONTEXT_EVENT_ADD: IPSEC_SA_CONTEXT_EVENT_TYPE0 =
    IPSEC_SA_CONTEXT_EVENT_TYPE0(1i32);
pub const IPSEC_SA_CONTEXT_EVENT_DELETE: IPSEC_SA_CONTEXT_EVENT_TYPE0 =
    IPSEC_SA_CONTEXT_EVENT_TYPE0(2i32);
pub const IPSEC_SA_CONTEXT_EVENT_MAX: IPSEC_SA_CONTEXT_EVENT_TYPE0 =
    IPSEC_SA_CONTEXT_EVENT_TYPE0(3i32);
impl ::std::convert::From<i32> for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    pub enumTemplate: *mut IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
    pub flags: u32,
    pub sessionKey: ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IPSEC_SA_CONTEXT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_CONTEXT_SUBSCRIPTION0")
            .field("enumTemplate", &self.enumTemplate)
            .field("flags", &self.flags)
            .field("sessionKey", &self.sessionKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate
            && self.flags == other.flags
            && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_SA_DETAILS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_DETAILS0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_DETAILS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_DETAILS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA_DETAILS0_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IPSEC_SA_DETAILS0_0 {}
impl ::std::default::Default for IPSEC_SA_DETAILS0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_DETAILS0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_DETAILS0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_DETAILS0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_SA_DETAILS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for IPSEC_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for IPSEC_SA_DETAILS1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for IPSEC_SA_DETAILS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for IPSEC_SA_DETAILS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA_DETAILS1_0 {
    pub udpEncapsulation: *mut IPSEC_V4_UDP_ENCAPSULATION0,
}
impl IPSEC_SA_DETAILS1_0 {}
impl ::std::default::Default for IPSEC_SA_DETAILS1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_DETAILS1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_DETAILS1_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_DETAILS1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_ENUM_TEMPLATE0 {
    pub saDirection: FWP_DIRECTION,
}
impl IPSEC_SA_ENUM_TEMPLATE0 {}
impl ::std::default::Default for IPSEC_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_ENUM_TEMPLATE0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_ENUM_TEMPLATE0")
            .field("saDirection", &self.saDirection)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.saDirection == other.saDirection
    }
}
impl ::std::cmp::Eq for IPSEC_SA_ENUM_TEMPLATE0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_ENUM_TEMPLATE0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_IDLE_TIMEOUT0 {
    pub idleTimeoutSeconds: u32,
    pub idleTimeoutSecondsFailOver: u32,
}
impl IPSEC_SA_IDLE_TIMEOUT0 {}
impl ::std::default::Default for IPSEC_SA_IDLE_TIMEOUT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_IDLE_TIMEOUT0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_IDLE_TIMEOUT0")
            .field("idleTimeoutSeconds", &self.idleTimeoutSeconds)
            .field(
                "idleTimeoutSecondsFailOver",
                &self.idleTimeoutSecondsFailOver,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_IDLE_TIMEOUT0 {
    fn eq(&self, other: &Self) -> bool {
        self.idleTimeoutSeconds == other.idleTimeoutSeconds
            && self.idleTimeoutSecondsFailOver == other.idleTimeoutSecondsFailOver
    }
}
impl ::std::cmp::Eq for IPSEC_SA_IDLE_TIMEOUT0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_IDLE_TIMEOUT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_LIFETIME0 {
    pub lifetimeSeconds: u32,
    pub lifetimeKilobytes: u32,
    pub lifetimePackets: u32,
}
impl IPSEC_SA_LIFETIME0 {}
impl ::std::default::Default for IPSEC_SA_LIFETIME0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_SA_LIFETIME0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_SA_LIFETIME0")
            .field("lifetimeSeconds", &self.lifetimeSeconds)
            .field("lifetimeKilobytes", &self.lifetimeKilobytes)
            .field("lifetimePackets", &self.lifetimePackets)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_LIFETIME0 {
    fn eq(&self, other: &Self) -> bool {
        self.lifetimeSeconds == other.lifetimeSeconds
            && self.lifetimeKilobytes == other.lifetimeKilobytes
            && self.lifetimePackets == other.lifetimePackets
    }
}
impl ::std::cmp::Eq for IPSEC_SA_LIFETIME0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_LIFETIME0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_SA_TRANSFORM0 {
    pub ipsecTransformType: IPSEC_TRANSFORM_TYPE,
    pub Anonymous: IPSEC_SA_TRANSFORM0_0,
}
impl IPSEC_SA_TRANSFORM0 {}
impl ::std::default::Default for IPSEC_SA_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_TRANSFORM0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_TRANSFORM0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_TRANSFORM0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_SA_TRANSFORM0_0 {
    pub ahTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espAuthTransform: *mut IPSEC_AUTH_TRANSFORM0,
    pub espCipherTransform: *mut IPSEC_CIPHER_TRANSFORM0,
    pub espAuthAndCipherTransform: *mut IPSEC_AUTH_AND_CIPHER_TRANSFORM0,
    pub espAuthFwTransform: *mut IPSEC_AUTH_TRANSFORM0,
}
impl IPSEC_SA_TRANSFORM0_0 {}
impl ::std::default::Default for IPSEC_SA_TRANSFORM0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_SA_TRANSFORM0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_SA_TRANSFORM0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_SA_TRANSFORM0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_STATISTICS0 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS0,
}
impl IPSEC_STATISTICS0 {}
impl ::std::default::Default for IPSEC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_STATISTICS0")
            .field("aggregateSaStatistics", &self.aggregateSaStatistics)
            .field("espDropPacketStatistics", &self.espDropPacketStatistics)
            .field("ahDropPacketStatistics", &self.ahDropPacketStatistics)
            .field(
                "aggregateDropPacketStatistics",
                &self.aggregateDropPacketStatistics,
            )
            .field("inboundTrafficStatistics", &self.inboundTrafficStatistics)
            .field("outboundTrafficStatistics", &self.outboundTrafficStatistics)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.aggregateSaStatistics == other.aggregateSaStatistics
            && self.espDropPacketStatistics == other.espDropPacketStatistics
            && self.ahDropPacketStatistics == other.ahDropPacketStatistics
            && self.aggregateDropPacketStatistics == other.aggregateDropPacketStatistics
            && self.inboundTrafficStatistics == other.inboundTrafficStatistics
            && self.outboundTrafficStatistics == other.outboundTrafficStatistics
    }
}
impl ::std::cmp::Eq for IPSEC_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_STATISTICS1 {
    pub aggregateSaStatistics: IPSEC_AGGREGATE_SA_STATISTICS0,
    pub espDropPacketStatistics: IPSEC_ESP_DROP_PACKET_STATISTICS0,
    pub ahDropPacketStatistics: IPSEC_AH_DROP_PACKET_STATISTICS0,
    pub aggregateDropPacketStatistics: IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1,
    pub inboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
    pub outboundTrafficStatistics: IPSEC_TRAFFIC_STATISTICS1,
}
impl IPSEC_STATISTICS1 {}
impl ::std::default::Default for IPSEC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_STATISTICS1")
            .field("aggregateSaStatistics", &self.aggregateSaStatistics)
            .field("espDropPacketStatistics", &self.espDropPacketStatistics)
            .field("ahDropPacketStatistics", &self.ahDropPacketStatistics)
            .field(
                "aggregateDropPacketStatistics",
                &self.aggregateDropPacketStatistics,
            )
            .field("inboundTrafficStatistics", &self.inboundTrafficStatistics)
            .field("outboundTrafficStatistics", &self.outboundTrafficStatistics)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.aggregateSaStatistics == other.aggregateSaStatistics
            && self.espDropPacketStatistics == other.espDropPacketStatistics
            && self.ahDropPacketStatistics == other.ahDropPacketStatistics
            && self.aggregateDropPacketStatistics == other.aggregateDropPacketStatistics
            && self.inboundTrafficStatistics == other.inboundTrafficStatistics
            && self.outboundTrafficStatistics == other.outboundTrafficStatistics
    }
}
impl ::std::cmp::Eq for IPSEC_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TOKEN0 {
    pub r#type: IPSEC_TOKEN_TYPE,
    pub principal: IPSEC_TOKEN_PRINCIPAL,
    pub mode: IPSEC_TOKEN_MODE,
    pub token: u64,
}
impl IPSEC_TOKEN0 {}
impl ::std::default::Default for IPSEC_TOKEN0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_TOKEN0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TOKEN0")
            .field("r#type", &self.r#type)
            .field("principal", &self.principal)
            .field("mode", &self.mode)
            .field("token", &self.token)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_TOKEN0 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
            && self.principal == other.principal
            && self.mode == other.mode
            && self.token == other.token
    }
}
impl ::std::cmp::Eq for IPSEC_TOKEN0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TOKEN0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_TOKEN_MODE(pub i32);
pub const IPSEC_TOKEN_MODE_MAIN: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(0i32);
pub const IPSEC_TOKEN_MODE_EXTENDED: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(1i32);
pub const IPSEC_TOKEN_MODE_MAX: IPSEC_TOKEN_MODE = IPSEC_TOKEN_MODE(2i32);
impl ::std::convert::From<i32> for IPSEC_TOKEN_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_TOKEN_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_TOKEN_PRINCIPAL(pub i32);
pub const IPSEC_TOKEN_PRINCIPAL_LOCAL: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(0i32);
pub const IPSEC_TOKEN_PRINCIPAL_PEER: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(1i32);
pub const IPSEC_TOKEN_PRINCIPAL_MAX: IPSEC_TOKEN_PRINCIPAL = IPSEC_TOKEN_PRINCIPAL(2i32);
impl ::std::convert::From<i32> for IPSEC_TOKEN_PRINCIPAL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_TOKEN_PRINCIPAL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_TOKEN_TYPE(pub i32);
pub const IPSEC_TOKEN_TYPE_MACHINE: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(0i32);
pub const IPSEC_TOKEN_TYPE_IMPERSONATION: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(1i32);
pub const IPSEC_TOKEN_TYPE_MAX: IPSEC_TOKEN_TYPE = IPSEC_TOKEN_TYPE(2i32);
impl ::std::convert::From<i32> for IPSEC_TOKEN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_TOKEN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TRAFFIC0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TRAFFIC0_0,
    pub Anonymous2: IPSEC_TRAFFIC0_1,
    pub trafficType: IPSEC_TRAFFIC_TYPE,
    pub Anonymous3: IPSEC_TRAFFIC0_2,
    pub remotePort: u16,
}
impl IPSEC_TRAFFIC0 {}
impl ::std::default::Default for IPSEC_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC0_0 {}
impl ::std::default::Default for IPSEC_TRAFFIC0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC0_1 {}
impl ::std::default::Default for IPSEC_TRAFFIC0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC0_1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC0_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
impl IPSEC_TRAFFIC0_2 {}
impl ::std::default::Default for IPSEC_TRAFFIC0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC0_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC0_2 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPSEC_TRAFFIC1 {}
impl ::std::default::Default for IPSEC_TRAFFIC1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC1_0 {}
impl ::std::default::Default for IPSEC_TRAFFIC1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC1_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC1_1 {}
impl ::std::default::Default for IPSEC_TRAFFIC1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC1_1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC1_2 {
    pub ipsecFilterId: u64,
    pub tunnelPolicyId: u64,
}
impl IPSEC_TRAFFIC1_2 {}
impl ::std::default::Default for IPSEC_TRAFFIC1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC1_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC1_2 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC1_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TRAFFIC_SELECTOR0_ {
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TRAFFIC_SELECTOR0__0,
    pub Anonymous2: IPSEC_TRAFFIC_SELECTOR0__1,
}
impl IPSEC_TRAFFIC_SELECTOR0_ {}
impl ::std::default::Default for IPSEC_TRAFFIC_SELECTOR0_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0_ {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_SELECTOR0_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC_SELECTOR0__0 {
    pub startV4Address: u32,
    pub startV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC_SELECTOR0__0 {}
impl ::std::default::Default for IPSEC_TRAFFIC_SELECTOR0__0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0__0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0__0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_SELECTOR0__0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TRAFFIC_SELECTOR0__1 {
    pub endV4Address: u32,
    pub endV6Address: [u8; 16],
}
impl IPSEC_TRAFFIC_SELECTOR0__1 {}
impl ::std::default::Default for IPSEC_TRAFFIC_SELECTOR0__1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR0__1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_SELECTOR0__1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_SELECTOR0__1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    pub flags: u32,
    pub numLocalTrafficSelectors: u32,
    pub localTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0_,
    pub numRemoteTrafficSelectors: u32,
    pub remoteTrafficSelectors: *mut IPSEC_TRAFFIC_SELECTOR0_,
}
impl IPSEC_TRAFFIC_SELECTOR_POLICY0_ {}
impl ::std::default::Default for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRAFFIC_SELECTOR_POLICY0_")
            .field("flags", &self.flags)
            .field("numLocalTrafficSelectors", &self.numLocalTrafficSelectors)
            .field("localTrafficSelectors", &self.localTrafficSelectors)
            .field("numRemoteTrafficSelectors", &self.numRemoteTrafficSelectors)
            .field("remoteTrafficSelectors", &self.remoteTrafficSelectors)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
            && self.numLocalTrafficSelectors == other.numLocalTrafficSelectors
            && self.localTrafficSelectors == other.localTrafficSelectors
            && self.numRemoteTrafficSelectors == other.numRemoteTrafficSelectors
            && self.remoteTrafficSelectors == other.remoteTrafficSelectors
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_SELECTOR_POLICY0_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TRAFFIC_STATISTICS0 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
}
impl IPSEC_TRAFFIC_STATISTICS0 {}
impl ::std::default::Default for IPSEC_TRAFFIC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_TRAFFIC_STATISTICS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRAFFIC_STATISTICS0")
            .field("encryptedByteCount", &self.encryptedByteCount)
            .field("authenticatedAHByteCount", &self.authenticatedAHByteCount)
            .field("authenticatedESPByteCount", &self.authenticatedESPByteCount)
            .field("transportByteCount", &self.transportByteCount)
            .field("tunnelByteCount", &self.tunnelByteCount)
            .field("offloadByteCount", &self.offloadByteCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.encryptedByteCount == other.encryptedByteCount
            && self.authenticatedAHByteCount == other.authenticatedAHByteCount
            && self.authenticatedESPByteCount == other.authenticatedESPByteCount
            && self.transportByteCount == other.transportByteCount
            && self.tunnelByteCount == other.tunnelByteCount
            && self.offloadByteCount == other.offloadByteCount
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_STATISTICS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_STATISTICS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TRAFFIC_STATISTICS1 {
    pub encryptedByteCount: u64,
    pub authenticatedAHByteCount: u64,
    pub authenticatedESPByteCount: u64,
    pub transportByteCount: u64,
    pub tunnelByteCount: u64,
    pub offloadByteCount: u64,
    pub totalSuccessfulPackets: u64,
}
impl IPSEC_TRAFFIC_STATISTICS1 {}
impl ::std::default::Default for IPSEC_TRAFFIC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_TRAFFIC_STATISTICS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRAFFIC_STATISTICS1")
            .field("encryptedByteCount", &self.encryptedByteCount)
            .field("authenticatedAHByteCount", &self.authenticatedAHByteCount)
            .field("authenticatedESPByteCount", &self.authenticatedESPByteCount)
            .field("transportByteCount", &self.transportByteCount)
            .field("tunnelByteCount", &self.tunnelByteCount)
            .field("offloadByteCount", &self.offloadByteCount)
            .field("totalSuccessfulPackets", &self.totalSuccessfulPackets)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.encryptedByteCount == other.encryptedByteCount
            && self.authenticatedAHByteCount == other.authenticatedAHByteCount
            && self.authenticatedESPByteCount == other.authenticatedESPByteCount
            && self.transportByteCount == other.transportByteCount
            && self.tunnelByteCount == other.tunnelByteCount
            && self.offloadByteCount == other.offloadByteCount
            && self.totalSuccessfulPackets == other.totalSuccessfulPackets
    }
}
impl ::std::cmp::Eq for IPSEC_TRAFFIC_STATISTICS1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_STATISTICS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_TRAFFIC_TYPE(pub i32);
pub const IPSEC_TRAFFIC_TYPE_TRANSPORT: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(0i32);
pub const IPSEC_TRAFFIC_TYPE_TUNNEL: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(1i32);
pub const IPSEC_TRAFFIC_TYPE_MAX: IPSEC_TRAFFIC_TYPE = IPSEC_TRAFFIC_TYPE(2i32);
impl ::std::convert::From<i32> for IPSEC_TRAFFIC_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_TRAFFIC_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPSEC_TRANSFORM_TYPE(pub i32);
pub const IPSEC_TRANSFORM_AH: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(1i32);
pub const IPSEC_TRANSFORM_ESP_AUTH: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(2i32);
pub const IPSEC_TRANSFORM_ESP_CIPHER: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(3i32);
pub const IPSEC_TRANSFORM_ESP_AUTH_AND_CIPHER: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(4i32);
pub const IPSEC_TRANSFORM_ESP_AUTH_FW: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(5i32);
pub const IPSEC_TRANSFORM_TYPE_MAX: IPSEC_TRANSFORM_TYPE = IPSEC_TRANSFORM_TYPE(6i32);
impl ::std::convert::From<i32> for IPSEC_TRANSFORM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPSEC_TRANSFORM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TRANSPORT_POLICY0 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY0,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TRANSPORT_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TRANSPORT_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPSEC_TRANSPORT_POLICY0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRANSPORT_POLICY0")
            .field("numIpsecProposals", &self.numIpsecProposals)
            .field("ipsecProposals", &self.ipsecProposals)
            .field("flags", &self.flags)
            .field(
                "ndAllowClearTimeoutSeconds",
                &self.ndAllowClearTimeoutSeconds,
            )
            .field("saIdleTimeout", &self.saIdleTimeout)
            .field("emPolicy", &self.emPolicy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TRANSPORT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals
            && self.ipsecProposals == other.ipsecProposals
            && self.flags == other.flags
            && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds
            && self.saIdleTimeout == other.saIdleTimeout
            && self.emPolicy == other.emPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TRANSPORT_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TRANSPORT_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TRANSPORT_POLICY1 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY1,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TRANSPORT_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TRANSPORT_POLICY1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPSEC_TRANSPORT_POLICY1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRANSPORT_POLICY1")
            .field("numIpsecProposals", &self.numIpsecProposals)
            .field("ipsecProposals", &self.ipsecProposals)
            .field("flags", &self.flags)
            .field(
                "ndAllowClearTimeoutSeconds",
                &self.ndAllowClearTimeoutSeconds,
            )
            .field("saIdleTimeout", &self.saIdleTimeout)
            .field("emPolicy", &self.emPolicy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TRANSPORT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals
            && self.ipsecProposals == other.ipsecProposals
            && self.flags == other.flags
            && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds
            && self.saIdleTimeout == other.saIdleTimeout
            && self.emPolicy == other.emPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TRANSPORT_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TRANSPORT_POLICY1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TRANSPORT_POLICY2 {
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub flags: IPSEC_POLICY_FLAG,
    pub ndAllowClearTimeoutSeconds: u32,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY2,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TRANSPORT_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TRANSPORT_POLICY2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IPSEC_TRANSPORT_POLICY2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_TRANSPORT_POLICY2")
            .field("numIpsecProposals", &self.numIpsecProposals)
            .field("ipsecProposals", &self.ipsecProposals)
            .field("flags", &self.flags)
            .field(
                "ndAllowClearTimeoutSeconds",
                &self.ndAllowClearTimeoutSeconds,
            )
            .field("saIdleTimeout", &self.saIdleTimeout)
            .field("emPolicy", &self.emPolicy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TRANSPORT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals
            && self.ipsecProposals == other.ipsecProposals
            && self.flags == other.flags
            && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds
            && self.saIdleTimeout == other.saIdleTimeout
            && self.emPolicy == other.emPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TRANSPORT_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TRANSPORT_POLICY2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TUNNEL_ENDPOINT0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous: IPSEC_TUNNEL_ENDPOINT0_0,
}
impl IPSEC_TUNNEL_ENDPOINT0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINT0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINT0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINT0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINT0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINT0_0 {
    pub v4Address: u32,
    pub v6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINT0_0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINT0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINT0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINT0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TUNNEL_ENDPOINTS0 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS0_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS0_1,
}
impl IPSEC_TUNNEL_ENDPOINTS0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS0_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS0_0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS0_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS0_1 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS0_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS0_1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_TUNNEL_ENDPOINTS1 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS1_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS1_1,
    pub localIfLuid: u64,
}
impl IPSEC_TUNNEL_ENDPOINTS1 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS1_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS1_0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS1_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS1_1 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS1_1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS1_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TUNNEL_ENDPOINTS2 {
    pub ipVersion: FWP_IP_VERSION,
    pub Anonymous1: IPSEC_TUNNEL_ENDPOINTS2_0,
    pub Anonymous2: IPSEC_TUNNEL_ENDPOINTS2_1,
    pub localIfLuid: u64,
    pub remoteFqdn: super::super::Foundation::PWSTR,
    pub numAddresses: u32,
    pub remoteAddresses: *mut IPSEC_TUNNEL_ENDPOINT0,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TUNNEL_ENDPOINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS2_0 {
    pub localV4Address: u32,
    pub localV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS2_0 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2_0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPSEC_TUNNEL_ENDPOINTS2_1 {
    pub remoteV4Address: u32,
    pub remoteV6Address: [u8; 16],
}
impl IPSEC_TUNNEL_ENDPOINTS2_1 {}
impl ::std::default::Default for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_ENDPOINTS2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPSEC_TUNNEL_ENDPOINTS2_1 {}
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_ENDPOINTS2_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TUNNEL_POLICY0 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS0,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY0,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TUNNEL_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TUNNEL_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_POLICY0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TUNNEL_POLICY0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TUNNEL_POLICY1 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS1,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY1,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TUNNEL_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TUNNEL_POLICY1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_POLICY1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TUNNEL_POLICY1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_POLICY1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IPSEC_TUNNEL_POLICY2 {
    pub flags: IPSEC_POLICY_FLAG,
    pub numIpsecProposals: u32,
    pub ipsecProposals: *mut IPSEC_PROPOSAL0,
    pub tunnelEndpoints: IPSEC_TUNNEL_ENDPOINTS2,
    pub saIdleTimeout: IPSEC_SA_IDLE_TIMEOUT0,
    pub emPolicy: *mut IKEEXT_EM_POLICY2,
    pub fwdPathSaLifetime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TUNNEL_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TUNNEL_POLICY2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_POLICY2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TUNNEL_POLICY2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_POLICY2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IPSEC_TUNNEL_POLICY3_ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IPSEC_TUNNEL_POLICY3_ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IPSEC_TUNNEL_POLICY3_ {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IPSEC_TUNNEL_POLICY3_ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IPSEC_TUNNEL_POLICY3_ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_V4_UDP_ENCAPSULATION0 {
    pub localUdpEncapPort: u16,
    pub remoteUdpEncapPort: u16,
}
impl IPSEC_V4_UDP_ENCAPSULATION0 {}
impl ::std::default::Default for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_V4_UDP_ENCAPSULATION0")
            .field("localUdpEncapPort", &self.localUdpEncapPort)
            .field("remoteUdpEncapPort", &self.remoteUdpEncapPort)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.localUdpEncapPort == other.localUdpEncapPort
            && self.remoteUdpEncapPort == other.remoteUdpEncapPort
    }
}
impl ::std::cmp::Eq for IPSEC_V4_UDP_ENCAPSULATION0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_V4_UDP_ENCAPSULATION0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    pub virtualIfTunnelId: u64,
    pub trafficSelectorId: u64,
}
impl IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {}
impl ::std::default::Default for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPSEC_VIRTUAL_IF_TUNNEL_INFO0")
            .field("virtualIfTunnelId", &self.virtualIfTunnelId)
            .field("trafficSelectorId", &self.trafficSelectorId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        self.virtualIfTunnelId == other.virtualIfTunnelId
            && self.trafficSelectorId == other.trafficSelectorId
    }
}
impl ::std::cmp::Eq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {}
unsafe impl ::windows::runtime::Abi for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct IPTLS_METADATA {
    pub SequenceNumber: u64,
}
impl IPTLS_METADATA {}
impl ::std::default::Default for IPTLS_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPTLS_METADATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPTLS_METADATA {}
unsafe impl ::windows::runtime::Abi for IPTLS_METADATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPV4_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for IPV4_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for IPV4_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for IPV4_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for IPV4_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV4_HEADER_0 {
    pub VersionAndHeaderLength: u8,
    pub Anonymous: IPV4_HEADER_0_0,
}
impl IPV4_HEADER_0 {}
impl ::std::default::Default for IPV4_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_HEADER_0_0 {
    pub _bitfield: u8,
}
impl IPV4_HEADER_0_0 {}
impl ::std::default::Default for IPV4_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV4_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV4_HEADER_1 {
    pub TypeOfServiceAndEcnField: u8,
    pub Anonymous: IPV4_HEADER_1_0,
}
impl IPV4_HEADER_1 {}
impl ::std::default::Default for IPV4_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_HEADER_1_0 {
    pub _bitfield: u8,
}
impl IPV4_HEADER_1_0 {}
impl ::std::default::Default for IPV4_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV4_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV4_HEADER_2 {
    pub FlagsAndOffset: u16,
    pub Anonymous: IPV4_HEADER_2_0,
}
impl IPV4_HEADER_2 {}
impl ::std::default::Default for IPV4_HEADER_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_2 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_HEADER_2_0 {
    pub _bitfield: u16,
}
impl IPV4_HEADER_2_0 {}
impl ::std::default::Default for IPV4_HEADER_2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV4_HEADER_2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV4_HEADER_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV4_HEADER_2_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_HEADER_2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV4_MAX_MINIMUM_MTU: u32 = 576u32;
pub const IPV4_MINIMUM_MTU: u32 = 576u32;
pub const IPV4_MIN_MINIMUM_MTU: u32 = 352u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_OPTION_HEADER {
    pub Anonymous: IPV4_OPTION_HEADER_0,
    pub OptionLength: u8,
}
impl IPV4_OPTION_HEADER {}
impl ::std::default::Default for IPV4_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_OPTION_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_OPTION_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV4_OPTION_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV4_OPTION_HEADER_0 {
    pub OptionType: u8,
    pub Anonymous: IPV4_OPTION_HEADER_0_0,
}
impl IPV4_OPTION_HEADER_0 {}
impl ::std::default::Default for IPV4_OPTION_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_OPTION_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_OPTION_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_OPTION_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_OPTION_HEADER_0_0 {
    pub _bitfield: u8,
}
impl IPV4_OPTION_HEADER_0_0 {}
impl ::std::default::Default for IPV4_OPTION_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV4_OPTION_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV4_OPTION_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV4_OPTION_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_OPTION_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPV4_OPTION_TYPE(pub i32);
pub const IP_OPT_EOL: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(0i32);
pub const IP_OPT_NOP: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(1i32);
pub const IP_OPT_SECURITY: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(130i32);
pub const IP_OPT_LSRR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(131i32);
pub const IP_OPT_TS: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(68i32);
pub const IP_OPT_RR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(7i32);
pub const IP_OPT_SSRR: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(137i32);
pub const IP_OPT_SID: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(136i32);
pub const IP_OPT_ROUTER_ALERT: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(148i32);
pub const IP_OPT_MULTIDEST: IPV4_OPTION_TYPE = IPV4_OPTION_TYPE(149i32);
impl ::std::convert::From<i32> for IPV4_OPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPV4_OPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_ROUTING_HEADER {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
}
impl IPV4_ROUTING_HEADER {}
impl ::std::default::Default for IPV4_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_ROUTING_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_ROUTING_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV4_ROUTING_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_TIMESTAMP_OPTION {
    pub OptionHeader: IPV4_OPTION_HEADER,
    pub Pointer: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0,
}
impl IPV4_TIMESTAMP_OPTION {}
impl ::std::default::Default for IPV4_TIMESTAMP_OPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_TIMESTAMP_OPTION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_TIMESTAMP_OPTION {}
unsafe impl ::windows::runtime::Abi for IPV4_TIMESTAMP_OPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV4_TIMESTAMP_OPTION_0 {
    pub FlagsOverflow: u8,
    pub Anonymous: IPV4_TIMESTAMP_OPTION_0_0,
}
impl IPV4_TIMESTAMP_OPTION_0 {}
impl ::std::default::Default for IPV4_TIMESTAMP_OPTION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV4_TIMESTAMP_OPTION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV4_TIMESTAMP_OPTION_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_TIMESTAMP_OPTION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV4_TIMESTAMP_OPTION_0_0 {
    pub _bitfield: u8,
}
impl IPV4_TIMESTAMP_OPTION_0_0 {}
impl ::std::default::Default for IPV4_TIMESTAMP_OPTION_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV4_TIMESTAMP_OPTION_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV4_TIMESTAMP_OPTION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV4_TIMESTAMP_OPTION_0_0 {}
unsafe impl ::windows::runtime::Abi for IPV4_TIMESTAMP_OPTION_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV4_VERSION: u32 = 4u32;
pub const IPV6_ECN_MASK: u32 = 12288u32;
pub const IPV6_ECN_SHIFT: u32 = 12u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_EXTENSION_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
}
impl IPV6_EXTENSION_HEADER {}
impl ::std::default::Default for IPV6_EXTENSION_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_EXTENSION_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPV6_EXTENSION_HEADER")
            .field("NextHeader", &self.NextHeader)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_EXTENSION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NextHeader == other.NextHeader && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for IPV6_EXTENSION_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV6_EXTENSION_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV6_FLOW_LABEL_MASK: u32 = 4294905600u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_FRAGMENT_HEADER {
    pub NextHeader: u8,
    pub Reserved: u8,
    pub Anonymous: IPV6_FRAGMENT_HEADER_0,
    pub Id: u32,
}
impl IPV6_FRAGMENT_HEADER {}
impl ::std::default::Default for IPV6_FRAGMENT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_FRAGMENT_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_FRAGMENT_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV6_FRAGMENT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV6_FRAGMENT_HEADER_0 {
    pub Anonymous: IPV6_FRAGMENT_HEADER_0_0,
    pub OffsetAndFlags: u16,
}
impl IPV6_FRAGMENT_HEADER_0 {}
impl ::std::default::Default for IPV6_FRAGMENT_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_FRAGMENT_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_FRAGMENT_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_FRAGMENT_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_FRAGMENT_HEADER_0_0 {
    pub _bitfield: u16,
}
impl IPV6_FRAGMENT_HEADER_0_0 {}
impl ::std::default::Default for IPV6_FRAGMENT_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_FRAGMENT_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_FRAGMENT_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV6_FRAGMENT_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_FRAGMENT_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV6_FULL_TRAFFIC_CLASS_MASK: u32 = 61455u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IPV6_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for IPV6_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for IPV6_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for IPV6_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for IPV6_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV6_HEADER_0 {
    pub VersionClassFlow: u32,
    pub Anonymous: IPV6_HEADER_0_0,
}
impl IPV6_HEADER_0 {}
impl ::std::default::Default for IPV6_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_HEADER_0_0 {
    pub _bitfield: u32,
}
impl IPV6_HEADER_0_0 {}
impl ::std::default::Default for IPV6_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV6_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV6_MINIMUM_MTU: u32 = 1280u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0,
    pub Value: u32,
}
impl IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {}
impl ::std::default::Default for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {}
unsafe impl ::windows::runtime::Abi for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {}
impl ::std::default::Default for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_NEIGHBOR_ADVERTISEMENT_FLAGS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_OPTION_HEADER {
    pub Type: u8,
    pub DataLength: u8,
}
impl IPV6_OPTION_HEADER {}
impl ::std::default::Default for IPV6_OPTION_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_OPTION_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPV6_OPTION_HEADER")
            .field("Type", &self.Type)
            .field("DataLength", &self.DataLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_OPTION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.DataLength == other.DataLength
    }
}
impl ::std::cmp::Eq for IPV6_OPTION_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV6_OPTION_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_OPTION_JUMBOGRAM {
    pub Header: IPV6_OPTION_HEADER,
    pub JumbogramLength: [u8; 4],
}
impl IPV6_OPTION_JUMBOGRAM {}
impl ::std::default::Default for IPV6_OPTION_JUMBOGRAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_OPTION_JUMBOGRAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPV6_OPTION_JUMBOGRAM")
            .field("Header", &self.Header)
            .field("JumbogramLength", &self.JumbogramLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_OPTION_JUMBOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.JumbogramLength == other.JumbogramLength
    }
}
impl ::std::cmp::Eq for IPV6_OPTION_JUMBOGRAM {}
unsafe impl ::windows::runtime::Abi for IPV6_OPTION_JUMBOGRAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_OPTION_ROUTER_ALERT {
    pub Header: IPV6_OPTION_HEADER,
    pub Value: [u8; 2],
}
impl IPV6_OPTION_ROUTER_ALERT {}
impl ::std::default::Default for IPV6_OPTION_ROUTER_ALERT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_OPTION_ROUTER_ALERT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPV6_OPTION_ROUTER_ALERT")
            .field("Header", &self.Header)
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_OPTION_ROUTER_ALERT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for IPV6_OPTION_ROUTER_ALERT {}
unsafe impl ::windows::runtime::Abi for IPV6_OPTION_ROUTER_ALERT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IPV6_OPTION_TYPE(pub i32);
pub const IP6OPT_PAD1: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(0i32);
pub const IP6OPT_PADN: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(1i32);
pub const IP6OPT_TUNNEL_LIMIT: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(4i32);
pub const IP6OPT_ROUTER_ALERT: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(5i32);
pub const IP6OPT_JUMBO: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(194i32);
pub const IP6OPT_NSAP_ADDR: IPV6_OPTION_TYPE = IPV6_OPTION_TYPE(195i32);
impl ::std::convert::From<i32> for IPV6_OPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IPV6_OPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    pub Anonymous: IPV6_ROUTER_ADVERTISEMENT_FLAGS_0,
    pub Value: u8,
}
impl IPV6_ROUTER_ADVERTISEMENT_FLAGS {}
impl ::std::default::Default for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPV6_ROUTER_ADVERTISEMENT_FLAGS {}
unsafe impl ::windows::runtime::Abi for IPV6_ROUTER_ADVERTISEMENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    pub _bitfield: u8,
}
impl IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {}
impl ::std::default::Default for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {}
unsafe impl ::windows::runtime::Abi for IPV6_ROUTER_ADVERTISEMENT_FLAGS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct IPV6_ROUTING_HEADER {
    pub NextHeader: u8,
    pub Length: u8,
    pub RoutingType: u8,
    pub SegmentsLeft: u8,
    pub Reserved: [u8; 4],
}
impl IPV6_ROUTING_HEADER {}
impl ::std::default::Default for IPV6_ROUTING_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IPV6_ROUTING_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IPV6_ROUTING_HEADER")
            .field("NextHeader", &self.NextHeader)
            .field("Length", &self.Length)
            .field("RoutingType", &self.RoutingType)
            .field("SegmentsLeft", &self.SegmentsLeft)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for IPV6_ROUTING_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.NextHeader == other.NextHeader
            && self.Length == other.Length
            && self.RoutingType == other.RoutingType
            && self.SegmentsLeft == other.SegmentsLeft
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for IPV6_ROUTING_HEADER {}
unsafe impl ::windows::runtime::Abi for IPV6_ROUTING_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPV6_TRAFFIC_CLASS_MASK: u32 = 49167u32;
pub const IPV6_VERSION: u32 = 96u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IP_OPTION_TIMESTAMP_FLAGS(pub i32);
pub const IP_OPTION_TIMESTAMP_ONLY: IP_OPTION_TIMESTAMP_FLAGS = IP_OPTION_TIMESTAMP_FLAGS(0i32);
pub const IP_OPTION_TIMESTAMP_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS = IP_OPTION_TIMESTAMP_FLAGS(1i32);
pub const IP_OPTION_TIMESTAMP_SPECIFIC_ADDRESS: IP_OPTION_TIMESTAMP_FLAGS =
    IP_OPTION_TIMESTAMP_FLAGS(3i32);
impl ::std::convert::From<i32> for IP_OPTION_TIMESTAMP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IP_OPTION_TIMESTAMP_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IP_VER_MASK: u32 = 240u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecDospGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospGetStatistics0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    idpstatistics: *mut IPSEC_DOSP_STATISTICS0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospGetStatistics0(
                enginehandle: super::super::Foundation::HANDLE,
                idpstatistics: *mut IPSEC_DOSP_STATISTICS0,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospGetStatistics0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(idpstatistics),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecDospSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const IPSEC_DOSP_STATE_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const IPSEC_DOSP_STATE_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospStateCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospStateDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecDospStateEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IPSEC_DOSP_STATE0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecDospStateEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IPSEC_DOSP_STATE0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecDospStateEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecGetStatistics0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    ipsecstatistics: *mut IPSEC_STATISTICS0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecGetStatistics0(
                enginehandle: super::super::Foundation::HANDLE,
                ipsecstatistics: *mut IPSEC_STATISTICS0,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecGetStatistics0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(ipsecstatistics),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecGetStatistics1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    ipsecstatistics: *mut IPSEC_STATISTICS1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecGetStatistics1(
                enginehandle: super::super::Foundation::HANDLE,
                ipsecstatistics: *mut IPSEC_STATISTICS1,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecGetStatistics1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(ipsecstatistics),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerAddAndRegister0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    keymanager: *const IPSEC_KEY_MANAGER0,
    keymanagercallbacks: *const IPSEC_KEY_MANAGER_CALLBACKS0,
    keymgmthandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerAddAndRegister0(
                enginehandle: super::super::Foundation::HANDLE,
                keymanager: *const IPSEC_KEY_MANAGER0,
                keymanagercallbacks: *const ::std::mem::ManuallyDrop<IPSEC_KEY_MANAGER_CALLBACKS0>,
                keymgmthandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecKeyManagerAddAndRegister0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(keymanager),
            ::std::mem::transmute(keymanagercallbacks),
            ::std::mem::transmute(keymgmthandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerGetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    reserved: *const ::std::ffi::c_void,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerGetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                reserved: *const ::std::ffi::c_void,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecKeyManagerGetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecKeyManagerSetSecurityInfoByKey0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    reserved: *const ::std::ffi::c_void,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerSetSecurityInfoByKey0(
                enginehandle: super::super::Foundation::HANDLE,
                reserved: *const ::std::ffi::c_void,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecKeyManagerSetSecurityInfoByKey0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(reserved),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecKeyManagerUnregisterAndDelete0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    keymgmthandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagerUnregisterAndDelete0(
                enginehandle: super::super::Foundation::HANDLE,
                keymgmthandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecKeyManagerUnregisterAndDelete0(
            enginehandle.into_param().abi(),
            keymgmthandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecKeyManagersGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut IPSEC_KEY_MANAGER0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecKeyManagersGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut IPSEC_KEY_MANAGER0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecKeyManagersGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    inboundbundle: *const IPSEC_SA_BUNDLE0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddInbound0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                inboundbundle: *const IPSEC_SA_BUNDLE0,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextAddInbound0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(inboundbundle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddInbound1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    inboundbundle: *const IPSEC_SA_BUNDLE1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddInbound1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                inboundbundle: *const IPSEC_SA_BUNDLE1,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextAddInbound1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(inboundbundle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    outboundbundle: *const IPSEC_SA_BUNDLE0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddOutbound0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                outboundbundle: *const IPSEC_SA_BUNDLE0,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextAddOutbound0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(outboundbundle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextAddOutbound1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    outboundbundle: *const IPSEC_SA_BUNDLE1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextAddOutbound1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                outboundbundle: *const IPSEC_SA_BUNDLE1,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextAddOutbound1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(outboundbundle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextCreate0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    outboundtraffic: *const IPSEC_TRAFFIC0,
    inboundfilterid: *mut u64,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreate0(
                enginehandle: super::super::Foundation::HANDLE,
                outboundtraffic: *const IPSEC_TRAFFIC0,
                inboundfilterid: *mut u64,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextCreate0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(outboundtraffic),
            ::std::mem::transmute(inboundfilterid),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextCreate1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    outboundtraffic: *const IPSEC_TRAFFIC1,
    virtualiftunnelinfo: *const IPSEC_VIRTUAL_IF_TUNNEL_INFO0,
    inboundfilterid: *mut u64,
    id: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreate1(
                enginehandle: super::super::Foundation::HANDLE,
                outboundtraffic: *const IPSEC_TRAFFIC1,
                virtualiftunnelinfo: *const IPSEC_VIRTUAL_IF_TUNNEL_INFO0,
                inboundfilterid: *mut u64,
                id: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextCreate1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(outboundtraffic),
            ::std::mem::transmute(virtualiftunnelinfo),
            ::std::mem::transmute(inboundfilterid),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const IPSEC_SA_CONTEXT_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextDeleteById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextDeleteById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextDeleteById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IPSEC_SA_CONTEXT0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IPSEC_SA_CONTEXT0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextEnum1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IPSEC_SA_CONTEXT1,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextEnum1(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IPSEC_SA_CONTEXT1,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextEnum1(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextExpire0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextExpire0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextExpire0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    sacontext: *mut *mut IPSEC_SA_CONTEXT0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                sacontext: *mut *mut IPSEC_SA_CONTEXT0,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(sacontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextGetById1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    sacontext: *mut *mut IPSEC_SA_CONTEXT1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetById1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                sacontext: *mut *mut IPSEC_SA_CONTEXT1,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextGetById1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(sacontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    getspi: *const IPSEC_GETSPI0,
    inboundspi: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetSpi0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                getspi: *const IPSEC_GETSPI0,
                inboundspi: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextGetSpi0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(getspi),
            ::std::mem::transmute(inboundspi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextGetSpi1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    getspi: *const IPSEC_GETSPI1,
    inboundspi: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextGetSpi1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                getspi: *const IPSEC_GETSPI1,
                inboundspi: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextGetSpi1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(getspi),
            ::std::mem::transmute(inboundspi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextSetSpi0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    getspi: *const IPSEC_GETSPI1,
    inboundspi: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSetSpi0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                getspi: *const IPSEC_GETSPI1,
                inboundspi: u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextSetSpi0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(getspi),
            ::std::mem::transmute(inboundspi),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextSubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    subscription: *const IPSEC_SA_CONTEXT_SUBSCRIPTION0,
    callback: ::std::option::Option<IPSEC_SA_CONTEXT_CALLBACK0>,
    context: *const ::std::ffi::c_void,
    eventshandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                subscription: *const IPSEC_SA_CONTEXT_SUBSCRIPTION0,
                callback: ::windows::runtime::RawPtr,
                context: *const ::std::ffi::c_void,
                eventshandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextSubscribe0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(subscription),
            ::std::mem::transmute(callback),
            ::std::mem::transmute(context),
            ::std::mem::transmute(eventshandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextSubscriptionsGet0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    entries: *mut *mut *mut IPSEC_SA_CONTEXT_SUBSCRIPTION0,
    numentries: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextSubscriptionsGet0(
                enginehandle: super::super::Foundation::HANDLE,
                entries: *mut *mut *mut IPSEC_SA_CONTEXT_SUBSCRIPTION0,
                numentries: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextSubscriptionsGet0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentries),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaContextUnsubscribe0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    eventshandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextUnsubscribe0(
                enginehandle: super::super::Foundation::HANDLE,
                eventshandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextUnsubscribe0(
            enginehandle.into_param().abi(),
            eventshandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaContextUpdate0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    flags: u64,
    newvalues: *const IPSEC_SA_CONTEXT1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaContextUpdate0(
                enginehandle: super::super::Foundation::HANDLE,
                flags: u64,
                newvalues: *const IPSEC_SA_CONTEXT1,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaContextUpdate0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(newvalues),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const IPSEC_SA_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const IPSEC_SA_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaDbGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDbGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaDbGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaDbSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDbSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaDbSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IPsecSaDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IPSEC_SA_DETAILS0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IPSEC_SA_DETAILS0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IPsecSaEnum1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IPSEC_SA_DETAILS1,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IPsecSaEnum1(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IPSEC_SA_DETAILS1,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IPsecSaEnum1(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextGetStatistics0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    ikeextstatistics: *mut IKEEXT_STATISTICS0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextGetStatistics0(
                enginehandle: super::super::Foundation::HANDLE,
                ikeextstatistics: *mut IKEEXT_STATISTICS0,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextGetStatistics0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(ikeextstatistics),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextGetStatistics1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    ikeextstatistics: *mut IKEEXT_STATISTICS1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextGetStatistics1(
                enginehandle: super::super::Foundation::HANDLE,
                ikeextstatistics: *mut IKEEXT_STATISTICS1,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextGetStatistics1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(ikeextstatistics),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaCreateEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumtemplate: *const IKEEXT_SA_ENUM_TEMPLATE0,
    enumhandle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaCreateEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumtemplate: *const IKEEXT_SA_ENUM_TEMPLATE0,
                enumhandle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaCreateEnumHandle0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(enumtemplate),
            ::std::mem::transmute(enumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaDbGetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *mut super::super::Foundation::PSID,
    sidgroup: *mut super::super::Foundation::PSID,
    dacl: *mut *mut super::super::Security::ACL,
    sacl: *mut *mut super::super::Security::ACL,
    securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDbGetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *mut super::super::Foundation::PSID,
                sidgroup: *mut super::super::Foundation::PSID,
                dacl: *mut *mut super::super::Security::ACL,
                sacl: *mut *mut super::super::Security::ACL,
                securitydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaDbGetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
            ::std::mem::transmute(securitydescriptor),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn IkeextSaDbSetSecurityInfo0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    securityinfo: u32,
    sidowner: *const super::super::Security::SID,
    sidgroup: *const super::super::Security::SID,
    dacl: *const super::super::Security::ACL,
    sacl: *const super::super::Security::ACL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDbSetSecurityInfo0(
                enginehandle: super::super::Foundation::HANDLE,
                securityinfo: u32,
                sidowner: *const super::super::Security::SID,
                sidgroup: *const super::super::Security::SID,
                dacl: *const super::super::Security::ACL,
                sacl: *const super::super::Security::ACL,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaDbSetSecurityInfo0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(securityinfo),
            ::std::mem::transmute(sidowner),
            ::std::mem::transmute(sidgroup),
            ::std::mem::transmute(dacl),
            ::std::mem::transmute(sacl),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaDeleteById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDeleteById0(enginehandle: super::super::Foundation::HANDLE, id: u64) -> u32;
        }
        ::std::mem::transmute(IkeextSaDeleteById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaDestroyEnumHandle0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaDestroyEnumHandle0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaDestroyEnumHandle0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IKEEXT_SA_DETAILS0,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum0(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IKEEXT_SA_DETAILS0,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaEnum0(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IKEEXT_SA_DETAILS1,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum1(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IKEEXT_SA_DETAILS1,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaEnum1(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaEnum2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    enumhandle: Param1,
    numentriesrequested: u32,
    entries: *mut *mut *mut IKEEXT_SA_DETAILS2,
    numentriesreturned: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaEnum2(
                enginehandle: super::super::Foundation::HANDLE,
                enumhandle: super::super::Foundation::HANDLE,
                numentriesrequested: u32,
                entries: *mut *mut *mut IKEEXT_SA_DETAILS2,
                numentriesreturned: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaEnum2(
            enginehandle.into_param().abi(),
            enumhandle.into_param().abi(),
            ::std::mem::transmute(numentriesrequested),
            ::std::mem::transmute(entries),
            ::std::mem::transmute(numentriesreturned),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById0<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    sa: *mut *mut IKEEXT_SA_DETAILS0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById0(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                sa: *mut *mut IKEEXT_SA_DETAILS0,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaGetById0(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(sa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById1<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    salookupcontext: *const ::windows::runtime::GUID,
    sa: *mut *mut IKEEXT_SA_DETAILS1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById1(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                salookupcontext: *const ::windows::runtime::GUID,
                sa: *mut *mut IKEEXT_SA_DETAILS1,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaGetById1(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(salookupcontext),
            ::std::mem::transmute(sa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IkeextSaGetById2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    enginehandle: Param0,
    id: u64,
    salookupcontext: *const ::windows::runtime::GUID,
    sa: *mut *mut IKEEXT_SA_DETAILS2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IkeextSaGetById2(
                enginehandle: super::super::Foundation::HANDLE,
                id: u64,
                salookupcontext: *const ::windows::runtime::GUID,
                sa: *mut *mut IKEEXT_SA_DETAILS2,
            ) -> u32;
        }
        ::std::mem::transmute(IkeextSaGetById2(
            enginehandle.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(salookupcontext),
            ::std::mem::transmute(sa),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MAX_IPV4_HLEN: u32 = 60u32;
pub const MAX_IPV4_PACKET: u32 = 65535u32;
pub const MAX_IPV6_PAYLOAD: u32 = 65535u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MLDV2_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for MLDV2_QUERY_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for MLDV2_QUERY_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for MLDV2_QUERY_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for MLDV2_QUERY_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union MLDV2_QUERY_HEADER_0 {
    pub MaxRespCode: u16,
    pub Anonymous: MLDV2_QUERY_HEADER_0_0,
}
impl MLDV2_QUERY_HEADER_0 {}
impl ::std::default::Default for MLDV2_QUERY_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MLDV2_QUERY_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MLDV2_QUERY_HEADER_0 {}
unsafe impl ::windows::runtime::Abi for MLDV2_QUERY_HEADER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MLDV2_QUERY_HEADER_0_0 {
    pub _bitfield: u16,
}
impl MLDV2_QUERY_HEADER_0_0 {}
impl ::std::default::Default for MLDV2_QUERY_HEADER_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MLDV2_QUERY_HEADER_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MLDV2_QUERY_HEADER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for MLDV2_QUERY_HEADER_0_0 {}
unsafe impl ::windows::runtime::Abi for MLDV2_QUERY_HEADER_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union MLDV2_QUERY_HEADER_1 {
    pub QueriersQueryInterfaceCode: u8,
    pub Anonymous: MLDV2_QUERY_HEADER_1_0,
}
impl MLDV2_QUERY_HEADER_1 {}
impl ::std::default::Default for MLDV2_QUERY_HEADER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MLDV2_QUERY_HEADER_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MLDV2_QUERY_HEADER_1 {}
unsafe impl ::windows::runtime::Abi for MLDV2_QUERY_HEADER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MLDV2_QUERY_HEADER_1_0 {
    pub _bitfield: u8,
}
impl MLDV2_QUERY_HEADER_1_0 {}
impl ::std::default::Default for MLDV2_QUERY_HEADER_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MLDV2_QUERY_HEADER_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MLDV2_QUERY_HEADER_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for MLDV2_QUERY_HEADER_1_0 {}
unsafe impl ::windows::runtime::Abi for MLDV2_QUERY_HEADER_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MLDV2_REPORT_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub Reserved: u16,
    pub RecordCount: u16,
}
impl MLDV2_REPORT_HEADER {}
impl ::std::default::Default for MLDV2_REPORT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MLDV2_REPORT_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MLDV2_REPORT_HEADER")
            .field("IcmpHeader", &self.IcmpHeader)
            .field("Reserved", &self.Reserved)
            .field("RecordCount", &self.RecordCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MLDV2_REPORT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.IcmpHeader == other.IcmpHeader
            && self.Reserved == other.Reserved
            && self.RecordCount == other.RecordCount
    }
}
impl ::std::cmp::Eq for MLDV2_REPORT_HEADER {}
unsafe impl ::windows::runtime::Abi for MLDV2_REPORT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLDV2_REPORT_RECORD_HEADER {
    pub Type: u8,
    pub AuxillaryDataLength: u8,
    pub SourceCount: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl MLDV2_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for MLDV2_REPORT_RECORD_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for MLDV2_REPORT_RECORD_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for MLDV2_REPORT_RECORD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for MLDV2_REPORT_RECORD_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MLD_HEADER {
    pub IcmpHeader: ICMP_HEADER,
    pub MaxRespTime: u16,
    pub Reserved: u16,
    pub MulticastAddress: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl MLD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for MLD_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for MLD_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for MLD_HEADER {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for MLD_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MLD_MAX_RESP_CODE_TYPE(pub i32);
pub const MLD_MAX_RESP_CODE_TYPE_NORMAL: MLD_MAX_RESP_CODE_TYPE = MLD_MAX_RESP_CODE_TYPE(0i32);
pub const MLD_MAX_RESP_CODE_TYPE_FLOAT: MLD_MAX_RESP_CODE_TYPE = MLD_MAX_RESP_CODE_TYPE(1i32);
impl ::std::convert::From<i32> for MLD_MAX_RESP_CODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MLD_MAX_RESP_CODE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ND_NA_FLAG_OVERRIDE: u32 = 536870912u32;
pub const ND_NA_FLAG_ROUTER: u32 = 2147483648u32;
pub const ND_NA_FLAG_SOLICITED: u32 = 1073741824u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ND_OPTION_TYPE(pub i32);
pub const ND_OPT_SOURCE_LINKADDR: ND_OPTION_TYPE = ND_OPTION_TYPE(1i32);
pub const ND_OPT_TARGET_LINKADDR: ND_OPTION_TYPE = ND_OPTION_TYPE(2i32);
pub const ND_OPT_PREFIX_INFORMATION: ND_OPTION_TYPE = ND_OPTION_TYPE(3i32);
pub const ND_OPT_REDIRECTED_HEADER: ND_OPTION_TYPE = ND_OPTION_TYPE(4i32);
pub const ND_OPT_MTU: ND_OPTION_TYPE = ND_OPTION_TYPE(5i32);
pub const ND_OPT_NBMA_SHORTCUT_LIMIT: ND_OPTION_TYPE = ND_OPTION_TYPE(6i32);
pub const ND_OPT_ADVERTISEMENT_INTERVAL: ND_OPTION_TYPE = ND_OPTION_TYPE(7i32);
pub const ND_OPT_HOME_AGENT_INFORMATION: ND_OPTION_TYPE = ND_OPTION_TYPE(8i32);
pub const ND_OPT_SOURCE_ADDR_LIST: ND_OPTION_TYPE = ND_OPTION_TYPE(9i32);
pub const ND_OPT_TARGET_ADDR_LIST: ND_OPTION_TYPE = ND_OPTION_TYPE(10i32);
pub const ND_OPT_ROUTE_INFO: ND_OPTION_TYPE = ND_OPTION_TYPE(24i32);
pub const ND_OPT_RDNSS: ND_OPTION_TYPE = ND_OPTION_TYPE(25i32);
pub const ND_OPT_DNSSL: ND_OPTION_TYPE = ND_OPTION_TYPE(31i32);
impl ::std::convert::From<i32> for ND_OPTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ND_OPTION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ND_OPT_DNSSL_MIN_LEN: u32 = 16u32;
pub const ND_OPT_PI_FLAG_AUTO: u32 = 64u32;
pub const ND_OPT_PI_FLAG_ONLINK: u32 = 128u32;
pub const ND_OPT_PI_FLAG_ROUTE: u32 = 1u32;
pub const ND_OPT_PI_FLAG_ROUTER_ADDR: u32 = 32u32;
pub const ND_OPT_PI_FLAG_SITE_PREFIX: u32 = 16u32;
pub const ND_OPT_RDNSS_MIN_LEN: u32 = 24u32;
pub const ND_OPT_RI_FLAG_PREFERENCE: u32 = 24u32;
pub const ND_RA_FLAG_HOME_AGENT: u32 = 32u32;
pub const ND_RA_FLAG_MANAGED: u32 = 128u32;
pub const ND_RA_FLAG_OTHER: u32 = 64u32;
pub const ND_RA_FLAG_PREFERENCE: u32 = 24u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct NPI_MODULEID {
    pub Length: u16,
    pub Type: NPI_MODULEID_TYPE,
    pub Anonymous: NPI_MODULEID_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl NPI_MODULEID {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for NPI_MODULEID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for NPI_MODULEID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for NPI_MODULEID {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for NPI_MODULEID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union NPI_MODULEID_0 {
    pub Guid: ::windows::runtime::GUID,
    pub IfLuid: super::super::System::SystemServices::LUID,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl NPI_MODULEID_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for NPI_MODULEID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for NPI_MODULEID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for NPI_MODULEID_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for NPI_MODULEID_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct NPI_MODULEID_TYPE(pub i32);
pub const MIT_GUID: NPI_MODULEID_TYPE = NPI_MODULEID_TYPE(1i32);
pub const MIT_IF_LUID: NPI_MODULEID_TYPE = NPI_MODULEID_TYPE(2i32);
impl ::std::convert::From<i32> for NPI_MODULEID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NPI_MODULEID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SIZEOF_IP_OPT_ROUTERALERT: u32 = 4u32;
pub const SIZEOF_IP_OPT_ROUTING_HEADER: u32 = 3u32;
pub const SIZEOF_IP_OPT_SECURITY: u32 = 11u32;
pub const SIZEOF_IP_OPT_STREAMIDENTIFIER: u32 = 4u32;
pub const SIZEOF_IP_OPT_TIMESTAMP_HEADER: u32 = 4u32;
pub const SNAP_CONTROL: u32 = 3u32;
pub const SNAP_DSAP: u32 = 170u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SNAP_HEADER {
    pub Dsap: u8,
    pub Ssap: u8,
    pub Control: u8,
    pub Oui: [u8; 3],
    pub Type: u16,
}
impl SNAP_HEADER {}
impl ::std::default::Default for SNAP_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SNAP_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SNAP_HEADER")
            .field("Dsap", &self.Dsap)
            .field("Ssap", &self.Ssap)
            .field("Control", &self.Control)
            .field("Oui", &self.Oui)
            .field("Type", &self.Type)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SNAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Dsap == other.Dsap
            && self.Ssap == other.Ssap
            && self.Control == other.Control
            && self.Oui == other.Oui
            && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for SNAP_HEADER {}
unsafe impl ::windows::runtime::Abi for SNAP_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SNAP_OUI: u32 = 0u32;
pub const SNAP_SSAP: u32 = 170u32;
pub const TH_ACK: u32 = 16u32;
pub const TH_CWR: u32 = 128u32;
pub const TH_ECE: u32 = 64u32;
pub const TH_FIN: u32 = 1u32;
pub const TH_OPT_EOL: u32 = 0u32;
pub const TH_OPT_FASTOPEN: u32 = 34u32;
pub const TH_OPT_MSS: u32 = 2u32;
pub const TH_OPT_NOP: u32 = 1u32;
pub const TH_OPT_SACK: u32 = 5u32;
pub const TH_OPT_SACK_PERMITTED: u32 = 4u32;
pub const TH_OPT_TS: u32 = 8u32;
pub const TH_OPT_WS: u32 = 3u32;
pub const TH_PSH: u32 = 8u32;
pub const TH_RST: u32 = 4u32;
pub const TH_SYN: u32 = 2u32;
pub const TH_URG: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TUNNEL_SUB_TYPE(pub i32);
pub const TUNNEL_SUB_TYPE_NONE: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(0i32);
pub const TUNNEL_SUB_TYPE_CP: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(1i32);
pub const TUNNEL_SUB_TYPE_IPTLS: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(2i32);
pub const TUNNEL_SUB_TYPE_HA: TUNNEL_SUB_TYPE = TUNNEL_SUB_TYPE(3i32);
impl ::std::convert::From<i32> for TUNNEL_SUB_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TUNNEL_SUB_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VLAN_TAG {
    pub Anonymous: VLAN_TAG_0,
    pub Type: u16,
}
impl VLAN_TAG {}
impl ::std::default::Default for VLAN_TAG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VLAN_TAG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VLAN_TAG {}
unsafe impl ::windows::runtime::Abi for VLAN_TAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VLAN_TAG_0 {
    pub Tag: u16,
    pub Anonymous: VLAN_TAG_0_0,
}
impl VLAN_TAG_0 {}
impl ::std::default::Default for VLAN_TAG_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VLAN_TAG_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VLAN_TAG_0 {}
unsafe impl ::windows::runtime::Abi for VLAN_TAG_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VLAN_TAG_0_0 {
    pub _bitfield: u16,
}
impl VLAN_TAG_0_0 {}
impl ::std::default::Default for VLAN_TAG_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VLAN_TAG_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VLAN_TAG_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for VLAN_TAG_0_0 {}
unsafe impl ::windows::runtime::Abi for VLAN_TAG_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const _BIG_ENDIAN: u32 = 4321u32;
pub const _LITTLE_ENDIAN: u32 = 1234u32;
pub const _PDP_ENDIAN: u32 = 3412u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_neighbor_advert {
    pub nd_na_hdr: ICMP_MESSAGE,
    pub nd_na_target: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl nd_neighbor_advert {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for nd_neighbor_advert {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for nd_neighbor_advert {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for nd_neighbor_advert {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for nd_neighbor_advert {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_neighbor_solicit {
    pub nd_ns_hdr: ICMP_MESSAGE,
    pub nd_ns_target: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl nd_neighbor_solicit {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for nd_neighbor_solicit {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for nd_neighbor_solicit {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for nd_neighbor_solicit {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for nd_neighbor_solicit {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_dnssl {
    pub nd_opt_dnssl_type: u8,
    pub nd_opt_dnssl_len: u8,
    pub nd_opt_dnssl_reserved: u16,
    pub nd_opt_dnssl_lifetime: u32,
}
impl nd_opt_dnssl {}
impl ::std::default::Default for nd_opt_dnssl {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_dnssl {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("nd_opt_dnssl")
            .field("nd_opt_dnssl_type", &self.nd_opt_dnssl_type)
            .field("nd_opt_dnssl_len", &self.nd_opt_dnssl_len)
            .field("nd_opt_dnssl_reserved", &self.nd_opt_dnssl_reserved)
            .field("nd_opt_dnssl_lifetime", &self.nd_opt_dnssl_lifetime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_dnssl {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_dnssl_type == other.nd_opt_dnssl_type
            && self.nd_opt_dnssl_len == other.nd_opt_dnssl_len
            && self.nd_opt_dnssl_reserved == other.nd_opt_dnssl_reserved
            && self.nd_opt_dnssl_lifetime == other.nd_opt_dnssl_lifetime
    }
}
impl ::std::cmp::Eq for nd_opt_dnssl {}
unsafe impl ::windows::runtime::Abi for nd_opt_dnssl {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_hdr {
    pub nd_opt_type: u8,
    pub nd_opt_len: u8,
}
impl nd_opt_hdr {}
impl ::std::default::Default for nd_opt_hdr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_hdr {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("nd_opt_hdr")
            .field("nd_opt_type", &self.nd_opt_type)
            .field("nd_opt_len", &self.nd_opt_len)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_hdr {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_type == other.nd_opt_type && self.nd_opt_len == other.nd_opt_len
    }
}
impl ::std::cmp::Eq for nd_opt_hdr {}
unsafe impl ::windows::runtime::Abi for nd_opt_hdr {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_mtu {
    pub nd_opt_mtu_type: u8,
    pub nd_opt_mtu_len: u8,
    pub nd_opt_mtu_reserved: u16,
    pub nd_opt_mtu_mtu: u32,
}
impl nd_opt_mtu {}
impl ::std::default::Default for nd_opt_mtu {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_mtu {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("nd_opt_mtu")
            .field("nd_opt_mtu_type", &self.nd_opt_mtu_type)
            .field("nd_opt_mtu_len", &self.nd_opt_mtu_len)
            .field("nd_opt_mtu_reserved", &self.nd_opt_mtu_reserved)
            .field("nd_opt_mtu_mtu", &self.nd_opt_mtu_mtu)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_mtu {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_mtu_type == other.nd_opt_mtu_type
            && self.nd_opt_mtu_len == other.nd_opt_mtu_len
            && self.nd_opt_mtu_reserved == other.nd_opt_mtu_reserved
            && self.nd_opt_mtu_mtu == other.nd_opt_mtu_mtu
    }
}
impl ::std::cmp::Eq for nd_opt_mtu {}
unsafe impl ::windows::runtime::Abi for nd_opt_mtu {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl nd_opt_prefix_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for nd_opt_prefix_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for nd_opt_prefix_info {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for nd_opt_prefix_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for nd_opt_prefix_info {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union nd_opt_prefix_info_0 {
    pub nd_opt_pi_flags_reserved: u8,
    pub Flags: nd_opt_prefix_info_0_0,
}
impl nd_opt_prefix_info_0 {}
impl ::std::default::Default for nd_opt_prefix_info_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for nd_opt_prefix_info_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for nd_opt_prefix_info_0 {}
unsafe impl ::windows::runtime::Abi for nd_opt_prefix_info_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_prefix_info_0_0 {
    pub _bitfield: u8,
}
impl nd_opt_prefix_info_0_0 {}
impl ::std::default::Default for nd_opt_prefix_info_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_prefix_info_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Flags_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_prefix_info_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for nd_opt_prefix_info_0_0 {}
unsafe impl ::windows::runtime::Abi for nd_opt_prefix_info_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union nd_opt_prefix_info_1 {
    pub nd_opt_pi_reserved2: u32,
    pub Anonymous: nd_opt_prefix_info_1_0,
}
impl nd_opt_prefix_info_1 {}
impl ::std::default::Default for nd_opt_prefix_info_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for nd_opt_prefix_info_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for nd_opt_prefix_info_1 {}
unsafe impl ::windows::runtime::Abi for nd_opt_prefix_info_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_prefix_info_1_0 {
    pub nd_opt_pi_reserved3: [u8; 3],
    pub nd_opt_pi_site_prefix_len: u8,
}
impl nd_opt_prefix_info_1_0 {}
impl ::std::default::Default for nd_opt_prefix_info_1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_prefix_info_1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("nd_opt_pi_reserved3", &self.nd_opt_pi_reserved3)
            .field("nd_opt_pi_site_prefix_len", &self.nd_opt_pi_site_prefix_len)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_prefix_info_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_pi_reserved3 == other.nd_opt_pi_reserved3
            && self.nd_opt_pi_site_prefix_len == other.nd_opt_pi_site_prefix_len
    }
}
impl ::std::cmp::Eq for nd_opt_prefix_info_1_0 {}
unsafe impl ::windows::runtime::Abi for nd_opt_prefix_info_1_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_rd_hdr {
    pub nd_opt_rh_type: u8,
    pub nd_opt_rh_len: u8,
    pub nd_opt_rh_reserved1: u16,
    pub nd_opt_rh_reserved2: u32,
}
impl nd_opt_rd_hdr {}
impl ::std::default::Default for nd_opt_rd_hdr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_rd_hdr {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("nd_opt_rd_hdr")
            .field("nd_opt_rh_type", &self.nd_opt_rh_type)
            .field("nd_opt_rh_len", &self.nd_opt_rh_len)
            .field("nd_opt_rh_reserved1", &self.nd_opt_rh_reserved1)
            .field("nd_opt_rh_reserved2", &self.nd_opt_rh_reserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_rd_hdr {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_rh_type == other.nd_opt_rh_type
            && self.nd_opt_rh_len == other.nd_opt_rh_len
            && self.nd_opt_rh_reserved1 == other.nd_opt_rh_reserved1
            && self.nd_opt_rh_reserved2 == other.nd_opt_rh_reserved2
    }
}
impl ::std::cmp::Eq for nd_opt_rd_hdr {}
unsafe impl ::windows::runtime::Abi for nd_opt_rd_hdr {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_rdnss {
    pub nd_opt_rdnss_type: u8,
    pub nd_opt_rdnss_len: u8,
    pub nd_opt_rdnss_reserved: u16,
    pub nd_opt_rdnss_lifetime: u32,
}
impl nd_opt_rdnss {}
impl ::std::default::Default for nd_opt_rdnss {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_rdnss {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("nd_opt_rdnss")
            .field("nd_opt_rdnss_type", &self.nd_opt_rdnss_type)
            .field("nd_opt_rdnss_len", &self.nd_opt_rdnss_len)
            .field("nd_opt_rdnss_reserved", &self.nd_opt_rdnss_reserved)
            .field("nd_opt_rdnss_lifetime", &self.nd_opt_rdnss_lifetime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_rdnss {
    fn eq(&self, other: &Self) -> bool {
        self.nd_opt_rdnss_type == other.nd_opt_rdnss_type
            && self.nd_opt_rdnss_len == other.nd_opt_rdnss_len
            && self.nd_opt_rdnss_reserved == other.nd_opt_rdnss_reserved
            && self.nd_opt_rdnss_lifetime == other.nd_opt_rdnss_lifetime
    }
}
impl ::std::cmp::Eq for nd_opt_rdnss {}
unsafe impl ::windows::runtime::Abi for nd_opt_rdnss {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl nd_opt_route_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for nd_opt_route_info {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for nd_opt_route_info {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for nd_opt_route_info {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for nd_opt_route_info {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union nd_opt_route_info_0 {
    pub nd_opt_ri_flags_reserved: u8,
    pub Flags: nd_opt_route_info_0_0,
}
impl nd_opt_route_info_0 {}
impl ::std::default::Default for nd_opt_route_info_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for nd_opt_route_info_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for nd_opt_route_info_0 {}
unsafe impl ::windows::runtime::Abi for nd_opt_route_info_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_opt_route_info_0_0 {
    pub _bitfield: u8,
}
impl nd_opt_route_info_0_0 {}
impl ::std::default::Default for nd_opt_route_info_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for nd_opt_route_info_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Flags_e__Struct")
            .field("_bitfield", &self._bitfield)
            .finish()
    }
}
impl ::std::cmp::PartialEq for nd_opt_route_info_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for nd_opt_route_info_0_0 {}
unsafe impl ::windows::runtime::Abi for nd_opt_route_info_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct nd_redirect {
    pub nd_rd_hdr: ICMP_MESSAGE,
    pub nd_rd_target: super::super::Networking::WinSock::IN6_ADDR,
    pub nd_rd_dst: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl nd_redirect {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for nd_redirect {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for nd_redirect {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for nd_redirect {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for nd_redirect {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_router_advert {
    pub nd_ra_hdr: ICMP_MESSAGE,
    pub nd_ra_reachable: u32,
    pub nd_ra_retransmit: u32,
}
impl nd_router_advert {}
impl ::std::default::Default for nd_router_advert {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for nd_router_advert {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for nd_router_advert {}
unsafe impl ::windows::runtime::Abi for nd_router_advert {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct nd_router_solicit {
    pub nd_rs_hdr: ICMP_MESSAGE,
}
impl nd_router_solicit {}
impl ::std::default::Default for nd_router_solicit {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for nd_router_solicit {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for nd_router_solicit {}
unsafe impl ::windows::runtime::Abi for nd_router_solicit {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
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
impl tcp_hdr {}
impl ::std::default::Default for tcp_hdr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tcp_hdr {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tcp_hdr {}
unsafe impl ::windows::runtime::Abi for tcp_hdr {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_opt_fastopen {
    pub Kind: u8,
    pub Length: u8,
    pub Cookie: [u8; 1],
}
impl tcp_opt_fastopen {}
impl ::std::default::Default for tcp_opt_fastopen {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_opt_fastopen {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_opt_fastopen")
            .field("Kind", &self.Kind)
            .field("Length", &self.Length)
            .field("Cookie", &self.Cookie)
            .finish()
    }
}
impl ::std::cmp::PartialEq for tcp_opt_fastopen {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Length == other.Length && self.Cookie == other.Cookie
    }
}
impl ::std::cmp::Eq for tcp_opt_fastopen {}
unsafe impl ::windows::runtime::Abi for tcp_opt_fastopen {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tcp_opt_mss {
    pub Kind: u8,
    pub Length: u8,
    pub Mss: u16,
}
impl tcp_opt_mss {}
impl ::std::default::Default for tcp_opt_mss {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tcp_opt_mss {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tcp_opt_mss {}
unsafe impl ::windows::runtime::Abi for tcp_opt_mss {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_opt_sack {
    pub Kind: u8,
    pub Length: u8,
    pub Block: [tcp_opt_sack_0; 1],
}
impl tcp_opt_sack {}
impl ::std::default::Default for tcp_opt_sack {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tcp_opt_sack {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tcp_opt_sack {}
unsafe impl ::windows::runtime::Abi for tcp_opt_sack {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tcp_opt_sack_0 {
    pub Left: u32,
    pub Right: u32,
}
impl tcp_opt_sack_0 {}
impl ::std::default::Default for tcp_opt_sack_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tcp_opt_sack_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tcp_opt_sack_0 {}
unsafe impl ::windows::runtime::Abi for tcp_opt_sack_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_opt_sack_permitted {
    pub Kind: u8,
    pub Length: u8,
}
impl tcp_opt_sack_permitted {}
impl ::std::default::Default for tcp_opt_sack_permitted {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_opt_sack_permitted {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_opt_sack_permitted")
            .field("Kind", &self.Kind)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for tcp_opt_sack_permitted {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for tcp_opt_sack_permitted {}
unsafe impl ::windows::runtime::Abi for tcp_opt_sack_permitted {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct tcp_opt_ts {
    pub Kind: u8,
    pub Length: u8,
    pub Val: u32,
    pub EcR: u32,
}
impl tcp_opt_ts {}
impl ::std::default::Default for tcp_opt_ts {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for tcp_opt_ts {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for tcp_opt_ts {}
unsafe impl ::windows::runtime::Abi for tcp_opt_ts {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_opt_unknown {
    pub Kind: u8,
    pub Length: u8,
}
impl tcp_opt_unknown {}
impl ::std::default::Default for tcp_opt_unknown {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_opt_unknown {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_opt_unknown")
            .field("Kind", &self.Kind)
            .field("Length", &self.Length)
            .finish()
    }
}
impl ::std::cmp::PartialEq for tcp_opt_unknown {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Length == other.Length
    }
}
impl ::std::cmp::Eq for tcp_opt_unknown {}
unsafe impl ::windows::runtime::Abi for tcp_opt_unknown {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_opt_ws {
    pub Kind: u8,
    pub Length: u8,
    pub ShiftCnt: u8,
}
impl tcp_opt_ws {}
impl ::std::default::Default for tcp_opt_ws {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_opt_ws {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_opt_ws")
            .field("Kind", &self.Kind)
            .field("Length", &self.Length)
            .field("ShiftCnt", &self.ShiftCnt)
            .finish()
    }
}
impl ::std::cmp::PartialEq for tcp_opt_ws {
    fn eq(&self, other: &Self) -> bool {
        self.Kind == other.Kind && self.Length == other.Length && self.ShiftCnt == other.ShiftCnt
    }
}
impl ::std::cmp::Eq for tcp_opt_ws {}
unsafe impl ::windows::runtime::Abi for tcp_opt_ws {
    type Abi = Self;
    type DefaultType = Self;
}
